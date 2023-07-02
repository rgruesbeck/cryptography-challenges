/*
    https://cryptopals.com/sets/1/challenges/1

    convert hex to base64
    *Always operate on raw bytes, never on encoded strings.
    *Only use hex and base64 for pretty-printing.
*/


/*
   hex char set: "0123456789abcdef"
   b64 char set: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
*/
fn hex_to_base64(s: &str) -> String {
    let mut result = new String();
    return result;
}

pub fn test() {
    let input_string: &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expected: &str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    let result = hex_to_base64(input_string);
    assert_eq!(result, expected);

    println!("\nChallenge 1: Hex to Base64");
    println!("exp: {}", expected);
    println!("res: {}", result);
}
