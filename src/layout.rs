//! Layout-related functionality

use rustc_serialize::json;
use std::path::Path;

use dao::{ChannelDao, FixtureDao, LayoutDao, SequenceDao};
use error::Error;
use project_types::{FileLayout, FilePatch};
use utils;


/// Patches a layout's channels based on a provided patch file
pub fn patch_layout<P: AsRef<Path>, LD: LayoutDao> (
    layout_dao: &LD,
    layout_id: u32,
    patch_file_path: P
) -> Result<(), Error> {

    // Load patch file
    let patch_json = try!(utils::file_as_string(patch_file_path.as_ref()));
    let patch_file: FilePatch = try!(json::decode(&patch_json).map_err(Error::JsonDecode));
    
    // Make sure patch is valid
    try!(patch_file.validate());

    // Apply patch
    for patch in patch_file.patches.iter() {
        match layout_dao.patch_channel(layout_id, patch.internalChannel, patch.dmxChannel) {
            Ok(1) => {},
            Ok(0) => println!("No channels patched. vix: {}, dmx: {}", patch.internalChannel, patch.dmxChannel),
            Ok(num_ch) => println!("Patched {} channels.", num_ch),
            Err(e) => println!("Error: {}", e),
        }
    }

    Ok(())
}

/// Creates a new layout
pub fn new_layout<P: AsRef<Path>, CD: ChannelDao, FD: FixtureDao, LD: LayoutDao>(
    chan_dao: &CD,
    fix_dao: &FD,
    layout_dao: &LD,
    layout_path: P,
) -> Result<u32, Error> {

    // Load layout from file
    let layout_json = try!(utils::file_as_string(layout_path.as_ref()));
    let file_layout: FileLayout = try!(json::decode(&layout_json).map_err(Error::JsonDecode));
    
    // Make sure layout is valid
    try!(file_layout.validate());

    // Create new channels and fixtures from layout and add to storage
    let (_, fixtures) = try!(file_layout.create_new_parts(chan_dao, fix_dao));

    // Create new layout from fixtures
    let fix_ids = fixtures.iter()
        .map(|fixture| fixture.fixid)
        .collect::<Vec<u32>>();
    let layout = try!(layout_dao.new_layout(&file_layout.layoutName, fix_ids));

    // Return layout id
    Ok(layout.layout_id)
}

/// Set a layout's sequence
pub fn set_sequence_layout<LD: LayoutDao, SD: SequenceDao>(
    layout_dao: &LD,
    sequence_dao: &SD,
    layout_id: u32,
    seqid: u32
) -> Result<(), Error> {


    // Check that sequence exists
    let sequence = try!(sequence_dao.get_sequence(seqid));

    // Check that current layout exists
    try!(layout_dao.layout_exists(sequence.layout_id));
    
    // Check that new layout exists
    try!(layout_dao.layout_exists(layout_id));

    // Set sequence layout id
    try!(sequence_dao.set_layout(seqid, layout_id));

    Ok(())
}
