//! This module manages project sequences

use rustc_serialize::json;
use std::path::{Path, PathBuf};
use std::fs;

use git2::Signature;
use regex::Regex;
use sfml::audio::Music;

use error::Error;
use project_types::{Permission, PermissionEnum, Sequence};
use dao::{FixtureDao, LayoutDao, PermissionDao, SequenceDao, UserDao};
use user;
use utils;

/// Creates a new sequence 
pub fn new_vixen_sequence<P: AsRef<Path>, FD: FixtureDao, LD: LayoutDao, PD: PermissionDao, SD: SequenceDao, UD: UserDao>(
    fixture_dao: &FD,
    layout_dao: &LD,
    perm_dao: &PD,
    seq_dao: &SD,
    user_dao: &UD,
    admin_key_path: P,
    name: &str,
    music_file_path: P,
    frame_duration_ms: u32,
    data_file_path: P,
    layout_id: u32
) -> Result<(), Error> {

    // Check that the admin has sufficient privileges
    let valid_permissions = vec![PermissionEnum::Administrate];
    let admin_uid = try!(utils::check_valid_permission(
        perm_dao,
        user_dao,
        admin_key_path,
        &valid_permissions));

    // Get layout (also checks if it exists)
    let layout = try!(layout_dao.get_layout(layout_id));

    // Get number of channels
    let num_channels = try!(layout.get_num_channels(fixture_dao));

    // Make sure the music file is a valid format
    try!(validate_file_type(&music_file_path));

    // Get name of music file from path
    let music_file_name = try!(utils::file_name_from_path(&music_file_path));

    // Get duration of music file
    let music_duration_sec = try!(get_music_duration_sec(&music_file_path));
    
    // Try to copy music file into music directory
    try!(copy_music_file(&music_file_path, &music_file_name, "Music"));

    // TODO: revert music file copy if rest fails

    // Read in sequence data
    let seq_data_str = try!(utils::file_as_string(data_file_path.as_ref()));
    let seq_data_json = try!(json::Json::from_str(&seq_data_str).map_err(Error::JsonParse));
    let seq_data = utils::sequence_json_to_vec(seq_data_json);

    return Err(Error::TodoErr);

    // Create sequence
    let sequence = try!(
        Sequence::new(
            admin_uid,
            name,
            &music_file_name,
            music_duration_sec,
            Some(frame_duration_ms),
            &layout,
            num_channels,
            Some(seq_data)
        )
    );

    // Try to add sequence
    try!(seq_dao.new_sequence(&sequence));

    // Commit changes
    let signature = Signature::now("Proton Lights", "proton@teslaworks.net").unwrap();
    let msg = format!("Adding new sequence '{}'", name);
    let repo_path: Option<P> = None;

    utils::commit_all(repo_path, &signature, &msg)
        .map(|_| ())
}

/// Creates a new user for the project in the current directory.
/// Assumes the current directory contains a Protonfile.json file.
///
/// Impure.
pub fn new_sequence<P: AsRef<Path>, FD: FixtureDao, LD: LayoutDao, PD: PermissionDao, SD: SequenceDao, UD: UserDao>(
    fixture_dao: &FD,
    layout_dao: &LD,
    perm_dao: &PD,
    seq_dao: &SD,
    user_dao: &UD,
    admin_key_path: P,
    name: &str,
    music_file_path: P,
    frame_duration_ms: Option<u32>,
    layout_id: u32
) -> Result<(), Error> {

    // Check that the admin has sufficient privileges
    let valid_permissions = vec![PermissionEnum::Administrate];
    let admin_uid = try!(utils::check_valid_permission(
        perm_dao,
        user_dao,
        admin_key_path,
        &valid_permissions));

    // Get layout (also checks if it exists)
    let layout = try!(layout_dao.get_layout(layout_id));

    // Get number of channels
    let num_channels = try!(layout.get_num_channels(fixture_dao));
    println!("num_channels: {}", &num_channels);

    // Make sure the music file is a valid format
    try!(validate_file_type(&music_file_path));

    // Get name of music file from path
    let music_file_name = try!(utils::file_name_from_path(&music_file_path));

    // Get duration of music file
    let music_duration_sec = try!(get_music_duration_sec(&music_file_path));
    
    // Try to copy music file into music directory
    try!(copy_music_file(&music_file_path, &music_file_name, "Music"));

    // TODO: revert music file copy if rest fails

    // Create sequence with default data
    let sequence = try!(
        Sequence::new(
            admin_uid,
            name,
            &music_file_name,
            music_duration_sec,
            frame_duration_ms,
            &layout,
            num_channels,
            None::<Vec<Vec<u16>>>
        )
    );

    // Try to add sequence
    try!(seq_dao.new_sequence(&sequence));

    // Commit changes
    let signature = Signature::now("Proton Lights", "proton@teslaworks.net").unwrap();
    let msg = format!("Adding new sequence '{}'", name);
    let repo_path: Option<P> = None;

    utils::commit_all(repo_path, &signature, &msg)
        .map(|_| ())
}

