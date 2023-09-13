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

    #[test]
    fn it_works() {
        let hex = "687e02000000011d0300010015000001a50000011101000112010001130100011401000115010001160100011701000118010001190100011a0100011b0100011c0100011d0100011e0100011f010001200100012101000122010001230100012401000125010001260100012701000128010001290100014f01000155010001";

        let message = hex::decode(hex);
        if let Ok(message) = message &&
            message.len() > 6
        {
            let message = message::Message::new(&message);
            assert_eq!(header(&message.header), Some(104));
            assert_eq!(apdu_length(&message.apdu_length), Some(126));


            let m = Frame::new(&message);
            println!("{:?}", m);
        }
    }
}
