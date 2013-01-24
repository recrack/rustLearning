
fn main(){

  //just empty call to my job
  myJob();

}


//called from main
fn myJob() -> ~str {
   
   copy os::args()[0]

}


#[test]
fn testMyJob(){

  assert(myJob() == ~"./HelloArgs");

}