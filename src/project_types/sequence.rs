use error::Error;
use project_types::Layout;

#[derive(Clone, Debug)]
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
        name: &str,
        music_file_name: &str,
        music_duration_sec: u32,
        seq_duration_ms: u32,
        frame_duration_ms: Option<u32>,
        layout: &Layout
    ) -> Result<Sequence, Error> {

        // Defaults
        let frame_dur_ms = frame_duration_ms.unwrap_or(50);
        if frame_dur_ms < 25 {
            return Err(Error::InvalidFrameDuration(frame_dur_ms));
        }

        // Calculate num_frames
        let num_frames_f32: f32 = seq_duration_ms as f32 / frame_dur_ms as f32;
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
}
