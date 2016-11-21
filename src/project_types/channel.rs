
#[derive(Debug)]
pub struct Channel {
    pub chanid: u32,
    pub name: String,
    pub numbers: (u32, u32), // Primary and secondary numbers (net lights)
    pub color: String,
    pub channel_dmx: u32,
    pub location: (i32, i32, i32),
    pub rotation: (i32, i32, i32),
}
