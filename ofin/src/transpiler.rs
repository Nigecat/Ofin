use super::OfinError;
use crate::lexer::TokenStream;

/// Transpile a script into rust code
pub fn transpile(tokens: TokenStream) -> Result<String, OfinError> {
    let mut script = String::new();

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
