use base::state::{
    DirectionType, InputOrderActionField, InputOrderField, OffsetFlag, TraderApiType,
};
use bincode::{config, Decode, Encode};
use futures::StreamExt;
use log::info;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use route;
use std::ffi::{CStr, CString};
use std::{io::Write, sync::Arc};
use tokio::{
    sync::Mutex,
    time::{self, Duration},
};

use localctp_sys::*;
use rust_share_util::*;

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

async fn simulate_market_data(api: &mut CThostFtdcTraderApi) {
    let mut interval = time::interval(Duration::from_millis(500));
    let mut rng = StdRng::from_entropy(); // Using a thread-safe RNG that can be sent across threads

    loop {
        interval.tick().await;
        let market_quote = FakeMarketQuote {
            instrument_id: "MA403".to_string(),
            bid_price: rng.gen_range(1000.0..2000.0),
            ask_price: rng.gen_range(1000.0..2000.0),
            quote_ref: format!("{:.1}", rng.gen::<f64>() * 1000.0),
            last_price: format!("{:.2}", rng.gen::<f64>() * 5000.0),
            settlement_price: format!("{:.2}", rng.gen::<f64>() * 5000.0),
            upper_limit_price: format!("{:.2}", rng.gen::<f64>() * 6000.0),
            lower_limit_price: format!("{:.2}", rng.gen::<f64>() * 4000.0),
            business_unit: format!("{:08}", rng.gen::<u32>()),
            volume: rng.gen_range(1..100),
        };

        if let Err(e) = api.insert_market_quote(market_quote) {
            eprintln!("Error inserting market quote: {:?}", e);
        }
    }
}

async fn revoke_limit_order(api: &mut dyn TraderApiType, account_config: &CtpAccountConfig) {
    // Define the necessary parameters for a limit order
    let broker_id = &account_config.broker_id;
    let account = &account_config.account;
    let exchange = "SHFE"; // Shanghai Futures Exchange, adjust as needed
    let symbol = "cu2101"; // Example futures contract for copper, adjust as needed
    let price = 50000.0; // Example price, adjust as needed
    let volume = 1; // Example volume, adjust as needed
    let order_ref = 123; // This should be a unique reference for the order, possibly incrementing
    let n_request_id = 1; // Request ID, unique per request

    // Initialize the order input with default values
    let order_input = InputOrderField {
        direction: DirectionType::Long, // Buying, adjust as needed (Long or Short)
        offset: OffsetFlag::Open,       // Order is to open a position, adjust as needed
        price,
        volume,
    };

    // Call the API to insert the order
    println!("insert_limit_order request {:#?}", order_input);
    let mut x_order_ref = [0i8; 13];
    set_cstr_from_str_truncate_i8(&mut x_order_ref, format!("{order_ref}").as_str());
    let input_order_action = InputOrderActionField {
        front_id: 1,   // Example ID, should be set according to your system's management of IDs
        session_id: 1, // Example Session ID
        // order_ref: set_cstr_from_str_truncate_i8("your_order_ref"), // Your order reference
        order_ref: x_order_ref,
    };
    let result = api.req_order_action(
        "broker_id",
        "account_id",
        "exchange",
        "symbol",
        &input_order_action,
        123, // order_action_ref, if applicable
        1,   // n_request_id
    );

    match result {
        Ok(_) => println!("Order action successfully requested."),
        Err(e) => eprintln!("Failed to request order action: {:?}", e),
    }
}

async fn insert_limit_order(api: &mut dyn TraderApiType, account_config: &CtpAccountConfig) {
    // Define the necessary parameters for a limit order
    let broker_id = &account_config.broker_id;
    let account = &account_config.account;
    let exchange = "SHFE"; // Shanghai Futures Exchange, adjust as needed
    let symbol = "cu2101"; // Example futures contract for copper, adjust as needed
    let price = 50000.0; // Example price, adjust as needed
    let volume = 1; // Example volume, adjust as needed
    let order_ref = 123; // This should be a unique reference for the order, possibly incrementing
    let n_request_id = 1; // Request ID, unique per request

    // Initialize the order input with default values
    let order_input = InputOrderField {
        direction: DirectionType::Long, // Buying, adjust as needed (Long or Short)
        offset: OffsetFlag::Open,       // Order is to open a position, adjust as needed
        price,
        volume,
    };

    // Call the req_order_insert method on the API object
    let result = api.req_order_insert(
        broker_id,
        account,
        exchange,
        symbol,
        &order_input,
        order_ref,
        n_request_id,
        &vec![], // Assuming _shareholder_accounts is not used in your example
    );

    // Check the result of the order insertion
    match result {
        Ok(_) => println!("Order successfully inserted."),
        Err(e) => println!("Error inserting order: {:?}", e),
    }
}

