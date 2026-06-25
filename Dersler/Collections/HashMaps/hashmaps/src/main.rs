use std::collections::HashMap;

fn hash_maps(){

    let mut sekiller = HashMap::new();
    let ucgen = String::from("Üçgen");
    sekiller.insert(ucgen,3);
    sekiller.insert("kare".into(),4);
    for (k, v) in &sekiller {
        println!("{} nin {} kenarı vardır", k, v);
    }
    sekiller.insert("Daire".into(),0);
    println!("Bir Karenin {} kenarı vardır", sekiller["kare"]);
    println!("Bir Dairenin {} kenarı vardır", sekiller["Daire"]);
    println!("HashMap içindeki değerler {:?}", sekiller);
    println!("--------------------");
    sekiller.entry("Dikdortgen".into()).or_insert(123); //Varsa olduğu gibi bırak yoksa Oluştur ve 123 ekle
    println!("HashMap içindeki değerler {:?}", sekiller);

    {
        let suan = sekiller.entry("Yamuk".into()).or_insert(5); //Varsa olduğu gibi bırak yoksa Oluştur ve 5 ekle
        println!("Yamuk nin {} kenarı vardır", *suan);
    }
}

fn main() {
    hash_maps();
}