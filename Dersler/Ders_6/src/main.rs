#![allow(dead_code)]
#![allow(unused_variables)]
//#![allow(unused_imports)]

enum Color {
    Red,
    Green,
    Blue,
    RGBColor(u8, u8, u8), //tuple
    CMYKColor {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    }, //struct
} 
//enum'ler birden fazla türü tek bir tür altında tanımlamamızı sağlar. 
//enum'lar genellikle birbirleriyle ilişkili değerleri gruplamak için kullanılır. 
//enum'lar, Rust'ta güçlü bir şekilde tiplenmiş veri yapılarıdır ve genellikle match ifadeleriyle birlikte kullanılır.

fn enums() {
    //let c = Color::Blue;
    let c = Color::RGBColor(10, 20, 0);

    match c {
        Color::Red => println!("R"),
        Color::Green => println!("G"),
        Color::Blue => println!("B"),
        Color::RGBColor(0, 255, 0) => println!("RGBColor(0, 255, 0) or Green"), //tuple destructuring
        Color::RGBColor(0, 0, 0) | 
        Color::CMYKColor{cyan: _, magenta: _, yellow: _, black: 255} => println!("RGBColor(0, 0, 0) or CMYKColor(0, 255, 255, 0) or Black"), //tuple destructuring  
        Color::CMYKColor{cyan: 0, magenta: 0, yellow: 0, black: 0} => println!("CMYKColor(0, 0, 0, 0) or Black"),
        Color::RGBColor(r, g, b) => println!("RGBColor({}, {}, {})", r, g, b), //tuple destructuring
        _ => println!("Other color"),
    }
}

fn main() {
    enums();
}
