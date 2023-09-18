#![feature(let_chains)]

use clap::Parser;
use iot_iec104::message::Frame;
use iot_iec104::message::Message;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Message to parse
    #[arg(short, long)]
    message: String
}

fn main() {
    let args = Args::parse();
    let message = hex::decode(args.message);
    if let Ok(message) = message &&
       message.len() >= 6
    {
        let message = Frame::new(&Message::new(&message));
        println!("{:?}", message);
    }
}