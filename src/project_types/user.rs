
/// Structure to contain user information
#[derive(Clone, Debug, Eq, RustcEncodable, RustcDecodable)]
pub struct User {
    pub uid: u32,
    pub name: String,
    pub public_key: String,
}

impl PartialEq for User {
	// Users are the same if either they have the same user id or they 
	// have the same public key
    fn eq(&self, other: &User) -> bool {
        self.uid == other.uid ||
        self.public_key == other.public_key
    }
}
