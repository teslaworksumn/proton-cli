use std::path::{Path, PathBuf};
use std::cmp;

use git2::Signature;

use error::Error;
use utils;

#[derive(Debug)]
pub struct Sequence {
    pub seqid: u32,
    pub name: String,
    pub music_file_name: String, // found in Music/
    pub music_duration_sec: u32,
    pub frame_duration_ms: u32,
    pub num_frames: u32,
    pub layout_id: u32,
    pub data: Vec<Vec<u16>>, // row is channel, col is frame
}

impl Sequence {
    /// Creates a new Sequence, allowing the default value of 50ms for frame_duration_ms
    pub fn new(
        uid: u32,
        name: &str,
        music_file_path: &str,
        music_duration_sec: u32,
        frame_duration_ms: Option<u32>,
        layout_id: u32 
    ) -> Result<Sequence, Error> {
        // Defaults
        let frame_dur_ms = frame_duration_ms.unwrap_or(50);
        if frame_dur_ms < 25 {
            return Err(Error::InvalidFrameDuration(frame_dur_ms));
        }

        // Make sure user has permission to create sequence
        // Check that layout id is valid
        // Check that music file exists
        // Check that name isn't already taken 
        // (useful for preventing creating multiple identical sequences accidentally)
        // Get name of music file
        let mf_path = Path::new(music_file_path);
        let music_file_name = try!(utils::file_name_from_path(&mf_path));
        // Copy music file to Music/
        // Calculate num_frames
        let num_frames = 200;
        // Create unique seqid
        let seqid = 0;
        // Get number of channels
        let num_channels = 400; // Layout::get_num_channels(layout_id)

        // Create sequence
        let mut sequence = Sequence {
            seqid: seqid,
            name: name.to_string(),
            music_file_name: music_file_name.to_string(),
            music_duration_sec: music_duration_sec,
            frame_duration_ms: frame_dur_ms,
            num_frames: num_frames,
            layout_id: layout_id,
            data: vec![vec![0; num_frames as usize]; num_channels as usize]
        };

        Ok(sequence)
    }

    pub fn update_data(&self, uid: u32, new_data: Vec<Vec<u16>>, secid: u32) -> Result<(), Error> {
        // Only update data within the given section
        // If sections overlap then this will overwrite the last update in the overlapping section
        // It is up to the user to deal with overlapping sections appropriately

        // Check if user has permission to update this sequence/section
        Err(Error::TodoErr)
    }


}

