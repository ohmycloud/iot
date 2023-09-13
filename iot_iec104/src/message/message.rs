#[derive(Debug)]
pub struct Message<'a> {
    // 启动字符
    pub header: &'a [u8],
    // APDU 长度
    pub apdu_length: &'a [u8],
    // 控制域
    pub control_domain: &'a [u8],
    // ASDU 应用服务数据单元
    pub asdu: &'a [u8],
}

impl<'a> Message<'a> {
    pub fn new(message: &'a [u8]) -> Message<'a> {
        Message {
            header: &message[0..1],
            apdu_length: &message[1..2],
            control_domain: &message[2..6],
            asdu: &message[6..],
        }
    }
}
