
fn factorial(n:int) -> int {

   if n == 1 {
      return 1;
   } 
   if n <= 0 {
      return 0;   
   }
   
    n * factorial(n-1)
}


#[test]
fn testFact(){
  assert  0 == factorial(0);
  assert 24 == factorial(4);

}

#[test]
#[should_fail]
fn testFact2(){

  assert 1 == factorial(-1);
}
