//executes a program and wait for termination


use std;

fn main(args: ~[~str]) {
   
    io::println(~"executing'" + args[1]);

    run::run_program(args[1],~[]);
   
}

