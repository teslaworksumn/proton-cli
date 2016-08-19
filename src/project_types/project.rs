
use project_types::{Sequence, User, Permission};
use error::Error;


/// Structure to represent a Proton Project.
/// This is what will be written to a Protonfile at the project root.
#[derive(Clone, Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct Project {
    pub name: String,
    pub users: Vec<User>,
    pub sequences: Vec<Sequence>,
}

impl Project {
    /// Creates an empty project with the given root user
    pub fn empty(root_pub_key: &str) -> Result<Project, Error> {

        let mut root = try!(User::new("root", &root_pub_key));
        let root_permission = Permission::Administrate;
        root.add_permission(root_permission);

        Ok(Project {
            name: "New Project".to_owned(),
            users: vec![root],
            sequences: Vec::new(),
        })
    }

    /// Finds a sequence by its name
    /// Returns the sequence if found, else None
    pub fn find_sequence_by_name(&self, name: &str) -> Option<&Sequence> {
        self.sequences.iter().find(|seq| seq.name == name)
    }

    /// Finds a user with the given public key
    /// Returns the user if found, else None
    fn find_user_by_public_key(&self, pub_key: &str) -> Option<&User> {
        self.users.iter().find(|user| user.public_key == pub_key)
    }

    /// Finds a user with the given name
    /// Returns the user if found, else None
    // TODO: make private?
    pub fn find_user_by_name(&self, name: &str) -> Option<&User> {
        self.users.iter().find(|user| user.name == name)
    }

    /// Finds a user in the users vector
    /// Returns true if found, else false
    pub fn user_exists(&self, user: &User) -> bool {
        self.users.iter().find(|u| u == &user).is_some()
    }

    /// Adds a user to the project
    /// Returns a new project with the user added
    pub fn add_user(&self, name: &str, pub_key: &str) -> Result<Project, Error> {

        let mut new_project = self.clone();
        let user = try!(User::new(name, pub_key));
        
        if self.find_user_by_name(name).is_some() ||
           self.find_user_by_public_key(pub_key).is_some() {
            return Err(Error::DuplicateUser(pub_key.to_owned(), name.to_owned()));
        } else {
            new_project.users.push(user);
        }

        Ok(new_project)
    }

    /// Removes a user from the project
    /// Returns a new project with the user removed
    pub fn remove_user(&self, name: &str) -> Result<Project, Error> {
        let mut new_project = self.clone();
        for (i, user) in self.users.iter().enumerate() {
            if user.name == name {
                new_project.users.remove(i);
                return Ok(new_project);
            }
        }
        Err(Error::UserNotFound)
    }

    /// Adds a sequence to the project
    /// Returns a new project with the sequence added
    pub fn add_sequence(
        &self,
        name: &str,
        directory_name: &str,
        music_file_name: &str,
        music_duration_sec: u32,
    ) -> Result<Project, Error> {
    
        let sequence = try!(Sequence::new(
            name,
            directory_name,
            music_file_name,
            music_duration_sec,
            None,
            None,
        ));

        // Check if duplicate
        match self.sequences
            .iter()
            .find(|seq| seq.name == name || seq.directory_name == directory_name) {
                Some(_) => {
                    return Err(Error::DuplicateSequence(name.to_owned()));
                },
                None => ()
        }

        let mut new_project = self.clone();
        new_project.sequences.push(sequence);
        Ok(new_project)
    }

    pub fn remove_sequence(&self, name: &str) -> Result<Project, Error> {
        let mut new_project = self.clone();
        for (i, seq) in self.sequences.iter().enumerate() {
            if seq.name == name {
                new_project.sequences.remove(i);
                return Ok(new_project);
            }
        }
        Err(Error::SequenceNotFound(name.to_owned()))
    }

    pub fn resection_sequence(&self, name: &str, num_sections: u32) -> Result<Project, Error> {
        let mut new_project = self.clone();
        match new_project.sequences.iter_mut().find(|seq| seq.name == name) {
            None => {
                return Err(Error::SequenceNotFound(name.to_owned()));
            },
            Some(sequence) => {
                try!(sequence.resection(num_sections));
            }
        }
        Ok(new_project)
    }

    /// Changes a user's permissions
    pub fn set_user_permission(
        &mut self,
        name: &str,
        perm: Permission,
        add: bool
    ) -> Result<(), Error> {
    
        match self.users.iter_mut().find(|u| u.name == name) {
            None => Err(Error::UserNotFound),
            Some(user) => {
                if add {
                    user.add_permission(perm.clone())
                } else {
                    user.remove_permission(perm.clone())
                }
                Ok(())
            }
        }
    }
}
