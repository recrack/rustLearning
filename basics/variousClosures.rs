import io::*;

fn call_closure_with_ten(b: fn(int)){
    b(10);
}

fn call_closure_with_str(b: fn(~str)){
        b(~"hello");
}

fn call_closure_with_float(b: fn(float)){
        b(4.32);
}

fn main(){
    let captured_var = 20;
   
    let close = |argue|println(fmt!("captured_var = %d, arg = %d", captured_var, argue));
    let closefloat = |arg|println(fmt!("captured_var = %d, arg = %f", captured_var, arg));    
    let closestring = |arg:~str|println(fmt!("captured_var = %d, arg = %s", captured_var, arg));

    call_closure_with_ten(close);
    call_closure_with_float(closefloat);
    call_closure_with_str(closestring);
}

