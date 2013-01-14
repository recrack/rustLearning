


use std;
import io::println;

fn fact(n: int) -> int {
   io::println(~"calling fact function");
    let mut result = 1, i = 1;
    while i <= n {
        result *= i;
        i += 1;
    }
    return result;
}


fn main() {
   io::println(~"calling main function");
    let result = fact(6);
    io::println(~"factoriay = %d",result);
}
