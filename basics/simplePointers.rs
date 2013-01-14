fn main( args: ~[~str]){
  
   let mut unique = ~"unique string";
   let mut unique_ptr = &unique;


   io::println(unique);
   io::println(*unique_ptr);

}
