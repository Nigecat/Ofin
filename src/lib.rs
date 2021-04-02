mod error;
mod source;

pub use error::Error;
pub use ofin_std as std;
pub use source::Source;

use ::std::convert::TryInto;

// #[macro_use]
// extern crate tracing;
// #[macro_use]
// extern crate lazy_static;

/// Run an ofin program from the supplied source.
pub fn run(source: Source) -> Result<(), Error> {
    let contents: String = source.try_into()?;

    std::gui::createWindow();

    Ok(())
}
