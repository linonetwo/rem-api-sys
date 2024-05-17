#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(unused_variables, unused_mut)]
#![allow(clippy::explicit_auto_deref)]

pub mod generated {
    pub mod api_wrapper;
    pub mod bindings;
    pub mod spi_wrapper;
}

pub use generated::api_wrapper;
pub use generated::bindings::*;
pub use generated::spi_wrapper::*;

mod ffi_utils;
pub use ffi_utils::*;

pub fn create_local_api(flow_path: &str) -> Box<CThostFtdcTraderApi> {
    let trade_flow_path = std::ffi::CString::new(flow_path).unwrap();
    unsafe {
        Box::from_raw(CThostFtdcTraderApi_CreateFtdcTraderApi(
            trade_flow_path.as_ptr(),
        ))
    }
}

pub fn get_api_version() -> *const std::os::raw::c_char {
    unsafe { CThostFtdcTraderApi_GetApiVersion() }
}

pub fn create_local_api_and_spi(
    flow_path: &str,
) -> (
    Box<CThostFtdcTraderApi>,
    Box<CThostFtdcTraderSpiStream<'static>>,
) {
    let mut api = create_local_api(flow_path);

    // Initialize the SPI and get the stream
    let mut stream = {
        let (stream, pp) = create_spi();
        api.register_spi(pp);
        stream
    };

    (api, stream)
}

#[derive(Debug, Clone)]
pub struct FakeMarketQuote {
    pub instrument_id: String,
    pub bid_price: f64,
    pub ask_price: f64,
    pub quote_ref: String,
    pub last_price: String,
    pub settlement_price: String,
    pub upper_limit_price: String,
    pub lower_limit_price: String,
    pub business_unit: String,
    pub volume: i32,
}

impl CThostFtdcTraderApi {
    pub fn insert_market_quote(
        &mut self,
        quote: FakeMarketQuote,
    ) -> Result<(), std::ffi::NulError> {
        let mut quote_field = CThostFtdcInputQuoteField::default();

        // Helper function to copy CString into a fixed-size array
        fn copy_to_fixed_array(target: &mut [i8], source: &std::ffi::CString) {
            let bytes = source.as_bytes_with_nul(); // Gets a slice of u8
            let len = bytes.len().min(target.len());
            // Convert u8 to i8 and copy to target array
            for (i, &byte) in bytes.iter().enumerate().take(len) {
                target[i] = byte as i8; // Convert u8 to i8
            }
            if len < target.len() {
                target[len..].fill(0); // Zero-fill the rest of the array
            }
        }

        let instrument_id_cstring = std::ffi::CString::new(quote.instrument_id)?;
        copy_to_fixed_array(&mut quote_field.InstrumentID, &instrument_id_cstring);

        quote_field.BidPrice = quote.bid_price;
        quote_field.AskPrice = quote.ask_price;

        let quote_ref_cstring = std::ffi::CString::new(quote.quote_ref)?;
        copy_to_fixed_array(&mut quote_field.QuoteRef, &quote_ref_cstring);

        let last_price_cstring = std::ffi::CString::new(quote.last_price)?;
        copy_to_fixed_array(&mut quote_field.UserID, &last_price_cstring);

        let settlement_price_cstring = std::ffi::CString::new(quote.settlement_price)?;
        copy_to_fixed_array(&mut quote_field.ForQuoteSysID, &settlement_price_cstring);

        let upper_limit_price_cstring = std::ffi::CString::new(quote.upper_limit_price)?;
        copy_to_fixed_array(&mut quote_field.BidOrderRef, &upper_limit_price_cstring);

        let lower_limit_price_cstring = std::ffi::CString::new(quote.lower_limit_price)?;
        copy_to_fixed_array(&mut quote_field.AskOrderRef, &lower_limit_price_cstring);

        let business_unit_cstring = std::ffi::CString::new(quote.business_unit)?;
        copy_to_fixed_array(&mut quote_field.BusinessUnit, &business_unit_cstring);

        self.req_quote_insert(&mut quote_field, quote.volume);

        Ok(())
    }
}


#[derive(Debug, PartialEq, Clone)]
pub enum DirectionType {
    Long = 0,
    Short = 1,
}

