use std;

fn get_tuple_of_two_ints() -> (int, int){
    return (3,4);
}

fn main(){

    let (a,b) = get_tuple_of_two_ints();
    io::println(#fmt("%d %d", a,b));
}
