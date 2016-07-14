//! A really dumbed-down version of the evtest utility

extern crate libevdev_sys;
extern crate libc;

mod util;

use libevdev_sys::evdev::*;
use libevdev_sys::linux_input::*;

use std::thread::sleep;
use std::time::Duration;

fn main() {
    let device = util::open();

    loop {
        // We need to set the blocking flag because we don't set the fd to O_NONBLOCK. If this isn't
        // done, we'll miss events.
        // FIXME: Figure out if O_NONBLOCK should be preferred to this
        let flags = libevdev_read_flag::LIBEVDEV_READ_FLAG_NORMAL as u32 |
                    libevdev_read_flag::LIBEVDEV_READ_FLAG_BLOCKING as u32;
        let mut ev = input_event::default();
        let ret = unsafe {
            libevdev_next_event(
                device,
                flags,
                &mut ev)
        };

        if ret == libevdev_read_status::LIBEVDEV_READ_STATUS_SUCCESS as i32 {
            println!("[{}.{}] Type {}, Code {}, Value {}",
                ev.time.tv_sec, ev.time.tv_usec, ev.type_, ev.code, ev.value);
        } else if ret == libevdev_read_status::LIBEVDEV_READ_STATUS_SYNC as i32 {
            println!("SYNC!");
            // TODO unhandled
        } else if ret == -libc::EAGAIN {
            // No events available, sleep and loop
            sleep(Duration::from_millis(20));
        } else if ret < 0 {
            panic!("failed to read event: {}", ret);
        }
    }
}
