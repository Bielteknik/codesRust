# 📚 Rust'ta Generics (Genel Tipler) - Kapsamlı Ders

Merhaba! Bu derste Rust'ın en güçlü özelliklerinden biri olan **Generics** konusunu Rust by Example'daki tüm alt başlıklarıyla birlikte ders anlatır gibi, detaylı ve kapsamlı şekilde işleyeceğiz. Hazırsanız başlayalım! 🚀

---

## 🎯 Bölüm 1: Generics Nedir? (Giriş)

### Temel Kavram

**Generics**, tipleri ve işlevsellikleri daha geniş durumlara genelleştirme konusudur. Bu, kod tekrarını azaltmak için son derece faydalıdır, ancak oldukça karmaşık bir sözdizimi gerektirebilir.

**Neden Generics'e İhtiyacımız Var?**

Şu senaryoyu düşünelim:

```rust
// ❌ Kod tekrarı var!
fn add_i32(a: i32, b: i32) -> i32 {
    a + b
}

fn add_f64(a: f64, b: f64) -> f64 {
    a + b
}

fn add_i64(a: i64, b: i64) -> i64 {
    a + b
}
```

Her tip için ayrı fonksiyon yazmak gereksiz! Generics ile bunu tek bir fonksiyona indirgeyebiliriz:

```rust
// ✅ Generic ile tek fonksiyon!
fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
```

### Tip Parametresi (Type Parameter)

Bir tip parametresi, **açılı ayraçlar (angle brackets)** ve **UpperCamelCase** kullanılarak belirtilir: `<Aaa, Bbb, ...>`

Genellikle "Generic tip parametreleri" `<T>` olarak temsil edilir.

```rust
// T, herhangi bir tipi temsil eden generic bir parametredir
fn foo<T>(arg: T) { 
    println!("Bir değer aldım!");
}

fn main() {
    foo(5);        // T = i32
    foo(3.14);     // T = f64
    foo("merhaba"); // T = &str
    foo(true);     // T = bool
}
```

**Önemli Not:** `<T>` ile belirtilen herhangi bir tip **generic**'tir, geri kalan her şey **concrete** (somut/non-generic) olarak adlandırılır.

---

## 🎯 Bölüm 2: Generic Fonksiyonlar (Functions)

Generic fonksiyonlar, farklı tiplerle çalışabilen fonksiyonlardır.

### Temel Yapı

```rust
// Genel sözdizimi:
fn fonksiyon_adi<T>(parametre: T) -> DonusTipi {
    // fonksiyon gövdesi
}
```

### Örnek 1: Basit Generic Fonksiyon

```rust
fn ekrana_yazdir<T>(deger: T) {
    println!("Değer: {:?}", deger);
}

fn main() {
    ekrana_yazdir(42);         // T = i32
    ekrana_yazdir("Merhaba");  // T = &str
    ekrana_yazdir(3.14);       // T = f64
    ekrana_yazdir(true);       // T = bool
}
```

### Örnek 2: Birden Fazla Tip Parametresi

```rust
fn karistir<T, U>(birinci: T, ikinci: U) {
    println!("Birinci: {:?}, İkinci: {:?}", birinci, ikinci);
}

fn main() {
    karistir(1, 'a');          // T = i32, U = char
    karistir("Rust", 3.14);    // T = &str, U = f64
}
```

### Örnek 3: Generic Fonksiyon Döndüren Fonksiyon

```rust
fn ilk_eleman<T>(liste: Vec<T>) -> Option<T> {
    if liste.is_empty() {
        None
    } else {
        Some(liste[0])  // Hata: move semantics nedeniyle
    }
}

// Daha doğru versiyon:
fn ilk_eleman_dogru<T>(liste: &[T]) -> Option<&T> {
    liste.first()
}

fn main() {
    let sayilar = vec![1, 2, 3, 4, 5];
    println!("İlk: {:?}", ilk_eleman_dogru(&sayilar)); // Some(1)
}
```

### Monomorphization (Tekilleştirme)

Rust, generic kodu **compile time**'da somut tiplere dönüştürür. Bu işleme **monomorphization** denir:

```rust
fn generic_fn<T>(x: T) {
    // ...
}

fn main() {
    generic_fn(5i32);    // Derleyici i32 için özel bir versiyon oluşturur
    generic_fn(5u8);     // Derleyici u8 için özel bir versiyon oluşturur
    // Arka planda iki farklı fonksiyon var!
}
```

**Avantajı:** Sıfır maliyetli soyutlama (zero-cost abstraction)! Runtime'da performans kaybı yok.

---

## 🎯 Bölüm 3: Generic Implementasyonlar (impl Blokları)

Struct ve enum'lar için generic `impl` blokları yazabiliriz.

### Temel Yapı

```rust
struct GenericVal<T>(T);  // Generic tip

// Somut tip için impl
impl GenericVal<f32> {}  // Sadece f32 için

impl GenericVal<u32> {}  // Sadece u32 için

// Generic impl - TÜM T'ler için
impl<T> GenericVal<T> {}
```

### Örnek 1: Generic Struct ve impl

```rust
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// Tüm T tipleri için impl
impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
    
    fn x(&self) -> &T {
        &self.x
    }
    
    fn y(&self) -> &T {
        &self.y
    }
}

// Sadece belirli bir tip için impl
impl Point<f64> {
    fn mesafe(&self, diger: &Point<f64>) -> f64 {
        let dx = self.x - diger.x;
        let dy = self.y - diger.y;
        (dx * dx + dy * dy).sqrt()
    }
}

fn main() {
    let tamsayi_nokta = Point::new(5, 10);
    let kesirli_nokta = Point::new(1.0, 4.0);
    let diger_nokta = Point::new(5.0, 8.0);
    
    println!("Tamsayı nokta: {:?}", tamsayi_nokta);
    println!("Mesafe: {}", kesirli_nokta.mesafe(&diger_nokta));
}
```

