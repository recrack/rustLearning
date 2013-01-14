import io::println;

fn main() {
    let mut a = ~[];
    vec::push(a, 1);
    vec::push(a, 2);
    vec::push(a, 3);
    for vec::each(a) |n| {
        println(fmt!("%d", n));
    }
}
