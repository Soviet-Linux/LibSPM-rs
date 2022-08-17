//! `LibSPM-rs` is a rust wrapper for [LibSPM](https://github.com/Soviet-Linux/CCCP)
//!


extern crate libc;
use std::os::raw::c_float;
// rust stuff version

extern "C" {
    fn version() -> libc::c_float;
    fn clean();
    fn update();
}

//  import libspm.rs


const VERSION: &str = "0.01";
                             
    ///Checks if all dependencies are installed
    pub fn init() {
        if !std::path::Path::new("/lib/libspm.so").exists() {
            println!("LibSPM missing! Please install it!");
            println!("From here: https://github.com/Soviet-Linux/CCCP")
        } 

    }
    /// Gets the current version
    pub fn get_version() -> f32 {
        unsafe {
            crate::version()
        }
    }
    /// Cleans work dirs
    pub fn clean_dirs() {
        unsafe {
            crate::clean();
        }
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_test() {
        println!("{}", get_version());
    }

    #[test]
    fn test_update() {
            clean_dirs()
    }
}
