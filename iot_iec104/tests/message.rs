#![feature(let_chains)]
#![feature(option_zip)]
#![allow(unused)]
#![allow(unused_imports)]

use iot_common::*;
use std::str;
use iot_iec104::message::Message;
use iot_iec104::message::Frame;
use iot_iec104::message::frame::{I, S, U};

#[test]
fn test_i_control_domain() {
    let hex = "68120e0002000d01030001000240000000084200";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
        message.len() > 6
    {
        let m = Message::new(&message);
        let i_message = Frame::new(&m);
        if let Frame::IMessage(I {send_serial_number, receive_serial_number}) = i_message {
            assert_eq!(send_serial_number, Some(7));
            assert_eq!(receive_serial_number, Some(1));
        }
    }
}

#[test]
fn test_s_control_domain() {
    let hex = "68040100e200";
    let message = hex::decode(hex);
    if let Ok(message) = message &&
       message.len() >= 6 {
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
        message.len() >= 6 {
        let m = Message::new(&message);
        let s_message = Frame::new(&m);
        if let Frame::UMessage(U {command, command_type }) = s_message {
            assert_eq!(command, "STARTDT".to_string());
            assert_eq!(command_type, "命令".to_string());
        }
    }
}