/// Adds the sequence with the given name to the project's playlist
pub fn add_sequence<P: AsRef<Path>, PD: PermissionDao, UD: UserDao>(
    perm_dao: &PD,
    user_dao: &UD,
    admin_key_path: P,
    seqid: u32
) -> Result<(), Error> {
    
    // Check that the admin has sufficient privileges
    let valid_permissions = vec![PermissionEnum::Administrate, PermissionEnum::EditSequence(seqid)];
    let admin_uid = try!(utils::check_valid_permission(
        perm_dao,
        user_dao,
        admin_key_path,
        &valid_permissions));

    // Check that seqid exists
    return Err(Error::TodoErr);

    // Add sequence to project's playlist
    let project = try!(utils::read_protonfile(None::<P>));
    let new_project = try!(project.add_sequence(seqid));

    // Save project
    try!(utils::write_protonfile(&new_project, None::<P>));

    // Commit changes
    let signature = Signature::now("Proton Lights", "proton@teslaworks.net").unwrap();
    let msg = format!("Adding sequence '{}' to playlist", seqid);
    let repo_path: Option<P> = None;

    utils::commit_all(repo_path, &signature, &msg)
        .map(|_| ())
}

/// Removes the sequence with the given name from the project
/// and deletes its files
pub fn remove_sequence<P: AsRef<Path>, PD: PermissionDao, UD: UserDao>(
    perm_dao: &PD,
    user_dao: &UD,
    admin_key_path: P,
    seqid: u32
) -> Result<(), Error> {
    
    return Err(Error::TodoErr);
    
    // Check that the admin has sufficient privileges
    // Remove sequence from project's playlist
    let project = try!(utils::read_protonfile(None::<P>));
    let new_project = try!(project.remove_sequence(seqid));

    // Remove sequence's music file if not used elsewhere in playlist

    // Save project
    try!(utils::write_protonfile(&new_project, None::<P>));

    // Commit changes
    let signature = Signature::now("Proton Lights", "proton@teslaworks.net").unwrap();
    let msg = format!("Removing sequence '{}' from playlist", seqid);
    let repo_path: Option<P> = None;

    utils::commit_all(repo_path, &signature, &msg)
        .map(|_| ())
}

/// Deletes sequence from storage
pub fn delete_sequence<P: AsRef<Path>, PD: PermissionDao, UD: UserDao, SD: SequenceDao> (
    perm_dao: &PD,
    user_dao: &UD,
    seq_dao: &SD,
    admin_key_path: P,
    seqid: u32
) -> Result<(), Error> {

    // Check admin permission
    // Check that sequence exists
    // Try to delete sequence
    Err(Error::TodoErr)
}

/// Retrieves the given sequence
pub fn get_sequence<SD: SequenceDao>(seq_dao: &SD, seqid: u32) -> Result<Sequence, Error> {
    seq_dao.get_sequence(seqid)
}

/// Check that the music file is a valid format
/// Full list of supported formats can be found at
/// http://www.rust-sfml.org/doc/rsfml/audio/struct.Music.html
fn validate_file_type<P: AsRef<Path>>(music_file_path: P) -> Result<(), Error> {
    match music_file_path.as_ref().extension() {
        Some(extension) => {
            match extension.to_str() {
                Some("ogg")  |
                Some("wav")  |
                Some("flac") |
                Some("aiff") |
                Some("raw") => Ok(()),
                None => Err(
                    Error::UnsupportedFileType("Extension is not valid unicode".to_string())
                    ),
                Some(ext) => Err(Error::UnsupportedFileType(ext.to_string())),
            }
        },
        None => Err(Error::UnsupportedFileType("No file extension".to_string())),
    }
}

/// Copies the file at music_file_path to the current directory
/// Throw error if file does not exist
///
/// Impure.
fn copy_music_file<P: AsRef<Path>>(
    music_file_path: P,
    music_file_name: &str,
    dest_folder: &str
) -> Result<PathBuf, Error> {

    // Make sure source file exists
    if !music_file_path.as_ref().exists() {
        Err(music_file_not_found(music_file_path))
    } else {
        let dest_path = Path::new(&dest_folder).join(&music_file_name);
        fs::copy(&music_file_path, &dest_path)
            .map_err(Error::Io)
            .map(|_| PathBuf::from(dest_path))
    }

}

fn music_file_not_found<P: AsRef<Path>>(path: P) -> Error {
    let path_as_str = path.as_ref().to_str().expect("Path not valid UTF-8");
    Error::MusicFileNotFound(path_as_str.to_string())
}

/// Extracts the duration of a music file
fn get_music_duration_sec<P: AsRef<Path>>(path: P) -> Result<u32, Error> {
    let path_str = &path.as_ref().to_str().expect("Path is invalid");
    let music = match Music::new_from_file(&path_str) {
        Some(m) => m,
        None => return Err(Error::Rsfml("Error reading file.".to_string())),
    };
    let duration_time = music.get_duration();
    let duration = duration_time.as_seconds() as u32;
    Ok(duration)
}
