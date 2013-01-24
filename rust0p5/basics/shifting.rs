enum test { thing = -5 >> 1u }
fn main() {
    assert(thing as int == -3);
    let mut a: int = 5;
    assert(a<<1 == 10);
    assert(a>>1 == 2);
}
