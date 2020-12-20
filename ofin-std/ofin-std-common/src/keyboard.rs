//! Common keyboard functionality
use enigo::KeyboardControllable;
use lazy_static::lazy_static;
use ofin_types::OfinChar;
use std::sync::Mutex;

lazy_static! {
    static ref DRIVER: Mutex<enigo::Enigo> = Mutex::new(enigo::Enigo::new());
}

pub fn click(key: OfinChar) {
    DRIVER.lock().unwrap().key_click(enigo::Key::Layout(*key));
}
