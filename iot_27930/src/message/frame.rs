#![feature(let_chains)]

use iot_common::*;
use crate::message::Message;
use std::str;

const BYTE_SIGN: u8 = 0xff;
const BYTE_HIGH: u16 = 0xffff;

#[derive(Debug)]
pub enum Frame {
    CrmMessage(CRM),
    BrmMessage(BRM),
    BcpMessage(BCP),
    CtsMessage(CTS),
    CmlMessage(CML),
    BroMessage(BRO),
    CroMessage(CRO),
    BclMessage(BCL),
    BcsMessage(BCS),
    CcsMessage(CCS),
    BmvMessage(BMV),
    BmtMessage(BMT),
    BstMessage(BST),
    CstMessage(CST),
    BsdMessage(BSD),
    CsdMessage(CSD),
    BemMessage(BEM),
    CemMessage(CEM),
    ChmMessage(CHM),
    BhmMessage(BHM),
    OrderStatus,
    Connect,
    Stats,
    Info,
    Forward,
    HeartMessage,
}

impl Frame {
    pub fn new(pf_code: u8, message: &Message) -> Self {
        match pf_code {
            1 => Frame::CrmMessage(CRM::new(message)),
            2 => Frame::BrmMessage(BRM::new(message)),
            6 => Frame::BcpMessage(BCP::new(message)),
            7 => Frame::CtsMessage(CTS::new(message)),
            8 => Frame::CmlMessage(CML::new(message)),
            9 => Frame::BroMessage(BRO::new(message)),
            10 => Frame::CroMessage(CRO::new(message)),
            16 => Frame::BclMessage(BCL::new(message)),
            17 => Frame::BcsMessage(BCS::new(message)),
            18 => Frame::CcsMessage(CCS::new(message)),
            21 => Frame::BmvMessage(BMV::new(message)),
            22 => Frame::BmtMessage(BMT::new(message)),
            25 => Frame::BstMessage(BST::new(message)),
            26 => Frame::CstMessage(CST::new(message)),
            28 => Frame::BsdMessage(BSD::new(message)),
            29 => Frame::CsdMessage(CSD::new(message)),
            30 => Frame::BemMessage(BEM::new(message)),
            31 => Frame::CemMessage(CEM::new(message)),
            38 => Frame::ChmMessage(CHM::new(message)),
            39 => Frame::BhmMessage(BHM::new(message)),
            241 => Frame::OrderStatus,
            250 => Frame::Connect,
            251 => Frame::Stats,
            252 => Frame::Info,
            254 => Frame::Forward,
            _ => Frame::HeartMessage,
        }
    }
}

#[derive(Debug)]
pub struct BCL {
    bcl_demand_volt: Option<f32>,
    bcl_demand_current: Option<f32>,
    bcl_charge_mode: Option<u8>,
}

impl BCL {
    fn new(m: &Message) -> Self {
        let bcl_demand_volt = bytes_to_f32(&m.data[0..1], &m.data[1..2], 256, 0.1, 0.0);
        let bcl_demand_current = bytes_to_f32(&m.data[2..3], &m.data[3..4], 256, 0.1, 0.0);
        let bcl_charge_mode = bytes_to_uint8(&m.data[4..5]);

        Self {
            bcl_demand_volt,
            bcl_demand_current,
            bcl_charge_mode,
        }
    }
}

#[derive(Debug)]
pub struct BCP {
    bcp_max_single_volt: Option<f32>,
    bcp_max_current: Option<f32>,
    bcp_max_energy: Option<f32>,
    bcp_max_volt: Option<f32>,
    bcp_max_temp: Option<u8>,
    bcp_start_soc: Option<f32>,
    bcp_start_volt: Option<f32>,
}

