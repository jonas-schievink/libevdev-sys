# Rust FFI bindings for libevdev

This crate provides generated FFI bindings for `libevdev` and `linux/input.h`.

To regenerate the bindings, run
```
bindgen --ctypes-prefix=libc --builtins /usr/include/libevdev-1.0/libevdev/libevdev.h -o src/gen.rs
```
and clean everything up. Should only take a week.
