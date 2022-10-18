/*
    https://cryptopals.com/sets/1/challenges/5

    Implement repeating-key XOR

    Here is the opening stanza of an important work of the English language:
        Burning 'em, if you ain't quick and nimble
        I go crazy when I hear a cymbal

    Encrypt it, under the key "ICE", using repeating-key XOR.
    In repeating-key XOR, you'll sequentially apply each byte of the key; the first byte of plaintext will be XOR'd against I, the next C, the next E, then I again for the 4th byte, and so on.

    It should come out to:
        0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272
        a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f

    Encrypt a bunch of stuff using your repeating-key XOR function. Encrypt your mail. Encrypt your password file. Your .sig file. Get a feel for it. I promise, we aren't wasting your time with this.

    strings: https://stackoverflow.com/questions/41034635/how-do-i-convert-between-string-str-vecu8-and-u8
    &str    -> String  | String::from(s) or s.to_string() or s.to_owned()
    &str    -> &[u8]   | s.as_bytes()
    &str    -> Vec<u8> | s.as_bytes().to_vec() or s.as_bytes().to_owned()
    String  -> &str    | &s if possible* else s.as_str()
    String  -> &[u8]   | s.as_bytes()
    String  -> Vec<u8> | s.into_bytes()
    &[u8]   -> &str    | s.to_vec() or s.to_owned()
    &[u8]   -> String  | std::str::from_utf8(s).unwrap(), but don't**
    &[u8]   -> Vec<u8> | String::from_utf8(s).unwrap(), but don't**
    Vec<u8> -> &str    | &s if possible* else s.as_slice()
    Vec<u8> -> String  | std::str::from_utf8(&s).unwrap(), but don't**
    Vec<u8> -> &[u8]   | String::from_utf8(s).unwrap(), but don't**

    * target should have explicit type (i.e., checker can't infer that)

    ** handle the error properly instead

*/

use crate::utils::hex;

fn encrypt(cypher: &str, plain_text: &str) -> String {
    let cypher_bytes = cypher.as_bytes();
    let mut cypher_text = String::new();
    let mut _hex_encoded_cypher_text = String::new();

    // iterate over plaintext bytes and apply cypher
    let plain_text_iter = plain_text.as_bytes().iter().enumerate();
    for (i, byte) in plain_text_iter {
        let cypher_byte = cypher_bytes[i % cypher_bytes.len()];
        let encrypted_byte = byte ^ cypher_byte;
        cypher_text.push(encrypted_byte as char);
    }

    // println!("{}, {}", cypher, cypher_text);

    // hex encode cypher text
    let encoded_cypher_text = hex::encode_bytes(cypher_text.into_bytes());

    encoded_cypher_text
}

pub fn test() {
    println!("\nChallenge 5: ");

    let plain_text = "Burning 'em, if you ain't quick and nimble I go crazy when I hear a cymbal";

    let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
    let result = encrypt("ICE", plain_text);
    println!("exp: {}", String::from(expected));
    println!("res: {}", result);
    // assert_eq!(result, String::from(expected));
}
