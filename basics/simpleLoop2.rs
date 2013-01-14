use std;
const repeat: uint = 5u;
fn main() {
    let hi = ~"Hi!";
    let mut count = 0u;
    while count < repeat {
        io::println(hi);
        count += 1u;
    }
}