# 🦀 Ders Notları: Bölüm 10 - 📚 Rust'ta Generics, Traits ve Lifetimes — Kapsamlı Ders

Rust'ın 10. bölümü, dilin en güçlü soyutlama mekanizmalarını içerir. Bu bölümü bir sınıf ortamında ders anlatır gibi, adım adım, bol örneklerle ve "neden?" sorusunu sürekli sorarak işleyeceğiz. Hazırsanız başlıyoruz! 🚀

---

## 🎯 Bölüm 1: Generics (Genelleme) Nedir?

### 1.1 — Motivasyon: Kod Tekrarını Azaltmak

Diyelim ki bir listedeki en büyük sayıyı bulan bir program yazıyorsunuz:

```rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("En büyük sayı: {largest}");
}
```

Şimdi aynı kodu iki farklı listede kullanmanız gerekti. İki seçeneğiniz var:

1. **Kodu kopyala-yapıştır** (kötü alışkanlık 😞)
2. **Fonksiyona çıkar** (daha iyi 👍)

```rust
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest { largest = item; }
    }
    largest
}
```

Harika! Ama şimdi bir sorun çıktı: **Ya `char` listesindeki en büyük karakteri bulmak istersem?** Yeni bir fonksiyon mu yazmalıyım?

```rust
fn largest_i32(list: &[i32]) -> &i32 { /* aynı kod */ }
fn largest_char(list: &[char]) -> &char { /* aynı kod */ }
```

Gördüğünüz gibi **mantık aynı, sadece tipler farklı**. İşte tam burada **Generics** devreye girer! 🎉

---

### 1.2 — Generic Fonksiyonlar

Generic, **"somut bir tip yerine soyut bir tip placeholder'ı"** kullanmaktır. Tıpkı fonksiyon parametrelerinin değerler için placeholder olması gibi, generic parametreler de **tipler için placeholder**'dır.

Sözdizimi şöyle:

```rust
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest { largest = item; }
    }
    largest
}
```

🔍 **Sözdizimini parçalayalım:**

| Parça | Anlamı |
|-------|--------|
| `<T>` | "Bu fonksiyon T adında bir tip parametresi üzerinde genellenmiştir" |
| `&[T]` | "T tipindeki değerlerden oluşan bir slice" |
| `-> &T` | "T tipinde bir referans döndürür" |

> 💡 **Kural:** Rust'ta tip parametreleri geleneksel olarak **tek harf** ve **UpperCamelCase** olur. `T` (Type), `U`, `V`, `K` (Key), `V` (Value) gibi.

⚠️ **Ama bu kod henüz derlenmez!** Çünkü derleyici `T` tipinin `>` operatörünü destekleyip desteklemediğini bilmiyor. Bu sorunu **Traits** bölümünde çözeceğiz.

---

### 1.3 — Generic Struct'lar

Struct tanımlarında da aynı sözdizimi kullanılır:

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };       // Point<i32>
    let float = Point { x: 1.0, y: 4.0 };      // Point<f64>
}
```

🚨 **Önemli Uyarı:** Tek bir `T` kullandığınız için `x` ve `y` **aynı tip** olmak zorundadır!

```rust
let wont_work = Point { x: 5, y: 4.0 }; // ❌ HATA!
// x i32 olarak sabitlendi, y f64 olamaz
```

#### Birden Fazla Generic Tip Parametresi

Eğer `x` ve `y`'nin farklı tiplerde olmasını istiyorsanız:

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 }; // ✅ Hepsi geçerli!
}
```

> 📌 **Pratik Kural:** Çok fazla generic parametre kullanmak kodu okunmaz hale getirir. Eğer 3-4'ten fazla parametre kullanıyorsanız, kodunuzu yeniden yapılandırmanız gerekebilir.

---

### 1.4 — Generic Enum'lar

Aslında siz zaten generic enum kullanıyorsunuz! `Option<T>` ve `Result<T, E>` buna örnektir:

