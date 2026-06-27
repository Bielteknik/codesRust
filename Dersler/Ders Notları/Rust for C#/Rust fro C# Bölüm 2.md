# 🎓 Rust for .NET Developers: Kapsamlı Dil Rehberi

C# ve Rust, ilk bakışta modern, tip-güvenli (type-safe) diller gibi görünse de, arka planda yattıkları felsefeler taban tabana zıttır. C# **Çöp Toplayıcı (GC)** ile çalışma zamanında belleği yönetirken, Rust **Sahiplik (Ownership)** sistemi ile derleme zamanında bellek güvenliğini garanti eder.

Bu rehberde, her konuyu **C# → Rust** şeklinde yan yana karşılaştırmalı örneklerle, bir ders anlatır gibi inceleyeceğiz.

---

# 📘 BÖLÜM 1: Değişkenler, Tipler ve Gölgeleme (Shadowing)

## 1.1 Değişmezlik (Immutability) Varsayılandır

**C#**:
```csharp
var x = 5;
x = 6; // ✅ Başarılı - varsayılan olarak değiştirilebilir
```

**Rust**:
```rust
let x = 5;
// x = 6; // ❌ HATA: `x` değişmez (immutable)

let mut y = 5;
y = 6; // ✅ Başarılı - açıkça `mut` ile belirtildi
```

> 💡 **Neden?** Rust, değişmezliği varsayılan yaparak "yanlışlıkla değişken değiştirme" hatalarını derleme zamanında engeller. Bu, özellikle çok thread'li (multi-threaded) programlarda veri yarışlarını (data race) önlemenin ilk adımıdır.

## 1.2 Gölgeleme (Shadowing)

C#'ta aynı isimde değişken tanımlayamazsınız, ancak Rust'ta bu bir **özelliktir**. Türü değiştirerek aynı ismi yeniden kullanabilirsiniz:

**Rust**:
```rust
let spaces = "   ";           // Tür: &str (string dilimi)
let spaces = spaces.len();    // Tür: usize (sayı) - aynı isim!
let spaces = spaces as f64;   // Tür: f64 - yine aynı isim!
```

> ⚠️ **Dikkat:** `mut` ile gölgeleme farklı şeylerdir. `mut` aynı bellek bölgesini değiştirirken, gölgeleme yeni bir değişken yaratır ve eskisini kapatır.

## 1.3 Sabitler (Constants)

**C#**:
```csharp
const int MAX_BOYUT = 100;
```

**Rust**:
```rust
const MAX_BOYUT: usize = 100;  // Tür açıkça belirtilmek zorunda
```

Rust'ta `const` ile `static` arasında fark vardır:
- `const`: Her kullanımda değeri kopyalar (inline edilir).
- `static`: Bellekte tek bir sabit adres tutar (global değişken gibi).

---

# 📘 BÖLÜM 2: Ownership, Borrowing ve Lifetimes ⭐ (EN KRİTİK BÖLÜM)

Bu bölüm, C# geliştiricilerinin Rust'ta **en çok zorlandığı** ama en çok güç kazandığı bölümdür. C#'ta GC her şeyi yönetir; Rust'ta ise **her değerin tek bir sahibi (owner)** vardır ve sahip kapsam dışına çıktığında değer otomatik olarak serbest bırakılır (Drop).

## 2.1 Sahiplik Kuralları

1. Her değerin **tek bir sahibi** vardır.
2. Bir zamanda sadece **bir sahip** olabilir.
3. Sahip kapsam (scope) dışına çıkınca değer **düşürülür (drop)**.

**C#**:
```csharp
void Metod() {
    var s1 = new StringBuilder("Merhaba");
    var s2 = s1;           // s1 ve s2 aynı nesneyi gösterir (referans kopyası)
    Console.WriteLine(s1); // ✅ Çalışır - GC halleder
}
```

**Rust**:
```rust
fn main() {
    let s1 = String::from("Merhaba");
    let s2 = s1;           // ⚠️ Sahiplik s1'den s2'ye TAŞINDI (move)
    // println!("{}", s1); // ❌ HATA: s1 artık geçersiz!
    println!("{}", s2);    // ✅ Çalışır
}
```

> 💡 **Neden böyle?** Rust, "double free" (aynı belleği iki kez serbest bırakma) hatasını önlemek için sahipliği taşır. C#'taki gibi iki değişkenin aynı heap verisini paylaşmasına izin vermez.

## 2.2 Kopyalama (Clone)

Eğer gerçekten iki ayrı kopya istiyorsanız, açıkça belirtmelisiniz:

```rust
let s1 = String::from("Merhaba");
let s2 = s1.clone();       // Gerçekten derin kopya (deep copy)
println!("s1: {}, s2: {}", s1, s2); // ✅ İkisi de geçerli
```

> 🎯 **Kural:** Rust'ta "bedava" bir işlem yoktur. Pahalı bir işlem (heap kopyası) yapıyorsanız, bunu kodda **açıkça** yazmalısınız.

## 2.3 Ödünç Alma (Borrowing) ve Referanslar

Sahipliği taşımak istemiyorsanız, **referans** ile ödünç alırsınız. Bu, C#'taki referanslara benzer ama kuralları çok daha katıdır.

```rust
fn uzunluk_yaz(s: &String) {   // & = referans (ödünç alma)
    println!("{}'nin uzunluğu: {}", s, s.len());
} // s burada kapsam dışına çıkar ama sahipliği onda olmadığı için drop edilmez

fn main() {
    let s1 = String::from("Merhaba");
    uzunluk_yaz(&s1);          // &s1 = s1'in referansını ödünç ver
    println!("{}", s1);        // ✅ s1 hala geçerli - sahiplik taşınmadı
}
```

### 🚨 Ödünç Alma Kuralları (Borrow Checker)

Rust derleyicisi şu iki kuralı **asla** çiğnetmez:

1. **Ya sadece TEK BİR değiştirilebilir (mutable) referans** olabilir.
2. **YA DA istediğiniz kadar değişmez (immutable) referans** olabilir.
3. Aynı anda ikisi birden **OLAMAZ**.

**C#** (Bu kod çalışır ama tehlikelidir):
```csharp
var liste = new List<int> { 1, 2, 3 };
var referans = liste;
liste.Add(4);                // Liste değişti
Console.WriteLine(referans.Count); // Beklenmedik davranışlara yol açabilir
```

**Rust** (Derleyici bunu engeller):
```rust
let mut v = vec![1, 2, 3];
let r1 = &v;          // İlk değişmez referans
let r2 = &v;          // İkinci değişmez referans - OK
println!("{}, {}", r1, r2);
// let r3 = &mut v;   // ❌ HATA: Değişmez referanslar varken mutable referans ALAMAZSINIZ
```

## 2.4 Lifetime (Yaşam Süresi)

