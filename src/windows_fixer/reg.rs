use crate::Result;
use std::ffi::OsString;

// TODO?: make it public later on possibly
fn set_owner(reg_path: &str) -> Result<()> {
    let mut sd = [0u8; 1024];

    /* println!("Setting owner not implemented yet"); */
    Ok(())
}
