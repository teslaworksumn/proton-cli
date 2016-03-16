/// Structure to represent a Proton Project.
/// This is what will be written to 
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Project {
    pub name: String,
}

impl Project {
    /// Creates an empty project
    pub fn empty() -> Project {
        Project {
            name: "Name This Project".to_owned(),
        }
    }
}
