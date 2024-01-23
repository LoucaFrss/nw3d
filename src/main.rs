#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(c_size_t)]

use eadk::{eadk_external_data, eadk_external_data_size};

pub mod app;
pub mod eadk;
pub mod gl;
pub mod glm;

#[used]
#[link_section = ".rodata.eadk_app_name"]
pub static EADK_APP_NAME: [u8; 10] = *b"HelloRust\0";

#[used]
#[link_section = ".rodata.eadk_api_level"]
pub static EADK_APP_API_LEVEL: u32 = 0;

#[used]
#[link_section = ".rodata.eadk_app_icon"]
pub static EADK_APP_ICON: [u8; 4250] = *include_bytes!("../target/icon.nwi");
#[no_mangle]
pub fn main() {
    app::app(unsafe { core::slice::from_raw_parts(eadk_external_data, eadk_external_data_size) });
}

pub static mut TEXT_BUFFER: [u8; 1024] = [0; 1024];

pub struct TextBuf<'a> {
    buf: &'a mut [u8],
    pub offset: usize,
}
impl<'a> TextBuf<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self { buf, offset: 0 }
    }
}
