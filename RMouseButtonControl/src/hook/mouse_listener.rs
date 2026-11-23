use crate::engine::EventDispatcher;
use crate::models::{MouseButton, MouseButtonState};
use crate::simulation::Simulator;
use crate::window::CurrentWindow;
use rdev::{grab, listen, Button, Event, EventType};
use std::sync::Arc;

pub struct MouseHookService<S: Simulator + ?Sized + 'static, W: CurrentWindow + 'static> {
    dispatcher: Arc<EventDispatcher<S>>,
    window_detector: Arc<W>,
}

impl<S: Simulator + ?Sized + 'static, W: CurrentWindow + 'static> MouseHookService<S, W> {
    pub fn new(dispatcher: Arc<EventDispatcher<S>>, window_detector: Arc<W>) -> Self {
        Self {
            dispatcher,
            window_detector,
        }
    }

    pub fn to_mouse_button(button: Button) -> Option<MouseButton> {
        match button {
            Button::Left => Some(MouseButton::MouseButton1),
            Button::Right => Some(MouseButton::MouseButton2),
            Button::Middle => Some(MouseButton::MouseButton3),
            Button::Unknown(8) | Button::Unknown(1) => Some(MouseButton::MouseButton4),
            Button::Unknown(9) | Button::Unknown(2) => Some(MouseButton::MouseButton5),
            _ => None,
        }
    }

