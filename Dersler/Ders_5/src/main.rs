use std::os::macos::raw::stat;

struct Point{
    x:f64,
    y:f64,
}

struct Line{
    start : Point,
    end : Point,
}


fn structs(){
    let p = Point{x:4.0,y:3.0};
    println!("Point p is ({}, {})", p.x, p.y);
    let p2 = Point{x:5.0, y:10.0};;
    let myLine = Line{start: p, end : p2};
    println!("My Line is on ({}, {}) - ({}, {}) points", myLine.start.x, myLine.start.y, myLine.end.x, myLine.end.y);

}


fn main() {
    structs();
}
