fn use_slices(slices: &mut[i32]) {
    println!("İlk elemman: {}, ayrıca elaman sayısı {}", slices[0], slices.len());
    slices[0] = 3333;
    slices[1] = 4444;
    println!("Yeni ilk elemman: {}", slices[0]);
}
fn slices(){
    let mut data = [1,2,3,4,5];
    println!("{:?}", data);
    use_slices(&mut data[1..4]);
    println!("{:?}", data);
}

fn main() {
    slices();
}
