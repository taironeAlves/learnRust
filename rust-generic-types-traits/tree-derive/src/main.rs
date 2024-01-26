#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}




fn main() {
 let p1 = Point {x:1, y:2};
 let p2 = Point {x:4, y:-3};

 if p1 == p2 {
    println!("equal!");
 }else {
    println!("not equal!");
 }

 println!("{}", p1); // can't print using the '{}' format specifier!
 println!("{:?}", p1); //  can't print using the '{:?}' format specifier!
}