### Örnek 2: Birden Fazla Tip Parametresi ile impl

```rust
#[derive(Debug)]
struct Cift<T, U> {
    birinci: T,
    ikinci: U,
}

impl<T, U> Cift<T, U> {
    fn yeni(birinci: T, ikinci: U) -> Self {
        Cift { birinci, ikinci }
    }
}

// T ve U farklı olduğunda özel impl
impl<T, U> Cift<T, U> 
where 
    T: std::fmt::Display, 
    U: std::fmt::Display 
{
    fn yazdir(&self) {
        println!("Birinci: {}, İkinci: {}", self.birinci, self.ikinci);
    }
}

fn main() {
    let cift = Cift::yeni(1, 'a');
    cift.yazdir(); // Birinci: 1, İkinci: a
}
```

### Örnek 3: Enum ile Generic impl

```rust
#[derive(Debug)]
enum Sonuc<T, E> {
    Basarili(T),
    Hata(E),
}

impl<T, E> Sonuc<T, E> {
    fn basarili_mi(&self) -> bool {
        match self {
            Sonuc::Basarili(_) => true,
            Sonuc::Hata(_) => false,
        }
    }
}

fn main() {
    let iyi: Sonuc<i32, String> = Sonuc::Basarili(42);
    let kotu: Sonuc<i32, String> = Sonuc::Hata("Hata!".to_string());
    
    println!("İyi başarılı mı? {}", iyi.basarili_mi()); // true
    println!("Kötü başarılı mı? {}", kotu.basarili_mi()); // false
}
```

---

## 🎯 Bölüm 4: Generic Trait'ler

Trait'ler de generic olabilir. Bu, trait'lerin farklı tipler için farklı şekillerde davranmasını sağlar.

### Temel Yapı

```rust
trait GenericTrait<T> {
    fn bir_sey_yap(&self, deger: T) -> T;
}
```

### Örnek 1: Generic Trait Tanımlama ve Kullanma

```rust
trait Toplanabilir<T> {
    fn topla(&self, diger: T) -> T;
}

// i32 için implementasyon
impl Toplanabilir<i32> for i32 {
    fn topla(&self, diger: i32) -> i32 {
        self + diger
    }
}

// f64 için implementasyon
impl Toplanabilir<f64> for f64 {
    fn topla(&self, diger: f64) -> f64 {
        self + diger
    }
}

fn main() {
    let a: i32 = 5;
    let b: i32 = 10;
    println!("Toplam: {}", a.topla(b)); // 15
    
    let x: f64 = 2.5;
    let y: f64 = 3.7;
    println!("Toplam: {}", x.topla(y)); // 6.2
}
```

### Örnek 2: Generic Trait ile Drop Yeniden Tanımlama

```rust
use std::fmt::Display;

trait GenericDrop<T: Display> {
    fn generic_drop(self, deger: T);
}

struct Yapilandirma;

impl<T: Display> GenericDrop<T> for Yapilandirma {
    fn generic_drop(self, deger: T) {
        println!("Drop ediliyor: {}", deger);
    }
}

fn main() {
    let y = Yapilandirma;
    y.generic_drop(42);
    y.generic_drop("Merhaba");
}
```

### Örnek 3: Struct İçin Generic Trait

```rust
trait Kutu<T> {
    fn icerigi_al(&self) -> &T;
    fn icerigi_degistir(&mut self, yeni: T);
}

#[derive(Debug)]
struct Sandik<T> {
    deger: T,
}

impl<T> Kutu<T> for Sandik<T> {
    fn icerigi_al(&self) -> &T {
        &self.deger
    }
    
    fn icerigi_degistir(&mut self, yeni: T) {
        self.deger = yeni;
    }
}

fn main() {
    let mut sandik = Sandik { deger: 10 };
    println!("İçerik: {}", sandik.icerigi_al()); // 10
    
    sandik.icerigi_degistir(20);
    println!("Yeni içerik: {}", sandik.icerigi_al()); // 20
}
```

---

## 🎯 Bölüm 5: Trait Bounds (Sınırlar)

Generic tiplerle çalışırken, tip parametrelerinin hangi trait'leri uygulaması gerektiğini belirtmek için **trait bounds** kullanırız.

### Neden Bounds'a İhtiyacımız Var?

```rust
// ❌ Bu çalışmaz! Çünkü T'nin Display trait'ini uygulayıp 
// uygulamadığını bilmiyoruz
fn yazdir<T>(deger: T) {
    println!("{}", deger); // HATA!
}
```

### Örnek 1: Trait Bound ile Fonksiyon

```rust
use std::fmt::Display;

// T, Display trait'ini uygulamalı
fn yazdir<T: Display>(deger: T) {
    println!("{}", deger);
}

fn main() {
    yazdir(42);         // ✅ i32 Display implement eder
    yazdir("Merhaba");  // ✅ &str Display implement eder
    yazdir(3.14);       // ✅ f64 Display implement eder
    // yazdir(vec![1, 2, 3]); // ❌ Vec Display implement ETMEZ!
}
```

### Örnek 2: Multiple Bounds (Birden Fazla Sınır)

```rust
use std::fmt::{Display, Debug};

// T hem Display hem de Debug trait'lerini uygulamalı
fn yazdir_ve_debug<T: Display + Debug>(deger: T) {
    println!("Display: {}", deger);
    println!("Debug: {:?}", deger);
}

fn main() {
    yazdir_ve_debug(42);
    // Output:
    // Display: 42
    // Debug: 42
}
```