Rust'ın en zorlayıcı konusu: referansların **ne kadar süre geçerli** olduğunu derleyiciye bildirmek.

```rust
// 'a lifetime: döndürülen referans, en az x veya y kadar yaşamalı
fn en_uzun<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

> 💡 C#'ta GC olduğu için lifetime düşünmezsiniz. Rust'ta ise referansların "dangling" (sarkık, artık var olmayan bir değeri gösteren) olması **derleme zamanında** engellenir.

---

# 📘 BÖLÜM 3: Koleksiyonlar

## 3.1 Vektör (Vec<T>) ↔ List<T>

**C#**:
```csharp
var liste = new List<int> { 1, 2, 3 };
liste.Add(4);
var ilk = liste[0];
```

**Rust**:
```rust
let mut v = vec![1, 2, 3];
v.push(4);
let ilk = v[0];              // ✅
// let riskli = v[100];      // ❌ Runtime panic!
let guvenli = v.get(100);    // ✅ Option<&i32> döner - güvenli
```

> ⚠️ **Önemli:** Rust'ta `v[i]` erişimi sınır dışındaysa **program çöker (panic)**. Güvenli erişim için `get()` kullanın, bu size `Option<T>` döner.

### Vektör Üzerinde Döngü ve Borrow Checker

```rust
let mut v = vec![1, 2, 3];
// for x in &v { v.push(4); }  // ❌ HATA: Döngü sırasında vektörü değiştiremezsiniz!
// Çünkü döngü v'yi ödünç alıyor, push ise mutable referans ister.
```

## 3.2 HashMap ↔ Dictionary<TKey, TValue>

**C#**:
```csharp
var sozluk = new Dictionary<string, int>();
sozluk["ali"] = 25;
```

**Rust**:
```rust
use std::collections::HashMap;

let mut sozluk = HashMap::new();
sozluk.insert("ali", 25);

// Erişim
if let Some(yas) = sozluk.get("ali") {
    println!("Ali'nin yaşı: {}", yas);
}

// Varsayılan değer ile
let yas = sozluk.get("veli").copied().unwrap_or(0);
```

## 3.3 Iterator ve LINQ Benzeri İşlemler

Rust'ta LINQ yoktur ama **Iterator trait**'i ile neredeyse aynı güce sahiptir.

**C# (LINQ)**:
```csharp
var sonuc = liste
    .Where(x => x > 5)
    .Select(x => x * 2)
    .Sum();
```

**Rust**:
```rust
let sonuc: i32 = vektör
    .iter()
    .filter(|&x| *x > 5)
    .map(|&x| x * 2)
    .sum();
```

> 💡 Rust'ta iterator'lar **"lazy"** (tembel) dir - `collect()` çağrılmadıkça işlem gerçekleşmez. Bu, LINQ ile aynıdır.

---

# 📘 BÖLÜM 4: String İşlemleri

C# geliştiricilerinin en çok karıştırdığı konu: **Rust'ta iki farklı string türü vardır.**

| Özellik | `String` | `&str` |
|---|---|---|
| Bellek | Heap'te | Herhangi bir yerde (genellikle binary içinde) |
| Değiştirilebilirlik | Evet | Hayır |
| Boyut | Dinamik | Sabit |
| C# Karşılığı | `StringBuilder` / `String` | `ReadOnlySpan<char>` / string literal |

```rust
// String oluşturma
let mut s1 = String::from("Merhaba");
s1.push_str(" Dünya");       // Değiştirilebilir
s1.push('!');

// &str - genelde string literal
let s2 = "Sabit metin";      // Binary içine gömülü, değiştirilemez

// Dönüşümler
let s3: String = s2.to_string();
let s4: &str = &s1;          // String'den &str'e otomatik dönüşüm (Deref)
```

### String Birleştirme

```rust
let s1 = String::from("Merhaba");
let s2 = String::from(" Dünya");

// Yöntem 1: format! makrosu (C#'taki string interpolation $"{s1}{s2}" gibi)
let s3 = format!("{}{}", s1, s2);

// Yöntem 2: push_str
let mut s4 = s1.clone();
s4.push_str(&s2);
```

---

# 📘 BÖLÜM 5: Struct, Enum ve Pattern Matching

## 5.1 Struct ↔ Class/Record

**C#**:
```csharp
public class Kisi {
    public string Ad { get; set; }
    public int Yas { get; set; }
}
```

**Rust**:
```rust
struct Kisi {
    ad: String,
    yas: u8,
}

impl Kisi {
    // Constructor benzeri (associated function)
    fn yeni(ad: String, yas: u8) -> Self {
        Kisi { ad, yas }
    }
    
    // Metot (instance method)
    fn selam_ver(&self) {
        println!("Ben {}", self.ad);
    }
    
    // Mutable metot
    fn yaslan(&mut self) {
        self.yas += 1;
    }
}
```

> 💡 **Önemli:** Rust'ta `struct` ve `impl` ayrıdır. Veri ve davranış aynı blokta değildir. Bu, C#'taki sınıf mantığından farklıdır.

## 5.2 Enum ↔ Enum (Ama Çok Daha Güçlü!)

C# enum'ları sadece isimlendirilmiş sayılardır. Rust enum'ları ise **veri taşıyabilir**. Bu, Rust'ın en güçlü özelliklerinden biridir.

**C#**:
```csharp
public enum MesajTipi { Cikis, Tasi, Yaz }
```

**Rust**:
```rust
enum Mesaj {
    Cikis,                              // Veri yok
    Tasi { x: i32, y: i32 },           // Struct gibi
    Renk(u8, u8, u8),                  // Tuple gibi
    YaziYaz(String),                   // Tek değer
}

let m1 = Mesaj::Cikis;
let m2 = Mesaj::Tasi { x: 10, y: 20 };
let m3 = Mesaj::YaziYaz(String::from("Merhaba"));
```

## 5.3 Pattern Matching ↔ Switch Expression

**C#**:
```csharp
var sonuc = mesaj switch {
    Mesaj.Cikis => "Çıkış yapılıyor",
    Mesaj.Tasi(var x, var y) when x == y => "Diyagonal",
    Mesaj.Tasi(var x, _) => $"X: {x}",
    _ => "Bilinmeyen"
};
```

**Rust**:
```rust
let sonuc = match mesaj {
    Mesaj::Cikis => "Çıkış yapılıyor",
    Mesaj::Tasi { x, y } if x == y => "Diyagonal",
    Mesaj::Tasi { x, .. } => format!("X: {}", x),
    _ => "Bilinmeyen",
};
```

> 🎯 **Kritik Fark:** Rust'ın `match` ifadesi **exhaustive** (kapsayıcı) olmak zorundadır. Tüm durumları ele almazsanız derleyici hata verir. Bu, unutulmuş `case` hatalarını imkansız kılar.

---

# 📘 BÖLÜM 6: Trait'ler ve Polimorfizm (Interface Karşılığı)

## 6.1 Trait Tanımlama ↔ Interface

**C#**:
```csharp
public interface IOzetlenebilir {
    string Ozet();
}

