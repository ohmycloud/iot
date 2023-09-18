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

#[derive(Debug, PartialEq, Eq)]
// 可变结构限定词
pub struct VarStructQualifier {
    pub sq: Option<u8>,
    pub info_element_num: Option<u16>
}


#[derive(Debug, PartialEq, Eq)]
pub struct Asdu {
    pub type_identifier: Option<u8>,
    pub var_struct_qualifier: VarStructQualifier,
    pub common_address: Option<u16>,
    pub init_info_obj_addr: Option<u32>,
}

#[derive(Debug)]
pub struct I {
    // 发送序列号
    pub send_serial_number: Option<u16>,
    // 接收序列号
    pub receive_serial_number: Option<u16>,
    // ASDU
    pub asdu: Asdu
}

impl I {
    fn new(m: &Message) -> Self {
        let send_serial_number = two_bytes_to_u16(&m.control_domain[0..1], &m.control_domain[1..2], 256).map(|x| x / 2);
        let receive_serial_number = two_bytes_to_u16(&m.control_domain[2..3], &m.control_domain[3..4], 256).map(|x| x / 2);
        let type_identifier = bytes_to_uint8(&m.type_identifier);
        let var_struct_qualifier = bytes_to_uint8(&m.var_struct_qualifier);
        let sq = var_struct_qualifier.map(|x| (x & 0x80) / 128 );
        let info_element_num = var_struct_qualifier.map(|x| x as u16 & 0x7f);
        let common_address = two_bytes_to_u16(&m.common_address[0..1], &m.common_address[1..2], 256);
        let start_info_obj_addr = init_info_obj_addr(&m.info_obj_addr[0..1], &m.info_obj_addr[1..2], &m.info_obj_addr[2..3]);

        match sq {
            Some(1) if type_identifier == Some(13) =>
            // 信息对象地址连续
                for i in 0..info_element_num.unwrap_or(0) {
                    println!("地址连续, 信息对象地址: {:?}, 信息对象值: {:?}", i as u32 + start_info_obj_addr.unwrap(),
                             bytes_to_float32(&m.asdu[6..][i as usize * 5 + 3 .. i as usize * 5 + 7])
                    );
                }

            Some(1) if type_identifier == Some(1) =>
            // 信息对象地址连续
                for i in 0..info_element_num.unwrap_or(0) {
                    println!("地址连续, 信息对象地址: {:?}, 信息对象值: {:?}", i as u32 + start_info_obj_addr.unwrap(),
                             bytes_to_int8(&m.asdu[6..][i as usize + 3 .. i as usize + 4])
                    );
                }

            Some(0) if type_identifier == Some(13) =>
            // 信息对象地址不连续
                for i in 0..info_element_num.unwrap_or(0) {
                    println!("地址非连续, 信息对象地址: {:?}, 信息对象值: {:?}", i as u32 + start_info_obj_addr.unwrap(),
                             bytes_to_float32(&m.asdu[6..][i as usize * 8 + 3 .. i as usize * 8 + 7])
                    );
                }

            Some(0) if type_identifier == Some(1) =>
            // 信息对象地址不连续
                for i in 1..=info_element_num.unwrap_or(0) {
                    println!("地址非连续, 信息对象地址: {:?}, 信息对象值: {:?}", init_info_obj_addr(&m.asdu[6..][(i as usize -1) * 4 .. (i as usize -1)*4 + 1], &m.asdu[6..][(i as usize -1)*4+1 .. (i as usize -1)*4+2], &m.asdu[6..][(i as usize -1)*4+2 .. (i as usize -1)*4+3]).unwrap(),
                        bytes_to_int8(&m.asdu[6..][i as usize * 4 - 1 .. i as usize * 4])
                    );
                }
            _ => println!("{:?}", "未知")
        }

        Self {
            send_serial_number,
            receive_serial_number,
            asdu: Asdu {
                type_identifier, 
                var_struct_qualifier: VarStructQualifier { sq, info_element_num },
                common_address,
                init_info_obj_addr: start_info_obj_addr
            }
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