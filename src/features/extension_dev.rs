use std::error::Error;
use std::path::Path;

pub struct ExtensionDev;

impl ExtensionDev {
    pub async fn load_unpacked(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    pub async fn reload_extension(&mut self, id: &str) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    pub async fn debug_extension(&mut self, id: &str) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
