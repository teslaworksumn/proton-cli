use project_types::{self, Permission};
use dao::{PermissionDao, PermissionDaoPostgres};
use error::Error;


impl PermissionDao for PermissionDaoPostgres {

    fn add_initial_permission(&self, root_uid: u32) -> Result<(), Error> {
        let statement = "INSERT INTO permissions (uid, seqid, secid, permission) VALUES ($1, $2, $3, $4)";
        let seqid = None::<i32>;
        let secid = None::<i32>;
        let permission = "Administrate"; // Has to match up to permission_enum
        let _ = try!(
            self.conn.execute(statement, &[&(root_uid as i32), &seqid, &secid, &permission.to_owned()])
            .map_err(Error::Postgres));
        Ok(())
    }

    fn get_all_permissions(&self, uid: u32) -> Result<Vec<Permission>, Error> {
        let query = "SELECT permid, seqid, secid, permission FROM permissions WHERE uid = $1";
        let results = try!(
            self.conn.query(query, &[&(uid as i32)])
            .map_err(Error::Postgres));
        let mut permissions = vec![];
        for row in &results {
            let permid: i32 = row.get(0);
            let seqid: Option<i32> = row.get(1);
            let secid: Option<i32> = row.get(2);
            let seq = seqid.map(|s| s as u32);
            let sec = secid.map(|s| s as u32);
            let perm_string: String = row.get(3);
            let perm_enum = try!(
                project_types::get_permission_enum(&perm_string, seq, sec));
            let permission = Permission {
                permid: permid as u32,
                uid: uid,
                seqid: seq,
                secid: sec,
                permission: perm_enum
            };
            permissions.push(permission);
        }

        Ok(permissions)
    }

    #[allow(unused_variables)]
    fn get_permission(&self, permid: u32) -> Result<Permission, Error> {
        Err(Error::TodoErr)
    }
}
