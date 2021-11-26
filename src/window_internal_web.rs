/*
 *  Copyright 2021 QuantumBadger
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 */

use std::borrow::Borrow;
use std::cell::{Cell, RefCell};
use std::convert::TryInto;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use wasm_bindgen::closure::Closure;
use web_sys::MouseEvent;

use crate::dimen::Vector2;
use crate::error::{BacktraceError, ErrorMessage};
use crate::numeric::RoundFloat;
use crate::web::{WebCanvasElement, WebCursorType, WebDocument, WebPending, WebWindow};
use crate::window::{
    DrawingWindowHandler,
    EventLoopSendError,
    MouseButton,
    UserEventSender,
    VirtualKeyCode,
    WindowFullscreenMode,
    WindowHandler,
    WindowHelper,
    WindowStartupInfo
};
use crate::{GLRenderer, WebCanvasAttachOptions};

fn key_code_from_web(code: &str) -> Option<VirtualKeyCode>
{
    match code {
        "Escape" => Some(VirtualKeyCode::Escape),
        "Digit1" => Some(VirtualKeyCode::Key1),
        "Digit2" => Some(VirtualKeyCode::Key2),
        "Digit3" => Some(VirtualKeyCode::Key3),
        "Digit4" => Some(VirtualKeyCode::Key4),
        "Digit5" => Some(VirtualKeyCode::Key5),
        "Digit6" => Some(VirtualKeyCode::Key6),
        "Digit7" => Some(VirtualKeyCode::Key7),
        "Digit8" => Some(VirtualKeyCode::Key8),
        "Digit9" => Some(VirtualKeyCode::Key9),
        "Digit0" => Some(VirtualKeyCode::Key0),
        "Minus" => Some(VirtualKeyCode::Minus),
        "Equal" => Some(VirtualKeyCode::Equals),
        "Backspace" => Some(VirtualKeyCode::Backspace),
        "Tab" => Some(VirtualKeyCode::Tab),
        "KeyQ" => Some(VirtualKeyCode::Q),
        "KeyW" => Some(VirtualKeyCode::W),
        "KeyE" => Some(VirtualKeyCode::E),
        "KeyR" => Some(VirtualKeyCode::R),
        "KeyT" => Some(VirtualKeyCode::T),
        "KeyY" => Some(VirtualKeyCode::Y),
        "KeyU" => Some(VirtualKeyCode::U),
        "KeyI" => Some(VirtualKeyCode::I),
        "KeyO" => Some(VirtualKeyCode::O),
        "KeyP" => Some(VirtualKeyCode::P),
        "BracketLeft" => Some(VirtualKeyCode::LBracket),
        "BracketRight" => Some(VirtualKeyCode::RBracket),
        "Enter" => Some(VirtualKeyCode::Return),
        "ControlLeft" => Some(VirtualKeyCode::LControl),
        "KeyA" => Some(VirtualKeyCode::A),
        "KeyS" => Some(VirtualKeyCode::S),
        "KeyD" => Some(VirtualKeyCode::D),
        "KeyF" => Some(VirtualKeyCode::F),
        "KeyG" => Some(VirtualKeyCode::G),
        "KeyH" => Some(VirtualKeyCode::H),
        "KeyJ" => Some(VirtualKeyCode::J),
        "KeyK" => Some(VirtualKeyCode::K),
        "KeyL" => Some(VirtualKeyCode::L),
        "Semicolon" => Some(VirtualKeyCode::Semicolon),
        "Quote" => Some(VirtualKeyCode::Apostrophe),
        "Backquote" => Some(VirtualKeyCode::Grave),
        "ShiftLeft" => Some(VirtualKeyCode::LShift),
        "Backslash" => Some(VirtualKeyCode::Backslash),
        "KeyZ" => Some(VirtualKeyCode::Z),
        "KeyX" => Some(VirtualKeyCode::X),
        "KeyC" => Some(VirtualKeyCode::C),
        "KeyV" => Some(VirtualKeyCode::V),
        "KeyB" => Some(VirtualKeyCode::B),
        "KeyN" => Some(VirtualKeyCode::N),
        "KeyM" => Some(VirtualKeyCode::M),
        "Comma" => Some(VirtualKeyCode::Comma),
        "Period" => Some(VirtualKeyCode::Period),
        "Slash" => Some(VirtualKeyCode::Slash),
        "ShiftRight" => Some(VirtualKeyCode::RShift),
        "NumpadMultiply" => Some(VirtualKeyCode::NumpadMultiply),
        "AltLeft" => Some(VirtualKeyCode::LAlt),
        "Space" => Some(VirtualKeyCode::Space),
        "CapsLock" => Some(VirtualKeyCode::Capital),
        "F1" => Some(VirtualKeyCode::F1),
        "F2" => Some(VirtualKeyCode::F2),
        "F3" => Some(VirtualKeyCode::F3),
        "F4" => Some(VirtualKeyCode::F4),
        "F5" => Some(VirtualKeyCode::F5),
        "F6" => Some(VirtualKeyCode::F6),
        "F7" => Some(VirtualKeyCode::F7),
        "F8" => Some(VirtualKeyCode::F8),
        "F9" => Some(VirtualKeyCode::F9),
        "F10" => Some(VirtualKeyCode::F10),
        "Pause" => Some(VirtualKeyCode::PauseBreak),
        "ScrollLock" => Some(VirtualKeyCode::ScrollLock),
        "Numpad7" => Some(VirtualKeyCode::Numpad7),
        "Numpad8" => Some(VirtualKeyCode::Numpad8),
        "Numpad9" => Some(VirtualKeyCode::Numpad9),
        "NumpadSubtract" => Some(VirtualKeyCode::NumpadSubtract),
        "Numpad4" => Some(VirtualKeyCode::Numpad4),
        "Numpad5" => Some(VirtualKeyCode::Numpad5),
        "Numpad6" => Some(VirtualKeyCode::Numpad6),
        "NumpadAdd" => Some(VirtualKeyCode::NumpadAdd),
        "Numpad1" => Some(VirtualKeyCode::Numpad1),
        "Numpad2" => Some(VirtualKeyCode::Numpad2),
        "Numpad3" => Some(VirtualKeyCode::Numpad3),
        "Numpad0" => Some(VirtualKeyCode::Numpad0),
        "NumpadDecimal" => Some(VirtualKeyCode::NumpadDecimal),
        "PrintScreen" => Some(VirtualKeyCode::PrintScreen),
        "IntlBackslash" => Some(VirtualKeyCode::Backslash),
        "F11" => Some(VirtualKeyCode::F11),
        "F12" => Some(VirtualKeyCode::F12),
        "NumpadEqual" => Some(VirtualKeyCode::NumpadEquals),
        "F13" => Some(VirtualKeyCode::F13),
        "F14" => Some(VirtualKeyCode::F14),
        "F15" => Some(VirtualKeyCode::F15),
        "F16" => Some(VirtualKeyCode::F16),
        "F17" => Some(VirtualKeyCode::F17),
        "F18" => Some(VirtualKeyCode::F18),
        "F19" => Some(VirtualKeyCode::F19),
        "F20" => Some(VirtualKeyCode::F20),
        "F21" => Some(VirtualKeyCode::F21),
        "F22" => Some(VirtualKeyCode::F22),
        "F23" => Some(VirtualKeyCode::F23),
        "KanaMode" => Some(VirtualKeyCode::Kana),
        "Lang2" => None,
        "Lang1" => None,
        "IntlRo" => None,
        "F24" => Some(VirtualKeyCode::F24),
        "Convert" => Some(VirtualKeyCode::Convert),
        "NonConvert" => Some(VirtualKeyCode::NoConvert),
        "IntlYen" => Some(VirtualKeyCode::Yen),
        "NumpadComma" => Some(VirtualKeyCode::NumpadComma),
        "Paste" => Some(VirtualKeyCode::Paste),
        "MediaTrackPrevious" => Some(VirtualKeyCode::PrevTrack),
        "Cut" => Some(VirtualKeyCode::Cut),
        "Copy" => Some(VirtualKeyCode::Copy),
        "MediaTrackNext" => Some(VirtualKeyCode::NextTrack),
        "NumpadEnter" => Some(VirtualKeyCode::NumpadEnter),
        "ControlRight" => Some(VirtualKeyCode::RControl),
        "AudioVolumeMute" => Some(VirtualKeyCode::Mute),
        "MediaPlayPause" => Some(VirtualKeyCode::PlayPause),
        "MediaStop" => Some(VirtualKeyCode::MediaStop),
        "VolumeDown" => Some(VirtualKeyCode::VolumeDown),
        "AudioVolumeDown" => Some(VirtualKeyCode::VolumeDown),
        "VolumeUp" => Some(VirtualKeyCode::VolumeUp),
        "AudioVolumeUp" => Some(VirtualKeyCode::VolumeUp),
        "BrowserHome" => Some(VirtualKeyCode::WebHome),
        "NumpadDivide" => Some(VirtualKeyCode::NumpadDivide),
        "AltRight" => Some(VirtualKeyCode::RAlt),
        "NumLock" => Some(VirtualKeyCode::Numlock),
        "Home" => Some(VirtualKeyCode::Home),
        "ArrowUp" => Some(VirtualKeyCode::Up),
        "PageUp" => Some(VirtualKeyCode::PageUp),
        "ArrowLeft" => Some(VirtualKeyCode::Left),
        "ArrowRight" => Some(VirtualKeyCode::Right),
        "End" => Some(VirtualKeyCode::End),
        "ArrowDown" => Some(VirtualKeyCode::Down),
        "PageDown" => Some(VirtualKeyCode::PageDown),
        "Insert" => Some(VirtualKeyCode::Insert),
        "Delete" => Some(VirtualKeyCode::Delete),
        "OSLeft" => Some(VirtualKeyCode::LWin),
        "MetaLeft" => Some(VirtualKeyCode::LWin),
        "OSRight" => Some(VirtualKeyCode::RWin),
        "MetaRight" => Some(VirtualKeyCode::RWin),
        "ContextMenu" => None,
        "Power" => Some(VirtualKeyCode::Power),
        "BrowserSearch" => Some(VirtualKeyCode::WebSearch),
        "BrowserFavorites" => Some(VirtualKeyCode::WebFavorites),
        "BrowserRefresh" => Some(VirtualKeyCode::WebRefresh),
        "BrowserStop" => Some(VirtualKeyCode::Stop),
        "BrowserForward" => Some(VirtualKeyCode::WebForward),
        "BrowserBack" => Some(VirtualKeyCode::WebBack),
        "LaunchMail" => Some(VirtualKeyCode::Mail),
        "MediaSelect" => Some(VirtualKeyCode::MediaSelect),
        _ => None
    }
}

