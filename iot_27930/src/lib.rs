#![feature(let_chains)]
#![feature(option_zip)]

pub mod message;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bsm() {
        let hex = "686804470500281318004800E70703010D2D0E5D5203400100D0D8";

        assert_eq!(
            vec![
                104, 104, 4, 71, 5, 0, 40, 19, 24, 0, 72, 0, 231, 7, 3, 1, 13, 45, 14, 93, 82, 3,
                64, 1, 0, 208, 216
            ],
            hex::decode(hex).unwrap()
        );
    }

    #[test]
    fn crm() {
        let hex = "686804470500280119000E00E70703010D2C11AA000000003132331C";
        assert_eq!(
            vec![
                104, 104, 4, 71, 5, 0, 40, 1, 25, 0, 14, 0, 231, 7, 3, 1, 13, 44, 17, 170, 0, 0, 0,
                0, 49, 50, 51, 28
            ],
            hex::decode(hex).unwrap()
        );
    }
}
