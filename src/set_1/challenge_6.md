# Break repeating XOR

plain text -> XOR -> base64

1. KEYSIZE = guess for key length
2. write function for [hamming distance](https://pequalsnp-team.github.io/cheatsheet/writing-good-writeup)
```rs
let a = "this is a test";
let b = "wokka wokka!!!";
let c = hamming_distance(a, b);
assert_eq!(c, 37)
```
3. test each keysize for smallest hamming distance

## hamming distance

- try XOR and then count the number of 1 bits
- count number of bits by [bit traversal](https://www.geeksforgeeks.org/count-total-bits-number/)

```rs
let mut count = 0;
for i in 1..3 {
    let bit = byte ^ 0b0000_0001;
}
```

- [creating writeups](https://youtu.be/Zw25_ySOrC0)
- [write up tips](https://pequalsnp-team.github.io/cheatsheet/writing-good-writeup)

index into string https://stackoverflow.com/questions/24542115/how-to-index-a-string-in-rust