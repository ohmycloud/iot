#![feature(let_chains)]
#![feature(option_zip)]
#![allow(unused)]
#![allow(unused_imports)]

use iot_common::*;
use std::str;
use iot_27930::message::Message;
use iot_27930::message::Frame;

#[test]
fn test_bcl() {
    let hex = "686804470500281016001005E70703010C2E30DE0E110D011A";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
        message.len() > 19
    {
        let m = Message::new(&message);

        let bcl_demand_volt =bytes_to_f32(&m.data[0..1], &m.data[1..2], 256, 0.1, 0.0);
        assert_eq!(bcl_demand_volt, Some(380.6));

        let bcl_demand_current = bytes_to_f32(&m.data[2..3], &m.data[3..4], 256, 0.1, 400.0);
        assert_eq!(bcl_demand_current, Some(-65.5));

        let bcl_charge_mode = bytes_to_uint8(&m.data[4..5]);
        assert_eq!(bcl_charge_mode, Some(1));
    }
}

#[test]
fn test_bcp() {
    let hex = "68680447050028061E000E00E70703010D2C117001B80BA201F30E731E02520DB0";
    let byte_sign = 0xff;
    let message = hex::decode(hex);

    if let Ok(message) = message &&
        message.len() > 19 {

        let m = Message::new(&message);
        let bcp_start_soc = bytes_to_f32(&m.data[9..10], &m.data[10..11], 256, 0.1, 0.0);
        assert_eq!(bcp_start_soc, Some(54.2));

        let bcp_start_volt = bytes_to_f32(&m.data[11..12], &m.data[12..13], 256, 0.1, 0.0);
        assert_eq!(bcp_start_volt, Some(341.0));

        let bcp_max_single_volt = bytes_to_uint8(&m.data[1..2]).map(|x| x as u16 * 256).map(|x| x as f32 * 0.01);
        assert_eq!(bcp_max_single_volt, Some(2.56));

        let bcp_max_current = bytes_to_f32(&m.data[2..3], &m.data[3..4], 256, 0.1, 400.0);
        assert_eq!(bcp_max_current, Some(-100.0));

        let bcp_max_energy = bytes_to_f32(&m.data[4..5], &m.data[5..6], 256, 0.1, 0.0);
        assert_eq!(bcp_max_energy, Some(41.8));

        let bcp_max_volt = bytes_to_f32(&m.data[6..7], &m.data[7..8], 256, 0.1, 0.0);
        assert_eq!(bcp_max_volt, Some(382.7));

        let bcp_max_temp = bytes_to_uint8(&m.data[8..9]).map(|x| x - 50);
        assert_eq!(bcp_max_temp, Some(65));
    }
}

#[test]
fn test_bcs() {
    let hex = "68680447050028111A001005E70703010C2E30B60D180D53112B3F00CA";

    let message = hex::decode(hex);
    if let Ok(message) = message &&
        message.len() > 19
    {
        let m = Message::new(&message);

        let bcs_volt_measure = bytes_to_f32(&m.data[0..1], &m.data[1..2], 256, 0.1, 0.0);
        assert_eq!(bcs_volt_measure, Some(351.0));

        let bcs_current_measure = bytes_to_f32(&m.data[2..3], &m.data[3..4], 256, 0.1, -400.0);
        assert_eq!(bcs_current_measure, Some(735.2));

        let bcs_max_single_volt = bytes_to_f32(&m.data[4..5], &m.data[5..6], 256, 0.01, 0.0);
        assert_eq!(bcs_max_single_volt, Some(44.35));

        let bcs_max_single_volt_group = bytes_to_uint8(&m.data[5..6]).map(|x| x / 16);
        assert_eq!(bcs_max_single_volt_group, Some(1));

        let bcs_soc = bytes_to_uint8(&m.data[6..7]);
        assert_eq!(bcs_soc, Some(43));

        let bcs_remain_time = two_bytes_to_u16(&m.data[7..8], &m.data[8..9], 256);
        assert_eq!(bcs_remain_time, Some(63));
    }
}

