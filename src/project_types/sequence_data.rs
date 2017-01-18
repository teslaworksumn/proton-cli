
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct SequenceData {
    pub name: String,
    pub music_file: String,
    pub frame_dur_ms: u32,
    pub num_frames: u32,
    pub data: Vec<Vec<u16>>
}