### Örnek 3: Struct ile Trait Bounds

```rust
use std::fmt::Display;

// T, Display trait'ini uygulamalı
struct Etiketli<T: Display> {
    icerik: T,
}

impl<T: Display> Etiketli<T> {
    fn yeni(icerik: T) -> Self {
        Etiketli { icerik }
    }
    
    fn yazdir(&self) {
        println!("Etiket: {}", self.icerik);
    }
}

fn main() {
    let etiket = Etiketli::yeni("Rust Harika!");
    etiket.yazdir();
    
    // ❌ AŞAĞIDAKİ ÇALIŞMAZ!
    // let hata = Etiketli::yeni(vec![1, 2, 3]); // Vec Display implement etmez
}
```

### Örnek 4: Trait Bound ile Method Erişimi

```rust
use std::fmt::Display;

trait Ozetlenebilir {
    fn ozet(&self) -> String;
}

struct Makale {
    baslik: String,
    icerik: String,
}

impl Ozetlenebilir for Makale {
    fn ozet(&self) -> String {
        format!("{}: {}...", self.baslik, &self.icerik[..20])
    }
}

// T, Ozetlenebilir trait'ini uygulamalı
fn ozeti_yazdir<T: Ozetlenebilir>(oge: T) {
    println!("Özet: {}", oge.ozet()); // Trait method'una erişebiliriz!
}

fn main() {
    let makale = Makale {
        baslik: "Rust Generics".to_string(),
        icerik: "Generics, Rust'ın güçlü özelliklerinden biridir...".to_string(),
    };
    
    ozeti_yazdir(makale);
}
```

### Örnek 5: impl Trait Sözdizimi (Alternatif)

```rust
use std::fmt::Display;

// Bu iki fonksiyon eşdeğerdir:
fn yazdir_1<T: Display>(deger: T) {
    println!("{}", deger);
}

// impl trait sözdizimi (daha kısa)
fn yazdir_2(deger: impl Display) {
    println!("{}", deger);
}

fn main() {
    yazdir_1(42);
    yazdir_2(42);
}
```

---

## 🎯 Bölüm 6: Birden Fazla Tip Parametresi (Multiple Types)

Generic yapılar birden fazla tip parametresi alabilir.

### Örnek 1: İki Tip Parametreli Struct

```rust
#[derive(Debug)]
struct Cift<T, U> {
    birinci: T,
    ikinci: U,
}

impl<T, U> Cift<T, U> {
    fn yeni(birinci: T, ikinci: U) -> Self {
        Cift { birinci, ikinci }
    }
    
    fn birinci_al(&self) -> &T {
        &self.birinci
    }
    
    fn ikinci_al(&self) -> &U {
        &self.ikinci
    }
}

fn main() {
    let cift = Cift::yeni(1, 'a');
    println!("Birinci: {}", cift.birinci_al()); // 1
    println!("İkinci: {}", cift.ikinci_al());   // a
    
    let baska_cift = Cift::yeni("Rust", 3.14);
    println!("Birinci: {}", baska_cift.birinci_al()); // Rust
    println!("İkinci: {}", baska_cift.ikinci_al());   // 3.14
}
```

### Örnek 2: Aynı Tipli Çiftler İçin Özel impl

```rust
#[derive(Debug)]
struct Cift<T, U> {
    birinci: T,
    ikinci: U,
}

// Sadece T == U olduğunda çalışan impl
impl<T> Cift<T, T> {
    fn esit_mi(&self) -> bool 
    where 
        T: PartialEq 
    {
        self.birinci == self.ikinci
    }
    
    fn buyuk_olan(&self) -> &T 
    where 
        T: PartialOrd 
    {
        if self.birinci >= self.ikinci {
            &self.birinci
        } else {
            &self.ikinci
        }
    }
}

fn main() {
    let cift1 = Cift { birinci: 5, ikinci: 10 };
    println!("Eşit mi? {}", cift1.esit_mi()); // false
    println!("Büyük olan: {}", cift1.buyuk_olan()); // 10
    
    let cift2 = Cift { birinci: 7, ikinci: 7 };
    println!("Eşit mi? {}", cift2.esit_mi()); // true
}
```

### Örnek 3: Enum ile Birden Fazla Tip

```rust
#[derive(Debug)]
enum Either<T, U> {
    Sol(T),
    Sag(U),
}

impl<T, U> Either<T, U> {
    fn sol(deger: T) -> Self {
        Either::Sol(deger)
    }
    
    fn sag(deger: U) -> Self {
        Either::Sag(deger)
    }
}

fn main() {
    let sol: Either<i32, String> = Either::sol(42);
    let sag: Either<i32, String> = Either::sag("Merhaba".to_string());
    
    println!("Sol: {:?}", sol);
    println!("Sağ: {:?}", sag);
}
```

---

## 🎯 Bölüm 7: Where Clause

Karmaşık bound'lar için `where` clause kullanmak kodu daha okunabilir hale getirir.

### Sorun: Karmaşık Bounds

```rust
// ❌ Okunması zor!
fn karmaşık_fn<T: Display + Clone, U: Clone + Debug>(t: T, u: U) {
    // ...
}
```

### Çözüm: Where Clause

```rust
// ✅ Çok daha okunabilir!
fn karmaşık_fn<T, U>(t: T, u: U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // ...
}
```

### Örnek 1: Basit Where Clause

```rust
use std::fmt::{Display, Debug};

fn bilgileri_yazdir<T, U>(t: T, u: U)
where
    T: Display,
    U: Debug,
{
    println!("T: {}", t);
    println!("U: {:?}", u);
}

fn main() {
    bilgileri_yazdir(42, "Merhaba");
    // Output:
    // T: 42
    // U: "Merhaba"
}
```

