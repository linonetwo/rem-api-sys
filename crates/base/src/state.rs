#![allow(non_upper_case_globals)]

use crate::error::Error;
use crate::util::*;
use crate::UniqueSymbol;
use itertools::Itertools;
use log::info;
use md_cache::MdCache;
use std::sync::atomic;
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod md_cache {
    use crate::state::MarketDataSnapshot;
    use crate::UniqueSymbol;
    use log::error;
    use std::collections::HashMap;
    use tokio::sync::mpsc;

    pub struct MdCache {
        pub tx: mpsc::Sender<UniqueSymbol>,
        pub hm_md: HashMap<UniqueSymbol, MarketDataSnapshot>,
        pub subscribed: Vec<UniqueSymbol>,
    }

    impl MdCache {
        pub async fn subscribe(&mut self, exchange: &str, symbol: &str) {
            let us = UniqueSymbol::new(exchange, symbol);
            if let None = self.subscribed.iter().find(|&e| *e == us) {
                self.subscribed.push(us.clone());
                if let Err(e) = self.tx.send(us).await {
                    error!(" send subscribe {e}");
                }
            }
        }

        pub fn get_md(&self, us: &UniqueSymbol) -> Option<&MarketDataSnapshot> {
            self.hm_md.get(us)
        }
    }
}

type RawApiDateType = [i8; 9];

#[derive(Debug)]
pub enum ReqMessage {
    SetContractTarget(crate::state::ContractPositionTarget),
    QueryPositionDetail,
    QueryTradingAccount,
}

pub type RspMessage = Result<Vec<u8>, crate::error::Error>;

#[derive(Debug, Clone)]
pub struct InstrumentField {
    pub price_tick: f64,
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

#[derive(Debug, PartialEq, Clone)]
pub enum OffsetFlag {
    Open,
    Close,
    CloseToday,
    CloseYesterday,
    OfOther,
}

impl DirectionType {
    pub fn opposite(&self) -> DirectionType {
        match *self {
            Self::Long => Self::Short,
            Self::Short => Self::Long,
        }
    }
}

pub enum PositionDateType {
    Today = 0,
    Yesterday = 1,
    Total = 2,
}

impl PositionDateType {
    pub fn get_close_offset_flag(&self) -> OffsetFlag {
        match *self {
            PositionDateType::Total => OffsetFlag::Close,
            PositionDateType::Today => OffsetFlag::CloseToday,
            PositionDateType::Yesterday => OffsetFlag::CloseYesterday,
        }
    }
}

#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ClosePriorityType {
    #[default]
    AnyFirst,
    YesterdayFirst,
    TodayFirst,
}

#[derive(Debug, Clone)]
pub struct MarketDataSnapshot {
    pub ask1: f64,
    pub ask1_volume1: i64,
    pub bid1: f64,
    pub bid1_volume: i64,
    pub timestamp: i64,
}

#[derive(PartialEq, Debug, Clone)]
pub enum PendingOrderStatus {
    Pending,
    Done,
    Canceled,
}

#[derive(Clone)]
pub struct PendingOrderTradeItem {
    pub volume: i32,
}

/// 追踪Order状态
/// 删除的条件是 status == canceld || status == done 并且 volume_traded 与trade_list合并结果相等
#[derive(Clone)]
pub struct PendingOrder {
    pub front_id: i32,
    pub session_id: i32,
    pub order_ref: [i8; 13],
    pub order_ref_i32: i32,
    pub order_sys_id: [i8; 21],
    pub volume_traded: i32,
    pub volume_total_original: i32,
    pub volume_canceled: i32,
    pub status: PendingOrderStatus,
    pub trades: Vec<PendingOrderTradeItem>,
}

impl std::fmt::Debug for PendingOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"{{status:{:?}, front_id:{}, session_id:{}, order_ref:[{}], order_sys_id:[{}], trades.len:{}, volume_traded:{}, trades_sum:{}, is_still_pending:{}, volume_total_original:{}, volume_canceled:{}}}"#,
            self.status,
            self.front_id,
            self.session_id,
            ascii_cstr_to_str_i8(&self.order_ref).unwrap(),
            ascii_cstr_to_str_i8(&self.order_sys_id).unwrap(),
            self.trades.len(),
            self.volume_traded,
            self.trades.iter().map(|x| x.volume).sum::<i32>(),
            self.is_still_pending(),
            self.volume_total_original,
            self.volume_canceled
        )
    }
}

