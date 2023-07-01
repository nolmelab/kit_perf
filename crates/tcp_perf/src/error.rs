use std::fmt::Formatter;

#[derive(Debug)]
pub struct Error {
    // error description
    desc: String,       
}

impl Error {
    pub fn new(addr: String, desc: String ) -> Self {
        Self {
            desc
        }
    }
}

impl std::error::Error for Error {
}

impl std::fmt::Display for Error {

    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> 
    { 
        write!(f, "desc: {}", self.desc)
    }
}

impl From<std::io::Error> for Error {

    fn from(e: std::io::Error) -> Self { 
        let desc = format!("io: {:?}", e);
        Self {
           desc 
        }
    }
}