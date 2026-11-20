use crate::engine::EventDispatcher;
use crate::models::{MouseButton, MouseButtonState};
use crate::simulation::Simulator;
use crate::window::CurrentWindow;
use rdev::{listen, Button, Event, EventType};
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

    /// Starts global mouse hook loop (blocks calling thread)
    pub fn run(&self) -> Result<(), rdev::ListenError> {
        let dispatcher = Arc::clone(&self.dispatcher);
        let detector = Arc::clone(&self.window_detector);

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
        })
    }
}
