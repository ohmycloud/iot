#![feature(option_zip)]
#![feature(let_chains)]

pub fn bytes_to_uint8(array: &[u8]) -> Option<u8> {
    if let Ok(slice) = array.try_into() {
        Some(u8::from_le_bytes(slice))
    } else {
        None
    }
}

pub fn bytes_to_uint16(array: &[u8]) -> Option<u16> {
    if let Ok(slice) = array.try_into() {
        Some(u16::from_le_bytes(slice))
    } else {
        None
    }
}

pub fn bytes_to_int8(array: &[u8]) -> Option<i8> {
    if let Ok(slice) = array.try_into() {
        Some(i8::from_le_bytes(slice))
    } else {
        None
    }
}

pub fn bytes_to_int16(array: &[u8]) -> Option<i16> {
    if let Ok(slice) = array.try_into() {
        Some(i16::from_le_bytes(slice))
    } else {
        None
    }
}

pub fn bytes_to_f32(
    low: &[u8],
    high: &[u8],
    remainder: u16,
    factor: f32,
    offset: f32,
) -> Option<f32> {
    bytes_to_uint8(low)
        .zip_with(
            bytes_to_uint8(high).map(|x| x as u16 * remainder),
            |x, y| (x as u16 + y) as f32 * factor,
        )
        .map(|x| x - offset)
}

pub fn two_bytes_to_u16(low: &[u8], high: &[u8], remainder: u16) -> Option<u16> {
    bytes_to_uint8(low).zip_with(
        bytes_to_uint8(high).map(|x| x as u16 * remainder),
        |x, y| x as u16 + y,
    )
}

pub fn bytes_to_string(array1: &[u8],
                       array2: &[u8],
                       array3: &[u8],
                       array4: &[u8],
                       remainder: u16) -> Option<String> {
    bytes_to_uint8(array1)
        .zip_with(
            bytes_to_uint8(array2).map(|x| x as u16 * remainder),
            |x, y| x as u16 + y,
        )
        .zip_with(bytes_to_uint8(array3), |x, y| {
            format!("{}-{}", x, y)
        })
        .zip_with(bytes_to_uint8(array4), |x, y| {
            format!("{}-{}", x, y)
        })
}

pub fn bytes_to_string_1(op1: Option<u8>, op2: Option<u16>) -> Option<String> {
    op1.zip_with(op2, |x, y| format!("V{}.{}", x, y))
}

pub fn bytes_to_uint32(array1: &[u8],
                       array2: &[u8],
                       array3: &[u8],
                       remainder1: u32,
                       remainder2: u32) -> Option<u32> {
    bytes_to_uint8(array1)
        .zip_with(
            bytes_to_uint8(array2).map(|x| x as u32 * remainder1),
            |x, y| x as u32 + y,
        )
        .zip_with(
            bytes_to_uint8(array3).map(|x| x as u32 * remainder2),
            |x, y| x + y,
        )
}

pub fn bytes_to_uint32_4(array1: &[u8],
                         array2: &[u8],
                         array3: &[u8],
                         array4: &[u8],
                         remainder1: u32,
                         remainder2: u32,
                         remainder3: u32) -> Option<u32> {
    bytes_to_uint8(array1)
        .zip_with(bytes_to_uint8(array2).map(|x| x as u32 * remainder1), |x, y| x as u32 + y)
        .zip_with(bytes_to_uint8(array3).map(|x| x as u32 * remainder2), |x, y| x + y)
        .zip_with(bytes_to_uint8(array4).map(|x| x as u32 * remainder3), |x, y| x + y)
}

pub fn bytes_to_string_2(array1: &[u8], array2: &[u8], and1: u8, and2: u16) -> Option<String> {
    bytes_to_uint8(array1)
        .map(|x| x & and1)
        .zip_with(
            bytes_to_uint16(array2).map(|x| x & and2),
            |x, y| format!("V{}.{}", x, y),
        )
}

pub fn bytes_to_uint8_2(array1: &[u8], array2: &[u8], array3: &[u8], offset: u16) -> Option<String> {
    bytes_to_uint8(array1)
        .map(|x| x as u16 + offset)
        .zip_with(bytes_to_uint8(array2), |x, y| {
            format!("{}-{}", x, y)
        })
        .zip_with(bytes_to_uint8(array3), |x, y| {
            format!("{}-{}", x, y)
        })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
