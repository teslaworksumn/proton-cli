use project_types::Project;
use error::Error;
use dao::{ProjectDao, ProjectDaoPostgres};

impl ProjectDao for ProjectDaoPostgres {
    fn new_project(&self, name: &str, layout_id: u32) -> Result<Project, Error> {
        let statement = "INSERT INTO projects (name,playlist,layoutid) VALUES ($1,$2,$3)";
        let playlist: Vec<i32> = Vec::new();
        let lid = layout_id as i32;
        let _ = try!(
            self.conn.execute(
                statement,
                &[
                    &name.to_owned(),
                    &playlist,
                    &lid
                ])
            .map_err(Error::Postgres));

        // Project name is unique
        self.get_project(name)
    }

    fn get_project(&self, name: &str) -> Result<Project, Error> {
        let query = "SELECT playlist,layoutid FROM projects WHERE name = $1";
        let results = try!(
            self.conn.query(query, &[&name.to_owned()])
            .map_err(Error::Postgres));
        match results.len() {
            0 => Err(Error::ProjectNotFound(name.to_owned())),
            1 => {
                let row = results.get(0);
                let playlist_i32: Vec<i32> = row.get(0);
                let playlist = playlist_i32.iter()
                    .map(|seqid| *seqid as u32)
                    .collect::<Vec<u32>>();
                let layout_id: i32 = row.get(1);
                Ok(Project {
                    name: name.to_owned(),
                    playlist: playlist,
                    layout_id: layout_id as u32
                })
            },
            x => Err(Error::InvalidNumResults(x)),
        }
    }

    fn update_project(&self, new_project: Project) -> Result<(), Error> {
        let query = "UPDATE projects SET playlist = $1, layoutid = $2 WHERE name = $3";
        let playlist_i32 = new_project.playlist.iter()
            .map(|seqid| *seqid as i32)
            .collect::<Vec<i32>>();
        let layoutid_i32 = new_project.layout_id as i32;
        let name = new_project.name;
        let _ = try!(self.conn.query(query, &[&playlist_i32, &layoutid_i32, &name])
            .map_err(Error::Postgres));
        Ok(())
    }
}
