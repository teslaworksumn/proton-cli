/// Structure to represent a Proton Project.
/// This is what will be written to a Protonfile at the project root.
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Project {
    pub name: String,
}

impl Project {
    /// Creates an empty project
    pub fn empty() -> Project {
        Project {
            name: "New Project".to_owned(),
        }
    }
}