pub struct WindowHelperWeb<UserEventType>
where
    UserEventType: 'static
{
    redraw_pending: RefCell<Option<WebPending>>,
    redraw_request_action: Option<Box<RefCell<dyn FnMut() -> WebPending>>>,
    post_user_event_action: Option<Rc<RefCell<UserEventSenderActionType<UserEventType>>>>,
    terminate_loop_action: Option<Box<dyn FnOnce()>>,
    canvas: WebCanvasElement,
    document: WebDocument,
    window: WebWindow
}

impl<UserEventType: 'static> WindowHelperWeb<UserEventType>
{
    fn new(canvas: WebCanvasElement, document: WebDocument, window: WebWindow) -> Self
    {
        Self {
            redraw_pending: RefCell::new(None),
            redraw_request_action: None,
            post_user_event_action: None,
            terminate_loop_action: None,
            canvas,
            document,
            window
        }
    }

    pub fn set_redraw_request_action<F>(&mut self, redraw_request_action: F)
    where
        F: FnMut() -> WebPending + 'static
    {
        self.redraw_request_action = Some(Box::new(RefCell::new(redraw_request_action)));
    }

    pub fn set_post_user_event_action<F>(&mut self, post_user_event_action: F)
    where
        F: FnMut(UserEventType) -> Result<(), BacktraceError<ErrorMessage>> + 'static
    {
        self.post_user_event_action = Some(Rc::new(RefCell::new(post_user_event_action)));
    }

