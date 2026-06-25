//Nedir
// Tuple, birden fazla değeri tek bir değişkende saklayan veri yapısıdır.
// Fonksiyonlardan birden fazla değer döndürmek için kullanılır.


fn topla_carp(x : i32, y: i32) -> (i32, i32) {
    (x + y, x * y)
}

fn tuples(){
    let x = 3;
    let y = 4;
    let (toplam, carpim) = topla_carp(x, y);
    println!("Toplam: {}, Çarpım: {}", toplam, carpim);
    let sonuclar = topla_carp(x, y);
    println!("Sonuçların çıkıtısı = Toplam: {}, Çarpım: {}", sonuclar.0, sonuclar.1);
    println!("Sonuçlar : {:?}", sonuclar);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sonuclar.0, sonuclar.1);
    let (toplam, carpim) = sonuclar;
    println!("Yeni sonuçlar: Toplam: {}, Çarpım: {}", toplam, carpim);
    println!("Deconstruct ile: Toplam: {}, Çarpım: {}", toplam, carpim);
    println!("-----------------------------");
    let sonuclar2 = topla_carp(4, 8);
    println!("Sonuçlar2 değerler : {}, {}", sonuclar2.0, sonuclar2.1);    
    let combine = (sonuclar, sonuclar2);
    println!("Tüm sonuçlar : {:?}", combine);
    println!("ilk eleman : {}", combine.0 .0);
    println!("ilk elemanın çarpımı : {}", combine.0 .1);
    println!("sonuncu eleman : {}", combine.1 .1);

    println!("-----------------------------");
    let elemanlar = (true, 3.14, "Merhaba, -119", false);
    println!("Elemanlar : {:?}", elemanlar);

    let (a, b, c, d) = elemanlar;
    println!("a: {}, b: {}, c: {}, d: {}", a, b, c, d);

    let elemanlar2 = (false,); //Tek elmanlı tuple için virgül koymak zorunludur.    
    println!("Elemanlar2 : {:?}", elemanlar2);
    let (d,) = elemanlar2;
    println!("d: {}", d);
}

fn main() {
    tuples();
}
