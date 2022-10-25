use egui::{Event, Key, Pos2, RawInput};
use sdl2_sys::{SDL_Event, SDL_Scancode, SDL_BUTTON_LEFT, SDL_BUTTON_MIDDLE, SDL_BUTTON_RIGHT};
pub struct Platform {
    pub raw_input: RawInput,
}

impl Platform {
    pub fn handle_event(event: *mut SDL_Event) -> Option<Platform> {
        let mut raw_input = egui::RawInput {
            ..Default::default()
        };
        let mut used = false;
        unsafe {
            match (*event).type_ {
                1025 /* SDL_EventType::SDL_MOUSEBUTTONDOWN */ => {
                    let event = &(*event).button;
                    let mouse_btn = match event.button as u32 {
                        SDL_BUTTON_LEFT => Some(egui::PointerButton::Primary),
                        SDL_BUTTON_MIDDLE => Some(egui::PointerButton::Middle),
                        SDL_BUTTON_RIGHT => Some(egui::PointerButton::Secondary),
                        _ => None,
                    };
                    if let Some(button) = mouse_btn {
                        raw_input.events.push(egui::Event::PointerButton {
                            pos: egui::Pos2::new(event.x as f32, event.y as f32),
                            button,
                            pressed: true,
                            modifiers: Default::default()
                        });
                    };
                }
                1026 /*SDL_EventType::SDL_MOUSEBUTTONUP*/ => {
                    let event = &(*event).button;
                    let mouse_btn = match event.button as u32 {
                        SDL_BUTTON_LEFT => Some(egui::PointerButton::Primary),
                        SDL_BUTTON_MIDDLE => Some(egui::PointerButton::Middle),
                        SDL_BUTTON_RIGHT => Some(egui::PointerButton::Secondary),
                        _ => None,
                    };
                    if let Some(button) = mouse_btn {
                        raw_input.events.push(Event::PointerButton {
                            pos: Pos2::new(event.x as f32, event.y as f32),
                            button,
                            pressed: false,
                            modifiers: Default::default()
                        });
                    };
                }
                1024 /*SDL_EventType::SDL_MOUSEMOTION*/ => {
                    let event = &(*event).motion;
                    raw_input.events.push(Event::PointerMoved(Pos2::new(event.x as f32, event.y as f32)));
                }
                768 /*SDL_EventType::SDL_KEYDOWN*/ => {
                    let event = &(*event).key;
                    // Unimplemented, ran out of time
                }
                _ => {}
            };
        }
        if !raw_input.events.is_empty() {
            Some(Platform { raw_input })
        } else {
            None
        }
    }
}

fn sdl_to_egui_key(key: sdl2_sys::SDL_Keysym) -> Option<egui::Key> {
    use SDL_Scancode::*;
    Some(match key.scancode {
        SDL_SCANCODE_LEFT => Key::ArrowLeft,
        SDL_SCANCODE_RIGHT => Key::ArrowRight,
        SDL_SCANCODE_UP => Key::ArrowUp,
        SDL_SCANCODE_DOWN => Key::ArrowDown,

        SDL_SCANCODE_ESCAPE => Key::Escape,
        SDL_SCANCODE_TAB => Key::Tab,
        SDL_SCANCODE_BACKSPACE => Key::Backspace,
        SDL_SCANCODE_SPACE => Key::Space,
        SDL_SCANCODE_RETURN => Key::Enter,

        SDL_SCANCODE_INSERT => Key::Insert,
        SDL_SCANCODE_HOME => Key::Home,
        SDL_SCANCODE_DELETE => Key::Delete,
        SDL_SCANCODE_END => Key::End,
        SDL_SCANCODE_PAGEDOWN => Key::PageDown,
        SDL_SCANCODE_PAGEUP => Key::PageUp,

        SDL_SCANCODE_KP_0 | SDL_SCANCODE_0 => Key::Num0,
        SDL_SCANCODE_KP_1 | SDL_SCANCODE_1 => Key::Num1,
        SDL_SCANCODE_KP_2 | SDL_SCANCODE_2 => Key::Num2,
        SDL_SCANCODE_KP_3 | SDL_SCANCODE_3 => Key::Num3,
        SDL_SCANCODE_KP_4 | SDL_SCANCODE_4 => Key::Num4,
        SDL_SCANCODE_KP_5 | SDL_SCANCODE_5 => Key::Num5,
        SDL_SCANCODE_KP_6 | SDL_SCANCODE_6 => Key::Num6,
        SDL_SCANCODE_KP_7 | SDL_SCANCODE_7 => Key::Num7,
        SDL_SCANCODE_KP_8 | SDL_SCANCODE_8 => Key::Num8,
        SDL_SCANCODE_KP_9 | SDL_SCANCODE_9 => Key::Num9,

        SDL_SCANCODE_A => Key::A,
        SDL_SCANCODE_B => Key::B,
        SDL_SCANCODE_C => Key::C,
        SDL_SCANCODE_D => Key::D,
        SDL_SCANCODE_E => Key::E,
        SDL_SCANCODE_F => Key::F,
        SDL_SCANCODE_G => Key::G,
        SDL_SCANCODE_H => Key::H,
        SDL_SCANCODE_I => Key::I,
        SDL_SCANCODE_J => Key::J,
        SDL_SCANCODE_K => Key::K,
        SDL_SCANCODE_L => Key::L,
        SDL_SCANCODE_M => Key::M,
        SDL_SCANCODE_N => Key::N,
        SDL_SCANCODE_O => Key::O,
        SDL_SCANCODE_P => Key::P,
        SDL_SCANCODE_Q => Key::Q,
        SDL_SCANCODE_R => Key::R,
        SDL_SCANCODE_S => Key::S,
        SDL_SCANCODE_T => Key::T,
        SDL_SCANCODE_U => Key::U,
        SDL_SCANCODE_V => Key::V,
        SDL_SCANCODE_W => Key::W,
        SDL_SCANCODE_X => Key::X,
        SDL_SCANCODE_Y => Key::Y,
        SDL_SCANCODE_Z => Key::Z,

        _ => return None,
    })
}
