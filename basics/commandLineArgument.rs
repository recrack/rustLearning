//suppose file name is argument.rs

//usage ./arguments  rust



fn main(args: ~[~str]) {
   
    io::println(~"hello world from '" + args[1] + ~"'!");
   
}
