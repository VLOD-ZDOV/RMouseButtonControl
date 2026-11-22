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

    /// Starts global mouse hook loop with event suppression (blocks calling thread)
    pub fn run(&self) -> Result<(), rdev::ListenError> {
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
