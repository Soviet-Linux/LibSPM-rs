//! `LibSPM-rs` is a rust wrapper for [LibSPM](https://github.com/Soviet-Linux/CCCP)
//!

extern crate libc;
// rust stuff version

extern "C" {
    fn version() -> libc::c_float;
    fn clean();
    fn update();
    // using c_uchar here because String::as_ptr() returns a *const u8 pointer, so it's pretty easy
    fn get(p_name: *const libc::c_uchar, out_path: *const libc::c_uchar);
    fn uninstall(p_name: *const libc::c_uchar, p_path: *const libc::c_uchar);
}

//  import libspm.rs

const VERSION: &str = "0.03";

///Checks if all dependencies are installed
pub fn init() {
    if !std::path::Path::new("/lib/libspm.so").exists() {
        println!("LibSPM missing! Please install it!");
        println!("From here: https://github.com/Soviet-Linux/CCCP")
    }
}
/// Gets the current version
pub fn get_version() -> f32 {
    unsafe { crate::version() }
}
/// Cleans work dirs
pub fn clean_dirs() {
    unsafe {
        crate::clean();
    }
}
/// Gets a package by name
pub fn get_package(package_name: String, out_path: String) {
    unsafe {
        get(package_name.as_ptr(), out_path.as_ptr());
    }
}

/// Uninstall a package by name (and path)
pub fn uninstall_package(package_name: String, package_path: String) {
    unsafe {
        uninstall(package_name.as_ptr(), package_path.as_ptr());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_test() {
        println!(
            "LibSPM version: {}\nLibSPM-rs wrapper version: {}",
            get_version(),
            VERSION
        );
    }

    #[test]
    fn test_clean() {
        clean_dirs()
    }

    // this test shouldn't be ran as it promps the user for input (which might also cause it to
    // panic or crash)
    // #[test]
    // fn test_get() {
    //     get_package("testing".to_string(), "./fake".to_string());
    // }
}
