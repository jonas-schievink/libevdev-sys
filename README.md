# Rust FFI bindings for libevdev

***DEPRECATED: There are other bindings and reimplementations of evdev functionality that are much better than this.***

This crate provides generated FFI bindings for `libevdev` and `linux/input.h`.

To regenerate the bindings, run
```
bindgen --ctypes-prefix=libc --builtins --match evdev /usr/include/libevdev-1.0/libevdev/libevdev.h > src/evdev.rs
bindgen --ctypes-prefix=libc --builtins --match input.h /usr/include/libevdev-1.0/libevdev/libevdev.h > src/linux_input.rs
```
and clean everything up. Should only take a week.