impl BCP {
    fn new(m: &Message) -> Self {
        let bcp_start_soc = bytes_to_f32(&m.data[9..10], &m.data[10..11], 256, 0.1, 0.0);
        let bcp_start_volt = bytes_to_f32(&m.data[11..12], &m.data[12..13], 256, 0.1, 0.0);
        let bcp_max_single_volt = bytes_to_uint8(&m.data[1..2])
            .map(|x| x as u16 * 256)
            .map(|x| x as f32 * 0.01);

        let bcp_max_current = bytes_to_f32(&m.data[2..3], &m.data[3..4], 256, 0.1, 400.0);
        let bcp_max_energy = bytes_to_f32(&m.data[4..5], &m.data[5..6], 256, 0.1, 0.0);
        let bcp_max_volt = bytes_to_f32(&m.data[6..7], &m.data[7..8], 256, 0.1, 0.0);
        let bcp_max_temp = bytes_to_uint8(&m.data[8..9]).map(|x| x - 50);
        Self {
            bcp_max_single_volt,
            bcp_max_current,
            bcp_max_energy,
            bcp_max_volt,
            bcp_max_temp,
            bcp_start_soc,
            bcp_start_volt,
        }
    }
}

#[derive(Debug)]
struct BCS {
    bcs_volt_measure: Option<f32>,
    bcs_current_measure: Option<f32>,
    bcs_max_single_volt: Option<f32>,
    bcs_max_single_volt_group: Option<u8>,
    bcs_soc: Option<u8>,
    bcs_remain_time: Option<u16>,
}

impl BCS {
    fn new(message: &Message) -> Self {
        let bcs_volt_measure =
            bytes_to_f32(&message.data[0..1], &message.data[1..2], 256, 0.1, 0.0);
        let bcs_current_measure =
            bytes_to_f32(&message.data[2..3], &message.data[3..4], 256, 0.1, -400.0);
        let bcs_max_single_volt =
            bytes_to_f32(&message.data[4..5], &message.data[5..6], 256, 0.01, 0.0);
        let bcs_max_single_volt_group = bytes_to_uint8(&message.data[5..6]).map(|x| x / 16);
        let bcs_soc = bytes_to_uint8(&message.data[6..7]);
        let bcs_remain_time = two_bytes_to_u16(&message.data[7..8], &message.data[8..9], 256);

        Self {
            bcs_volt_measure,
            bcs_current_measure,
            bcs_max_single_volt,
            bcs_max_single_volt_group,
            bcs_soc,
            bcs_remain_time,
        }
    }
}

#[derive(Debug)]
struct BEM {
    bem_error: Option<u32>,
}

impl BEM {
    fn new(m: &Message) -> Self {
        let bem_error = bytes_to_uint32_4(&m.data[0..1], &m.data[1..2], &m.data[2..3], &m.data[3..4], 256, 65536, 16777216);
        Self { bem_error }
    }
}

#[derive(Debug)]
struct BHM {
    bhm_max_charge_volt: Option<f32>,
}

impl BHM {
    fn new(m: &Message) -> Self {
        let bhm_max_charge_volt = bytes_to_f32(&m.data[0..1], &m.data[1..2], 256, 0.1, 0.0);

        Self {
            bhm_max_charge_volt,
        }
    }
}

#[derive(Debug)]
struct BMT {
    bmt_single_temp: Option<String>,
}

