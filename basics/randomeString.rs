// rust syntax is under heavy change mk_rng() has been changed to Rng()



use std;
use std::rand;

fn main(){

let r = rand::Rng();
io::println(~"random string =" + r.gen_str(10));


}

