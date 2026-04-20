fn arrays(){
    let a:[i32;5] = [1,2,3,4,5];
    let b = [6,7,8,9,0];
    println!("{:?}", a);
    println!("{}", a[0]);    
    println!("a - b:{}",a.len() - b.len());
    println!("a + b:{}",a.len() + b.len());
    println("a dizisi {} elemana sahiptir ve ilk elemeanı: {}", a.len(), a[0]);
}

fn main() {
    arrays();
}
