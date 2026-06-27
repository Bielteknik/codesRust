# 📚 RUST BY EXAMPLE — Kapsamlı Ders Anlatımı

Aşağıda, Rust programlama dilini örneklerle öğreten resmi dokümantasyon olan **Rust By Example**'ın tüm ana başlıklarını bir ders gibi, adım adım ve detaylı şekilde işliyoruz. Hazırsanız başlayalım! 🚀

---

## 🎯 BÖLÜM 0: Rust Nedir? Neden Önemli?

**Rust**, Mozilla tarafından başlatılan, modern bir **sistem programlama dilidir**. Üç temel hedefi vardır:

| Hedef | Açıklama |
|-------|----------|
| 🔒 **Güvenlik (Safety)** | Bellek güvenliği — null pointer, dangling pointer, data race gibi hataları derleme zamanında engeller. |
| ⚡ **Hız (Speed)** | C/C++ ile yarışan performans. Sıfır maliyetli soyutlamalar (zero-cost abstractions). |
| 🔄 **Eşzamanlılık (Concurrency)** | "Fearless concurrency" — derleyici eşzamanlı hataları yakalar. |

> **Önemli:** Rust, çöp toplayıcı (garbage collector) kullanmadan bellek güvenliğini sağlar. Bunu **Ownership (Sahiplik)**, **Borrowing (Ödünç Alma)** ve **Lifetimes (Yaşam Ömürleri)** sistemiyle yapar.

---

## 📖 BÖLÜM 1: Hello World — İlk Adım

Her programlama dilinde olduğu gibi Rust'a da klasik "Merhaba Dünya" ile başlıyoruz.

```rust
fn main() {
    println!("Merhaba Dünya!");
}
```

### 🔍 Analiz:
- **`fn main()`**: Programın giriş noktası. Her Rust binary'si bir `main` fonksiyonu ile başlar.
- **`println!`**: Dikkat edin, sonunda **`!`** var. Bu bir **makro** çağrısıdır, normal fonksiyon değil! Makrolar derleme zamanında kod üretir.
- **`;` (noktalı virgül)**: İfadelerin sonunu belirtir.

### 🔨 Derleme ve Çalıştırma:
```bash
$ rustc hello.rs    # hello adında bir binary üretir
$ ./hello           # Çalıştır
Merhaba Dünya!
```

