pub struct CLI {
    pub path: String,
}

impl CLI {
    pub fn parse() -> Result<Self, crate::error::Error> {
        let path = match std::env::args().nth(1) {
            Some(p) => Ok(p),
            None => Err(error!("no input file")),
        };

        Ok(CLI { path: path? })
    }
}
