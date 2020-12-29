use super::OfinError;
use crate::lexer::TokenStream;

/// The line offset from the start of the file the source code is located in the transpiled code
/// This should be kept up to date with the format string contained in this file
pub static OFFSET: usize = 11;

/// Transpile a script into rust code
pub fn transpile(tokens: TokenStream) -> Result<String, OfinError> {
    let mut script = String::new();

    for token in tokens {
        let literal: String = token.into();
        debug!("Converted token into string {:?}", &literal);
        script.push_str(&literal);
    }

    // Wrap the script in a rust main function, link our standard library, and include the prelude
    script = format!(
        "
        #![no_std]
        #![no_implicit_prelude]
        #![allow(clippy::all)]
        #![allow(dead_code)]
        #![allow(unused_mut)]
        #![allow(unused_imports)]
        #![allow(unused_variables)]

        extern crate ofin_std; 
        use ofin_std::prelude::*; 
        
        fn main() {{
            {}
        }}
    ",
        script
    );

    debug!("Finished transpiling with code:\n{}", script);

    Ok(script)
}
