// sum of a large vector
// 1. linear sum
// 2. parallel sum using 8 tasks
// time comparison
use pipes::{stream, SharedChan};
use task::spawn;

fn main(){
  
  let  rawdata: [int * 1000] =  [1, ..1000];
  let mut data = copy rawdata;
  let mut count = 0;
  
  assert data.len() > 1;
  
  io::println("setting values");
  while count < 1000 {
  
    data[count] = count+1;
    count += 1;
  }
  
  
  //make it immutable so that callee function is happy... uffff
  let rawdata1 = data;
  //io::println(fmt!("sum is %d",mySerialJob(data);));

  let (port,chan) = stream();
  let chan = SharedChan(move chan);
  
  let childChan1 = chan.clone();
  let childChan2 = chan.clone();
  
  do spawn |move childChan1|{
     childChan1.send(myParallelSum(rawdata1, 0,500));
  }
  
    do spawn |move childChan2|{
     childChan2.send(myParallelSum(rawdata1, 500,1000));
  }
  
  
  let mut sum:int = 0;
  sum = port.recv() + port.recv();
  
  io::println(fmt!("final value of sum is %d",sum));
}


//called from main
fn mySerialJob(data:[int * 1000]) -> int{
   
   //assert data.len() == 1000;  
   let mut sum:int = 0;
   
   io::println("calculating sum...");
   for data.each |n|{
      sum = sum + *n;
   }
   return sum;
}

fn myParallelSum(data: [int * 1000], start:int, end:int) -> int{
   let mut sum:int = 0;
   let mut startIndex = move start;
   
   io::println(fmt!("calculating sum from %d to %d ", startIndex, end));
   
   while startIndex < end{
     sum = sum + data[startIndex];
     startIndex += 1;
   }
   
   io::println("returning ..... ");
   return sum;

}

#[test]
fn testMyJob(){

  assert(true);

}