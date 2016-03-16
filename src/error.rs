use std::{io, error, fmt};
use git2;

#[derive(Debug)]
pub enum Error<'a> {
    Io(io::Error),
    Git(git2::Error),
    FolderNotEmpty(&'a str, usize),
    TodoErr,
}

impl<'a> error::Error for Error<'a> {
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

impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => write!(f, "IO error occurred: {}", error::Error::description(err)),
            Error::Git(ref err) => write!(f, "Libgit2 error occurred: {}", error::Error::description(err)),
            Error::FolderNotEmpty(root, count) => write!(f, "{} was not empty: {} files exist", root, count),
            Error::TodoErr => write!(f, "TodoErr"),
        }
    }
}

impl<'a> From<io::Error> for Error<'a> {   
    fn from(err: io::Error) -> Error<'a> {   
        Error::Io(err)   
    }    
}    
    
impl<'a> From<git2::Error> for Error<'a> {   
    fn from(err: git2::Error) -> Error<'a> {   
        Error::Git(err)    
    }    
}
