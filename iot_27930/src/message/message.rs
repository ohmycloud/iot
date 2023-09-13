#[derive(Debug)]
pub struct Message<'a> {
    // 消息头
    pub header: &'a [u8],
    // 设备ID长度
    pub id_length: &'a [u8],
    // 设备ID
    pub id: &'a [u8],
    // PF码
    pub pfcode: &'a [u8],
    // 数据长度
    pub data_length: &'a [u8],
    // 时间索引
    pub time_index: &'a [u8],
    // 绝对时间
    pub abs_time: &'a [u8],
    // 数据
    pub data: &'a [u8],
}

impl<'a> Message<'a> {
    pub fn new(message: &'a [u8]) -> Message<'a> {
        Message {
            header: &message[0..2],
            id_length: &message[2..3],
            id: &message[3..7],
            pfcode: &message[7..8],
            data_length: &message[8..10],
            time_index: &message[10..12],
            abs_time: &message[12..19],
            data: &message[19..],
        }
    }
}
