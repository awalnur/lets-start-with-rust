#![allow(dead_code)]

use std::f32::consts::PI;

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
struct Rectangle{
     // a rentangle can be specified by where the top left and botton right
     // corners are in space.
     top_left: Point,
     bottom_right: Point
 }


fn extract(a: i32, b:&i32)->i32{
    return a+b
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

    let bottom = Point{x: 4.23, ..another_point};
    println!("hasil penambahana: {}",extract(10, &18));
    println!("Another Point {:?}, {:?}", bottom.x, bottom.y);

    let _rectangle = Rectangle{top_left: Point{x: PI, y: PI}, bottom_right: Point{x: PI, y: PI}};
    println!("{:?}", _rectangle.bottom_right.x);
    println!("{:?}", 22/7);
}
