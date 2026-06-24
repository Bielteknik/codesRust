/*
/// Matrisler Öğrenici Notları;///

Öğrenici Açıklaması: Bu kod, Rust'ta iki boyutlu dizilerin (matrislerin) temel işlemlerini gösterir:

    [[f32; 3]; 3] syntax'ı 3x3'luk bir f32 matrisi tanımlar
    mtx.len() dış boyut (satır sayısını) verir
    mtx[i].len() iç boyut (sütun sayısını) verir
    İç içe for döngüleri tüm elemanlara erişim sağlar
    i == j kontrolü köşegen elemanları seçer

*/
fn matris(){
    let mtx: [[f32; 3]; 3] = [[1.0,0.0,0.0], [0.0,1.0,0.0], [0.0,0.0,1.0]]; //3x3 Matris tanımlaması
    let mtx1: [[f32;3];2] = [
        [1.0,0.0,0.0],
        [0.0,1.0,0.0]
    ]; //2x3 Matris tanımlaması 
    println!("MTX Elemanlar: {:?} boyutu {}", mtx, mtx.len());
    println!("MTX1: {:?} boyutu {}", mtx1, mtx1.len());

    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            println!("MTX [{}][{}] = {}", i, j, mtx[i][j]);
        }
    }
    println!("-----------İndis Sayıları Eşit olanlar");

    for i in 0..mtx.len() {
        for j in 0..mtx[i].len(){
            if i == j {
                println!("MTX [{}][{}] = {}", i, j, mtx[i][j]);
            }
        }
    }
}

fn main() {
    matris();
}