```rust
// Standart kütüphaneden
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Bu sayede `Option<i32>`, `Option<String>`, `Result<File, io::Error>` gibi sonsuz kombinasyon kullanabiliriz.

> 💡 **İpucu:** Kendi struct/enum tanımlarınızda, sadece tuttukları veri tipleri farklı olan birden fazla tanım görüyorsanız → **Generic kullanma zamanı gelmiştir!**

---

### 1.5 — Generic Method'lar

Method tanımlarında `impl` bloğundan sonra generic tip bildirilir:

```rust
struct Point<T> { x: T, y: T }

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

#### Sadece Belirli Bir Tip İçin Method Tanımlamak

Bazen bir methodu **sadece belirli bir somut tip** için tanımlamak isteriz:

```rust
impl Point<f32> {  // <T> yok! Sadece f32 için.
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

Bu method sadece `Point<f32>` için vardır. `Point<i32>`'de bu method **yoktur**.

#### Method İçinde Farklı Generic Parametreler

```rust
struct Point<X1, Y1> { x: X1, y: Y1 }

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point { x: self.x, y: other.y }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };       // Point<i32, f64>
    let p2 = Point { x: "Hello", y: 'c' };  // Point<&str, char>
    let p3 = p1.mixup(p2);                  // Point<i32, char>
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y); // 5, c
}
```

---

### 1.6 — Monomorphization: Generic'lerin Sıfır Maliyeti! 🎁

Çok güzel bir soru: **"Generic kullanmak runtime'da performansı düşürür mü?"**

**Cevap: HAYIR!** Rust, generic kodu derleme zamanında **somut koda dönüştürür**. Bu işleme **monomorphization** denir.

Örnek:

```rust
let integer = Some(5);       // Option<i32>
let float = Some(5.0);       // Option<f64>
```

Derleyici bunu şuna dönüştürür:

```rust
enum Option_i32 { Some(i32), None }
enum Option_f64 { Some(f64), None }

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

Yani **runtime'da hiçbir ekstra maliyet yok**. Elle yazılmış gibi çalışır! 🚀

---

## 🎯 Bölüm 2: Traits (Özellikler)

### 2.1 — Trait Nedir?

**Trait**, bir tipin sahip olması gereken **davranışları (method imzalarını)** tanımlayan bir sözleşmedir. Diğer dillerdeki **"interface"** kavramına benzer.

Örnek senaryo: Bir medya uygulaması yapıyoruz. Hem `NewsArticle` hem de `SocialPost` için özet göstermek istiyoruz.

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

> 📌 **Önemli:** Trait içinde method imzalarından sonra **süslü parantez değil, noktalı virgül** kullanılır.

---

### 2.2 — Trait Implementasyonu

Bir tipi trait ile implemente etmek için `impl TraitName for TypeName` sözdizimi kullanılır:

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

Artık her iki tip üzerinde de `summarize()` çağırabiliriz:

```rust
let post = SocialPost { /* ... */ };
println!("{}", post.summarize());
```

---

### 2.3 — Varsayılan Method Implementasyonları

Trait method'larına **varsayılan gövde** verebilirsiniz:

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Devamını oku...)")
    }
}

impl Summary for NewsArticle {}  // Varsayılanı kullanır
```

İsterseniz override edebilirsiniz:

```rust
impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)  // Override
    }
}
```

> 💡 **Güçlü Özellik:** Varsayılan method, trait içindeki diğer (varsayılan olmayan) method'ları çağırabilir:

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;  // Zorunlu
    
    fn summarize(&self) -> String {        // Varsayılan
        format!("(Devamını oku: {}...)", self.summarize_author())
    }
}
```

---

### 2.4 — Trait Bound: Generic Tipleri Kısıtlamak

Hatırlayın, `largest` fonksiyonunda `T` tipi `>` operatörünü desteklemiyordu. Trait'ler bunu çözer!

#### `impl Trait` Sözdizimi (Basit)

```rust
pub fn notify(item: &impl Summary) {
    println!("Son dakika! {}", item.summarize());
}
```

Bu fonksiyon, `Summary` trait'ini implement eden **herhangi bir tip** kabul eder.