### Örnek 2: Struct ile Where Clause

```rust
use std::fmt::Display;

struct Kisi<T>
where
    T: Display,
{
    isim: T,
}

impl<T> Kisi<T>
where
    T: Display,
{
    fn yeni(isim: T) -> Self {
        Kisi { isim }
    }
    
    fn tanit(&self) {
        println!("Benim adım {}", self.isim);
    }
}

fn main() {
    let kisi = Kisi::yeni("Ahmet");
    kisi.tanit(); // Benim adım Ahmet
}
```

### Örnek 3: Where Clause ile Trait Bounds

```rust
trait Bilgi {
    fn bilgi_ver(&self) -> String;
}

struct Ogrenci {
    ad: String,
    numara: u32,
}

impl Bilgi for Ogrenci {
    fn bilgi_ver(&self) -> String {
        format!("{} - {}", self.ad, self.numara)
    }
}

fn bilgi_yazdir<T>(oge: T)
where
    T: Bilgi,
{
    println!("{}", oge.bilgi_ver());
}

fn main() {
    let ogrenci = Ogrenci {
        ad: "Ayşe".to_string(),
        numara: 12345,
    };
    
    bilgi_yazdir(ogrenci); // Ayşe - 12345
}
```

### Örnek 4: Where Clause ile Referanslar

```rust
use std::fmt::Display;

fn yazdir_ref<T>(deger: &T)
where
    T: Display + ?Sized, // ?Sized: Boyutu bilinmeyen tipler için
{
    println!("{}", deger);
}

fn main() {
    yazdir_ref(&42);
    yazdir_ref(&"Merhaba");
    yazdir_ref(&[1, 2, 3]);
}
```

### Ne Zaman Where Clause Kullanmalıyız?

1. **Birden fazla tip parametresi** olduğunda
2. **Karmaşık trait bound'lar** olduğunda
3. **Associated type'lar** olduğunda
4. **Kod okunabilirliği** için

---

## 🎯 Bölüm 8: New Type Idiom (Yeni Tip Deyimi)

**New Type Idiom**, mevcut bir tip etrafında yeni bir tip sarmalayıcı oluşturma tekniğidir. Bu, type-safety (tip güvenliği) sağlamak ve farklı davranışlar eklemek için kullanılır.

### Neden İhtiyacımız Var?

```rust
// ❌ Sorun: Metre ve Milimetre aynı tip!
fn mesafe_yazdir(m: u32) {
    println!("{} metre", m);
}

fn main() {
    let metre = 5;
    let milimetre = 5000;
    
    mesafe_yazdir(milimetre); // Derleyici hata vermez ama mantıksız!
}
```

### Örnek 1: New Type ile Tip Güvenliği

```rust
// ✅ New Type Idiom ile çözüm
struct Metre(u32);
struct Milimetre(u32);
struct Saniye(u32);

impl Metre {
    fn new(deger: u32) -> Self {
        Metre(deger)
    }
    
    fn milimetre_ol(&self) -> Milimetre {
        Milimetre(self.0 * 1000)
    }
}

impl std::fmt::Display for Metre {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} metre", self.0)
    }
}

impl std::fmt::Display for Milimetre {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} milimetre", self.0)
    }
}

fn mesafe_yazdir(m: Metre) {
    println!("{}", m);
}

fn main() {
    let m = Metre::new(5);
    let mm = Milimetre(5000);
    
    mesafe_yazdir(m);     // ✅ 5 metre
    // mesafe_yazdir(mm); // ❌ HATA! Milimetre, Metre değil!
    
    println!("{} = {}", m, m.milimetre_ol());
}
```

### Örnek 2: Trait Implementasyonu İçin New Type

```rust
// Foreign trait'i foreign tip için implement etmek istiyoruz
// Rust buna izin vermez (orphan rule)
// Ama new type ile çözebiliriz!

struct Sayi(Vec<i32>);

impl std::fmt::Display for Sayi {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let elemanlar: Vec<String> = self.0.iter()
            .map(|x| x.to_string())
            .collect();
        write!(f, "[{}]", elemanlar.join(", "))
    }
}

fn main() {
    let sayi = Sayi(vec![1, 2, 3, 4, 5]);
    println!("{}", sayi); // [1, 2, 3, 4, 5]
}
```

### Örnek 3: New Type ile Method Ekleme

```rust
struct Metin(String);

impl Metin {
    fn yeni(metin: &str) -> Self {
        Metin(metin.to_string())
    }
    
    fn buyuk_harf(&self) -> Self {
        Metin(self.0.to_uppercase())
    }
    
    fn kelime_sayisi(&self) -> usize {
        self.0.split_whitespace().count()
    }
    
    fn icerir_mi(&self, aranan: &str) -> bool {
        self.0.contains(aranan)
    }
}

impl std::fmt::Display for Metin {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    let metin = Metin::yeni("Rust programlama dili harika");
    
    println!("{}", metin.buyuk_harf());
    println!("Kelime sayısı: {}", metin.kelime_sayisi());
    println!("'harika' içeriyor mu? {}", metin.icerir_mi("harika"));
}
```

---

## 🎯 Bölüm 9: Associated Items (İlişkili Öğeler)

**Associated items**, trait'ler veya impl blokları içinde tanımlanan öğelerdir. Üç çeşidi vardır:

1. **Associated Constants** (İlişkili Sabitler)
2. **Associated Functions** (İlişkili Fonksiyonlar)
3. **Associated Types** (İlişkili Tipler)

### Örnek 1: Associated Constants

