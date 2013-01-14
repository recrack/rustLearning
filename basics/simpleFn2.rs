fn is_four(x: int)->bool{
    if x==4 {
        return true;
    }  
    else{
        return false;
    }  
}

fn main(){
    let chk:bool;
    chk = is_four(3);

    if(chk == false){
        io::println("false");
    }  
    else{
        io::println("true");
    }  

}
