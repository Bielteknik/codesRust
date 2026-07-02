# 🧠 Rust for .NET Developers: Bellek Yönetimi (Memory Management)

C# ve Rust arasındaki **en temel felsefe farkı** bellek yönetiminde ortaya çıkar. Bu bölüm, Rust'ın kalbi olan ve C# geliştiricilerinin zihniyetini tamamen değiştiren konuları içerir.

> 🎯 **Temel Fark:** C# **Çöp Toplayıcı (GC)** ile çalışma zamanında (runtime) belleği yönetir. Rust ise **Sahiplik (Ownership)** sistemi ile derleme zamanında (compile-time) bellek güvenliğini garanti eder. GC yoktur, bu yüzden "pause the world" (dünyayı durdurma) gibi GC kaynaklı performans kayıpları da yoktur.

---

# 📚 BÖLÜM 1: Stack vs Heap (Yığın vs Öbek)

Hem C# hem Rust aynı temel bellek mimarisini kullanır, ancak veriyi nereye koydukları farklıdır.

## 1.1 Stack (Yığın)

- **Sabit boyutlu**, hızlı erişimli bellek bölgesi.
- LIFO (Last In, First Out) prensibiyle çalışır.
- Her thread'in kendi stack'i vardır.
- **Kural:** Stack'e konan verinin boyutu **derleme zamanında bilinmelidir**.

## 1.2 Heap (Öbek/Yığın)

- **Dinamik boyutlu**, daha yavaş erişimli bellek bölgesi.
- Tüm thread'ler tarafından paylaşılır.
- Bellek tahsisi (allocation) ve serbest bırakma (deallocation) maliyetlidir.

## 1.3 C# vs Rust Karşılaştırması

**C#**:
```csharp
// Stack'te (değer tipleri)
int x = 42;                    // 4 byte stack'te
var p = new Point(10, 20);     // Point nesnesi HEAP'te, p referansı stack'te

// GC hepsini yönetir
```

**Rust**:
```rust
// Stack'te
let x: i32 = 42;               // 4 byte stack'te

// Heap'te (dinamik veri)
let s: String = String::from("Merhaba");  
// s (pointer, len, capacity) → stack'te (24 byte)
// "Merhaba" verisi → heap'te
// capacity, uzunluk ve gerçek veri ayrı yerlerde!
```

> 💡 **Neden Önemli?** Rust'ta `String` (heap) ile `&str` (genellikle stack/binary) arasındaki farkı anlamak, bellek yönetiminin anahtarıdır.

---

# 📚 BÖLÜM 2: Ownership (Sahiplik) Sistemi ⭐

Rust'ın **en devrimci** özelliği. GC olmadan bellek güvenliğini sağlayan üç kural bütünüdür.

## 2.1 Üç Altın Kural

```
1. Rust'taki her değerin SAHİBİ (owner) denilen tek bir değişkeni vardır.
2. Bir anda sadece BİR SAHİP olabilir.
3. Sahip kapsam (scope) dışına çıkınca, değer OTOMATİK OLARAK düşürülür (drop).
```

## 2.2 Değişken Kapsamı (Variable Scope)

**C#**:
```csharp
{
    var s = new StringBuilder("Merhaba");
    Console.WriteLine(s);
} // s burada GC tarafından "bir ara" temizlenir - kesin zaman belirsiz
```

**Rust**:
```rust
{
    let s = String::from("Merhaba");
    println!("{}", s);
} // s TAM OLARAK burada düşürülür (drop) - deterministik!
  // Bellek anında serbest bırakılır
```

> 🎯 **RAII (Resource Acquisition Is Initialization):** Rust'ta kaynak yönetimi kapsam (scope) ile yapılır. C#'taki `using` bloğuna gerek yoktur - her şey otomatiktir.

---

# 📚 BÖLÜM 3: Move Semantics (Taşıma Semantiği)

Bu, C# geliştiricilerinin **en çok şaşırdığı** konudur.

## 3.1 Atama = Sahiplik Taşıma