#[test]
fn test_bem() {
    let hex = "686804820600281E15000000E70703010E0719F1F0F0FCD4";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
        message.len() > 19
    {
        let m = Message::new(&message);

        let bem_error = bytes_to_uint8(&m.data[0..1])
            .zip_with(bytes_to_uint8(&m.data[1..2]).map(|x| x as u32 * 256), |x, y| x as u32 + y)
            .zip_with(bytes_to_uint8(&m.data[2..3]).map(|x| x as u32 * 65536), |x, y| x + y)
            .zip_with(bytes_to_uint8(&m.data[3..4]).map(|x| x as u32 * 16777216), |x, y| x + y);

        assert_eq!(bem_error, Some(4243648753));
    }
}

#[test]
fn test_bhm() {
    let hex = "686804840000272713000000E707030115313B6612D4";

    let message = hex::decode(hex);
    if let Ok(message) = message &&
        message.len() > 19
    {
        let m = Message::new(&message);
        let bhm_max_charge_volt = bytes_to_f32(&m.data[0..1], &m.data[1..2], 256, 0.1, 0.0);
        assert_eq!( bhm_max_charge_volt, Some(471.0));
    }
}

#[test]
fn test_bmt() {
    let hex = "68680482060028161900D805E70703010500344747484849484A472B";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
        message.len() > 19
    {
        let m = Message::new(&message);
        let bmt_single_temp = (0..(m.data.len() - 1)).map(|i| bytes_to_uint8(&m.data[i..i+1]).map(|x| x - 50 ))
            .map(|x| if x.is_some() { x.unwrap().to_string() } else { "".to_string() } )
            .collect::<Vec<String>>()
            .join(",");
        assert_eq!(bmt_single_temp, String::from("21,21,22,22,23,22,24,21"));
    }
}

#[test]
fn test_bmv() {
    let hex = "68680482060028155B00D811E7070301053404A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A00181";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
        message.len() > 19
    {
        let m = Message::new(&message);
        let bmv_single_volt = (0..((m.data.len() - 1)/2)).map(|i| bytes_to_uint8(&m.data[i*2..i*2+1])
            .zip_with(bytes_to_uint8(&m.data[i*2+1..i*2+2])
                          .map(|x| x as u16 * 256), |x, y| (x as u16 + y) as f32 * 0.01))
            .map(|x| if x.is_some() { x.unwrap().to_string() } else { "".to_string() } )
            .collect::<Vec<String>>()
            .join(",");
        assert_eq!(bmv_single_volt, String::from("4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16,4.16"));
    }
}

