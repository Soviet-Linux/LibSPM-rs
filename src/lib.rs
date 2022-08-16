extern crate libc;
use std::os::raw::c_float;
// rust stuff version

extern "C" {
    fn version() -> libc::c_float;
    fn clean();
    fn update();
}

mod spm {
//  import libspm.rs


const VERSION: &str = "0.01";



    pub fn get_version() -> f32 {
        unsafe {
            crate::version()
        }
    }

    pub fn clean_dirs() {
        unsafe {
            crate::clean();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_test() {
        println!("{}", spm::get_version());
    }

    #[test]
    fn test_update() {
        unsafe {
            spm::clean_dirs()
        }
    }
}
