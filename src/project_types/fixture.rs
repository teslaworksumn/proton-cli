
/// Holds metadata for a Fixture, which is logically a collection of channels 
/// that can be used as a unit (e.g. sunbursts, net light)
#[derive(Debug)]
pub struct Fixture {
    pub fixid: u32,
    pub name: String,
    pub location: (i32, i32, i32),
    pub rotation: (i32, i32, i32),
    pub channels: Vec<u32>,
}

impl Fixture {
    pub fn new(
        name: &str,
        location: (i32, i32, i32),
        rotation: (i32, i32, i32),
        channels: Vec<u32>
    ) -> Fixture {
        Fixture {
            fixid: 0, // Default, set by dao
            name: name.to_owned(),
            location: location,
            rotation: rotation,
            channels: channels
        }
    }
}