// impl<TT: TradeType> std::cmp::PartialEq<PendingOrder<TT>> for dyn OrderType {
//     fn eq(&self, other: &PendingOrder<TT>) -> bool {
//         self.front_id() == other.front_id
//             && self.session_id() == other.session_id
//             && cstr_i8_eq(self.order_ref(), &other.order_ref)
//     }
// }

impl PendingOrder {
    /// 判断是否可以删除
    /// 确保所有的trade都已经收到并已经更新到position_detail中，以避免出现重复发单或错误发单
    fn is_still_pending(&self) -> bool {
        match self.status {
            PendingOrderStatus::Done | PendingOrderStatus::Canceled => {
                !(self.volume_traded == self.trades.iter().map(|x| x.volume).sum::<i32>()
                    && self.volume_total_original == self.volume_canceled + self.volume_traded)
            }
            _ => true,
        }
    }
}

/// 访问持仓明细
#[derive(Clone, Debug, Default)]
pub struct PositionDetail {
    pub open_date: [i8; 9],
    pub volume: i32,
    pub direction: DirectionType,
    pub exchange: String,
    pub symbol: String,
    pub open_price: f64,
    pub trade_id: [i8; 21],
}

impl PositionDetail {
    pub fn check_open_date(&self) {
        if self.open_date[0] == 0i8 {
            panic!("OpenDate == 0");
        }
    }

    pub fn from_trade(&mut self, tt: &dyn TradeType) {
        self.exchange = tt.exchange().into();
        self.symbol = tt.symbol().into();
        self.direction = tt.direction();
        self.open_price = tt.price();
        self.volume = tt.volume();
        self.trade_id = tt.trade_id().clone();
        self.open_date = *tt.trading_day();
    }
}

/// 访问委托
pub trait OrderType {
    fn volume_traded(&self) -> i32;
    fn volume_canceled(&self) -> i32;
    fn exchange(&self) -> &str;
    fn symbol(&self) -> &str;
    fn order_sys_id(&self) -> &[i8; 21];
    // fn front_id(&self) -> i32;
    // fn session_id(&self) -> i32;
    // fn order_ref(&self) -> &[i8; 13];
    fn to_pending_order(&self) -> PendingOrder;
    fn pending_status(&self) -> PendingOrderStatus;
}

/// 访问成交
pub trait TradeType {
    fn volume(&self) -> i32;
    fn direction(&self) -> DirectionType;
    fn direction_i8(&self) -> i8;
    fn trade_date(&self) -> &[i8; 9];
    fn trading_day(&self) -> &[i8; 9];
    fn exchange(&self) -> &str;
    fn symbol(&self) -> &str;
    fn order_sys_id(&self) -> &[i8; 21];
    fn trade_id(&self) -> &[i8; 21];
    fn offset_flag(&self) -> OffsetFlag;
    fn price(&self) -> f64;
    fn to_position_detail(&self) -> PositionDetail;
}

