import io::println;

fn each(v: ~[int], op: fn(int) ->bool){
    let mut n = 0;
    while n < v.len(){
        if!op(v[n]){
            break;
        }
        n+=1;
    }
}

fn main(){
    each(~[2,4,8,5,16], |n|{
        if n%2 !=0{
            println(fmt!("%d found odd number!",n));
            false
        }else {println(fmt!("%d", n));
            true}
    });
}

