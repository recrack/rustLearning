// rust syntax is under heavy change mk_rng() has been changed to Rng()

fn main(){

  //just empty call to my job
  myJob();

}


//called from main
fn myJob() -> ~str{
  rand::Rng().gen_str(10)
}


#[test]
fn testMyJob(){

  assert(myJob() != myJob());

}


