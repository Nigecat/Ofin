pub fn find_end_at(slice: &str, at: usize, pat: &'static str) -> Option<usize> {
    slice[at..].find(pat).map(|i| at + i + pat.len())
}