> 💡 **Pratikte** genelde `rustc` yerine **Cargo** kullanılır (Bölüm 13'e bakınız).

---

## 📖 BÖLÜM 2: Primitives (İlkel Tipler)

Rust, zengin bir ilkel tip yelpazesine sahiptir. Tip güvenliği çok güçlüdür.

### 🔢 Sayısal Tipler

#### İşaretli Tamsayılar (Signed Integers):
| Tip | Bit | Aralık |
|-----|-----|--------|
| `i8` | 8 | -128 .. 127 |
| `i16` | 16 | -32768 .. 32767 |
| `i32` | 32 | (varsayılan) |
| `i64` | 64 | |
| `i128` | 128 | |
| `isize` | pointer boyutu | 32 veya 64 bit |

#### İşaretsiz Tamsayılar (Unsigned Integers):
| Tip | Bit |
|-----|-----|
| `u8` | 8 (0..255) |
| `u16` | 16 |
| `u32` | 32 |
| `u64`, `u128`, `usize` | |

#### Kayan Nokta (Floating Point):
- **`f32`** — 32 bit
- **`f64`** — 64 bit (varsayılan)

### 🔤 Diğer İlkel Tipler

```rust
let harf: char = 'a';          // Unicode karakter (4 byte)
let emoji: char = '🦀';        // Evet, Rust yengeci bile!
let dogru_mu: bool = true;     // true veya false
let birim: () = ();            // Unit tip — boş değer
```

### 📦 Bileşik Tipler (Compound Types)

```rust
// Array — sabit uzunlukta, aynı tip
let dizi: [i32; 3] = [1, 2, 3];

// Tuple — farklı tipler bir arada
let ikili: (i32, bool) = (42, true);
```

### 🎯 Tip Çıkarımı (Type Inference)

```rust
let x = 42;        // Derleyici i32 olduğunu anlar
let y = 3.14;      // f64 olduğunu anlar
let z: u8 = 255;   // Açıkça belirtmek de mümkün
let w = 100_u64;   // Son ek (suffix) ile de belirtilebilir
```

> 📌 **Kural:** Sayılar varsayılan olarak `i32` ve `f64` kabul edilir.

---

## 📖 BÖLÜM 3: Custom Types (Özel Tipler)

Rust'ta kendi tiplerinizi oluşturabilirsiniz. Üç ana yapı vardır:

### 🏗️ `struct` — Yapılar

Üç farklı struct türü vardır:

```rust
// 1. Klasik C-benzeri struct
struct Dikdortgen {
    genislik: f64,
    yukseklik: f64,
}

// 2. Tuple struct (isimli tuple)
struct Renk(u8, u8, u8);

// 3. Unit struct (alanı olmayan — trait'ler için yararlı)
struct Birim;
```

### 🌳 `enum` — Numaralandırmalar

Bir değerin birkaç varyanttan biri olabileceği durumlar için:

```rust
enum Yon {
    Kuzey,
    Guney,
    Dogu,
    Bati,
}

// Daha güçlü: her varyant veri taşıyabilir!
enum Mesaj {
    Cikis,
    Tası(Vec<i32>),
    RenkDegistir(u8, u8, u8),
    Metin(String),
}
```

> 💡 Rust'ın meşhur `Option<T>` ve `Result<T, E>` tipleri aslında birer enum'dur!

### 🔒 `const` ve `static` — Sabitler

```rust
const PI: f64 = 3.14159;            // Değiştirilemez sabit
static MAX_BOYUT: i32 = 1024;       // 'static yaşam ömrü
static mut DEGISKEN: i32 = 0;       // Mutable static — unsafe gerektirir!
```

---

## 📖 BÖLÜM 4: Variable Bindings (Değişken Bağlamaları)

Rust'ta değişkenler `let` anahtar kelimesi ile bağlanır.

### 📌 Temel Bağlama

```rust
let x = 5;              // Değiştirilemez (immutable) — varsayılan
let mut y = 10;         // Değiştirilebilir (mutable)
y = 20;                 // ✅ OK
// x = 6;               // ❌ Hata! Immutable
```

### 🎭 Gölgeleme (Shadowing)

Aynı isimle yeni bir bağlama yapabilirsiniz:

```rust
let x = 5;
let x = x + 1;          // Yeni bir x oluşturuldu, eskisi gölgelendi
let x = "artık string"; // Tip bile değişebilir!
```

### 📦 Yapı Çözme (Destructuring)

```rust
let (a, b) = (10, 20);          // Tuple'dan çıkarma
let Dikdortgen { genislik, yukseklik } = rect;  // Struct'tan çıkarma
```

### 🔍 Kapsam (Scope)

```rust
let x = 5;
{
    let y = 10;
    println!("{} {}", x, y);  // ✅ İkisine de erişim
}
// println!("{}", y);         // ❌ y artık kapsam dışı
```

---

## 📖 BÖLÜM 5: Types (Tipler)

### 🎯 Tip Belirtme ve Çıkarımı

Rust **statik tiplidir** ama çoğu zaman tipi kendisi çıkarır:

```rust
let sayi: i32 = 42;          // Açık tip
let metin = "merhaba";       // &str olarak çıkarılır
let liste = vec![1, 2, 3];   // Vec<i32> olarak çıkarılır
```

### 🔄 Tip Dönüşümleri (Casting)

```rust
let x: i32 = 42;
let y: f64 = x as f64;       // as anahtar kelimesi ile dönüşüm
let z: i32 = 3.99 as i32;    // z = 3 (kesme işlemi)
```

---

## 📖 BÖLÜM 6: Conversion (Dönüşümler)

### 🔄 `From` ve `Into` Trait'leri

Özel tipler arası dönüşümler trait'lerle yapılır:

```rust
let s: String = String::from("merhaba");   // &str -> String
let n: i64 = 42_i32.into();                 // i32 -> i64 (Into kullanımı)
let m: i32 = i32::from(42_i64);             // From kullanımı
```

> 📌 **Kural:** `From` dönüşümü **asla hata vermez**. Hata ihtimali varsa `TryFrom`/`TryInto` kullanılır.

### 📝 String Dönüşümleri

```rust
let s = "42".parse::<i32>();           // String -> i32 (Result döner)
let n = 42.to_string();                 // i32 -> String
```

---

## 📖 BÖLÜM 7: Expressions (İfadeler)

Rust **ifade tabanlı (expression-oriented)** bir dildir. Neredeyse her şey bir değer döndürür!

```rust
let x = 5;                    // ifade
let y = {                     // blok da bir ifadedir
    let a = 10;
    a + 20                    // ← noktalı virgül YOK, bu değeri döndürür
};
// y = 30
```

> ⚠️ **Dikkat:** Bir satırın sonuna `;` koyarsanız, o ifade `()` (unit) döndürür. Koyamazsanız, değeri döndürür.

---

## 📖 BÖLÜM 8: Flow of Control (Kontrol Akışı)

### 🔀 `if` / `else`

```rust
let sayi = 42;

if sayi > 0 {
    println!("Pozitif");
} else if sayi < 0 {
    println!("Negatif");
} else {
    println!("Sıfır");
}

// if bir ifadedir, değer döndürebilir:
let durum = if sayi > 0 { "pozitif" } else { "negatif" };
```

> 📌 **Önemli:** `if`-`else` bir **ifade** olduğu için her iki dal da **aynı tipte** değer döndürmelidir.

### 🔁 `for` Döngüsü

```rust
// Range ile
for i in 0..5 {
    println!("{}", i);        // 0, 1, 2, 3, 4
}

for i in 0..=5 {
    println!("{}", i);        // 0, 1, 2, 3, 4, 5 (kapalı aralık)
}

// Koleksiyon üzerinde
let dizi = [10, 20, 30];
for deger in &dizi {
    println!("{}", deger);
}
```

#### 🔍 Üç Farklı Iterator Türü:

| Metod | Davranış |
|-------|----------|
| `iter()` | Ödünç alır (`&T`), koleksiyon korunur |
| `iter_mut()` | Değiştirilebilir ödünç alır (`&mut T`) |
| `into_iter()` | Sahiplenir, koleksiyonu tüketir |

### 🔁 `while` Döngüsü

```rust
let mut n = 0;
while n < 5 {
    println!("{}", n);
    n += 1;
}
```

### 🔁 `loop` — Sonsuz Döngü

```rust
let mut sayac = 0;
let sonuc = loop {
    sayac += 1;
    if sayac == 10 {
        break sayac * 2;    // loop'dan değer döndürülebilir!
    }
};
// sonuc = 20
```

### 🎯 `match` — Desen Eşleştirme

Rust'ın en güçlü özelliklerinden biri. C'nin `switch`'ine benzer ama çok daha güçlü:

```rust
let sayi = 42;

match sayi {
    0 => println!("Sıfır"),
    1..=9 => println!("Tek haneli"),
    10..=99 => println!("İki haneli"),
    _ => println!("Başka bir şey"),    // _ : yakalanmamış her şey
}
```

> ⚠️ **Kritik kural:** `match` **tüm olası durumları** kapsamalıdır (exhaustive).

---

## 📖 BÖLÜM 9: Functions (Fonksiyonlar)

### 📌 Temel Fonksiyonlar

```rust
fn topla(a: i32, b: i32) -> i32 {
    a + b           // Son ifade otomatik döner
}

fn merhaba() {
    println!("Merhaba!");
}
```

> 📌 **Not:** `return` kullanılabilir ama gerek yoktur; son ifade otomatik döndürülür.

### 🏗️ Metodlar — `impl` Blokları

```rust
struct Dikdortgen { w: f64, h: f64 }

impl Dikdortgen {
    // İlişkili fonksiyon (constructor gibi)
    fn yeni(w: f64, h: f64) -> Self {
        Dikdortgen { w, h }
    }
    
    // Metod — &self ile
    fn alan(&self) -> f64 {
        self.w * self.h
    }
    
    // Değiştiren metod — &mut self ile
    fn olcekile(&mut self, faktor: f64) {
        self.w *= faktor;
        self.h *= faktor;
    }
}

let mut r = Dikdortgen::yeni(3.0, 4.0);
println!("Alan: {}", r.alan());
```

### 🎭 Closure'lar (Kapamalar)

İsimsiz fonksiyonlar, çevrelerindeki değişkenleri yakalayabilir:

```rust
let carp = |x, y| x * y;
println!("{}", carp(3, 4));     // 12

let isim = String::from("Rust");
let selamla = || println!("Merhaba {}", isim);  // değişkeni yakaladı
selamla();
```

### 🎓 Yüksek-Seviyeli Fonksiyonlar

Fonksiyonlar başka fonksiyonlara parametre olarak verilebilir:

```rust
fn islem<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

let kare = |x| x * x;
println!("{}", islem(kare, 5));   // 25
```

---

## 📖 BÖLÜM 10: Modules (Modüller)

Rust'ın modül sistemi kodu mantıksal birimlere ayırır ve görünürlüğü yönetir.

```rust
mod matematik {
    pub fn topla(a: i32, b: i32) -> i32 {
        a + b
    }
    
    fn gizli() -> i32 {  // pub yoksa private!
        42
    }
}

fn main() {
    println!("{}", matematik::topla(2, 3));
    // matematik::gizli();  // ❌ Hata! Private
}
```

### 📂 Dosya Yapısı

```
src/
├── main.rs
├── lib.rs
└── modeller/
    ├── mod.rs
    └── matematik.rs
```

> 📌 `pub use` ile yeniden dışa aktarım yapılabilir.

---

## 📖 BÖLÜM 11: Crates (Kasa/Paket)

**Crate**, Rust'ta derleme birimidir. İki türü vardır:
- **Binary crate** → çalıştırılabilir program (`main.rs`)
- **Library crate** → kütüphane (`lib.rs`)

```rust
// Başka bir crate'i kullanmak:
use std::collections::HashMap;
use std::io::{self, Read, Write};
```

---

## 📖 BÖLÜM 12: Cargo — Paket Yöneticisi

**Cargo**, Rust'ın resmi paket yöneticisi ve derleme aracıdır.

### 🛠️ Temel Komutlar:

```bash
cargo new projem          # Yeni proje oluştur
cargo build               # Derle
cargo run                 # Derle ve çalıştır
cargo check               # Sadece kontrol et (hızlı)
cargo test                # Testleri çalıştır
cargo doc --open          # Dokümantasyonu oluştur ve aç
cargo update              # Bağımlılıkları güncelle
cargo add serde           # Bağımlılık ekle
```

### 📦 `Cargo.toml` Örneği:

```toml
[package]
name = "projem"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0"
tokio = { version = "1", features = ["full"] }
```

---

## 📖 BÖLÜM 13: Attributes (Öznitelikler)

Öznitelikler, modül/crate/öğe üzerine metadata ekler.

```rust
// Test olduğunu belirtir
#[test]
fn bir_test() {
    assert_eq!(2 + 2, 4);
}

// Derleyici uyarısını bastır
#[allow(dead_code)]
fn kullanilmayan() {}

// Binary'nin adını değiştir
#![crate_name = "benim_kutum"]

// Derleyici özelliğini etkinleştir
#![feature(some_nightly_feature)]
```

---

## 📖 BÖLÜM 14: Generics (Jenerikler)

Jenerikler, kod tekrarını önlemek için tipleri genelleştirir.

### 🔧 Jenerik Fonksiyon

```rust
fn ilk_eleman<T>(liste: &[T]) -> Option<&T> {
    liste.first()
}

let sayilar = vec![1, 2, 3];
let harfler = vec!['a', 'b', 'c'];

println!("{:?}", ilk_eleman(&sayilar));   // Some(1)
println!("{:?}", ilk_eleman(&harfler));   // Some('a')
```

### 🔧 Jenerik Struct

```rust
struct Cift<T> {
    birinci: T,
    ikinci: T,
}

let c1 = Cift { birinci: 1, ikinci: 2 };
let c2 = Cift { birinci: 1.0, ikinci: 2.0 };
```

### 📐 Trait Sınırları (Trait Bounds)

Jenerik tiplerin belirli özelliklere sahip olmasını zorunlu kılabilirsiniz:

```rust
use std::fmt::Display;

fn yazdir<T: Display>(deger: T) {
    println!("Değer: {}", deger);
}

// where sözdizimi ile:
fn karmasik<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // ...
}
```

---

## 📖 BÖLÜM 15: Scoping Rules (Kapsam Kuralları)

Kapsamlar; **ownership**, **borrowing** ve **lifetimes** için kritiktir. Derleyiciye kaynakların ne zaman serbest bırakılacağını söyler.

```rust
fn ornek() {
    let x = String::from("merhaba");  // x buradan itibaren geçerli
    
    {
        let y = &x;                    // y, x'i ödünç alıyor
        println!("{}", y);
    }  // y kapsam dışı
    
    println!("{}", x);                 // ✅ x hala geçerli
}  // x burada düşürülür (drop)
```

### 📚 RAII Prensibi

Rust'ta kaynaklar kapsam ile yönetilir — bir değişken kapsam dışına çıktığında otomatik olarak temizlenir.

---

## 📖 BÖLÜM 16: Traits (Özellikler)

**Trait**, bilinmeyen bir tip (`Self`) için tanımlanan metodların koleksiyonudur. Diğer dillerdeki **interface** kavramına benzer.

```rust
trait Hayvan {
    fn ses_cikar(&self) -> String;
    
    // Varsayılan implementasyon
    fn selamla(&self) -> String {
        format!("Merhaba, ben bir hayvanım ve {}", self.ses_cikar())
    }
}

struct Kedi;
struct Kopek;

impl Hayvan for Kedi {
    fn ses_cikar(&self) -> String { "Miyav!".to_string() }
}

impl Hayvan for Kopek {
    fn ses_cikar(&self) -> String { "Hav hav!".to_string() }
}

fn main() {
    let k = Kedi;
    println!("{}", k.selamla());   // "Merhaba, ben bir hayvanım ve Miyav!"
}
```

### 🎯 Önemli Trait'ler

| Trait | Açıklama |
|-------|----------|
| `Display` | Kullanıcı dostu yazdırma (`{}`) |
| `Debug` | Debug yazdırma (`{:?}`) |
| `Clone` | `.clone()` ile kopyalama |
| `Copy` | Otomatik bit-kopyası |
| `Drop` | Değer düşürüldüğünde çalışacak kod |
| `Iterator` | Döngüsel erişim |
| `From` / `Into` | Tip dönüşümleri |

### 🔗 Trait Kalıtımı

```rust
trait Yazdirilabilir: Display + Debug {
    // Hem Display hem Debug gerektirir
}
```

---

## 📖 BÖLÜM 17: Macros (Makrolar)

Makrolar, **kod yazan kod**dur (metaprogramming). C'deki makrolardan farklı olarak Rust makroları **soyut sözdizim ağacına (AST)** genişletilir, bu yüzden öncelik hataları olmaz.

### 📌 Makro Tanımlama

```rust
macro_rules! selamla {
    ($isim:expr) => {
        println!("Merhaba, {}!", $isim);
    };
    ($isim:expr, $sayi:expr) => {
        println!("Merhaba {}, sayın: {}", $isim, $sayi);
    };
}

selamla!("Ahmet");              // "Merhaba, Ahmet!"
selamla!("Ayşe", 42);           // "Merhaba Ayşe, sayın: 42"
```

### 🎯 Makrolar Neden Faydalı?

1. **DRY (Don't Repeat Yourself)** — Tekrar eden kodları önler
2. **Alan-spesifik diller (DSL)** — Özel sözdizimi tanımlanabilir
3. **Değişken sayıda argüman** — `println!` gibi

> 📌 Makrolar kullanılmadan **önce tanımlanmalıdır** (veya `#[macro_use]` ile import edilmeli).

---

## 📖 BÖLÜM 18: Error Handling (Hata Yönetimi)

Rust'ta hatalar ciddidir ve açıkça yönetilmelidir.

### 😱 `panic!` — Kurtarılamaz Hatalar

```rust
fn bol(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Sıfıra bölme hatası!");   // Program çöker
    }
    a / b
}
```

### 📦 `Option<T>` — Değer Var/Yok

```rust
fn bul(dizi: &[i32], hedef: i32) -> Option<usize> {
    for (i, &v) in dizi.iter().enumerate() {
        if v == hedef {
            return Some(i);
        }
    }
    None
}

match bul(&[1, 2, 3], 2) {
    Some(i) => println!("Bulundu: {}", i),
    None => println!("Bulunamadı"),
}
```

### 📦 `Result<T, E>` — Başarı/Hata

```rust
use std::fs::File;
use std::io::Read;

fn dosya_oku(yol: &str) -> Result<String, std::io::Error> {
    let mut dosya = File::open(yol)?;   // ? operatörü — hata varsa erken döner
    let mut icerik = String::new();
    dosya.read_to_string(&mut icerik)?;
    Ok(icerik)
}
```

### 🎯 Hata Yönetimi Rehberi

| Durum | Çözüm |
|-------|-------|
| Kurtarılamaz hata | `panic!` |
| Değer opsiyonel | `Option<T>` |
| Hata ihtimali var | `Result<T, E>` |
| Prototip/test | `unwrap()` veya `expect("mesaj")` |
| Ciddi kod | `?` operatörü, `match` |

---

## 📖 BÖLÜM 19: Std Library Types (Standart Kütüphane Tipleri)

Rust'ın `std` kütüphanesi, ilkel tiplerin ötesinde güçlü tipler sunar:

### 📝 `String` — Büyüyebilir Metin

```rust
let mut s = String::from("Merhaba");
s.push_str(" Dünya!");
s.push('!');
println!("{}", s);   // "Merhaba Dünya!!"
```

### 📋 `Vec<T>` — Büyüyebilir Dizi

```rust
let mut v = vec![1, 2, 3];
v.push(4);
v.push(5);
for x in &v {
    println!("{}", x);
}
```

### 🗺️ `HashMap<K, V>` — Sözlük

```rust
use std::collections::HashMap;

let mut skorlar = HashMap::new();
skorlar.insert("Ali", 95);
skorlar.insert("Veli", 87);

if let Some(s) = skorlar.get("Ali") {
    println!("Ali'nin skoru: {}", s);
}
```

### 📦 Diğer Önemli Tipler

- **`Box<T>`** — Heap'te tahsis
- **`Rc<T>`** — Referans sayımı ile çoklu sahiplik
- **`Arc<T>`** — Thread-safe Rc
- **`RefCell<T>`** — Çalışma zamanında borrowing kuralları
- **`Cell<T>`** — İç değişkenlik (Copy tipler için)

---

## 📖 BÖLÜM 20: Std Misc — Dosya, Thread ve Diğerleri

### 📁 Dosya İşlemleri

```rust
use std::fs::File;
use std::io::Write;

let mut f = File::create("test.txt").unwrap();
f.write_all(b"Merhaba dosya!").unwrap();
```

### 🧵 Thread'ler

```rust
use std::thread;
use std::time::Duration;

let handle = thread::spawn(|| {
    for i in 1..5 {
        println!("Thread: {}", i);
        thread::sleep(Duration::from_millis(100));
    }
});

handle.join().unwrap();
```

### 📡 Kanallar (Channels)

```rust
use std::sync::mpsc::channel;

let (tx, rx) = channel();
thread::spawn(move || {
    tx.send("Merhaba!").unwrap();
});
let mesaj = rx.recv().unwrap();
println!("{}", mesaj);
```

---

## 📖 BÖLÜM 21: Testing (Test)

Rust, test yazımını dilin içine yerleştirmiştir.

### 🧪 Üç Test Türü:

1. **Unit Testler** — Birim bazlı
2. **Doc Testler** — Dokümantasyon içinde
3. **Integration Testler** — Entegrasyon

```rust
pub fn topla(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod testler {
    use super::*;
    
    #[test]
    fn toplama_testi() {
        assert_eq!(topla(2, 3), 5);
    }
    
    #[test]
    #[should_panic]
    fn panik_testi() {
        panic!("Bu test panikleyerek geçmeli");
    }
}
```

### 📝 Doc Test

```rust
/// İki sayıyı toplar
///
/// # Examples
/// ```
/// let sonuc = topla(2, 3);
/// assert_eq!(sonuc, 5);
/// ```
pub fn topla(a: i32, b: i32) -> i32 { a + b }
```

### 🏃 Test Çalıştırma:
```bash
cargo test
```

---

## 📖 BÖLÜM 22: Unsafe Operations (Güvensiz İşlemler)

Rust'ın güvenlik garantilerini devre dışı bırakmak için kullanılır. **Minimum düzeyde kullanılmalıdır.**

### ⚠️ Unsafe'ın 4 Kullanım Alanı:

1. **Ham pointer'ları (raw pointers) serbest bırakma**
2. **`unsafe` fonksiyonları çağırma** (FFI dahil)
3. **Mutable static değişkenlere erişme**
4. **Unsafe trait'leri implement etme**

```rust
unsafe fn tehlikeli() {
    let p: *const i32 = &42;
    println!("Değer: {}", *p);   // Ham pointer serbest bırakma
}

fn main() {
    unsafe {
        tehlikeli();
    }
}
```

> ⚠️ **Uyarı:** Unsafe blok içindeki hataların sorumluluğu programcıya aittir. Derleyici burada yardım etmez.

---

## 📖 BÖLÜM 23: Meta — Dokümantasyon ve Benchmarking

### 📚 Dokümantasyon

```rust
/// Bu fonksiyon iki sayıyı toplar.
///
/// Detaylı açıklama buraya gelir.
/// 
/// # Arguments
/// * `a` - İlk sayı
/// * `b` - İkinci sayı
///
/// # Returns
/// Toplamları
pub fn topla(a: i32, b: i32) -> i32 { a + b }
```

```bash
cargo doc --open   # HTML dokümantasyonu oluşturur
```

### ⏱️ Benchmarking

```rust
#![feature(test)]
extern crate test;

#[bench]
fn bench_topla(b: &mut test::Bencher) {
    b.iter(|| topla(100, 200));
}
```

---

## 📖 BÖLÜM 24: Compatibility (Uyumluluk)

Rust sürekli gelişir. Sürümler arası uyumluluğu yönetmek için:

- **Edition'lar** — `edition = "2021"` gibi
- **Feature flag'ler** — Kararsız özellikler
- **Deprecation** — Eski API'lerin işaretlenmesi

---

## 🎓 ÖZET VE ÖĞRENME YOLU

Aşağıdaki sırayla ilerlemeniz önerilir:

```
1. Hello World → 2. Primitives → 3. Custom Types → 4. Variables
↓
5. Flow Control → 6. Functions → 7. Modules → 8. Cargo
↓
9. Generics → 10. Traits → 11. Error Handling
↓
12. Std Types → 13. Testing → 14. Macros
↓
15. Unsafe (son çare) → 16. Meta
```

### 📌 Altın Kurallar:

1. ✅ **Mümkün olduğunca güvenli kod yazın.**
2. ✅ **`unsafe`'ı son çare olarak kullanın.**
3. ✅ **Hataları görmezden gelmeyin — `unwrap()` sadece testlerde.**
4. ✅ **Trait'ler ile soyutlama yapın.**
5. ✅ **Cargo kullanın, `rustc`'yi nadiren.**

---

## 🚀 Sonraki Adımlar

- 📘 **The Rust Programming Language (The Book)** — Derinlemesine teori
- 🎯 **Rustlings** — Pratik alıştırmalar
- 🏋️ **Exercism Rust Track** — Mentorlu alıştırmalar
- 📖 **Rust Cookbook** — Gerçek dünya tarifleri
- 🔍 **crates.io** — 140.000+ hazır kütüphane

---

> 🦀 **"Rust ile programlamak, derleyici ile dans etmeye benzer — başta zor gelir ama adımları öğrendiğinizde muhteşem bir uyum yakalarsınız!"**

Başarılar! Herhangi bir bölüm hakkında daha fazla detay ister misiniz?

# DeepSeek Yorumu --------------------

Özür dilerim, haklısınız. Biraz “terimler sözlüğü” gibi kalmış, not alıp çalışılacak bir ders anlatımı olmamış. Şimdi elime kalemi alıp senin için baştan, **samimi bir ders notu/çalışma rehberi** formatında yazıyorum. 

Amaç, her konuyu okurken **“aaa demek bu yüzden böyleymiş”** dedirtmek. Hadi başlayalım.

---

## 🦀 RUST ÇALIŞMA NOTLARI (Rust By Example Özeti)

Rust öğrenirken aklında hep şu olsun: **Rust seni hata yapmadan ÖNCE durdurmak ister.** Derleyiciyle inatlaşmak yerine onu bir akıl hocası gibi düşün.

---

### 1. Hello World – İlk Temas
**Amaç:** Program nasıl başlar, ekrana nasıl yazı yazılır?

```rust
fn main() {
    println!("Merhaba!");
}
```
- `fn main()` her programın giriş kapısıdır.
- `println!` bir **makro** (sonunda `!` var). Normal fonksiyon değil, kod üreten bir yapı.
- **Yorumlar:** `//` normal, `///` dökümantasyon için. Bol bol kullan.

**Püf noktası:** `println!("{}", değişken)` yapısı formatlama içindir. `{:?}` ise geliştirici çıktısı verir (Debug). Kendi yapına `#[derive(Debug)]` eklersen `{:?}` ile yazdırabilirsin.

---

### 2. İlkel Türler – Programın Tuğlaları
Her şeyin temeli. Sayı, harf, liste...

- **Sayılar:** `i32` (default), `u8` (0-255), `f64` (default ondalık). Türü sonuna yaz: `let x = 5u8;`
- **Tuple:** Karışık türleri bir arada tutar, sabit boyutlu. `(1, "merhaba", true)`. Elemana `.0` ile eriş.
- **Array:** Aynı tür, sabit boyut: `[1, 2, 3]` veya `[0; 10]` (10 tane sıfır).
- **Slice:** Bir dizinin parçası: `&dizi[0..2]`. Hafif ve güvenli.

**Not:** Rust’ta string işleri karışık gelebilir. `"metin"` bir `&str` (dilim), sahipli metin ise `String` türüdür. İkisi farklı, ileride değineceğiz.

---

### 3. Kendi Türlerini Yarat (Struct & Enum)
Programını gerçek hayata benzetmek için.

**Struct:** Bilgi kümesi.
```rust
struct Araba {
    marka: String,
    hiz: u32,
}
```
**Enum:** Durum makinesi. Olmazsa olmaz!
```rust
enum Cevap {
    Evet,
    Hayir,
    Belki(i32), // veri de taşıyabilir
}
```
**Püf:** `match` ile enum’ı işlersen derleyici tüm ihtimalleri kontrol etmeye zorlar, hata kaçırmazsın.

---

### 4. Değişken Bağlama – İnatçı `let`
**Çok önemli:** Rust’ta her şey varsayılan olarak **değiştirilemez (immutable).** Değiştireceksen `let mut` kullan.

```rust
let x = 5;
// x = 6; HATA!
let mut y = 5;
y = 6; // Sorun yok.
```
**Gölgeleme (Shadowing):** Aynı ismi yeniden `let` ile tanımlarsan öncekini gölgelersin, türü bile değiştirebilirsin. `mut` ile karıştırma, farklı şeyler.

---

### 5. Akış Denetimi – Programın Mantığı
**`if` bir ifadedir**, değer döndürür.
```rust
let sayi = if kosul { 10 } else { 20 };
```
**`loop`:** Sonsuz döngü. `break` değer döndürebilir.
**`while`:** Klasik.
**`for`:** En temizi, vektörler ve diziler için: `for oge in &dizi { ... }`.

**En güçlü silah: `match`**. `if`’lerden kurtarır, desen eşler. `_` ile “diğer tüm durumlar” yakalanır.

---

### 6. Sahiplik (Ownership) – Rust’ın Kalbi
Bellek güvenliğinin sırrı. **Her değerin tek bir sahibi var.** Sahip işi bitince değer bellekten silinir.

**Kural 1:** Değer bir değişkene atanınca **taşınır (move)**.
```rust
let a = String::from("Rust");
let b = a; // a artık geçersiz! Sahiplik b'ye geçti.
```
**Kural 2:** Fonksiyona değer göndermek de taşımaktır. Geri döndürmezsen düşer.
**Kural 3:** Referans (`&`) ile **ödünç alarak** sahipliği değiştirmeden kullanabilirsin.

**Referansın altın kuralı:**
- Aynı anda **ya bir tane değiştirilebilir (`&mut`) referans**,
- **ya da birden çok değişmez (`&`) referans** olabilir. İkisi bir arada olmaz.

Bu kural, veri yarışını (data race) derleme aşamasında yok eder.

---

### 7. Fonksiyonlar ve Metotlar
```rust
fn topla(x: i32, y: i32) -> i32 {
    x + y // noktalı virgül yok -> bu bir ifade, değer döner
}
```
**Metot:** `impl` bloğu içinde yazılır, ilk parametre `&self` olur.
```rust
impl Araba {
    fn yeni() -> Self { ... } // static metot (self yok)
    fn gaz_ver(&mut self) { self.hiz += 10; }
}
```
**Closure (Anonim Fonksiyon):** Kısa, çevresindeki değişkenleri yakalar.
```rust
let kare_al = |x| x * x;
```
Closure’lar çok kullanılır; iterator’larda, fonksiyonel işlemlerde.

---

### 8. Modüller ve Crate’ler – Büyüyen Kod
Kod büyüdükçe bölmek şart.
- `mod` ile modül oluştur. Varsayılan olarak içindekiler **gizlidir**, `pub` ile aç.
- `use` ile yolu kısalt.
- Farklı dosyalar: `mod dosya_adi;` derleyici otomatik bulur.
- `Cargo.toml` ile harici kütüphaneler eklenir.

---

### 9. Hata Yönetimi – “Panik yok, Result var!”
Rust’ta istisna (exception) diye bir şey yok. İki tür hata:
- **Kurtarılamaz:** `panic!()` ile program çöker.
- **Kurtarılabilir:** `Result<T, E>` ve `Option<T>`.

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
**Çok kullanılan yöntemler:**
- `unwrap()`: Panik yapar, sadece eminsen veya prototip yaparken kullan.
- `expect("mesaj")`: Panik ama mesajla.
- **`?` operatörü:** Hata varsa otomatik döndürür, kodu inanılmaz temizler.
```rust
let dosya = File::open("test.txt")?; // hata olursa fonksiyondan çıkar
```
**Öğüt:** Hataları yutma, güzelce işle. `match` veya `unwrap_or_else` ile alternatif yol sun.

---

### 10. Generics ve Trait’ler – Yeniden Kullanılabilir Kod
**Generics:** Türden bağımsız kod.
```rust
fn en_buyuk<T: PartialOrd>(liste: &[T]) -> &T { ... }
```
**Trait:** “Davranış” tanımlar. Diğer dillerdeki interface gibi.
```rust
trait SesCikar {
    fn ses(&self) -> String;
}
impl SesCikar for Kedi {
    fn ses(&self) -> String { "Miyav".to_string() }
}
```
**Trait Sınırı (Bound):** `T: SesCikar + Debug` diyerek “T hem ses çıkarabilir hem yazdırılabilir olmalı” dersin.

---

### 11. Koleksiyonlar – Verilerle Oynamak
- **Vec<T>:** Dinamik dizi. `vec![]` makrosuyla oluştur. `push`, `pop`, indeksleme.
- **String:** Sahipli, büyüyebilir metin. `push_str()` ile ekleme.
- **HashMap<K,V>:** Sözlük. `insert`, `get`, `entry().or_insert()` çok pratiktir.

**İterator’lar** (`iter()`, `map()`, `filter()`, `collect()`) Rust’ın gizli silahıdır, döngüleri temizler.

---

### 12. Test Yazma – Geleceğine Yatırım
`#[test]` ile işaretle, `cargo test` ile çalıştır. Küçük fonksiyonlarını test et. Hata mesajlarını kontrol etmek için `#[should_panic]` kullan. Test yazmak Rust kültürünün parçasıdır, atlama.

---

### 13. Makrolar (`macro_rules!`) – Kod Yazan Kod
Sık tekrarlanan kalıpları kısaltır. `vec![1,2,3]` bile bir makrodur. Ünlem (`!`) ile çağrılır. Başlangıçta yazmak zor gelebilir, acele etme. Ama ne işe yaradığını bil.

---

### 14. Unsafe Rust – Kilidi Açmak
Rust’ın güvenlik garantilerini “ben sorumluluk alırım” diyerek esnetirsin. Ham işaretçiler (`*const T`, `*mut T`) ve FFI (C kodu çağırma) için gereklidir. **Günlük kodda neredeyse hiç kullanmazsın.** Varlığından haberdar ol yeter.

---

## ✅ ÇALIŞMA TAVSİYELERİ
1. **Her konuyu kendin ufak örneklerle dene.** Sadece okumak yetmez, hata mesajlarını gör.
2. **Derleyiciye sinirlenme, oku.** Mesajların çoğu çözüm önerir.
3. **Sahiplik ve ödünç alma konusunu tam anlamadan ilerleme.** Gerisi bu temelin üstüne kurulu.
4. **`match` ve `?` operatörünü hayatına sok.** Kodu yarı yarıya kısaltır.
5. **Resmi dökümandaki canlı kodları çalıştır, boz, düzelt.** En iyi öğrenme yolu budur.

Bu notları defterine geçirip kenarına kendi küçük örneklerini yazarsan, Rust senin için çok daha anlaşılır olacak. Yolun açık olsun!