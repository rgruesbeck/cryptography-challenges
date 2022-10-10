/*
   Decode hexadecimal byte to binary value

   https://en.wikipedia.org/wiki/Hexadecimal#Base16_(transfer_encoding)
*/

/*
pub fn encode_byte(byte: u8) -> &static mut str {
    let char_set = "0123456789abcdef";
    let mut res = [0u8; 2];

    let a = u8::from(byte & 0b1111_0000) >> 4;
    res[0] = match char_set.chars().nth(a.into()) {
        Some(c) => c,
        None => 0,
    };

    let b = u8::from(byte & 0b0000_1111);
    res[1] = match char_set.chars().nth(b.into()) {
        Some(c) => c,
        None => 0,
    };

    std::str::from_utf8_mut(&mut res).unwrap()
}
*/

pub fn decode_byte(byte: u8) -> u8 {
    match byte & 0b0111_0000 {
        48 => byte & 0b0000_1111,
        96 => (byte & 0b0000_1111) + 9,
        _ => 0,
    }
}
