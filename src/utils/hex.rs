/*
    Decode hexadecimal byte to binary value

    https://en.wikipedia.org/wiki/Hexadecimal#Base16_(transfer_encoding)
 */
pub fn decode(b: u8) -> u8 {
    match b & 0b0111_0000 {
        48 => b & 0b0000_1111,
        96 => (b & 0b0000_1111) + 9,
        _ => 0
    }
}