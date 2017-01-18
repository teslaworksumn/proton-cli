
#[derive(Debug)]
pub struct Channel {
    pub chanid: u32,
    pub name: String,
    pub numbers: (Option<u32>, Option<u32>), // Primary and secondary numbers (net lights)
    pub color: String,
    pub channel_internal: u32,
    pub channel_dmx: u32,
    pub location: (Option<i32>, Option<i32>, Option<i32>),
    pub rotation: (Option<i32>, Option<i32>, Option<i32>),
}
