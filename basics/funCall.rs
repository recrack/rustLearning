import io::println;

fn main() {
    println("calling main function");
    let result = facto(6);
    println(#fmt("factoria = %d",result));
}

fn facto (n: int) -> (int) {

    println("called function");
    
    let mut res = 1;
    let mut i = 1;
    
    while i < n {
        res = res * i;
        i += 1;
    }
    
    res
}