/// 合约明细数据
#[derive(Clone)]
pub struct ContractDetail<OT: OrderType, TT: TradeType> {
    pub us: UniqueSymbol,
    pub pl: Vec<PositionDetail>,                // 持仓明细
    pub pol: Vec<PendingOrder>,                 // 活跃委托
    pub hol: Vec<OT>,                           // 历史委托
    pub htl: Vec<TT>,                           // 历史成交明细
    pub target: Option<ContractPositionTarget>, // 目标仓位
    pub price_tick: f64,                        // 最小变动价位
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

/// 操作类型
pub enum Operation {
    NOP,                           // 无操作
    Input(InputOrderField),        // 需要下单
    Cancel(InputOrderActionField), // 需要撤单
}

impl Operation {
    pub fn or(self, opb: Operation) -> Operation {
        match self {
            Operation::NOP => opb,
            _ => self,
        }
    }
}

/// 交易接口
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

/// 合约目标仓位
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ContractPositionTarget {
    pub exchange: String,
    pub symbol: String,
    pub position: i32,                     // 目标数量
    pub shift: i32,                        // 撤单策略
    pub close_priority: ClosePriorityType, // 平仓优先顺序
}

impl ContractPositionTarget {
    pub fn direction(&self) -> DirectionType {
        if self.position > 0 {
            DirectionType::Long
        } else {
            DirectionType::Short
        }
    }
    pub fn opposite_direction(&self) -> DirectionType {
        if self.position > 0 {
            DirectionType::Short
        } else {
            DirectionType::Long
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ShareholderAccount {
    pub investor_id: String,
    pub exchange_id: String,
    pub shareholder_id: String,
    pub shareholder_id_type: i8,
    pub market_id: i8,
}

/// 内存中维护一个交易账户的最新镜像
/// 可用于更新委托，成交等
/// 有些品种需要区分平今平昨
/// update_by_order的时候，如果order.status = Done先调用，但trade还没回来，那有较小概率导致多平一次仓
pub struct AccountState<OT: OrderType + std::fmt::Debug, TT: TradeType + std::fmt::Debug> {
    pub broker_id: String,
    pub account: String,
    pub front_id: i32,
    pub session_id: i32,
    pub trading_day_i32: i32,
    pub trading_day_raw: RawApiDateType,
    pub sorted_cds: Vec<ContractDetail<OT, TT>>,
    pub hm_inst: std::collections::HashMap<UniqueSymbol, InstrumentField>,
    pub request_id: atomic::AtomicI32,
    pub order_ref: atomic::AtomicI32,
    pub shareholder_accounts: Vec<ShareholderAccount>,
}

impl<
        OT: OrderType + std::fmt::Debug + std::cmp::PartialEq + std::cmp::PartialEq<PendingOrder>,
        TT: TradeType + std::fmt::Debug + Clone,
    > AccountState<OT, TT>
{
    pub fn new(broker_id: &str, account: &str) -> Self {
        Self {
            broker_id: broker_id.to_owned(),
            account: account.to_owned(),
            sorted_cds: Vec::new(),
            front_id: 0,
            session_id: 0,
            order_ref: atomic::AtomicI32::new(1),
            trading_day_i32: 0,
            trading_day_raw: [0; 9],
            request_id: atomic::AtomicI32::new(0),
            hm_inst: std::collections::HashMap::new(),
            shareholder_accounts: vec![],
        }
    }

    /// 登陆数量更新
    pub fn on_login(
        &mut self,
        front_id: i32,
        session_id: i32,
        max_order_ref: &[i8; 13],
        trading_day: &RawApiDateType,
    ) {
        self.front_id = front_id;
        self.session_id = session_id;
        let mut max_order_ref = ascii_cstr_to_str_i8(max_order_ref)
            .expect("")
            .parse::<i32>()
            .unwrap();
        if max_order_ref < 1 {
            max_order_ref = 1;
        }
        self.order_ref
            .store(max_order_ref, atomic::Ordering::SeqCst);
        self.trading_day_raw = trading_day.clone();
        self.trading_day_i32 = ascii_cstr_to_str_i8(trading_day)
            .expect("")
            .parse::<i32>()
            .unwrap();
    }

    // get request_id
    pub fn get_request_id(&self) -> i32 {
        self.request_id.fetch_add(1, atomic::Ordering::SeqCst)
    }

    // get order_ref
    pub fn get_order_ref(&self) -> i32 {
        self.order_ref.fetch_add(1, atomic::Ordering::SeqCst)
    }

    /// like hashmap entry
    fn contract_detail_entry(&mut self, us: UniqueSymbol) -> Result<usize, Error> {
        let i = match self.sorted_cds.binary_search_by(|probe| probe.us.cmp(&us)) {
            Ok(i) => i,
            Err(i) => {
                let price_tick = self.get_price_tick(&us)?;
                self.sorted_cds
                    .insert(i, ContractDetail::new(us, price_tick));
                i
            }
        };
        Ok(i)
    }

    // 初始化持仓明细
    pub fn insert_position_detail(&mut self, pd: PositionDetail) -> Result<(), Error> {
        pd.check_open_date();
        let us = UniqueSymbol::new(&pd.exchange, &pd.symbol);
        let i = self.contract_detail_entry(us)?;
        self.sorted_cds[i].pl.push(pd);
        Ok(())
    }

    /// 初始化时更新委托
    /// 初始化查询得到的最新 Order snapshot，由于状态更新问题，vo必须要考虑顺序
    /// vt不用考虑顺序
    /// vo中pending的部分会放在state中
    /// AccountState中初始化得到的PositionDetailList + OnRtnTrade 合并得到最新的 PositionSnapshot, 但并不查询全部的Trade
    pub fn update_on_initialized(&mut self, vo: Vec<OT>, vt: Vec<TT>) -> Result<(), Error> {
        info!(
            "update_on_initialized vo.len={} vt.len={}",
            vo.len(),
            vt.len()
        );
        let mut v: Vec<OT> = vec![];
        for o in vo.into_iter() {
            match v.iter_mut().find(|x| **x == o) {
                Some(xo) => *xo = o,
                None => v.push(o),
            }
        }
        let mut vtx: Vec<TT> = vec![];
        for t in vt.into_iter() {
            if let None = vtx
                .iter()
                .find(|xt| cstr_i8_eq(t.order_sys_id(), xt.order_sys_id()))
            {
                vtx.push(t);
            }
        }
        for x in v
            .iter()
            .filter(|x| x.pending_status() == PendingOrderStatus::Pending)
        {
            let mut po = x.to_pending_order();
            po.trades = vtx
                .iter()
                .filter(|xx| cstr_i8_eq(xx.order_sys_id(), x.order_sys_id()))
                .map(|x| PendingOrderTradeItem { volume: x.volume() })
                .collect_vec();
            let us = UniqueSymbol {
                exchange: x.exchange().into(),
                symbol: x.symbol().into(),
            };
            let i = self.contract_detail_entry(us)?;
            self.sorted_cds[i].pol.push(po);
        }
        Ok(())
    }

    /// 委托更新
    /// AccountState中只保留PendingOrder, 完成成交和已撤单的委托都会被清除
    /// 追踪PendingOrder可以保证对齐持仓的过程中顺序操作，以免出现重复开平仓情形
    /// 返回true表示移除了一个pending order，往往需要重新check_target
    pub fn update_by_order(&mut self, o: OT) -> Result<bool, Error> {
        let us = UniqueSymbol::new(o.exchange(), o.symbol());
        let i = self.contract_detail_entry(us)?;
        let cd = &mut self.sorted_cds[i];
        let index = match cd.pol.iter().position(|x| o == *x) {
            Some(index) => {
                let old = &mut cd.pol[index];
                if old.order_sys_id[0] == 0 {
                    old.order_sys_id = o.order_sys_id().clone();
                }
                old.volume_traded = o.volume_traded();
                old.volume_canceled = o.volume_canceled();
                old.status = o.pending_status();
                index
            }
            None => {
                // 可能是其他终端下单
                let mut po = o.to_pending_order();
                po.trades = cd
                    .htl
                    .iter()
                    .filter(|xx| cstr_i8_eq(xx.order_sys_id(), o.order_sys_id()))
                    .map(|x| PendingOrderTradeItem { volume: x.volume() })
                    .collect_vec();
                cd.pol.push(po);
                cd.pol.len() - 1
            }
        };
        info!("update_by_order po={:?}", cd.pol[index]);
        if !cd.pol[index].is_still_pending() {
            info!("update_by_order remove none pending");
            cd.pol.swap_remove(index);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// remove pending order when some error on order insert reponse
    /// 例如当 ReqOrderInsert错误，并通过OnRspOrderInsert返回时，直接删除委托
    pub fn remove_po(
        &mut self,
        us: &UniqueSymbol,
        front_id: i32,
        session_id: i32,
        order_ref: &[i8; 13],
    ) {
        if let Ok(i) = self.sorted_cds.binary_search_by(|probe| probe.us.cmp(us)) {
            let cd = &mut self.sorted_cds[i];
            if let Some(index) = cd.pol.iter().position(|x| {
                x.front_id == front_id
                    && x.session_id == session_id
                    && cstr_i8_eq(&x.order_ref, order_ref)
            }) {
                cd.pol.swap_remove(index);
            }
        }
    }

    /// 更新成交列表
    /// 更新本地持仓报表, 并删除已完全成交的PendingOrder
    /// 返回true表示需要马上重新对齐仓位, false表示无需操作
    pub fn update_by_trade(&mut self, t: TT) -> Result<bool, Error> {
        let offset = t.offset_flag();
        let us = UniqueSymbol::new(t.exchange(), t.symbol());
        let i = self.contract_detail_entry(us)?;
        let cd = &mut self.sorted_cds[i];
        if offset == OffsetFlag::Open {
            cd.pl.push(t.to_position_detail());
        } else {
            let mut left_volume = t.volume();
            for pd in cd.pl.iter_mut().filter(|pd| {
                if pd.volume > 0 && t.direction() != pd.direction {
                    // 平今 SHFE INE区分
                    if cd.us.exchange == "SHFE" || cd.us.exchange == "INE" {
                        match offset {
                            OffsetFlag::Open => false,
                            OffsetFlag::CloseToday => {
                                t.trade_date().cmp(&pd.open_date) == std::cmp::Ordering::Equal
                            }
                            OffsetFlag::CloseYesterday | OffsetFlag::Close => {
                                pd.open_date.cmp(t.trade_date()) == std::cmp::Ordering::Less
                            }
                            _ => true,
                        }
                    } else {
                        true
                    }
                } else {
                    false
                }
            }) {
                let v = std::cmp::min(left_volume, pd.volume);
                pd.volume -= v;
                left_volume -= v;
                if left_volume < 0 {
                    panic!("left_volume < 0");
                }
                if left_volume == 0 {
                    break;
                }
            }
            if left_volume > 0 {
                info!("pl={:?}", cd.pl);
                info!(
                    "trade={:?} TradeDate={} TradingDay={}",
                    t,
                    get_ascii_str(t.trade_date()).unwrap(),
                    get_ascii_str(t.trading_day()).unwrap()
                );
                panic!("left_volume>0 无仓可平");
            }
        }
        let index = cd
            .pol
            .iter_mut()
            .position(|x| cstr_i8_eq(&x.order_sys_id, t.order_sys_id()))
            .expect("update_by_trade trade must get pending order");
        let po = &mut cd.pol[index];
        po.trades.push(PendingOrderTradeItem { volume: t.volume() });
        if !po.is_still_pending() {
            cd.pol.swap_remove(index);
            info!("update_by_trade remove none pending");
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// 查询合约最小变动价位
    pub fn get_price_tick(&self, us: &UniqueSymbol) -> Result<f64, Error> {
        Ok(self
            .hm_inst
            .get(us)
            .ok_or(Error::InstrumentNotFound)?
            .price_tick)
    }

    /// 设置某合约的target 并返回对应的操作
    /// 如果target = None, 则查看已经设置好的target并进行持仓对齐
    pub async fn set_check_target(
        &mut self,
        us: UniqueSymbol,
        target: Option<ContractPositionTarget>,
        cmc: &Arc<Mutex<MdCache>>,
        api: &mut Box<dyn TraderApiType + Send>,
    ) -> Result<(), Error> {
        let mut cmc = cmc.lock().await;
        let md = cmc.get_md(&us);
        match md {
            Some(md) => {
                let i = self.contract_detail_entry(us)?;
                let op = {
                    if target.is_some() {
                        self.sorted_cds[i].target = target;
                    }
                    self.sorted_cds[i].check_target(md, &self.trading_day_raw)
                };
                let cd = &self.sorted_cds[i];
                match op {
                    Operation::NOP => {
                        info!("NOP");
                        Ok(())
                    }
                    Operation::Input(iof) => {
                        info!(
                            "Input {}:{} Direction={:?} Volume={} Price={} md={:?}",
                            &cd.us.exchange,
                            &cd.us.symbol,
                            iof.direction,
                            iof.volume,
                            iof.price,
                            md
                        );
                        let order_ref = self.get_order_ref();
                        match api.req_order_insert(
                            &self.broker_id,
                            &self.account,
                            &cd.us.exchange,
                            &cd.us.symbol,
                            &iof,
                            order_ref,
                            self.get_request_id(),
                            &self.shareholder_accounts,
                        ) {
                            Ok(()) => {
                                let mut x_order_ref = [0i8; 13];
                                set_cstr_from_str_truncate_i8(
                                    &mut x_order_ref,
                                    format!("{order_ref}").as_str(),
                                );
                                let po = PendingOrder {
                                    front_id: self.front_id,
                                    session_id: self.session_id,
                                    order_ref: x_order_ref,
                                    order_ref_i32: order_ref,
                                    order_sys_id: [0; 21],
                                    volume_total_original: iof.volume,
                                    volume_canceled: 0,
                                    volume_traded: 0,
                                    trades: vec![],
                                    status: PendingOrderStatus::Pending,
                                };
                                self.sorted_cds[i].pol.push(po);
                                Ok(())
                            }
                            Err(e) => Err(e),
                        }
                    }
                    Operation::Cancel(ioaf) => {
                        info!(
                            "Cancel front_id={} session_id={} order_ref={}",
                            ioaf.front_id,
                            ioaf.session_id,
                            ascii_cstr_to_str_i8(&ioaf.order_ref).unwrap(),
                        );
                        api.req_order_action(
                            &self.broker_id,
                            &self.account,
                            &cd.us.exchange,
                            &cd.us.symbol,
                            &ioaf,
                            self.get_order_ref(),
                            self.get_request_id(),
                        )
                    }
                }
            }
            None => {
                cmc.subscribe(&us.exchange, &us.symbol).await;
                Err(Error::MdNotFound)
            }
        }
    }
}

impl<OT: OrderType, TT: TradeType> ContractDetail<OT, TT> {
    pub fn new(us: UniqueSymbol, price_tick: f64) -> Self {
        let cd = ContractDetail {
            us,
            price_tick,
            pl: vec![],
            pol: vec![],
            hol: vec![],
            htl: vec![],
            target: None,
        };
        cd
    }

    /// 统计持仓
    pub fn summation(
        &self,
        d: &DirectionType,
        pos_date_type: &PositionDateType,
        trading_day: &RawApiDateType,
    ) -> i32 {
        self.pl
            .iter()
            .filter(|detail| {
                if trading_day < &detail.open_date {
                    info!(
                        "trading_day={} detail.open_date={} exchange={} symbol={}",
                        ascii_cstr_to_str_i8(trading_day).unwrap(),
                        ascii_cstr_to_str_i8(&detail.open_date).unwrap(),
                        &self.us.exchange,
                        &self.us.symbol
                    );
                    panic!("trading_day < detail.open_date");
                }
                detail.check_open_date();
                if detail.direction == *d {
                    match pos_date_type {
                        PositionDateType::Today => {
                            detail.open_date.cmp(trading_day) == std::cmp::Ordering::Equal
                        }
                        PositionDateType::Yesterday => {
                            detail.open_date.cmp(trading_day) == std::cmp::Ordering::Less
                        }
                        PositionDateType::Total => true,
                    }
                } else {
                    false
                }
            })
            .map(|x| x.volume)
            .sum()
    }

    /// 平相反仓位
    fn close_opposite(
        &self,
        opposite_direction: &DirectionType,
        pos_date_type: &PositionDateType,
        trading_day: &RawApiDateType,
    ) -> Option<(i32, DirectionType, OffsetFlag)> {
        let sum = self.summation(opposite_direction, pos_date_type, trading_day);
        if sum > 0 {
            Some((
                sum,
                opposite_direction.opposite(),
                pos_date_type.get_close_offset_flag(),
            ))
        } else {
            None
        }
    }

    /// 平同向仓位
    fn close_same_direction(
        &self,
        direction: &DirectionType,
        pos_date_type: &PositionDateType,
        diff: i32,
        trading_day: &RawApiDateType,
    ) -> Option<(i32, DirectionType, OffsetFlag)> {
        let sum = self.summation(direction, pos_date_type, trading_day);
        if sum == 0 {
            return None;
        }
        if sum <= diff {
            Some((
                sum,
                direction.opposite(),
                pos_date_type.get_close_offset_flag(),
            ))
        } else {
            Some((
                diff,
                direction.opposite(),
                pos_date_type.get_close_offset_flag(),
            ))
        }
    }

    /// 平反向昨仓
    fn close_position(
        &self,
        target: &ContractPositionTarget,
        md: &MarketDataSnapshot,
        price_tick: f64,
        trading_day: &RawApiDateType,
    ) -> Operation {
        let d = target.direction();
        let opd = target.opposite_direction();

        // 1. 先平反向仓
        let op = match target.close_priority {
            ClosePriorityType::AnyFirst => {
                self.close_opposite(&opd, &PositionDateType::Total, trading_day)
            }
            ClosePriorityType::YesterdayFirst => self
                .close_opposite(&opd, &PositionDateType::Yesterday, trading_day)
                .or(self.close_opposite(&opd, &PositionDateType::Today, trading_day)),
            ClosePriorityType::TodayFirst => self
                .close_opposite(&opd, &PositionDateType::Today, trading_day)
                .or(self.close_opposite(&opd, &PositionDateType::Yesterday, trading_day)),
        };
        // 2. 再平同向仓
        let op = op.or({
            let target_position = target.position.abs();
            let current_total = self.summation(&d, &PositionDateType::Total, trading_day);
            let diff = current_total - target_position;
            if diff > 0 {
                info!(
                    "diff={diff} target_position={target_position} current_total={current_total} d={:?}", d
                );
                match target.close_priority {
                    ClosePriorityType::AnyFirst => {
                        self.close_same_direction(&d, &PositionDateType::Total, diff, trading_day)
                    }
                    ClosePriorityType::YesterdayFirst => self
                        .close_same_direction(&d, &PositionDateType::Yesterday, diff, trading_day)
                        .or(self.close_same_direction(
                            &d,
                            &PositionDateType::Today,
                            diff,
                            trading_day,
                        )),
                    ClosePriorityType::TodayFirst => self
                        .close_same_direction(&d, &PositionDateType::Today, diff, trading_day)
                        .or(self.close_same_direction(
                            &d,
                            &PositionDateType::Yesterday,
                            diff,
                            trading_day,
                        )),
                }
            } else {
                None
            }
        });
        match op {
            Some((volume, direction, offset)) => {
                info!(
                    "{}:{} close potition volume={volume} direction={:?} offset={:?}",
                    &self.us.exchange, &self.us.symbol, direction, offset
                );
                let price = get_counterparty_price(md, &direction, price_tick, target.shift);
                let input = InputOrderField {
                    direction,
                    offset,
                    price,
                    volume,
                };
                Operation::Input(input)
            }
            None => Operation::NOP,
        }
    }

    /// 开同向仓
    fn open_position(
        &self,
        target: &ContractPositionTarget,
        md: &MarketDataSnapshot,
        price_tick: f64,
        trading_day: &RawApiDateType,
    ) -> Operation {
        let d = target.direction();
        let current = self.summation(&d, &PositionDateType::Total, trading_day);
        info!(
            "{}:{} open_position current={} target.position={}",
            self.us.exchange,
            self.us.symbol,
            current,
            target.position.abs()
        );
        if current < target.position.abs() {
            info!(
                "{}:{} current_position={} od={:?} &md.TradingDay={} target={}",
                self.us.exchange,
                self.us.symbol,
                current,
                target.direction(),
                ascii_cstr_to_str_i8(trading_day).unwrap(),
                target.position
            );
            let price = get_counterparty_price(md, &d, price_tick, target.shift);
            let input = InputOrderField {
                direction: d,
                offset: OffsetFlag::Open,
                price,
                volume: target.position.abs() - current,
            };
            return Operation::Input(input);
        }
        Operation::NOP
    }

    /// 检查持仓是否与目标一致，如果不一致，则返回相应的操作.
    fn check_target(&self, md: &MarketDataSnapshot, trading_day: &RawApiDateType) -> Operation {
        match &self.target {
            Some(target) => {
                // pol 是活跃订单，只要有活跃订单就先撤再重发. pol 会由Spi返回的相关事件进行更新
                // 如果要求撤单同时就发新单，则需要另外写处理逻辑
                for o in self.pol.iter() {
                    info!("po={:?}", o);
                    let ioaf = InputOrderActionField {
                        front_id: o.front_id,
                        session_id: o.session_id,
                        order_ref: o.order_ref.clone(),
                    };
                    return Operation::Cancel(ioaf);
                }
                // 先平仓，再开仓
                self.close_position(target, md, self.price_tick, trading_day)
                    .or(self.open_position(target, md, self.price_tick, trading_day))
                // 考虑到股票的情况，既不分平今平昨，同时还有最小交易量的限制，导致平仓的时候可能出现如昨仓还剩下2股，今仓8股，但最小交易量为10股/手，
                // 这时平不出去，如果先平今的话，8股也不够一手
            }
            None => Operation::NOP,
        }
    }
}

/// 对手价
/// 1 注意 bid , ask 可能因极端行情而特别偏离当前价.
fn get_counterparty_price(
    md: &MarketDataSnapshot,
    d: &DirectionType,
    price_tick: f64,
    shift: i32,
) -> f64 {
    match d {
        DirectionType::Long => md.bid1 + price_tick * shift as f64,
        DirectionType::Short => md.ask1 - price_tick * shift as f64,
    }
}
