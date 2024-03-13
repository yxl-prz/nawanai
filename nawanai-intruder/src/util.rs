/// Turns a 16 bit integer into two 8 bit integers
///
/// `0001000000001000` (4104) -> `00010000` (16) `00001000` (8)
pub fn u16_to_u8(value: u16) -> (u8, u8) {
    let a = (value >> 8) as u8;
    let b = (value & 0xFF) as u8;
    (a, b)
}
