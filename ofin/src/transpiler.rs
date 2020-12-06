use regex::Regex;

macro_rules! regex {
    ($pattern:expr) => {
        Regex::new($pattern).unwrap()
    };
}

lazy_static! {
    static ref REGEX_PATTERNS: [(Regex, &'static str); 1] =
        [(regex!(r#"(^|\s)print\("#), "println!("),];
}

/// Transpile a script into rust code
pub fn transpile(mut script: String) -> String {
    REGEX_PATTERNS.iter().for_each(|r| {
        let regex: &Regex = &r.0;
        let replace: &'static str = r.1;
        trace!("Running regex: {:?} -> {}", regex, replace);
        script = regex.replace_all(&script, replace).to_string();
    });

    // Wrap the script in a rust main function
    script = format!("fn main() {{ {} }}", script);

    trace!("Finished transpiling with code:\n{}", script);

    script
}