#### Trait Bound Sözdizimi (Uzun Form)

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Son dakika! {}", item.summarize());
}
```

Bu iki sözdizimi **eşdeğerdir**, ama trait bound daha esnektir.

#### Birden Fazla Trait Bound

```rust
// Bir tip hem Summary hem de Display implement etmeli
pub fn notify(item: &(impl Summary + Display)) { /* ... */ }

// Veya generic formda:
pub fn notify<T: Summary + Display>(item: &T) { /* ... */ }
```

#### `where` Clause (Okunabilirlik İçin)

Çok fazla trait bound olduğunda:

```rust
// Önce:
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 { }

// Sonra (daha temiz):
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{ }
```

---

### 2.5 — `impl Trait` Return Type Olarak

Bir fonksiyonun döndürdüğü değerin belirli bir trait'i implement ettiğini belirtebilirsiniz:

```rust
fn returns_summarizable() -> impl Summary {
    SocialPost { /* ... */ }
}
```

> ⚠️ **Kısıtlama:** Bu sözdizimi **sadece tek bir tip** döndürüldüğünde çalışır. `if/else` ile farklı tipler döndüremezsiniz (bunun için **trait object** gerekir, Ch.18).

---

### 2.6 — Koşullu Method Implementasyonu

Bir methodu, **sadece belirli trait bound'ları karşılayan generic tipler için** implement edebilirsiniz:

```rust
use std::fmt::Display;

struct Pair<T> { x: T, y: T }

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self { Self { x, y } }
}

// Sadece T hem Display hem PartialOrd implement ediyorsa:
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("En büyük: x = {}", self.x);
        } else {
            println!("En büyük: y = {}", self.y);
        }
    }
}
```

---

### 2.7 — Blanket Implementation (Örtülü Implementasyon)

Bir trait'i, **başka bir trait'i implement eden TÜM tipler için** otomatik implement edebilirsiniz:

```rust
// Standart kütüphaneden gerçek bir örnek:
impl<T: Display> ToString for T {
    // ...
}
```

Bu sayede `Display` implement eden her tip otomatik olarak `to_string()` methoduna sahip olur:

```rust
let s = 3.to_string();  // i32 → Display implement eder → ToString da var!
```

---

## 🎯 Bölüm 3: Lifetimes (Ömürler)

### 3.1 — Lifetime Nedir?

**Lifetime**, bir referansın **geçerli olduğu kapsamı (scope)** ifade eder. Rust'ın en özgün ve en zorlayıcı özelliğidir.

> 💡 **Önemli:** Lifetime'lar referansların **ne kadar yaşadığını DEĞİŞTİRMEZ**. Sadece derleyiciye referanslar arasındaki **ilişkiyi açıklar**.

### 3.2 — Dangling Reference (Sarkan Referans) Problemi

Rust'ın lifetime sistemi, **dangling reference**'ları önlemek için vardır:

```rust
fn main() {
    let r;
    {
        let x = 5;
        r = &x;  // ❌ x, scope'un sonunda yok olacak!
    }
    println!("r: {r}");  // r artık geçersiz bir referans
}
```

Hata: `x does not live long enough` (x yeterince uzun yaşamıyor)

#### Borrow Checker

Derleyici, **borrow checker** ile referansların ömürlerini karşılaştırır:

```rust
let r;                // ---+-- 'a (uzun ömür)
{                     //    |
    let x = 5;        // -+-- 'b (kısa ömür)
    r = &x;           //  |
}                     // -+
println!("r: {r}");   //    |
```

`'b < 'a` olduğu için **reddedilir**. ✅ Doğru versiyon:

```rust
let x = 5;            // ----------+-- 'b
let r = &x;           // ---+-- 'a |
println!("r: {r}");   // ---+      |
                      // ----------+