**C#** (Referans Kopyalama):
```csharp
var s1 = new StringBuilder("Merhaba");
var s2 = s1;                    // s1 ve s2 AYNI nesneyi gösterir
Console.WriteLine(s1);          // ✅ Çalışır - iki referans da geçerli
```

**Rust** (Sahiplik Taşıma):
```rust
let s1 = String::from("Merhaba");
let s2 = s1;                    // ⚠️ Sahiplik s1'den s2'ye TAŞINDI
// println!("{}", s1);          // ❌ HATA: s1 artık geçersiz!
println!("{}", s2);             // ✅ Çalışır
```

> 💡 **Neden?** Rust, "double free" (aynı belleği iki kez serbest bırakma) hatasını önlemek için sahipliği taşır. İki değişken aynı heap verisini **asla** paylaşamaz.

## 3.2 Stack Verileri: Copy Semantics

Stack'teki basit tipler (integer, bool, char) **kopyalanır**, taşınmaz. Çünkü kopyalama ucuzdur.

```rust
let x = 5;
let y = x;                      // x kopyalandı (copy)
println!("x: {}, y: {}", x, y); // ✅ İkisi de geçerli - x hala kullanılabilir
```

> 🎯 **Kural:** `Copy` trait'i implemente eden tipler (tüm primitive'ler, `&str`, `Copy` içeren tuple'lar) taşınmaz, kopyalanır.

---

# 📚 BÖLÜM 4: Clone (Derin Kopya)

Eğer gerçekten heap verisinin iki ayrı kopyasını istiyorsanız, **açıkça** belirtmelisiniz:

```rust
let s1 = String::from("Merhaba");
let s2 = s1.clone();            // Gerçekten derin kopya (deep copy)
println!("s1: {}, s2: {}", s1, s2); // ✅ İkisi de geçerli, ikisi de farklı bellek
```

> ⚠️ **Performans Uyarısı:** `clone()` pahalı bir işlemdir. Rust felsefesi: **"Pahalı işlemler görünür olmalıdır."** Kodu okuyan biri `clone()` gördüğünde "burada ciddi bir kopyalama var" diye anlamalıdır.

---

# 📚 BÖLÜM 5: Borrowing (Ödünç Alma) ve Referanslar

Sahipliği taşımak istemiyorsanız, veriyi **ödünç alırsınız**. Bu, C#'taki referanslara benzer ama çok daha katı kuralları vardır.

## 5.1 Değişmez Referans (&T)

```rust
fn uzunluk_yaz(s: &String) {     // & = referans (ödünç alma)
    println!("{}'nin uzunluğu: {}", s, s.len());
} // s burada kapsam dışına çıkar ama sahipliği onda olmadığı için drop edilmez

fn main() {
    let s1 = String::from("Merhaba");
    uzunluk_yaz(&s1);            // &s1 = s1'in referansını ödünç ver
    println!("{}", s1);          // ✅ s1 hala geçerli - sahiplik taşınmadı
}
```

## 5.2 Değiştirilebilir Referans (&mut T)

Veriyi değiştirmek istiyorsanız `&mut` kullanırsınız:

```rust
fn sonuna_ekle(s: &mut String) {
    s.push_str(" Dünya");
}

fn main() {
    let mut s1 = String::from("Merhaba");
    sonuna_ekle(&mut s1);
    println!("{}", s1);          // "Merhaba Dünya"
}
```

> ⚠️ **Dikkat:** `&mut` alabilmek için orijinal değişkenin `mut` olması gerekir.

---

# 📚 BÖLÜM 6: Borrow Checker Kuralları 🚨

Rust derleyicisi (borrow checker) şu iki kuralı **ASLA** çiğnetmez:

```
1. Ya sadece TEK BİR değiştirilebilir (&mut) referans olabilir.
2. Ya da istediğiniz kadar değişmez (&) referans olabilir.
3. Aynı anda ikisi birden O-LA-MAZ!
```

## 6.1 Neden Bu Kural Var?

**Senaryo 1: Iterator Invalidation**
```rust
let mut v = vec![1, 2, 3];
let r1 = &v;                     // Değişmez referans
let r2 = &mut v;                 // ❌ HATA: r1 hala aktifken mutable referans alamazsınız
r2.push(4);
println!("{}", r1[0]);           // Eğer izin verilse, r1 artık geçersiz olurdu!
```

