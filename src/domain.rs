use std::path::PathBuf;

pub struct LicenseArgs {
    pub filepath: PathBuf,
    pub osi: bool,
    pub fsf: bool,
    pub deprecated: bool,
}