public class Makale : IOzetlenebilir {
    public string Ozet() => "Makale özeti";
}
```

**Rust**:
```rust
trait Ozetlenebilir {
    fn ozet(&self) -> String;
}

struct Makale {
    baslik: String,
}

impl Ozetlenebilir for Makale {
    fn ozet(&self) -> String {
        format!("Makale: {}", self.baslik)
    }
}
```

## 6.2 Default Implementation (Varsayılan Metot)

Rust trait'leri, C# 8+ default interface metotları gibi **varsayılan implementasyon** içerebilir:

```rust
trait Ozetlenebilir {
    fn ozet(&self) -> String;
    
    // Varsayılan implementasyon
    fn detayli(&self) -> String {
        format!("Detay: {}", self.ozet())
    }
}
```

## 6.3 Trait Bound (Generic Kısıtlama)

**C#**:
```csharp
public void Yazdir<T>(T item) where T : IFormattable {
    Console.WriteLine(item);
}
```

**Rust**:
```rust
// Yöntem 1: Inline syntax
fn yazdir<T: std::fmt::Display>(item: T) {
    println!("{}", item);
}

// Yöntem 2: where clause (birden fazla kısıt için okunaklı)
fn karmasik<T, U>(t: T, u: U) -> String 
where 
    T: Display + Clone,
    U: Debug + Clone,
{
    format!("{} {:?}", t, u)
}
```

## 6.4 Trait Object (Dinamik Polimorfizm)

**C#**:
```csharp
public void Yazdir(IOzetlenebilir o) => Console.WriteLine(o.Ozet());
```

**Rust**:
```rust
// &dyn Trait = dinamik trait objesi (C#'taki interface referansı gibi)
fn yazdir(o: &dyn Ozetlenebilir) {
    println!("{}", o.ozet());
}
```

> 💡 **Statik vs Dinamik:** Rust'ta `impl Trait` (statik dispatch - daha hızlı) ve `dyn Trait` (dinamik dispatch - esnek) ayrımı vardır. C#'ta bu ayrım yoktur, her zaman sanal çağrı (virtual call) vardır.

---

# 📘 BÖLÜM 7: Hata Yönetimi ⭐ (EN BÜYÜK ZİHNİYET DEĞİŞİKLİĞİ)

## 7.1 Result<T, E> ↔ try-catch

**Rust'ta `try-catch` YOKTUR.** Bunun yerine `Result` enum'u kullanılır:

```rust
enum Result<T, E> {
    Ok(T),   // Başarılı, değer T
    Err(E),  // Başarısız, hata E
}
```

**C#**:
```csharp
try {
    var icerik = File.ReadAllText("dosya.txt");
} catch (FileNotFoundException ex) {
    Console.WriteLine("Dosya yok: " + ex.Message);
}
```

**Rust**:
```rust
use std::fs::File;
use std::io::Read;

fn dosya_oku(dosya_adi: &str) -> Result<String, std::io::Error> {
    let mut dosya = File::open(dosya_adi)?;  // ? operatörü - aşağıda açıklanacak
    let mut icerik = String::new();
    dosya.read_to_string(&mut icerik)?;
    Ok(icerik)
}

fn main() {
    match dosya_oku("dosya.txt") {
        Ok(icerik) => println!("İçerik: {}", icerik),
        Err(hata) => eprintln!("Hata: {}", hata),
    }
}
```

## 7.2 `?` Operatörü (Propagation Operator)

`?` operatörü, Rust'ın en zarif özelliklerinden biridir. Eğer `Result::Err` ise, **hata anında fonksiyondan döner**. Değilse, `Ok` içindeki değeri çıkarır.

```rust
fn iki_dosya_topla() -> Result<String, std::io::Error> {
    // Her ? bir try-catch bloğu gibi çalışır ama çok daha temiz
    let mut s1 = File::open("a.txt")?;
    let mut s2 = File::open("b.txt")?;
    // ...
}
```

> 🎯 **Avantaj:** C#'ta her `try` bloğunda hata türünü belirtmek gerekirken, Rust'ta `?` operatörü hatayı otomatik olarak dönüştürür (eğer `From` trait'i implemente edilmişse).

## 7.3 Option<T> ↔ Nullable Reference Types

**C#**:
```csharp
string? isim = null;
var uzunluk = isim?.Length ?? 0;
```

**Rust**:
```rust
let isim: Option<String> = None;
let uzunluk = isim.map(|s| s.len()).unwrap_or(0);

// veya
let uzunluk = match isim {
    Some(s) => s.len(),
    None => 0,
};
```

> ⚠️ **Rust'ta `null` YOKTUR.** `Option::None` vardır ve derleyici sizi onu ele almaya zorlar. Bu, "billion dollar mistake" (null referans hatası) olarak bilinen sorunu kökten çözer.

---

# 📘 BÖLÜM 8: Closure, Lambda ve Fonksiyonel Özellikler

## 8.1 Closure Sözdizimi

**C#**:
```csharp
Func<int, int> kare = x => x * x;
Func<int, int, int> topla = (x, y) => x + y;
```

**Rust**:
```rust
let kare = |x: i32| -> i32 { x * x };
let kare_kisa = |x| x * x;            // Tür çıkarımı
let topla = |x, y| x + y;
```

## 8.2 Closure'ların Çevreyi Yakalaması (Capture)

Rust closure'ları, çevrelerindeki değişkenleri üç şekilde yakalayabilir:

1. **Borrow (Ödünç):** `&x` - değişmez referans
2. **Mut Borrow:** `&mut x` - değiştirilebilir referans
3. **Move (Taşı):** `x` - sahipliği alır

```rust
let liste = vec![1, 2, 3];
let say = liste.len();

