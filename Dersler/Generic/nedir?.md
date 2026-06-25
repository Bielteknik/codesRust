 'ta **Generic (Jenerik/Genelleme)**, tiplerin ve fonksiyonların **birden fazla veri tipiyle çalışabilmesini** sağlayan güçlü bir özelliktir. Kod tekrarını önler, tip güvenliğini korur ve sıfır maliyetle soyutlama (zero-cost abstraction) sağlar.

Diğer dillerdeki "template" (C++) veya "generic" (Java, C#) kavramlarına benzer, ancak  'ın **trait bounds** sistemi sayesinde çok daha güvenlidir.

İşte   Generic yapısının detaylı anlatımı:

---

## 1. Neden Generic'e İhtiyacımız Var?

Generic olmadan, farklı tiplerle çalışan fonksiyonlar için her tip için ayrı fonksiyon yazmanız gerekirdi:

``` 
// ❌ KÖTÜ: Kod tekrarı
fn en_buyuk_i32(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

fn en_buyuk_f64(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

fn en_buyuk_char(a: char, b: char) -> char {
    if a > b { a } else { b }
}
```

Generic ile **tek bir fonksiyon** yazarsınız:

``` 
// ✅ İYİ: Generic fonksiyon
fn en_buyuk<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

fn main() {
    println!("{}", en_buyuk(5, 10));        // i32
    println!("{}", en_buyuk(3.14, 2.71));   // f64
    println!("{}", en_buyuk('a', 'z'));     // char
}
```

---

## 2. Generic Fonksiyonlar

Generic fonksiyonlarda tip parametresi `<T>` şeklinde belirtilir. `T`, "Type" (Tip) anlamına gelir.

``` 
// T: Herhangi bir tip olabilir
fn ilk_elemani_yazdir<T>(liste: &[T]) {
    if let Some(ilk) = liste.first() {
        println!("İlk eleman: {:?}", ilk);
    }
}

fn main() {
    let sayilar = vec![1, 2, 3];
    let isimler = vec!["Ali", "Ayşe", "Mehmet"];
    
    ilk_elemani_yazdir(&sayilar);  // İlk eleman: 1
    ilk_elemani_yazdir(&isimler);  // İlk eleman: "Ali"
}
```

### Birden Fazla Tip Parametresi

``` 
fn karistir<T, U>(birinci: T, ikinci: U) -> (T, U) {
    (birinci, ikinci)
}

fn main() {
    let sonuc = karistir(42, "merhaba");
    println!("{:?}", sonuc); // (42, "merhaba")
    // T = i32, U = &str
}
```

---

## 3. Generic Struct'lar

Struct tanımlarında da generic kullanılabilir.

``` 
#[derive(Debug)]
struct Nokta<T> {
    x: T,
    y: T,
}

fn main() {
    let tam_sayili_nokta = Nokta { x: 5, y: 10 };       // T = i32
    let ondalikli_nokta = Nokta { x: 1.5, y: 4.3 };    // T = f64
    
    println!("{:?}", tam_sayili_nokta);
    println!("{:?}", ondalikli_nokta);
}
```

### Farklı Tiplerle

``` 
#[derive(Debug)]
struct Nokta<T, U> {
    x: T,
    y: U,
}

fn main() {
    let karisik_nokta = Nokta { x: 5, y: 4.3 };
    // T = i32, U = f64
    println!("{:?}", karisik_nokta);
}
```

---

## 4. Generic Enum'lar

Enum'lar da generic olabilir.  'ın standart kütüphanesindeki en önemli örnekler `Option` ve `Result`'tır:

``` 
//  'ın standart kütüphanesindeki gerçek tanımlar:
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Kendi generic enum'umuz:
#[derive(Debug)]
enum Komut<T> {
    Baslat(T),
    Durdur,
    Guncelle(T, T),
}

fn main() {
    let sayisal_komut: Komut<i32> = Komut::Baslat(10);
    let metinsel_komut: Komut<String> = Komut::Guncelle(
        String::from("eski"),
        String::from("yeni")
    );
    
    println!("{:?}", sayisal_komut);
    println!("{:?}", metinsel_komut);
}
```

---

## 5. Generic Metodlar

Struct'lar üzerinde generic metodlar tanımlayabilirsiniz.

``` 
#[derive(Debug)]
struct Cift<T> {
    birinci: T,
    ikinci: T,
}

impl<T> Cift<T> {
    // Tüm T tipleri için geçerli metod
    fn yeni(birinci: T, ikinci: T) -> Self {
        Cift { birinci, ikinci }
    }
}

// Sadece belirli tipler için metod (Trait Bound gerekli)
impl<T: std::fmt::Display + PartialOrd> Cift<T> {
    fn buyuk_olan(&self) -> &T {
        if self.birinci >= self.ikinci {
            &self.birinci
        } else {
            &self.ikinci
        }
    }
    
    fn yazdir(&self) {
        println!("{} ve {}", self.birinci, self.ikinci);
    }
}

fn main() {
    let sayi_cifti = Cift::yeni(5, 10);
    println!("Büyük olan: {}", sayi_cifti.buyuk_olan());
    sayi_cifti.yazdir();
    
    let string_cifti = Cift::yeni("elma", "armut");
    println!("Büyük olan: {}", string_cifti.buyuk_olan());
}
```

### impl<T> Sözdizimi

``` 
#[derive(Debug)]
struct Liste<T> {
    elemanlar: Vec<T>,
}

impl<T> Liste<T> {
    fn yeni() -> Self {
        Liste { elemanlar: Vec::new() }
    }
    
    fn ekle(&mut self, eleman: T) {
        self.elemanlar.push(eleman);
    }
    
    fn uzunluk(&self) -> usize {
        self.elemanlar.len()
    }
}

fn main() {
    let mut liste = Liste::yeni();
    liste.ekle(1);
    liste.ekle(2);
    println!("Uzunluk: {}", liste.uzunluk());
}
```

---

## 6. Monomorphization (Kod Üretimi) - Sıfır Maliyet

 , generic kodu **derleme zamanında** her kullanılan tip için ayrı ayrı kod üretir. Bu işleme **monomorphization** denir. Sonuç: **sıfır runtime maliyeti**.

``` 
// Yazdığınız generic kod:
fn en_buyuk<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

fn main() {
    let a = en_buyuk(5, 10);       // i32 için
    let b = en_buyuk(3.14, 2.71);  // f64 için
}

// Derleyicinin ürettiği gerçek kod (arkaplanda):
fn en_buyuk_i32(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

fn en_buyuk_f64(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}
```

**Avantajı:** Generic kullanmanın **hiçbir performans cezası yoktur**. Elle yazılmış spesifik kodla aynı hızda çalışır.

---

## 7. Trait Bounds (En Önemli Bağlı Konu)

Generic tipler üzerinde belirli işlemler yapabilmek için **trait bound** kullanmanız gerekir. `T: Trait` sözdizimi, "T tipi Trait'i implemente etmelidir" anlamına gelir.

### Neden Trait Bound Gerekir?

``` 
// ❌ HATA: T'nin karşılaştırılabileceğini bilmiyoruz
fn en_buyuk<T>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

// ✅ DOĞRU: T, PartialOrd trait'ini implemente etmeli
fn en_buyuk<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
```

### Yaygın Trait'ler ve Kullanımları

``` 
// Display: Yazdırılabilir
fn yazdir<T: std::fmt::Display>(deger: &T) {
    println!("Değer: {}", deger);
}

// Debug: Debug formatında yazdırılabilir
fn debug_yazdir<T: std::fmt::Debug>(deger: &T) {
    println!("Debug: {:?}", deger);
}

// Clone: Klonlanabilir
fn kopyala<T: Clone>(deger: &T) -> T {
    deger.clone()
}

// Default: Varsayılan değer üretebilir
fn varsayilan_olustur<T: Default>() -> T {
    T::default()
}

fn main() {
    yazdir(&42);
    debug_yazdir(&vec![1, 2, 3]);
    let kopya = kopyala(&String::from("merhaba"));
    let sayi: i32 = varsayilan_olustur(); // 0
}
```

### Birden Fazla Trait Bound

``` 
// Yöntem 1: + operatörü
fn islem<T: std::fmt::Display + Clone + PartialOrd>(a: T, b: T) {
    println!("a: {}, b: {}", a, b);
    if a > b {
        let _kopya = a.clone();
    }
}

// Yöntem 2: where clause (daha okunabilir)
fn islem2<T>(a: T, b: T)
where
    T: std::fmt::Display + Clone + PartialOrd,
{
    println!("a: {}, b: {}", a, b);
}

fn main() {
    islem(5, 10);
    islem2(5, 10);
}
```

---

## 8. where Clause

Trait bound'lar karmaşıklaştığında `where` clause kullanmak okunabilirliği artırır.

``` 
// ❌ Okunması zor
fn karmasik_fonksiyon<T: Display + Clone, U: Clone + Debug>(
    a: T, b: U
) -> T 
where 
    T: PartialEq,
{
    // ...
    a
}

// ✅ where clause ile daha temiz
fn karmasik_fonksiyon_temiz<T, U>(a: T, b: U) -> T
where
    T: Display + Clone + PartialEq,
    U: Clone + Debug,
{
    println!("{}", a);
    a
}

use std::fmt::{Display, Debug};
```

---

## 9. impl Trait Sözdizimi

Fonksiyon parametrelerinde ve dönüş tiplerinde `impl Trait` kullanılabilir.

``` 
// Bu iki fonksiyon eşdeğerdir:
fn yazdir1<T: Display>(deger: T) {
    println!("{}", deger);
}

fn yazdir2(deger: impl Display) {
    println!("{}", deger);
}

// Dönüş tipi olarak impl Trait
fn cift_sayi_olustur() -> impl Iterator<Item = i32> {
    (0..).step_by(2)
}

fn main() {
    yazdir1(42);
    yazdir2(42);
    
    for sayi in cift_sayi_olustur().take(5) {
        print!("{} ", sayi); // 0 2 4 6 8
    }
}
```

---

## 10. Lifetime ile Generic İlişkisi

Generic tipler ve lifetime'lar birlikte kullanılabilir.

``` 
#[derive(Debug)]
struct OnemliMetin<'a, T> {
    icerik: &'a T,
    onem_seviyesi: u8,
}

impl<'a, T: std::fmt::Display> OnemliMetin<'a, T> {
    fn yeni(icerik: &'a T, onem: u8) -> Self {
        OnemliMetin {
            icerik,
            onem_seviyesi: onem,
        }
    }
    
    fn yazdir(&self) {
        println!("[Önem: {}] {}", self.onem_seviyesi, self.icerik);
    }
}

fn main() {
    let mesaj = String::from("Sistem çöküyor!");
    let onemli = OnemliMetin::yeni(&mesaj, 5);
    onemli.yazdir();
}
```

---

## 11. Const Generics (Sabit Jenerikler)

  1.51+ ile **sabit (const) generic** parametreler de kullanılabilir. Bu, boyut gibi derleme zamanında bilinen değerler için kullanılır.

``` 
// N: Derleme zamanında bilinen bir sayı
#[derive(Debug)]
struct Matris<T, const N: usize> {
    veri: [[T; N]; N],
}

impl<T: Default + Copy, const N: usize> Matris<T, N> {
    fn sifir() -> Self {
        Matris {
            veri: [[T::default(); N]; N],
        }
    }
}

fn main() {
    let m3x3: Matris<i32, 3> = Matris::sifir();
    let m4x4: Matris<f64, 4> = Matris::sifir();
    
    println!("{:?}", m3x3);
    println!("{:?}", m4x4);
}
```

---

## 12. Turbofish Operatörü `::<>`

Bazen derleyici tipi otomatik algılayamaz. Bu durumlarda **turbofish** `::<>` operatörü ile tipi açıkça belirtirsiniz.

``` 
fn main() {
    // parse() metodu generic'tir, tipi belirtmek gerekir
    let sayi: i32 = "42".parse().unwrap();
    
    // Turbofish ile alternatif yazım:
    let sayi2 = "42".parse::<i32>().unwrap();
    
    // Vec::new() için
    let mut v = Vec::<i32>::new();
    v.push(1);
    
    // Option için
    let deger = None::<String>;
    
    println!("{} {} {:?}", sayi, sayi2, deger);
}
```

---

## 13. Standart Kütüphaneden Generic Örnekleri

 'ın standart kütüphanesi generic'lerle doludur:

``` 
fn main() {
    // Vec<T>
    let mut v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];  // vec! macro'su
    
    // Option<T>
    let bazi_deger: Option<i32> = Some(42);
    let bos_deger: Option<i32> = None;
    
    // Result<T, E>
    let basarili: Result<i32, String> = Ok(200);
    let hatali: Result<i32, String> = Err("Hata!".to_string());
    
    // HashMap<K, V>
    use std::collections::HashMap;
    let mut harita = HashMap::new();
    harita.insert("Ali", 25);
    harita.insert("Ayşe", 30);
    
    // Box<T> (Heap allocation)
    let kutu = Box::new(42);
    
    println!("{:?} {:?} {:?} {:?}", bazi_deger, basarili, harita, kutu);
}
```

---

## 14. Gerçek Dünya Örnekleri

### A. Generic Stack Yapısı

``` 
#[derive(Debug)]
struct Stack<T> {
    elemanlar: Vec<T>,
}

impl<T> Stack<T> {
    fn yeni() -> Self {
        Stack { elemanlar: Vec::new() }
    }
    
    fn push(&mut self, deger: T) {
        self.elemanlar.push(deger);
    }
    
    fn pop(&mut self) -> Option<T> {
        self.elemanlar.pop()
    }
    
    fn peek(&self) -> Option<&T> {
        self.elemanlar.last()
    }
    
    fn bos_mu(&self) -> bool {
        self.elemanlar.is_empty()
    }
    
    fn uzunluk(&self) -> usize {
        self.elemanlar.len()
    }
}

fn main() {
    let mut sayi_stack = Stack::yeni();
    sayi_stack.push(1);
    sayi_stack.push(2);
    sayi_stack.push(3);
    
    println!("Üst eleman: {:?}", sayi_stack.peek());
    println!("Pop: {:?}", sayi_stack.pop());
    
    let mut string_stack = Stack::yeni();
    string_stack.push(String::from("merhaba"));
    string_stack.push(String::from("dünya"));
    
    println!("String stack: {:?}", string_stack);
}
```

### B. Generic Cache

``` 
use std::collections::HashMap;
use std::hash::Hash;

struct Cache<T, U>
where
    T: Eq + Hash,
    U: Clone,
{
    hesaplayici: Box<dyn Fn(T) -> U>,
    degerler: HashMap<T, U>,
}

impl<T, U> Cache<T, U>
where
    T: Eq + Hash + Clone,
    U: Clone,
{
    fn yeni(hesaplayici: impl Fn(T) -> U + 'static) -> Self {
        Cache {
            hesaplayici: Box::new(hesaplayici),
            degerler: HashMap::new(),
        }
    }
    
    fn deger_al(&mut self, anahtar: T) -> U {
        if !self.degerler.contains_key(&anahtar) {
            let sonuc = (self.hesaplayici)(anahtar.clone());
            self.degerler.insert(anahtar.clone(), sonuc);
        }
        self.degerler.get(&anahtar).unwrap().clone()
    }
}

fn main() {
    let mut cache = Cache::yeni(|x: i32| x * x);
    
    println!("{}", cache.deger_al(4));  // Hesaplar: 16
    println!("{}", cache.deger_al(4));  // Cache'den alır: 16
    println!("{}", cache.deger_al(5));  // Hesaplar: 25
}
```

### C. Generic Wrapper

``` 
#[derive(Debug)]
struct Wrapper<T> {
    deger: T,
}

impl<T> Wrapper<T> {
    fn yeni(deger: T) -> Self {
        Wrapper { deger }
    }
    
    fn icerigi_al(self) -> T {
        self.deger
    }
}

impl<T: std::fmt::Display> Wrapper<T> {
    fn yazdir(&self) {
        println!("Wrapper içeriği: {}", self.deger);
    }
}

fn main() {
    let w1 = Wrapper::yeni(42);
    w1.yazdir();
    
    let w2 = Wrapper::yeni(String::from("Merhaba"));
    w2.yazdir();
    
    let deger = w1.icerigi_al();
    println!("Çıkarılan: {}", deger);
}
```

---

## 15. Sık Yapılan Hatalar

### ❌ Hata 1: Trait Bound Unutmak

``` 
// ❌ HATA
fn topla<T>(a: T, b: T) -> T {
    a + b  // T'nin + operatörünü desteklediğini bilmiyoruz
}

// ✅ DOĞRU
fn topla<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
```

### ❌ Hata 2: Farklı Tiplerde Aynı Struct

``` 
struct Nokta<T> {
    x: T,
    y: T,
}

fn main() {
    // ❌ HATA: x ve y farklı tipte olamaz
    let p = Nokta { x: 5, y: 4.3 };
    
    // ✅ DOĞRU: İki farklı generic tip
    let p2 = Nokta2 { x: 5, y: 4.3 };
}

struct Nokta2<T, U> {
    x: T,
    y: U,
}
```

### ❌ Hata 3: impl Block'ta Lifetime Unutmak

``` 
struct Metin<'a, T> {
    icerik: &'a T,
}

// ❌ HATA: impl block'ta lifetime belirtilmeli
impl<T> Metin<'_, T> {
    fn icerigi_al(&self) -> &T {
        self.icerik
    }
}

// ✅ DOĞRU
impl<'a, T> Metin<'a, T> {
    fn icerigi_al(&self) -> &T {
        self.icerik
    }
}
```

---

## 16. Karşılaştırma Tablosu

| Özellik | Generic | Trait Object (dyn Trait) |
|:---|:---|:---|
| **Tip Bilgisi** | Compile-time'da belli | Runtime'da belli |
| **Performans** | Sıfır maliyet (monomorphization) | Dinamik dispatch, küçük maliyet |
| **Binary Boyutu** | Büyük (her tip için kod üretilir) | Küçük (tek kod) |
| **Esneklik** | Tip derleme zamanında belli olmalı | Runtime'da tip değişebilir |
| **Kullanım** | Performans kritik yerler | Plugin sistemleri, dinamik davranış |

---

## Özet ve İpuçları

1. **Kod Tekrarını Önler:** Aynı mantığı farklı tipler için tekrar tekrar yazmayın.

2. **Tip Güvenliği:** Generic, tip güvenliğini korur. `Vec<i32>` içine `String` ekleyemezsiniz.

3. **Sıfır Maliyet:** Monomorphization sayesinde generic kullanmanın performans cezası yoktur.

4. **Trait Bounds Gerekli:** Generic tip üzerinde işlem yapacaksanız, o işlemin trait'ini bound olarak ekleyin.

5. **where Clause Kullanın:** Karmaşık trait bound'lar için `where` clause okunabilirliği artırır.

6. **impl Trait:** Fonksiyon parametrelerinde ve dönüş tiplerinde kısa yol olarak kullanılabilir.

7. **Const Generics:** Boyut gibi derleme zamanında bilinen değerler için `const N: usize` kullanın.

8. **Turbofish:** Tip algılanamadığında `::<>` operatörü ile açıkça belirtin.

9. **Standart Kütüphaneyi İnceleyin:** `Vec<T>`, `Option<T>`, `Result<T, E>` gibi yapılar generic'in en iyi örnekleridir.

10. **Lifetime ile Dikkatli:** Generic ve lifetime birlikte kullanıldığında sözdizimi karmaşıklaşır, dikkatli olun.

Generic,  'ın en güçlü özelliklerinden biridir. Doğru kullanıldığında hem kod tekrarını önler hem de tip güvenliğini korur. Trait bounds ile birlikte kullanıldığında ise  'ın tip sisteminin tüm gücünü ortaya çıkarır. Özellikle kendi veri yapılarınızı (Stack, Queue, LinkedList vb.) yazarken generic kullanmak neredeyse zorunludur.