**Senaryo 2: Data Race (Veri Yarışı)**
```rust
// İki thread aynı anda aynı veriyi değiştirirse ne olur?
// Rust bunu derleme zamanında engeller!
```

## 6.2 C# vs Rust

**C#** (Bu kod çalışır ama tehlikelidir):
```csharp
var liste = new List<int> { 1, 2, 3 };
var referans = liste;
liste.Add(4);
Console.WriteLine(referans.Count); // Beklenmedik sonuçlara yol açabilir
```

**Rust** (Derleyici bunu engeller):
```rust
let mut v = vec![1, 2, 3];
let r1 = &v;
let r2 = &v;                      // ✅ Birden fazla değişmez referans OK
println!("{}, {}", r1[0], r2[0]);
// let r3 = &mut v;               // ❌ HATA!
```

## 6.3 NLL (Non-Lexical Lifetimes)

Modern Rust, referansların yaşam süresini **son kullanıma** göre belirler:

```rust
let mut v = vec![1, 2, 3];
let r1 = &v;
println!("{}", r1[0]);           // r1'in son kullanımı burada
let r2 = &mut v;                 // ✅ OK - r1 artık kullanılmıyor
r2.push(4);
```

---

# 📚 BÖLÜM 7: Lifetimes (Yaşam Süreleri) 🧬

Rust'ın **en zorlayıcı** konusu. Referansların **ne kadar süre geçerli** olduğunu derleyiciye bildirmek için kullanılır.

## 7.1 Neden Lifetime Gerekli?

```rust
// Derleyici hangisinin daha uzun yaşadığını nasıl bilecek?
fn en_uzun(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
// ❌ HATA: Döndürülen referansın lifetime'ı belirsiz
```

## 7.2 Lifetime Annotation Sözdizimi

```rust
// 'a lifetime: döndürülen referans, en az x veya y kadar yaşamalı
fn en_uzun<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

> 💡 Lifetime'lar referansların **ne kadar yaşadığını değiştirmez**, sadece derleyiciye **ilişkiyi** anlatır. C#'ta GC olduğu için lifetime düşünmezsiniz; Rust'ta ise "dangling reference" (sarkık referans) **derleme zamanında** engellenir.

## 7.3 Lifetime Elision Kuralları

Derleyici bazı durumlarda lifetime'ları otomatik çıkarır:

1. Her referans parametreye kendi lifetime'ı verilir.
2. Tek giriş lifetime'ı varsa, tüm çıkış lifetime'ları ona eşitlenir.
3. `&self` veya `&mut self` varsa, çıkış lifetime'ı self'e eşitlenir.

```rust
// Bu iki satır aynı şeydir:
fn ilk_kelime(s: &str) -> &str;
fn ilk_kelime<'a>(s: &'a str) -> &'a str;
```

## 7.4 Struct'larda Lifetime

Bir struct referans tutuyorsa, lifetime belirtmek **zorunludur**:

```rust
// ❌ HATA
struct Metin {
    icerik: &str,
}

// ✅ Doğru
struct Metin<'a> {
    icerik: &'a str,
}

impl<'a> Metin<'a> {
    fn en_uzun_kelime(&self) -> &str {
        // ...
    }
}
```

## 7.5 `'static` Lifetime

Programın **tamamı boyunca** geçerli olan referanslar için kullanılır:

```rust
let s: &'static str = "Bu string binary içinde gömülü, asla drop edilmez";
```

---

# 📚 BÖLÜM 8: Drop Trait (Dispose Karşılığı)

C#'taki `IDisposable` ve `using` bloğunun Rust'taki karşılığıdır.

## 8.1 Drop Nasıl Çalışır?