// Closure, `say` değerini yakalıyor
let closure = || println!("Uzunluk: {}", say);
closure();
```

> 💡 **`move` Anahtar Kelimesi:** Thread'lere closure gönderirken kullanılır. Çevredeki değişkenlerin sahipliğini closure'a taşır.

```rust
let mesaj = String::from("Merhaba");
let handle = std::thread::spawn(move || {
    println!("{}", mesaj);  // mesaj'ın sahipliği closure'a taşındı
});
handle.join().unwrap();
// println!("{}", mesaj); // ❌ HATA: mesaj artık bu kapsamda geçersiz
```

---

# 📘 BÖLÜM 9: Async/Await

## 9.1 Temel Async Yapısı

**C#**:
```csharp
public async Task<string> VeriIndirAsync(string url) {
    using var client = new HttpClient();
    return await client.GetStringAsync(url);
}
```

**Rust**:
```rust
async fn veri_indir(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    let icerik = response.text().await?;
    Ok(icerik)
}
```

## 9.2 Kritik Fark: Runtime

**C#**: .NET runtime (Task Scheduler) async işleri yönetir.
**Rust**: Async kod **çalışmaz** - sadece bir "future" üretir. Çalıştırmak için bir **runtime** gerekir:

- `tokio` (en popüler, C#'taki Task'a benzer)
- `async-std`
- `smol`

```rust
// Tokio ile
#[tokio::main]
async fn main() {
    let sonuc = veri_indir("https://example.com").await;
    println!("{:?}", sonuc);
}
```

## 9.3 Concurrent Görevler

**C#**:
```csharp
var t1 = VeriIndirAsync("url1");
var t2 = VeriIndirAsync("url2");
await Task.WhenAll(t1, t2);
```

**Rust**:
```rust
let t1 = tokio::spawn(veri_indir("url1"));
let t2 = tokio::spawn(veri_indir("url2"));
let (r1, r2) = tokio::join!(t1, t2);
```

---

# 📘 BÖLÜM 10: Modül Sistemi ve Visibility

## 10.1 Modül Yapısı ↔ Namespace

**C#**:
```csharp
namespace Sirket.Urun {
    public class Urun { }
}
```

**Rust** (modüler dosya yapısı):
```
src/
├── main.rs
├── lib.rs
└── urun/
    ├── mod.rs
    └── kategori.rs
```

```rust
// src/urun/mod.rs
pub mod kategori;

pub struct Urun {
    pub ad: String,        // public alan
    fiyat: f64,            // private (varsayılan)
}

impl Urun {
    pub fn yeni(ad: String, fiyat: f64) -> Self {
        Urun { ad, fiyat }
    }
}
```

## 10.2 `use` Anahtar Kelimesi ↔ `using`

**C#**:
```csharp
using System.Collections.Generic;
using System.IO;
```

**Rust**:
```rust
use std::collections::HashMap;
use std::io::{self, Read, Write};  // Gruplama

// Glob import (C#'taki using static gibi)
use std::sync::mpsc::*;

// Yeniden adlandırma (alias)
use std::io::Result as IoResult;
```

## 10.3 Visibility (Erişim Belirteçleri)

| C# | Rust | Açıklama |
|---|---|---|
| `public` | `pub` | Her yerden erişilebilir |
| `internal` | *(varsayılan)* | Sadece aynı crate içinde |
| `private` | *(varsayılan modül)* | Sadece tanımlandığı modülde |
| `protected` | `pub(crate)` veya yok | Rust'ta kalıtım olmadığı için farklı |
| `friend` | `pub(super)` | Üst modüle açık |

---

# 📘 BÖLÜM 11: Diğer Önemli Konular

## 11.1 `using` ↔ `Drop` Trait (RAII)

**C#**:
```csharp
using (var dosya = File.Open("x.txt")) {
    // kullan
} // otomatik Dispose
```

**Rust**:
```rust
{
    let dosya = File::open("x.txt").unwrap();
    // kullan
} // kapsam sonu = otomatik drop (Dispose)
```

> 💡 **RAII (Resource Acquisition Is Initialization):** Rust'ta kaynaklar, kapsam (scope) ile yönetilir. `using` bloğuna gerek yoktur - kapsam sonu otomatik olarak `drop()` çağırır.

## 11.2 Operatör Aşırı Yükleme

**C#**:
```csharp
public static Nokta operator +(Nokta a, Nokta b) => new(a.X + b.X, a.Y + b.Y);
```

**Rust**:
```rust
use std::ops::Add;

#[derive(Clone, Copy)]
struct Nokta { x: f64, y: f64 }

impl Add for Nokta {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Nokta { x: self.x + other.x, y: self.y + other.y }
    }
}

let p = Nokta { x: 1.0, y: 2.0 } + Nokta { x: 3.0, y: 4.0 };
```

## 11.3 Makrolar (Macros)

Rust'ta iki tür makro vardır:

1. **Declarative (Bildirimli):** `macro_rules!` - C#'taki T4 template'lere benzer
2. **Procedural (Yordamsal):** `#[derive]`, `#[tokio::main]` gibi attribute'lar - Roslyn Source Generators'a benzer

```rust
// println! bir makrodur, fonksiyon değil!
println!("Merhaba {}", isim);  // Tür güvenliği derleme zamanında kontrol edilir

// Özel derive makrosu (serde ile)
#[derive(Serialize, Deserialize, Debug)]
struct Config {
    port: u16,
    host: String,
}
```

---

# 🎯 ÖZET: C# Geliştiricisi İçin Rust'a Geçiş Rehberi

| Kavram | C# | Rust |
|---|---|---|
| Değişken | `var x = 5` (mutable) | `let x = 5` (immutable), `let mut x = 5` |
| Bellek Yönetimi | GC (Runtime) | Ownership + Drop (Compile-time) |
| Null | `null`, `?` | `Option<T>` |
| Hata | `try-catch`, `Exception` | `Result<T, E>`, `?` operatörü |
| Interface | `interface` | `trait` |
| Class | `class` | `struct` + `impl` |
| Enum | Sayısal | Veri taşıyabilir (algebraic) |
| Pattern Match | `switch expression` | `match` (exhaustive) |
| Koleksiyon | `List<T>`, `Dictionary` | `Vec<T>`, `HashMap` |
| LINQ | `Where`, `Select` | `iter().filter().map()` |
| Async | `Task`, `async/await` | `Future`, `async/await` + runtime |
| Namespace | `namespace` | `mod` |
| Import | `using` | `use` |
| Dispose | `using` + `IDisposable` | Kapsam sonu + `Drop` trait |
| String | `string` (UTF-16, immutable) | `String` (UTF-8, mutable) + `&str` |
| Generics | `<T> where T : I` | `<T: Trait>` |

---

# 🚀 Son Tavsiyeler

1. **Derleyici ile Savaşmayın:** Rust derleyicisi (özellikle `borrow checker`) sizin en iyi pair-programming ortağınızdır. Hata mesajları uzun ama çok açıklayıcıdır.
2. **Küçük Adımlarla İlerleyin:** ÖnceOwnership, sonra Borrowing, sonra Lifetimes sırasını takip edin.
3. **`cargo clippy` Kullanın:** C#'taki Roslyn analyzers gibi, kod kalitesini artırır.
4. **`The Rust Book` Okuyun:** Microsoft'un bu rehberi harika bir başlangıç, ama resmi kitap (doc.rust-lang.org/book) derinleşmek için şart.
5. **Pratik Yapın:** RP2354B ve step motor projeniz için Rust'ın `embedded-hal` kütüphanesi ile haşır neşir olmak, bu kavramları somutlaştırmanın en iyi yoludur.

Rust'ın öğrenme eğrisi diktir, ancak bir kez "derleyici ile düşünmeyi" öğrendiğinizde, C# dünyasında asla yakalayamayacağınız bir **güven ve performans** seviyesine ulaşırsınız. 🦀
</think>

