import io::*;

struct Point {x:float, y:float}

enum Shape{
    Circle(Point, float),
    Rectangle(Point, Point)
}

fn area(sh: Shape)->float{


    match sh{
        Circle(_,size)=>float::consts::pi*size*size,

        Rectangle(point1, point2)=>(point2.x-point1.x)*(point2.y-point1.y)
    }
}

fn main(){
  
    let circle = Circle(Point{x:4.5, y:5.5}, 1.0);
    let rec = Rectangle(Point{x:1.4, y:2.6},Point{x:4.4, y:10.6});

    io::println(fmt!("%f",area(circle)));
    io::println(fmt!("%f",area(rec)));
}
