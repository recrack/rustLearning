use std;


// a function returns a closure
fn mk_appender(suffix: ~str) -> fn@(~str) -> ~str {
    return fn@(s: ~str) -> ~str { s + suffix };
}

fn main() {

    let shout = mk_appender(~"!");
    io::println(shout(~"hey ho, let's go"));
}