```

Artık `'a ⊆ 'b`, yani referans, verinin ömrü içinde. ✅

---

### 3.3 — Generic Lifetime Annotation Sözdizimi

Lifetime parametreleri `'` (apostrof) ile başlar ve genellikle kısadır (`'a`, `'b`, `'aaa` gibi):

```rust
&i32          // Normal referans
&'a i32       // 'a lifetime'ına sahip referans
&'a mut i32   // 'a lifetime'ına sahip mutable referans
```

### 3.4 — Fonksiyonlarda Lifetime'lar

İki string slice'tan daha uzun olanı döndüren bir fonksiyon yazalım:

```rust
fn longest(x: &str, y: &str) -> &str {  // ❌ HATA!
    if x.len() > y.len() { x } else { y }
}
```

**Hata:** `missing lifetime specifier` — Derleyici, döndürülen referansın `x`'ten mi yoksa `y`'den mi geldiğini bilmiyor!

#### Çözüm: Lifetime Annotation

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

🔍 **Bu ne anlama geliyor?**

> "Öyle bir `'a` lifetime'ı var ki, hem `x` hem `y` en az `'a` kadar yaşıyor. Döndürülen referans da en az `'a` kadar yaşayacak."

Pratik olarak: **Döndürülen referans, `x` ve `y`'nin ömürlerinin KESİŞİMİ kadar yaşar.**

#### Kullanım Örnekleri

```rust
// ✅ Geçerli: result, string2'nin scope'u içinde kullanılıyor
fn main() {
    let string1 = String::from("uzun bir cümle");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("En uzun: {result}");
    }
}

// ❌ Geçersiz: result, string2 scope dışına çıktıktan sonra kullanılıyor
fn main() {
    let string1 = String::from("uzun bir cümle");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }  // string2 burada öldü
    println!("En uzun: {result}");  // ❌ HATA!
}
```

> 🧠 **İnsan olarak** biz `string1`'in daha uzun olduğunu görüyoruz, ama **derleyici göremez**. Biz ona lifetime annotation ile "döndürdüğüm referans, parametrelerin küçük olan ömrü kadar yaşar" dedik.

---

### 3.5 — Fonksiyonlarda Lifetime Düşünme Rehberi

Bir fonksiyonda lifetime annotation yazarken şu kuralları düşünün:

1. **Döndürülen referans, parametrelerden birine mi ait?** → O parametrenin lifetime'ını kullan.
2. **Döndürülen referans, fonksiyon içinde oluşturulan bir veriye mi ait?** → ❌ Bu **dangling reference** olur, derlenmez!

```rust
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("çok uzun");
    result.as_str()  // ❌ HATA! result fonksiyon sonunda yok olur
}
```

**Çözüm:** Referans yerine **sahip olunan (owned)** tip döndürün:

```rust
fn longest(x: &str, y: &str) -> String {
    if x.len() > y.len() { x.to_string() } else { y.to_string() }
}
```

---

### 3.6 — Struct'larda Lifetime'lar

Bir struct referans tutuyorsa, **lifetime annotation zorunludur**:

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt { part: first_sentence };
    // 'i', 'novel'den uzun yaşayamaz!
}
```

> 📌 **Anlamı:** `ImportantExcerpt` örneği, içindeki referanstan daha uzun yaşayamaz.

---

### 3.7 — Lifetime Elision (Otomatik Çıkarım) Kuralları

Bazı durumlarda lifetime annotation yazmak **gerekmez**. Derleyici 3 kural uygular:

#### Kural 1: Her referans parametreye bir lifetime ver
```rust
fn foo(x: &i32)        →  fn foo<'a>(x: &'a i32)
fn foo(x: &i32, y: &i32)  →  fn foo<'a, 'b>(x: &'a i32, y: &'b i32)
```

#### Kural 2: Tek input lifetime varsa, onu output'a ata
```rust
fn foo(x: &i32) -> &i32   →   fn foo<'a>(x: &'a i32) -> &'a i32
```

#### Kural 3: `&self` veya `&mut self` varsa, self'in lifetime'ını output'a ata
```rust
impl Foo {
    fn method(&self, x: &i32) -> &i32 { ... }
    // self'in lifetime'ı output'a atanır
}
```

#### Örnek: Bu fonksiyon neden lifetime annotation olmadan derlenir?

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { return &s[0..i]; }
    }
    &s[..]
}
```

