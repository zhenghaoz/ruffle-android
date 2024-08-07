use android_activity::input::Keycode as Android;
use ruffle_core::events::KeyCode as Ruffle;

pub fn android_keycode_to_ruffle(android: Android) -> Option<(Ruffle, Option<char>)> {
    Some(match android {
        Android::DpadUp => (Ruffle::Up, None),
        Android::DpadDown => (Ruffle::Down, None),
        Android::DpadLeft => (Ruffle::Left, None),
        Android::DpadRight => (Ruffle::Right, None),
        Android::Keycode0 => (Ruffle::Key0, Some('0')),
        Android::Keycode1 => (Ruffle::Key1, Some('1')),
        Android::Keycode2 => (Ruffle::Key2, Some('2')),
        Android::Keycode3 => (Ruffle::Key3, Some('3')),
        Android::Keycode4 => (Ruffle::Key4, Some('4')),
        Android::Keycode5 => (Ruffle::Key5, Some('5')),
        Android::Keycode6 => (Ruffle::Key6, Some('6')),
        Android::Keycode7 => (Ruffle::Key7, Some('7')),
        Android::Keycode8 => (Ruffle::Key8, Some('8')),
        Android::Keycode9 => (Ruffle::Key9, Some('9')),
        Android::A => (Ruffle::A, Some('a')),
        Android::B => (Ruffle::B, Some('b')),
        Android::C => (Ruffle::C, Some('c')),
        Android::D => (Ruffle::D, Some('d')),
        Android::E => (Ruffle::E, Some('e')),
        Android::F => (Ruffle::F, Some('f')),
        Android::G => (Ruffle::G, Some('g')),
        Android::H => (Ruffle::H, Some('h')),
        Android::I => (Ruffle::I, Some('i')),
        Android::J => (Ruffle::J, Some('j')),
        Android::K => (Ruffle::K, Some('k')),
        Android::L => (Ruffle::L, Some('l')),
        Android::M => (Ruffle::M, Some('m')),
        Android::N => (Ruffle::N, Some('n')),
        Android::O => (Ruffle::O, Some('o')),
        Android::P => (Ruffle::P, Some('p')),
        Android::Q => (Ruffle::Q, Some('q')),
        Android::R => (Ruffle::R, Some('r')),
        Android::S => (Ruffle::S, Some('s')),
        Android::T => (Ruffle::T, Some('t')),
        Android::U => (Ruffle::U, Some('u')),
        Android::V => (Ruffle::V, Some('v')),
        Android::W => (Ruffle::W, Some('w')),
        Android::X => (Ruffle::X, Some('x')),
        Android::Y => (Ruffle::Y, Some('y')),
        Android::Z => (Ruffle::Z, Some('z')),
        Android::Comma => (Ruffle::Comma, Some(',')),
        Android::Period => (Ruffle::Period, Some('.')),
        Android::AltLeft => (Ruffle::Alt, None),
        Android::AltRight => (Ruffle::Alt, None),
        Android::ShiftLeft => (Ruffle::Shift, None),
        Android::ShiftRight => (Ruffle::Shift, None),
        Android::Tab => (Ruffle::Tab, Some('\t')),
        Android::Space => (Ruffle::Space, Some(' ')),
        Android::Enter => (Ruffle::Return, Some(13 as char)),
        Android::Del => (Ruffle::Backspace, Some(8 as char)),
        Android::Grave => (Ruffle::Grave, Some('`')),
        Android::Minus => (Ruffle::Minus, Some('-')),
        Android::Equals => (Ruffle::Equals, Some('=')),
        Android::LeftBracket => (Ruffle::LBracket, Some('(')),
        Android::RightBracket => (Ruffle::RBracket, Some(')')),
        Android::Backslash => (Ruffle::Backslash, Some('\\')),
        Android::Semicolon => (Ruffle::Semicolon, Some(';')),
        Android::Apostrophe => (Ruffle::Apostrophe, Some('\'')),
        Android::Slash => (Ruffle::Slash, Some('/')),
        Android::Plus => (Ruffle::Plus, Some('+')),
        Android::PageUp => (Ruffle::PgUp, None),
        Android::PageDown => (Ruffle::PgDown, None),
        Android::Escape => (Ruffle::Escape, None),
        Android::ForwardDel => (Ruffle::Delete, Some(127 as char)),
        Android::CtrlLeft => (Ruffle::Control, None),
        Android::CtrlRight => (Ruffle::Control, None),
        Android::CapsLock => (Ruffle::CapsLock, None),
        Android::ScrollLock => (Ruffle::ScrollLock, None),
        Android::Break => (Ruffle::Pause, None),
        Android::MoveHome => (Ruffle::Home, None),
        Android::MoveEnd => (Ruffle::End, None),
        Android::Insert => (Ruffle::Insert, None),
        Android::F1 => (Ruffle::F1, None),
        Android::F2 => (Ruffle::F2, None),
        Android::F3 => (Ruffle::F3, None),
        Android::F4 => (Ruffle::F4, None),
        Android::F5 => (Ruffle::F5, None),
        Android::F6 => (Ruffle::F6, None),
        Android::F7 => (Ruffle::F7, None),
        Android::F8 => (Ruffle::F8, None),
        Android::F9 => (Ruffle::F9, None),
        Android::F10 => (Ruffle::F10, None),
        Android::F11 => (Ruffle::F11, None),
        Android::F12 => (Ruffle::F12, None),
        Android::NumLock => (Ruffle::NumLock, None),
        Android::Numpad0 => (Ruffle::Numpad0, Some('0')),
        Android::Numpad1 => (Ruffle::Numpad1, Some('1')),
        Android::Numpad2 => (Ruffle::Numpad2, Some('2')),
        Android::Numpad3 => (Ruffle::Numpad3, Some('3')),
        Android::Numpad4 => (Ruffle::Numpad4, Some('4')),
        Android::Numpad5 => (Ruffle::Numpad5, Some('5')),
        Android::Numpad6 => (Ruffle::Numpad6, Some('6')),
        Android::Numpad7 => (Ruffle::Numpad7, Some('7')),
        Android::Numpad8 => (Ruffle::Numpad8, Some('8')),
        Android::Numpad9 => (Ruffle::Numpad9, Some('9')),
        Android::NumpadDivide => (Ruffle::NumpadSlash, Some('/')),
        Android::NumpadMultiply => (Ruffle::Multiply, Some('*')),
        Android::NumpadSubtract => (Ruffle::NumpadMinus, Some('-')),
        Android::NumpadAdd => (Ruffle::Plus, Some('+')),
        Android::NumpadDot => (Ruffle::NumpadPeriod, Some('.')),
        Android::NumpadComma => (Ruffle::Comma, Some(',')),
        Android::NumpadEnter => (Ruffle::NumpadEnter, Some(13 as char)),
        Android::NumpadEquals => (Ruffle::Equals, Some('=')),
        _ => return None,
    })
}