    pub fn set_terminate_loop_action<F>(&mut self, terminate_loop_action: F)
    where
        F: FnOnce() + 'static
    {
        self.terminate_loop_action = Some(Box::new(terminate_loop_action));
    }

    pub fn clear_redraw_pending_flag(&self)
    {
        if let Some(pending) = self.redraw_pending.borrow_mut().deref_mut() {
            pending.mark_as_triggered()
        }
        self.redraw_pending.replace(None);
    }

    pub fn terminate_loop(&mut self)
    {
        self.redraw_pending.replace(None);
        self.redraw_request_action = None;
        if let Some(action) = self.terminate_loop_action.take() {
            action();
        }
    }

    pub fn set_icon_from_rgba_pixels<S>(
        &self,
        _data: Vec<u8>,
        _size: S
    ) -> Result<(), BacktraceError<ErrorMessage>>
    where
        S: Into<Vector2<u32>>
    {
        // Do nothing
        Err(ErrorMessage::msg("Cannot set icon for WebCanvas"))
    }

    pub fn set_cursor_visible(&self, visible: bool)
    {
        if visible {
            self.canvas.set_cursor(WebCursorType::Auto);
        } else {
            self.canvas.set_cursor(WebCursorType::None);
        }
    }

    pub fn set_cursor_grab(
        &self,
        grabbed: bool
    ) -> Result<(), BacktraceError<ErrorMessage>>
    {
        if grabbed {
            self.canvas.request_pointer_lock();
        } else {
            self.window.document().unwrap().exit_pointer_lock();
        }

        Ok(())
    }