**Derleyicinin düşünce süreci:**
1. Kural 1: `s: &'a str`
2. Kural 2: Tek input var → output da `'a` olur: `-> &'a str`
3. ✅ Tamamlandı!

#### Ama bu derlenmez:

```rust
fn longest(x: &str, y: &str) -> &str { ... }
```

1. Kural 1: `x: &'a str, y: &'b str`
2. Kural 2: İki input var → uygulanmaz
3. Kural 3: `self` yok → uygulanmaz
4. ❌ Output'un lifetime'ı belirlenemedi!

---

### 3.8 — Method'larda Lifetime'lar

```rust
struct ImportantExcerpt<'a> { part: &'a str }

impl<'a> ImportantExcerpt<'a> {
    // Kural 3 sayesinde lifetime annotation gerekmez
    fn level(&self) -> i32 { 3 }
    
    // Kural 3: return, self'in lifetime'ını alır
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Dikkat: {announcement}");
        self.part
    }
}
```

---

### 3.9 — `'static` Lifetime

`'static`, programın **tamamı boyunca** geçerli olan özel bir lifetime'dır:

```rust
let s: &'static str = "Ben statik bir ömre sahibim.";
```

String literal'ler doğrudan binary'ye gömüldüğü için her zaman `'static`'tir.

> ⚠️ **Uyarı:** Hata mesajlarında `'static` önerisi gördüğünüzde hemen `'static` eklemeyin! Genellikle asıl sorun dangling reference'dur.

---

### 3.10 — Her Şeyi Bir Arada: Generic + Trait + Lifetime

Gerçek dünya kodunda bu üçü birlikte kullanılır:

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Duyuru! {ann}");
    if x.len() > y.len() { x } else { y }
}
```

🔍 **Çözümleme:**
- `<'a, T>` → Hem lifetime hem generic tip parametresi
- `&'a str` → Lifetime annotation'lı referans
- `T` → Herhangi bir tip olabilir
- `where T: Display` → `T`, `Display` trait'ini implement etmeli

---

## 🎓 Özet Tablosu

| Kavram | Ne Yapar? | Sözdizimi |
|--------|-----------|-----------|
| **Generic Type** | Farklı tiplerle çalışan kod | `<T>`, `<T, U>` |
| **Trait** | Paylaşılan davranış tanımlar | `trait Summary { ... }` |
| **Trait Bound** | Generic tipi kısıtlar | `T: Summary`, `impl Summary` |
| **Lifetime** | Referansların geçerliliğini garanti eder | `<'a>`, `&'a T` |
| **Monomorphization** | Generic'leri sıfır maliyetle somutlaştırır | Derleyici otomatik yapar |

---

## 🧠 Akılda Tutulması Gerekenler

1. ✅ **Generic'ler sıfır runtime maliyetine sahiptir** (monomorphization sayesinde).
2. ✅ **Trait'ler, Rust'ın "interface" mekanizmasıdır**; davranış paylaşımı sağlar.
3. ✅ **Lifetime'lar referansların ömrünü DEĞİŞTİRMEZ**, sadece derleyiciye ilişkiyi açıklar.
4. ✅ **Elision kuralları** çoğu lifetime annotation'ı otomatik yapar.
5. ✅ **`'static`** her zaman çözüm değildir; genellikle asıl bug'ı maskeler.
6. ✅ **Trait bound olmadan generic tip üzerinde method çağrılamaz** (ör. `>` operatörü).

---

## 🚀 Sıradaki Adım

Bu bölümü anladıysanız:
- **Ch.17**: Trait object'ler ile **runtime polymorphism** (dynamic dispatch)
- **Ch.18**: Closure ve iterator'ler (generic'lerin pratik kullanımı)
- **Ch.19**: Gelişmiş trait özellikleri (associated types, default generic parameters)

Sorularınız varsa sormaktan çekinmeyin! Her kavramı daha fazla örnekle pekiştirebiliriz. 🎯