/*
   Decode hexadecimal byte to binary value

   https://en.wikipedia.org/wiki/Hexadecimal#Base16_(transfer_encoding)
*/

pub fn decode_byte(byte: u8) -> u8 {
    match byte & 0b0111_0000 {
        48 => byte & 0b0000_1111,
        96 => (byte & 0b0000_1111) + 9,
        _ => 0,
    }
}

pub fn encode_byte(byte: u8) -> String {
    let char_set = "0123456789abcdef";
    let mut res = [0u8; 2];

    let a = u8::from(byte & 0b1111_0000) >> 4;
    res[0] = match char_set.chars().nth(a.into()) {
        Some(c) => c as u8,
        None => 0,
    };

    let b = u8::from(byte & 0b0000_1111);
    res[1] = match char_set.chars().nth(b.into()) {
        Some(c) => c as u8,
        None => 0,
    };

    std::str::from_utf8_mut(&mut res).unwrap().to_owned()
}

pub fn decode_string(string: &str) -> Vec<u8> {
    let mut bytes = vec![0u8; string.len()];
    // iterate string slice and push to decoded byte to byte vector
    for byte in string.as_bytes() {
        bytes.push(decode_byte(*byte));
    }

    bytes
}

pub fn encode_bytes(bytes: Vec<u8>) -> String {
    let mut res = Vec::with_capacity(bytes.len());
    
    for byte in bytes {
        res.push(encode_byte(byte));
    }

    res.concat()
}