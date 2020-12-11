use super::OfinError;
use crate::lexer::TokenStream;

/// Transpile a script into rust code
pub fn transpile(tokens: TokenStream) -> Result<String, OfinError> {
    let mut script = String::new();

    for token in tokens {
        let literal: String = token.into();
        trace!("Converted token into string {:?}", &literal);
        script.push_str(&literal);
    }

    // Wrap the script in a rust main function, link our standard library, and include the prelude
    script = format!(
        "
        #![allow(unused_imports)]

        extern crate ofin_std; 
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