```rust
struct Kaynak {
    isim: String,
}

impl Drop for Kaynak {
    fn drop(&mut self) {
        println!("{} serbest bırakılıyor!", self.isim);
    }
}

fn main() {
    let k1 = Kaynak { isim: String::from("DB Bağlantısı") };
    let k2 = Kaynak { isim: String::from("Dosya Tanıtıcısı") };
    println!("Program çalışıyor...");
}
// Çıktı sırası:
// Program çalışıyor...
// Dosya Tanıtıcısı serbest bırakılıyor!  ← k2 önce (LIFO)
// DB Bağlantısı serbest bırakılıyor!     ← k1 sonra
```

> 🎯 **Kritik Fark:** C#'ta `Dispose` çağrısını unutmak mümkündür (bu yüzden `using` gerekir). Rust'ta `drop` **her zaman** otomatik çağrılır - unutmak imkansızdır.

## 8.2 `std::mem::drop` Fonksiyonu

Bir değeri kapsamından önce manuel olarak düşürmek için:

```rust
let s = String::from("Merhaba");
// s ile işimiz bitti, belleği hemen serbest bırak
drop(s);
// println!("{}", s);  // ❌ HATA: s artık kullanılamaz
```

---

# 📚 BÖLÜM 9: Smart Pointers (Akıllı İşaretçiler) 🎯

Bazı durumlarda sahiplik kurallarını esnetmek gerekir. Rust bunun için **Smart Pointer**'lar sunar.

## 9.1 `Box<T>` - Heap'e Yükleme

Değeri heap'te tutar, stack'te sadece işaretçi olur:

```rust
enum Liste {
    Kons(Box<Liste>, i32),   // Recursive tip - Box şart!
    Son,
}

let liste = Liste::Kons(Box::new(Liste::Son), 5);
```

**Kullanım Alanları:**
- Recursive tipler (boyut derleme zamanında bilinemez)
- Çok büyük verileri stack yerine heap'te tutmak
- Trait object'ler (`Box<dyn Trait>`)

## 9.2 `Rc<T>` - Referans Sayımı (Reference Counting)

**Tek thread** ortamında birden fazla sahiplik sağlar:

```rust
use std::rc::Rc;

let a = Rc::new(String::from("Merhaba"));
let b = Rc::clone(&a);         // Referans sayısı: 2
let c = Rc::clone(&a);         // Referans sayısı: 3

println!("Referans sayısı: {}", Rc::strong_count(&a)); // 3

drop(c);
drop(b);
println!("Referans sayısı: {}", Rc::strong_count(&a)); // 1
// a da drop olunca bellek serbest bırakılır
```

> ⚠️ **Dikkat:** `Rc<T>` thread-safe **değildir**. Multi-thread için `Arc<T>` kullanılır.

## 9.3 `Arc<T>` - Atomik Referans Sayımı

`Rc<T>`'nin thread-safe versiyonu:

```rust
use std::sync::Arc;
use std::thread;

let veri = Arc::new(vec![1, 2, 3]);

for i in 0..3 {
    let veri_kopya = Arc::clone(&veri);
    thread::spawn(move || {
        println!("Thread {}: {:?}", i, veri_kopya);
    });
}
```

> 💡 **Performans Notu:** `Arc<T>`, `Rc<T>`'ye göre daha yavaştır çünkü atomik işlemler kullanır. Tek thread ortamında `Rc<T>` tercih edin.

## 9.4 `RefCell<T>` - İç Değişkenlik (Interior Mutability)

**Değişmez** bir referans üzerinden veriyi **değiştirmenizi** sağlar. Borrow checker kurallarını **runtime'da** kontrol eder:

```rust
use std::cell::RefCell;

let veri = RefCell::new(String::from("Merhaba"));

// Borrows: Değişmez ödünç alma
{
    let r = veri.borrow();
    println!("{}", r);
}

// Borrow_mut: Değiştirilebilir ödünç alma
{
    let mut w = veri.borrow_mut();
    w.push_str(" Dünya");
}

// Eğer aynı anda hem borrow hem borrow_mut çağrılırsa → runtime PANIC!
```

> 🎯 **Kritik Fark:**
> - `&mut T` → Derleme zamanında kontrol (güvenli)
> - `RefCell<T>` → Çalışma zamanında kontrol (panik riski, ama esnek)

## 9.5 Smart Pointer Özet Tablosu

