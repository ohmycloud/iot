#![feature(let_chains)]
#![feature(option_zip)]
#![allow(unused)]
#![allow(unused_imports)]

use iot_27930::message::frame::*;
use iot_common::*;
use iot_27930::message::{Message, Frame};

#[test]
fn test_bcl() {
    let hex = "686804470500281016001005E70703010C2E30DE0E110D011A";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
       message.len() > 19
    {
        if let Frame::BclMessage(BCL {
            bcl_demand_volt, 
            bcl_demand_current, 
            bcl_charge_mode
        }) = Frame::new(&Message::new(&message)) {
            assert_eq!(bcl_demand_volt, Some(380.6));
            assert_eq!(bcl_demand_current, Some(-65.5));
            assert_eq!(bcl_charge_mode, Some(1));
        }
    }
}

#[test]
fn test_bcp() {
    let hex = "68680447050028061E000E00E70703010D2C117001B80BA201F30E731E02520DB0";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
       message.len() > 19 {
        if let Frame::BcpMessage(BCP {
            bcp_start_soc,
            bcp_start_volt,
            bcp_max_single_volt,
            bcp_max_current,
            bcp_max_energy,
            bcp_max_volt,
            bcp_max_temp
        }) = Frame::new(&Message::new(&message)) {
            assert_eq!(bcp_start_soc, Some(54.2));
            assert_eq!(bcp_start_volt, Some(341.0));
            assert_eq!(bcp_max_single_volt, Some(2.56));
            assert_eq!(bcp_max_current, Some(-100.0));
            assert_eq!(bcp_max_energy, Some(41.8));
            assert_eq!(bcp_max_volt, Some(382.7));
            assert_eq!(bcp_max_temp, Some(65));
        }
    }
}

#[test]
fn test_bcs() {
    let hex = "68680447050028111A001005E70703010C2E30B60D180D53112B3F00CA";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
       message.len() > 19
    {
        if let Frame::BcsMessage(BCS {
            bcs_volt_measure,
            bcs_current_measure,
            bcs_max_single_volt,
            bcs_max_single_volt_group,
            bcs_soc,
            bcs_remain_time
        }) = Frame::new(&Message::new(&message)) {
            assert_eq!(bcs_volt_measure, Some(351.0));
            assert_eq!(bcs_current_measure, Some(735.2));
            assert_eq!(bcs_max_single_volt, Some(44.35));
            assert_eq!(bcs_max_single_volt_group, Some(1));
            assert_eq!(bcs_soc, Some(43));
            assert_eq!(bcs_remain_time, Some(63));
        }
    }
}

#[test]
fn test_bem() {
    let hex = "686804820600281E15000000E70703010E0719F1F0F0FCD4";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
       message.len() > 19
    {
        if let Frame::BemMessage(BEM { bem_error }) = Frame::new(&Message::new(&message)) {
            assert_eq!(bem_error, Some(4243648753));
        }        
    }
}

#[test]
fn test_bhm() {
    let hex = "686804840000272713000000E707030115313B6612D4";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
       message.len() > 19
    {
        if let Frame::BhmMessage((BHM { bhm_max_charge_volt })) = Frame::new(&Message::new(&message)) {
            assert_eq!( bhm_max_charge_volt, Some(471.0));
        } 
    }
}

#[test]
fn test_bmt() {
    let hex = "68680482060028161900D805E70703010500344747484849484A472B";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
       message.len() > 19
    {
        if let Frame::BmtMessage(BMT { bmt_single_temp }) = Frame::new(&Message::new(&message)) {
            assert_eq!(bmt_single_temp, Some(String::from("21,21,22,22,23,22,24,21")));
        }
    }
}

#[test]
fn test_bmv() {
    let hex = "68680482060028155B00D811E7070301053404A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A00181";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
       message.len() > 19
    {
        if let Frame::BmvMessage(BMV { bmv_single_volt }) = Frame::new(&Message::new(&message)) {
            assert_eq!(bmv_single_volt, Some(String::from("4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16")));
        }        
    }
}

