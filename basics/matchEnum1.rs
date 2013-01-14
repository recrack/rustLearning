import io::*;
import core::*;

enum DAY {
    sun, mon, tue, wed, thu, fri, sat
}

fn main(){
    let stack_crayons: &[DAY] = &[sun, mon, tue, wed, thu, fri, sat];

    loop{
    println("sun : 0 , mon : 1 ... sat : 6");
    let sel_num = option::unwrap(int::from_str((io::stdin().read_line())));

    match stack_crayons[sel_num]{
            sun => io::println("Sunday "),
                mon => io::println("Monday "),
                tue => io::println("Tuesday "),
                wed => io::println("Wednesday "),
                thu => io::println("Thursday"),
                fri => io::println("Friday"),
                sat => io::println("Saturday"),
        }
    }
}

