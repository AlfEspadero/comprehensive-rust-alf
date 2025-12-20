mod ffi {
	use std::os::raw::{c_char, c_int};
	#[cfg(not(target_os = "macos"))]
	use std::os::raw::{c_long, c_uchar, c_ulong, c_ushort};

	// Opaque type. See https://doc.rust-lang.org/nomicon/ffi.html.
	#[repr(C)]
	pub struct Dir {
		_data: [u8; 0],
		_marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
	}

	// Layout according to the Linux man page for readdir(3), where ino_t and
	// off_t are resolved according to the definitions in
	// /usr/include/x86_64-linux-gnu/{sys/types.h, bits/typesizes.h}.
	#[cfg(not(target_os = "macos"))]
	#[repr(C)]
	pub struct Dirent {
		pub d_ino: c_ulong,
		pub d_off: c_long,
		pub d_reclen: c_ushort,
		pub d_type: c_uchar,
		pub d_name: [c_char; 256],
	}

	// Layout according to the macOS man page for dir(5).
	#[cfg(target_os = "macos")]
	#[repr(C)]
	pub struct dirent {
		pub d_fileno: u64,
		pub d_seekoff: u64,
		pub d_reclen: u16,
		pub d_namlen: u16,
		pub d_type: u8,
		pub d_name: [c_char; 1024],
	}

	unsafe extern "C" {
		pub unsafe fn opendir(s: *const c_char) -> *mut Dir;

		#[cfg(not(all(target_os = "macos", target_arch = "x86_64")))]
		pub unsafe fn readdir(s: *mut Dir) -> *const Dirent;

		// See https://github.com/rust-lang/libc/issues/414 and the section on
		// _DARWIN_FEATURE_64_BIT_INODE in the macOS man page for stat(2).
		//
		// "Platforms that existed before these updates were available" refers
		// to macOS (as opposed to iOS / wearOS / etc.) on Intel and PowerPC.
		#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
		#[link_name = "readdir$INODE64"]
		pub unsafe fn readdir(s: *mut Dir) -> *const Dirent;

		pub unsafe fn closedir(s: *mut Dir) -> c_int;
	}
}

use std::ffi::{CStr, CString, OsStr, OsString};
use std::os::unix::ffi::OsStrExt;

#[derive(Debug)]
struct DirectoryIterator {
	path: CString,
	dir: *mut ffi::Dir,
}

impl DirectoryIterator {
	fn new(path: &str) -> Result<DirectoryIterator, String> {
		// Call opendir and return a Ok value if that worked,
		// otherwise return Err with a message.

		match CString::new(path) {
			Ok(c_path) => {
				let dir_ptr = unsafe { ffi::opendir(c_path.as_ptr()) };
				if dir_ptr.is_null() {
					Err(format!("Failed to open directory: {}", path))
				} else {
					Ok(DirectoryIterator {
						path: c_path,
						dir: dir_ptr,
					})
				}
			}
			Err(_) => Err(format!("Failed to convert path to CString: {}", path)),
		}
	}
}

impl Iterator for DirectoryIterator {
	type Item = OsString;
	fn next(&mut self) -> Option<OsString> {
		// Keep calling readdir until we get a NULL pointer back.
		let dirent_ptr = unsafe { ffi::readdir(self.dir) };
		if dirent_ptr.is_null() {
			None
		} else {
			// Convert the d_name field to an OsString and return it.
			let d_name = unsafe { CStr::from_ptr((*dirent_ptr).d_name.as_ptr()) };
			let os_string = OsStr::from_bytes(d_name.to_bytes()).to_os_string();
			Some(os_string.to_owned())
		}
	}
}

impl Drop for DirectoryIterator {
	fn drop(&mut self) {
		// Call closedir as needed.
		if unsafe { ffi::closedir(self.dir) } != 0 {
			panic!("Failed to close directory: {}", self.path.to_string_lossy());
		}
	}
}

pub fn wrapper_test() -> Result<(), String> {
	let iter = DirectoryIterator::new(".")?;
	println!("files: {:#?}", iter.collect::<Vec<_>>());
	Ok(())
}
