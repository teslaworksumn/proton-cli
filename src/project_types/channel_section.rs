use project_types::User;

#[derive(Clone, Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct ChannelSection {
    pub channel_name: String,
    pub section_index: u32,
    pub num_frames: u32,
    pub num_channels: u32,
    pub data: Vec<u8>,
}

impl ChannelSection {

}
