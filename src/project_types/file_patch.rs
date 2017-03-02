use error::Error;

/// Mapping for patch JSON object
#[derive(Debug, RustcDecodable)]
pub struct FilePatch {
    pub patches: Vec<FilePatchRow>,
}

/// Mapping for one row (patch) in the patch JSON object
#[derive(Debug, RustcDecodable)]
#[allow(non_snake_case)]
pub struct FilePatchRow {
    pub internalChannel: u32,
    pub dmxChannel: u32
}

impl FilePatch {
    /// Check that all channels are valid
    pub fn validate(&self) -> Result<(), Error> {

        for patch in &self.patches {
            // Make sure internal channel > 0 (indexed same as DMX)
            if patch.internalChannel < 1 {
                return Err(Error::InvalidPatch(String::from("Internal channels start at 1, not 0")))
            }

            // Make sure DMX > 0
            if patch.dmxChannel < 1 {
                return Err(Error::InvalidPatch(String::from("DMX channels start at 1, not 0")))
            }
        }
        Ok(())
    }
}
