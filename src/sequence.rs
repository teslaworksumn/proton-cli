//! This module manages project sequences

use std::path::{Path, PathBuf};
use std::fs;

use git2::Signature;
use regex::Regex;
use sfml::audio::Music;

use error::Error;
use project_types::Permission;
use dao::SequenceDao;
use user;
use utils;


/// Creates a new user for the project in the current directory.
/// Assumes the current directory contains a Protonfile.json file.
///
/// Impure.
pub fn new_sequence<P: AsRef<Path>>(
    admin_key_path: P,
    name: &str,
    music_file_path: P,
    frame_duration_ms: Option<u32>
) -> Result<(), Error> {
    Err(Error::TodoErr)
    
    // // Get admin from key
    // let admin_uid = 1;

    // // Make sure the music file is a valid format
    // try!(validate_file_type(&music_file_path));

    // // Get name of music file from path
    // let music_file_name = try!(utils::file_name_from_path(&music_file_path));

    // // Try to copy music file into music directory
    // let dest_path = "";

    // // Get duration of music file
    // let music_duration_sec = try!(get_music_duration_sec(&dest_path));

    // // Add sequence to project
    // let project = try!(utils::read_protonfile(None::<P>));
    // let new_project = match project.add_sequence(
    //     admin_uid,
    //     name,
    //     &music_file_name,
    //     music_duration_sec,
    //     frame_duration_ms
    // ) {
    //     Ok(proj) => proj,
    //     Err(e) => {
    //         // Remove copied music file (clean up)
    //         try!(fs::remove_file(&dest_path).map_err(Error::Io));
    //         panic!(e.to_string())
    //     },
    // };

    // // Save project
    // try!(utils::write_protonfile(&new_project, None::<P>));

    // // Commit changes
    // let signature = Signature::now("Proton Lights", "proton@teslaworks.net").unwrap();
    // let msg = format!("Adding new sequence '{}'", name);
    // let repo_path: Option<P> = None;

    // utils::commit_all(repo_path, &signature, &msg)
    //     .map(|_| ())
}

/// Removes the sequence with the given name from the project
/// and deletes its files
pub fn remove_sequence<P: AsRef<Path>>(admin_key_path: P, seqid: u32) -> Result<(), Error> {
    
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
    let msg = format!("Removing sequence '{}'", seqid);
    let repo_path: Option<P> = None;

    utils::commit_all(repo_path, &signature, &msg)
        .map(|_| ())
}

// Deletes sequence from storage
pub fn delete_sequence<P: AsRef<Path>, D: SequenceDao> (
    dao: D,
    admin_key_path: P,
    seqid: u32
) -> Result<(), Error> {

    // Check admin permission
    // Check that sequence exists
    // Try to delete sequence
    Err(Error::TodoErr)
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
fn copy_music_file<P: AsRef<Path>>(music_file_path: P, dest_folder: &str) -> Result<PathBuf, Error> {
    // Make sure source file exists
    if !music_file_path.as_ref().exists() {
        Err(music_file_not_found(music_file_path))
    } else {
        let file_name = try!(utils::file_name_from_path(&music_file_path));
        let dest_path = Path::new(&dest_folder).join(&file_name);
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
