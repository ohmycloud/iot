#![feature(let_chains)]
#![feature(option_zip)]
#![allow(unused)]
#![allow(unused_imports)]

use iot_common::*;
use std::str;
use iot_iec104::message::Message;
use iot_iec104::message::Frame;
use iot_iec104::message::frame::*;

#[test]
fn test_i_control_domain() {
    let hex = "68120e0002000d01030001000240000000084200";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
       message.len() > 6
    {
        if let Frame::IMessage(I {
            send_serial_number, 
            receive_serial_number, 
            asdu
        }) = Frame::new(&Message::new(&message)) {
            assert_eq!(send_serial_number, Some(7));
            assert_eq!(receive_serial_number, Some(1));
            assert_eq!(asdu, Asdu { 
                type_identifier: Some(13), 
                var_struct_qualifier: VarStructQualifier { sq: Some(0), info_element_num: Some(1) },
                common_address: Some(1),
                init_info_obj_addr: Some(16386)
            });
        }
    }
}

#[test]
fn test_asdu() {
    let hex = "6871780002000d9414000100454200343353400034335340003433534000343353400034335340003433534000d222534000d222534000d2225340006f125340006f125340006f125340006f1253400034335340003433534000d222534000d2225340006f1253400000004040000000803f00";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
       message.len() > 6 {
        if let Frame::IMessage(I {
            send_serial_number, 
            receive_serial_number, 
            asdu
        }) = Frame::new(&Message::new(&message)) {
            assert_eq!(send_serial_number, Some(60));
            assert_eq!(receive_serial_number, Some(1));
            assert_eq!(asdu, Asdu { 
                type_identifier: Some(13), 
                var_struct_qualifier: VarStructQualifier { sq: Some(1), info_element_num: Some(20) },
                common_address: Some(1),
                init_info_obj_addr: Some(16965)
            });
        }
    }
}

#[test]
fn test_asdu_not_continues() {
    let hex = "68aa040000000d1403000100014000000020420002400000000c420003400000003442000440000000d2420018400085eb474200194000cd0cc743001a40009ad9c743001b4000cd4cc743001c40000000ff43001d400000608645001f400000001643002040000000344300234000678638440025400000004040002840009a593844002a400000002043002b400000002043002c400000003442002d40000000c842002e4000cdcc024300";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
       message.len() > 6 {
        if let Frame::IMessage(I {
                                   send_serial_number,
                                   receive_serial_number,
                                   asdu
                               }) = Frame::new(&Message::new(&message)) {
            assert_eq!(send_serial_number, Some(2));
            assert_eq!(receive_serial_number, Some(0));
            assert_eq!(asdu, Asdu {
                type_identifier: Some(13),
                var_struct_qualifier: VarStructQualifier { sq: Some(0), info_element_num: Some(20) },
                common_address: Some(1),
                init_info_obj_addr: Some(16385)
            });
        }
    }
}

#[test]
fn test_s_control_domain() {
    let hex = "68040100e200";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
       message.len() <= 6 {
        let m = Message::new(&message);
        let s_message = Frame::new(&m);
        if let Frame::SMessage(S {receive_serial_number }) = s_message {
            assert_eq!(receive_serial_number, Some(113));
        }
    }
}

#[test]
fn test_u_control_domain() {
    let hex = "680407000000";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
        message.len() <= 6 {
        let m = Message::new(&message);
        let s_message = Frame::new(&m);
        if let Frame::UMessage(U {command, command_type }) = s_message {
            assert_eq!(command, "STARTDT".to_string());
            assert_eq!(command_type, "命令".to_string());
        }
    }
}

