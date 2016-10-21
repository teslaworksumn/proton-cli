use project_types::ChannelSection;

#[derive(Clone, Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct Channel {
    pub name: String,
    pub numbers: (u32, u32), // Primary and secondary numbers (net lights)
    pub color: String,
    pub channel: u32,
    pub num_frames: u32,
    pub data: Vec<ChannelSection>,
}
