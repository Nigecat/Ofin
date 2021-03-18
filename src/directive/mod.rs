/// A higher level representation of ofin source code, this is completely decoupled from the syntax
pub enum Directive {
    Import(String),
    Using(String),
}
