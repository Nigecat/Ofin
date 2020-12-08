use super::pattern::TranspilePattern;

lazy_static! {
    static ref TRANSPILE_PATTERNS: [TranspilePattern<'static>; 0] = [];
}

/// Transpile a script into rust code
pub fn transpile(mut script: String) -> String {
    TRANSPILE_PATTERNS.iter().for_each(|p| {
        script = p.replace(&script);
    });

    // Wrap the script in a rust main function
    script = format!("fn main() {{ {} }}", script);

    trace!("Finished transpiling with code:\n{}", script);

    script
}