# 🎓 Rust for .NET Developers: Kapsamlı Dil Rehberi

C# ve Rust, ilk bakışta modern, tip-güvenli (type-safe) diller gibi görünse de, arka planda yattıkları felsefeler taban tabana zıttır. C# **Çöp Toplayıcı (GC)** ile çalışma zamanında belleği yönetirken, Rust **Sahiplik (Ownership)** sistemi ile derleme zamanında bellek güvenliğini garanti eder.

Bu rehberde, her konuyu **C# → Rust** şeklinde yan yana karşılaştırmalı örneklerle, bir ders anlatır gibi inceleyeceğiz.

---

# 📘 BÖLÜM 1: Değişkenler, Tipler ve Gölgeleme (Shadowing)

## 1.1 Değişmezlik (Immutability) Varsayılandır

**C#**:
```csharp
var x = 5;
x = 6; // ✅ Başarılı - varsayılan olarak değiştirilebilir
```

**Rust**:
```rust
let x = 5;
// x = 6; // ❌ HATA: `x` değişmez (immutable)

let mut y = 5;
y = 6; // ✅ Başarılı - açıkça `mut` ile belirtildi
```

> 💡 **Neden?** Rust, değişmezliği varsayılan yaparak "yanlışlıkla değişken değiştirme" hatalarını derleme zamanında engeller. Bu, özellikle çok thread'li (multi-threaded) programlarda veri yarışlarını (data race) önlemenin ilk adımıdır.

## 1.2 Gölgeleme (Shadowing)

C#'ta aynı isimde değişken tanımlayamazsınız, ancak Rust'ta bu bir **özelliktir**. Türü değiştirerek aynı ismi yeniden kullanabilirsiniz:

**Rust**:
```rust
let spaces = "   ";           // Tür: &str (string dilimi)
let spaces = spaces.len();    // Tür: usize (sayı) - aynı isim!
let spaces = spaces as f64;   // Tür: f64 - yine aynı isim!
```

> ⚠️ **Dikkat:** `mut` ile gölgeleme farklı şeylerdir. `mut` aynı bellek bölgesini değiştirirken, gölgeleme yeni bir değişken yaratır ve eskisini kapatır.

## 1.3 Sabitler (Constants)

**C#**:
```csharp
const int MAX_BOYUT = 100;
```

**Rust**:
```rust
const MAX_BOYUT: usize = 100;  // Tür açıkça belirtilmek zorunda
```

Rust'ta `const` ile `static` arasında fark vardır:
- `const`: Her kullanımda değeri kopyalar (inline edilir).
- `static`: Bellekte tek bir sabit adres tutar (global değişken gibi).

---

# 📘 BÖLÜM 2: Ownership, Borrowing ve Lifetimes ⭐ (EN KRİTİK BÖLÜM)

Bu bölüm, C# geliştiricilerinin Rust'ta **en çok zorlandığı** ama en çok güç kazandığı bölümdür. C#'ta GC her şeyi yönetir; Rust'ta ise **her değerin tek bir sahibi (owner)** vardır ve sahip kapsam dışına çıktığında değer otomatik olarak serbest bırakılır (Drop).

## 2.1 Sahiplik Kuralları

1. Her değerin **tek bir sahibi** vardır.
2. Bir zamanda sadece **bir sahip** olabilir.
3. Sahip kapsam (scope) dışına çıkınca değer **düşürülür (drop)**.

**C#**:
```csharp
void Metod() {
    var s1 = new StringBuilder("Merhaba");
    var s2 = s1;           // s1 ve s2 aynı nesneyi gösterir (referans kopyası)
    Console.WriteLine(s1); // ✅ Çalışır - GC halleder
}
```

**Rust**:
```rust
fn main() {
    let s1 = String::from("Merhaba");
    let s2 = s1;           // ⚠️ Sahiplik s1'den s2'ye TAŞINDI (move)
    // println!("{}", s1); // ❌ HATA: s1 artık geçersiz!
    println!("{}", s2);    // ✅ Çalışır
}
```

> 💡 **Neden böyle?** Rust, "double free" (aynı belleği iki kez serbest bırakma) hatasını önlemek için sahipliği taşır. C#'taki gibi iki değişkenin aynı heap verisini paylaşmasına izin vermez.

## 2.2 Kopyalama (Clone)

Eğer gerçekten iki ayrı kopya istiyorsanız, açıkça belirtmelisiniz:

```rust
let s1 = String::from("Merhaba");
let s2 = s1.clone();       // Gerçekten derin kopya (deep copy)
println!("s1: {}, s2: {}", s1, s2); // ✅ İkisi de geçerli
```

> 🎯 **Kural:** Rust'ta "bedava" bir işlem yoktur. Pahalı bir işlem (heap kopyası) yapıyorsanız, bunu kodda **açıkça** yazmalısınız.

## 2.3 Ödünç Alma (Borrowing) ve Referanslar

Sahipliği taşımak istemiyorsanız, **referans** ile ödünç alırsınız. Bu, C#'taki referanslara benzer ama kuralları çok daha katıdır.

```rust
fn uzunluk_yaz(s: &String) {   // & = referans (ödünç alma)
    println!("{}'nin uzunluğu: {}", s, s.len());
} // s burada kapsam dışına çıkar ama sahipliği onda olmadığı için drop edilmez

fn main() {
    let s1 = String::from("Merhaba");
    uzunluk_yaz(&s1);          // &s1 = s1'in referansını ödünç ver
    println!("{}", s1);        // ✅ s1 hala geçerli - sahiplik taşınmadı
}
```

### 🚨 Ödünç Alma Kuralları (Borrow Checker)

Rust derleyicisi şu iki kuralı **asla** çiğnetmez:

1. **Ya sadece TEK BİR değiştirilebilir (mutable) referans** olabilir.
2. **YA DA istediğiniz kadar değişmez (immutable) referans** olabilir.
3. Aynı anda ikisi birden **OLAMAZ**.

**C#** (Bu kod çalışır ama tehlikelidir):
```csharp
var liste = new List<int> { 1, 2, 3 };
var referans = liste;
liste.Add(4);                // Liste değişti
Console.WriteLine(referans.Count); // Beklenmedik davranışlara yol açabilir
```

**Rust** (Derleyici bunu engeller):
```rust
let mut v = vec![1, 2, 3];
let r1 = &v;          // İlk değişmez referans
let r2 = &v;          // İkinci değişmez referans - OK
println!("{}, {}", r1, r2);
// let r3 = &mut v;   // ❌ HATA: Değişmez referanslar varken mutable referans ALAMAZSINIZ
```

## 2.4 Lifetime (Yaşam Süresi)

Rust'ın en zorlayıcı konusu: referansların **ne kadar süre geçerli** olduğunu derleyiciye bildirmek.

