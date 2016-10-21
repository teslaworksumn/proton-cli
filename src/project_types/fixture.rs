use project_types::Channel;

#[derive(Clone, Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct Fixture {
    pub name: String,
    pub location: (i32, i32, i32),
    pub rotation: (i32, i32, i32),
    pub editor: Option<User>,
    pub channels: Vec<Channel>,
    pub num_frames: u32,
    pub num_sections: u32,
}