impl BMT {
    fn new(m: &Message) -> Self {
        let bmt_single_temp = if m.data.len() < 2 {
            None
        } else {
            Some(
                (0..(m.data.len() - 1))
                    .map(|i| bytes_to_uint8(&m.data[i..i + 1]).map(|x| x - 50))
                    .map(|x| {
                        if x.is_some() {
                            x.unwrap().to_string()
                        } else {
                            "".to_string()
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(","),
            )
        };
        Self { bmt_single_temp }
    }
}

#[derive(Debug)]
struct BMV {
    bmv_single_volt: Option<String>,
}

impl BMV {
    fn new(m: &Message) -> Self {
        let bmv_single_volt = if m.data.len() < 2 {
            None
        } else {
            Some(
                (0..((m.data.len() - 1) / 2))
                    .map(|i| {
                        bytes_to_f32(&m.data[i * 2..i * 2 + 1], &m.data[i * 2 + 1..i * 2 + 2], 256, 0.01, 0.0)
                    })
                    .map(|x| {
                        if x.is_some() {
                            x.unwrap().to_string()
                        } else {
                            "".to_string()
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(","),
            )
        };
        Self { bmv_single_volt }
    }
}

#[derive(Debug)]
struct BRM {
    brm_version: Option<String>,
    brm_battery_type: Option<i8>,
    brm_rated_cap: Option<f32>,
    brm_rated_volt: Option<f32>,
    brm_manufacturer: Option<String>,
    brm_bp_id: Option<i8>,
    brm_bp_date: Option<String>,
    brm_bp_charge_count: Option<u32>,
    brm_equity_type: Option<u8>,
    brm_vin: Option<String>,
    brm_bms_version: Option<u32>,
    brm_bms_version_date: Option<String>,
    brm_bms_version_no: Option<u8>,
}

impl BRM {
    fn new(m: &Message) -> Self {
        let brm_version = bytes_to_string_2(&m.data[0..1], &m.data[1..3], BYTE_SIGN, BYTE_HIGH);
        let brm_battery_type = bytes_to_int8(&m.data[3..4]);
        let brm_rated_cap = bytes_to_f32(&m.data[4..5], &m.data[5..6], 256, 0.1, 0.0);
        let brm_rated_volt = bytes_to_f32(&m.data[6..7], &m.data[7..8], 256, 0.1, 0.0);
        let brm_manufacturer = str::from_utf8(&m.data[8..12]).ok().map(|x| x.to_string());
        let brm_bp_id = bytes_to_int8(&m.data[12..13]);
        let brm_bp_date = bytes_to_uint8_2(&m.data[16..17], &m.data[17..18], &m.data[18..19], 1985);
        let brm_bp_charge_count = bytes_to_uint32(&m.data[19..20], &m.data[20..21], &m.data[21..22], 256, 65536);
        let brm_equity_type = bytes_to_uint8(&m.data[22..23]);

        let brm_vin = if let Ok(brm_vin) = str::from_utf8(&m.data[24..41]) {
            if brm_vin.starts_with("L") {
                Some(brm_vin.to_string())
            } else if brm_vin.ends_with("L") {
                Some(brm_vin.chars().rev().collect::<String>())
            } else {
                None
            }
        } else {
            None
        };

        let brm_bms_version_no = bytes_to_uint8(&m.data[42..43]);
        let brm_bms_version_date = bytes_to_string(&m.data[44..45], &m.data[45..46], &m.data[43..44], &m.data[42..43], 256);
        let brm_bms_version = bytes_to_uint32(&m.data[46..47], &m.data[47..48], &m.data[48..49], 256, 65536);

        Self {
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
            brm_bms_version_no,
        }
    }
}

#[derive(Debug)]
struct BRO {
    cro_bms_status: Option<u8>,
}

impl BRO {
    fn new(m: &Message) -> Self {
        let cro_bms_status = bytes_to_uint8(&m.data[0..1]);
        Self { cro_bms_status }
    }
}

#[derive(Debug)]
struct BSD {
    bsd_stop_soc: Option<u8>,
    bsd_min_single_volt: Option<f32>,
    bsd_max_single_volt: Option<f32>,
    bsd_min_temp: Option<u8>,
    bsd_max_temp: Option<u8>,
}

impl BSD {
    fn new(m: &Message) -> Self {
        let bsd_stop_soc = bytes_to_uint8(&m.data[0..1]);
        let bsd_min_single_volt = bytes_to_f32(&m.data[0..1], &m.data[1..2], 256, 0.01, 0.0);
        let bsd_max_single_volt = bytes_to_f32(&m.data[2..3], &m.data[3..4], 256, 0.01, 0.0);
        let bsd_min_temp = bytes_to_uint8(&m.data[5..6]).map(|x| x - 50);
        let bsd_max_temp = bytes_to_uint8(&m.data[6..7]).map(|x| x - 50);
        Self {
            bsd_stop_soc,
            bsd_min_single_volt,
            bsd_max_single_volt,
            bsd_min_temp,
            bsd_max_temp,
        }
    }
}

#[derive(Debug)]
struct BSM {
    bsm_max_single_volt_no: Option<u8>,
    bsm_max_temp: Option<u8>,
    bsm_max_temp_no: Option<u8>,
    bsm_min_temp: Option<u8>,
    bsm_min_temp_no: Option<u8>,
    bsm_single_volt_status: Option<u8>,
    bsm_soc_status: Option<u8>,
    bsm_current_status: Option<u8>,
    bsm_temp_status: Option<u8>,
    bsm_insulate_status: Option<u8>,
    bsm_connect_status: Option<u8>,
    bsm_charge_status: Option<u8>,
}

impl BSM {
    fn new(m: &Message) -> Self {
        let bsm_max_single_volt_no = bytes_to_uint8(&m.data[0..1]).map(|x| x + 1);
        let bsm_max_temp = bytes_to_uint8(&m.data[1..2]).map(|x| x - 50);
        let bsm_max_temp_no = bytes_to_uint8(&m.data[2..3]).map(|x| x + 1);
        let bsm_min_temp = bytes_to_uint8(&m.data[3..4]).map(|x| x - 50);
        let bsm_min_temp_no = bytes_to_uint8(&m.data[4..5]).map(|x| x + 1);
        let bsm_single_volt_status = bytes_to_uint8(&m.data[5..6]).map(|x| x % 4);
        let bsm_soc_status = bytes_to_uint8(&m.data[5..6]).map(|x| x / 4 % 4);
        let bsm_current_status = bytes_to_uint8(&m.data[5..6]).map(|x| x / 16 % 4);
        let bsm_temp_status = bytes_to_uint8(&m.data[5..6]).map(|x| x / 64 % 4);
        let bsm_insulate_status = bytes_to_uint8(&m.data[6..7]).map(|x| x % 4);
        let bsm_connect_status = bytes_to_uint8(&m.data[6..7]).map(|x| x / 4 % 4);
        let bsm_charge_status = bytes_to_uint8(&m.data[6..7]).map(|x| x / 16 % 4);

        Self {
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
            bsm_charge_status,
        }
    }
}

#[derive(Debug)]
struct BST {
    bst_insulation_status: Option<u8>,
    bst_conn_over_temp_status: Option<u8>,
    bst_bms_over_temp_status: Option<u8>,
    bst_charge_conn_status: Option<u8>,
    bst_battery_pack_temp_status: Option<u8>,
    bst_high_volt_relay_status: Option<u8>,
    bst_charging_port_volt_status: Option<u8>,
    bst_other_status: Option<u8>,
    bst_current_status: Option<u8>,
    bst_volt_status: Option<u8>,
}

impl BST {
    fn new(m: &Message) -> Self {
        let bst_insulation_status = bytes_to_uint8(&m.data[0..1]).map(|x| x & 3);
        let bst_conn_over_temp_status = bytes_to_uint8(&m.data[1..2]).map(|x| x & 12 / 4);
        let bst_bms_over_temp_status = bytes_to_uint8(&m.data[1..2]).map(|x| x & 48 / 16);
        let bst_charge_conn_status = bytes_to_uint8(&m.data[1..2]).map(|x| x & 192 / 64);
        let bst_battery_pack_temp_status = bytes_to_uint8(&m.data[2..3]).map(|x| x & 3);
        let bst_high_volt_relay_status = bytes_to_uint8(&m.data[2..3]).map(|x| x & 12 / 4);
        let bst_charging_port_volt_status = bytes_to_uint8(&m.data[2..3]).map(|x| x & 48 / 16);
        let bst_other_status = bytes_to_uint8(&m.data[2..3]).map(|x| x & 192 / 64);
        let bst_current_status = bytes_to_uint8(&m.data[3..4]).map(|x| x & 3);
        let bst_volt_status = bytes_to_uint8(&m.data[3..4]).map(|x| x & 12 / 4);

        Self {
            bst_insulation_status,
            bst_conn_over_temp_status,
            bst_bms_over_temp_status,
            bst_charge_conn_status,
            bst_battery_pack_temp_status,
            bst_high_volt_relay_status,
            bst_charging_port_volt_status,
            bst_other_status,
            bst_current_status,
            bst_volt_status,
        }
    }
}

#[derive(Debug)]
struct CCS {
    ccs_out_volt: Option<f32>,
    ccs_out_current: Option<f32>,
    ccs_charge_time: Option<u16>,
    ccs_charge_status: Option<u8>,
}

impl CCS {
    fn new(m: &Message) -> Self {
        let ccs_out_volt = bytes_to_f32(&m.data[0..1], &m.data[1..2], 256, 0.1, 0.0);
        let ccs_out_current = bytes_to_f32(&m.data[2..3], &m.data[3..4], 256, 0.1, 400.0);
        let ccs_charge_time = two_bytes_to_u16(&m.data[4..5], &m.data[5..6], 256);
        let ccs_charge_status = bytes_to_uint8(&m.data[6..7]).map(|x| x % 4);

        Self {
            ccs_out_volt,
            ccs_out_current,
            ccs_charge_time,
            ccs_charge_status,
        }
    }
}

#[derive(Debug)]
struct CEM {
    cem_error: Option<u32>,
}

impl CEM {
    fn new(m: &Message) -> Self {
        let cem_error = bytes_to_uint32_4(&m.data[0..1], &m.data[1..2], &m.data[2..3], &m.data[3..4], 256, 65536, 16777216);
        Self { cem_error }
    }
}

#[derive(Debug)]
struct CML {
    cm_max_out_volt: Option<f32>,
    cml_min_out_volt: Option<f32>,
    cml_max_out_current: Option<f32>,
    cml_min_out_current: Option<f32>,
}

impl CML {
    fn new(m: &Message) -> Self {
        let cm_max_out_volt = bytes_to_f32(&m.data[0..1], &m.data[1..2], 256, 0.1, 0.0);
        let cml_min_out_volt = bytes_to_f32(&m.data[2..3], &m.data[3..4], 256, 0.1, 0.0);
        let cml_max_out_current = bytes_to_f32(&m.data[4..5], &m.data[5..6], 256, 0.1, 400.0);
        let cml_min_out_current = bytes_to_f32(&m.data[6..7], &m.data[7..8], 256, 0.1, 400.0);

        Self {
            cm_max_out_volt,
            cml_min_out_volt,
            cml_max_out_current,
            cml_min_out_current,
        }
    }
}

#[derive(Debug)]
struct CRM {
    crm_version: Option<String>,
    crm_recognizable: Option<i8>,
    crm_charge_pile_no: Option<String>,
}

impl CRM {
    fn new(m: &Message) -> Self {
        let crm_recognizable = bytes_to_int8(&m.data[0..1]);
        let crm_version = str::from_utf8(&m.data[5..8]).ok().map(|x| x.to_string());
        let crm_charge_pile_no = str::from_utf8(&m.data[0..4]).ok().map(|x| x.to_string());

        Self {
            crm_version,
            crm_recognizable,
            crm_charge_pile_no,
        }
    }
}

#[derive(Debug)]
struct CRO {
    cro_bms_status: Option<u8>,
}

impl CRO {
    fn new(m: &Message) -> Self {
        let cro_bms_status = bytes_to_uint8(&m.data[0..1]);
        Self { cro_bms_status }
    }
}

#[derive(Debug)]
struct CSD {
    csd_charge_time: Option<f32>,
    csd_out_energy: Option<f32>,
    csd_charge_pile_no: Option<u32>,
}

impl CSD {
    fn new(m: &Message) -> Self {
        let csd_charge_time = bytes_to_f32(&m.data[0..1], &m.data[1..2], 256, 1.0, 0.0);
        let csd_out_energy = bytes_to_f32(&m.data[1..2], &m.data[2..3], 256, 0.1, 0.0);

        let csd_charge_pile_no = bytes_to_uint32_4(&m.data[4..5], &m.data[5..6], &m.data[6..7], &m.data[7..8], 256, 65536, 16777216);

        Self {
            csd_charge_time,
            csd_out_energy,
            csd_charge_pile_no,
        }
    }
}

#[derive(Debug)]
struct CST {
    cst_temp_status: Option<u8>,
    cst_connector_status: Option<u8>,
    cst_internal_temp_status: Option<u8>,
    cst_power_status: Option<u8>,
    cst_emergency_stop_status: Option<u8>,
    cst_other_status: Option<u8>,
    cst_current_status: Option<u8>,
    cst_volt_status: Option<u8>,
}

impl CST {
    fn new(m: &Message) -> Self {
        let cst_temp_status = bytes_to_uint8(&m.data[1..2]).map(|x| x & 3);
        let cst_connector_status = bytes_to_uint8(&m.data[1..2]).map(|x| x & 12 / 4);
        let cst_internal_temp_status = bytes_to_uint8(&m.data[1..2]).map(|x| x & 48 / 16);
        let cst_power_status = bytes_to_uint8(&m.data[1..2]).map(|x| x & 192 / 64);
        let cst_emergency_stop_status = bytes_to_uint8(&m.data[2..3]).map(|x| x & 3);
        let cst_other_status = bytes_to_uint8(&m.data[2..3]).map(|x| x & 12 / 4);
        let cst_current_status = bytes_to_uint8(&m.data[3..4]).map(|x| x & 3);
        let cst_volt_status = bytes_to_uint8(&m.data[3..4]).map(|x| x & 12 / 4);
        Self {
            cst_temp_status,
            cst_connector_status,
            cst_internal_temp_status,
            cst_power_status,
            cst_emergency_stop_status,
            cst_other_status,
            cst_current_status,
            cst_volt_status,
        }
    }
}

#[derive(Debug)]
struct CTS {
    cts_time: Option<String>,
}

impl CTS {
    fn new(m: &Message) -> Self {
        let cts_year = bytes_to_uint8(&m.data[6..7]);
        let cts_month = bytes_to_uint8(&m.data[4..5]);
        let cts_day = bytes_to_uint8(&m.data[3..4]);
        let cts_hour = bytes_to_uint8(&m.data[2..3]);
        let cts_minute = bytes_to_uint8(&m.data[1..2]);
        let cts_second = bytes_to_uint8(&m.data[0..1]);

        let cts_time = if let Some(cts_year)   = cts_year   &&
                              let Some(cts_month)  = cts_month  &&
                              let Some(cts_day)    = cts_day    &&
                              let Some(cts_hour)   = cts_hour   &&
                              let Some(cts_minute) = cts_minute &&
                              let Some(cts_second) = cts_second {
            let cts_year   = format!("{}{}{}{}", cts_year/16,  cts_year%16,cts_year/16,  cts_year%16);
            let cts_month  = format!("{}{}", cts_month/16, cts_month%16);
            let cts_day    = format!("{}{}", cts_day/16, cts_day%16);
            let cts_hour   = format!("{}{}", cts_hour/16, cts_hour%16);
            let cts_minute = format!("{}{}", cts_minute/16, cts_minute%16);
            let cts_second = format!("{}{}", cts_second/16, cts_second%16);

            Some(format!("{}-{}-{} {}:{}:{}", cts_year, cts_month, cts_day, cts_hour, cts_minute, cts_second))
        } else { None };

        Self { cts_time }
    }
}

#[derive(Debug)]
struct CHM {
    chm_version: Option<String>,
}

impl CHM {
    fn new(m: &Message) -> Self {
        let prefix_v = bytes_to_uint8(&m.data[0..1]);
        let suffix_v = two_bytes_to_u16(&m.data[1..2], &m.data[2..3], 256);
        let chm_version =  bytes_to_string_1(prefix_v, suffix_v);
        Self { chm_version }
    }
}