```rust
trait Yapilandirici {
    const MAX_BOYUT: u32;
    const MIN_BOYUT: u32;
    
    fn boyut_kontrol(&self, boyut: u32) -> bool;
}

struct Sunucu;

impl Yapilandirici for Sunucu {
    const MAX_BOYUT: u32 = 1000;
    const MIN_BOYUT: u32 = 10;
    
    fn boyut_kontrol(&self, boyut: u32) -> bool {
        boyut >= Self::MIN_BOYUT && boyut <= Self::MAX_BOYUT
    }
}

fn main() {
    let sunucu = Sunucu;
    println!("Max boyut: {}", Sunucu::MAX_BOYUT);
    println!("Min boyut: {}", Sunucu::MIN_BOYUT);
    println!("50 geçerli mi? {}", sunucu.boyut_kontrol(50));
}
```

### Örnek 2: Associated Functions (Static Methods)

```rust
struct Dikdortgen {
    genislik: f64,
    yukseklik: f64,
}

impl Dikdortgen {
    // Associated function (constructor)
    fn yeni(genislik: f64, yukseklik: f64) -> Self {
        Dikdortgen { genislik, yukseklik }
    }
    
    // Kare oluşturan associated function
    fn kare(kenar: f64) -> Self {
        Dikdortgen {
            genislik: kenar,
            yukseklik: kenar,
        }
    }
    
    // Instance method
    fn alan(&self) -> f64 {
        self.genislik * self.yukseklik
    }
    
    fn cevre(&self) -> f64 {
        2.0 * (self.genislik + self.yukseklik)
    }
}

fn main() {
    let d1 = Dikdortgen::yeni(5.0, 10.0);
    let d2 = Dikdortgen::kare(4.0);
    
    println!("D1 alanı: {}", d1.alan());     // 50
    println!("D2 alanı: {}", d2.alan());     // 16
    println!("D1 çevresi: {}", d1.cevre());  // 30
}
```

### Örnek 3: Associated Types

**Associated types**, trait içinde bir tip placeholder olarak tanımlanır. Bu, trait implementasyonunda somut bir tip belirtilmesini sağlar.

```rust
trait Konteyner {
    type Oge; // Associated type
    
    fn icerik(&self) -> Vec<Self::Oge>;
    fn ekle(&mut self, oge: Self::Oge);
}

struct SayiKutusu {
    sayilar: Vec<i32>,
}

impl Konteyner for SayiKutusu {
    type Oge = i32; // Burada somut tipi belirliyoruz
    
    fn icerik(&self) -> Vec<Self::Oge> {
        self.sayilar.clone()
    }
    
    fn ekle(&mut self, oge: Self::Oge) {
        self.sayilar.push(oge);
    }
}

fn main() {
    let mut kutu = SayiKutusu { sayilar: vec![1, 2, 3] };
    kutu.ekle(4);
    kutu.ekle(5);
    
    println!("İçerik: {:?}", kutu.icerik());
}
```

### Örnek 4: Generic Associated Types (GATs)

```rust
trait Iterator2 {
    type Oge;
    
    fn sonraki(&mut self) -> Option<Self::Oge>;
}

struct Sayac {
    deger: u32,
    max: u32,
}

impl Iterator2 for Sayac {
    type Oge = u32;
    
    fn sonraki(&mut self) -> Option<Self::Oge> {
        if self.deger < self.max {
            let sonuc = self.deger;
            self.deger += 1;
            Some(sonuc)
        } else {
            None
        }
    }
}

fn main() {
    let mut sayac = Sayac { deger: 0, max: 5 };
    
    while let Some(deger) = sayac.sonraki() {
        print!("{} ", deger);
    }
    // Output: 0 1 2 3 4
}
```

---

## 🎯 Bölüm 10: Phantom Type Parameters (Hayalet Tip Parametreleri)

**Phantom type parameter**, runtime'da görünmeyen ama compile-time'da tip kontrolü için kullanılan bir tip parametresidir.

### Neden İhtiyacımız Var?

Bazen bir struct'ın içinde bir tipi saklamak istemeyiz ama tip sisteminde o tipin var olmasını isteriz. Bu durumda `PhantomData` kullanırız.

### Örnek 1: Temel PhantomData Kullanımı

```rust
use std::marker::PhantomData;

// Birim dönüşümü için phantom type
struct Birim<T> {
    deger: f64,
    _birim: PhantomData<T>, // T'yi saklamıyoruz, sadece tip sisteminde var!
}

// Birim marker'ları (boş struct'lar)
struct Metre;
struct Kilometre;
struct Saniye;

impl Birim<Metre> {
    fn yeni(deger: f64) -> Self {
        Birim {
            deger,
            _birim: PhantomData,
        }
    }
}

impl Birim<Kilometre> {
    fn yeni(deger: f64) -> Self {
        Birim {
            deger,
            _birim: PhantomData,
        }
    }
}

impl std::fmt::Display for Birim<Metre> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} m", self.deger)
    }
}

impl std::fmt::Display for Birim<Kilometre> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} km", self.deger)
    }
}

fn main() {
    let m = Birim::<Metre>::yeni(100.0);
    let km = Birim::<Kilometre>::yeni(1.0);
    
    println!("{}", m);  // 100 m
    println!("{}", km); // 1 km
    
    // let hata: Birim<Metre> = km; // ❌ HATA! Kilometre != Metre
}
```

### Örnek 2: PhantomData ile Tip Güvenliği