#[test]
fn test_brm() {
    let hex = "686804470500280242000E00E70703010D2C1001010003E204140D4341544CFFFFFFFFFFFFFFFFFFFFFFFF383434303530424B30544754533338524CFFFFFFFFFFFFFFFF5E";
    let byte_sign = 0xff;
    let message = hex::decode(hex);
    if let Ok(message) = message &&
        message.len() > 19
    {
        let m = Message::new(&message);

        let brm_version = bytes_to_uint8(&m.data[0..1])
            .zip_with(bytes_to_uint16(&m.data[1..3]), |x, y| format!("V{}.{}", x & byte_sign, y & 0xffff));
        assert_eq!(brm_version,  Some("V1.1".to_string()));

        let brm_battery_type = bytes_to_int8(&m.data[3..4]);
        assert_eq!(brm_battery_type, Some(3));

        let brm_rated_cap = bytes_to_f32(&m.data[4..5], &m.data[5..6], 256, 0.1, 0.0);
        assert_eq!(brm_rated_cap, Some(125.0));

        let brm_rated_volt = bytes_to_f32(&m.data[6..7], &m.data[7..8], 256, 0.1, 0.0);
        assert_eq!(brm_rated_volt, Some(334.80002));

        // brm_manufacturer
        let brm_manufacturer = str::from_utf8(&m.data[8..12]).ok().map(|x| x.to_string());
        assert_eq!(brm_manufacturer, Some("CATL".to_string()));

        // brm_bpID
        let bp1 = bytes_to_int8(&m.data[12..13]);
        let bp2 = bytes_to_int8(&m.data[13..14]);
        let bp3 = bytes_to_int8(&m.data[14..15]);
        let bp4 = bytes_to_int8(&m.data[15..16]);
        if let Some(w) = bp1 {
            println!("{:?}", w );
        }

        let brm_bp_date = bytes_to_uint8(&m.data[16..17]).map(|x| x as u16 + 1985)
            .zip_with(bytes_to_uint8(&m.data[17..18]), |x, y| format!("{}-{}", x, y))
            .zip_with(bytes_to_uint8(&m.data[18..19]), |x, y| format!("{}-{}", x, y));
        assert_eq!(brm_bp_date, Some("2240-255-255".to_string()));


        let brm_bp_charge_count = bytes_to_uint8(&m.data[19..20])
            .zip_with(bytes_to_uint8(&m.data[20..21]).map(|x| x as u16 * 256), |x, y| x as u32 + y as u32)
            .zip_with(bytes_to_uint8(&m.data[21..22]).map(|x| x as u32 * 65536), |x, y| x + y);

        assert_eq!(brm_bp_charge_count, Some(16777215));

        let brm_equity_type = bytes_to_uint8(&m.data[22..23]);
        assert_eq!(brm_equity_type, Some(255));

        // brm_vin
        let brm_vin = if let Ok(brm_vin) = str::from_utf8(&m.data[24..41]) {
            if brm_vin.starts_with("L") {
                brm_vin.to_string()
            } else if brm_vin.ends_with("L") {
                brm_vin.chars().rev().collect::<String>()
            } else {
                "".to_string()
            }
        } else { "".to_string() };

        assert_eq!(brm_vin, "LR83STGT0KB050448".to_string());

        let brm_bms_version_no = bytes_to_uint8(&m.data[42..43]);
        assert_eq!(brm_bms_version_no, Some(255));

        let brm_bms_version_date = bytes_to_uint8(&m.data[44..45])
            .zip_with(bytes_to_uint8(&m.data[45..46]).map(|x| x as u16 * 256), |x, y| x as u16 + y)
            .zip_with(bytes_to_uint8(&m.data[43..44]), |x, y| format!("{}-{}", x, y))
            .zip_with(bytes_to_uint8(&m.data[42..43]), |x, y| format!("{}-{}", x, y));

        assert_eq!(brm_bms_version_date, Some("65535-255-255".to_string()));

        let brm_bms_version = bytes_to_uint8(&m.data[46..47])
            .zip_with(bytes_to_uint8(&m.data[47..48]).map(|x| x as u32 * 256), |x, y| x as u32 + y)
            .zip_with(bytes_to_uint8(&m.data[48..49]).map(|x| x as u32 * 65536), |x, y| x as u32 + y);
        assert_eq!(brm_bms_version, Some(16777215));
    }
}

#[test]
fn test_bro() {
    let hex = "686804470500280912000F00E70703010D2C11AADE";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
        message.len() > 19
    {   let m = Message::new(&message);
        let cro_bms_status = bytes_to_uint8(&m.data[0..1]);
        assert_eq!(cro_bms_status, Some(170));
    }
}

#[test]
fn test_bsd() {
    let hex = "686804820600281C1800DB0DE7070301030A0C61A901AA01474B23";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
        message.len() > 19
    {
        let m = Message::new(&message);

        let bsd_stop_soc = bytes_to_uint8(&m.data[0..1]);
        assert_eq!(bsd_stop_soc, Some(97));

        let bsd_min_single_volt = bytes_to_f32(&m.data[0..1], &m.data[1..2], 256, 0.01, 0.0);
        assert_eq!(bsd_min_single_volt, Some(433.61));

        let bsd_max_single_volt = bytes_to_f32(&m.data[2..3], &m.data[3..4], 256, 0.01, 0.0);
        assert_eq!(bsd_max_single_volt, Some(435.21));

        let bsd_min_temp = bytes_to_uint8(&m.data[5..6]).map(|x| x - 50);
        assert_eq!(bsd_min_temp, Some(21));

        let bsd_max_temp = bytes_to_uint8(&m.data[6..7]).map(|x| x - 50);
        assert_eq!(bsd_max_temp, Some(25));
    }
}

