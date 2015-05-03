=ROT13=
A simple <a href="http://en.wikipedia.org/wiki/ROT13">ROT13</a> encryption function.
Can be applied on encrypted code again to reveal the plain text.

ROT13 works by shifting every character in the alphabet by 13 elements. This implementation
maintains upper/lower cases and ignores non-[A-Za-z] characters, thus is safe to use on
any string.

Takes a ```&str``` and returns a ```String```.

This crate was created as an exercise for me to learn Rust, but it's tested and fully
working (as far as I can tell), so go for it.

==Example==
```
extern crate rot13;

#[cfg(not(test))]
fn main() {
    println!("{}", rot13::rot13("Hello World!"));
}
```