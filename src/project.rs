use std::path::Path;
use rustc_serialize::json;

use utils;
use error::Error;
use dao::{ChannelDao, DataDao, LayoutDao, PermissionDao, ProjectDao, SequenceDao, UserDao};


/// Initializes a new project at root. The root must either not exist, or must
/// be an empty directory. This will
///
/// 1. Create the directory if it doesn't exist.
/// 2. Create a Protonfile
/// 3. Initialize a git repository and commit the protonfile.
///
/// Impure.
pub fn new_project<LD: LayoutDao, PMD: PermissionDao, PTD: ProjectDao, UD: UserDao>(
    layout_dao: &LD,
    perm_dao: &PMD,
    project_dao: &PTD,
    user_dao: &UD,
    name: &str,
    layout_id: u32
) -> Result<String, Error> {

    // Check that layout exists
    let _ = try!(layout_dao.get_layout(layout_id));

    // Create keys
    let (root_pub_key, root_private_key) = try!(utils::create_pub_priv_keys());

    // Add project root user
    let root_uid = try!(user_dao.add_initial_user(name, &root_private_key, &root_pub_key));

    // Give initial user admin permissions
    try!(perm_dao.add_initial_permission(root_uid));

    // Create new project
    let _ = try!(project_dao.new_project(name, layout_id));

    // Return root user's public key
    Ok(root_pub_key)
}

pub fn get_layout_id<PD: ProjectDao>(
    proj_dao: &PD,
    proj_name: &str
) -> Result<u32, Error> {
    
    // Check that project exists
    let project = try!(proj_dao.get_project(proj_name));

    // Return layout id
    Ok(project.layout_id)
}

/// Gets all sequence data in the project's playlist
/// Returns as JSON array, each index corresponding to a DMX channel
/// The value at index 0 will always be 0, since DMX starts at 1
pub fn get_playlist_data<CD: ChannelDao, DD: DataDao, PD: ProjectDao, SD: SequenceDao>(
    chan_dao: &CD,
    data_dao: &DD,
    proj_dao: &PD,
    seq_dao: &SD,
    proj_name: &str
) -> Result<String, Error> {

    // Check that project exists
    let project = try!(proj_dao.get_project(proj_name));

    let mut playlist_data = vec![Vec::new(); project.playlist.len()];

    // Go through each sequence in the playlist
    for (i, seqid) in project.playlist.iter().enumerate() {

        // Get sequence
        let sequence = try!(seq_dao.get_sequence(seqid.to_owned()));

        // Get the sequence's channel ids
        let chan_ids = try!(seq_dao.get_channel_ids(seqid.to_owned()));

        // Create vector for sequence data
        // Up to 512 channels per universe, plus one because DMX starts at 1
        let mut seq_data = vec![vec![0; sequence.num_frames as usize]; 513];

        // Get each channel's data and put it in the correct vector slot
        for chanid in chan_ids {
            let channel = try!(chan_dao.get_channel(chanid));
            let chan_data = try!(data_dao.get_data(seqid.to_owned(), chanid.to_owned()));
            seq_data[channel.channel_dmx as usize] = chan_data;
        }

        playlist_data[i] = seq_data;
    }

    json::encode(&playlist_data).map_err(Error::JsonEncode)
}
