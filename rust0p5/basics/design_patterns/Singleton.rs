
  //data fields are comma seperated in rust structure
  struct mySingle{
    someData:int
  }
  
  impl mySingle{
    static  instance: @mySingle = 0;
    
    static fn getInstance() -> @mySingle {
       if ! self.instance{
          self.instance = mySingle{someData:0};
       }
       return self.instance;
    }
  }
  
fn main(){

  //just empty call to my job
  myJob();

}


//called from main
fn myJob(){
  
let anInst = mySingle::getInstance();
let anotherInst = mySingle::getInstance();

}


#[test]
fn testMyJob(){

  assert(true);

}