pub struct OfinString {
    inner: String,
}

impl OfinString {
    fn new<S: AsRef<str>>(string: S) -> Self {
        OfinString {
            inner: string.as_ref().to_string(),
        }
    }
}
