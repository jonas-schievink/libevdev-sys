//! FFI bindings for the `linux/input.h` header.

use libc::timeval;

pub type __u8 = u8;
pub type __u16 = u16;
pub type __s16 = i16;
pub type __u32 = u32;
pub type __s32 = i32;
pub type __u64 = u64;

// (also renamed `Union_Unnamed1` to `ff_effect_union`)

/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct input_event {
    pub time: timeval,
    pub type_: __u16,
    pub code: __u16,
    pub value: __s32,
}
impl ::std::default::Default for input_event {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct input_id {
    pub bustype: __u16,
    pub vendor: __u16,
    pub product: __u16,
    pub version: __u16,
}
impl ::std::default::Default for input_id {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct input_absinfo {
    pub value: __s32,
    pub minimum: __s32,
    pub maximum: __s32,
    pub fuzz: __s32,
    pub flat: __s32,
    pub resolution: __s32,
}
impl ::std::default::Default for input_absinfo {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct input_keymap_entry {
    pub flags: __u8,
    pub len: __u8,
    pub index: __u16,
    pub keycode: __u32,
    pub scancode: [__u8; 32usize],
}
impl ::std::default::Default for input_keymap_entry {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct input_mask {
    pub type_: __u32,
    pub codes_size: __u32,
    pub codes_ptr: __u64,
}
impl ::std::default::Default for input_mask {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct ff_replay {
    pub length: __u16,
    pub delay: __u16,
}
impl ::std::default::Default for ff_replay {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct ff_trigger {
    pub button: __u16,
    pub interval: __u16,
}
impl ::std::default::Default for ff_trigger {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct ff_envelope {
    pub attack_length: __u16,
    pub attack_level: __u16,
    pub fade_length: __u16,
    pub fade_level: __u16,
}
impl ::std::default::Default for ff_envelope {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct ff_constant_effect {
    pub level: __s16,
    pub envelope: ff_envelope,
}
impl ::std::default::Default for ff_constant_effect {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct ff_ramp_effect {
    pub start_level: __s16,
    pub end_level: __s16,
    pub envelope: ff_envelope,
}
impl ::std::default::Default for ff_ramp_effect {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct ff_condition_effect {
    pub right_saturation: __u16,
    pub left_saturation: __u16,
    pub right_coeff: __s16,
    pub left_coeff: __s16,
    pub deadband: __u16,
    pub center: __s16,
}
impl ::std::default::Default for ff_condition_effect {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct ff_periodic_effect {
    pub waveform: __u16,
    pub period: __u16,
    pub magnitude: __s16,
    pub offset: __s16,
    pub phase: __u16,
    pub envelope: ff_envelope,
    pub custom_len: __u32,
    pub custom_data: *mut __s16,
}
impl ::std::default::Default for ff_periodic_effect {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct ff_rumble_effect {
    pub strong_magnitude: __u16,
    pub weak_magnitude: __u16,
}
impl ::std::default::Default for ff_rumble_effect {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct ff_effect {
    pub type_: __u16,
    pub id: __s16,
    pub direction: __u16,
    pub trigger: ff_trigger,
    pub replay: ff_replay,
    pub u: ff_effect_union,
}
impl ::std::default::Default for ff_effect {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct ff_effect_union {
    pub _bindgen_data_: [u64; 4usize],
}
impl ff_effect_union {
    pub unsafe fn constant(&mut self) -> *mut ff_constant_effect {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn ramp(&mut self) -> *mut ff_ramp_effect {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn periodic(&mut self) -> *mut ff_periodic_effect {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn condition(&mut self) -> *mut [ff_condition_effect; 2usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn rumble(&mut self) -> *mut ff_rumble_effect {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::default::Default for ff_effect_union {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