#[tokio::main]
async fn main() {
    // FIXME: 启用 log 导致报错
    // init_logger();
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

async fn query(ctp_account: &CtpAccountConfig) {
    use localctp_sys::trader_api::*;
    let broker_id = ctp_account.broker_id.as_str();
    let account = ctp_account.account.as_str();
    let trade_front = ctp_account.trade_fronts[0].as_str();
    let name_server = ctp_account.name_servers[0].as_str();
    let auth_code = ctp_account.auth_code.as_str();
    let user_product_info = ctp_account.user_product_info.as_str();
    let app_id = ctp_account.app_id.as_str();
    // let password = ca.password.as_str();
    let mut request_id: i32 = 0;
    let mut get_request_id = || {
        request_id += 1;
        request_id
    };
    let flow_path = format!(".cache/localctp_sys_trade_flow_{}_{}//", broker_id, account);
    check_make_dir(&flow_path);
    let mut api_box = create_api(&flow_path, false);
    let mut stream = {
        let (stream, pp) = create_spi();
        api_box.register_spi(pp);
        stream
    };
    if name_server.len() > 0 {
        api_box.register_name_server(CString::new(name_server).unwrap());
    } else {
        api_box.register_front(CString::new(trade_front).unwrap());
        info!("register front {}", trade_front);
    }
    api_box.subscribe_public_topic(localctp_sys::THOST_TE_RESUME_TYPE_THOST_TERT_QUICK);
    api_box.subscribe_private_topic(localctp_sys::THOST_TE_RESUME_TYPE_THOST_TERT_QUICK);
    api_box.init();
    let mut result = CtpQueryResult::default();
    result.broker_id = broker_id.to_string();
    result.account = account.to_string();
    // 处理登陆初始化查询
    // 登录后才能发单
    // let mut login_req = CThostFtdcReqUserLoginField::default();
    // set_cstr_from_str_truncate_i8(&mut login_req.BrokerID, &ctp_account.broker_id);
    // set_cstr_from_str_truncate_i8(&mut login_req.UserID, &ctp_account.account);
    // set_cstr_from_str_truncate_i8(&mut login_req.Password, &ctp_account.password);
    // api_box.req_user_login(&mut login_req, 1);
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
                api_box.req_authenticate(&mut req, get_request_id());
                info!("OnFrontConnected");
            }
            OnRspAuthenticate(p) => {
                // 认证后才能登录
                let mut req = CThostFtdcReqUserLoginField::default();
                set_cstr_from_str_truncate_i8(&mut req.BrokerID, broker_id);
                set_cstr_from_str_truncate_i8(&mut req.UserID, account);
                set_cstr_from_str_truncate_i8(&mut req.Password, &ctp_account.password);
                // 登录后才能下单
                api_box.req_user_login(&mut req, get_request_id());
                break;
            }
            OnRtnOrder(p) => {
                info!("报单成功回报 Order Return: {:?}", p);
            }
            OnRspOrderInsert(p) => {
                info!("报单失败回报 OnRspOrderInsert: {:?}", p);
            }
            OnRtnTrade(p) => {
                info!("成交回报 OnRtnTrade: {:?}", p);
            }
            _ => {}
        }
    }
    println!("完成认证");
    result.timestamp = chrono::Local::now().timestamp();
    info!("开始输入行情");
    // Wrap the API in Arc and Mutex for shared ownership and thread safety
    let shared_api = Arc::new(Mutex::new(create_api(&flow_path, false)));
    // Clone the API handle for the spawned task
    let api_clone = shared_api.clone();
    tokio::spawn(async move {
        let mut api = api_clone.lock().await;
        simulate_market_data(&mut *api).await;
    });
    let raw_api = api_box.as_mut();
    time::sleep(Duration::from_millis(200)).await;
    insert_limit_order(raw_api, ctp_account).await;
    // Wait for 2 seconds after inserting the limit order
    time::sleep(Duration::from_secs(2)).await;

    // let ver = unsafe { CStr::from_ptr(get_api_version()) }
    //     .to_str()
    //     .unwrap();
    // info!("version={ver}");
    // api_box.release();
}