    #[cfg(target_os = "linux")]
    pub fn try_run_evdev(&self) -> bool {
        let mut mice = Vec::new();
        for (path, device) in evdev::enumerate() {
            let is_mouse = device.supported_keys().map_or(false, |keys| {
                keys.contains(evdev::KeyCode::BTN_LEFT)
            });
            let is_our_virtual = device.name().map_or(false, |name| {
                name.contains("RMouseButtonControl")
            });
            if is_mouse && !is_our_virtual {
                println!("[MouseHookService] Discovered evdev mouse: {:?} ({})", path, device.name().unwrap_or(""));
                mice.push(device);
            }
        }

        if mice.is_empty() {
            return false;
        }

        println!("[MouseHookService] Initialized evdev listener across {} mouse devices (Wayland native)", mice.len());

        let mut handles = Vec::new();
        for mut device in mice {
            let dispatcher = Arc::clone(&self.dispatcher);
            let detector = Arc::clone(&self.window_detector);

            let handle = std::thread::spawn(move || {
                loop {
                    match device.fetch_events() {
                        Ok(events) => {
                            for ev in events {
                                match ev.event_type() {
                                    evdev::EventType::KEY => {
                                        let code = evdev::KeyCode::new(ev.code());
                                        let mb = match code {
                                            evdev::KeyCode::BTN_LEFT => Some(MouseButton::MouseButton1),
                                            evdev::KeyCode::BTN_RIGHT => Some(MouseButton::MouseButton2),
                                            evdev::KeyCode::BTN_MIDDLE => Some(MouseButton::MouseButton3),
                                            evdev::KeyCode::BTN_SIDE => Some(MouseButton::MouseButton4),
                                            evdev::KeyCode::BTN_EXTRA => Some(MouseButton::MouseButton5),
                                            _ => None,
                                        };
                                        if let Some(button) = mb {
                                            let state = if ev.value() == 1 {
                                                MouseButtonState::Pressed
                                            } else if ev.value() == 0 {
                                                MouseButtonState::Released
                                            } else {
                                                continue;
                                            };
                                            let foreground = detector.get_foreground_window();
                                            dispatcher.process_event(button, state, &foreground);
                                        }
                                    }
                                    evdev::EventType::RELATIVE => {
                                        let code = evdev::RelativeAxisCode(ev.code());
                                        let foreground = detector.get_foreground_window();
                                        match code {
                                            evdev::RelativeAxisCode::REL_WHEEL => {
                                                if ev.value() > 0 {
                                                    dispatcher.process_event(
                                                        MouseButton::MouseWheelUp,
                                                        MouseButtonState::Pressed,
                                                        &foreground,
                                                    );
                                                } else if ev.value() < 0 {
                                                    dispatcher.process_event(
                                                        MouseButton::MouseWheelDown,
                                                        MouseButtonState::Pressed,
                                                        &foreground,
                                                    );
                                                }
                                            }
                                            evdev::RelativeAxisCode::REL_HWHEEL => {
                                                if ev.value() > 0 {
                                                    dispatcher.process_event(
                                                        MouseButton::MouseWheelRight,
                                                        MouseButtonState::Pressed,
                                                        &foreground,
                                                    );
                                                } else if ev.value() < 0 {
                                                    dispatcher.process_event(
                                                        MouseButton::MouseWheelLeft,
                                                        MouseButtonState::Pressed,
                                                        &foreground,
                                                    );
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        Err(err) => {
                            eprintln!("[MouseHookService] evdev fetch_events error: {err}");
                            break;
                        }
                    }
                }
            });
            handles.push(handle);
        }

        for h in handles {
            let _ = h.join();
        }

        true
    }

    /// Starts global mouse hook loop with event suppression (blocks calling thread)
    pub fn run(&self) -> Result<(), rdev::ListenError> {
        #[cfg(target_os = "linux")]
        {
            if self.try_run_evdev() {
                return Ok(());
            }
            eprintln!("[MouseHookService] evdev listener not available, falling back to rdev");
        }

        let dispatcher = Arc::clone(&self.dispatcher);
        let detector = Arc::clone(&self.window_detector);

        let dispatcher_grab = Arc::clone(&dispatcher);
        let detector_grab = Arc::clone(&detector);

        let grab_res = grab(move |event: Event| -> Option<Event> {
            let mut suppress = false;
            match event.event_type {
                EventType::ButtonPress(button) => {
                    if let Some(mb) = Self::to_mouse_button(button) {
                        let foreground = detector_grab.get_foreground_window();
                        let disp = dispatcher_grab.process_event(mb, MouseButtonState::Pressed, &foreground);
                        suppress = disp.should_suppress;
                    }
                }
                EventType::ButtonRelease(button) => {
                    if let Some(mb) = Self::to_mouse_button(button) {
                        let foreground = detector_grab.get_foreground_window();
                        let disp = dispatcher_grab.process_event(mb, MouseButtonState::Released, &foreground);
                        suppress = disp.should_suppress;
                    }
                }
                EventType::Wheel { delta_x, delta_y } => {
                    let foreground = detector_grab.get_foreground_window();
                    if delta_y > 0 {
                        let disp = dispatcher_grab.process_event(
                            MouseButton::MouseWheelUp,
                            MouseButtonState::Pressed,
                            &foreground,
                        );
                        suppress = disp.should_suppress;
                    } else if delta_y < 0 {
                        let disp = dispatcher_grab.process_event(
                            MouseButton::MouseWheelDown,
                            MouseButtonState::Pressed,
                            &foreground,
                        );
                        suppress = disp.should_suppress;
                    }
                    if delta_x < 0 {
                        let disp = dispatcher_grab.process_event(
                            MouseButton::MouseWheelLeft,
                            MouseButtonState::Pressed,
                            &foreground,
                        );
                        suppress = suppress || disp.should_suppress;
                    } else if delta_x > 0 {
                        let disp = dispatcher_grab.process_event(
                            MouseButton::MouseWheelRight,
                            MouseButtonState::Pressed,
                            &foreground,
                        );
                        suppress = suppress || disp.should_suppress;
                    }
                }
                _ => {}
            }

            if suppress {
                None
            } else {
                Some(event)
            }
        });

        if let Err(err) = grab_res {
            eprintln!("[MouseHookService] rdev::grab error: {err:?}, falling back to rdev::listen");
            listen(move |event: Event| {
                match event.event_type {
                    EventType::ButtonPress(button) => {
                        if let Some(mb) = Self::to_mouse_button(button) {
                            let foreground = detector.get_foreground_window();
                            dispatcher.process_event(mb, MouseButtonState::Pressed, &foreground);
                        }
                    }
                    EventType::ButtonRelease(button) => {
                        if let Some(mb) = Self::to_mouse_button(button) {
                            let foreground = detector.get_foreground_window();
                            dispatcher.process_event(mb, MouseButtonState::Released, &foreground);
                        }
                    }
                    EventType::Wheel { delta_x, delta_y } => {
                        let foreground = detector.get_foreground_window();
                        if delta_y > 0 {
                            dispatcher.process_event(
                                MouseButton::MouseWheelUp,
                                MouseButtonState::Pressed,
                                &foreground,
                            );
                        } else if delta_y < 0 {
                            dispatcher.process_event(
                                MouseButton::MouseWheelDown,
                                MouseButtonState::Pressed,
                                &foreground,
                            );
                        }
                        if delta_x < 0 {
                            dispatcher.process_event(
                                MouseButton::MouseWheelLeft,
                                MouseButtonState::Pressed,
                                &foreground,
                            );
                        } else if delta_x > 0 {
                            dispatcher.process_event(
                                MouseButton::MouseWheelRight,
                                MouseButtonState::Pressed,
                                &foreground,
                            );
                        }
                    }
                    _ => {}
                }
            })?;
        }

        Ok(())
    }
}
