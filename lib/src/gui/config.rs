use derive_more::Deref;

#[derive(Debug, Deref)]
pub struct Name(&'static str);

impl Default for Name {
    fn default() -> Self {
        Name("ofin")
    }
}

#[derive(Debug, Deref)]
pub struct Height(i32);

impl Default for Height {
    fn default() -> Self {
        Height(720)
    }
}

#[derive(Debug, Deref)]
pub struct Width(i32);

impl Default for Width {
    fn default() -> Self {
        Width(480)
    }
}

#[derive(Debug, Deref)]
pub struct Menu(iui::controls::WindowType);

impl Default for Menu {
    fn default() -> Self {
        Menu(iui::controls::WindowType::HasMenubar)
    }
}
