use std::collections::HashSet;

fn hash_sets(){
    let mut veri: HashSet<&str> = HashSet::new();    
    veri.insert("Bir");
    veri.insert("Iki");    
    println!("{:?}", veri);

    let eklendi_mi = veri.insert("Üç");
    if eklendi_mi {
        println!("Üç Eklendi");
    }
    println!("{:?}",veri);
    println!("--------------------------");
    veri.insert("Dört");
    if !veri.contains("Dört"){
        println!("Dört Yok");
    }else {
        println!("Dört var");
    }
    println!("--------------------------");
    println!("{:?}",veri);

}

fn main() {
    hash_sets();
}
