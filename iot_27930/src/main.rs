#![feature(let_chains)]

use iot_common::bytes_to_uint8;
use iot_27930::message::Frame;
use iot_27930::message::Message;

fn main() {
    let messages = vec![
        "686804470500281318004800E70703010D2D0E5D5203400100D0D8",
        "686804470500280119000E00E70703010D2C11AA000000003132331C",
        "686804470500280242000E00E70703010D2C1001010003E204140D4341544CFFFFFFFFFFFFFFFFFFFFFFFF383434303530424B30544754533338524CFFFFFFFFFFFFFFFF5E",
        "68680447050028061E000E00E70703010D2C117001B80BA201F30E731E02520DB0",
        "686804470500280718000F00E70703010E151B2521140103232077",
        "686804470500280819000E00E70703010D2C114C1DE8038C0A960F72",
        "686814470500280819000E00E70703010D2C114C1DE8038C0A960F72",
        "686804470500280912000F00E70703010D2C11AADE",
        "686804470500280A12001000E70703010D2C12AA8B",
        "686804470500281016001005E70703010C2E30DE0E110D011A",
        "68680447050028111A001005E70703010C2E30B60D180D53112B3F00CA",
        "686804470500281218004800E70703010D2D0EAA0D570D0100FD3D",
        "686814470500281218004800E70703010D2D0EAA0D570D0100FD3D",
        "68680482060028155B00D811E7070301053404A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A001A00181",
        "68680482060028161900D805E70703010500344747484849484A472B",
        "68680482060028191500F30BE707030102061D000000F0E7",
        "686804820600281A1500440CE70703010708260400F0FA48",
        "686804820600281C1800DB0DE7070301030A0C61A901AA01474B23",
        "686804820600281D1900F40BE707030102061D320010010100000044",
        "686804820600281E15000000E70703010E0719F1F0F0FCD4",
        "686804840000271F15000200E70703010F1623FCF0C0FDC8",
        "686804840000272613000000E707030115313B6612D4A7",
        "686804840000272713000000E707030115313B6612D4",
        "68680401000028F119000000E707060207320201E70706020732029A",
        "68680401000028FA19000000E707060207320201070706020732029A",
        "68680401000028FB19000000E707060207320201070706020732029A",
        "68680401000028FC19000000E707060207320201070706020732029A028FC19000000E707060207320201070706020732029A",
        "68680401000028FE19000000E707060207320201070706020732029A"
    ];

    for hex in messages.iter() {
        let message = hex::decode(hex);
        if let Ok(message) = message &&
            message.len() > 19
        {
            let message = Frame::new(&Message::new(&message));
            println!("{:?}", message);
        }
    }
}