```rust
use std::marker::PhantomData;

// Durum makinesi için phantom types
struct Durum<T> {
    _state: PhantomData<T>,
}

// Durum marker'ları
struct Kilitli;
struct Acik;
struct Beklemede;

impl Durum<Kilitli> {
    fn yeni() -> Self {
        Durum { _state: PhantomData }
    }
    
    fn kilidi_ac(self) -> Durum<Acik> {
        println!("Kilit açıldı!");
        Durum { _state: PhantomData }
    }
}

impl Durum<Acik> {
    fn kilitle(self) -> Durum<Kilitli> {
        println!("Kilitlendi!");
        Durum { _state: PhantomData }
    }
    
    fn beklemeye_al(self) -> Durum<Beklemede> {
        println!("Beklemeye alındı!");
        Durum { _state: PhantomData }
    }
}

impl Durum<Beklemede> {
    fn devam_et(self) -> Durum<Acik> {
        println!("Devam ediyor!");
        Durum { _state: PhantomData }
    }
}

fn main() {
    let kilitli = Durum::<Kilitli>::yeni();
    let acik = kilitli.kilidi_ac();
    let beklemede = acik.beklemeye_al();
    let tekrar_acik = beklemede.devam_et();
    
    // Derleyici geçersiz durum geçişlerini engeller!
    // let hata = kilitli.beklemeye_al(); // ❌ HATA!
}
```

### Örnek 3: PhantomData ve Lifetime

```rust
use std::marker::PhantomData;

// Bir referansı "ödünç alan" ama saklamayan bir struct
struct OduncAl<'a, T> {
    _veri: PhantomData<&'a T>,
    id: u32,
}

impl<'a, T> OduncAl<'a, T> {
    fn yeni(id: u32) -> Self {
        OduncAl {
            _veri: PhantomData,
            id,
        }
    }
}

fn main() {
    let sayi = 42;
    let odunc = OduncAl::<i32>::yeni(1);
    
    println!("Ödünç alma ID: {}", odunc.id);
    println!("Sayı hala geçerli: {}", sayi);
}
```

### PhantomData Ne Zaman Kullanılır?

1. **Tip güvenliği** sağlamak için (birim dönüşümleri, durum makineleri)
2. **Variance** kontrolü için (covariant, contravariant, invariant)
3. **Drop check** davranışını kontrol etmek için
4. **Lifetime** ilişkilerini belirtmek için

---

## 🎯 Özet ve En İyi Pratikler

### Generics'in Gücü

✅ **Kod tekrarını azaltır**
✅ **Tip güvenliğini artırır**
✅ **Sıfır maliyetli soyutlama** (zero-cost abstraction)
✅ **Esnek ve yeniden kullanılabilir kod**

### En İyi Pratikler

1. **İsimlendirme:**
   - Tek tip parametresi: `T` (Type)
   - Birden fazla: `T, U, V` veya anlamlı isimler: `K, V` (Key, Value)
   - Trait bound'lar için anlamlı isimler: `T: Iterator` yerine `I: Iterator`

2. **Bounds Kullanımı:**
   - Sadece ihtiyaç duyulan trait'leri bound olarak ekleyin
   - `where` clause ile okunabilirliği artırın
   - `impl Trait` sözdizimini fonksiyon parametrelerinde kullanın

3. **New Type Idiom:**
   - Type-safety için kullanın
   - Foreign trait'leri foreign type'lara implement etmek için kullanın
   - Domain-specific method'lar eklemek için kullanın

4. **PhantomData:**
   - Sadece gerçekten ihtiyaç duyulduğunda kullanın
   - Tip güvenliğini artırmak için güçlü bir araçtır

### Sık Yapılan Hatalar

❌ **Gereksiz bounds eklemek:**
```rust
// ❌ Kötü
fn foo<T: Display + Clone + Debug + Default + Send + Sync>(t: T) { }

// ✅ İyi - Sadece ihtiyaç duyulan
fn foo<T: Display>(t: T) { }
```

❌ **Concrete tip yerine generic kullanmamak:**
```rust
// ❌ Kod tekrarı
fn toplam_i32(a: i32, b: i32) -> i32 { a + b }
fn toplam_f64(a: f64, b: f64) -> f64 { a + b }

// ✅ Generic
fn toplam<T: std::ops::Add<Output = T>>(a: T, b: T) -> T { a + b }
```

---

## 🎓 Sonraki Adımlar

Generics konusunda ustalaşmak için:

1. **Pratik yapın:** Kendi generic struct ve trait'lerinizi yazın
2. **Standart kütüphaneyi inceleyin:** `Option<T>`, `Result<T, E>`, `Vec<T>` gibi tiplerin kaynak kodunu okuyun
3. **Trait bounds ile deneyler yapın:** Farklı kombinasyonlar deneyin
4. **PhantomData ile oynayın:** Tip güvenliğini artıran örnekler yazın

---

## 📚 Kaynaklar

