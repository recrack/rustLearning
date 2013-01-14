import io::println;

struct point{
    x:float,
    y:float
}

enum direction{
    east =0 ,
    west,
    south,
    north
}

fn point_from_direction(dir:direction)->point{
    match dir{
        east    =>point{x: 1f, y:0f},
        west    =>point{x: -1f, y: 0f},
        south   =>point{x: 0f, y: -1f},
        north   =>point{x: 0f, y: 0f}
    }
}

fn main(){
    let dir : direction = east;

    let p:point = point_from_direction(dir);
  
    println(fmt!("%d", west as int));
    println(fmt!("%f %f", p.x, p.y));
}
