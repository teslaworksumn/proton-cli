
#[derive(Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
    WarmWhite,
    TrueWhite,
}

#[derive(Debug)]
pub struct Channel {
    pub chanid: u32,
    pub name: String,
    pub numbers: (u32, u32), // Primary and secondary numbers (net lights)
    pub fixid: u32,
    pub color: Color,
    pub channel_internal: u32,
    pub channel_dmx: u32,
}
