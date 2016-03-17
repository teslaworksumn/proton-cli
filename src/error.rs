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
