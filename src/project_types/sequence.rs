use rustc_serialize::json;

use error::Error;
use project_types::Layout;

#[derive(Debug)]
pub struct Sequence {
    pub seqid: u32,
    pub name: String,
    pub music_file_name: String, // found in Music/
    pub music_duration_sec: u32,
    pub frame_duration_ms: u32,
    pub num_frames: u32,
    pub layout_id: u32,
}

impl Sequence {
    /// Creates a new Sequence, allowing the default value of 50ms for frame_duration_ms
    pub fn new(
        admin_uid: u32,
        name: &str,
        music_file_name: &str,
        music_duration_sec: u32,
        frame_duration_ms: Option<u32>,
        layout: &Layout,
        num_channels: u32
    ) -> Result<Sequence, Error> {
        // Defaults
        let frame_dur_ms = frame_duration_ms.unwrap_or(50);
        if frame_dur_ms < 25 {
            return Err(Error::InvalidFrameDuration(frame_dur_ms));
        }

        // Calculate num_frames
        let num_frames_f32: f32 = (music_duration_sec as f32 * 1000_f32) / frame_dur_ms as f32;
        let num_frames = num_frames_f32.ceil() as u32;

        // Create temporary seqid (seqid will be set internally by the sequence dao)
        let seqid = 0;

        // Get layout id
        let layout_id = layout.layout_id;

        // Create sequence
        let sequence = Sequence {
            seqid: seqid,
            name: name.to_string(),
            music_file_name: music_file_name.to_string(),
            music_duration_sec: music_duration_sec,
            frame_duration_ms: frame_dur_ms,
            num_frames: num_frames,
            layout_id: layout_id
        };

        Ok(sequence)
    }

    // pub fn data_as_json(&self) -> Result<json::Json, Error> {        
    //     let json_str = try!(json::encode(&self.data).map_err(Error::JsonEncode));
    //     // Unwrap should be safe here, since we just encoded successfully
    //     Ok(json::Json::from_str(&json_str).unwrap())
    // }

    // pub fn update_data(&self, uid: u32, new_data: Vec<Vec<u16>>, secid: u32) -> Result<(), Error> {
    //     // Only update data within the given section
    //     // If sections overlap then this will overwrite the last update in the overlapping section
    //     // It is up to the user to deal with overlapping sections appropriately

    //     // Check if user has permission to update this sequence/section
    //     Err(Error::TodoErr)
    // }


}