#[test]
fn test_bsm() {
    let hex = "686804470500281318004800E70703010D2D0E5D5203400100D0D8";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
        message.len() > 19
    {
        let m = Message::new(&message);

        let bsm_max_single_volt_no = bytes_to_uint8(&m.data[0..1]).map(|x| x + 1);
        assert_eq!(bsm_max_single_volt_no, Some(94));

        let bsm_max_temp = bytes_to_uint8(&m.data[1..2]).map(|x| x - 50);
        assert_eq!(bsm_max_temp, Some(32));

        let bsm_max_temp_no = bytes_to_uint8(&m.data[2..3]).map(|x| x + 1);
        assert_eq!(bsm_max_temp_no, Some(4));

        let bsm_min_temp = bytes_to_uint8(&m.data[3..4]).map(|x| x- 50);
        assert_eq!(bsm_min_temp, Some(14));

        let bsm_min_temp_no = bytes_to_uint8(&m.data[4..5]).map(|x| x + 1);
        assert_eq!(bsm_min_temp_no, Some(2));

        let bsm_single_volt_status = bytes_to_uint8(&m.data[5..6]).map(|x| x % 4);
        assert_eq!(bsm_single_volt_status, Some(0));

        let bsm_soc_status = bytes_to_uint8(&m.data[5..6]).map(|x| x / 4 % 4);
        assert_eq!(bsm_soc_status, Some(0));

        let bsm_current_status = bytes_to_uint8(&m.data[5..6]).map(|x| x / 16 % 4);
        assert_eq!(bsm_current_status, Some(0));

        let bsm_temp_status = bytes_to_uint8(&m.data[5..6]).map(|x| x / 64 % 4);
        assert_eq!(bsm_temp_status, Some(0));

        let bsm_insulate_status = bytes_to_uint8(&m.data[6..7]).map(|x| x % 4);
        assert_eq!(bsm_insulate_status, Some(0));

        let bsm_connect_status = bytes_to_uint8(&m.data[6..7]).map(|x| x / 4 % 4);
        assert_eq!(bsm_connect_status, Some(0));

        let bsm_charge_status = bytes_to_uint8(&m.data[6..7]).map(|x| x / 16 % 4);
        assert_eq!(bsm_charge_status, Some(1));
    }
}

#[test]
fn test_bst() {
    let hex = "68680482060028191500F30BE707030102061D000000F0E7";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
        message.len() > 19
    {
        let m = Message::new(&message);

        let bst_insulation_status = bytes_to_uint8(&m.data[0..1]).map(|x| x & 3);
        assert_eq!(bst_insulation_status, Some(0));

        let bst_conn_over_temp_status = bytes_to_uint8(&m.data[1..2]).map(|x| x & 12 / 4);
        assert_eq!(bst_conn_over_temp_status, Some(0));

        let bst_bms_over_temp_status = bytes_to_uint8(&m.data[1..2]).map(|x| x & 48 / 16);
        assert_eq!(bst_bms_over_temp_status, Some(0));

        let bst_charge_conn_status = bytes_to_uint8(&m.data[1..2]).map(|x| x & 192 / 64);
        assert_eq!(bst_charge_conn_status, Some(0));

        let bst_battery_pack_temp_status = bytes_to_uint8(&m.data[2..3]).map(|x| x & 3);
        assert_eq!(bst_battery_pack_temp_status, Some(0));

        let bst_high_volt_relay_status = bytes_to_uint8(&m.data[2..3]).map(|x| x & 12 / 4);
        assert_eq!(bst_high_volt_relay_status, Some(0));

        let bst_charging_port_volt_status = bytes_to_uint8(&m.data[2..3]).map(|x| x & 48 / 16);
        assert_eq!(bst_charging_port_volt_status, Some(0));

        let bst_other_status = bytes_to_uint8(&m.data[2..3]).map(|x| x & 192 / 64);
        assert_eq!(bst_other_status, Some(0));

        let bst_current_status = bytes_to_uint8(&m.data[3..4]).map(|x| x & 3);
        assert_eq!(bst_current_status, Some(0));

        let bst_volt_status = bytes_to_uint8(&m.data[3..4]).map(|x| x & 12 / 4);
        assert_eq!(bst_volt_status, Some(0));
    }
}

