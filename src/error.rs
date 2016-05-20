use std::{io, error, fmt};
use git2;
use rustc_serialize::json;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Git(git2::Error),
    JsonDecode(json::DecoderError),
    FolderNotEmpty(String, usize),
    ArgumentNotFound,
    TodoErr,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(_) => "IO error occurred",
            Error::Git(_) => "Libgit2 error occurred",
            Error::JsonDecode(_) => "Json decoding error occurred",
            Error::FolderNotEmpty(_, _) => "Root folder was not empty",
            Error::ArgumentNotFound => "Argument not found/matched",
            Error::TodoErr => "Todo",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
           Error::Io(ref err) => Some(err),
           Error::Git(ref err) => Some(err),
           Error::JsonDecode(ref err) => Some(err),
           Error::FolderNotEmpty(_, _) => None,
           Error::ArgumentNotFound => None,
           Error::TodoErr => None,
       }
   }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => write!(f,
                "IO error occurred: {}", error::Error::description(err)),
            Error::Git(ref err) => write!(f,
                "Libgit2 error occurred: {}", error::Error::description(err)),
            Error::JsonDecode(ref err) => write!(f,
                "Json decoding error occurred: {}", error::Error::description(err)),
            Error::FolderNotEmpty(ref root, count) => write!(f,
                "{} was not empty: {} files exist", root, count),
            Error::ArgumentNotFound => write!(f,
                "Argument not found. Did you type it correctly?"),
            Error::TodoErr => write!(f, "TodoErr"),
        }
    }
}
