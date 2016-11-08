use project_types::Channel;

#[derive(Debug)]
pub struct Fixture {
    pub fixid: u32,
    pub name: String,
    pub location: (i32, i32, i32),
    pub rotation: (i32, i32, i32),
    pub channels: Vec<u32>,
}

