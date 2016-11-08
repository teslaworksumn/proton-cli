use std::path::Path;

use project_types::User;
use dao::UserDao;
use error::Error;

pub struct UserDaoPostgres{}

impl UserDao for UserDaoPostgres {

    fn get_user(&self, uid: u32) -> Result<User, Error> {
        Err(Error::TodoErr)
    }

    /// Identifies a user by their private SSH key by finding the
    /// corresponding public key in the project. This private key
    /// acts like the user's password, and should be protected.
    /// 
    /// Impure.
    fn id_user<P: AsRef<Path>>(&self, private_key_path: P) -> Result<u32, Error> {
        Err(Error::TodoErr)
    //     let test_data: &[u8] = b"Testing to find private/public key pair";
        
    //     let mut private_key_file = try!(File::open(&private_key_path).map_err(Error::Io));
        
    //     let private_key = try!(openssl_RSA::private_key_from_pem(&mut private_key_file)
    //         .map_err(Error::Ssl));

    //     let signature = try!(private_key.sign(openssl_HashType::MD5, &test_data)
    //         .map_err(Error::Ssl));

    //     let users = get_all_users();
    //     for user in project.users {
    //         let user_key = user.public_key.clone();
    //         let mut pub_key_readable = Cursor::new(&user_key);

    //         let rsa_public = try!(openssl_RSA::public_key_from_pem(&mut pub_key_readable)
    //             .map_err(Error::Ssl));
            
    //         match rsa_public.verify(openssl_HashType::MD5, &test_data, &signature) {
    //             Ok(valid) => if valid {
    //                 return Ok(user)
    //             },
    //             Err(e) => return Err(Error::Ssl(e)),
    //         };
    //     };
        
    //     Err(Error::UserNotFound)
    }
    
}
