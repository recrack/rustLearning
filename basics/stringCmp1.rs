import io::*;
import core::*;
fn main(){
    let mut price;

    let mut item = io::stdin().read_line();
    let mut item1 = io::stdin().read_line();
    let mut item2 = io::stdin().read_line();

    if str::eq(&item, &item2){
        io::println(#fmt("item==item2 %d",str::eq(&item, &item2)as int));
        price = 3.5;
    }else if str::eq(&item,&item1){
        io::println(#fmt("item==item1 %d",str::eq(&item, &item1)as int));
        price = 2.25;
    }else{
        io::println(#fmt("item!=item1!=item2 %d",str::eq(&item, &item2)as int));        price = 5.0;
    }     

   io::println(#fmt("%f",price));
}

