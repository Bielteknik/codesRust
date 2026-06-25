use std::mem; //  'ta bellek yönetimi için kullanılan bir moddur. Dizilerin bellek kullanımını incelemek için kullanılır.

fn arrays() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [6, 7, 8, 9, 0];
    println!("{:?}", a);
    println!("{}", a[0]);
    println!("a - b:{}", a.len() - b.len());
    println!("a + b:{}", a.len() + b.len());
    println!(
        "a dizisi {} elemana sahiptir ve ilk elemeanı: {}",
        a.len(),
        a[0]
    );

    let c = 3;
    let d = 1 + 2;
    assert_eq!(c, d);
    assert_eq!(c, d, "we are testing addition with {} and {}", c, d);

    let e = [1; 10]; // 10 elemanlı bir d dizisi oluşturur ve tüm elemanları 1 olarak başlatır.
    for i in 0..e.len() {
        println!("{}. elemanı : {}", i + 1, e[i]);
    }

    println!("------------------");

    let f: [i32; 10] = [77; 10];
    println!("{:?}", f); // f dizisinin tüm elemanlarını tek seferde yazdırır.
    println!("f dizisinin bellek boyutu {}", mem::size_of_val(&f)); // f dizisinin bellek boyutunu byte cinsinden yazdırır.

    println!("------------------");

    let g: [u32; 10] = [5u32; 10]; // 10 elemanlı bir g dizisi oluşturur ve tüm elemanları 5 olarak başlatır. Eleman türü u16 (16-bit unsigned integer) olarak belirtilmiştir.
    println!("{:?}", g); // g dizisinin tüm elemanlarını tek seferde yazdırır.
    println!("g dizisinin bellek boyutu {}", mem::size_of_val(&g)); // g dizisinin bellek boyutunu byte cinsinden yazdırır.
}

fn main() {
    arrays();
}
