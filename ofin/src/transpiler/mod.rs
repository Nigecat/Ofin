mod pattern;
mod rule;
use super::OfinError;
use crate::util;
use pattern::TranspilePattern;
use rule::BlockRule;

lazy_static! {
    static ref TRANSPILE_PATTERNS: [TranspilePattern; 1] = [
        // Convert `import <std/a/b>` to `use ofin_std::a::b`
        TranspilePattern::new(r#"import\s*<.*>"#, "use $1", Some("<.*>"), Some(|s| format!("ofin_{}", util::remove_last(util::remove_first(&s.replace("/", "::")).unwrap()).unwrap()) )),
        // Convert `"abc"` to `OfinString::new("abc")` to allow the compiler to convert the str into our type
        //TranspilePattern::new(r#"(")((?:(?!")[^\\]|(?:\\\\)*\\[^\\])*)(")"#, "OfinString::new($1)", Some(r#"(")((?:(?!")[^\\]|(?:\\\\)*\\[^\\])*)(")"#), None),
    ];
    static ref BLOCK_RULES: [BlockRule; 1] = [
        // Disallow the rust namespace identifier
        BlockRule::new("::", OfinError::SyntaxError),
    ];
}

/// Transpile a script into rust code
pub fn transpile(mut script: String) -> Result<String, OfinError> {
    for rule in BLOCK_RULES.iter() {
        trace!("Checking rule: {:?}", rule);
        rule.matches(&script)?;
    }

    for pattern in TRANSPILE_PATTERNS.iter() {
        trace!("Running pattern: {:?}", pattern);
        script = pattern.replace(&script);
    }

    // Wrap the script in a rust main function, link our standard library, and include the prelude
    script = format!(
        "
        extern crate ofin_std; 
        #[allow(unused_imports)]
        use ofin_std::prelude::*; 
        
        fn main() {{
            {}
        }}
    ",
        script
    );

    trace!("Finished transpiling with code:\n{}", script);

    Ok(script)
}
