use libc::size_t;
use linux_input::{input_event, input_absinfo};

/// AFAIK there's no useful and officially supported way a Rust function can accept a C `va_list`,
/// so you won't be able to use a custom logging function.
pub enum va_list {}

/* automatically generated by rust-bindgen */

pub enum libevdev { }
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum libevdev_read_flag {
    LIBEVDEV_READ_FLAG_SYNC = 1,
    LIBEVDEV_READ_FLAG_NORMAL = 2,
    LIBEVDEV_READ_FLAG_FORCE_SYNC = 4,
    LIBEVDEV_READ_FLAG_BLOCKING = 8,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum libevdev_log_priority {
    LIBEVDEV_LOG_ERROR = 10,
    LIBEVDEV_LOG_INFO = 20,
    LIBEVDEV_LOG_DEBUG = 30,
}
pub type libevdev_log_func_t =
    ::std::option::Option<unsafe extern "C" fn(priority:
                                                   libevdev_log_priority,
                                               data: *mut ::libc::c_void,
                                               file: *const ::libc::c_char,
                                               line: ::libc::c_int,
                                               func: *const ::libc::c_char,
                                               format: *const ::libc::c_char,
                                               args: va_list)>;
pub type libevdev_device_log_func_t =
    ::std::option::Option<unsafe extern "C" fn(dev: *const libevdev,
                                               priority:
                                                   libevdev_log_priority,
                                               data: *mut ::libc::c_void,
                                               file: *const ::libc::c_char,
                                               line: ::libc::c_int,
                                               func: *const ::libc::c_char,
                                               format: *const ::libc::c_char,
                                               args: va_list)>;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum libevdev_grab_mode { LIBEVDEV_GRAB = 3, LIBEVDEV_UNGRAB = 4, }
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum libevdev_read_status {
    LIBEVDEV_READ_STATUS_SUCCESS = 0,
    LIBEVDEV_READ_STATUS_SYNC = 1,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum libevdev_led_value { LIBEVDEV_LED_ON = 3, LIBEVDEV_LED_OFF = 4, }
#[link(name = "evdev")]
extern "C" {
    pub fn libevdev_new() -> *mut libevdev;
    pub fn libevdev_new_from_fd(fd: ::libc::c_int, dev: *mut *mut libevdev)
     -> ::libc::c_int;
    pub fn libevdev_free(dev: *mut libevdev);
    pub fn libevdev_set_log_function(logfunc: libevdev_log_func_t,
                                     data: *mut ::libc::c_void);
    pub fn libevdev_set_log_priority(priority: libevdev_log_priority);
    pub fn libevdev_get_log_priority() -> libevdev_log_priority;
    pub fn libevdev_set_device_log_function(dev: *mut libevdev,
                                            logfunc:
                                                libevdev_device_log_func_t,
                                            priority: libevdev_log_priority,
                                            data: *mut ::libc::c_void);
    pub fn libevdev_grab(dev: *mut libevdev, grab: libevdev_grab_mode)
     -> ::libc::c_int;
    pub fn libevdev_set_fd(dev: *mut libevdev, fd: ::libc::c_int)
     -> ::libc::c_int;
    pub fn libevdev_change_fd(dev: *mut libevdev, fd: ::libc::c_int)
     -> ::libc::c_int;
    pub fn libevdev_get_fd(dev: *const libevdev) -> ::libc::c_int;
    pub fn libevdev_next_event(dev: *mut libevdev, flags: ::libc::c_uint,
                               ev: *mut input_event) -> ::libc::c_int;
    pub fn libevdev_has_event_pending(dev: *mut libevdev) -> ::libc::c_int;
    pub fn libevdev_get_name(dev: *const libevdev) -> *const ::libc::c_char;
    pub fn libevdev_set_name(dev: *mut libevdev, name: *const ::libc::c_char);
    pub fn libevdev_get_phys(dev: *const libevdev) -> *const ::libc::c_char;
    pub fn libevdev_set_phys(dev: *mut libevdev, phys: *const ::libc::c_char);
    pub fn libevdev_get_uniq(dev: *const libevdev) -> *const ::libc::c_char;
    pub fn libevdev_set_uniq(dev: *mut libevdev, uniq: *const ::libc::c_char);
    pub fn libevdev_get_id_product(dev: *const libevdev) -> ::libc::c_int;
    pub fn libevdev_set_id_product(dev: *mut libevdev,
                                   product_id: ::libc::c_int);
    pub fn libevdev_get_id_vendor(dev: *const libevdev) -> ::libc::c_int;
    pub fn libevdev_set_id_vendor(dev: *mut libevdev,
                                  vendor_id: ::libc::c_int);
    pub fn libevdev_get_id_bustype(dev: *const libevdev) -> ::libc::c_int;
    pub fn libevdev_set_id_bustype(dev: *mut libevdev,
                                   bustype: ::libc::c_int);
    pub fn libevdev_get_id_version(dev: *const libevdev) -> ::libc::c_int;
    pub fn libevdev_set_id_version(dev: *mut libevdev,
                                   version: ::libc::c_int);
    pub fn libevdev_get_driver_version(dev: *const libevdev) -> ::libc::c_int;
    pub fn libevdev_has_property(dev: *const libevdev, prop: ::libc::c_uint)
     -> ::libc::c_int;
    pub fn libevdev_enable_property(dev: *mut libevdev, prop: ::libc::c_uint)
     -> ::libc::c_int;
    pub fn libevdev_has_event_type(dev: *const libevdev,
                                   type_: ::libc::c_uint) -> ::libc::c_int;
    pub fn libevdev_has_event_code(dev: *const libevdev,
                                   type_: ::libc::c_uint,
                                   code: ::libc::c_uint) -> ::libc::c_int;
    pub fn libevdev_get_abs_minimum(dev: *const libevdev,
                                    code: ::libc::c_uint) -> ::libc::c_int;
    pub fn libevdev_get_abs_maximum(dev: *const libevdev,
                                    code: ::libc::c_uint) -> ::libc::c_int;
    pub fn libevdev_get_abs_fuzz(dev: *const libevdev, code: ::libc::c_uint)
     -> ::libc::c_int;
    pub fn libevdev_get_abs_flat(dev: *const libevdev, code: ::libc::c_uint)
     -> ::libc::c_int;
    pub fn libevdev_get_abs_resolution(dev: *const libevdev,
                                       code: ::libc::c_uint) -> ::libc::c_int;
    pub fn libevdev_get_abs_info(dev: *const libevdev, code: ::libc::c_uint)
     -> *const input_absinfo;
    pub fn libevdev_get_event_value(dev: *const libevdev,
                                    type_: ::libc::c_uint,
                                    code: ::libc::c_uint) -> ::libc::c_int;
    pub fn libevdev_set_event_value(dev: *mut libevdev, type_: ::libc::c_uint,
                                    code: ::libc::c_uint,
                                    value: ::libc::c_int) -> ::libc::c_int;
    pub fn libevdev_fetch_event_value(dev: *const libevdev,
                                      type_: ::libc::c_uint,
                                      code: ::libc::c_uint,
                                      value: *mut ::libc::c_int)
     -> ::libc::c_int;
    pub fn libevdev_get_slot_value(dev: *const libevdev, slot: ::libc::c_uint,
                                   code: ::libc::c_uint) -> ::libc::c_int;
    pub fn libevdev_set_slot_value(dev: *mut libevdev, slot: ::libc::c_uint,
                                   code: ::libc::c_uint, value: ::libc::c_int)
     -> ::libc::c_int;
    pub fn libevdev_fetch_slot_value(dev: *const libevdev,
                                     slot: ::libc::c_uint,
                                     code: ::libc::c_uint,
                                     value: *mut ::libc::c_int)
     -> ::libc::c_int;
    pub fn libevdev_get_num_slots(dev: *const libevdev) -> ::libc::c_int;
    pub fn libevdev_get_current_slot(dev: *const libevdev) -> ::libc::c_int;
    pub fn libevdev_set_abs_minimum(dev: *mut libevdev, code: ::libc::c_uint,
                                    min: ::libc::c_int);
    pub fn libevdev_set_abs_maximum(dev: *mut libevdev, code: ::libc::c_uint,
                                    max: ::libc::c_int);
    pub fn libevdev_set_abs_fuzz(dev: *mut libevdev, code: ::libc::c_uint,
                                 fuzz: ::libc::c_int);
    pub fn libevdev_set_abs_flat(dev: *mut libevdev, code: ::libc::c_uint,
                                 flat: ::libc::c_int);
    pub fn libevdev_set_abs_resolution(dev: *mut libevdev,
                                       code: ::libc::c_uint,
                                       resolution: ::libc::c_int);
    pub fn libevdev_set_abs_info(dev: *mut libevdev, code: ::libc::c_uint,
                                 abs: *const input_absinfo);
    pub fn libevdev_enable_event_type(dev: *mut libevdev,
                                      type_: ::libc::c_uint) -> ::libc::c_int;
    pub fn libevdev_disable_event_type(dev: *mut libevdev,
                                       type_: ::libc::c_uint)
     -> ::libc::c_int;
    pub fn libevdev_enable_event_code(dev: *mut libevdev,
                                      type_: ::libc::c_uint,
                                      code: ::libc::c_uint,
                                      data: *const ::libc::c_void)
     -> ::libc::c_int;
    pub fn libevdev_disable_event_code(dev: *mut libevdev,
                                       type_: ::libc::c_uint,
                                       code: ::libc::c_uint) -> ::libc::c_int;
    pub fn libevdev_kernel_set_abs_info(dev: *mut libevdev,
                                        code: ::libc::c_uint,
                                        abs: *const input_absinfo)
     -> ::libc::c_int;
    pub fn libevdev_kernel_set_led_value(dev: *mut libevdev,
                                         code: ::libc::c_uint,
                                         value: libevdev_led_value)
     -> ::libc::c_int;
    pub fn libevdev_kernel_set_led_values(dev: *mut libevdev, ...)
     -> ::libc::c_int;
    pub fn libevdev_set_clock_id(dev: *mut libevdev, clockid: ::libc::c_int)
     -> ::libc::c_int;
    pub fn libevdev_event_is_type(ev: *const input_event,
                                  type_: ::libc::c_uint) -> ::libc::c_int;
    pub fn libevdev_event_is_code(ev: *const input_event,
                                  type_: ::libc::c_uint, code: ::libc::c_uint)
     -> ::libc::c_int;
    pub fn libevdev_event_type_get_name(type_: ::libc::c_uint)
     -> *const ::libc::c_char;
    pub fn libevdev_event_code_get_name(type_: ::libc::c_uint,
                                        code: ::libc::c_uint)
     -> *const ::libc::c_char;
    pub fn libevdev_property_get_name(prop: ::libc::c_uint)
     -> *const ::libc::c_char;
    pub fn libevdev_event_type_get_max(type_: ::libc::c_uint)
     -> ::libc::c_int;
    pub fn libevdev_event_type_from_name(name: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn libevdev_event_type_from_name_n(name: *const ::libc::c_char,
                                           len: size_t) -> ::libc::c_int;
    pub fn libevdev_event_code_from_name(type_: ::libc::c_uint,
                                         name: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn libevdev_event_code_from_name_n(type_: ::libc::c_uint,
                                           name: *const ::libc::c_char,
                                           len: size_t) -> ::libc::c_int;
    pub fn libevdev_property_from_name(name: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn libevdev_property_from_name_n(name: *const ::libc::c_char,
                                         len: size_t) -> ::libc::c_int;
    pub fn libevdev_get_repeat(dev: *const libevdev,
                               delay: *mut ::libc::c_int,
                               period: *mut ::libc::c_int) -> ::libc::c_int;
}
