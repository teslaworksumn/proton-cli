
#[derive(Debug, RustcEncodable)]
pub struct PlaylistData {
    pub seqid: u32,
    pub data: Vec<Vec<u16>>
}
