/*
   XOR functions

   https://en.wikipedia.org/wiki/Exclusive_or
*/

/*
    XOR 2 byte vectors
*/
pub fn vecu8(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    a.iter()
    .zip(b.iter())
    .map(|(aa, bb)| aa ^ bb)
    .collect()
}

/*
    XOR 2 string slices
 */
pub fn str(a: &str, b: &str) -> Vec<u8> {
    vecu8(a.as_bytes().to_vec(), b.as_bytes().to_vec())
}