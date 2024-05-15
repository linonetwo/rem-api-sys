use crate::{bindings::*, spi_wrapper::{YDListenerTrait, YD_LISTENER_VTABLE}};

/* Param: yd_exchange_id *//* Param: yd_product_id *//* Param: yd_instrument_id *//* Param: yd_long_instrument_id *//* Param: yd_trading_code *//* Param: yd_account_id *//* Param: yd_password *//* Param: yd_string *//* Param: yd_sys_order_id *//* Param: yd_trade_id *//* Param: yd_local_order_id *//* Param: ydrfqid *//* Param: yd_long_order_sys_id *//* Param: yd_long_trade_id *//* Param: yd_long_rfqid *//* Param: yd_pc_futures *//* Param: yd_pc_options *//* Param: yd_pc_combination *//* Param: yd_pc_index *//* Param: yd_pc_cash *//* Param: yd_spc_other *//* Param: yd_spc_stock *//* Param: yd_spc_bond *//* Param: yd_spc_fund *//* Param: yd_ot_not_option *//* Param: yd_ot_call_option *//* Param: yd_ot_put_option *//* Param: yd_d_buy *//* Param: yd_d_sell *//* Param: yd_d_make *//* Param: yd_d_split *//* Param: yd_d_freeze *//* Param: yd_d_unfreeze *//* Param: yd_d_normal_2_covered *//* Param: yd_d_covered_2_normal *//* Param: yd_pd_long *//* Param: yd_pd_short *//* Param: yd_psd_today *//* Param: yd_psd_history *//* Param: yd_hf_speculation *//* Param: yd_hf_arbitrage *//* Param: yd_hf_hedge *//* Param: yd_hf_internal *//* Param: yd_hf_normal *//* Param: yd_hf_covered *//* Param: yd_max_hedge_flag *//* Param: yd_tr_allow *//* Param: yd_tr_close_only *//* Param: yd_tr_forbidden *//* Param: yd_of_open *//* Param: yd_of_close *//* Param: yd_of_force_close *//* Param: yd_of_close_today *//* Param: yd_of_close_yesterday *//* Param: yd_of_open_1_close_2 *//* Param: yd_of_close_1_open_2 *//* Param: yd_odt_limit *//* Param: yd_odt_fak *//* Param: yd_odt_market *//* Param: yd_odt_fok *//* Param: yd_odt_close_self_option_position *//* Param: yd_odt_reserve_option_position *//* Param: yd_odt_sell_close_self_futures_position *//* Param: yd_odt_position_offset_mark *//* Param: yd_odt_option_abandon_execute_mark *//* Param: yd_odt_close_futures_position_mark *//* Param: yd_ott_no_trigger *//* Param: yd_ott_take_profit *//* Param: yd_ott_stop_loss *//* Param: yd_gorf_increase *//* Param: yd_gorf_increase_one *//* Param: yd_os_accepted *//* Param: yd_os_queuing *//* Param: yd_os_canceled *//* Param: yd_os_all_traded *//* Param: yd_os_rejected *//* Param: yd_ots_not_triggered *//* Param: yd_ots_triggered *//* Param: yd_yof_normal *//* Param: yd_yof_quote_derived *//* Param: yd_yof_option_execute *//* Param: yd_yof_option_abandon_execute *//* Param: yd_yof_request_for_quote *//* Param: yd_yof_comb_position *//* Param: yd_yof_option_execute_together *//* Param: yd_yof_mark *//* Param: yd_yof_option_self_close *//* Param: yd_yof_freeze_underlying *//* Param: yd_yof_cover *//* Param: yd_yqf_response_of_rfq *//* Param: yd_yqf_replace_last_quote *//* Param: yd_chf_spec_spec *//* Param: yd_chf_spec_hedge *//* Param: yd_chf_hedge_hedge *//* Param: yd_chf_hedge_spec *//* Param: yd_max_comb_hedge_flag *//* Param: yd_cpt_dce_futures_offset *//* Param: yd_cpt_dce_options_offset *//* Param: yd_cpt_dce_futures_calendar_spread *//* Param: yd_cpt_dce_futures_product_spread *//* Param: yd_cpt_dce_buy_options_vertical_spread *//* Param: yd_cpt_dce_sell_options_vertical_spread *//* Param: yd_cpt_dce_options_straddle *//* Param: yd_cpt_dce_options_strangle *//* Param: yd_cpt_dce_buy_options_covered *//* Param: yd_cpt_dce_sell_options_covered *//* Param: yd_cpt_gfex_futures_offset *//* Param: yd_cpt_gfex_options_offset *//* Param: yd_cpt_gfex_futures_calendar_spread *//* Param: yd_cpt_gfex_futures_product_spread *//* Param: yd_cpt_gfex_buy_options_vertical_spread *//* Param: yd_cpt_gfex_sell_options_vertical_spread *//* Param: yd_cpt_gfex_options_straddle *//* Param: yd_cpt_gfex_options_strangle *//* Param: yd_cpt_gfex_buy_options_covered *//* Param: yd_cpt_gfex_sell_options_covered *//* Param: yd_cpt_czce_spread *//* Param: yd_cpt_czce_straddle_strangle *//* Param: yd_cpt_czce_sell_option_convered *//* Param: yd_cpt_stock_option_cnsjc *//* Param: yd_cpt_stock_option_cxsjc *//* Param: yd_cpt_stock_option_pnsjc *//* Param: yd_cpt_stock_option_pxsjc *//* Param: yd_cpt_stock_option_ks *//* Param: yd_cpt_stock_option_kks *//* Param: yd_cs_any *//* Param: yd_cs_fixed *//* Param: yd_cs_prefered *//* Param: yd_am_modify_usage *//* Param: yd_am_deposit *//* Param: yd_am_frozen_withdraw *//* Param: yd_am_cancel_frozen_withdraw *//* Param: yd_am_withdraw *//* Param: yd_am_deposit_to *//* Param: yd_am_withdraw_to *//* Param: yd_am_force_modify_usage *//* Param: yd_rt_change_password *//* Param: yd_rt_set_trading_right *//* Param: yd_rt_alter_money *//* Param: yd_rt_select_connection *//* Param: yd_rt_admin_trading *//* Param: yd_rt_update_margin_rate *//* Param: yd_rt_update_spot_position *//* Param: yd_rt_update_spot_alive *//* Param: yd_rt_adjust_account_margin_model_info *//* Param: yd_rt_update_message_commission_config *//* Param: yd_rt_update_holding_external_frozen *//* Param: yd_ae_tcp_trade_connected *//* Param: yd_ae_tcp_trade_disconnected *//* Param: yd_ae_tcp_market_data_connected *//* Param: yd_ae_tcp_market_data_disconnected *//* Param: yd_ae_server_restarted *//* Param: yd_ae_server_switched *//* Param: yd_ae_xtcp_trade_connected *//* Param: yd_ae_xtcp_trade_disconnected *//* Param: yd_af_select_connection *//* Param: yd_af_auto_make_comb_position *//* Param: yd_af_raw_protocol *//* Param: yd_af_disable_self_trade_check *//* Param: yd_af_notify_order_accept *//* Param: yd_af_no_close_frozen_on_insert_order *//* Param: yd_af_order_ref_check *//* Param: yd_cbt_pre_settlement_price *//* Param: yd_cbt_open_price *//* Param: yd_cbt_last_price *//* Param: yd_cbt_market_average_price *//* Param: yd_cbt_max_last_pre_settlement_price *//* Param: yd_cbt_order_price *//* Param: yd_cbt_none *//* Param: yd_cbt_same_price *//* Param: yd_idt_normal_order_sys_id *//* Param: yd_idt_quote_derived_order_sys_id *//* Param: yd_idt_option_execute_order_sys_id *//* Param: yd_idt_option_abandon_execute_order_sys_id *//* Param: yd_idt_request_for_quote_order_sys_id *//* Param: yd_idt_comb_position_order_sys_id *//* Param: yd_idt_option_execute_together *//* Param: yd_idt_mark *//* Param: yd_idt_option_self_close *//* Param: yd_idt_freeze_underlying *//* Param: yd_idt_cover *//* Param: yd_idt_trade_id *//* Param: yd_idt_comb_position_detail_id *//* Param: yd_idt_quote_sys_id *//* Param: yd_grpt_option_long_position_cost *//* Param: yd_grpt_trade_position_ratio *//* Param: yd_grpt_order_cancel_ratio *//* Param: yd_grpt_dynamic_price_limit_upper_ratio *//* Param: yd_grpt_dynamic_price_limit_lower_ratio *//* Param: yd_grpt_dynamic_price_limit_upper_tick_count *//* Param: yd_grpt_dynamic_price_limit_lower_tick_count *//* Param: yd_grpt_dynamic_last_price_limit_upper_ratio *//* Param: yd_grpt_dynamic_last_price_limit_lower_ratio *//* Param: yd_grpt_dynamic_last_price_limit_upper_tick_count *//* Param: yd_grpt_dynamic_last_price_limit_lower_tick_count *//* Param: yd_grpt_exchange_max_order_volume *//* Param: yd_grpt_product_max_order_volume *//* Param: yd_grpt_instrument_max_order_volume *//* Param: yd_grpt_exchange_option_long_position_cost *//* Param: yd_grpt_exchange_st_buy_volume *//* Param: yd_grpt_product_st_buy_volume *//* Param: yd_grpt_exchange_buy_volume *//* Param: yd_grpt_product_buy_volume *//* Param: yd_grpt_instrument_buy_volume *//* Param: yd_grpt_exchange_cash_trading_right *//* Param: yd_grpt_product_cash_trading_right *//* Param: yd_grpt_instrument_cash_trading_right *//* Param: yd_grpt_exchange_holding_limit *//* Param: yd_grpt_product_holding_limit *//* Param: yd_grpt_instrument_holding_limit *//* Param: yd_grpt_trade_st *//* Param: yd_grpt_trade_star_st *//* Param: yd_grpt_qualified_bond_investor *//* Param: yd_grpt_qualified_bond_corp_investor *//* Param: yd_trs_admin_permanent *//* Param: yd_trs_user_permanent *//* Param: yd_trs_admin_temp *//* Param: yd_trs_user_temp *//* Param: yd_trs_count *//* Param: yd_ts_no_trading *//* Param: yd_ts_continuous *//* Param: yd_ts_auction *//* Param: yd_mm_normal *//* Param: yd_mm_spbm *//* Param: yd_mm_rule *//* Param: yd_mm_new_model_start *//* Param: yd_mm_count *//* Param: yd_cv_verify *//* Param: yd_cv_not_verify *//* Param: yd_ef_shfe *//* Param: yd_ef_dce *//* Param: yd_ef_czce *//* Param: yd_ef_cffex *//* Param: yd_ef_ine *//* Param: yd_ef_sse_option *//* Param: yd_ef_szse_option *//* Param: yd_ef_gfex *//* Param: yd_ef_sse_cash *//* Param: yd_ef_szse_cash *//* Param: yd_ecs_disconnected *//* Param: yd_ecs_connected *//* Param: yd_cht_history *//* Param: yd_cht_today_trading *//* Param: yd_cht_today_creation_redemption *//* Param: yd_cht_count *//* Param: yd_cct_stamp_duty *//* Param: yd_cct_securities_management_fee *//* Param: yd_cct_handling_fee *//* Param: yd_cct_transfer_fee *//* Param: yd_cct_brokerage_fee *//* Param: yd_cct_count *//* Param: yd_cif_support_day_trading *//* Param: yd_cif_support_trading *//* Param: yd_cif_support_creation_redemption *//* Param: yd_cif_is_repo *//* Param: yd_tcf_st *//* Param: yd_tcf_star_st *//* Param: yd_tcf_qualified_bond_investor *//* Param: yd_tcf_qualified_bond_corp_investor *//* Param: yd_mdf_pause_trading */// FunctionPrototype: oppositePositionDirection(int)
/* Param: yd_missing_order */
/* Generated by handle_api */
impl YDApi {

