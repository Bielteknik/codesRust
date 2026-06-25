fn vektors(){
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    println!("a Vektörü = {:?}", a);
}

fn vektors_a(){
    let mut b = vec![1,2,3,4,5];
    println!("b Vektörü : {:?}", b);
    b.push(6);
    println!("Yeni ekleme sonrası B Vektörü: {:?}", b);
    println!("b vektörünün uzunluğu: {}", b.len());
    println!("b vektörünün kapasitesi :{}", b.capacity());
    println!("b vektörünün boş olup olduğu : {}", b.is_empty());
    println!("b vektörünün 2. elemanı: {}", b[1]);
    println!("b vektörünün son elemanı: {}", b[b.len() - 1]);

    match b.get(11){
        Some(x) => println!("b vektörünün 11. elemanı: {}", x),
        None => println!("b vektörünün 11. elemanı yoktur. En büyük index değeri: {}", b.len() - 1),
    }

    for x in &b {
        println!("b vektörünün elemanları: {}", x);
    }

    b.push(7);
    println!("Yeni eklenen eleman {}", b[b.len() - 1]);

    let son_eleman = b.pop();
    println!("Son eleman {}, b Vektörü: {:?}", son_eleman.unwrap(), b);

    let Some(_son_eleman) = b.pop() else {
        println!("b vektörü boş");
        return;
    };

    while let Some(_son_eleman) = b.pop(){
        println!("Son Eleman {}, b Vektörü : {:?}", _son_eleman, b);
    }

}


fn main() {
    println!("Hello, world!");
    vektors();
    vektors_a();
}