```rust
// 'a lifetime: döndürülen referans, en az x veya y kadar yaşamalı
fn en_uzun<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

> 💡 C#'ta GC olduğu için lifetime düşünmezsiniz. Rust'ta ise referansların "dangling" (sarkık, artık var olmayan bir değeri gösteren) olması **derleme zamanında** engellenir.

---

# 📘 BÖLÜM 3: Koleksiyonlar

## 3.1 Vektör (Vec<T>) ↔ List<T>

**C#**:
```csharp
var liste = new List<int> { 1, 2, 3 };
liste.Add(4);
var ilk = liste[0];
```

**Rust**:
```rust
let mut v = vec![1, 2, 3];
v.push(4);
let ilk = v[0];              // ✅
// let riskli = v[100];      // ❌ Runtime panic!
let guvenli = v.get(100);    // ✅ Option<&i32> döner - güvenli
```

> ⚠️ **Önemli:** Rust'ta `v[i]` erişimi sınır dışındaysa **program çöker (panic)**. Güvenli erişim için `get()` kullanın, bu size `Option<T>` döner.

### Vektör Üzerinde Döngü ve Borrow Checker

```rust
let mut v = vec![1, 2, 3];
// for x in &v { v.push(4); }  // ❌ HATA: Döngü sırasında vektörü değiştiremezsiniz!
// Çünkü döngü v'yi ödünç alıyor, push ise mutable referans ister.
```

## 3.2 HashMap ↔ Dictionary<TKey, TValue>

**C#**:
```csharp
var sozluk = new Dictionary<string, int>();
sozluk["ali"] = 25;
```

**Rust**:
```rust
use std::collections::HashMap;

let mut sozluk = HashMap::new();
sozluk.insert("ali", 25);

// Erişim
if let Some(yas) = sozluk.get("ali") {
    println!("Ali'nin yaşı: {}", yas);
}

// Varsayılan değer ile
let yas = sozluk.get("veli").copied().unwrap_or(0);
```

## 3.3 Iterator ve LINQ Benzeri İşlemler

Rust'ta LINQ yoktur ama **Iterator trait**'i ile neredeyse aynı güce sahiptir.

**C# (LINQ)**:
```csharp
var sonuc = liste
    .Where(x => x > 5)
    .Select(x => x * 2)
    .Sum();
```

**Rust**:
```rust
let sonuc: i32 = vektör
    .iter()
    .filter(|&x| *x > 5)
    .map(|&x| x * 2)
    .sum();
```

> 💡 Rust'ta iterator'lar **"lazy"** (tembel) dir - `collect()` çağrılmadıkça işlem gerçekleşmez. Bu, LINQ ile aynıdır.

---

# 📘 BÖLÜM 4: String İşlemleri

C# geliştiricilerinin en çok karıştırdığı konu: **Rust'ta iki farklı string türü vardır.**

| Özellik | `String` | `&str` |
|---|---|---|
| Bellek | Heap'te | Herhangi bir yerde (genellikle binary içinde) |
| Değiştirilebilirlik | Evet | Hayır |
| Boyut | Dinamik | Sabit |
| C# Karşılığı | `StringBuilder` / `String` | `ReadOnlySpan<char>` / string literal |

```rust
// String oluşturma
let mut s1 = String::from("Merhaba");
s1.push_str(" Dünya");       // Değiştirilebilir
s1.push('!');

// &str - genelde string literal
let s2 = "Sabit metin";      // Binary içine gömülü, değiştirilemez

// Dönüşümler
let s3: String = s2.to_string();
let s4: &str = &s1;          // String'den &str'e otomatik dönüşüm (Deref)
```

### String Birleştirme

```rust
let s1 = String::from("Merhaba");
let s2 = String::from(" Dünya");

// Yöntem 1: format! makrosu (C#'taki string interpolation $"{s1}{s2}" gibi)
let s3 = format!("{}{}", s1, s2);

// Yöntem 2: push_str
let mut s4 = s1.clone();
s4.push_str(&s2);
```

---

# 📘 BÖLÜM 5: Struct, Enum ve Pattern Matching

## 5.1 Struct ↔ Class/Record

**C#**:
```csharp
public class Kisi {
    public string Ad { get; set; }
    public int Yas { get; set; }
}
```

**Rust**:
```rust
struct Kisi {
    ad: String,
    yas: u8,
}

impl Kisi {
    // Constructor benzeri (associated function)
    fn yeni(ad: String, yas: u8) -> Self {
        Kisi { ad, yas }
    }
    
    // Metot (instance method)
    fn selam_ver(&self) {
        println!("Ben {}", self.ad);
    }
    
    // Mutable metot
    fn yaslan(&mut self) {
        self.yas += 1;
    }
}
```

> 💡 **Önemli:** Rust'ta `struct` ve `impl` ayrıdır. Veri ve davranış aynı blokta değildir. Bu, C#'taki sınıf mantığından farklıdır.

## 5.2 Enum ↔ Enum (Ama Çok Daha Güçlü!)

C# enum'ları sadece isimlendirilmiş sayılardır. Rust enum'ları ise **veri taşıyabilir**. Bu, Rust'ın en güçlü özelliklerinden biridir.

**C#**:
```csharp
public enum MesajTipi { Cikis, Tasi, Yaz }
```

**Rust**:
```rust
enum Mesaj {
    Cikis,                              // Veri yok
    Tasi { x: i32, y: i32 },           // Struct gibi
    Renk(u8, u8, u8),                  // Tuple gibi
    YaziYaz(String),                   // Tek değer
}

let m1 = Mesaj::Cikis;
let m2 = Mesaj::Tasi { x: 10, y: 20 };
let m3 = Mesaj::YaziYaz(String::from("Merhaba"));
```

## 5.3 Pattern Matching ↔ Switch Expression

**C#**:
```csharp
var sonuc = mesaj switch {
    Mesaj.Cikis => "Çıkış yapılıyor",
    Mesaj.Tasi(var x, var y) when x == y => "Diyagonal",
    Mesaj.Tasi(var x, _) => $"X: {x}",
    _ => "Bilinmeyen"
};
```

**Rust**:
```rust
let sonuc = match mesaj {
    Mesaj::Cikis => "Çıkış yapılıyor",
    Mesaj::Tasi { x, y } if x == y => "Diyagonal",
    Mesaj::Tasi { x, .. } => format!("X: {}", x),
    _ => "Bilinmeyen",
};
```

> 🎯 **Kritik Fark:** Rust'ın `match` ifadesi **exhaustive** (kapsayıcı) olmak zorundadır. Tüm durumları ele almazsanız derleyici hata verir. Bu, unutulmuş `case` hatalarını imkansız kılar.

---

# 📘 BÖLÜM 6: Trait'ler ve Polimorfizm (Interface Karşılığı)

## 6.1 Trait Tanımlama ↔ Interface

**C#**:
```csharp
public interface IOzetlenebilir {
    string Ozet();
}