    pub fn start(&mut self, p_listener: *const dyn YDListenerTrait) -> bool {
        let p_listener = Box::into_raw(Box::new((&YD_LISTENER_VTABLE, p_listener)));
        unsafe {
            ((*(*self).vtable_).YDApi_start)(self as *mut YDApi, p_listener as *mut YDListener)
        }
    }
    pub fn start_destroy(&mut self) -> () {
        unsafe {
            ((*(*self).vtable_).YDApi_startDestroy)(self as *mut YDApi)
        }
    }
    pub fn disconnect(&mut self) -> () {
        unsafe {
            ((*(*self).vtable_).YDApi_disconnect)(self as *mut YDApi)
        }
    }
    pub fn login(&mut self, username: std::ffi::CString, password: std::ffi::CString, app_id: std::ffi::CString, auth_code: std::ffi::CString) -> bool {
        unsafe {
            ((*(*self).vtable_).YDApi_login)(self as *mut YDApi, username.as_ptr(), password.as_ptr(), app_id.as_ptr(), auth_code.as_ptr())
        }
    }
    pub fn insert_order(&mut self, p_input_order: &mut YDInputOrder, p_instrument: &mut YDInstrument, p_account: &mut YDAccount) -> bool {
        unsafe {
            ((*(*self).vtable_).YDApi_insertOrder)(self as *mut YDApi, &mut *p_input_order, &mut *p_instrument, &mut *p_account)
        }
    }
    pub fn cancel_order(&mut self, p_cancel_order: &mut YDCancelOrder, p_exchange: &mut YDExchange, p_account: &mut YDAccount) -> bool {
        unsafe {
            ((*(*self).vtable_).YDApi_cancelOrder)(self as *mut YDApi, &mut *p_cancel_order, &mut *p_exchange, &mut *p_account)
        }
    }
    pub fn insert_quote(&mut self, p_input_quote: &mut YDInputQuote, p_instrument: &mut YDInstrument, p_account: &mut YDAccount) -> bool {
        unsafe {
            ((*(*self).vtable_).YDApi_insertQuote)(self as *mut YDApi, &mut *p_input_quote, &mut *p_instrument, &mut *p_account)
        }
    }
    pub fn cancel_quote(&mut self, p_cancel_quote: &mut YDCancelQuote, p_exchange: &mut YDExchange, p_account: &mut YDAccount) -> bool {
        unsafe {
            ((*(*self).vtable_).YDApi_cancelQuote)(self as *mut YDApi, &mut *p_cancel_quote, &mut *p_exchange, &mut *p_account)
        }
    }
    pub fn insert_comb_position_order(&mut self, p_input_order: &mut YDInputOrder, p_comb_position_def: &mut YDCombPositionDef, p_account: &mut YDAccount) -> bool {
        unsafe {
            ((*(*self).vtable_).YDApi_insertCombPositionOrder)(self as *mut YDApi, &mut *p_input_order, &mut *p_comb_position_def, &mut *p_account)
        }
    }
    pub fn insert_option_exec_together_order(&mut self, p_input_order: &mut YDInputOrder, p_instrument: &mut YDInstrument, p_instrument_2: &mut YDInstrument, p_account: &mut YDAccount) -> bool {
        unsafe {
            ((*(*self).vtable_).YDApi_insertOptionExecTogetherOrder)(self as *mut YDApi, &mut *p_input_order, &mut *p_instrument, &mut *p_instrument_2, &mut *p_account)
        }
    }
    // insert_multi_orders // Ignored (MethodFlavor::ApiTrait)
    // cancel_multi_orders // Ignored (MethodFlavor::ApiTrait)
    // insert_multi_quotes // Ignored (MethodFlavor::ApiTrait)
    // cancel_multi_quotes // Ignored (MethodFlavor::ApiTrait)
    pub fn subscribe(&mut self, p_instrument: &mut YDInstrument) -> bool {
        unsafe {
            ((*(*self).vtable_).YDApi_subscribe)(self as *mut YDApi, &mut *p_instrument)
        }
    }
    pub fn unsubscribe(&mut self, p_instrument: &mut YDInstrument) -> bool {
        unsafe {
            ((*(*self).vtable_).YDApi_unsubscribe)(self as *mut YDApi, &mut *p_instrument)
        }
    }
    pub fn set_trading_right(&mut self, p_account: &mut YDAccount, p_instrument: &mut YDInstrument, p_product: &mut YDProduct, p_exchange: &mut YDExchange, trading_right: std::os::raw::c_int, request_id: std::os::raw::c_int, trading_right_source: std::os::raw::c_int) -> bool {
        unsafe {
            ((*(*self).vtable_).YDApi_setTradingRight)(self as *mut YDApi, &mut *p_account, &mut *p_instrument, &mut *p_product, &mut *p_exchange, trading_right, request_id, trading_right_source)
        }
    }
    pub fn alter_money(&mut self, p_account: &mut YDAccount, alter_money_type: std::os::raw::c_int, alter_value: f64, request_id: std::os::raw::c_int) -> bool {
        unsafe {
            ((*(*self).vtable_).YDApi_alterMoney)(self as *mut YDApi, &mut *p_account, alter_money_type, alter_value, request_id)
        }
    }
    pub fn update_margin_rate(&mut self, p_update_margin_rate: &mut YDUpdateMarginRate, request_id: std::os::raw::c_int) -> bool {
        unsafe {
            ((*(*self).vtable_).YDApi_updateMarginRate)(self as *mut YDApi, &mut *p_update_margin_rate, request_id)
        }
    }
    pub fn update_message_commission_config(&mut self, p_update_message_commission_config: &mut YDUpdateMessageCommissionConfig, request_id: std::os::raw::c_int) -> bool {
        unsafe {
            ((*(*self).vtable_).YDApi_updateMessageCommissionConfig)(self as *mut YDApi, &mut *p_update_message_commission_config, request_id)
        }
    }
    pub fn adjust_account_margin_model_info(&mut self, p_account_margin_model_info: &mut YDAccountMarginModelInfo, request_id: std::os::raw::c_int) -> bool {
        unsafe {
            ((*(*self).vtable_).YDApi_adjustAccountMarginModelInfo)(self as *mut YDApi, &mut *p_account_margin_model_info, request_id)
        }
    }
    pub fn update_spot_position(&mut self, p_account: &mut YDAccount, p_instrument: &mut YDInstrument, position: std::os::raw::c_int, request_id: std::os::raw::c_int) -> bool {
        unsafe {
            ((*(*self).vtable_).YDApi_updateSpotPosition)(self as *mut YDApi, &mut *p_account, &mut *p_instrument, position, request_id)
        }
    }
    pub fn update_spot_alive(&mut self, p_exchange: &mut YDExchange, request_id: std::os::raw::c_int) -> bool {
        unsafe {
            ((*(*self).vtable_).YDApi_updateSpotAlive)(self as *mut YDApi, &mut *p_exchange, request_id)
        }
    }
    pub fn update_holding_external_frozen(&mut self, p_account: &mut YDAccount, p_instrument: &mut YDInstrument, external_sell_frozen: std::os::raw::c_int, request_id: std::os::raw::c_int) -> bool {
        unsafe {
            ((*(*self).vtable_).YDApi_updateHoldingExternalFrozen)(self as *mut YDApi, &mut *p_account, &mut *p_instrument, external_sell_frozen, request_id)
        }
    }
    pub fn change_password(&mut self, username: std::ffi::CString, old_password: std::ffi::CString, new_password: std::ffi::CString, request_id: std::os::raw::c_int) -> bool {
        unsafe {
            ((*(*self).vtable_).YDApi_changePassword)(self as *mut YDApi, username.as_ptr(), old_password.as_ptr(), new_password.as_ptr(), request_id)
        }
    }
    pub fn select_connections(&mut self, p_exchange: &mut YDExchange, connection_list: u64, request_id: std::os::raw::c_int) -> bool {
        unsafe {
            ((*(*self).vtable_).YDApi_selectConnections)(self as *mut YDApi, &mut *p_exchange, connection_list, request_id)
        }
    }
    pub fn has_finished_init(&mut self) -> bool {
        unsafe {
            ((*(*self).vtable_).YDApi_hasFinishedInit)(self as *mut YDApi)
        }
    }
    pub fn get_system_param_count(&mut self) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).YDApi_getSystemParamCount)(self as *mut YDApi)
        }
    }
    pub fn get_system_param(&mut self, pos: std::os::raw::c_int) -> *const YDSystemParam {
        unsafe {
            ((*(*self).vtable_).YDApi_getSystemParam)(self as *mut YDApi, pos)
        }
    }
    pub fn get_system_param_by_name(&mut self, name: std::ffi::CString, target: std::ffi::CString) -> *const YDSystemParam {
        unsafe {
            ((*(*self).vtable_).YDApi_getSystemParamByName)(self as *mut YDApi, name.as_ptr(), target.as_ptr())
        }
    }
    pub fn get_exchange_count(&mut self) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).YDApi_getExchangeCount)(self as *mut YDApi)
        }
    }
    pub fn get_exchange(&mut self, pos: std::os::raw::c_int) -> *const YDExchange {
        unsafe {
            ((*(*self).vtable_).YDApi_getExchange)(self as *mut YDApi, pos)
        }
    }
    pub fn get_exchange_by_id(&mut self, exchange_id: std::ffi::CString) -> *const YDExchange {
        unsafe {
            ((*(*self).vtable_).YDApi_getExchangeByID)(self as *mut YDApi, exchange_id.as_ptr())
        }
    }
    pub fn get_product_count(&mut self) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).YDApi_getProductCount)(self as *mut YDApi)
        }
    }
    pub fn get_product(&mut self, pos: std::os::raw::c_int) -> *const YDProduct {
        unsafe {
            ((*(*self).vtable_).YDApi_getProduct)(self as *mut YDApi, pos)
        }
    }
    pub fn get_product_by_id(&mut self, product_id: std::ffi::CString) -> *const YDProduct {
        unsafe {
            ((*(*self).vtable_).YDApi_getProductByID)(self as *mut YDApi, product_id.as_ptr())
        }
    }
    pub fn get_instrument_count(&mut self) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).YDApi_getInstrumentCount)(self as *mut YDApi)
        }
    }
    pub fn get_instrument(&mut self, pos: std::os::raw::c_int) -> *const YDInstrument {
        unsafe {
            ((*(*self).vtable_).YDApi_getInstrument)(self as *mut YDApi, pos)
        }
    }
    pub fn get_instrument_by_id(&mut self, instrument_id: std::ffi::CString) -> *const YDInstrument {
        unsafe {
            ((*(*self).vtable_).YDApi_getInstrumentByID)(self as *mut YDApi, instrument_id.as_ptr())
        }
    }
    pub fn get_comb_position_def_count(&mut self) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).YDApi_getCombPositionDefCount)(self as *mut YDApi)
        }
    }
    pub fn get_comb_position_def(&mut self, pos: std::os::raw::c_int) -> *const YDCombPositionDef {
        unsafe {
            ((*(*self).vtable_).YDApi_getCombPositionDef)(self as *mut YDApi, pos)
        }
    }
    pub fn get_comb_position_def_by_id(&mut self, comb_position_id: std::ffi::CString, comb_hedge_flag: std::os::raw::c_int) -> *const YDCombPositionDef {
        unsafe {
            ((*(*self).vtable_).YDApi_getCombPositionDefByID)(self as *mut YDApi, comb_position_id.as_ptr(), comb_hedge_flag)
        }
    }
    pub fn get_account_count(&mut self) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).YDApi_getAccountCount)(self as *mut YDApi)
        }
    }
    pub fn get_account(&mut self, pos: std::os::raw::c_int) -> *const YDAccount {
        unsafe {
            ((*(*self).vtable_).YDApi_getAccount)(self as *mut YDApi, pos)
        }
    }
    pub fn get_account_by_id(&mut self, account_id: std::ffi::CString) -> *const YDAccount {
        unsafe {
            ((*(*self).vtable_).YDApi_getAccountByID)(self as *mut YDApi, account_id.as_ptr())
        }
    }
    pub fn get_my_account(&mut self) -> *const YDAccount {
        unsafe {
            ((*(*self).vtable_).YDApi_getMyAccount)(self as *mut YDApi)
        }
    }
    pub fn get_pre_position_count(&mut self) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).YDApi_getPrePositionCount)(self as *mut YDApi)
        }
    }
    pub fn get_pre_position(&mut self, pos: std::os::raw::c_int) -> *const YDPrePosition {
        unsafe {
            ((*(*self).vtable_).YDApi_getPrePosition)(self as *mut YDApi, pos)
        }
    }
    pub fn get_pre_holding_count(&mut self) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).YDApi_getPreHoldingCount)(self as *mut YDApi)
        }
    }
    pub fn get_pre_holding(&mut self, pos: std::os::raw::c_int) -> *const YDPreHolding {
        unsafe {
            ((*(*self).vtable_).YDApi_getPreHolding)(self as *mut YDApi, pos)
        }
    }
    pub fn get_spot_pre_position_count(&mut self) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).YDApi_getSpotPrePositionCount)(self as *mut YDApi)
        }
    }
    pub fn get_spot_pre_position(&mut self, pos: std::os::raw::c_int) -> *const YDSpotPrePosition {
        unsafe {
            ((*(*self).vtable_).YDApi_getSpotPrePosition)(self as *mut YDApi, pos)
        }
    }
    pub fn get_margin_rate_count(&mut self) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).YDApi_getMarginRateCount)(self as *mut YDApi)
        }
    }
    pub fn get_margin_rate(&mut self, pos: std::os::raw::c_int) -> *const YDMarginRate {
        unsafe {
            ((*(*self).vtable_).YDApi_getMarginRate)(self as *mut YDApi, pos)
        }
    }
    pub fn get_commission_rate_count(&mut self) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).YDApi_getCommissionRateCount)(self as *mut YDApi)
        }
    }
    pub fn get_commission_rate(&mut self, pos: std::os::raw::c_int) -> *const YDCommissionRate {
        unsafe {
            ((*(*self).vtable_).YDApi_getCommissionRate)(self as *mut YDApi, pos)
        }
    }
    pub fn get_cash_commission_rate_count(&mut self) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).YDApi_getCashCommissionRateCount)(self as *mut YDApi)
        }
    }
    pub fn get_cash_commission_rate(&mut self, pos: std::os::raw::c_int) -> *const YDCashCommissionRate {
        unsafe {
            ((*(*self).vtable_).YDApi_getCashCommissionRate)(self as *mut YDApi, pos)
        }
    }
    pub fn get_message_commission_rate_count(&mut self) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).YDApi_getMessageCommissionRateCount)(self as *mut YDApi)
        }
    }
    pub fn get_message_commission_rate(&mut self, pos: std::os::raw::c_int) -> *const YDMessageCommissionRate {
        unsafe {
            ((*(*self).vtable_).YDApi_getMessageCommissionRate)(self as *mut YDApi, pos)
        }
    }
    pub fn get_margin_model_param_count(&mut self) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).YDApi_getMarginModelParamCount)(self as *mut YDApi)
        }
    }
    pub fn get_margin_model_param(&mut self, pos: std::os::raw::c_int) -> *const YDMarginModelParam {
        unsafe {
            ((*(*self).vtable_).YDApi_getMarginModelParam)(self as *mut YDApi, pos)
        }
    }
    pub fn get_account_exchange_info(&mut self, p_exchange: &mut YDExchange, p_account: &mut YDAccount) -> *const YDAccountExchangeInfo {
        unsafe {
            ((*(*self).vtable_).YDApi_getAccountExchangeInfo)(self as *mut YDApi, &mut *p_exchange, &mut *p_account)
        }
    }
    pub fn get_account_product_info(&mut self, p_product: &mut YDProduct, p_account: &mut YDAccount) -> *const YDAccountProductInfo {
        unsafe {
            ((*(*self).vtable_).YDApi_getAccountProductInfo)(self as *mut YDApi, &mut *p_product, &mut *p_account)
        }
    }
    pub fn get_account_instrument_info(&mut self, p_instrument: &mut YDInstrument, p_account: &mut YDAccount) -> *const YDAccountInstrumentInfo {
        unsafe {
            ((*(*self).vtable_).YDApi_getAccountInstrumentInfo)(self as *mut YDApi, &mut *p_instrument, &mut *p_account)
        }
    }
    pub fn get_instrument_margin_rate(&mut self, p_instrument: &mut YDInstrument, hedge_flag: std::os::raw::c_int, p_account: &mut YDAccount) -> *const YDMarginRate {
        unsafe {
            ((*(*self).vtable_).YDApi_getInstrumentMarginRate)(self as *mut YDApi, &mut *p_instrument, hedge_flag, &mut *p_account)
        }
    }
    pub fn get_instrument_commission_rate(&mut self, p_instrument: &mut YDInstrument, hedge_flag: std::os::raw::c_int, p_account: &mut YDAccount) -> *const YDCommissionRate {
        unsafe {
            ((*(*self).vtable_).YDApi_getInstrumentCommissionRate)(self as *mut YDApi, &mut *p_instrument, hedge_flag, &mut *p_account)
        }
    }
    pub fn get_instrument_cash_commission_rate(&mut self, p_instrument: &mut YDInstrument, yd_order_flag: std::os::raw::c_int, direction: std::os::raw::c_int, p_account: &mut YDAccount) -> *const YDCashCommissionRate {
        unsafe {
            ((*(*self).vtable_).YDApi_getInstrumentCashCommissionRate)(self as *mut YDApi, &mut *p_instrument, yd_order_flag, direction, &mut *p_account)
        }
    }
    pub fn get_account_margin_model_info(&mut self, margin_model_id: std::os::raw::c_int, p_account: &mut YDAccount) -> *const YDAccountMarginModelInfo {
        unsafe {
            ((*(*self).vtable_).YDApi_getAccountMarginModelInfo)(self as *mut YDApi, margin_model_id, &mut *p_account)
        }
    }
    pub fn get_general_risk_param_count(&mut self) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).YDApi_getGeneralRiskParamCount)(self as *mut YDApi)
        }
    }
    pub fn get_general_risk_param(&mut self, pos: std::os::raw::c_int) -> *const YDGeneralRiskParam {
        unsafe {
            ((*(*self).vtable_).YDApi_getGeneralRiskParam)(self as *mut YDApi, pos)
        }
    }
    pub fn write_log(&mut self, format: std::ffi::CString) -> () {
        unsafe {
            ((*(*self).vtable_).YDApi_writeLog)(self as *mut YDApi, format.as_ptr())
        }
    }
    pub fn get_version(&mut self) -> *const std::os::raw::c_char {
        unsafe {
            ((*(*self).vtable_).YDApi_getVersion)(self as *mut YDApi)
        }
    }
