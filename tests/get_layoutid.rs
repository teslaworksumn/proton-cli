extern crate proton_cli;

mod dao;

use proton_cli::project_types::Project;


#[test]
fn proper_id_returned_with_existing_name() {
    let project_name = "Name";
    let mut project_dao = dao::ProjectDaoTesting::new();
    project_dao.get_project_fn = Box::new(|name: String| {
        match name.as_ref() {
            "Name" => Ok(Project {
                name: name,
                playlist: Vec::new(),
                layout_id: 1
            }),
            _ => Ok(Project {
                name: name,
                playlist: Vec::new(),
                layout_id: 0
            })
        }
    });
    let expected = 1;
    let actual = proton_cli::get_layout_id(&project_dao, project_name).expect("Error getting layout id");
    assert_eq!(expected, actual);
}

#[test]
#[should_panic(expected = "InvalidProjectName")]
fn fails_if_name_invalid() {
    let project_name = "_*($%";
    let project_dao = dao::ProjectDaoTesting::new();
    let _ = proton_cli::get_layout_id(&project_dao, project_name).expect("Error getting layout id");
}