- [Rust By Example - Generics](https://doc.rust-lang.org/rust-by-example/generics.html)
- [The Rust Book - Generic Types, Traits, and Lifetimes](https://doc.rust-lang.org/book/ch10-00-generics.html)
- [Rust Documentation - std::marker::PhantomData](https://doc.rust-lang.org/std/marker/struct.PhantomData.html)

---

Bu dersin sonunda Rust generics konusunu kapsamlı bir şekilde öğrendiniz! Artık kendi generic yapılarınızı oluşturabilir, trait bounds'ları etkili bir şekilde kullanabilir ve phantom type parameters ile tip güvenliğini artırabilirsiniz. 

**Mutlu kodlamalar! 🦀✨**

# DeepSeek Yorumu --------------

## Generics (Genel Türler) - Rust By Example

Rust'ta **generics** (genel türler), türleri ve işlevleri daha geniş durumlara genelleştirme konusudur. Bu, kod tekrarını azaltmak için son derece kullanışlıdır, ancak bazen karmaşık bir sözdizimi gerektirebilir. Generic yapılar kullanılırken, hangi türler üzerinde geçerli olduklarını dikkatle belirtmek gerekir.

---

### 1. Tür Parametreleri (Type Parameters)

Generics'in en basit ve en yaygın kullanımı **tür parametreleridir**. Bir tür parametresi, açılı parantezler (`<>`) ve CamelCase yazımı ile belirtilir.

```rust
// Somut (concrete) bir tür
struct A;

// `A` türünü kullanan somut bir yapı
struct Single(A);            // Generic DEĞİL

// `T` ile generic bir yapı
struct SingleGen<T>(T);      // Generic
```

`SingleGen<T>` generic bir türdür çünkü `T` bir tür parametresidir ve herhangi bir türü temsil edebilir.

```rust
fn main() {
    let _s = Single(A);                    // Somut
    let _char: SingleGen<char> = SingleGen('a'); // Açıkça belirtilmiş
    let _t = SingleGen(A);                 // Tür çıkarımı ile
    let _i32 = SingleGen(6);               // i32 çıkarımı
    let _char = SingleGen('a');            // char çıkarımı
}
```

Rust'ta "generic" terimi, bir veya daha fazla generic tür parametresi kabul eden her şeyi tanımlar. Tür parametresi olarak belirtilen her tür generic'tir; geri kalan her şey **somut (concrete)** türdür.

---

### 2. Generic Fonksiyonlar (Generic Functions)

Aynı kurallar fonksiyonlar için de geçerlidir: Bir `T` türü, başına `<>` konulduğunda generic olur.

```rust
struct A;
struct S(A);
struct SGen<T>(T);

// Generic OLMAYAN fonksiyonlar
fn reg_fn(_s: S) {}

// `A` özel olarak belirtilmiş, ama generic değil
fn gen_spec_t(_s: SGen<A>) {}

// `i32` özel olarak belirtilmiş, generic değil
fn gen_spec_i32(_s: SGen<i32>) {}

// GERÇEK generic fonksiyon: herhangi bir T ile çalışır
fn generic<T>(_s: SGen<T>) {}
```

Generic fonksiyonlar bazen tür parametrelerinin açıkça belirtilmesini gerektirir. Bu, fonksiyonun dönüş türü generic olduğunda veya derleyici tür parametrelerini çıkarımlamak için yeterli bilgiye sahip olmadığında geçerlidir.

```rust
fn main() {
    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));
    
    // Açıkça belirtilmiş tür parametresi
    generic::<char>(SGen('a'));
    
    // Örtük tür çıkarımı
    generic(SGen('c'));
}
```

---

### 3. Generic Implementation (Impl Blokları)

Fonksiyonlara benzer şekilde, `impl` blokları da generic olabilir.

```rust
struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

// Val için somut impl
impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

// GenVal için generic impl
impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}
```

Generic bir yapı için `impl` yazarken, türün generic olduğunu belirtmek için `impl<T>` kullanılır.

```rust
fn main() {
    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };
    println!("{}, {}", x.value(), y.value());
}
```

---

### 4. Generic Trait'ler

Trait'ler de generic olabilir.

```rust
// Non-copyable türler
struct Empty;
struct Null;

// T üzerinde generic bir trait
trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

// U (çağıran) ve T (parametre) için generic implementasyon
impl<T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {}
}

fn main() {
    let empty = Empty;
    let null = Null;
    empty.double_drop(null); // Her iki değer de burada drop olur
}
```

Bu örnekte `DoubleDrop<T>` trait'i generic'tir ve `U` türü için `T` türünde bir parametre alan `double_drop` metodu tanımlar.

---

### 5. Bounds (Sınırlar)

Generic tür parametreleri, genellikle hangi işlevleri desteklediklerini belirtmek için **trait sınırlarına (bounds)** ihtiyaç duyar.

```rust
use std::fmt::Display;

// T, Display trait'ini uygulamak ZORUNDA
fn printer<T: Display>(t: T) {
    println!("{}", t);
}
```

Sınırlar, generic türleri belirli trait'leri uygulayan türlerle kısıtlar:

```rust
// HATA! Vec<T> Display uygulamaz
// let s = S(vec![1]);
```

Sınırların bir diğer etkisi, generic örneklerin sınırlarda belirtilen trait'lerin metodlarına erişebilmesidir.

```rust
use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

// T, Debug uygulamak zorunda
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

// T, HasArea uygulamak zorunda
fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

fn main() {
    let rectangle = Rectangle { length: 3.0, height: 4.0 };
    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));
}
```

---

### 6. Multiple Bounds (Birden Fazla Sınır)

Tek bir tür için birden fazla sınır `+` ile uygulanabilir.

```rust
use std::fmt::{Debug, Display};

// T hem Debug hem Display uygulamalı
fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: `{:?}`", t);
    println!("Display: `{}`", t);
}

// T ve U farklı türler olabilir, her ikisi de Debug uygulamalı
fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: `{:?}`", t);
    println!("u: `{:?}`", u);
}

fn main() {
    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];
    
    compare_prints(&string);
    compare_types(&array, &vec);
}
```

---

### 7. Where Clauses

Sınırlar, türün ilk kullanımından hemen önce değil, açılış `{` öncesinde bir `where` cümlesi ile de ifade edilebilir.

```rust
use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

// Burada Option<T>: Debug sınırı where ile ifade ediliyor
impl<T> PrintInOption for T
where
    Option<T>: Debug,
{
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn main() {
    let vec = vec![1, 2, 3];
    vec.print_in_option();
}
```

`where` cümleleri şu durumlarda kullanışlıdır:
- Generic türleri ve sınırları ayrı ayrı belirtmek daha okunaklı olduğunda
- Normal sözdizimi ile doğrudan ifade edilemeyen durumlarda (örneğin `Option<T>: Debug` gibi)

---

### 8. New Type Idiom (Yeni Tür İdiomu)

**Newtype** idiyomu, programa doğru türde değerin verildiğine dair derleme zamanı garantisi sağlar.

```rust
struct Miles(f64);
struct Kilometers(f64);

impl Miles {
    pub fn to_kilometers(&self) -> Kilometers {
        Kilometers(self.0 * 1.609344)
    }
}

impl Kilometers {
    pub fn to_miles(&self) -> Miles {
        Miles(self.0 / 1.609344)
    }
}

fn is_a_marathon(distance: &Miles) -> bool {
    distance.0 >= 26.2
}

fn main() {
    let distance = Miles(30.0);
    let distance_km = distance.to_kilometers();
    
    println!("Is a marathon? {}", is_a_marathon(&distance));
    println!("Is a marathon? {}", is_a_marathon(&distance_km.to_miles()));
    // HATA: is_a_marathon(&distance_km); // Tür uyuşmazlığı!
}
```

Newtype'ın değerini temel tür olarak almak için tuple veya yapısöküm (destructuring) kullanılabilir:

```rust
struct Miles(f64);

fn main() {
    let distance = Miles(42.0);
    let distance_as_primitive_1: f64 = distance.0;      // Tuple erişimi
    let Miles(distance_as_primitive_2) = distance;      // Yapısöküm
}
```

---

### 9. Associated Items (İlişkili Öğeler)

**İlişkili öğeler**, trait generics'in bir uzantısıdır ve trait'lerin içinde yeni öğeler tanımlamasına olanak tanır.

#### The Problem (Sorun)

Bir container tipi üzerinde generic olan bir trait, tüm generic türlerinin belirtilmesini zorunlu kılar:

```rust
struct Container(i32, i32);

trait Contains {
    fn contains(&self, _: &i32, _: &i32) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    fn first(&self) -> i32 { self.0 }
    fn last(&self) -> i32 { self.1 }
}

// C, A ve B'yi içerir. A ve B'yi tekrar belirtmek zorunda kalmak can sıkıcı
fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}
```

#### Associated Types (İlişkili Türler)

**İlişkili türler**, iç türleri bir trait'e **çıktı** türleri olarak taşıyarak kod okunabilirliğini artırır:

```rust
trait Contains {
    type A;
    type B;
    
    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    type A = i32;
    type B = i32;
    
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    fn first(&self) -> i32 { self.0 }
    fn last(&self) -> i32 { self.1 }
}

// İlişkili türler sayesinde A ve B'yi belirtmeye gerek kalmaz
fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}
```

---

### 10. Phantom Type Parameters (Hayalet Tür Parametreleri)

**Hayalet tür parametresi**, çalışma zamanında görünmeyen, ancak derleme zamanında statik olarak kontrol edilen bir parametredir. Veri türleri, ekstra generic tür parametrelerini işaretleyici olarak veya derleme zamanı tür kontrolü yapmak için kullanabilir.

```rust
use std::marker::PhantomData;

// A üzerinde generic, B ile gizli parametreye sahip phantom tuple struct
#[derive(PartialEq)]
struct PhantomTuple<A, B>(A, PhantomData<B>);

// A üzerinde generic, B ile gizli parametreye sahip phantom struct
#[derive(PartialEq)]
struct PhantomStruct<A, B> {
    first: A,
    phantom: PhantomData<B>,
}

fn main() {
    // f32 ve f64 gizli parametreler
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);
    
    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    
    // Derleme zamanı HATASI! Türler uyuşmaz, karşılaştırılamaz
    // println!("_tuple1 == _tuple2: {}", _tuple1 == _tuple2);
}
```

---

### 11. Testcase: Unit Clarification (Birim Dönüşümü)

Phantom type parametreleri ile birim dönüşümleri yapmak için `Add` trait'i uygulanabilir:

```rust
use std::ops::Add;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
enum Inch {}

#[derive(Debug, Clone, Copy)]
enum Mm {}

#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;
    
    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        Length(self.0 + rhs.0, PhantomData)
    }
}

fn main() {
    let one_foot: Length<Inch> = Length(12.0, PhantomData);
    let one_meter: Length<Mm> = Length(1000.0, PhantomData);
    
    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;
    
    println!("one foot + one_foot = {:?} in", two_feet.0);
    println!("one meter + one_meter = {:?} mm", two_meters.0);
    
    // Derleme zamanı HATASI! Farklı birimler toplanamaz
    // let one_feter = one_foot + one_meter;
}
```

Bu yaklaşım, farklı birimlerin yanlışlıkla toplanmasını derleme zamanında engeller.

---

## Özet

Rust'ta generics:

| Kavram | Açıklama |
|--------|----------|
| **Tür Parametreleri** | `<>` ile belirtilen generic türler |
| **Generic Fonksiyonlar** | Tür parametresi alan fonksiyonlar |
| **Generic Impl** | Generic yapılar için implementasyonlar |
| **Generic Trait'ler** | Tür parametresi alan trait'ler |
| **Bounds** | Trait sınırları ile tür kısıtlama |
| **Multiple Bounds** | `+` ile birden fazla sınır |
| **Where Clauses** | Sınırları ayrı bir blokta belirtme |
| **New Type** | Tür güvenliği için sarmalayıcı türler |
| **Associated Types** | Trait içinde çıktı türleri tanımlama |
| **Phantom Data** | Derleme zamanı kontrolü için işaretleyiciler |

Generics, Rust'ın güçlü tür sisteminin temel taşlarından biridir ve doğru kullanıldığında hem güvenli hem de esnek kod yazmanıza olanak tanır.