    pub fn set_resizable(&self, _resizable: bool)
    {
        // Do nothing
    }

    #[inline]
    pub fn request_redraw(&self)
    {
        if self.redraw_request_action.borrow().is_none() {
            log::warn!("Ignoring call to request_redraw() in invalid state");
            return;
        }

        if self.redraw_pending.borrow().is_none() {
            self.redraw_pending.replace(Some(self
                .redraw_request_action
                .as_ref()
                .unwrap()
                .deref()
                .borrow_mut()()));
        }
    }

    pub fn set_title(&self, title: &str)
    {
        self.window.document().unwrap().set_title(title);
    }

    pub fn set_fullscreen_mode(&self, mode: WindowFullscreenMode)
    {
        match mode {
            WindowFullscreenMode::Windowed => {
                self.document.exit_fullscreen();
            }
            WindowFullscreenMode::FullscreenBorderless => {
                self.canvas.request_fullscreen();
            }
        }
    }

    pub fn set_size_pixels<S: Into<Vector2<u32>>>(&self, _size: S)
    {
        // Do nothing
    }

    pub fn set_position_pixels<P: Into<Vector2<i32>>>(&self, _position: P)
    {
        // Do nothing
    }

    pub fn set_size_scaled_pixels<S: Into<Vector2<f32>>>(&self, _size: S)
    {
        // Do nothing
    }

    pub fn set_position_scaled_pixels<P: Into<Vector2<f32>>>(&self, _position: P)
    {
        // Do nothing
    }

    #[inline]
    #[must_use]
    pub fn get_scale_factor(&self) -> f64
    {
        self.window.device_pixel_ratio()
    }

