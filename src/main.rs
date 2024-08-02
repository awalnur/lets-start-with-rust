#![allow(dead_code)]

#[derive(Debug)]
struct Car{
    police_number: String,
    build_year: String,
    age: u8,
}

// a Unit Struct
struct Unit;

// A tuple Struct
struct Pair(i32, f32);

// a struct with two fields
struct Point{
    x: f32,
    y: f32,
}
 // a struct can be reuse as fields of another stuct
struct Rentangle{
     // a rentangle can be specified by where the top left and botton right
     // corners are in space.
     top_left: Point,
     bottom_right: Point
 }

fn main(){
    let police_number = String::from("C 9999 DE");
    let build_year = String::from("x");
    let age = 23;
    let car = Car{police_number, build_year, age};
    println!("{:?}", car);

    let point: Point = Point{x:190.2, y:201.3};
    let another_point: Point = Point{x: 3.2, y: 5.6};

    println!("point coordinates: ({}, {})", point.x, point.y);
    eprintln!("ERROR: tet");
    let bottom = Point{x: 4.23, ..another_point};

    println!("Another Point {:?}, {:?}", bottom.x, bottom.y);

}
