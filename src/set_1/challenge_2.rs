/*
    https://cryptopals.com/sets/1/challenges/2

    Fixed XOR
    Write a function that takes two equal-length buffers and produces their XOR combination.
    If your function works properly, then when you feed it the string: 

    https://en.wikipedia.org/wiki/One-time_pad
    khan academy: https://youtu.be/FlIG3TvQCBQ
    spy-story: https://youtu.be/S2nh2YrbweM
*/

fn fixed_xor(a: &str, b: &str) -> String {
    let char_set = "0123456789abcdef";
    let mut result = String::new();

    fn decode_hex_byte(n: u8) -> u8 {
        let a = u8::from(n & 0b_0_111_0000) >> 4;
        let b = u8::from(n & 0b_0_000_1111);
        let c = match a {
            3 => b,
            6 => b + 9,
            _ => 0
        };
        c
    }

    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();

    let mut i = 0;
    for a_byte in a_bytes {
        let b_byte = &b_bytes[i];
        let x_byte = u8::from(decode_hex_byte(*a_byte) ^ decode_hex_byte(*b_byte));
        match char_set.chars().nth(x_byte.into()) {
            Some(x) => {
                result.push(x);
            }
            None => ()
        }
        i += 1;
    }

    result
}

pub fn test() {
    let a = "1c0111001f010100061a024b53535009181c";
    let b = "686974207468652062756c6c277320657965";
    let expected = "746865206b696420646f6e277420706c6179";

    let result = fixed_xor(a, b);

    // https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
    assert_eq!(result, expected);

    println!("\nChallenge 2: Fixed XOR");
    println!("exp: {}", expected);
    println!("res: {}", result);
}