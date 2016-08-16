
use project_types::{Sequence, User, Permission, PermissionEnum};
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
        let root_permission = try!(Permission::new(PermissionEnum::Administrate, None::<String>));
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
        for s in &self.sequences {
            if s.name == name {
                return Some(s);
            }
        }

        None::<&Sequence>
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
    // TODO: make private?
    pub fn find_user_by_name(&self, name: &str) -> Option<&User> {
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
    /// Returns a new project with the user added
    pub fn add_user(&self, name: &str, pub_key: &str) -> Result<Project, Error> {
        
        let user = try!(User::new(name, pub_key));

        if self.find_user_by_name(name).is_some() ||
           self.find_user_by_public_key(pub_key).is_some() {
            Err(Error::DuplicateUser(pub_key.to_owned(), name.to_owned()))
        } else {
            let mut new_project = self.clone();
            new_project.users.push(user);
            Ok(new_project)
        }
    }

    /// Removes a user from the project
    /// Returns a new project with the user removed
    pub fn remove_user(&self, name: &str) -> Result<Project, Error> {
        let mut new_project = self.clone();
        for i in 0..new_project.users.len() {
            if new_project.users[i].name == name {
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
        for s in &self.sequences {
            if s.name == name
            || s.directory_name == directory_name {
                return Err(Error::DuplicateSequence(name.to_owned()));
            }
        }

        let mut new_project = self.clone();
        new_project.sequences.push(sequence);
        Ok(new_project)
    }

    pub fn remove_sequence(&self, name: &str) -> Result<Project, Error> {
        let mut new_project = self.clone();
        for i in 0..new_project.sequences.len() {
            if new_project.sequences[i].name == name {
                new_project.sequences.remove(i);
                return Ok(new_project);
            }
        }
        Err(Error::SequenceNotFound(name.to_owned()))
    }

    pub fn resection_sequence(&self, name: &str, num_sections: u32) -> Result<Project, Error> {
        let mut new_project = self.clone();
        for i in 0..new_project.sequences.len() {
            if new_project.sequences[i].name == name {
                {
                    let sequence = &mut new_project.sequences[i];
                    try!(sequence.resection(num_sections));
                }
                return Ok(new_project);
            }
        }
        Err(Error::SequenceNotFound(name.to_owned()))
    }

    /// Changes a user's permissions
    pub fn set_user_permission(
        &mut self,
        name: &str,
        perm: Permission,
        add: bool
    ) -> Result<(), Error> {
    
        for i in 0..self.users.len() {
            if self.users[i].name == name {
                let mut editor = None::<User>;
                {
                    let u = &mut self.users[i];
                    if add {
                        u.add_permission(perm.clone());
                        editor = Some(u.to_owned());
                    } else {
                        u.remove_permission(perm.clone());
                    }
                }
                // Set sequence section's Editor field if necessary
                if &perm.which == &PermissionEnum::EditSeqSec {
                    let (seq_name, seq_sec_num) = try!(Permission::parse_seq_sec_target(&perm.target));
                    let sequence = self.find_sequence_by_name(&seq_name)
                        .expect("Error finding sequence");
                    let mut seq_sec = try!(sequence.get_section(seq_sec_num));
                    seq_sec.editor = editor;
                    try!(seq_sec.write_to_file());
                }
                return Ok(());
            }
        }

        Err(Error::UserNotFound)
    }
}
