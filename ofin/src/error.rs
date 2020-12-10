use thiserror::Error;

#[derive(PartialEq, Debug, Error)]
pub enum OfinError {
    #[error("This is an internal error and should not have occured")]
    InternalError,
    #[error("A syntax error has occured at row {row}, column {column}:\n{ctx}")]
    SyntaxError {
        /// The column this error occured in
        column: usize,
        /// The row this error occured in
        row: usize,
        /// Any context for this error
        ctx: String,
    },
}