#[test]
fn test_ccs() {
    let hex = "686804470500281218004800E70703010D2D0EAA0D570D0100FD3D";

    let message = hex::decode(hex);
    if let Ok(message) = message &&
        message.len() > 19
    {
        let m = Message::new(&message);

        let ccs_out_volt = bytes_to_f32(&m.data[0..1], &m.data[1..2], 256, 0.1, 0.0);
        assert_eq!(ccs_out_volt, Some(349.80002));

        let ccs_out_current = bytes_to_f32(&m.data[2..3], &m.data[3..4], 256, 0.1, 400.0);
        assert_eq!(ccs_out_current, Some(-58.5));

        let ccs_charge_time = two_bytes_to_u16(&m.data[4..5], &m.data[5..6], 256);
        assert_eq!(ccs_charge_time, Some(1));

        let ccs_charge_status = bytes_to_uint8(&m.data[6..7]).map(|x| x % 4);
        assert_eq!(ccs_charge_status, Some(1));
    }
}

#[test]
fn test_cem() {
    let hex = "686804840000271F15000200E70703010F1623FCF0C0FDC8";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
        message.len() > 19
    {
        let m = Message::new(&message);

        let cem_error = bytes_to_uint8(&m.data[0..1])
            .zip_with(bytes_to_uint8(&m.data[1..2]).map(|x| x as u32 * 256), |x, y| x as u32 + y)
            .zip_with(bytes_to_uint8(&m.data[2..3]).map(|x| x as u32 * 65536), |x, y| x + y)
            .zip_with(bytes_to_uint8(&m.data[3..4]).map(|x| x as u32 * 16777216), |x, y| x + y);
        assert_eq!(cem_error,  Some(4257280252));
    }
}

#[test]
fn test_cml() {
    let hex = "686804470500280819000E00E70703010D2C114C1DE8038C0A960F72";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
        message.len() > 19
    {
        let m = Message::new(&message);

        let cml_max_out_volt = bytes_to_f32(&m.data[0..1], &m.data[1..2], 256, 0.1, 0.0);
        assert_eq!(cml_max_out_volt, Some(750.0));

        let cml_min_out_volt = bytes_to_f32(&m.data[2..3], &m.data[3..4], 256, 0.1, 0.0);
        assert_eq!(cml_min_out_volt, Some(100.0));

        let cml_max_out_current = bytes_to_f32(&m.data[4..5], &m.data[5..6], 256, 0.1, 400.0);
        assert_eq!(cml_max_out_current, Some(-130.0));

        let cml_min_out_current = bytes_to_f32(&m.data[6..7], &m.data[7..8], 256, 0.1, 400.0);
        assert_eq!(cml_min_out_current, Some(-1.0));
    }
}

#[test]
fn test_crm() {
    let hex = "686804470500280119000E00E70703010D2C11AA000000003132331C";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
        message.len() > 19
    {
        let m = Message::new(&message);

        let crm_recognizable = bytes_to_int8(&m.data[0..1]);
        assert_eq!(crm_recognizable, Some(-86));

        let crm_version = str::from_utf8(&m.data[5..8]).ok().map(|x| x.to_string());
        assert_eq!(crm_version, Some("123".to_string()));

        let crm_charge_pile_no = str::from_utf8(&m.data[0..4]).ok().map(|x| x.to_string());
        assert_eq!(crm_charge_pile_no, None);
    }
}

#[test]
fn test_cro() {
    let hex = "686804470500280A12001000E70703010D2C12AA8B";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
        message.len() > 19
    {
        let m = Message::new(&message);

        let cro_bms_status = bytes_to_uint8(&m.data[0..1]);
        assert_eq!(cro_bms_status, Some(170));
    }
}

