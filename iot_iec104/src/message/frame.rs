use iot_common::*;
use crate::message::Message;

#[derive(Debug)]
pub enum Frame {
    IMessage(I),
    UMessage(U),
    SMessage(S),
    HeartMessage
}

impl Frame {
    pub fn new(message: &Message) -> Self {
        match bytes_to_uint8(&message.control_domain[0..1]).map(|x| x  & 0x03) {
            Some(1) => Frame::SMessage(S::new(message)),
            Some(3) => Frame::UMessage(U::new(message)),
            _ => Frame::IMessage(I::new(message))
        }
    }
}

#[derive(Debug)]
pub struct I {
    // 发送序列号
    pub send_serial_number: Option<u16>,
    // 接收序列号
    pub receive_serial_number: Option<u16>
}

impl I {
    fn new(m: &Message) -> Self {
        let send_serial_number = two_bytes_to_u16(&m.control_domain[0..1], &m.control_domain[1..2], 256).map(|x| x / 2);
        let receive_serial_number = two_bytes_to_u16(&m.control_domain[2..3], &m.control_domain[3..4], 256).map(|x| x / 2);
        Self {
            send_serial_number,
            receive_serial_number
        }
    }
}

#[derive(Debug)]
pub struct S {
    // 接收序列号
    pub receive_serial_number: Option<u16>
}

impl S {
    fn new(m: &Message) -> Self {
        let receive_serial_number = two_bytes_to_u16(&m.control_domain[2..3], &m.control_domain[3..4], 256).map(|x| x / 2);
        Self {
            receive_serial_number
        }
    }
}

#[derive(Debug)]
pub struct U {
    pub command: String,
    pub command_type: String
}

impl U {
    fn new(m: &Message) -> Self {
        let buf = bytes_to_uint8( &m.control_domain[0..1]);
        let (command, command_type) = if buf.map(|x| x & 0xC0) == Some(128) {
            ("TESTFR", "确认")
        } else if buf.map(|x| x & 0xC0) == Some(64) {
            ("TESTFR", "命令")
        } else if buf.map(|x| x & 0x30) == Some(32) {
            ("STOPDT", "确认")
        } else if buf.map(|x| x & 0x30) == Some(16) {
            ("STOPDT", "命令")
        } else if buf.map(|x| x & 0x0C) == Some(8) {
            ("STARTDT", "确认")
        } else if buf.map(|x| x & 0x0C) == Some(4) {
            ("STARTDT", "命令")
        } else {
            ("", "")
        };
        Self {
            command: command.to_string(),
            command_type: command_type.to_string()
        }
    }
}