| Pointer | Sahiplik | Thread-Safe | Değişkenlik | Kullanım |
|---|---|---|---|---|
| `Box<T>` | Tek sahip | ✅ | Normal | Heap allocation, recursive types |
| `Rc<T>` | Çoklu sahip | ❌ | Sadece okuma | Tek thread, paylaşılan veri |
| `Arc<T>` | Çoklu sahip | ✅ | Sadece okuma | Multi-thread, paylaşılan veri |
| `RefCell<T>` | Tek sahip | ❌ | İç değişkenlik | Runtime borrow kontrolü |
| `Mutex<T>` | Tek sahip | ✅ | İç değişkenlik | Thread-safe mutable erişim |
| `RwLock<T>` | Tek sahip | ✅ | İç değişkenlik | Çoklu okuma, tek yazma |

---

# 📚 BÖLÜM 10: Garbage Collection vs Rust

## 10.1 C# GC Nasıl Çalışır?

1. **Managed Heap**: Tüm referans tipleri burada.
2. **Generations** (0, 1, 2): Kısa ömürlü nesneler hızlı temizlenir.
3. **Mark and Sweep**: GC, erişilemeyen nesneleri işaretler ve temizler.
4. **Pause the World**: GC çalışırken uygulama DURAKLAR.

**Sorunlar:**
- Tahmin edilemeyen duraksamalar (latency spike)
- Bellek kullanımı yüksek
- CPU tüketimi
- Finalizer'lar belirsiz zamanda çalışır

## 10.2 Rust'ın Yaklaşımı

- **GC YOKTUR.**
- Her bellek, kapsam sonu geldiğinde **deterministik** olarak serbest bırakılır.
- Derleme zamanında kontrol → **Runtime maliyeti sıfır**.
- Tahmin edilebilir performans (real-time sistemler için ideal).

## 10.3 Karşılaştırma Tablosu