#[test]
fn test_brm() {
    let hex = "686804470500280242000E00E70703010D2C1001010003E204140D4341544CFFFFFFFFFFFFFFFFFFFFFFFF383434303530424B30544754533338524CFFFFFFFFFFFFFFFF5E";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
       message.len() > 19
    {
        if let Frame::BrmMessage(BRM {
            brm_version,
            brm_battery_type,
            brm_rated_cap,
            brm_rated_volt,
            brm_manufacturer,
            brm_bp_id,
            brm_bp_date,
            brm_bp_charge_count,
            brm_equity_type,
            brm_vin,
            brm_bms_version,
            brm_bms_version_date,
            brm_bms_version_no
        }) = Frame::new(&Message::new(&message)) {
            assert_eq!(brm_version,  Some("V1.1".to_string()));
            assert_eq!(brm_battery_type, Some(3));
            assert_eq!(brm_rated_cap, Some(125.0));
            assert_eq!(brm_rated_volt, Some(334.80002));
            assert_eq!(brm_manufacturer, Some("CATL".to_string()));
            assert_eq!(brm_bp_date, Some("2240-255-255".to_string()));
            assert_eq!(brm_bp_charge_count, Some(16777215));
            assert_eq!(brm_equity_type, Some(255));
            assert_eq!(brm_vin, Some("LR83STGT0KB050448".to_string()));
            assert_eq!(brm_bms_version_no, Some(255));
            assert_eq!(brm_bms_version_date, Some("65535-255-255".to_string()));
            assert_eq!(brm_bms_version, Some(16777215));
        }
    }
}

#[test]
fn test_bro() {
    let hex = "686804470500280912000F00E70703010D2C11AADE";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
       message.len() > 19
    {
        if let Frame::BroMessage(BRO { bro_bms_status }) = Frame::new(&Message::new(&message)) {
            assert_eq!(bro_bms_status, Some(170));
        }
    }
}

#[test]
fn test_bsd() {
    let hex = "686804820600281C1800DB0DE7070301030A0C61A901AA01474B23";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
       message.len() > 19
    {
        if let Frame::BsdMessage(BSD {
            bsd_stop_soc,
            bsd_min_single_volt,
            bsd_max_single_volt,
            bsd_min_temp,
            bsd_max_temp
        }) = Frame::new(&Message::new(&message)) {
            assert_eq!(bsd_stop_soc, Some(97));
            assert_eq!(bsd_min_single_volt, Some(433.61));
            assert_eq!(bsd_max_single_volt, Some(435.21));
            assert_eq!(bsd_min_temp, Some(21));
            assert_eq!(bsd_max_temp, Some(25));
        }
    }
}

#[test]
fn test_bsm() {
    let hex = "686804470500281318004800E70703010D2D0E5D5203400100D0D8";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
       message.len() > 19
    {
        if let Frame::BsmMessage(BSM {
            bsm_max_single_volt_no,
            bsm_max_temp,
            bsm_max_temp_no,
            bsm_min_temp,
            bsm_min_temp_no,
            bsm_single_volt_status,
            bsm_soc_status,
            bsm_current_status,
            bsm_temp_status,
            bsm_insulate_status,
            bsm_connect_status,
            bsm_charge_status
        }) = Frame::new(&Message::new(&message)) {
            assert_eq!(bsm_max_single_volt_no, Some(94));
            assert_eq!(bsm_max_temp, Some(32));
            assert_eq!(bsm_max_temp_no, Some(4));
            assert_eq!(bsm_min_temp, Some(14));
            assert_eq!(bsm_min_temp_no, Some(2));
            assert_eq!(bsm_single_volt_status, Some(0));
            assert_eq!(bsm_soc_status, Some(0));
            assert_eq!(bsm_current_status, Some(0));
            assert_eq!(bsm_temp_status, Some(0));
            assert_eq!(bsm_insulate_status, Some(0));
            assert_eq!(bsm_connect_status, Some(0));
            assert_eq!(bsm_charge_status, Some(1));
        }        
    }
}

#[test]
fn test_bst() {
    let hex = "68680482060028191500F30BE707030102061D000000F0E7";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
       message.len() > 19
    {
        if let Frame::BstMessage(BST {
            bst_insulation_status,
            bst_conn_over_temp_status,
            bst_bms_over_temp_status,
            bst_charge_conn_status,
            bst_battery_pack_temp_status,
            bst_high_volt_relay_status,
            bst_charging_port_volt_status,
            bst_other_status,
            bst_current_status,
            bst_volt_status
        }) = Frame::new(&Message::new(&message)) {
            assert_eq!(bst_insulation_status, Some(0));
            assert_eq!(bst_conn_over_temp_status, Some(0));
            assert_eq!(bst_bms_over_temp_status, Some(0));
            assert_eq!(bst_charge_conn_status, Some(0));
            assert_eq!(bst_battery_pack_temp_status, Some(0));
            assert_eq!(bst_high_volt_relay_status, Some(0));
            assert_eq!(bst_charging_port_volt_status, Some(0));
            assert_eq!(bst_other_status, Some(0));
            assert_eq!(bst_current_status, Some(0));
            assert_eq!(bst_volt_status, Some(0));
        }
    }
}