/* Param: yd_packet_type */    pub fn get_client_packet_header(&mut self, type_: YDApi_YDPacketType, p_header: u8, len: std::os::raw::c_int, protocol_version: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).YDApi_getClientPacketHeader)(self as *mut YDApi, type_, p_header as *mut ::std::os::raw::c_uchar, len, protocol_version)
        }
    }
    pub fn get_trading_day(&mut self) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).YDApi_getTradingDay)(self as *mut YDApi)
        }
    }
    pub fn get_session_id(&mut self) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).YDApi_getSessionID)(self as *mut YDApi)
        }
    }
    pub fn get_config(&mut self, name: std::ffi::CString) -> *const std::os::raw::c_char {
        unsafe {
            ((*(*self).vtable_).YDApi_getConfig)(self as *mut YDApi, name.as_ptr())
        }
    }
    pub fn get_configs(&mut self, name: std::ffi::CString) -> *const YDQueryResult {
        unsafe {
            ((*(*self).vtable_).YDApi_getConfigs)(self as *mut YDApi, name.as_ptr())
        }
    }

    /// encapsulate the raw pointer within YDApi
    pub unsafe fn from_raw(api_ptr: *mut YDApi) -> Box<YDApi> {
        // Ensure the pointer is not null
        if api_ptr.is_null() {
            panic!("YDApi pointer is null");
        }

        // Dereference the pointer to obtain YDApi and return it
        // This assumes that YDApi can be directly obtained from the pointer
        // Adjust based on your actual struct layout
        Box::from_raw(api_ptr)
    }
}
unsafe impl Send for YDApi {}
/* Param: yd_error_no_error *//* Param: yd_error_no_position_to_close *//* Param: yd_error_no_money_to_open *//* Param: yd_error_system_not_ready *//* Param: yd_error_invalid_instrument *//* Param: yd_error_invalid_account *//* Param: yd_error_order_field_error *//* Param: yd_error_memory_exceed *//* Param: yd_error_no_trading_code_in_exchange *//* Param: yd_error_can_not_send_to_exchange *//* Param: yd_error_no_trading_right *//* Param: yd_error_invalid_order_volume *//* Param: yd_error_invalid_client_app *//* Param: yd_error_position_limit_exceed *//* Param: yd_error_trade_volume_exceed *//* Param: yd_error_order_cancel_limit_exceed *//* Param: yd_error_order_open_limit_exceed *//* Param: yd_error_invalid_connection_id *//* Param: yd_error_already_logined *//* Param: yd_error_password_error *//* Param: yd_error_too_many_requests *//* Param: yd_error_invalid_username *//* Param: yd_error_username_mismatch *//* Param: yd_error_old_password_mismatch *//* Param: yd_error_insert_order_too_fast *//* Param: yd_error_possible_self_trade *//* Param: yd_error_no_admin_right *//* Param: yd_error_invalid_address *//* Param: yd_error_order_type_not_supported *//* Param: yd_error_cancel_order_field_error *//* Param: yd_error_invalid_exchange *//* Param: yd_error_order_not_found *//* Param: yd_error_order_not_belong_to_account *//* Param: yd_error_order_finished *//* Param: yd_error_only_limit_order_can_be_canceled *//* Param: yd_error_client_report_error *//* Param: yd_error_too_many_orders *//* Param: yd_error_instrument_can_not_trade *//* Param: yd_error_yd_order_flag_not_supported *//* Param: yd_error_not_option_instrument *//* Param: yd_error_price_out_of_limit *//* Param: yd_error_cross_price_in_quote *//* Param: yd_error_quote_field_error *//* Param: yd_error_quote_volume_error *//* Param: yd_error_quote_not_found *//* Param: yd_error_cancel_quote_field_error *//* Param: yd_error_quote_not_belong_to_account *//* Param: yd_error_quote_finished *//* Param: yd_error_quote_not_supported *//* Param: yd_error_cannot_cancel_quote_derived_order *//* Param: yd_error_too_many_logines *//* Param: yd_error_no_enough_positionto_make_comb_position *//* Param: yd_error_no_enough_comb_position *//* Param: yd_error_no_money_for_split_comb_position *//* Param: yd_error_invalid_comb_position *//* Param: yd_error_cannot_select_connection *//* Param: yd_error_select_connection_too_frequently *//* Param: yd_error_invalid_select_connection *//* Param: yd_error_too_low_api_version *//* Param: yd_error_invalid_trading_right *//* Param: yd_error_invalid_product *//* Param: yd_error_invalid_alter_money_field *//* Param: yd_error_too_high_api_version *//* Param: yd_error_money_usage_too_low *//* Param: yd_error_invalid_instrument_pair_to_execute_together *//* Param: yd_error_not_on_expire_day *//* Param: yd_error_not_proper_time *//* Param: yd_error_opton_long_position_cost_limit_exceed *//* Param: yd_error_price_to_trigger_fuse *//* Param: yd_error_exchange_does_not_support *//* Param: yd_error_not_cash_instrument *//* Param: yd_error_no_enough_positionto_freeze *//* Param: yd_error_no_enough_positionto_unfreeze *//* Param: yd_error_cannot_cover_long_position *//* Param: yd_error_not_enough_spot_position_for_cover *//* Param: yd_error_not_enough_position_for_cover_order *//* Param: yd_error_not_enough_money_for_cover_order *//* Param: yd_error_not_enough_spot_position_for_exec *//* Param: yd_error_invalid_order_ref *//* Param: yd_error_can_not_send_to_exchange_for_flow_control *//* Param: yd_error_exchange_connection_send_error *//* Param: yd_error_internal_rejected *//* Param: yd_error_cannot_set_trading_right *//* Param: yd_error_invalid_group_order_ref *//* Param: yd_error_invalid_order_group_id *//* Param: yd_error_no_login *//* Param: yd_error_invalid_account_margin_model_info_field *//* Param: yd_error_account_not_using_this_margin_model *//* Param: yd_error_cannot_use_comb_position_in_this_margin_model *//* Param: yd_error_too_many_cancels *//* Param: yd_error_message_count_exceed *//* Param: yd_error_direction_offset_flag_mismatch_in_order *//* Param: yd_error_price_tick_error *//* Param: yd_error_instrument_trading_paused *//* Param: yd_error_not_qualified_investor *//* Param: yd_error_not_tradable_product *//* Param: yd_error_holding_limit_exceed *//* Param: yd_error_cash_buy_volume_limit_exceed *//* Param: yd_error_cannot_update_holding_external_frozen *//* Param: yd_error_password_not_set *//* Param: yd_error_need_ignore *//* Param: yd_error_cannot_send *//* Param: yd_error_too_many_in_multi_orders *//* Param: yd_error_exchange_report_error */// FunctionPrototype: string2TimeID(const char *)
// FunctionPrototype: string2TimeStamp(const char *)
// FunctionPrototype: timeID2String(unsigned int, char *)
// FunctionPrototype: timeStamp2String(unsigned int, char *)
// FunctionPrototype: dumpField(FILE *, int)
// FunctionPrototype: dumpField(FILE *, unsigned int)
// FunctionPrototype: dumpField(FILE *, short)
// FunctionPrototype: dumpField(FILE *, unsigned short)
// FunctionPrototype: dumpField(FILE *, char)
// FunctionPrototype: dumpField(FILE *, unsigned char)
// FunctionPrototype: dumpField(FILE *, unsigned long long)
// FunctionPrototype: dumpField(FILE *, long long)
// FunctionPrototype: dumpField(FILE *, bool)
// FunctionPrototype: dumpField(FILE *, double)
// FunctionPrototype: dumpField(FILE *, const char *)
// FunctionPrototype: dumpTimeField(FILE *, int)
// FunctionPrototype: dumpTimeStampField(FILE *, int)