    pub fn create_user_event_sender(&self) -> UserEventSender<UserEventType>
    {
        UserEventSender::new(UserEventSenderWeb::new(
            self.post_user_event_action.as_ref().unwrap().clone()
        ))
    }
}

type UserEventSenderActionType<UserEventType> =
    dyn FnMut(UserEventType) -> Result<(), BacktraceError<ErrorMessage>>;

#[derive(Clone)]
pub struct UserEventSenderWeb<UserEventType>
where
    UserEventType: 'static
{
    action: Rc<RefCell<UserEventSenderActionType<UserEventType>>>
}

impl<UserEventType: 'static> UserEventSenderWeb<UserEventType>
{
    fn new(action: Rc<RefCell<UserEventSenderActionType<UserEventType>>>) -> Self
    {
        Self { action }
    }

    #[inline]
    pub fn send_event(&self, event: UserEventType) -> Result<(), EventLoopSendError>
    {
        self.action.borrow_mut()(event).unwrap();
        Ok(())
    }
}

pub struct WebCanvasImpl<UserEventType>
where
    UserEventType: 'static
{
    user_event_queue: Vec<UserEventType>,
    event_listeners_to_clean_up: Rc<RefCell<Vec<WebPending>>>
}

impl<UserEventType: 'static> WebCanvasImpl<UserEventType>
{
    pub fn new<S, H>(
        element_id: S,
        handler: H,
        _options: Option<WebCanvasAttachOptions>
    ) -> Result<Self, BacktraceError<ErrorMessage>>
    where
        S: AsRef<str>,
        H: WindowHandler<UserEventType> + 'static
    {
        let window = WebWindow::new()?;
        let document = window.document()?;

        let canvas = WebCanvasElement::new_by_id(&element_id)?;

        let initial_size_scaled = canvas.html_element().element().dimensions();
        let initial_dpr = window.device_pixel_ratio();

        let initial_size_unscaled =
            (initial_size_scaled * initial_dpr).round().into_u32();

        canvas.set_buffer_dimensions(&initial_size_unscaled);

        // Needed to ensure we can get keyboard focus
        canvas.set_tab_index(0);

        let mut event_listeners_to_clean_up = Vec::new();
        let is_pointer_locked = Rc::new(Cell::new(false));

        let renderer =
            GLRenderer::new_for_web_canvas_by_id(initial_size_unscaled, &element_id)
                .map_err(|err| {
                    ErrorMessage::msg_with_cause("Failed to create renderer", err)
                })?;

        let handler = Rc::new(RefCell::new(DrawingWindowHandler::new(handler, renderer)));

        let helper = {
            Rc::new(RefCell::new(WindowHelper::new(WindowHelperWeb::new(
                canvas.clone(),
                document.clone(),
                window.clone()
            ))))
        };

        {
            let helper_inner = helper.clone();
            let window = window.clone();
            let handler = handler.clone();

            let frame_callback = RefCell::new(Closure::wrap(Box::new(move || {
                helper_inner
                    .borrow_mut()
                    .inner()
                    .clear_redraw_pending_flag();
                handler
                    .borrow_mut()
                    .on_draw(helper_inner.borrow_mut().deref_mut());
            })
                as Box<dyn FnMut()>));

            let redraw_request_action =
                move || window.request_animation_frame(&frame_callback).unwrap();

            helper
                .borrow_mut()
                .inner()
                .set_redraw_request_action(redraw_request_action);
        }

        {
            let user_event_queue = Rc::new(RefCell::new(Vec::new()));
            let user_event_callback_pending = Rc::new(RefCell::new(None));
            let window = window.clone();

            let callback = {
                let handler = handler.clone();
                let helper = helper.clone();
                let user_event_queue = user_event_queue.clone();
                let user_event_callback_pending = user_event_callback_pending.clone();

                RefCell::new(Closure::wrap(Box::new(move || {
                    let user_event_callback_pending: Option<WebPending> =
                        user_event_callback_pending.take();
                    user_event_callback_pending.unwrap().mark_as_triggered();

                    let mut pending_events = Vec::new();
                    std::mem::swap(
                        &mut pending_events,
                        user_event_queue.borrow_mut().deref_mut()
                    );
                    pending_events.drain(..).for_each(|event| {
                        handler
                            .borrow_mut()
                            .on_user_event(helper.borrow_mut().deref_mut(), event)
                    });
                }) as Box<dyn FnMut()>))
            };

            helper
                .borrow_mut()
                .inner()
                .set_post_user_event_action(move |event| {
                    user_event_queue.borrow_mut().push(event);

                    if user_event_callback_pending.deref().borrow().is_none() {
                        user_event_callback_pending
                            .replace(Some(window.set_timeout_immediate(&callback)?));
                    }

                    Ok(())
                })
        }

        let canvas_event_target = canvas
            .html_element()
            .element()
            .clone()
            .dyn_into_event_target()?;

        match canvas_event_target
            .register_event_listener_mouse("contextmenu", move |event| {
                event.prevent_default()
            }) {
            Ok(listener) => event_listeners_to_clean_up.push(listener),
            Err(err) => {
                log::error!("Failed to register context menu event listener: {:?}", err)
            }
        }

        {
            let handler = handler.clone();
            let helper = helper.clone();
            let window_inner = window.clone();
            let canvas = canvas.clone();

            event_listeners_to_clean_up.push(
                window
                    .dyn_into_event_target()?
                    .register_event_listener_void("resize", move || {
                        let size_scaled = canvas.html_element().element().dimensions();
                        let dpr = window_inner.device_pixel_ratio();

                        let size_unscaled = (size_scaled * dpr).round().into_u32();

                        canvas.set_buffer_dimensions(&size_unscaled);

                        handler
                            .borrow_mut()
                            .on_resize(helper.borrow_mut().deref_mut(), size_unscaled);

                        handler
                            .borrow_mut()
                            .on_draw(helper.borrow_mut().deref_mut());
                    })?
            );
        }

        {
            let handler = handler.clone();
            let helper = helper.clone();
            let canvas = canvas.clone();
            let is_pointer_locked = is_pointer_locked.clone();

            event_listeners_to_clean_up.push(
                document
                    .clone()
                    .dyn_into_event_target()?
                    .register_event_listener_void("pointerlockchange", move || {
                        let mouse_grabbed = canvas.is_pointer_lock_active();

                        is_pointer_locked.set(mouse_grabbed);

                        handler.borrow_mut().on_mouse_grab_status_changed(
                            helper.borrow_mut().deref_mut(),
                            mouse_grabbed
                        );
                    })?
            );
        }

        {
            let handler = handler.clone();
            let helper = helper.clone();

            event_listeners_to_clean_up.push(
                document
                    .dyn_into_event_target()?
                    .register_event_listener_void("fullscreenchange", move || {
                        let fullscreen = canvas.is_fullscreen_active();

                        handler.borrow_mut().on_fullscreen_status_changed(
                            helper.borrow_mut().deref_mut(),
                            fullscreen
                        );
                    })?
            );
        }

        {
            let handler = handler.clone();
            let helper = helper.clone();

            event_listeners_to_clean_up.push(
                canvas_event_target.register_event_listener_mouse(
                    "mousemove",
                    move |event| {
                        let position = if is_pointer_locked.get() {
                            Vector2::new(event.movement_x(), event.movement_y())
                                .into_f32()
                        } else {
                            Vector2::new(event.offset_x(), event.offset_y()).into_f32()
                        };

                        handler
                            .borrow_mut()
                            .on_mouse_move(helper.borrow_mut().deref_mut(), position);
                    }
                )?
            );
        }

        {
            let handler = handler.clone();
            let helper = helper.clone();

            event_listeners_to_clean_up.push(
                canvas_event_target.register_event_listener_mouse(
                    "mousedown",
                    move |event| match mouse_button_from_event(&event) {
                        None => {
                            log::error!(
                                "Mouse down: Unknown mouse button {}",
                                event.button()
                            )
                        }
                        Some(button) => handler
                            .borrow_mut()
                            .on_mouse_button_down(helper.borrow_mut().deref_mut(), button)
                    }
                )?
            );
        }

        {
            let handler = handler.clone();
            let helper = helper.clone();

            event_listeners_to_clean_up.push(
                canvas_event_target.register_event_listener_mouse(
                    "mouseup",
                    move |event| match mouse_button_from_event(&event) {
                        None => {
                            log::error!(
                                "Mouse up: Unknown mouse button {}",
                                event.button()
                            )
                        }
                        Some(button) => handler
                            .borrow_mut()
                            .on_mouse_button_up(helper.borrow_mut().deref_mut(), button)
                    }
                )?
            );
        }

        {
            let handler = handler.clone();
            let helper = helper.clone();

            event_listeners_to_clean_up.push(
                canvas_event_target.register_event_listener_keyboard(
                    "keydown",
                    move |event| {
                        let code : String = event.code();
                        let virtual_key_code = key_code_from_web(code.as_str());

                        if let Some(virtual_key_code) = virtual_key_code {
                            let scancode = virtual_key_code.get_scan_code();

                            if let Some(scancode) = scancode {
                                handler.borrow_mut().on_key_down(
                                    helper.borrow_mut().deref_mut(),
                                    Some(virtual_key_code),
                                    scancode
                                );
                            } else {
                                log::warn!(
                                    "Ignoring key {:?} due to unknown scancode",
                                    virtual_key_code
                                );
                            }
                        } else {
                            log::warn!("Ignoring unknown key code {}", code);
                        }

                        // TODO invoke char typed API (regardless of repeat)

                        log::info!(
                            "RRDEBUG key='{}' code='{}'",
                            event.key(),
                            event.code()
                        );

                        return true;
                    }
                )?
            );
        }

        let terminated = Rc::new(Cell::new(false));
        let event_listeners_to_clean_up =
            Rc::new(RefCell::new(event_listeners_to_clean_up));

        {
            let terminated = terminated.clone();
            let event_listeners_to_clean_up = event_listeners_to_clean_up.clone();

            helper
                .borrow_mut()
                .inner()
                .set_terminate_loop_action(move || {
                    log::info!("Terminating event loop");
                    terminated.set(true);
                    event_listeners_to_clean_up.borrow_mut().clear();
                });
        }

        log::info!(
            "Initial scaled canvas size: {:?}, dpr {}, unscaled: {:?}",
            initial_size_scaled,
            initial_dpr,
            initial_size_unscaled
        );

        handler.borrow_mut().on_start(
            helper.borrow_mut().deref_mut(),
            WindowStartupInfo::new(initial_size_unscaled, initial_dpr)
        );

        if !terminated.get() {
            handler
                .borrow_mut()
                .on_draw(helper.borrow_mut().deref_mut());
        }

        // TODO https://stackoverflow.com/questions/4470417/how-do-i-consume-a-key-event-in-javascript-so-that-it-doesnt-propagate

        // TODO what happens when web-sys APIs don't exist?

        // TODO MODIFIER key events
        // TODO all remaining events

        Ok(WebCanvasImpl {
            user_event_queue: Vec::new(),
            event_listeners_to_clean_up
        })
    }
}

impl<UserEventType: 'static> Drop for WebCanvasImpl<UserEventType>
{
    fn drop(&mut self)
    {
        log::info!("Unregistering WebCanvasImpl")
    }
}

fn mouse_button_from_event(event: &MouseEvent) -> Option<MouseButton>
{
    let button: i16 = event.button();
    match button {
        0 => Some(MouseButton::Left),
        1 => Some(MouseButton::Middle),
        2 => Some(MouseButton::Right),
        _ => Some(MouseButton::Other(button.try_into().unwrap()))
    }
}
