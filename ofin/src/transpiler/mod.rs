mod pattern;
mod rule;
use super::OfinError;
use pattern::TranspilePattern;
use rule::BlockRule;

lazy_static! {
    static ref TRANSPILE_PATTERNS: [TranspilePattern; 0] = [];
    static ref BLOCK_RULES: [BlockRule; 0] = [];
}

/// Transpile a script into rust code
pub fn transpile(mut script: String) -> Result<String, OfinError> {
    for pattern in TRANSPILE_PATTERNS.iter() {
        trace!("Running pattern: {:?}", pattern);
        script = pattern.replace(&script);
    }

    for rule in BLOCK_RULES.iter() {
        trace!("Checking rule: {:?}", rule);
        rule.matches(&script)?;
    }

    // Wrap the script in a rust main function, link our standard library, and include the prelude
    script = format!(
        "
        extern crate ofin_std; 
        #[allow(unused_imports)]
        use ofin_std::prelude::*; 
        
        fn main() {{ {} }}
    ",
        script
    );

    trace!("Finished transpiling with code:\n{}", script);

    Ok(script)
}
