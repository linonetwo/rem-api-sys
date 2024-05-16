#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(unused_variables, unused_mut)]
#![allow(clippy::explicit_auto_deref)]

mod generated;
pub use generated::api_wrapper;
pub use generated::bindings;
use generated::bindings::CThostFtdcTraderApi_CreateFtdcTraderApi;
use generated::bindings::CThostFtdcTraderSpi;
use generated::bindings::{CThostFtdcTraderApi};
pub use generated::spi_wrapper;

mod ffi_utils;
pub use ffi_utils::*;
use generated::spi_wrapper::create_spi;
use generated::spi_wrapper::CThostFtdcTraderSpiStream;
use std::ffi::{CStr, CString};

pub fn create_local_api(flow_path: &str) -> Box<CThostFtdcTraderApi> {
    let trade_flow_path = std::ffi::CString::new(flow_path).unwrap();
    unsafe {
        Box::from_raw(CThostFtdcTraderApi_CreateFtdcTraderApi(
            trade_flow_path.as_ptr(),
        ))
    }
}

pub fn create_local_api_and_spi(config_filename: &str) -> (Box<CThostFtdcTraderApi>, Box<CThostFtdcTraderSpiStream<'static>>) {
    let mut api = create_local_api(config_filename);

    // Initialize the SPI and get the stream
    let (spi_stream, mut spi_ptr) = create_spi();

    // Convert the raw pointer back to a mutable reference for `CThostFtdcTraderSpi`
    // This assumes that `CThostFtdcTraderSpiStream` can be safely treated as `CThostFtdcTraderSpi`.
    // This is a crucial assumption and needs to be validated.
    unsafe {
        let spi_ref = &mut *(spi_ptr as *mut CThostFtdcTraderSpi);  // Cast `*mut CThostFtdcTraderSpiStream` to `*mut CThostFtdcTraderSpi`
        api.register_spi(spi_ref);
    }

    (api, spi_stream)
}

