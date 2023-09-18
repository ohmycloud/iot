#![feature(let_chains)]

use iot_common::*;
use crate::message::Frame;

pub mod message;

pub fn header(message: &[u8]) -> Option<u8> {
    bytes_to_uint8(message)
}
pub fn apdu_length(message: &[u8]) -> Option<u8> { bytes_to_uint8(message) }

#[cfg(test)]
mod tests {
    use super::*;
    use message::frame::*;
    use message::Message;
    #[test]
    fn it_works() {
        let hex = "687e02000000011d0300010015000001a50000011101000112010001130100011401000115010001160100011701000118010001190100011a0100011b0100011c0100011d0100011e0100011f010001200100012101000122010001230100012401000125010001260100012701000128010001290100014f01000155010001";
        let message = hex::decode(hex);
        if let Ok(message) = message &&
           message.len() > 6
        {
            if let Frame::IMessage(I {
                                       send_serial_number,
                                       receive_serial_number,
                                       asdu
                                   }) = Frame::new(&Message::new(&message)) {
                assert_eq!(send_serial_number, Some(1));
                assert_eq!(receive_serial_number, Some(0));
                assert_eq!(asdu, Asdu {
                    type_identifier: Some(1),
                    var_struct_qualifier: VarStructQualifier { sq: Some(0), info_element_num: Some(29) },
                    common_address: Some(1),
                    init_info_obj_addr: Some(21)
                });
            }

        }
    }
}
