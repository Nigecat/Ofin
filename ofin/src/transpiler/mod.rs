use regex::Regex;

macro_rules! regex {
    ($pattern:expr) => {
        Regex::new($pattern).unwrap()
    };
}

lazy_static! {
    static ref REGEX_PATTERNS: [(Regex, &'static str); 0] = [];
}

pub fn transpile(mut script: String) -> String {
    REGEX_PATTERNS.iter().for_each(|r| {
        let regex: &Regex = &r.0;
        let replace: &'static str = r.1;
        trace!("Running regex: {:?} -> {}", regex, replace);
        script = regex.replace_all(&script, replace).to_string();
    });

    script
}
