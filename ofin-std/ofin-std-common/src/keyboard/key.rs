/// A (case-insensitive) control key
///
/// # Valid strings
///
///  - f1
///  - f2
///  - f3
///  - f4
///  - f5
///  - f6
///  - f7
///  - f8
///  - f9
///
///  - up_arrow
///  - down_arrow
///  - left_arrow
///  - right_arrow
///
///  - alt
///  - backspace
///  - capslock
///  - control
///  - delete
///  - end
///  - escape
///  - home
///  - meta
///  - option
///  - page_down
///  - page_up
///  - return
///  - shift
///  - space
///  - tab
pub trait Key {
    fn into_key(&self) -> enigo::Key;
}

impl<S: AsRef<str>> Key for S {
    fn into_key(&self) -> enigo::Key {
        match &*self.as_ref().to_lowercase() {
            "f1" => enigo::Key::F1,
            "f2" => enigo::Key::F2,
            "f3" => enigo::Key::F3,
            "f4" => enigo::Key::F4,
            "f5" => enigo::Key::F5,
            "f6" => enigo::Key::F6,
            "f7" => enigo::Key::F7,
            "f8" => enigo::Key::F8,
            "f9" => enigo::Key::F9,

            "up_arrow" => enigo::Key::UpArrow,
            "down_arrow" => enigo::Key::DownArrow,
            "left_arrow" => enigo::Key::LeftArrow,
            "right_arrow" => enigo::Key::RightArrow,

            "alt" => enigo::Key::Alt,
            "backspace" => enigo::Key::Backspace,
            "capslock" => enigo::Key::CapsLock,
            "control" => enigo::Key::Control,
            "delete" => enigo::Key::Delete,
            "end" => enigo::Key::End,
            "escape" => enigo::Key::Escape,
            "home" => enigo::Key::Home,
            "meta" => enigo::Key::Meta,
            "option" => enigo::Key::Option,
            "page_down" => enigo::Key::PageDown,
            "page_up" => enigo::Key::PageUp,
            "return" => enigo::Key::Return,
            "shift" => enigo::Key::Shift,
            "space" => enigo::Key::Space,
            "tab" => enigo::Key::Tab,

            _ => panic!("{} is not a valid control key", self.as_ref()),
        }
    }
}
