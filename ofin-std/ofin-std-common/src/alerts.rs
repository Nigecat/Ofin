//! Various functions for creating alerts

use ofin_types::OfinString;
use notify_rust::Notification;
use msgbox::IconType;

fn create_msgbox(title: OfinString, content: OfinString, icon: IconType) {
    msgbox::create(&String::from(title), &String::from(content), icon).unwrap();
}

/// Create an info message box
/// 
/// # Arguments
/// 
/// * `title` - The title of the message box
/// * `content` - The contents of the message box
pub fn infoBox(title: OfinString, content: OfinString) {
    create_msgbox(title, content, IconType::Info);
}

/// Create an error message box
/// 
/// # Arguments
/// 
/// * `title` - The title of the message box
/// * `content` - The contents of the message box
pub fn errorBox(title: OfinString, content: OfinString) {
    create_msgbox(title, content, IconType::Error);
}

/// Create a toast notification
/// 
/// # Arguments
/// 
/// * `title` - The title of the notification
/// * `content` - The body of the notificaton
pub fn toast(title: OfinString, content: OfinString) {
    Notification::new()
        .summary(&String::from(title))
        .body(&String::from(content))
        .show()
        .unwrap();
}