public class Makale : IOzetlenebilir {
    public string Ozet() => "Makale özeti";
}
```

**Rust**:
```rust
trait Ozetlenebilir {
    fn ozet(&self) -> String;
}

struct Makale {
    baslik: String,
}

impl Ozetlenebilir for Makale {
    fn ozet(&self) -> String {
        format!("Makale: {}", self.baslik)
    }
}
```

## 6.2 Default Implementation (Varsayılan Metot)

Rust trait'leri, C# 8+ default interface metotları gibi **varsayılan implementasyon** içerebilir:

```rust
trait Ozetlenebilir {
    fn ozet(&self) -> String;
    
    // Varsayılan implementasyon
    fn detayli(&self) -> String {
        format!("Detay: {}", self.ozet())
    }
}
```

## 6.3 Trait Bound (Generic Kısıtlama)

**C#**:
```csharp
public void Yazdir<T>(T item) where T : IFormattable {
    Console.WriteLine(item);
}
```

**Rust**:
```rust
// Yöntem 1: Inline syntax
fn yazdir<T: std::fmt::Display>(item: T) {
    println!("{}", item);
}

// Yöntem 2: where clause (birden fazla kısıt için okunaklı)
fn karmasik<T, U>(t: T, u: U) -> String 
where 
    T: Display + Clone,
    U: Debug + Clone,
{
    format!("{} {:?}", t, u)
}
```

## 6.4 Trait Object (Dinamik Polimorfizm)

**C#**:
```csharp
public void Yazdir(IOzetlenebilir o) => Console.WriteLine(o.Ozet());
```

**Rust**:
```rust
// &dyn Trait = dinamik trait objesi (C#'taki interface referansı gibi)
fn yazdir(o: &dyn Ozetlenebilir) {
    println!("{}", o.ozet());
}
```

> 💡 **Statik vs Dinamik:** Rust'ta `impl Trait` (statik dispatch - daha hızlı) ve `dyn Trait` (dinamik dispatch - esnek) ayrımı vardır. C#'ta bu ayrım yoktur, her zaman sanal çağrı (virtual call) vardır.

---

# 📘 BÖLÜM 7: Hata Yönetimi ⭐ (EN BÜYÜK ZİHNİYET DEĞİŞİKLİĞİ)

## 7.1 Result<T, E> ↔ try-catch

**Rust'ta `try-catch` YOKTUR.** Bunun yerine `Result` enum'u kullanılır:

```rust
enum Result<T, E> {
    Ok(T),   // Başarılı, değer T
    Err(E),  // Başarısız, hata E
}
```

**C#**:
```csharp
try {
    var icerik = File.ReadAllText("dosya.txt");
} catch (FileNotFoundException ex) {
    Console.WriteLine("Dosya yok: " + ex.Message);
}
```

**Rust**:
```rust
use std::fs::File;
use std::io::Read;

fn dosya_oku(dosya_adi: &str) -> Result<String, std::io::Error> {
    let mut dosya = File::open(dosya_adi)?;  // ? operatörü - aşağıda açıklanacak
    let mut icerik = String::new();
    dosya.read_to_string(&mut icerik)?;
    Ok(icerik)
}

fn main() {
    match dosya_oku("dosya.txt") {
        Ok(icerik) => println!("İçerik: {}", icerik),
        Err(hata) => eprintln!("Hata: {}", hata),
    }
}
```

## 7.2 `?` Operatörü (Propagation Operator)

`?` operatörü, Rust'ın en zarif özelliklerinden biridir. Eğer `Result::Err` ise, **hata anında fonksiyondan döner**. Değilse, `Ok` içindeki değeri çıkarır.

```rust
fn iki_dosya_topla() -> Result<String, std::io::Error> {
    // Her ? bir try-catch bloğu gibi çalışır ama çok daha temiz
    let mut s1 = File::open("a.txt")?;
    let mut s2 = File::open("b.txt")?;
    // ...
}
```

> 🎯 **Avantaj:** C#'ta her `try` bloğunda hata türünü belirtmek gerekirken, Rust'ta `?` operatörü hatayı otomatik olarak dönüştürür (eğer `From` trait'i implemente edilmişse).

## 7.3 Option<T> ↔ Nullable Reference Types

**C#**:
```csharp
string? isim = null;
var uzunluk = isim?.Length ?? 0;
```

**Rust**:
```rust
let isim: Option<String> = None;
let uzunluk = isim.map(|s| s.len()).unwrap_or(0);

// veya
let uzunluk = match isim {
    Some(s) => s.len(),
    None => 0,
};
```

> ⚠️ **Rust'ta `null` YOKTUR.** `Option::None` vardır ve derleyici sizi onu ele almaya zorlar. Bu, "billion dollar mistake" (null referans hatası) olarak bilinen sorunu kökten çözer.

---

# 📘 BÖLÜM 8: Closure, Lambda ve Fonksiyonel Özellikler

## 8.1 Closure Sözdizimi

**C#**:
```csharp
Func<int, int> kare = x => x * x;
Func<int, int, int> topla = (x, y) => x + y;
```

**Rust**:
```rust
let kare = |x: i32| -> i32 { x * x };
let kare_kisa = |x| x * x;            // Tür çıkarımı
let topla = |x, y| x + y;
```

## 8.2 Closure'ların Çevreyi Yakalaması (Capture)

Rust closure'ları, çevrelerindeki değişkenleri üç şekilde yakalayabilir:

1. **Borrow (Ödünç):** `&x` - değişmez referans
2. **Mut Borrow:** `&mut x` - değiştirilebilir referans
3. **Move (Taşı):** `x` - sahipliği alır

```rust
let liste = vec![1, 2, 3];
let say = liste.len();

