fn main(){

  //just empty call to my job
  myJob();

}


//called from main
fn myJob(){
  
  //normal loop
  let mut count: i8 = 5;
  while count > 1 {
    count -= 1 ;
  }
  
  
  //for vectors 
   vec::each([1, 2, 3], |n| -> bool{
    printInteger(*n);
    true
  });

  
}


fn printInteger(n:int){
  io::println(fmt!(" %d ",n));

}


#[test]
fn testMyJob(){
  myJob();
  assert(true);

}