| Özellik | C# (GC) | Rust |
|---|---|---|
| Bellek Yönetimi | Runtime (GC) | Compile-time (Ownership) |
| Performans Tahmini | Zor (GC pause) | Kolay (deterministik) |
| Bellek Sızıntısı | Mümkün (event handler'lar) | İmkansıza yakın |
| Use-After-Free | Nadir (GC korur) | Derleyici engeller |
| Double-Free | İmkansıZ | Derleyici engeller |
| Data Race | Runtime kontrolü | Derleyici engeller |
| Öğrenme Eğrisi | Kolay | Dik |
| Geliştirme Hızı | Hızlı | Yavaş başlar, sonra hızlanır |
| Real-Time Uygunluk | Zor | Mükemmel |

---

# 📚 BÖLÜM 11: Pratik Bellek Yönetimi Desenleri

## 11.1 String Döngü Problemi

**Yanlış** (Derlenmez):
```rust
let mut v = vec![String::from("a"), String::from("b")];
for s in &v {
    v.push(String::from("c"));  // ❌ HATA: v ödünç alınmışken değiştiremezsiniz
}
```

**Doğru**:
```rust
let mut v = vec![String::from("a"), String::from("b")];
let yeni = String::from("c");
v.push(yeni);  // Döngü dışında ekle
```

## 11.2 Struct'ta Self Referans (Zor!)

Rust'ta bir struct'ın kendi üyesine referans vermesi **çok zordur**:

```rust
// ❌ Bu çalışmaz - lifetime cehennemi
struct SelfRef<'a> {
    veri: String,
    referans: &'a str,  // veri'ye işaret etmeli
}
```

**Çözüm**: Ya `Rc`/`Arc` kullanın ya da indeks ile çalışın:

```rust
struct SelfRef {
    veri: String,
    referans_baslangic: usize,
    referans_bitis: usize,
}
```

## 11.3 Global Değişkenler

Rust'ta mutable global değişkenler **tehlikeli** ve kısıtlıdır:

```rust
// Immutable global - OK
static MAX_BOYUT: usize = 100;

// Mutable global - unsafe gerekli!
use std::sync::Mutex;
static SAYAC: Mutex<i32> = Mutex::new(0);

fn artir() {
    let mut s = SAYAC.lock().unwrap();
    *s += 1;
}
```

> 💡 **Tavsiye:** Global mutable state yerine **dependency injection** veya **context passing** kullanın.

---

# 📚 BÖLÜM 12: Embedded Sistemlerde Bellek Yönetimi 🎯

RP2354B ve step motor projeniz için bu bölüm kritik önem taşır.

## 12.1 Allocator-Free (Tahsisçisiz) Programlama

Embedded sistemlerde genellikle heap kullanılmaz. Bunun yerine:

```rust
// Static allocation
static mut MOTOR_POZISYONU: i32 = 0;

// Stack-based buffers
let mut buffer: [u8; 1024] = [0; 1024];  // Stack'te 1KB

// Static mut yerine güvenli alternatifler
use core::cell::UnsafeCell;
static POZISYON: UnsafeCell<i32> = UnsafeCell::new(0);
```

## 12.2 `no_std` Ortamı

Embedded Rust genellikle `no_std` ile çalışır (standart kütüphane yok):

```rust
#![no_std]
#![no_main]

// String ve Vec YOK (heap yok)
// Bunun yerine:
use heapless::Vec;     // Sabit kapasiteli vektör
use heapless::String;  // Sabit kapasiteli string

let mut v: Vec<i32, 16> = Vec::new();  // Max 16 eleman, stack'te
v.push(42).unwrap();
```

## 12.3 Step Motor Projesi İçin Bellek Stratejisi

```rust
// ❌ Kötü: Dinamik allocation her adımda
fn adim_at() {
    let komut = String::from("STEP");  // Her çağrıda heap allocation!
}

// ✅ İyi: Static veya stack-based
static ADIM_KOMUTU: &[u8] = b"STEP";  // Binary içinde gömülü

fn adim_at() {
    let buffer: [u8; 32] = [0; 32];   // Stack'te, sıfır maliyet
    // ...
}
```

---

# 🎯 ÖZET: Bellek Yönetimi Kontrol Listesi

| Durum | Çözüm |
|---|---|
| Basit, küçük veri | Stack'te bırak (primitive, küçük struct) |
| Büyük, dinamik veri | `Box<T>` veya `Vec<T>` |
| Tek thread, paylaşılan okuma | `Rc<T>` |
| Multi-thread, paylaşılan okuma | `Arc<T>` |
| Değişmez referansla değiştirme | `RefCell<T>` (tek thread) veya `Mutex<T>` (multi-thread) |
| String fonksiyon parametresi | `&str` kullan (esneklik) |
| String sahipliği gerektiğinde | `String` kullan |
| Recursive tip | `Box<T>` ile sarmala |
| Global mutable state | `Mutex<T>` veya `AtomicT` |
| Embedded sistem | `heapless` crate, static allocation |

---

# 🚀 Son Tavsiyeler

1. **Önce Derleyiciyi Dinleyin:** Rust'ın hata mesajları size ne yapmanız gerektiğini söyler. `%90`'ını okuyarak çözebilirsiniz.
2. **Mümkün Olduğunca Ödünç Alın:** `clone()` son çaredir. Referans kullanmak her zaman daha ucuzdur.
3. **Lifetime'ları Anlamaya Çalışın:** `'a` gördüğünüzde "bu referans ne kadar yaşamalı?" diye sorun.
4. **Smart Pointer'ları Doğru Seçin:** `Rc` mi `Arc` mı? Tek thread mi multi-thread mi?
5. **Embedded Projelerde Heap'ten Kaçının:** RP2354B projenizde `heapless` crate kullanmak, bellek sızıntısı riskini sıfıra indirir.
6. **`cargo clippy` Kullanın:** Bellek yönetiminde yaygın hataları yakalar.

> 🦀 **Unutmayın:** Rust'ın öğrenme eğrisi diktir ama bir kez "derleyici ile düşünmeyi" öğrendiğinizde, C# dünyasında asla yakalayamayacağınız bir **güven ve performans** seviyesine ulaşırsınız. Step motor projenizde milisaniyelik hassasiyet ve sıfır bellek sızıntısı, bu çabaya değecektir.