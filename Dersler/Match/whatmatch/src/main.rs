fn ne_kadar(x: i32) -> &'static str {

    match x {

    0 => "hiç",
  1 | 2 => "bir veya iki",
    12 =>  "bir düzine",
z @ 6..=8 => ("6 ile 8 arasında"),
_ if (x % 2 == 0) => "çift sayı",
_ => "diğer",
}
}

fn match_string(){

    for x in 0..13 {
        println!("{} benim  {} elmam var", x, ne_kadar(x));
    }

    let point = (0,0);

    match point {
        (0,0) => println!("Orijinde"),
        (0,y) => println!("x Ekseninde, y ={}", y),
        (ref x,0) => println!("y Ekseninde, x = {}", x),
        (_, y) => println!("(?,{}",y),
        _ => println!("Başka bir noktada"),
    }

}



fn main() {
     match_string();
}
