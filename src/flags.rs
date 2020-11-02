bitflags! {
    pub struct Flags: u32 {
        /// Whether variable declarations are case-sensitive
        const CASE_SENSITIVE = 0b00000001;
    }
}