#[test]
fn test_csd() {
    let hex = "686804820600281D1900F40BE707030102061D320010010100000044";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
        message.len() > 19
    {
        let m = Message::new(&message);

        let csd_charge_time = bytes_to_f32(&m.data[0..1], &m.data[1..2], 256, 1.0, 0.0);
        assert_eq!(csd_charge_time, Some(50.0));

        let csd_out_energy = bytes_to_f32(&m.data[1..2], &m.data[2..3], 256, 0.1, 0.0);
        assert_eq!(csd_out_energy, Some(409.6));

        let csd_charge_pile_no = bytes_to_uint8(&m.data[4..5])
            .zip_with(bytes_to_uint8(&m.data[5..6]).map(|x| x as u16 * 256), |x, y| x as u32 + y as u32)
            .zip_with(bytes_to_uint8(&m.data[6..7]).map(|x| x as u32 * 65536), |x, y| x + y)
            .zip_with(bytes_to_uint8(&m.data[7..8]).map(|x| x as u32 * 16777216), |x, y| x + y);
        assert_eq!(csd_charge_pile_no, Some(1));
    }
}

#[test]
fn test_cst() {
    let hex = "686804820600281A1500440CE70703010708260400F0FA48";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
        message.len() > 19
    {
        let m = Message::new(&message);

        let cst_temp_status = bytes_to_uint8(&m.data[1..2]).map(|x| x & 3);
        assert_eq!(cst_temp_status, Some(0));

        let cst_connector_status = bytes_to_uint8(&m.data[1..2]).map(|x| x & 12 / 4);
        assert_eq!(cst_connector_status, Some(0));

        let cst_internal_temp_status = bytes_to_uint8(&m.data[1..2]).map(|x| x & 48 / 16);
        assert_eq!(cst_internal_temp_status, Some(0));

        let cst_power_status = bytes_to_uint8(&m.data[1..2]).map(|x| x & 192 / 64);
        assert_eq!(cst_power_status, Some(0));

        let cst_emergency_stop_status = bytes_to_uint8(&m.data[2..3]).map(|x| x & 3);
        assert_eq!(cst_emergency_stop_status, Some(0));

        let cst_other_status = bytes_to_uint8(&m.data[2..3]).map(|x|x & 12 / 4);
        assert_eq!(cst_other_status, Some(0));

        let cst_current_status = bytes_to_uint8(&m.data[3..4]).map(|x| x & 3);
        assert_eq!(cst_current_status, Some(2));

        let cst_volt_status = bytes_to_uint8(&m.data[3..4]).map(|x| x & 12 / 4);
        assert_eq!(cst_volt_status, Some(2));
    }
}

#[test]
fn test_cts() {
    let hex = "686804470500280718000F00E70703010E151B2521140103232077";
    let byte_sign = 0xff;
    let message = hex::decode(hex);
    if let Ok(message) = message &&
        message.len() > 19 {
        let m = Message::new(&message);
        let cts_year = bytes_to_uint8(&m.data[6..7]);
        let cts_month = bytes_to_uint8(&m.data[4..5]);
        let cts_day = bytes_to_uint8(&m.data[3..4]);
        let cts_hour = bytes_to_uint8(&m.data[2..3]);
        let cts_minute = bytes_to_uint8(&m.data[1..2]);
        let cts_second = bytes_to_uint8(&m.data[0..1]);

        if let Some(cts_year) = cts_year &&
        let Some(cts_month) = cts_month &&
        let Some(cts_day) = cts_day &&
        let Some(cts_hour) = cts_hour &&
        let Some(cts_minute) = cts_minute &&
        let Some(cts_second) = cts_second {
            let cts_year = format!("{}{}{}{}", cts_year/16,  cts_year%16,cts_year/16,  cts_year%16);
            let cts_month = format!("{}{}", cts_month/16, cts_month%16);
            let cts_day = format!("{}{}", cts_day/16, cts_day%16);
            let cts_hour = format!("{}{}", cts_hour/16, cts_hour%16);
            let cts_minute = format!("{}{}", cts_minute/16, cts_minute%16);
            let cts_second = format!("{}{}", cts_second/16, cts_second%16);
            assert_eq!(
            format!("{}-{}-{} {}:{}:{}", cts_year,cts_month, cts_day,cts_hour, cts_minute, cts_second),
            "2020-03-01 14:21:25"
            );
        }
    }
}
