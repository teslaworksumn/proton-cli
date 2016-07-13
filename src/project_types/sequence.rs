
use error::Error;
use project_types::SequenceSection;


#[derive(Clone, Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct Sequence {
    pub name: String,
    pub directory_name: String,
    pub music_file_name: String,
    pub music_duration_sec: u32,
    pub frame_duration_ms: u32,
    pub num_sections: u32,
}

impl Sequence {
    /// Creates a new Sequence, allowing default values
    /// of 50ms for frame_duration_ms and 1 for num_sections
    /// Also initializes section file(s)
    pub fn new(name: &str,
        seq_directory_name: &str,
        music_file_name: &str,
        music_duration_sec: u32,
        frame_duration_ms: Option<u32>,
        num_sections: Option<u32>
    ) -> Result<Sequence, Error> {
        // Defaults
        let frame_dur_ms = frame_duration_ms.unwrap_or(50);
        let num_sects = num_sections.unwrap_or(1);

        // Create sequence
        let sequence = Sequence {
            name: name.to_string(),
            directory_name: seq_directory_name.to_string(),
            music_file_name: music_file_name.to_string(),
            music_duration_sec: music_duration_sec,
            frame_duration_ms: frame_dur_ms,
            num_sections: num_sects,
        };

        // Section sequence
        try!(sequence.resection());

        Ok(sequence)
    }

    /// Resection a sequence
    pub fn resection(&self) -> Result<(), Error> {
        if self.num_sections == 1 {
            let num_frames_f32: f32 = (self.music_duration_sec as f32 * 1000_f32) / self.frame_duration_ms as f32;
            let num_frames = num_frames_f32.ceil() as u32;
            let num_channels = 1; // TODO: change when add layout
            let sequence_section = SequenceSection {
                seq_name: self.name.to_string(),
                index: 1,
                num_frames: num_frames,
                editor: None,
                data: vec![vec![0; num_frames as usize]; num_channels],
            };
            sequence_section.write_to_file(&self.directory_name)
        } else {
            Err(Error::TodoErr)
        }
    }

}

