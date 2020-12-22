//! Common keyboard functionality
mod key;
use enigo::KeyboardControllable;
pub use key::Key;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref DRIVER: Mutex<enigo::Enigo> = Mutex::new(enigo::Enigo::new());
}

/// Send some text
///
/// # Arguments
///
/// * `text` - The text to send
pub fn send<S: Into<String>>(text: S) {
    DRIVER.lock().unwrap().key_sequence(&text.into());
}

/// Hold down a control key, this is not let go of until [keyUp](keyUp) is called
///
/// # Arguments
///
/// * `key` - The control key to hold down, must be a valid [`Key`](trait.Key.html)
pub fn sendKeyDown<S: Into<String>>(key: S) {
    DRIVER.lock().unwrap().key_down(key.into().into_key());
}

/// Let go of a control key, this will do nothing if [keyDown](keyDown) has not been called
///
/// # Arguments
///
/// * `key` - The control key to let go of, must be a valid [`Key`](trait.Key.html)
pub fn sendKeyUp<S: Into<String>>(key: S) {
    DRIVER.lock().unwrap().key_up(key.into().into_key());
}
