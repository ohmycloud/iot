#[derive(Debug)]
pub struct Message<'a> {
    // 启动字符
    pub header: &'a [u8],
    // APDU 长度
    pub apdu_length: &'a [u8],
    // 控制域
    pub control_domain: &'a [u8],
    // 类型标识
    pub type_identifier: &'a [u8],
    // 可变结构限定词
    pub var_struct_qualifier: &'a [u8],
    // 传输原因
    pub trans_reason: &'a[u8],
    // 公共地址
    pub common_address: &'a[u8],
    // 信息对象地址
    pub info_obj_addr: &'a[u8],
    // ASDU 应用服务数据单元
    pub asdu: &'a [u8],
}

impl<'a> Message<'a> {
    pub fn new(message: &'a [u8]) -> Message<'a> {
        Message {
            header: &message[0..1],         // 1 字节
            apdu_length: &message[1..2],    // 1 字节
            control_domain: &message[2..6], // 4 字节
            type_identifier: if message.len() > 6 { &message[6..7] } else { &[] },      // 1 字节
            var_struct_qualifier: if message.len() > 6 { &message[7..8] } else { &[] }, // 1 字节
            trans_reason: if message.len() > 6 { &message[8..10] } else { &[] },        // 2 字节
            common_address: if message.len() > 6 { &message[10..12] } else { &[] },     // 2 字节
            info_obj_addr: if message.len() > 6 { &message[12..15] } else { &[] },      // 3 字节
            asdu: if message.len() > 6 { &message[6..] } else { &[] },
        }
    }
}
