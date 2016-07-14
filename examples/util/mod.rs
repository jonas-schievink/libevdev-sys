use libevdev_sys::evdev::*;

use std::os::unix::fs::FileTypeExt;
use std::os::unix::io::IntoRawFd;
use std::fs::{File, read_dir};
use std::io::{stdin, stdout};
use std::io::prelude::*;
use std::ptr;
use std::ffi::CStr;

/// Makes the user select a device, which is then opened and returned.
pub fn open() -> *mut libevdev {
    let paths = read_dir("/dev/input/").unwrap().filter_map(|entry| {
        let entry = entry.unwrap();
        let file_type = entry.file_type().unwrap();
        if file_type.is_block_device() || file_type.is_char_device() {
            Some(entry.path())
        } else {
            None
        }
    }).collect::<Vec<_>>();

    if paths.is_empty() {
        panic!("no suitable device found!");
    }

    for (i, path) in paths.iter().enumerate() {
        println!("{}\t{}", i, path.display());
    }

    print!("Select device to open [0-{}]: ", paths.len());
    stdout().flush().unwrap();

    let stdin = stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();

    let index = line.parse::<usize>().expect("input not a valid number");

    println!("opening {}", paths[index].display());

    let file = File::open(&paths[index]).unwrap();
    let fd = file.into_raw_fd();

    let mut evdev: *mut libevdev = ptr::null_mut();
    assert_eq!(unsafe { libevdev_new_from_fd(fd, &mut evdev) }, 0, "opening device failed");

    let name = unsafe { libevdev_get_name(evdev) };
    println!("device name: {}", unsafe { CStr::from_ptr(name) }.to_str().unwrap());

    evdev
}
