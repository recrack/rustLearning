
use std;
use comm::Chan;
use comm::Port;
use comm::send;
use comm::recv;

fn a(c: Chan<int>, data: int) {
   io::println(#fmt("sending data %d",data)); 
   send(c, data);
}

fn main() {
    let p = Port();
    let ch = Chan(p);
    task::spawn(|| a(ch,100) );
    task::spawn(|| a(ch,200) );
    task::spawn(|| a(ch,300) );
    task::spawn(|| a(ch,400) );
   
    let mut n: int = 0;
    n = recv(p);
   
    io::println(#fmt("main fn :: received :: %d ",n));
   
    n = recv(p);

    io::println(#fmt("main fn :: received :: %d ",n)); 


    n = recv(p);

    io::println(#fmt("main fn :: received :: %d ",n));

    n = recv(p);

    io::println(#fmt("main fn :: received :: %d ",n));
   //debug!("Finished.");
}
