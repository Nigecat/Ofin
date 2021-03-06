fn main() {
    // Write compilation metadata
    built::write_built_file().expect("failed to acquire build-time information");
}
