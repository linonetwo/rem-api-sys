use crate::{bindings::*};

/* Param: thost_te_resume_type *//* Param: t_thost_ftdc_trader_id_type *//* Param: t_thost_ftdc_investor_id_type *//* Param: t_thost_ftdc_broker_id_type *//* Param: t_thost_ftdc_broker_abbr_type *//* Param: t_thost_ftdc_broker_name_type *//* Param: t_thost_ftdc_old_exchange_inst_id_type *//* Param: t_thost_ftdc_exchange_inst_id_type *//* Param: t_thost_ftdc_order_ref_type *//* Param: t_thost_ftdc_participant_id_type *//* Param: t_thost_ftdc_user_id_type *//* Param: t_thost_ftdc_password_type *//* Param: t_thost_ftdc_client_id_type *//* Param: t_thost_ftdc_instrument_id_type *//* Param: t_thost_ftdc_old_instrument_id_type *//* Param: t_thost_ftdc_instrument_code_type *//* Param: t_thost_ftdc_market_id_type *//* Param: t_thost_ftdc_product_name_type *//* Param: t_thost_ftdc_exchange_id_type *//* Param: t_thost_ftdc_exchange_name_type *//* Param: t_thost_ftdc_exchange_abbr_type *//* Param: t_thost_ftdc_exchange_flag_type *//* Param: t_thost_ftdc_mac_address_type *//* Param: t_thost_ftdc_system_id_type *//* Param: t_thost_ftdc_client_login_remark_type *//* Param: t_thost_ftdc_exchange_property_type *//* Param: t_thost_ftdc_date_type *//* Param: t_thost_ftdc_time_type *//* Param: t_thost_ftdc_long_time_type *//* Param: t_thost_ftdc_instrument_name_type *//* Param: t_thost_ftdc_settlement_group_id_type *//* Param: t_thost_ftdc_order_sys_id_type *//* Param: t_thost_ftdc_trade_id_type *//* Param: t_thost_ftdc_command_type_type *//* Param: t_thost_ftdc_old_ip_address_type *//* Param: t_thost_ftdc_ip_address_type *//* Param: t_thost_ftdc_ip_port_type *//* Param: t_thost_ftdc_product_info_type *//* Param: t_thost_ftdc_protocol_info_type *//* Param: t_thost_ftdc_business_unit_type *//* Param: t_thost_ftdc_deposit_seq_no_type *//* Param: t_thost_ftdc_identified_card_no_type *//* Param: t_thost_ftdc_id_card_type_type *//* Param: t_thost_ftdc_order_local_id_type *//* Param: t_thost_ftdc_user_name_type *//* Param: t_thost_ftdc_party_name_type *//* Param: t_thost_ftdc_error_msg_type *//* Param: t_thost_ftdc_field_name_type *//* Param: t_thost_ftdc_field_content_type *//* Param: t_thost_ftdc_system_name_type *//* Param: t_thost_ftdc_content_type *//* Param: t_thost_ftdc_investor_range_type *//* Param: t_thost_ftdc_department_range_type *//* Param: t_thost_ftdc_data_sync_status_type *//* Param: t_thost_ftdc_broker_data_sync_status_type *//* Param: t_thost_ftdc_exchange_connect_status_type *//* Param: t_thost_ftdc_trader_connect_status_type *//* Param: t_thost_ftdc_function_code_type *//* Param: t_thost_ftdc_broker_function_code_type *//* Param: t_thost_ftdc_order_action_status_type *//* Param: t_thost_ftdc_order_status_type *//* Param: t_thost_ftdc_order_submit_status_type *//* Param: t_thost_ftdc_position_date_type *//* Param: t_thost_ftdc_position_date_type_type *//* Param: t_thost_ftdc_trading_role_type *//* Param: t_thost_ftdc_product_class_type *//* Param: t_thost_ftdc_api_product_class_type *//* Param: t_thost_ftdc_inst_life_phase_type *//* Param: t_thost_ftdc_direction_type *//* Param: t_thost_ftdc_position_type_type *//* Param: t_thost_ftdc_posi_direction_type *//* Param: t_thost_ftdc_sys_settlement_status_type *//* Param: t_thost_ftdc_ratio_attr_type *//* Param: t_thost_ftdc_hedge_flag_type *//* Param: t_thost_ftdc_bill_hedge_flag_type *//* Param: t_thost_ftdc_client_id_type_type *//* Param: t_thost_ftdc_order_price_type_type *//* Param: t_thost_ftdc_offset_flag_type *//* Param: t_thost_ftdc_force_close_reason_type *//* Param: t_thost_ftdc_order_type_type *//* Param: t_thost_ftdc_time_condition_type *//* Param: t_thost_ftdc_volume_condition_type *//* Param: t_thost_ftdc_contingent_condition_type *//* Param: t_thost_ftdc_action_flag_type *//* Param: t_thost_ftdc_trading_right_type *//* Param: t_thost_ftdc_order_source_type *//* Param: t_thost_ftdc_trade_type_type *//* Param: t_thost_ftdc_spec_posi_type_type *//* Param: t_thost_ftdc_price_source_type *//* Param: t_thost_ftdc_instrument_status_type *//* Param: t_thost_ftdc_inst_status_enter_reason_type *//* Param: t_thost_ftdc_order_action_ref_type *//* Param: t_thost_ftdc_install_count_type *//* Param: t_thost_ftdc_install_id_type *//* Param: t_thost_ftdc_error_id_type *//* Param: t_thost_ftdc_settlement_id_type *//* Param: t_thost_ftdc_volume_type *//* Param: t_thost_ftdc_front_id_type *//* Param: t_thost_ftdc_session_id_type *//* Param: t_thost_ftdc_sequence_no_type *//* Param: t_thost_ftdc_command_no_type *//* Param: t_thost_ftdc_millisec_type *//* Param: t_thost_ftdc_sec_type *//* Param: t_thost_ftdc_volume_multiple_type *//* Param: t_thost_ftdc_trading_segment_sn_type *//* Param: t_thost_ftdc_request_id_type *//* Param: t_thost_ftdc_year_type *//* Param: t_thost_ftdc_month_type *//* Param: t_thost_ftdc_bool_type *//* Param: t_thost_ftdc_price_type *//* Param: t_thost_ftdc_comb_offset_flag_type *//* Param: t_thost_ftdc_comb_hedge_flag_type *//* Param: t_thost_ftdc_ratio_type *//* Param: t_thost_ftdc_money_type *//* Param: t_thost_ftdc_large_volume_type *//* Param: t_thost_ftdc_sequence_series_type *//* Param: t_thost_ftdc_comm_phase_no_type *//* Param: t_thost_ftdc_sequence_label_type *//* Param: t_thost_ftdc_underlying_multiple_type *//* Param: t_thost_ftdc_priority_type *//* Param: t_thost_ftdc_contract_code_type *//* Param: t_thost_ftdc_city_type *//* Param: t_thost_ftdc_is_stock_type *//* Param: t_thost_ftdc_channel_type *//* Param: t_thost_ftdc_address_type *//* Param: t_thost_ftdc_zip_code_type *//* Param: t_thost_ftdc_telephone_type *//* Param: t_thost_ftdc_fax_type *//* Param: t_thost_ftdc_mobile_type *//* Param: t_thost_ftdc_e_mail_type *//* Param: t_thost_ftdc_memo_type *//* Param: t_thost_ftdc_company_code_type *//* Param: t_thost_ftdc_website_type *//* Param: t_thost_ftdc_tax_no_type *//* Param: t_thost_ftdc_batch_status_type *//* Param: t_thost_ftdc_property_id_type *//* Param: t_thost_ftdc_property_name_type *//* Param: t_thost_ftdc_license_no_type *//* Param: t_thost_ftdc_agent_id_type *//* Param: t_thost_ftdc_agent_name_type *//* Param: t_thost_ftdc_agent_group_id_type *//* Param: t_thost_ftdc_agent_group_name_type *//* Param: t_thost_ftdc_return_style_type *//* Param: t_thost_ftdc_return_pattern_type *//* Param: t_thost_ftdc_return_level_type *//* Param: t_thost_ftdc_return_standard_type *//* Param: t_thost_ftdc_mortgage_type_type *//* Param: t_thost_ftdc_investor_settlement_param_id_type *//* Param: t_thost_ftdc_exchange_settlement_param_id_type *//* Param: t_thost_ftdc_system_param_id_type *//* Param: t_thost_ftdc_trade_param_id_type *//* Param: t_thost_ftdc_settlement_param_value_type *//* Param: t_thost_ftdc_counter_id_type *//* Param: t_thost_ftdc_investor_group_name_type *//* Param: t_thost_ftdc_brand_code_type *//* Param: t_thost_ftdc_warehouse_type *//* Param: t_thost_ftdc_product_date_type *//* Param: t_thost_ftdc_grade_type *//* Param: t_thost_ftdc_classify_type *//* Param: t_thost_ftdc_position_type *//* Param: t_thost_ftdc_yieldly_type *//* Param: t_thost_ftdc_weight_type *//* Param: t_thost_ftdc_sub_entry_fund_no_type *//* Param: t_thost_ftdc_file_id_type *//* Param: t_thost_ftdc_file_name_type *//* Param: t_thost_ftdc_file_type_type *//* Param: t_thost_ftdc_file_format_type *//* Param: t_thost_ftdc_file_upload_status_type *//* Param: t_thost_ftdc_transfer_direction_type *//* Param: t_thost_ftdc_upload_mode_type *//* Param: t_thost_ftdc_account_id_type *//* Param: t_thost_ftdc_bank_flag_type *//* Param: t_thost_ftdc_bank_account_type *//* Param: t_thost_ftdc_open_name_type *//* Param: t_thost_ftdc_open_bank_type *//* Param: t_thost_ftdc_bank_name_type *//* Param: t_thost_ftdc_publish_path_type *//* Param: t_thost_ftdc_operator_id_type *//* Param: t_thost_ftdc_month_count_type *//* Param: t_thost_ftdc_advance_month_array_type *//* Param: t_thost_ftdc_date_expr_type *//* Param: t_thost_ftdc_instrument_id_expr_type *//* Param: t_thost_ftdc_instrument_name_expr_type *//* Param: t_thost_ftdc_special_create_rule_type *//* Param: t_thost_ftdc_basis_price_type_type *//* Param: t_thost_ftdc_product_life_phase_type *//* Param: t_thost_ftdc_delivery_mode_type *//* Param: t_thost_ftdc_log_level_type *//* Param: t_thost_ftdc_process_name_type *//* Param: t_thost_ftdc_operation_memo_type *//* Param: t_thost_ftdc_fund_io_type_type *//* Param: t_thost_ftdc_fund_type_type *//* Param: t_thost_ftdc_fund_direction_type *//* Param: t_thost_ftdc_fund_status_type *//* Param: t_thost_ftdc_bill_no_type *//* Param: t_thost_ftdc_bill_name_type *//* Param: t_thost_ftdc_publish_status_type *//* Param: t_thost_ftdc_enum_value_id_type *//* Param: t_thost_ftdc_enum_value_type_type *//* Param: t_thost_ftdc_enum_value_label_type *//* Param: t_thost_ftdc_enum_value_result_type *//* Param: t_thost_ftdc_system_status_type *//* Param: t_thost_ftdc_settlement_status_type *//* Param: t_thost_ftdc_range_int_type_type *//* Param: t_thost_ftdc_range_int_from_type *//* Param: t_thost_ftdc_range_int_to_type *//* Param: t_thost_ftdc_function_id_type *//* Param: t_thost_ftdc_function_value_code_type *//* Param: t_thost_ftdc_function_name_type *//* Param: t_thost_ftdc_role_id_type *//* Param: t_thost_ftdc_role_name_type *//* Param: t_thost_ftdc_description_type *//* Param: t_thost_ftdc_combine_id_type *//* Param: t_thost_ftdc_combine_type_type *//* Param: t_thost_ftdc_investor_type_type *//* Param: t_thost_ftdc_broker_type_type *//* Param: t_thost_ftdc_risk_level_type *//* Param: t_thost_ftdc_fee_accept_style_type *//* Param: t_thost_ftdc_password_type_type *//* Param: t_thost_ftdc_algorithm_type *//* Param: t_thost_ftdc_include_close_profit_type *//* Param: t_thost_ftdc_all_without_trade_type *//* Param: t_thost_ftdc_comment_type *//* Param: t_thost_ftdc_version_type *//* Param: t_thost_ftdc_trade_code_type *//* Param: t_thost_ftdc_trade_date_type *//* Param: t_thost_ftdc_trade_time_type *//* Param: t_thost_ftdc_trade_serial_type *//* Param: t_thost_ftdc_trade_serial_no_type *//* Param: t_thost_ftdc_future_id_type *//* Param: t_thost_ftdc_bank_id_type *//* Param: t_thost_ftdc_bank_brch_id_type *//* Param: t_thost_ftdc_bank_branch_id_type *//* Param: t_thost_ftdc_oper_no_type *//* Param: t_thost_ftdc_device_id_type *//* Param: t_thost_ftdc_record_num_type *//* Param: t_thost_ftdc_future_account_type *//* Param: t_thost_ftdc_future_pwd_flag_type *//* Param: t_thost_ftdc_transfer_type_type *//* Param: t_thost_ftdc_future_acc_pwd_type *//* Param: t_thost_ftdc_currency_code_type *//* Param: t_thost_ftdc_ret_code_type *//* Param: t_thost_ftdc_ret_info_type *//* Param: t_thost_ftdc_trade_amt_type *//* Param: t_thost_ftdc_use_amt_type *//* Param: t_thost_ftdc_fetch_amt_type *//* Param: t_thost_ftdc_transfer_valid_flag_type *//* Param: t_thost_ftdc_cert_code_type *//* Param: t_thost_ftdc_reason_type *//* Param: t_thost_ftdc_fund_project_id_type *//* Param: t_thost_ftdc_sex_type *//* Param: t_thost_ftdc_profession_type *//* Param: t_thost_ftdc_national_type *//* Param: t_thost_ftdc_province_type *//* Param: t_thost_ftdc_region_type *//* Param: t_thost_ftdc_country_type *//* Param: t_thost_ftdc_license_no_type *//* Param: t_thost_ftdc_company_type_type *//* Param: t_thost_ftdc_business_scope_type *//* Param: t_thost_ftdc_capital_currency_type *//* Param: t_thost_ftdc_user_type_type *//* Param: t_thost_ftdc_branch_id_type *//* Param: t_thost_ftdc_rate_type_type *//* Param: t_thost_ftdc_note_type_type *//* Param: t_thost_ftdc_settlement_style_type *//* Param: t_thost_ftdc_broker_dns_type *//* Param: t_thost_ftdc_sentence_type *//* Param: t_thost_ftdc_settlement_bill_type_type *//* Param: t_thost_ftdc_user_right_type_type *//* Param: t_thost_ftdc_margin_price_type_type *//* Param: t_thost_ftdc_bill_gen_status_type *//* Param: t_thost_ftdc_algo_type_type *//* Param: t_thost_ftdc_handle_position_algo_id_type *//* Param: t_thost_ftdc_find_margin_rate_algo_id_type *//* Param: t_thost_ftdc_handle_trading_account_algo_id_type *//* Param: t_thost_ftdc_person_type_type *//* Param: t_thost_ftdc_query_investor_range_type *//* Param: t_thost_ftdc_investor_risk_status_type *//* Param: t_thost_ftdc_leg_id_type *//* Param: t_thost_ftdc_leg_multiple_type *//* Param: t_thost_ftdc_imply_level_type *//* Param: t_thost_ftdc_clear_account_type *//* Param: t_thost_ftdc_organ_no_type *//* Param: t_thost_ftdc_clearbarch_id_type *//* Param: t_thost_ftdc_user_event_type_type *//* Param: t_thost_ftdc_user_event_info_type *//* Param: t_thost_ftdc_close_style_type *//* Param: t_thost_ftdc_stat_mode_type *//* Param: t_thost_ftdc_parked_order_status_type *//* Param: t_thost_ftdc_parked_order_id_type *//* Param: t_thost_ftdc_parked_order_action_id_type *//* Param: t_thost_ftdc_vir_deal_status_type *//* Param: t_thost_ftdc_org_system_id_type *//* Param: t_thost_ftdc_vir_trade_status_type *//* Param: t_thost_ftdc_vir_bank_acc_type_type *//* Param: t_thost_ftdc_virement_status_type *//* Param: t_thost_ftdc_virement_avail_ability_type *//* Param: t_thost_ftdc_virement_trade_code_type *//* Param: t_thost_ftdc_photo_type_name_type *//* Param: t_thost_ftdc_photo_type_id_type *//* Param: t_thost_ftdc_photo_name_type *//* Param: t_thost_ftdc_topic_id_type *//* Param: t_thost_ftdc_report_type_id_type *//* Param: t_thost_ftdc_character_id_type *//* Param: t_thost_ftdc_aml_param_id_type *//* Param: t_thost_ftdc_aml_investor_type_type *//* Param: t_thost_ftdc_aml_id_card_type_type *//* Param: t_thost_ftdc_aml_trade_direct_type *//* Param: t_thost_ftdc_aml_trade_model_type *//* Param: t_thost_ftdc_aml_op_param_value_type *//* Param: t_thost_ftdc_aml_customer_card_type_type *//* Param: t_thost_ftdc_aml_institution_name_type *//* Param: t_thost_ftdc_aml_district_id_type *//* Param: t_thost_ftdc_aml_relation_ship_type *//* Param: t_thost_ftdc_aml_institution_type_type *//* Param: t_thost_ftdc_aml_institution_id_type *//* Param: t_thost_ftdc_aml_account_type_type *//* Param: t_thost_ftdc_aml_trading_type_type *//* Param: t_thost_ftdc_aml_transact_class_type *//* Param: t_thost_ftdc_aml_capital_io_type *//* Param: t_thost_ftdc_aml_site_type *//* Param: t_thost_ftdc_aml_capital_purpose_type *//* Param: t_thost_ftdc_aml_report_type_type *//* Param: t_thost_ftdc_aml_serial_no_type *//* Param: t_thost_ftdc_aml_status_type *//* Param: t_thost_ftdc_aml_gen_status_type *//* Param: t_thost_ftdc_aml_seq_code_type *//* Param: t_thost_ftdc_aml_file_name_type *//* Param: t_thost_ftdc_aml_money_type *//* Param: t_thost_ftdc_aml_file_amount_type *//* Param: t_thost_ftdc_cfmmc_key_type *//* Param: t_thost_ftdc_cfmmc_token_type *//* Param: t_thost_ftdc_cfmmc_key_kind_type *//* Param: t_thost_ftdc_aml_report_name_type *//* Param: t_thost_ftdc_individual_name_type *//* Param: t_thost_ftdc_currency_id_type *//* Param: t_thost_ftdc_cust_number_type *//* Param: t_thost_ftdc_organ_code_type *//* Param: t_thost_ftdc_organ_name_type *//* Param: t_thost_ftdc_super_organ_code_type *//* Param: t_thost_ftdc_sub_branch_id_type *//* Param: t_thost_ftdc_sub_branch_name_type *//* Param: t_thost_ftdc_branch_net_code_type *//* Param: t_thost_ftdc_branch_net_name_type *//* Param: t_thost_ftdc_organ_flag_type *//* Param: t_thost_ftdc_bank_coding_for_future_type *//* Param: t_thost_ftdc_bank_return_code_type *//* Param: t_thost_ftdc_plate_return_code_type *//* Param: t_thost_ftdc_bank_sub_branch_id_type *//* Param: t_thost_ftdc_future_branch_id_type *//* Param: t_thost_ftdc_return_code_type *//* Param: t_thost_ftdc_operator_code_type *//* Param: t_thost_ftdc_clear_dep_id_type *//* Param: t_thost_ftdc_clear_brch_id_type *//* Param: t_thost_ftdc_clear_name_type *//* Param: t_thost_ftdc_bank_account_name_type *//* Param: t_thost_ftdc_inv_dep_id_type *//* Param: t_thost_ftdc_inv_brch_id_type *//* Param: t_thost_ftdc_message_format_version_type *//* Param: t_thost_ftdc_digest_type *//* Param: t_thost_ftdc_authentic_data_type *//* Param: t_thost_ftdc_password_key_type *//* Param: t_thost_ftdc_future_account_name_type *//* Param: t_thost_ftdc_mobile_phone_type *//* Param: t_thost_ftdc_future_main_key_type *//* Param: t_thost_ftdc_future_work_key_type *//* Param: t_thost_ftdc_future_trans_key_type *//* Param: t_thost_ftdc_bank_main_key_type *//* Param: t_thost_ftdc_bank_work_key_type *//* Param: t_thost_ftdc_bank_trans_key_type *//* Param: t_thost_ftdc_bank_server_description_type *//* Param: t_thost_ftdc_add_info_type *//* Param: t_thost_ftdc_descr_info_for_return_code_type *//* Param: t_thost_ftdc_country_code_type *//* Param: t_thost_ftdc_serial_type *//* Param: t_thost_ftdc_plate_serial_type *//* Param: t_thost_ftdc_bank_serial_type *//* Param: t_thost_ftdc_correct_serial_type *//* Param: t_thost_ftdc_future_serial_type *//* Param: t_thost_ftdc_application_id_type *//* Param: t_thost_ftdc_bank_proxy_id_type *//* Param: t_thost_ftdc_fbt_core_id_type *//* Param: t_thost_ftdc_server_port_type *//* Param: t_thost_ftdc_repealed_times_type *//* Param: t_thost_ftdc_repeal_time_interval_type *//* Param: t_thost_ftdc_total_times_type *//* Param: t_thost_ftdc_fbt_request_id_type *//* Param: t_thost_ftdc_tid_type *//* Param: t_thost_ftdc_trade_amount_type *//* Param: t_thost_ftdc_cust_fee_type *//* Param: t_thost_ftdc_future_fee_type *//* Param: t_thost_ftdc_single_max_amt_type *//* Param: t_thost_ftdc_single_min_amt_type *//* Param: t_thost_ftdc_total_amt_type *//* Param: t_thost_ftdc_certification_type_type *//* Param: t_thost_ftdc_file_business_code_type *//* Param: t_thost_ftdc_cash_exchange_code_type *//* Param: t_thost_ftdc_yes_no_indicator_type *//* Param: t_thost_ftdc_banlance_type_type *//* Param: t_thost_ftdc_gender_type *//* Param: t_thost_ftdc_fee_pay_flag_type *//* Param: t_thost_ftdc_pass_word_key_type_type *//* Param: t_thost_ftdc_fbt_pass_word_type_type *//* Param: t_thost_ftdc_fbt_encry_mode_type *//* Param: t_thost_ftdc_bank_repeal_flag_type *//* Param: t_thost_ftdc_broker_repeal_flag_type *//* Param: t_thost_ftdc_institution_type_type *//* Param: t_thost_ftdc_last_fragment_type *//* Param: t_thost_ftdc_bank_acc_status_type *//* Param: t_thost_ftdc_money_account_status_type *//* Param: t_thost_ftdc_manage_status_type *//* Param: t_thost_ftdc_system_type_type *//* Param: t_thost_ftdc_txn_end_flag_type *//* Param: t_thost_ftdc_process_status_type *//* Param: t_thost_ftdc_cust_type_type *//* Param: t_thost_ftdc_fbt_transfer_direction_type *//* Param: t_thost_ftdc_open_or_destroy_type *//* Param: t_thost_ftdc_availability_flag_type *//* Param: t_thost_ftdc_organ_type_type *//* Param: t_thost_ftdc_organ_level_type *//* Param: t_thost_ftdc_protocal_id_type *//* Param: t_thost_ftdc_connect_mode_type *//* Param: t_thost_ftdc_sync_mode_type *//* Param: t_thost_ftdc_bank_acc_type_type *//* Param: t_thost_ftdc_future_acc_type_type *//* Param: t_thost_ftdc_organ_status_type *//* Param: t_thost_ftdc_ccb_fee_mode_type *//* Param: t_thost_ftdc_comm_api_type_type *//* Param: t_thost_ftdc_service_id_type *//* Param: t_thost_ftdc_service_line_no_type *//* Param: t_thost_ftdc_service_name_type *//* Param: t_thost_ftdc_link_status_type *//* Param: t_thost_ftdc_comm_api_pointer_type *//* Param: t_thost_ftdc_pwd_flag_type *//* Param: t_thost_ftdc_secu_acc_type_type *//* Param: t_thost_ftdc_transfer_status_type *//* Param: t_thost_ftdc_sponsor_type_type *//* Param: t_thost_ftdc_req_rsp_type_type *//* Param: t_thost_ftdc_fbt_user_event_type_type *//* Param: t_thost_ftdc_bank_id_by_bank_type *//* Param: t_thost_ftdc_bank_oper_no_type *//* Param: t_thost_ftdc_bank_cust_no_type *//* Param: t_thost_ftdc_dbop_seq_no_type *//* Param: t_thost_ftdc_table_name_type *//* Param: t_thost_ftdc_pk_name_type *//* Param: t_thost_ftdc_pk_value_type *//* Param: t_thost_ftdc_db_operation_type *//* Param: t_thost_ftdc_sync_flag_type *//* Param: t_thost_ftdc_target_id_type *//* Param: t_thost_ftdc_sync_type_type *//* Param: t_thost_ftdc_fbe_time_type *//* Param: t_thost_ftdc_fbe_bank_no_type *//* Param: t_thost_ftdc_fbe_cert_no_type *//* Param: t_thost_ftdc_ex_direction_type *//* Param: t_thost_ftdc_fbe_bank_account_type *//* Param: t_thost_ftdc_fbe_bank_account_name_type *//* Param: t_thost_ftdc_fbe_amt_type *//* Param: t_thost_ftdc_fbe_business_type_type *//* Param: t_thost_ftdc_fbe_post_script_type *//* Param: t_thost_ftdc_fbe_remark_type *//* Param: t_thost_ftdc_ex_rate_type *//* Param: t_thost_ftdc_fbe_result_flag_type *//* Param: t_thost_ftdc_fbe_rtn_msg_type *//* Param: t_thost_ftdc_fbe_extend_msg_type *//* Param: t_thost_ftdc_fbe_business_serial_type *//* Param: t_thost_ftdc_fbe_system_serial_type *//* Param: t_thost_ftdc_fbe_total_ex_cnt_type *//* Param: t_thost_ftdc_fbe_exch_status_type *//* Param: t_thost_ftdc_fbe_file_flag_type *//* Param: t_thost_ftdc_fbe_already_trade_type *//* Param: t_thost_ftdc_fbe_open_bank_type *//* Param: t_thost_ftdc_fbe_user_event_type_type *//* Param: t_thost_ftdc_fbe_file_name_type *//* Param: t_thost_ftdc_fbe_batch_serial_type *//* Param: t_thost_ftdc_fbe_req_flag_type *//* Param: t_thost_ftdc_notify_class_type *//* Param: t_thost_ftdc_risk_nofity_info_type *//* Param: t_thost_ftdc_force_close_scene_id_type *//* Param: t_thost_ftdc_force_close_type_type *//* Param: t_thost_ftdc_instrument_i_ds_type *//* Param: t_thost_ftdc_risk_notify_method_type *//* Param: t_thost_ftdc_risk_notify_status_type *//* Param: t_thost_ftdc_risk_user_event_type *//* Param: t_thost_ftdc_param_id_type *//* Param: t_thost_ftdc_param_name_type *//* Param: t_thost_ftdc_param_value_type *//* Param: t_thost_ftdc_conditional_order_sort_type_type *//* Param: t_thost_ftdc_send_type_type *//* Param: t_thost_ftdc_client_id_status_type *//* Param: t_thost_ftdc_industry_id_type *//* Param: t_thost_ftdc_question_id_type *//* Param: t_thost_ftdc_question_content_type *//* Param: t_thost_ftdc_option_id_type *//* Param: t_thost_ftdc_option_content_type *//* Param: t_thost_ftdc_question_type_type *//* Param: t_thost_ftdc_process_id_type *//* Param: t_thost_ftdc_seq_no_type *//* Param: t_thost_ftdc_uoa_process_status_type *//* Param: t_thost_ftdc_process_type_type *//* Param: t_thost_ftdc_business_type_type *//* Param: t_thost_ftdc_cfmmc_return_code_type *//* Param: t_thost_ftdc_ex_return_code_type *//* Param: t_thost_ftdc_client_type_type *//* Param: t_thost_ftdc_exchange_id_type_type *//* Param: t_thost_ftdc_ex_client_id_type_type *//* Param: t_thost_ftdc_client_classify_type *//* Param: t_thost_ftdc_uoa_organ_type_type *//* Param: t_thost_ftdc_uoa_country_code_type *//* Param: t_thost_ftdc_area_code_type *//* Param: t_thost_ftdc_futures_id_type *//* Param: t_thost_ftdc_cffmc_date_type *//* Param: t_thost_ftdc_cffmc_time_type *//* Param: t_thost_ftdc_noc_id_type *//* Param: t_thost_ftdc_update_flag_type *//* Param: t_thost_ftdc_apply_operate_id_type *//* Param: t_thost_ftdc_apply_status_id_type *//* Param: t_thost_ftdc_send_method_type *//* Param: t_thost_ftdc_event_type_type *//* Param: t_thost_ftdc_event_mode_type *//* Param: t_thost_ftdc_uoa_auto_send_type *//* Param: t_thost_ftdc_query_depth_type *//* Param: t_thost_ftdc_data_center_id_type *//* Param: t_thost_ftdc_flow_id_type *//* Param: t_thost_ftdc_check_level_type *//* Param: t_thost_ftdc_check_no_type *//* Param: t_thost_ftdc_check_status_type *//* Param: t_thost_ftdc_used_status_type *//* Param: t_thost_ftdc_rate_template_name_type *//* Param: t_thost_ftdc_property_string_type *//* Param: t_thost_ftdc_bank_acount_origin_type *//* Param: t_thost_ftdc_month_bill_trade_sum_type *//* Param: t_thost_ftdc_fbt_trade_code_enum_type *//* Param: t_thost_ftdc_rate_template_id_type *//* Param: t_thost_ftdc_risk_rate_type *//* Param: t_thost_ftdc_timestamp_type *//* Param: t_thost_ftdc_investor_id_rule_name_type *//* Param: t_thost_ftdc_investor_id_rule_expr_type *//* Param: t_thost_ftdc_last_drift_type *//* Param: t_thost_ftdc_last_success_type *//* Param: t_thost_ftdc_auth_key_type *//* Param: t_thost_ftdc_serial_number_type *//* Param: t_thost_ftdc_otp_type_type *//* Param: t_thost_ftdc_otp_vendors_id_type *//* Param: t_thost_ftdc_otp_vendors_name_type *//* Param: t_thost_ftdc_otp_status_type *//* Param: t_thost_ftdc_broker_user_type_type *//* Param: t_thost_ftdc_future_type_type *//* Param: t_thost_ftdc_fund_event_type_type *//* Param: t_thost_ftdc_account_source_type_type *//* Param: t_thost_ftdc_code_source_type_type *//* Param: t_thost_ftdc_user_range_type *//* Param: t_thost_ftdc_time_span_type *//* Param: t_thost_ftdc_import_sequence_id_type *//* Param: t_thost_ftdc_by_group_type *//* Param: t_thost_ftdc_trade_sum_stat_mode_type *//* Param: t_thost_ftdc_com_type_type *//* Param: t_thost_ftdc_user_product_id_type *//* Param: t_thost_ftdc_user_product_name_type *//* Param: t_thost_ftdc_user_product_memo_type *//* Param: t_thost_ftdc_csrc_cancel_flag_type *//* Param: t_thost_ftdc_csrc_date_type *//* Param: t_thost_ftdc_csrc_investor_name_type *//* Param: t_thost_ftdc_csrc_open_investor_name_type *//* Param: t_thost_ftdc_csrc_investor_id_type *//* Param: t_thost_ftdc_csrc_identified_card_no_type *//* Param: t_thost_ftdc_csrc_client_id_type *//* Param: t_thost_ftdc_csrc_bank_flag_type *//* Param: t_thost_ftdc_csrc_bank_account_type *//* Param: t_thost_ftdc_csrc_open_name_type *//* Param: t_thost_ftdc_csrc_memo_type *//* Param: t_thost_ftdc_csrc_time_type *//* Param: t_thost_ftdc_csrc_trade_id_type *//* Param: t_thost_ftdc_csrc_exchange_inst_id_type *//* Param: t_thost_ftdc_csrc_mortgage_name_type *//* Param: t_thost_ftdc_csrc_reason_type *//* Param: t_thost_ftdc_is_settlement_type *//* Param: t_thost_ftdc_csrc_money_type *//* Param: t_thost_ftdc_csrc_price_type *//* Param: t_thost_ftdc_csrc_options_type_type *//* Param: t_thost_ftdc_csrc_strike_price_type *//* Param: t_thost_ftdc_csrc_target_product_id_type *//* Param: t_thost_ftdc_csrc_target_instr_id_type *//* Param: t_thost_ftdc_comm_model_name_type *//* Param: t_thost_ftdc_comm_model_memo_type *//* Param: t_thost_ftdc_expr_set_mode_type *//* Param: t_thost_ftdc_rate_investor_range_type *//* Param: t_thost_ftdc_agent_broker_id_type *//* Param: t_thost_ftdc_dr_identity_id_type *//* Param: t_thost_ftdc_dr_identity_name_type *//* Param: t_thost_ftdc_db_link_id_type *//* Param: t_thost_ftdc_sync_data_status_type *//* Param: t_thost_ftdc_trade_source_type *//* Param: t_thost_ftdc_flex_stat_mode_type *//* Param: t_thost_ftdc_by_investor_range_type *//* Param: t_thost_ftdc_s_risk_rate_type *//* Param: t_thost_ftdc_sequence_no_12_type *//* Param: t_thost_ftdc_property_investor_range_type *//* Param: t_thost_ftdc_file_status_type *//* Param: t_thost_ftdc_file_gen_style_type *//* Param: t_thost_ftdc_sys_oper_mode_type *//* Param: t_thost_ftdc_sys_oper_type_type *//* Param: t_thost_ftdc_csrc_data_quey_type_type *//* Param: t_thost_ftdc_freeze_status_type *//* Param: t_thost_ftdc_standard_status_type *//* Param: t_thost_ftdc_csrc_freeze_status_type *//* Param: t_thost_ftdc_right_param_type_type *//* Param: t_thost_ftdc_right_template_id_type *//* Param: t_thost_ftdc_right_template_name_type *//* Param: t_thost_ftdc_data_status_type *//* Param: t_thost_ftdc_aml_check_status_type *//* Param: t_thost_ftdc_aml_date_type_type *//* Param: t_thost_ftdc_aml_check_level_type *//* Param: t_thost_ftdc_aml_check_flow_type *//* Param: t_thost_ftdc_data_type_type *//* Param: t_thost_ftdc_export_file_type_type *//* Param: t_thost_ftdc_settle_manager_type_type *//* Param: t_thost_ftdc_settle_manager_id_type *//* Param: t_thost_ftdc_settle_manager_name_type *//* Param: t_thost_ftdc_settle_manager_level_type *//* Param: t_thost_ftdc_settle_manager_group_type *//* Param: t_thost_ftdc_check_result_memo_type *//* Param: t_thost_ftdc_function_url_type *//* Param: t_thost_ftdc_auth_info_type *//* Param: t_thost_ftdc_auth_code_type *//* Param: t_thost_ftdc_limit_use_type_type *//* Param: t_thost_ftdc_data_resource_type *//* Param: t_thost_ftdc_margin_type_type *//* Param: t_thost_ftdc_active_type_type *//* Param: t_thost_ftdc_margin_rate_type_type *//* Param: t_thost_ftdc_back_up_status_type *//* Param: t_thost_ftdc_init_settlement_type *//* Param: t_thost_ftdc_report_status_type *//* Param: t_thost_ftdc_save_status_type *//* Param: t_thost_ftdc_sett_archive_status_type *//* Param: t_thost_ftdc_ctp_type_type *//* Param: t_thost_ftdc_tool_id_type *//* Param: t_thost_ftdc_tool_name_type *//* Param: t_thost_ftdc_close_deal_type_type *//* Param: t_thost_ftdc_mortgage_fund_use_range_type *//* Param: t_thost_ftdc_currency_unit_type *//* Param: t_thost_ftdc_exchange_rate_type *//* Param: t_thost_ftdc_spec_product_type_type *//* Param: t_thost_ftdc_fund_mortgage_type_type *//* Param: t_thost_ftdc_account_settlement_param_id_type *//* Param: t_thost_ftdc_currency_name_type *//* Param: t_thost_ftdc_currency_sign_type *//* Param: t_thost_ftdc_fund_mort_direction_type *//* Param: t_thost_ftdc_business_class_type *//* Param: t_thost_ftdc_swap_source_type_type *//* Param: t_thost_ftdc_curr_ex_direction_type *//* Param: t_thost_ftdc_currency_swap_status_type *//* Param: t_thost_ftdc_curr_exch_cert_no_type *//* Param: t_thost_ftdc_batch_serial_no_type *//* Param: t_thost_ftdc_req_flag_type *//* Param: t_thost_ftdc_res_flag_type *//* Param: t_thost_ftdc_page_control_type *//* Param: t_thost_ftdc_record_count_type *//* Param: t_thost_ftdc_currency_swap_memo_type *//* Param: t_thost_ftdc_ex_status_type *//* Param: t_thost_ftdc_client_region_type *//* Param: t_thost_ftdc_work_place_type *//* Param: t_thost_ftdc_business_period_type *//* Param: t_thost_ftdc_web_site_type *//* Param: t_thost_ftdc_uoa_id_card_type_type *//* Param: t_thost_ftdc_client_mode_type *//* Param: t_thost_ftdc_investor_full_name_type *//* Param: t_thost_ftdc_uoa_broker_id_type *//* Param: t_thost_ftdc_uoa_zip_code_type *//* Param: t_thost_ftdc_uoae_mail_type *//* Param: t_thost_ftdc_old_city_type *//* Param: t_thost_ftdc_corporate_identified_card_no_type *//* Param: t_thost_ftdc_has_board_type *//* Param: t_thost_ftdc_start_mode_type *//* Param: t_thost_ftdc_template_type_type *//* Param: t_thost_ftdc_login_mode_type *//* Param: t_thost_ftdc_prompt_type_type *//* Param: t_thost_ftdc_ledger_manage_id_type *//* Param: t_thost_ftdc_invest_variety_type *//* Param: t_thost_ftdc_bank_account_type_type *//* Param: t_thost_ftdc_ledger_manage_bank_type *//* Param: t_thost_ftdc_cffex_department_name_type *//* Param: t_thost_ftdc_cffex_department_code_type *//* Param: t_thost_ftdc_has_trustee_type *//* Param: t_thost_ftdc_csrc_memo_1_type *//* Param: t_thost_ftdc_assetmgr_c_full_name_type *//* Param: t_thost_ftdc_assetmgr_approval_no_type *//* Param: t_thost_ftdc_assetmgr_mgr_name_type *//* Param: t_thost_ftdc_am_type_type *//* Param: t_thost_ftdc_csrc_am_type_type *//* Param: t_thost_ftdc_csrc_fund_io_type_type *//* Param: t_thost_ftdc_cus_account_type_type *//* Param: t_thost_ftdc_csrc_national_type *//* Param: t_thost_ftdc_csrc_sec_agent_id_type *//* Param: t_thost_ftdc_language_type_type *//* Param: t_thost_ftdc_am_account_type *//* Param: t_thost_ftdc_assetmgr_client_type_type *//* Param: t_thost_ftdc_assetmgr_type_type *//* Param: t_thost_ftdc_uom_type *//* Param: t_thost_ftdc_shfe_inst_life_phase_type *//* Param: t_thost_ftdc_shfe_product_class_type *//* Param: t_thost_ftdc_price_decimal_type *//* Param: t_thost_ftdc_in_the_money_flag_type *//* Param: t_thost_ftdc_check_instr_type_type *//* Param: t_thost_ftdc_delivery_type_type *//* Param: t_thost_ftdc_big_money_type *//* Param: t_thost_ftdc_max_margin_side_algorithm_type *//* Param: t_thost_ftdc_da_client_type_type *//* Param: t_thost_ftdc_combin_instr_id_type *//* Param: t_thost_ftdc_combin_settle_price_type *//* Param: t_thost_ftdc_dce_priority_type *//* Param: t_thost_ftdc_trade_group_id_type *//* Param: t_thost_ftdc_is_check_prepa_type *//* Param: t_thost_ftdc_uoa_assetmgr_type_type *//* Param: t_thost_ftdc_direction_en_type *//* Param: t_thost_ftdc_offset_flag_en_type *//* Param: t_thost_ftdc_hedge_flag_en_type *//* Param: t_thost_ftdc_fund_io_type_en_type *//* Param: t_thost_ftdc_fund_type_en_type *//* Param: t_thost_ftdc_fund_direction_en_type *//* Param: t_thost_ftdc_fund_mort_direction_en_type *//* Param: t_thost_ftdc_swap_business_type_type *//* Param: t_thost_ftdc_options_type_type *//* Param: t_thost_ftdc_strike_mode_type *//* Param: t_thost_ftdc_strike_type_type *//* Param: t_thost_ftdc_apply_type_type *//* Param: t_thost_ftdc_give_up_data_source_type *//* Param: t_thost_ftdc_exec_order_sys_id_type *//* Param: t_thost_ftdc_exec_result_type *//* Param: t_thost_ftdc_strike_sequence_type *//* Param: t_thost_ftdc_strike_time_type *//* Param: t_thost_ftdc_combination_type_type *//* Param: t_thost_ftdc_dce_combination_type_type *//* Param: t_thost_ftdc_option_royalty_price_type_type *//* Param: t_thost_ftdc_balance_algorithm_type *//* Param: t_thost_ftdc_action_type_type *//* Param: t_thost_ftdc_for_quote_status_type *//* Param: t_thost_ftdc_value_method_type *//* Param: t_thost_ftdc_exec_order_position_flag_type *//* Param: t_thost_ftdc_exec_order_close_flag_type *//* Param: t_thost_ftdc_product_type_type *//* Param: t_thost_ftdc_czce_upload_file_name_type *//* Param: t_thost_ftdc_dce_upload_file_name_type *//* Param: t_thost_ftdc_shfe_upload_file_name_type *//* Param: t_thost_ftdc_cffex_upload_file_name_type *//* Param: t_thost_ftdc_comb_direction_type *//* Param: t_thost_ftdc_strike_offset_type_type *//* Param: t_thost_ftdc_reserve_open_acc_stas_type *//* Param: t_thost_ftdc_login_remark_type *//* Param: t_thost_ftdc_invest_unit_id_type *//* Param: t_thost_ftdc_bulletin_id_type *//* Param: t_thost_ftdc_news_type_type *//* Param: t_thost_ftdc_news_urgency_type *//* Param: t_thost_ftdc_abstract_type *//* Param: t_thost_ftdc_come_from_type *//* Param: t_thost_ftdc_url_link_type *//* Param: t_thost_ftdc_long_individual_name_type *//* Param: t_thost_ftdc_long_fbe_bank_account_name_type *//* Param: t_thost_ftdc_date_time_type *//* Param: t_thost_ftdc_weak_password_source_type *//* Param: t_thost_ftdc_random_string_type *//* Param: t_thost_ftdc_opt_self_close_flag_type *//* Param: t_thost_ftdc_biz_type_type *//* Param: t_thost_ftdc_app_type_type *//* Param: t_thost_ftdc_app_id_type *//* Param: t_thost_ftdc_system_info_len_type *//* Param: t_thost_ftdc_additional_info_len_type *//* Param: t_thost_ftdc_client_system_info_type *//* Param: t_thost_ftdc_additional_info_type *//* Param: t_thost_ftdc_base_64_client_system_info_type *//* Param: t_thost_ftdc_base_64_additional_info_type *//* Param: t_thost_ftdc_current_auth_method_type *//* Param: t_thost_ftdc_captcha_info_len_type *//* Param: t_thost_ftdc_captcha_info_type *//* Param: t_thost_ftdc_user_text_seq_type *//* Param: t_thost_ftdc_handshake_data_type *//* Param: t_thost_ftdc_handshake_data_len_type *//* Param: t_thost_ftdc_crypto_key_version_type *//* Param: t_thost_ftdc_rsa_key_version_type *//* Param: t_thost_ftdc_software_provider_id_type *//* Param: t_thost_ftdc_collect_time_type *//* Param: t_thost_ftdc_query_freq_type *//* Param: t_thost_ftdc_response_value_type *//* Param: t_thost_ftdc_otc_trade_type_type *//* Param: t_thost_ftdc_match_type_type *//* Param: t_thost_ftdc_otc_trader_id_type *//* Param: t_thost_ftdc_risk_value_type *//* Param: t_thost_ftdc_idb_name_type *//* Param: t_thost_ftdc_discount_ratio_type *//* Param: t_thost_ftdc_auth_type_type *//* Param: t_thost_ftdc_class_type_type *//* Param: t_thost_ftdc_trading_type_type *//* Param: t_thost_ftdc_product_status_type *//* Param: t_thost_ftdc_sync_delta_status_type *//* Param: t_thost_ftdc_action_direction_type *//* Param: t_thost_ftdc_order_cancel_alg_type *//* Param: t_thost_ftdc_sync_description_type *//* Param: t_thost_ftdc_common_int_type *//* Param: t_thost_ftdc_sys_version_type *//* Param: t_thost_ftdc_open_limit_control_level_type *//* Param: t_thost_ftdc_order_freq_control_level_type *//* Param: t_thost_ftdc_enum_bool_type *//* Param: t_thost_ftdc_time_range_type *//* Param: t_thost_ftdc_delta_type *//* Param: t_thost_ftdc_spread_id_type *//* Param: t_thost_ftdc_portfolio_type *//* Param: t_thost_ftdc_portfolio_def_id_type *//* Param: t_thost_ftdc_with_draw_param_id_type *//* Param: t_thost_ftdc_with_draw_param_value_type *//* Param: t_thost_ftdc_invst_trading_right_type *//* Param: t_thost_ftdc_thost_function_code_type */
/* Generated by handle_api */
impl CThostFtdcTraderApi {
    pub fn release(&mut self) -> () {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_Release)(self as *mut CThostFtdcTraderApi)
        }
    }
    pub fn init(&mut self) -> () {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_Init)(self as *mut CThostFtdcTraderApi)
        }
    }
    pub fn join(&mut self) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_Join)(self as *mut CThostFtdcTraderApi)
        }
    }
    pub fn get_trading_day(&mut self) -> *const std::os::raw::c_char {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_GetTradingDay)(self as *mut CThostFtdcTraderApi)
        }
    }
    pub fn register_front(&mut self, psz_front_address: std::ffi::CString) -> () {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_RegisterFront)(self as *mut CThostFtdcTraderApi, psz_front_address.into_raw())
        }
    }
    pub fn register_name_server(&mut self, psz_ns_address: std::ffi::CString) -> () {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_RegisterNameServer)(self as *mut CThostFtdcTraderApi, psz_ns_address.into_raw())
        }
    }
    pub fn register_fens_user_info(&mut self, p_fens_user_info: &mut CThostFtdcFensUserInfoField) -> () {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_RegisterFensUserInfo)(self as *mut CThostFtdcTraderApi, &mut *p_fens_user_info)
        }
    }
    pub fn register_spi(&mut self, p_spi: &mut CThostFtdcTraderSpi) -> () {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_RegisterSpi)(self as *mut CThostFtdcTraderApi, &mut *p_spi)
        }
    }
    pub fn subscribe_private_topic(&mut self, n_resume_type: THOST_TE_RESUME_TYPE) -> () {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_SubscribePrivateTopic)(self as *mut CThostFtdcTraderApi, n_resume_type)
        }
    }
    pub fn subscribe_public_topic(&mut self, n_resume_type: THOST_TE_RESUME_TYPE) -> () {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_SubscribePublicTopic)(self as *mut CThostFtdcTraderApi, n_resume_type)
        }
    }
    pub fn req_authenticate(&mut self, p_req_authenticate_field: &mut CThostFtdcReqAuthenticateField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqAuthenticate)(self as *mut CThostFtdcTraderApi, &mut *p_req_authenticate_field, n_request_id)
        }
    }
    pub fn register_user_system_info(&mut self, p_user_system_info: &mut CThostFtdcUserSystemInfoField) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_RegisterUserSystemInfo)(self as *mut CThostFtdcTraderApi, &mut *p_user_system_info)
        }
    }
    pub fn submit_user_system_info(&mut self, p_user_system_info: &mut CThostFtdcUserSystemInfoField) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_SubmitUserSystemInfo)(self as *mut CThostFtdcTraderApi, &mut *p_user_system_info)
        }
    }
    pub fn req_user_login(&mut self, p_req_user_login_field: &mut CThostFtdcReqUserLoginField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqUserLogin)(self as *mut CThostFtdcTraderApi, &mut *p_req_user_login_field, n_request_id)
        }
    }
    pub fn req_user_logout(&mut self, p_user_logout: &mut CThostFtdcUserLogoutField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqUserLogout)(self as *mut CThostFtdcTraderApi, &mut *p_user_logout, n_request_id)
        }
    }
    pub fn req_user_password_update(&mut self, p_user_password_update: &mut CThostFtdcUserPasswordUpdateField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqUserPasswordUpdate)(self as *mut CThostFtdcTraderApi, &mut *p_user_password_update, n_request_id)
        }
    }
    pub fn req_trading_account_password_update(&mut self, p_trading_account_password_update: &mut CThostFtdcTradingAccountPasswordUpdateField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqTradingAccountPasswordUpdate)(self as *mut CThostFtdcTraderApi, &mut *p_trading_account_password_update, n_request_id)
        }
    }
    pub fn req_user_auth_method(&mut self, p_req_user_auth_method: &mut CThostFtdcReqUserAuthMethodField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqUserAuthMethod)(self as *mut CThostFtdcTraderApi, &mut *p_req_user_auth_method, n_request_id)
        }
    }
    pub fn req_gen_user_captcha(&mut self, p_req_gen_user_captcha: &mut CThostFtdcReqGenUserCaptchaField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqGenUserCaptcha)(self as *mut CThostFtdcTraderApi, &mut *p_req_gen_user_captcha, n_request_id)
        }
    }
    pub fn req_gen_user_text(&mut self, p_req_gen_user_text: &mut CThostFtdcReqGenUserTextField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqGenUserText)(self as *mut CThostFtdcTraderApi, &mut *p_req_gen_user_text, n_request_id)
        }
    }
    pub fn req_user_login_with_captcha(&mut self, p_req_user_login_with_captcha: &mut CThostFtdcReqUserLoginWithCaptchaField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqUserLoginWithCaptcha)(self as *mut CThostFtdcTraderApi, &mut *p_req_user_login_with_captcha, n_request_id)
        }
    }
    pub fn req_user_login_with_text(&mut self, p_req_user_login_with_text: &mut CThostFtdcReqUserLoginWithTextField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqUserLoginWithText)(self as *mut CThostFtdcTraderApi, &mut *p_req_user_login_with_text, n_request_id)
        }
    }
    pub fn req_user_login_with_otp(&mut self, p_req_user_login_with_otp: &mut CThostFtdcReqUserLoginWithOTPField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqUserLoginWithOTP)(self as *mut CThostFtdcTraderApi, &mut *p_req_user_login_with_otp, n_request_id)
        }
    }
    pub fn req_order_insert(&mut self, p_input_order: &mut CThostFtdcInputOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqOrderInsert)(self as *mut CThostFtdcTraderApi, &mut *p_input_order, n_request_id)
        }
    }
    pub fn req_parked_order_insert(&mut self, p_parked_order: &mut CThostFtdcParkedOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqParkedOrderInsert)(self as *mut CThostFtdcTraderApi, &mut *p_parked_order, n_request_id)
        }
    }
    pub fn req_parked_order_action(&mut self, p_parked_order_action: &mut CThostFtdcParkedOrderActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqParkedOrderAction)(self as *mut CThostFtdcTraderApi, &mut *p_parked_order_action, n_request_id)
        }
    }
    pub fn req_order_action(&mut self, p_input_order_action: &mut CThostFtdcInputOrderActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqOrderAction)(self as *mut CThostFtdcTraderApi, &mut *p_input_order_action, n_request_id)
        }
    }
    pub fn req_qry_max_order_volume(&mut self, p_qry_max_order_volume: &mut CThostFtdcQryMaxOrderVolumeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryMaxOrderVolume)(self as *mut CThostFtdcTraderApi, &mut *p_qry_max_order_volume, n_request_id)
        }
    }
    pub fn req_settlement_info_confirm(&mut self, p_settlement_info_confirm: &mut CThostFtdcSettlementInfoConfirmField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqSettlementInfoConfirm)(self as *mut CThostFtdcTraderApi, &mut *p_settlement_info_confirm, n_request_id)
        }
    }
    pub fn req_remove_parked_order(&mut self, p_remove_parked_order: &mut CThostFtdcRemoveParkedOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqRemoveParkedOrder)(self as *mut CThostFtdcTraderApi, &mut *p_remove_parked_order, n_request_id)
        }
    }
    pub fn req_remove_parked_order_action(&mut self, p_remove_parked_order_action: &mut CThostFtdcRemoveParkedOrderActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqRemoveParkedOrderAction)(self as *mut CThostFtdcTraderApi, &mut *p_remove_parked_order_action, n_request_id)
        }
    }
    pub fn req_exec_order_insert(&mut self, p_input_exec_order: &mut CThostFtdcInputExecOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqExecOrderInsert)(self as *mut CThostFtdcTraderApi, &mut *p_input_exec_order, n_request_id)
        }
    }
    pub fn req_exec_order_action(&mut self, p_input_exec_order_action: &mut CThostFtdcInputExecOrderActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqExecOrderAction)(self as *mut CThostFtdcTraderApi, &mut *p_input_exec_order_action, n_request_id)
        }
    }
    pub fn req_for_quote_insert(&mut self, p_input_for_quote: &mut CThostFtdcInputForQuoteField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqForQuoteInsert)(self as *mut CThostFtdcTraderApi, &mut *p_input_for_quote, n_request_id)
        }
    }
    pub fn req_quote_insert(&mut self, p_input_quote: &mut CThostFtdcInputQuoteField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQuoteInsert)(self as *mut CThostFtdcTraderApi, &mut *p_input_quote, n_request_id)
        }
    }
    pub fn req_quote_action(&mut self, p_input_quote_action: &mut CThostFtdcInputQuoteActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQuoteAction)(self as *mut CThostFtdcTraderApi, &mut *p_input_quote_action, n_request_id)
        }
    }
    pub fn req_batch_order_action(&mut self, p_input_batch_order_action: &mut CThostFtdcInputBatchOrderActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqBatchOrderAction)(self as *mut CThostFtdcTraderApi, &mut *p_input_batch_order_action, n_request_id)
        }
    }
    pub fn req_option_self_close_insert(&mut self, p_input_option_self_close: &mut CThostFtdcInputOptionSelfCloseField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqOptionSelfCloseInsert)(self as *mut CThostFtdcTraderApi, &mut *p_input_option_self_close, n_request_id)
        }
    }
    pub fn req_option_self_close_action(&mut self, p_input_option_self_close_action: &mut CThostFtdcInputOptionSelfCloseActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqOptionSelfCloseAction)(self as *mut CThostFtdcTraderApi, &mut *p_input_option_self_close_action, n_request_id)
        }
    }
    pub fn req_comb_action_insert(&mut self, p_input_comb_action: &mut CThostFtdcInputCombActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqCombActionInsert)(self as *mut CThostFtdcTraderApi, &mut *p_input_comb_action, n_request_id)
        }
    }
    pub fn req_qry_order(&mut self, p_qry_order: &mut CThostFtdcQryOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryOrder)(self as *mut CThostFtdcTraderApi, &mut *p_qry_order, n_request_id)
        }
    }
    pub fn req_qry_trade(&mut self, p_qry_trade: &mut CThostFtdcQryTradeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryTrade)(self as *mut CThostFtdcTraderApi, &mut *p_qry_trade, n_request_id)
        }
    }
    pub fn req_qry_investor_position(&mut self, p_qry_investor_position: &mut CThostFtdcQryInvestorPositionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryInvestorPosition)(self as *mut CThostFtdcTraderApi, &mut *p_qry_investor_position, n_request_id)
        }
    }
    pub fn req_qry_trading_account(&mut self, p_qry_trading_account: &mut CThostFtdcQryTradingAccountField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryTradingAccount)(self as *mut CThostFtdcTraderApi, &mut *p_qry_trading_account, n_request_id)
        }
    }
    pub fn req_qry_investor(&mut self, p_qry_investor: &mut CThostFtdcQryInvestorField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryInvestor)(self as *mut CThostFtdcTraderApi, &mut *p_qry_investor, n_request_id)
        }
    }
    pub fn req_qry_trading_code(&mut self, p_qry_trading_code: &mut CThostFtdcQryTradingCodeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryTradingCode)(self as *mut CThostFtdcTraderApi, &mut *p_qry_trading_code, n_request_id)
        }
    }
    pub fn req_qry_instrument_margin_rate(&mut self, p_qry_instrument_margin_rate: &mut CThostFtdcQryInstrumentMarginRateField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryInstrumentMarginRate)(self as *mut CThostFtdcTraderApi, &mut *p_qry_instrument_margin_rate, n_request_id)
        }
    }
    pub fn req_qry_instrument_commission_rate(&mut self, p_qry_instrument_commission_rate: &mut CThostFtdcQryInstrumentCommissionRateField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryInstrumentCommissionRate)(self as *mut CThostFtdcTraderApi, &mut *p_qry_instrument_commission_rate, n_request_id)
        }
    }
    pub fn req_qry_exchange(&mut self, p_qry_exchange: &mut CThostFtdcQryExchangeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryExchange)(self as *mut CThostFtdcTraderApi, &mut *p_qry_exchange, n_request_id)
        }
    }
    pub fn req_qry_product(&mut self, p_qry_product: &mut CThostFtdcQryProductField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryProduct)(self as *mut CThostFtdcTraderApi, &mut *p_qry_product, n_request_id)
        }
    }
    pub fn req_qry_instrument(&mut self, p_qry_instrument: &mut CThostFtdcQryInstrumentField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryInstrument)(self as *mut CThostFtdcTraderApi, &mut *p_qry_instrument, n_request_id)
        }
    }
    pub fn req_qry_depth_market_data(&mut self, p_qry_depth_market_data: &mut CThostFtdcQryDepthMarketDataField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryDepthMarketData)(self as *mut CThostFtdcTraderApi, &mut *p_qry_depth_market_data, n_request_id)
        }
    }
    pub fn req_qry_trader_offer(&mut self, p_qry_trader_offer: &mut CThostFtdcQryTraderOfferField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryTraderOffer)(self as *mut CThostFtdcTraderApi, &mut *p_qry_trader_offer, n_request_id)
        }
    }
    pub fn req_qry_settlement_info(&mut self, p_qry_settlement_info: &mut CThostFtdcQrySettlementInfoField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQrySettlementInfo)(self as *mut CThostFtdcTraderApi, &mut *p_qry_settlement_info, n_request_id)
        }
    }
    pub fn req_qry_transfer_bank(&mut self, p_qry_transfer_bank: &mut CThostFtdcQryTransferBankField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryTransferBank)(self as *mut CThostFtdcTraderApi, &mut *p_qry_transfer_bank, n_request_id)
        }
    }
    pub fn req_qry_investor_position_detail(&mut self, p_qry_investor_position_detail: &mut CThostFtdcQryInvestorPositionDetailField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryInvestorPositionDetail)(self as *mut CThostFtdcTraderApi, &mut *p_qry_investor_position_detail, n_request_id)
        }
    }
    pub fn req_qry_notice(&mut self, p_qry_notice: &mut CThostFtdcQryNoticeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryNotice)(self as *mut CThostFtdcTraderApi, &mut *p_qry_notice, n_request_id)
        }
    }
    pub fn req_qry_settlement_info_confirm(&mut self, p_qry_settlement_info_confirm: &mut CThostFtdcQrySettlementInfoConfirmField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQrySettlementInfoConfirm)(self as *mut CThostFtdcTraderApi, &mut *p_qry_settlement_info_confirm, n_request_id)
        }
    }
    pub fn req_qry_investor_position_combine_detail(&mut self, p_qry_investor_position_combine_detail: &mut CThostFtdcQryInvestorPositionCombineDetailField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryInvestorPositionCombineDetail)(self as *mut CThostFtdcTraderApi, &mut *p_qry_investor_position_combine_detail, n_request_id)
        }
    }
    pub fn req_qry_cfmmc_trading_account_key(&mut self, p_qry_cfmmc_trading_account_key: &mut CThostFtdcQryCFMMCTradingAccountKeyField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryCFMMCTradingAccountKey)(self as *mut CThostFtdcTraderApi, &mut *p_qry_cfmmc_trading_account_key, n_request_id)
        }
    }
    pub fn req_qry_e_warrant_offset(&mut self, p_qry_e_warrant_offset: &mut CThostFtdcQryEWarrantOffsetField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryEWarrantOffset)(self as *mut CThostFtdcTraderApi, &mut *p_qry_e_warrant_offset, n_request_id)
        }
    }
    pub fn req_qry_investor_product_group_margin(&mut self, p_qry_investor_product_group_margin: &mut CThostFtdcQryInvestorProductGroupMarginField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryInvestorProductGroupMargin)(self as *mut CThostFtdcTraderApi, &mut *p_qry_investor_product_group_margin, n_request_id)
        }
    }
    pub fn req_qry_exchange_margin_rate(&mut self, p_qry_exchange_margin_rate: &mut CThostFtdcQryExchangeMarginRateField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryExchangeMarginRate)(self as *mut CThostFtdcTraderApi, &mut *p_qry_exchange_margin_rate, n_request_id)
        }
    }
    pub fn req_qry_exchange_margin_rate_adjust(&mut self, p_qry_exchange_margin_rate_adjust: &mut CThostFtdcQryExchangeMarginRateAdjustField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryExchangeMarginRateAdjust)(self as *mut CThostFtdcTraderApi, &mut *p_qry_exchange_margin_rate_adjust, n_request_id)
        }
    }
    pub fn req_qry_exchange_rate(&mut self, p_qry_exchange_rate: &mut CThostFtdcQryExchangeRateField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryExchangeRate)(self as *mut CThostFtdcTraderApi, &mut *p_qry_exchange_rate, n_request_id)
        }
    }
    pub fn req_qry_sec_agent_acid_map(&mut self, p_qry_sec_agent_acid_map: &mut CThostFtdcQrySecAgentACIDMapField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQrySecAgentACIDMap)(self as *mut CThostFtdcTraderApi, &mut *p_qry_sec_agent_acid_map, n_request_id)
        }
    }
    pub fn req_qry_product_exch_rate(&mut self, p_qry_product_exch_rate: &mut CThostFtdcQryProductExchRateField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryProductExchRate)(self as *mut CThostFtdcTraderApi, &mut *p_qry_product_exch_rate, n_request_id)
        }
    }
    pub fn req_qry_product_group(&mut self, p_qry_product_group: &mut CThostFtdcQryProductGroupField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryProductGroup)(self as *mut CThostFtdcTraderApi, &mut *p_qry_product_group, n_request_id)
        }
    }
    pub fn req_qry_mm_instrument_commission_rate(&mut self, p_qry_mm_instrument_commission_rate: &mut CThostFtdcQryMMInstrumentCommissionRateField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryMMInstrumentCommissionRate)(self as *mut CThostFtdcTraderApi, &mut *p_qry_mm_instrument_commission_rate, n_request_id)
        }
    }
    pub fn req_qry_mm_option_instr_comm_rate(&mut self, p_qry_mm_option_instr_comm_rate: &mut CThostFtdcQryMMOptionInstrCommRateField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryMMOptionInstrCommRate)(self as *mut CThostFtdcTraderApi, &mut *p_qry_mm_option_instr_comm_rate, n_request_id)
        }
    }
    pub fn req_qry_instrument_order_comm_rate(&mut self, p_qry_instrument_order_comm_rate: &mut CThostFtdcQryInstrumentOrderCommRateField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryInstrumentOrderCommRate)(self as *mut CThostFtdcTraderApi, &mut *p_qry_instrument_order_comm_rate, n_request_id)
        }
    }
    pub fn req_qry_sec_agent_trading_account(&mut self, p_qry_trading_account: &mut CThostFtdcQryTradingAccountField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQrySecAgentTradingAccount)(self as *mut CThostFtdcTraderApi, &mut *p_qry_trading_account, n_request_id)
        }
    }
    pub fn req_qry_sec_agent_check_mode(&mut self, p_qry_sec_agent_check_mode: &mut CThostFtdcQrySecAgentCheckModeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQrySecAgentCheckMode)(self as *mut CThostFtdcTraderApi, &mut *p_qry_sec_agent_check_mode, n_request_id)
        }
    }
    pub fn req_qry_sec_agent_trade_info(&mut self, p_qry_sec_agent_trade_info: &mut CThostFtdcQrySecAgentTradeInfoField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQrySecAgentTradeInfo)(self as *mut CThostFtdcTraderApi, &mut *p_qry_sec_agent_trade_info, n_request_id)
        }
    }
    pub fn req_qry_option_instr_trade_cost(&mut self, p_qry_option_instr_trade_cost: &mut CThostFtdcQryOptionInstrTradeCostField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryOptionInstrTradeCost)(self as *mut CThostFtdcTraderApi, &mut *p_qry_option_instr_trade_cost, n_request_id)
        }
    }
    pub fn req_qry_option_instr_comm_rate(&mut self, p_qry_option_instr_comm_rate: &mut CThostFtdcQryOptionInstrCommRateField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryOptionInstrCommRate)(self as *mut CThostFtdcTraderApi, &mut *p_qry_option_instr_comm_rate, n_request_id)
        }
    }
    pub fn req_qry_exec_order(&mut self, p_qry_exec_order: &mut CThostFtdcQryExecOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryExecOrder)(self as *mut CThostFtdcTraderApi, &mut *p_qry_exec_order, n_request_id)
        }
    }
    pub fn req_qry_for_quote(&mut self, p_qry_for_quote: &mut CThostFtdcQryForQuoteField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryForQuote)(self as *mut CThostFtdcTraderApi, &mut *p_qry_for_quote, n_request_id)
        }
    }
    pub fn req_qry_quote(&mut self, p_qry_quote: &mut CThostFtdcQryQuoteField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryQuote)(self as *mut CThostFtdcTraderApi, &mut *p_qry_quote, n_request_id)
        }
    }
    pub fn req_qry_option_self_close(&mut self, p_qry_option_self_close: &mut CThostFtdcQryOptionSelfCloseField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryOptionSelfClose)(self as *mut CThostFtdcTraderApi, &mut *p_qry_option_self_close, n_request_id)
        }
    }
    pub fn req_qry_invest_unit(&mut self, p_qry_invest_unit: &mut CThostFtdcQryInvestUnitField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryInvestUnit)(self as *mut CThostFtdcTraderApi, &mut *p_qry_invest_unit, n_request_id)
        }
    }
    pub fn req_qry_comb_instrument_guard(&mut self, p_qry_comb_instrument_guard: &mut CThostFtdcQryCombInstrumentGuardField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryCombInstrumentGuard)(self as *mut CThostFtdcTraderApi, &mut *p_qry_comb_instrument_guard, n_request_id)
        }
    }
    pub fn req_qry_comb_action(&mut self, p_qry_comb_action: &mut CThostFtdcQryCombActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryCombAction)(self as *mut CThostFtdcTraderApi, &mut *p_qry_comb_action, n_request_id)
        }
    }
    pub fn req_qry_transfer_serial(&mut self, p_qry_transfer_serial: &mut CThostFtdcQryTransferSerialField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryTransferSerial)(self as *mut CThostFtdcTraderApi, &mut *p_qry_transfer_serial, n_request_id)
        }
    }
    pub fn req_qry_accountregister(&mut self, p_qry_accountregister: &mut CThostFtdcQryAccountregisterField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryAccountregister)(self as *mut CThostFtdcTraderApi, &mut *p_qry_accountregister, n_request_id)
        }
    }
    pub fn req_qry_contract_bank(&mut self, p_qry_contract_bank: &mut CThostFtdcQryContractBankField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryContractBank)(self as *mut CThostFtdcTraderApi, &mut *p_qry_contract_bank, n_request_id)
        }
    }
    pub fn req_qry_parked_order(&mut self, p_qry_parked_order: &mut CThostFtdcQryParkedOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryParkedOrder)(self as *mut CThostFtdcTraderApi, &mut *p_qry_parked_order, n_request_id)
        }
    }
    pub fn req_qry_parked_order_action(&mut self, p_qry_parked_order_action: &mut CThostFtdcQryParkedOrderActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryParkedOrderAction)(self as *mut CThostFtdcTraderApi, &mut *p_qry_parked_order_action, n_request_id)
        }
    }
    pub fn req_qry_trading_notice(&mut self, p_qry_trading_notice: &mut CThostFtdcQryTradingNoticeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryTradingNotice)(self as *mut CThostFtdcTraderApi, &mut *p_qry_trading_notice, n_request_id)
        }
    }
    pub fn req_qry_broker_trading_params(&mut self, p_qry_broker_trading_params: &mut CThostFtdcQryBrokerTradingParamsField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryBrokerTradingParams)(self as *mut CThostFtdcTraderApi, &mut *p_qry_broker_trading_params, n_request_id)
        }
    }
    pub fn req_qry_broker_trading_algos(&mut self, p_qry_broker_trading_algos: &mut CThostFtdcQryBrokerTradingAlgosField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryBrokerTradingAlgos)(self as *mut CThostFtdcTraderApi, &mut *p_qry_broker_trading_algos, n_request_id)
        }
    }
    pub fn req_query_cfmmc_trading_account_token(&mut self, p_query_cfmmc_trading_account_token: &mut CThostFtdcQueryCFMMCTradingAccountTokenField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQueryCFMMCTradingAccountToken)(self as *mut CThostFtdcTraderApi, &mut *p_query_cfmmc_trading_account_token, n_request_id)
        }
    }
    pub fn req_from_bank_to_future_by_future(&mut self, p_req_transfer: &mut CThostFtdcReqTransferField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqFromBankToFutureByFuture)(self as *mut CThostFtdcTraderApi, &mut *p_req_transfer, n_request_id)
        }
    }
    pub fn req_from_future_to_bank_by_future(&mut self, p_req_transfer: &mut CThostFtdcReqTransferField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqFromFutureToBankByFuture)(self as *mut CThostFtdcTraderApi, &mut *p_req_transfer, n_request_id)
        }
    }
    pub fn req_query_bank_account_money_by_future(&mut self, p_req_query_account: &mut CThostFtdcReqQueryAccountField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQueryBankAccountMoneyByFuture)(self as *mut CThostFtdcTraderApi, &mut *p_req_query_account, n_request_id)
        }
    }
    pub fn req_qry_classified_instrument(&mut self, p_qry_classified_instrument: &mut CThostFtdcQryClassifiedInstrumentField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryClassifiedInstrument)(self as *mut CThostFtdcTraderApi, &mut *p_qry_classified_instrument, n_request_id)
        }
    }
    pub fn req_qry_comb_promotion_param(&mut self, p_qry_comb_promotion_param: &mut CThostFtdcQryCombPromotionParamField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryCombPromotionParam)(self as *mut CThostFtdcTraderApi, &mut *p_qry_comb_promotion_param, n_request_id)
        }
    }
    pub fn req_qry_risk_settle_invst_position(&mut self, p_qry_risk_settle_invst_position: &mut CThostFtdcQryRiskSettleInvstPositionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryRiskSettleInvstPosition)(self as *mut CThostFtdcTraderApi, &mut *p_qry_risk_settle_invst_position, n_request_id)
        }
    }
    pub fn req_qry_risk_settle_product_status(&mut self, p_qry_risk_settle_product_status: &mut CThostFtdcQryRiskSettleProductStatusField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryRiskSettleProductStatus)(self as *mut CThostFtdcTraderApi, &mut *p_qry_risk_settle_product_status, n_request_id)
        }
    }
    pub fn req_qry_spbm_future_parameter(&mut self, p_qry_spbm_future_parameter: &mut CThostFtdcQrySPBMFutureParameterField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQrySPBMFutureParameter)(self as *mut CThostFtdcTraderApi, &mut *p_qry_spbm_future_parameter, n_request_id)
        }
    }
    pub fn req_qry_spbm_option_parameter(&mut self, p_qry_spbm_option_parameter: &mut CThostFtdcQrySPBMOptionParameterField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQrySPBMOptionParameter)(self as *mut CThostFtdcTraderApi, &mut *p_qry_spbm_option_parameter, n_request_id)
        }
    }
    pub fn req_qry_spbm_intra_parameter(&mut self, p_qry_spbm_intra_parameter: &mut CThostFtdcQrySPBMIntraParameterField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQrySPBMIntraParameter)(self as *mut CThostFtdcTraderApi, &mut *p_qry_spbm_intra_parameter, n_request_id)
        }
    }
    pub fn req_qry_spbm_inter_parameter(&mut self, p_qry_spbm_inter_parameter: &mut CThostFtdcQrySPBMInterParameterField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQrySPBMInterParameter)(self as *mut CThostFtdcTraderApi, &mut *p_qry_spbm_inter_parameter, n_request_id)
        }
    }
    pub fn req_qry_spbm_portf_definition(&mut self, p_qry_spbm_portf_definition: &mut CThostFtdcQrySPBMPortfDefinitionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQrySPBMPortfDefinition)(self as *mut CThostFtdcTraderApi, &mut *p_qry_spbm_portf_definition, n_request_id)
        }
    }
    pub fn req_qry_spbm_investor_portf_def(&mut self, p_qry_spbm_investor_portf_def: &mut CThostFtdcQrySPBMInvestorPortfDefField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQrySPBMInvestorPortfDef)(self as *mut CThostFtdcTraderApi, &mut *p_qry_spbm_investor_portf_def, n_request_id)
        }
    }
    pub fn req_qry_investor_portf_margin_ratio(&mut self, p_qry_investor_portf_margin_ratio: &mut CThostFtdcQryInvestorPortfMarginRatioField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryInvestorPortfMarginRatio)(self as *mut CThostFtdcTraderApi, &mut *p_qry_investor_portf_margin_ratio, n_request_id)
        }
    }
    pub fn req_qry_investor_prod_spbm_detail(&mut self, p_qry_investor_prod_spbm_detail: &mut CThostFtdcQryInvestorProdSPBMDetailField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
        unsafe {
            ((*(*self).vtable_).CThostFtdcTraderApi_ReqQryInvestorProdSPBMDetail)(self as *mut CThostFtdcTraderApi, &mut *p_qry_investor_prod_spbm_detail, n_request_id)
        }
    }

    /// encapsulate the raw pointer within CThostFtdcTraderApi
    pub unsafe fn from_raw(api_ptr: *mut CThostFtdcTraderApi) -> Box<CThostFtdcTraderApi> {
        // Ensure the pointer is not null
        if api_ptr.is_null() {
            panic!("CThostFtdcTraderApi pointer is null");
        }

        // Dereference the pointer to obtain CThostFtdcTraderApi and return it
        // This assumes that CThostFtdcTraderApi can be directly obtained from the pointer
        // Adjust based on your actual struct layout
        Box::from_raw(api_ptr)
    }
}
unsafe impl Send for CThostFtdcTraderApi {}
