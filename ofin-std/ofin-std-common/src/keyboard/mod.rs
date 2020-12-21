//! Common keyboard functionality
mod key;
use enigo::KeyboardControllable;
pub use key::Key;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref DRIVER: Mutex<enigo::Enigo> = Mutex::new(enigo::Enigo::new());
}

/// Press a char on the keyboard
///
/// # Arguments
///
/// * `key` - The character to click
pub fn click<S: Into<char>>(key: S) {
    DRIVER
        .lock()
        .unwrap()
        .key_click(enigo::Key::Layout(key.into()));
}

/// Send some text (equivalent to calling [click] for every character in the text)
///
/// # Arguments
///
/// * `text` - The text to send
pub fn send<S: Into<String>>(text: S) {
    DRIVER.lock().unwrap().key_sequence(&text.into());
}

/// Hold down a control key, this is not let go of until [keyUp] is called
///
/// # Arguments
///
/// * `key` - The control key to hold down, must be a valid [Key]
pub fn keyDown<S: Into<String>>(key: S) {
    DRIVER.lock().unwrap().key_down(key.into().into_key());
}

/// Let go of a control key, this will do nothing if [keyDown] has not been called
///
/// # Arguments
///
/// * `key` - The control key to let go of, must be a valid [Key]
pub fn keyUp<S: Into<String>>(key: S) {
    DRIVER.lock().unwrap().key_up(key.into().into_key());
}
