/* automatically generated by rust-bindgen */

use
super::*;

pub const SENSOR_NETWORK_SIZE: u32 = 5;
pub type __uint8_t = ::cty::c_uchar;
pub type __uint16_t = ::cty::c_ushort;
#[repr(C)]
pub struct oc_server_handle {
    _unused: [u8; 0],
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[allow(clashing_extern_declarations)] ////TODO
    pub fn init_sensor_post(server: *mut oc_server_handle) -> bool;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    pub fn do_sensor_post() -> bool;
}
#[repr(C)]
pub struct sensor_network_interface {
    pub iface_type: u8,
    pub network_device: *const ::cty::c_char,
    pub server_endpoint_size: u8,
    pub register_transport_func: ::core::option::Option<
        unsafe extern "C" fn(
            network_device: *const ::cty::c_char,
            server_endpoint: *mut ::cty::c_void,
            host: *const ::cty::c_char,
            port: u16,
            server_endpoint_size: u8,
        ) -> ::cty::c_int,
    >,
    pub transport_registered: u8,
}
impl Default for sensor_network_interface {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = ""]
    pub fn start_server_transport() -> ::cty::c_int;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    pub fn start_collector_transport() -> ::cty::c_int;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    pub fn sensor_network_start_transport(iface_type: u8) -> ::cty::c_int;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = ""]
    pub fn register_server_transport() -> ::cty::c_int;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    pub fn register_collector_transport() -> ::cty::c_int;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    pub fn sensor_network_register_transport(iface_type: u8) -> ::cty::c_int;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    pub fn init_server_post(uri: *const ::cty::c_char) -> bool;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    pub fn init_collector_post() -> bool;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    pub fn sensor_network_init_post(iface_type: u8, uri: *const ::cty::c_char) -> bool;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    pub fn sensor_network_prepare_post(encoding: ::cty::c_int) -> bool;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    pub fn do_server_post() -> bool;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    pub fn do_collector_post() -> bool;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    pub fn sensor_network_do_post(iface_type: u8) -> bool;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    pub fn is_collector_node() -> bool;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    pub fn is_sensor_node() -> bool;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    pub fn is_standalone_node() -> bool;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    pub fn should_send_to_collector(
        val: *mut sensor_value,
        device_name: *const ::cty::c_char,
    ) -> bool;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    pub fn get_device_id() -> *const ::cty::c_char;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = ""]
    pub fn sensor_network_init();
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    pub fn sensor_network_register_interface(
        iface: *const sensor_network_interface,
    ) -> ::cty::c_int;
}
#[doc = ""]
#[repr(C)]
#[derive(Default)]
pub struct sensor_network_endpoint {
    pub endpoint: [u8; 16usize],
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    pub static mut sensor_network_interfaces: [sensor_network_interface; 2usize];
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    pub static mut sensor_network_endpoints: [sensor_network_endpoint; 2usize];
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    pub static mut sensor_network_encoding: [::cty::c_int; 2usize];
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    pub static mut sensor_network_shortname: [*const ::cty::c_char; 2usize];
}
