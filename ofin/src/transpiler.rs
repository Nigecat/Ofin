use super::pattern::TranspilePattern;

lazy_static! {
    static ref TRANSPILE_PATTERNS: [TranspilePattern; 0] = [];
}

/// Transpile a script into rust code
pub fn transpile(mut script: String) -> String {
    TRANSPILE_PATTERNS.iter().for_each(|p| {
        script = p.replace(&script);
    });

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

    script
}
