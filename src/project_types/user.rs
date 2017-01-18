
// Note: DO NOT return a uid in any public function. They are used for authentication and are
// for internal function calls only.

#[derive(Clone, Debug, Eq, RustcEncodable, RustcDecodable)]
pub struct User {
    pub uid: u32,
    pub name: String,
    pub public_key: String,
}

impl PartialEq for User {
    fn eq(&self, other: &User) -> bool {
        self.uid == other.uid ||
        self.public_key == other.public_key
    }
}
