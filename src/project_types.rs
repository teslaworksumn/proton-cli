
use Error;

/// Structure to represent a Proton Project.
/// This is what will be written to a Protonfile at the project root.
#[derive(Clone, Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct Project {
    pub name: String,
    pub users: Vec<User>,
}

#[derive(Clone, Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct User {
    pub name: String,
    pub public_key: String,
}

impl Project {
    /// Creates an empty project
    pub fn empty() -> Project {
        Project {
            name: "New Project".to_owned(),
            users: Vec::new(),
        }
    }

    /// Finds a user with the given public key
    /// Returns the user if found, else None
    fn find_user_by_public_key(&self, pub_key: &str) -> Option<&User> {
        for u in &self.users {
            if u.public_key == pub_key {
                return Some(u);
            }
        }
        None::<&User>
    }

    /// Finds a user with the given name
    /// Returns the user if found, else None
    fn find_user_by_name(&self, name: &str) -> Option<&User> {
        for u in &self.users {
            if u.name == name {
                return Some(u);
            }
        }
        None::<&User>
    }

    /// Finds a user in the users vector
    /// Returns true if found, else false
    pub fn user_exists(&self, user: &User) -> bool {
        for u in &self.users {
            if user == u {
                return true;
            }
        }
        return false;
    }

    /// Adds a user to the project
    pub fn add_user(&self, name: &str, pub_key: &str) -> Result<Project, Error> {
        
        let user = User {
            name: name.to_string(),
            public_key: pub_key.to_string(),
        };

        if self.find_user_by_name(name).is_some() ||
           self.find_user_by_public_key(pub_key).is_some() {
           
            Err(self.duplicate_user(pub_key, name))
        } else {
            let mut new_project = self.clone();
            new_project.users.push(user);
            Ok(new_project)
        }
    }

    fn duplicate_user(&self, pub_key: &str, name: &str) -> Error {
        Error::DuplicateUser(pub_key.to_string(), name.to_string())
    }

}
