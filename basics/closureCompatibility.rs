import io::println;

fn main() {

   // takes a function or closure as an argument and executes it
   fn call_twice(f: fn()) {
                         f();
                         f();
                   };

call_twice(|| { println(~"I am an inferred stack closure") ; } );
call_twice(fn&() { println(~"I am also a stack closure"); } );
call_twice(fn@() { println(~"I am a boxed closure"); });
call_twice(fn~() { println(~"I am a unique closure"); });

//calling bare function
fn bare_function() { println(~"I am a plain function"); }
call_twice(bare_function);


}