// Closure, `say` değerini yakalıyor
let closure = || println!("Uzunluk: {}", say);
closure();
```

> 💡 **`move` Anahtar Kelimesi:** Thread'lere closure gönderirken kullanılır. Çevredeki değişkenlerin sahipliğini closure'a taşır.

```rust
let mesaj = String::from("Merhaba");
let handle = std::thread::spawn(move || {
    println!("{}", mesaj);  // mesaj'ın sahipliği closure'a taşındı
});
handle.join().unwrap();
// println!("{}", mesaj); // ❌ HATA: mesaj artık bu kapsamda geçersiz
```

---

# 📘 BÖLÜM 9: Async/Await

## 9.1 Temel Async Yapısı

**C#**:
```csharp
public async Task<string> VeriIndirAsync(string url) {
    using var client = new HttpClient();
    return await client.GetStringAsync(url);
}
```

**Rust**:
```rust
async fn veri_indir(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    let icerik = response.text().await?;
    Ok(icerik)
}
```

## 9.2 Kritik Fark: Runtime

**C#**: .NET runtime (Task Scheduler) async işleri yönetir.
**Rust**: Async kod **çalışmaz** - sadece bir "future" üretir. Çalıştırmak için bir **runtime** gerekir:

- `tokio` (en popüler, C#'taki Task'a benzer)
- `async-std`
- `smol`

```rust
// Tokio ile
#[tokio::main]
async fn main() {
    let sonuc = veri_indir("https://example.com").await;
    println!("{:?}", sonuc);
}
```

## 9.3 Concurrent Görevler

**C#**:
```csharp
var t1 = VeriIndirAsync("url1");
var t2 = VeriIndirAsync("url2");
await Task.WhenAll(t1, t2);
```

**Rust**:
```rust
let t1 = tokio::spawn(veri_indir("url1"));
let t2 = tokio::spawn(veri_indir("url2"));
let (r1, r2) = tokio::join!(t1, t2);
```

---

# 📘 BÖLÜM 10: Modül Sistemi ve Visibility

## 10.1 Modül Yapısı ↔ Namespace

**C#**:
```csharp
namespace Sirket.Urun {
    public class Urun { }
}
```

**Rust** (modüler dosya yapısı):
```
src/
├── main.rs
├── lib.rs
└── urun/
    ├── mod.rs
    └── kategori.rs
```

```rust
// src/urun/mod.rs
pub mod kategori;

pub struct Urun {
    pub ad: String,        // public alan
    fiyat: f64,            // private (varsayılan)
}

impl Urun {
    pub fn yeni(ad: String, fiyat: f64) -> Self {
        Urun { ad, fiyat }
    }
}
```

## 10.2 `use` Anahtar Kelimesi ↔ `using`

**C#**:
```csharp
using System.Collections.Generic;
using System.IO;
```

**Rust**:
```rust
use std::collections::HashMap;
use std::io::{self, Read, Write};  // Gruplama

// Glob import (C#'taki using static gibi)
use std::sync::mpsc::*;

// Yeniden adlandırma (alias)
use std::io::Result as IoResult;
```

## 10.3 Visibility (Erişim Belirteçleri)

| C# | Rust | Açıklama |
|---|---|---|
| `public` | `pub` | Her yerden erişilebilir |
| `internal` | *(varsayılan)* | Sadece aynı crate içinde |
| `private` | *(varsayılan modül)* | Sadece tanımlandığı modülde |
| `protected` | `pub(crate)` veya yok | Rust'ta kalıtım olmadığı için farklı |
| `friend` | `pub(super)` | Üst modüle açık |

---

# 📘 BÖLÜM 11: Diğer Önemli Konular

## 11.1 `using` ↔ `Drop` Trait (RAII)

**C#**:
```csharp
using (var dosya = File.Open("x.txt")) {
    // kullan
} // otomatik Dispose
```

**Rust**:
```rust
{
    let dosya = File::open("x.txt").unwrap();
    // kullan
} // kapsam sonu = otomatik drop (Dispose)
```

> 💡 **RAII (Resource Acquisition Is Initialization):** Rust'ta kaynaklar, kapsam (scope) ile yönetilir. `using` bloğuna gerek yoktur - kapsam sonu otomatik olarak `drop()` çağırır.

## 11.2 Operatör Aşırı Yükleme

**C#**:
```csharp
public static Nokta operator +(Nokta a, Nokta b) => new(a.X + b.X, a.Y + b.Y);
```

**Rust**:
```rust
use std::ops::Add;

#[derive(Clone, Copy)]
struct Nokta { x: f64, y: f64 }

impl Add for Nokta {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Nokta { x: self.x + other.x, y: self.y + other.y }
    }
}

let p = Nokta { x: 1.0, y: 2.0 } + Nokta { x: 3.0, y: 4.0 };
```

## 11.3 Makrolar (Macros)

Rust'ta iki tür makro vardır:

1. **Declarative (Bildirimli):** `macro_rules!` - C#'taki T4 template'lere benzer
2. **Procedural (Yordamsal):** `#[derive]`, `#[tokio::main]` gibi attribute'lar - Roslyn Source Generators'a benzer

```rust
// println! bir makrodur, fonksiyon değil!
println!("Merhaba {}", isim);  // Tür güvenliği derleme zamanında kontrol edilir

// Özel derive makrosu (serde ile)
#[derive(Serialize, Deserialize, Debug)]
struct Config {
    port: u16,
    host: String,
}
```

---

# 🎯 ÖZET: C# Geliştiricisi İçin Rust'a Geçiş Rehberi

| Kavram | C# | Rust |
|---|---|---|
| Değişken | `var x = 5` (mutable) | `let x = 5` (immutable), `let mut x = 5` |
| Bellek Yönetimi | GC (Runtime) | Ownership + Drop (Compile-time) |
| Null | `null`, `?` | `Option<T>` |
| Hata | `try-catch`, `Exception` | `Result<T, E>`, `?` operatörü |
| Interface | `interface` | `trait` |
| Class | `class` | `struct` + `impl` |
| Enum | Sayısal | Veri taşıyabilir (algebraic) |
| Pattern Match | `switch expression` | `match` (exhaustive) |
| Koleksiyon | `List<T>`, `Dictionary` | `Vec<T>`, `HashMap` |
| LINQ | `Where`, `Select` | `iter().filter().map()` |
| Async | `Task`, `async/await` | `Future`, `async/await` + runtime |
| Namespace | `namespace` | `mod` |
| Import | `using` | `use` |
| Dispose | `using` + `IDisposable` | Kapsam sonu + `Drop` trait |
| String | `string` (UTF-16, immutable) | `String` (UTF-8, mutable) + `&str` |
| Generics | `<T> where T : I` | `<T: Trait>` |

---

# 🚀 Son Tavsiyeler

1. **Derleyici ile Savaşmayın:** Rust derleyicisi (özellikle `borrow checker`) sizin en iyi pair-programming ortağınızdır. Hata mesajları uzun ama çok açıklayıcıdır.
2. **Küçük Adımlarla İlerleyin:** Önce Ownership, sonra Borrowing, sonra Lifetimes sırasını takip edin.
3. **`cargo clippy` Kullanın:** C#'taki Roslyn analyzers gibi, kod kalitesini artırır.
4. **`The Rust Book` Okuyun:** Microsoft'un bu rehberi harika bir başlangıç, ama resmi kitap (doc.rust-lang.org/book) derinleşmek için şart.
5. **Pratik Yapın:** RP2354B ve step motor projeniz için Rust'ın `embedded-hal` kütüphanesi ile haşır neşir olmak, bu kavramları somutlaştırmanın en iyi yoludur.

Rust'ın öğrenme eğrisi diktir, ancak bir kez "derleyici ile düşünmeyi" öğrendiğinizde, C# dünyasında asla yakalayamayacağınız bir **güven ve performans** seviyesine ulaşırsınız. 🦀

