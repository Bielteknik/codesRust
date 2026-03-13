#[allow(dead_code)]
#[allow(unused_variables)]

fn if_statement() {
    let sicaklik = 5;
    if sicaklik > 35
    {
        println!("Çok sıcak");
    }
    else if sicaklik < 10 { println!("Hava Soğuk"); }
    else { println!("Hava Şartları İdeal"); }

    let gun = if sicaklik > 20 {"Güneşli"} else { "Bulutlu" };
    println!("Bugün {}", gun);
    println!(" Hava Durumu : {}", if sicaklik > 20 { "Sıcak" } else if sicaklik < 10  { " Soğuk" } else { "İyi" });
    println!( " Hava Durumu Özeti :{}", if sicaklik > 20 { if sicaklik > 30 { "Çok Sıcak"} else { " Sıcak" }} else if sicaklik < 10 { " Soğuk"} else {" İyi "});

}
fn while_and_loop(){
   let mut x = 1;
    while x < 1000 {
        x *= 2;
        if x == 128 { continue};
        println!("x {}",x);
    }
    println!(" loop konusu");
    loop {
        let mut y = 2;
        y *= 2;
        println!("Y = {}",y);
        if y == 128 {break};//1<<10 { break };
    }
}
fn for_loop(){
    for x in 1..11 {
        if x == 3 {continue};
        if x == 8 {break};
        println!("x değeri = {}", x);
    }
    println!(" --- enumerate ---");
    for (poz, y) in (50..61).enumerate(){
        println!("{} : {}", poz, y);
    }
    for (poz, z) in (100..111).enumerate() {
        println!("{} : {}", poz, z*2);
    }
}
fn match_case (){
    let ulke_kodu = 44;
    let ulke = match ulke_kodu{
        90=>"Türkiye",
        7=>"Russia",
        46=>"Sweden",
        44=>"UK",
        1..=1000=>"Unknown",
        _=>"Invalid"
    };
    println!("the country code {} is {}",ulke_kodu, ulke );
    let x = false;
    let deger = match x {
        false=>"Yanlış",
        true=>"Doğru",
        _=>"Tanımsız"
    };
    println!("{}", deger);

}

fn main() {
    //if_statement();
    //while_and_loop();
    //for_loop();
    match_case();
}