impl Default for DirectionType {
    fn default() -> Self {
        return DirectionType::Long;
    }
}
impl DirectionType {
    pub fn opposite(&self) -> DirectionType {
        match *self {
            Self::Long => Self::Short,
            Self::Short => Self::Long,
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub enum OffsetFlag {
    Open,
    Close,
    CloseToday,
    CloseYesterday,
    OfOther,
}

/// 发送委托
#[derive(Debug, Clone)]
pub struct InputOrderField {
    pub direction: DirectionType,
    pub offset: OffsetFlag,
    pub price: f64,
    pub volume: i32,
}

/// 发送撤单
#[derive(Clone)]
pub struct InputOrderActionField {
    pub front_id: i32,
    pub session_id: i32,
    pub order_ref: [i8; 13],
}


#[derive(Clone, Debug, Default)]
pub struct ShareholderAccount {
    pub investor_id: String,
    pub exchange_id: String,
    pub shareholder_id: String,
    pub shareholder_id_type: i8,
    pub market_id: i8,
}

#[derive(Debug)]
pub enum Error {
    AccountNotFound,
    InstrumentNotFound,
    DumplicateTrade,
    FrontDisconnected,
    CtpAuthFailed,
    MpscSendErr,
    InvalidCtpInstrumentId,
    CtpLastQueryIsProceeding,
    CtpQueryTimeout,
    InvalidSymbol,
    MdNotFound,
    TraderApiErr(i32),
    SimpleErr(simple_error::SimpleError),
    OrderNotFound,
    ShareholderAccountNotFound,
}


pub trait TraderApiType {
    fn req_order_insert(
        &mut self,
        broker_id: &str,
        account: &str,
        exchange: &str,
        symbol: &str,
        i: &InputOrderField,
        order_ref: i32,
        n_request_id: i32,
        shareholder_accounts: &Vec<ShareholderAccount>,
    ) -> Result<(), Error>;
    fn req_order_action(
        &mut self,
        broker_id: &str,
        account: &str,
        exchange: &str,
        symbol: &str,
        i: &InputOrderActionField,
        order_action_ref: i32,
        n_request_id: i32,
    ) -> Result<(), Error>;
}


impl TraderApiType for CThostFtdcTraderApi {
    fn req_order_insert(
        &mut self,
        broker_id: &str,
        account: &str,
        exchange: &str,
        symbol: &str,
        i: &InputOrderField,
        order_ref: i32,
        n_request_id: i32,
        _shareholder_accounts: &Vec<ShareholderAccount>,
    ) -> Result<(), Error> {
        let mut input = CThostFtdcInputOrderField::default();
        set_cstr_from_str_truncate_i8(&mut input.BrokerID, broker_id);
        set_cstr_from_str_truncate_i8(&mut input.UserID, account);
        set_cstr_from_str_truncate_i8(&mut input.InvestorID, account);
        set_cstr_from_str_truncate_i8(&mut input.ExchangeID, exchange);
        set_cstr_from_str_truncate_i8(&mut input.InstrumentID, symbol);
        input.Direction = match i.direction {
            DirectionType::Long => THOST_FTDC_D_Buy,
            DirectionType::Short => THOST_FTDC_D_Sell,
        } as i8;
        input.CombOffsetFlag[0] = match i.offset {
            OffsetFlag::Open => THOST_FTDC_OF_Open,
            OffsetFlag::Close => THOST_FTDC_OF_Close,
            OffsetFlag::CloseToday => THOST_FTDC_OF_CloseToday,
            OffsetFlag::CloseYesterday => THOST_FTDC_OF_CloseYesterday,
            OffsetFlag::OfOther => panic!("Invalid offset"),
        } as i8;
        input.CombHedgeFlag[0] = THOST_FTDC_HF_Speculation as i8;
        input.LimitPrice = i.price;
        input.VolumeTotalOriginal = i.volume;
        input.OrderPriceType = THOST_FTDC_OPT_LimitPrice as i8;
        if input.LimitPrice == 0.0 {
            input.OrderPriceType = THOST_FTDC_OPT_AnyPrice as i8;
        }
        input.TimeCondition = THOST_FTDC_TC_GFD as i8;
        input.VolumeCondition = THOST_FTDC_VC_AV as i8;
        input.MinVolume = 1;
        input.ContingentCondition = THOST_FTDC_CC_Immediately as i8;
        input.ForceCloseReason = THOST_FTDC_FCC_NotForceClose as i8;
        input.IsAutoSuspend = 0;
        input.UserForceClose = 0;
        set_cstr_from_str_truncate_i8(&mut input.OrderRef, &format!("{order_ref}"));
        let ret = self.req_order_insert(&mut input, n_request_id);
        if ret != 0 {
            return Err(Error::TraderApiErr(ret));
        }
        Ok(())
    }

    fn req_order_action(
        &mut self,
        broker_id: &str,
        account: &str,
        exchange: &str,
        symbol: &str,
        i: &InputOrderActionField,
        _order_action_ref: i32,
        n_request_id: i32,
    ) -> Result<(), Error> {
        let mut r = CThostFtdcInputOrderActionField::default();
        set_cstr_from_str_truncate_i8(&mut r.ExchangeID, exchange);
        set_cstr_from_str_truncate_i8(&mut r.InstrumentID, symbol);
        set_cstr_from_str_truncate_i8(&mut r.BrokerID, broker_id);
        set_cstr_from_str_truncate_i8(&mut r.UserID, account);
        set_cstr_from_str_truncate_i8(&mut r.InvestorID, account);
        r.FrontID = i.front_id;
        r.SessionID = i.session_id;
        r.OrderRef = i.order_ref.clone();
        r.ActionFlag = THOST_FTDC_AF_Delete as i8;
        let ret = self.req_order_action(&mut r, n_request_id);
        if ret != 0 {
            return Err(Error::TraderApiErr(ret));
        }
        Ok(())
    }
}