#[test]
fn test_ccs() {
    let hex = "686804470500281218004800E70703010D2D0EAA0D570D0100FD3D";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
       message.len() > 19
    {
        if let Frame::CcsMessage(CCS {
            ccs_out_volt,
            ccs_out_current,
            ccs_charge_time,
            ccs_charge_status
        }) = Frame::new(&Message::new(&message)) {
            assert_eq!(ccs_out_volt, Some(349.80002));
            assert_eq!(ccs_out_current, Some(-58.5));
            assert_eq!(ccs_charge_time, Some(1));
            assert_eq!(ccs_charge_status, Some(1));
        }
    }
}

#[test]
fn test_cem() {
    let hex = "686804840000271F15000200E70703010F1623FCF0C0FDC8";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
       message.len() > 19
    {
        if let Frame::CemMessage(CEM { cem_error }) = Frame::new(&Message::new(&message)) {
            assert_eq!(cem_error,  Some(4257280252));
        }
    }
}

#[test]
fn test_cml() {
    let hex = "686804470500280819000E00E70703010D2C114C1DE8038C0A960F72";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
       message.len() > 19
    {
        if let Frame::CmlMessage(CML {
            cml_max_out_volt,
            cml_min_out_volt,
            cml_max_out_current,
            cml_min_out_current
        }) = Frame::new(&Message::new(&message)) {
            assert_eq!(cml_max_out_volt, Some(750.0));
            assert_eq!(cml_min_out_volt, Some(100.0));
            assert_eq!(cml_max_out_current, Some(-130.0));
            assert_eq!(cml_min_out_current, Some(-1.0));
        }
    }
}

#[test]
fn test_crm() {
    let hex = "686804470500280119000E00E70703010D2C11AA000000003132331C";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
       message.len() > 19
    {
        if let Frame::CrmMessage( CRM {
            crm_recognizable,
            crm_version,
            crm_charge_pile_no
        }) = Frame::new(&Message::new(&message)) {
            assert_eq!(crm_recognizable, Some(-86));
            assert_eq!(crm_version, Some("123".to_string()));
            assert_eq!(crm_charge_pile_no, None);
        }
    }
}

#[test]
fn test_cro() {
    let hex = "686804470500280A12001000E70703010D2C12AA8B";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
       message.len() > 19
    {
        if let Frame::CroMessage(CRO { cro_bms_status }) = Frame::new(&Message::new(&message)) {
            assert_eq!(cro_bms_status, Some(170));
        }     
    }
}

#[test]
fn test_csd() {
    let hex = "686804820600281D1900F40BE707030102061D320010010100000044";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
       message.len() > 19
    {
        if let Frame::CsdMessage(CSD {
            csd_charge_time,
            csd_out_energy,
            csd_charge_pile_no
        }) = Frame::new(&Message::new(&message)) {
            assert_eq!(csd_charge_time, Some(50.0));
            assert_eq!(csd_out_energy, Some(409.6));
            assert_eq!(csd_charge_pile_no, Some(1));
        }
    }
}

#[test]
fn test_cst() {
    let hex = "686804820600281A1500440CE70703010708260400F0FA48";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
       message.len() > 19
    {
        if let Frame::CstMessage(CST {
            cst_temp_status,
            cst_connector_status,
            cst_internal_temp_status,
            cst_power_status,
            cst_emergency_stop_status,
            cst_other_status,
            cst_current_status,
            cst_volt_status
        }) = Frame::new(&Message::new(&message)) {
            assert_eq!(cst_temp_status, Some(0));
            assert_eq!(cst_connector_status, Some(0));
            assert_eq!(cst_internal_temp_status, Some(0));
            assert_eq!(cst_power_status, Some(0));
            assert_eq!(cst_emergency_stop_status, Some(0));
            assert_eq!(cst_other_status, Some(0));
            assert_eq!(cst_current_status, Some(2));
            assert_eq!(cst_volt_status, Some(2));
        }
    }        
}

#[test]
fn test_cts() {
    let hex = "686804470500280718000F00E70703010E151B2521140103232077";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
       message.len() > 19 {
        if let Frame::CtsMessage(CTS { cts_time }) = Frame::new(&Message::new(&message)) {
            assert_eq!(cts_time, Some("2020-03-01 14:21:25".to_string()));
        }
    }
}