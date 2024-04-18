use std::io::Write;

use bincode::{config, Decode, Encode};
use localctp_sys::*;
use futures::StreamExt;
use log::info;
use rust_share_util::*;
use std::ffi::{CStr, CString};

pub fn init_logger() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::init();
}

#[derive(Clone, Debug)]
pub struct CtpAccountConfig {
    pub broker_id: String,
    pub account: String,
    pub trade_fronts: Vec<String>,
    pub md_fronts: Vec<String>,
    pub name_servers: Vec<String>,
    pub auth_code: String,
    pub user_product_info: String,
    pub app_id: String,
    pub password: String,
    pub remark: String,
}

#[tokio::main]
async fn main() {
    init_logger();
    // let trade_front = "tcp://180.168.146.187:10130"; // 7*24
    let account = CtpAccountConfig {
        broker_id: "9999".to_string(),
        account: "143650".to_string(),
        trade_fronts: vec!["tcp://180.168.146.187:10201".to_string()],
        md_fronts: vec!["180.168.146.187:10211".to_string()],
        name_servers: vec!["".to_string()],
        auth_code: "0000000000000000".to_string(),
        user_product_info: "".to_string(),
        app_id: "simnow_client_test".to_string(),
        password: "198612".to_string(),
        remark: "".into(),
    };
    query(&account).await;
}

#[derive(Clone, Debug, Default, Encode, Decode)]
pub struct CtpQueryResult {
    broker_id: String,
    account: String,
    trading_day: i32,
    timestamp: i64,
    dmd_list: Vec<CThostFtdcDepthMarketDataField>,
    icr_list: Vec<CThostFtdcInstrumentCommissionRateField>,
    instrument_list: Vec<CThostFtdcInstrumentField>,
    position_list: Vec<CThostFtdcInvestorPositionField>,
    position_detail_list: Vec<CThostFtdcInvestorPositionDetailField>,
    trading_account: CThostFtdcTradingAccountField,
    product_list: Vec<CThostFtdcProductField>,
    order_list: Vec<CThostFtdcOrderField>,
    trade_list: Vec<CThostFtdcTradeField>,
}

async fn query(ca: &CtpAccountConfig) {
    use localctp_sys::trader_api::*;
    let broker_id = ca.broker_id.as_str();
    let account = ca.account.as_str();
    let trade_front = ca.trade_fronts[0].as_str();
    let name_server = ca.name_servers[0].as_str();
    let auth_code = ca.auth_code.as_str();
    let user_product_info = ca.user_product_info.as_str();
    let app_id = ca.app_id.as_str();
    let password = ca.password.as_str();
    let mut request_id: i32 = 0;
    let mut get_request_id = || {
        request_id += 1;
        request_id
    };
    let flow_path = format!(".cache/localctp_sys_trade_flow_{}_{}//", broker_id, account);
    check_make_dir(&flow_path);
    let mut api = create_api(&flow_path, false);
    let mut stream = {
        let (stream, pp) = create_spi();
        api.register_spi(pp);
        stream
    };
    if name_server.len() > 0 {
        api.register_name_server(CString::new(name_server).unwrap());
    } else {
        api.register_front(CString::new(trade_front).unwrap());
        info!("register front {}", trade_front);
    }
    api.subscribe_public_topic(localctp_sys::THOST_TE_RESUME_TYPE_THOST_TERT_QUICK);
    api.subscribe_private_topic(localctp_sys::THOST_TE_RESUME_TYPE_THOST_TERT_QUICK);
    api.init();
    let mut result = CtpQueryResult::default();
    result.broker_id = broker_id.to_string();
    result.account = account.to_string();
    // 处理登陆初始化查询
    while let Some(spi_msg) = stream.next().await {
        use localctp_sys::trader_api::CThostFtdcTraderSpiOutput::*;
        match spi_msg {
            OnFrontConnected(_p) => {
                let mut req = CThostFtdcReqAuthenticateField::default();
                set_cstr_from_str_truncate_i8(&mut req.BrokerID, broker_id);
                set_cstr_from_str_truncate_i8(&mut req.UserID, account);
                set_cstr_from_str_truncate_i8(&mut req.AuthCode, auth_code);
                set_cstr_from_str_truncate_i8(&mut req.UserProductInfo, user_product_info);
                set_cstr_from_str_truncate_i8(&mut req.AppID, app_id);
                api.req_authenticate(&mut req, get_request_id());
                info!("OnFrontConnected");
            }

            _ => {}
        }
    }
    result.timestamp = chrono::Local::now().timestamp();
    let config = config::standard();
    let encoded: Vec<u8> = bincode::encode_to_vec(&result, config).unwrap();
    let save_path = std::path::Path::new(".cache")
        .join("localctp_sys_query_result")
        .join(format!("{}_{}", broker_id, account));
    info!("save_path = {:?}", save_path);
    check_make_dir(save_path.to_str().unwrap());
    let save_path = save_path.join(format!("{}.dat", result.trading_day));
    info!("path={:?}", save_path);
    let mut f = std::fs::File::create(&save_path).unwrap();
    f.write_all(&encoded).unwrap();
    info!("{} 初始化查询完成. bin.len={}", account, encoded.len());
    let (decoded, _len): (CtpQueryResult, usize) =
        bincode::decode_from_slice(&encoded[..], config).unwrap();
    info!(
        "decoded {} {} {}",
        decoded.broker_id, decoded.account, decoded.trading_day
    );
    let ver = unsafe { CStr::from_ptr(get_api_version()) }
        .to_str()
        .unwrap();
    info!("version={ver}");
    api.release();
    Box::leak(api);
    info!("完成保存查询结果");
    info!("开始输入行情");
    let mut interval = time::interval(Duration::from_millis(500));
    loop {
        interval.tick().await;
        let mut rng = rand::thread_rng();

        let mut market_data = CThostFtdcInputQuoteField::default();
        market_data.BidPrice = rng.gen_range(1000.0..2000.0);
        market_data.AskPrice = market_data.BidPrice + rng.gen_range(5.0..20.0); // Ensures ask price is always higher than bid price
        market_data.InstrumentID = CString::new("MA403").unwrap().into_raw();
        market_data.QuoteRef = CString::new(format!("{:.1}", rng.gen_range(1000.0..2000.0))).unwrap().into_raw();
        market_data.UserID = CString::new(format!("{:.1}", rng.gen_range(4900.0..5100.0))).unwrap().into_raw();
        market_data.ForQuoteSysID = CString::new(format!("{:.1}", rng.gen_range(4900.0..5100.0))).unwrap().into_raw();
        market_data.BidOrderRef = CString::new(format!("{:.1}", rng.gen_range(9500.0..10000.0))).unwrap().into_raw();
        market_data.AskOrderRef = CString::new(format!("{:.1}", rng.gen_range(400.0..600.0))).unwrap().into_raw();
        market_data.BusinessUnit = CString::new(format!("{:.1}", rng.gen::<u32>())).unwrap().into_raw();
        let volume = rng.gen::<i32>();

        unsafe {
            api.req_quote_insert(&mut market_data as *mut _, volume);
        }

        // To prevent memory leaks, make sure to free the CString memory if necessary
        // This example assumes ownership issues are handled elsewhere or API is adjusted accordingly
    }
}
