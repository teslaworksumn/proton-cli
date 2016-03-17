use std::{io, error, fmt};
use git2;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Git(git2::Error),
    FolderNotEmpty(String, usize),
    TodoErr,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(_) => "IO error occurred",
            Error::Git(_) => "Libgit2 error occurred",
            Error::FolderNotEmpty(_, _) => "Root folder was not empty",
            Error::TodoErr => "Todo",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
           Error::Io(ref err) => Some(err),
           Error::Git(ref err) => Some(err),
           Error::FolderNotEmpty(_, _) => None,
           Error::TodoErr => None,
       }
   }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => write!(f, "IO error occurred: {}", error::Error::description(err)),
            Error::Git(ref err) => write!(f, "Libgit2 error occurred: {}", error::Error::description(err)),
            Error::FolderNotEmpty(ref root, count) => write!(f, "{} was not empty: {} files exist", root, count),
            Error::TodoErr => write!(f, "TodoErr"),
        }
    }
}

impl From<io::Error> for Error {   
    fn from(err: io::Error) -> Error {   
        Error::Io(err)   
    }    
}    
    
impl From<git2::Error> for Error {   
    fn from(err: git2::Error) -> Error {   
        Error::Git(err)    
    }    
}
