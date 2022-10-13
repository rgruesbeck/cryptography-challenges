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
*/

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

    // hex encode cypher text
    // needs hex sting encoder
    println!("{}, {}", cypher, cypher_text);

    cypher_text
}

pub fn test() {
    println!("\nChallenge 5: ");

    let plain_text = "
        Burning 'em, if you ain't quick and nimble
        I go crazy when I hear a cymbal
    ";

    let expected = "
        0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272
        a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f
    ";
    let result = encrypt("ICE", plain_text);
    println!("exp: {}", expected);
    println!("res: {}", result);
    // assert_eq!(result, expected);
}
