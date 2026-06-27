# 🦀 Rust'ta Dönüşüm (Conversion) — Kapsamlı Ders

Rust'ta tip dönüşümleri, dilin güvenlik felsefesinin en net görüldüğü alanlardan biridir. Rust, **örtük (implicit) tip dönüşümüne izin vermez**; yani `i32`'yi otomatik olarak `u8` yapmaz, sizden bunu açıkça istemenizi bekler. Bu, "sessizce veri kaybı olmasını önlemek" için alınmış bilinçli bir tasarım kararıdır.

Bu derste Rust By Example'ın Conversion bölümünü ve tüm alt başlıklarını adım adım, bol örnekli bir şekilde inceleyeceğiz.

---

## 📚 Bölüm Haritası

1. [`as` ile Casting](#1-as-ile-casting-tip-dönüşümü) — İlkel tipler arası dönüşüm
2. [`From` ve `Into`](#2-from-ve-into-traitleri) — Güvenli, hata vermeyen dönüşümler
3. [`TryFrom` ve `TryInto`](#3-tryfrom-ve-tryinto-traitleri) — Hata verebilen (fallible) dönüşümler
4. [`ToString` ve `FromStr`](#4-tostring-ve-fromstr-string-dönüşümleri) — String ile ilgili dönüşümler

---

## 1. `as` ile Casting (Tip Dönüşümü)

Rust'ta ilkel (primitive) tipler arasında dönüşüm yapmak için `as` anahtar kelimesi kullanılır. Bu, C/C++'taki `(int)x` cast'ine benzer ama Rust'ta davranış **tamamen tanımlıdır** — C'deki "undefined behavior" diye bir şey yoktur.

### Temel Kullanım

```rust
fn main() {
    let decimal = 65.4321_f32;

    // ❌ HATA! Rust örtük dönüşüm yapmaz
    // let integer: u8 = decimal;

    // ✅ Açık (explicit) dönüşüm
    let integer = decimal as u8;       // 65
    let character = integer as char;   // 'A' (ASCII 65)

    println!("{} -> {} -> {}", decimal, integer, character);
    // Çıktı: 65.4321 -> 65 -> A
}
```

> ⚠️ **Önemli:** Float'ı doğrudan `char`'a dönüştüremezsiniz. Önce tam sayıya, sonra char'a.

### Büyük Tipten Küçük Tipe Dönüşüm (Truncation)

Küçük bir tipe (örneğin `u16` → `u8`) dönüştürme yapıldığında, Rust **en anlamlı bitleri (MSB) atar**, sadece en az anlamlı bitleri (LSB) tutar. Bu, modüler aritmetiğe benzer:

```rust
fn main() {
    // 1000, u16'ya sığar
    println!("1000 as u16: {}", 1000 as u16);  // 1000

    // 1000, u8'e sığmaz! 1000 mod 256 = 232
    // 1000 = 0b0000_0011_1110_1000
    // u8'e alınca sadece son 8 bit kalır: 0b1110_1000 = 232
    println!("1000 as u8: {}", 1000 as u8);    // 232

    // -1 (i8) → u8: Two's complement
    println!("-1 as u8: {}", (-1i8) as u8);    // 255
}
```

### İşaretli Tiplere Dönüşüm (Two's Complement)

İşaretli (signed) tipe dönüştürme yapıldığında, önce karşılık gelen işaretsiz tipe dönüştürülmüş gibi davranılır, sonra en soldaki bit (MSB) 1 ise negatif sayı olarak yorumlanır:

```rust
fn main() {
    println!("128 as i16: {}", 128 as i16);   // 128 (sığar)
    println!("128 as i8: {}", 128 as i8);     // -128 (8-bit two's complement)

    // 1000 as u8 = 232, ve 232 i8'de -24'e karşılık gelir
    println!("1000 as u8: {}", 1000 as u8);   // 232
    println!("232 as i8: {}", 232 as i8);     // -24
}
```

### Float → Int Dönüşümünde Saturating Cast (Rust 1.45+)

Rust 1.45'ten itibaren, float'tan tam sayıya dönüşümde **saturating cast** uygulanır: değer hedef tipin sınırlarını aşıyorsa, en yakın sınır değer kullanılır.

```rust
fn main() {
    println!("300.0 as u8: {}", 300.0_f32 as u8);   // 255 (u8::MAX)
    println!("-100.0 as u8: {}", -100.0_f32 as u8); // 0   (u8::MIN)
    println!("NaN as u8: {}", f32::NAN as u8);      // 0
}
```

### `unsafe` ile Kontrolsüz Dönüşüm

Eğer performans kritikse ve sınırların aşılmayacağından eminseniz, `unsafe` metodlar kullanılabilir. Ancak bu **soundness** garantisi bozar:

```rust
unsafe {
    // Saturating cast YAPMAZ, bit pattern'ı olduğu gibi yorumlar
    println!("300.0 as u8: {}", 300.0_f32.to_int_unchecked::<u8>()); // 44 (!)
}
```

> 🎓 **Özet:** `as` ucuz, hızlı ama "sessizce" veri kaybı yapabilir. Bu yüzden Rust topluluğu `as` yerine mümkün olduğunda `From`/`Into` trait'lerini tercih eder.

---

## 2. `From` ve `Into` Trait'leri

`From` ve `Into`, Rust'ın **kendi tipleriniz arasında** dönüşüm tanımlamanızı sağlayan, birbiriyle bağlantılı iki trait'idir.

### Felsefe

- **`From<T>`**: "Ben, T tipinden oluşturulabilirim" der. Kaynak tipi hedef tipe dönüştürür.
- **`Into<T>`**: "Ben, T tipine dönüşebilirim" der. Hedef tipe dönüşüm yapar.

İkisi birbirinin **tam tersidir** ve standart kütüphanede şu şekilde tanımlıdır:

```rust
// std kütüphanesindeki blanket implementation
impl<T, U> Into<U> for T where U: From<T> {
    fn into(self) -> U {
        U::from(self)
    }
}
```

Yani **`From` implement ederseniz, `Into` otomatik olarak gelir**. Tersi geçerli değildir!

### `From` Kullanımı

```rust
use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

// Number'ın i32'den nasıl oluşturulacağını tanımlıyoruz
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number: {:?}", num);  // My number: Number { value: 30 }
}
```

Standart kütüphanede de çok sayıda `From` implementasyonu vardır:

```rust
let my_str = "hello";
let my_string = String::from(my_str);  // &str -> String
```

### `Into` Kullanımı

`Into` kullanırken genellikle **hedef tipi belirtmeniz gerekir**, çünkü derleyici nereye dönüştürmek istediğinizi bilemez:

```rust
use std::convert::Into;

#[derive(Debug)]
struct Number {
    value: i32,
}

// ❌ Bu şekilde implement etmek ÖNERİLMEZ!
// Sadece From implement etmeniz yeterli.
impl Into<Number> for i32 {
    fn into(self) -> Number {
        Number { value: self }
    }
}

fn main() {
    let int = 5;
    // Tip açıklaması (type annotation) GEREKLİ
    let num: Number = int.into();
    println!("My number: {:?}", num);
}
```

### Doğru Yaklaşım: Sadece `From` Implement Et

```rust
use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let int = 5;
    // From implement ettiğimiz için Into otomatik geldi!
    let num: Number = int.into();
    println!("My number: {:?}", num);
}
```

> 🎓 **Altın Kural:** Her zaman sadece `From` implement edin, `Into`'ya dokunmayın. `Into` bedavaya gelir.

### Fonksiyon Parametrelerinde `Into` Kullanımı

`Into`'nun en güçlü kullanım alanı, fonksiyonlara esnek parametreler kabul etmektir:

```rust
fn print_length(s: impl Into<String>) {
    let s = s.into();
    println!("Uzunluk: {}", s.len());
}

fn main() {
    print_length("merhaba");           // &str kabul eder
    print_length(String::from("dünya")); // String kabul eder
}
```

---

## 3. `TryFrom` ve `TryInto` Trait'leri

Bazı dönüşümler **her zaman başarılı olmayabilir**. Örneğin bir `i32`'yi `u8`'e dönüştürmeye çalışırsanız, değer 0-255 arasında değilse başarısız olur. İşte bu durumlar için `TryFrom` ve `TryInto` vardır.

### Felsefe

- `From`/`Into` → **Infallible** (asla hata vermez)
- `TryFrom`/`TryInto` → **Fallible** (hata verebilir, `Result` döner)

İmzaları şöyledir:

```rust
trait TryFrom<T>: Sized {
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}

trait TryInto<T>: Sized {
    type Error;
    fn try_into(self) -> Result<T, Self::Error>;
}
```

### Örnek: Sadece Çift Sayıları Kabul Eden Bir Tip

```rust
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();  // Hata tipi olarak birim () kullanıyoruz

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    // TryFrom kullanımı
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto kullanımı (tip açıklaması gerekli)
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));

    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}
```

### Gerçekçi Bir Örnek: Yaş Doğrulama

```rust
#[derive(Debug)]
struct Person {
    age: u8,
}

#[derive(Debug)]
enum AgeError {
    Negative,
    TooOld,
}

impl TryFrom<i32> for Person {
    type Error = AgeError;

    fn try_from(age: i32) -> Result<Self, Self::Error> {
        if age < 0 {
            Err(AgeError::Negative)
        } else if age > 150 {
            Err(AgeError::TooOld)
        } else {
            Ok(Person { age: age as u8 })
        }
    }
}

fn main() {
    match Person::try_from(25) {
        Ok(p) => println!("Kişi: {:?}", p),
        Err(e) => println!("Hata: {:?}", e),
    }

    match Person::try_from(-5) {
        Ok(p) => println!("Kişi: {:?}", p),
        Err(e) => println!("Hata: {:?}", e),  // Hata: Negative
    }
}
```

> 🎓 **Ne Zaman Hangisi?**
> - Dönüşüm **her zaman** başarılı olabiliyorsa → `From` / `Into`
> - Dönüşüm **başarısız olabiliyorsa** → `TryFrom` / `TryInto`

---

## 4. `ToString` ve `FromStr` — String Dönüşümleri

String dönüşümleri günlük Rust programlamada en sık karşılaşılan konulardandır. İki yönü vardır:
- **Tipe → String** dönüştürmek (`ToString` trait'i)
- **String → Tip** dönüştürmek (`FromStr` trait'i)

### Herhangi Bir Tipi `String`'e Dönüştürmek

Bir tipi `String`'e dönüştürmek için doğrudan `ToString` trait'ini implement etmek yerine, **`fmt::Display`** trait'ini implement etmelisiniz. Çünkü `Display` implement ettiğinizde `ToString` **otomatik olarak** sağlanır (blanket impl ile).

```rust
use std::fmt;

struct Circle {
    radius: i32,
}

// Display implement ediyoruz — ToString otomatik gelir
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
    // Çıktı: Circle of radius 6
}
```

> 🎓 **Neden direkt ToString değil?** Çünkü `Display` implement ettiğinizde hem `to_string()` hem de `println!("{}", x)` gibi formatlama özelliklerini bedavaya alırsınız.

### String'den Bir Tipe Dönüşüm: `parse()`

String'den başka bir tipe (özellikle sayılara) dönüşüm için `parse()` metodu kullanılır. Bu metod `FromStr` trait'ini kullanır.

```rust
fn main() {
    // Yöntem 1: Tip çıkarımı (type inference)
    let parsed: i32 = "5".parse().unwrap();

    // Yöntem 2: Turbofish syntax (::<> ile tip belirtme)
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Toplam: {:?}", sum);  // Toplam: 15
}
```

### Kendi Tipiniz İçin `FromStr` Implement Etmek

Kendi bir tipinizi string'den parse edilebilir yapmak için `FromStr` trait'ini implement edin:

```rust
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Circle {
    radius: i32,
}

impl FromStr for Circle {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().parse() {
            Ok(num) => Ok(Circle { radius: num }),
            Err(e) => Err(e),
        }
    }
}

fn main() {
    let circle: Circle = " 3 ".parse().unwrap();
    println!("{:?}", circle);  // Circle { radius: 3 }
}
```

### Hata Yönetimi ile Parse

`unwrap()` kullanmak yerine `match` veya `?` operatörüyle hataları zarif şekilde yönetebilirsiniz:

```rust
fn main() {
    let input = "abc";

    match input.parse::<i32>() {
        Ok(n) => println!("Sayı: {}", n),
        Err(e) => println!("Parse hatası: {}", e),
        // Çıktı: Parse hatası: invalid digit found in string
    }
}
```

---

## 🗺️ Hangi Durumda Hangi Yöntem? — Özet Tablo

| Durum | Kullanılacak Trait/Anahtar | Dönen Değer |
|-------|---------------------------|-------------|
| İlkel tipler arası dönüşüm (i32 → u8, f32 → i32) | `as` | Hedef tip (veri kaybı olabilir) |
| Her zaman başarılı olan özel tip dönüşümü | `From` / `Into` | Hedef tip |
| Başarısız olabilen özel tip dönüşümü | `TryFrom` / `TryInto` | `Result<Hedef, Hata>` |
| Herhangi bir tipi `String` yapma | `Display` (→ `ToString`) | `String` |
| `String`'den bir tipe dönme | `FromStr` (→ `parse()`) | `Result<Tip, ParseError>` |

---

## 🎯 Pratik İpuçları

1. **`as` yerine `From` tercih edin.** `as` sessizce veri kaybı yapabilir; `From` ise ya başarılı olur ya da hiç compile olmaz.
2. **Sadece `From` implement edin**, `Into` otomatik gelir.
3. **Fonksiyon parametrelerinde `impl Into<String>`** kullanarak hem `&str` hem `String` kabul edin.
4. **`parse()` dönüşlerini asla `unwrap()`'lamayın** — production kodunda `match` veya `?` kullanın.
5. **Dönüşümün başarısız olma ihtimali varsa** mutlaka `TryFrom`/`TryInto` kullanın.

---

## 🧠 Zihinsel Model

Rust'ın dönüşüm sistemini şöyle hayal edebilirsiniz:

```
┌─────────────────────────────────────────────────────────┐
│                   TİP DÖNÜŞÜMLERİ                        │
├─────────────────────────────────────────────────────────┤
│                                                          │
│   İlkel Tipler ──────► as (casting)                      │
│                         • Hızlı ama tehlikeli            │
│                         • Veri kaybı olabilir            │
│                                                          │
│   Özel Tipler ───────► From / Into                       │
│                         • Güvenli, hata vermez           │
│                         • Sadece From implement et       │
│                                                          │
│   Özel Tipler ───────► TryFrom / TryInto                 │
│   (riskli dönüşüm)    • Result döner                     │
│                         • Başarısız olabilir             │
│                                                          │
│   String ────────────► Display / ToString                │
│   (gidiş)              • to_string()                     │
│                                                          │
│   String ────────────► FromStr / parse()                 │
│   (dönüş)              • Result döner                    │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

Bu yapı, Rust'ın "doğru şeyi yapmayı kolay, yanlış şeyi yapmayı zor" felsefesinin mükemmel bir yansımasıdır. Dönüşümlerinizde bu trait'leri doğru seçmek, hem güvenli hem de okunaklı kod yazmanın anahtarıdır. 🦀

# DeepSeek Yorumu -------------------

Bugünkü dersimizde Rust programlama dilinin en temel taşlarından biri olan **tür dönüşümlerini** (type conversions) ele alacağız. Rust By Example kitabının "Conversion" sayfası ve alt başlıklarındaki konuları, adım adım, bol örnekle ve ders anlatır gibi detaylıca inceleyeceğiz. Hazırsanız başlayalım.

---

## 1. Tür Dönüşümüne Neden İhtiyaç Duyarız?

Gerçek hayatta bir sayıyı metne çevirmek (`42` → `"42"`), bir tamsayıyı kayan noktalı sayıya dönüştürmek (`5` → `5.0`) ya da özel bir veri yapısını başka bir türe dönüştürmek sıkça yaptığımız işlemlerdir. Rust, tür güvenliğine çok önem verdiği için bu dönüşümlerin açıkça belirtilmesini ister. "Implicit conversion" (örtük dönüşüm) neredeyse yoktur; bunun yerine dönüşümler **trait'ler** (özellikler) aracılığıyla standart bir yapıya oturtulmuştur.

Rust'ta başlıca dönüşüm mekanizmaları şunlardır:
- `From<T>` / `Into<T>` – *her zaman başarılı* olan dönüşümler
- `TryFrom<T>` / `TryInto<T>` – *başarısız olma ihtimali* bulunan dönüşümler
- String dönüşümleri için `ToString` ve `FromStr`

Bu trait'leri tek tek öğrenelim.

---

## 2. `From` ve `Into`

### `From<T>` Trait’i
`From`, bir türün başka bir türden nasıl oluşturulacağını tanımlar. Eğer `A` türü için `From<B>` implemente edilmişse, `B` değerinden `A` değerini "sorunsuzca" üretebilirsiniz.

Tanımı:
```rust
pub trait From<T> {
    fn from(value: T) -> Self;
}
```

Kullanım senaryosu: Dönüşümün her koşulda başarılı olacağına eminsek `From` kullanırız. Örneğin `i32`'den `i64`'e dönüşüm kayıpsızdır, her zaman mümkündür.

```rust
let num_i32: i32 = 10;
let num_i64: i64 = i64::from(num_i32); // From<i32> for i64
// veya
let num_i64 = <i64 as From<i32>>::from(num_i32);
```

Standart kütüphanede birçok tür için `From` implementasyonları hazırdır. Kendi türlerimiz için de `From` implemente edebiliriz.

```rust
#[derive(Debug)]
struct Celsius(f64);

#[derive(Debug)]
struct Fahrenheit(f64);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 1.8 + 32.0)
    }
}

fn main() {
    let sicaklik_c = Celsius(100.0);
    let sicaklik_f = Fahrenheit::from(sicaklik_c);
    println!("{:?}", sicaklik_f);
}
```

### `Into<T>` Trait’i
`Into`, `From`'un tersidir. Eğer `From<A> for B` varsa, `A` için otomatik olarak `Into<B>` implemente edilir (blanket implementation). Yani `From`'u implemente ettiğinizde `Into`'yu bedavaya alırsınız.

```rust
let num_i32: i32 = 10;
let num_i64: i64 = num_i32.into(); // Into<i64> for i32 çalışır
```

`Into` genellikle fonksiyon parametrelerinde esneklik sağlamak için kullanılır:

```rust
fn hesapla(sayi: impl Into<i64>) -> i64 {
    let s: i64 = sayi.into();
    s * 2
}

fn main() {
    let x: i32 = 10;
    println!("{}", hesapla(x));     // i32 -> i64 dönüşümü otomatik
    println!("{}", hesapla(5i64));  // zaten i64
}
```

**Önemli not:** `From` ve `Into` dönüşümleri kayıplı olabilir mi? Standart kütüphane, kayıplı olabilecek `i64`'ten `i32`'ye `From`'u bilerek **implemente etmez**. Çünkü `From` başarısızlık beklemez. Kayıplı veya hata üretebilecek dönüşümler için bir sonraki başlığa geçiyoruz.

---

## 3. `TryFrom` ve `TryInto`

Bu ikili, dönüşüm sırasında hata oluşabilecek durumlar içindir. Dönüşüm sonucu `Result` tipinde olur.

### `TryFrom<T>` Trait’i
```rust
pub trait TryFrom<T> {
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}
```

Örnek: `i16`'yı `i8`'e çevirmek. Değer `i8` aralığına sığmazsa hata döner.

```rust
use std::convert::TryFrom;

fn main() {
    let buyuk: i16 = 300;
    let kucuk = i8::try_from(buyuk);
    match kucuk {
        Ok(deger) => println!("Dönüşüm başarılı: {}", deger),
        Err(e) => println!("Hata: {:?}", e),
    }
}
```

Çıktı: `Hata: TryFromIntError(())` (çünkü 300, `i8`'e sığmaz).

### `TryInto<T>` Trait’i
`TryFrom`'un tersidir ve yine blanket implementasyon sayesinde otomatik gelir.

```rust
let buyuk: i16 = 300;
let sonuc: Result<i8, _> = buyuk.try_into();
```

Fonksiyon parametrelerinde `TryInto` kullanarak hata yönetimini çağıran tarafa bırakabiliriz:

```rust
fn daralt<T: TryInto<i8>>(deger: T) -> Result<i8, T::Error> {
    deger.try_into()
}

fn main() {
    println!("{:?}", daralt(100i16));  // Ok(100)
    println!("{:?}", daralt(500i16));  // Err(...)
}
```

Kendi türlerimiz için `TryFrom` implementasyonu:

```rust
use std::convert::TryFrom;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
struct CiftSayi(i32);

impl TryFrom<i32> for CiftSayi {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(CiftSayi(value))
        } else {
            Err("Sayı çift değil!")
        }
    }
}

fn main() {
    let a: Result<CiftSayi, _> = CiftSayi::try_from(4);
    let b: Result<CiftSayi, _> = 5.try_into();
    println!("{:?}", a); // Ok(CiftSayi(4))
    println!("{:?}", b); // Err("Sayı çift değil!")
}
```

**Unutmayın:** `TryFrom`/`TryInto` ile `From`/`Into` arasındaki fark, dönüşümün başarısız olma ihtimalidir. `From` her zaman başarılıdır (infallible), `TryFrom` ise başarısızlık durumunda `Result::Err` döner.

---

## 4. String Dönüşümleri (`ToString` ve `FromStr`)

Metin işlemleri her programın olmazsa olmazıdır. Rust'ta bir değeri string yapmak ve string'i başka bir türe çevirmek için iki temel trait vardır.

### `ToString` Trait’i
Bir türü `String`'e dönüştürmek için kullanılır. Doğrudan `ToString` implemente etmek yerine, `Display` trait'ini implemente etmeniz yeterlidir; çünkü `Display` implemente eden her tür için `ToString` otomatik olarak implemente edilir.

```rust
use std::fmt;

struct Nokta {
    x: i32,
    y: i32,
}

impl fmt::Display for Nokta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let nokta = Nokta { x: 5, y: 10 };
    let metin = nokta.to_string();
    println!("Nokta: {}", metin);
}
```

`to_string()` metodu `ToString` trait'inden gelir ve yeni bir `String` oluşturur. `Display` implementasyonu sayesinde `println!("{}", nokta)` da çalışır.

### `FromStr` Trait’i
Bir `&str`'i başka bir türe dönüştürmek (parsing) için kullanılır. Standart kütüphanedeki `parse` metodu bu trait'i kullanır.

```rust
pub trait FromStr {
    type Err;
    fn from_str(s: &str) -> Result<Self, Self::Err>;
}
```

En sık kullanıldığı yer:

```rust
fn main() {
    let sayi_str = "42";
    let sayi: i32 = sayi_str.parse().unwrap(); // FromStr implementasyonu sayesinde
    println!("sayi + 1 = {}", sayi + 1);

    // Hata durumu:
    let hatali = "42a";
    let sonuc: Result<i32, _> = hatali.parse();
    match sonuc {
        Ok(s) => println!("Geçerli: {}", s),
        Err(e) => println!("Geçersiz sayı: {}", e),
    }
}
```

Kendi türümüz için `FromStr` implementasyonu:

```rust
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl FromStr for RGB {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // "#RRGGBB" formatını bekleyelim
        if s.len() != 7 || !s.starts_with('#') {
            return Err("Geçersiz format".to_string());
        }
        let r = u8::from_str_radix(&s[1..3], 16).map_err(|_| "Kırmızı hatalı")?;
        let g = u8::from_str_radix(&s[3..5], 16).map_err(|_| "Yeşil hatalı")?;
        let b = u8::from_str_radix(&s[5..7], 16).map_err(|_| "Mavi hatalı")?;
        Ok(RGB { r, g, b })
    }
}

fn main() {
    let renk: RGB = "#FF00AA".parse().unwrap();
    println!("{:?}", renk); // RGB { r: 255, g: 0, b: 170 }

    let hata = "#GGGGGG".parse::<RGB>();
    println!("{:?}", hata); // Err("Kırmızı hatalı")
}
```

`parse` metodunun tür çıkarımına dikkat edin; `let renk: RGB = ...` veya turbofish `::<RGB>` kullanmanız gerekebilir.

---

## 5. Tüm Dönüşüm Türlerine Toplu Bakış

| Trait | Kullanım Amacı | Dönüş Şekli | Başarısızlık Durumu |
|-------|----------------|-------------|---------------------|
| `From<T>` | A türünden B türüne kesin dönüşüm | `B::from(a)` | Yok (infallible) |
| `Into<T>` | `From`'un tersi, `a.into()` | `a.into()` | Yok |
| `TryFrom<T>` | Hata verebilen dönüşüm | `B::try_from(a)` → `Result<B, E>` | `Err` döner |
| `TryInto<T>` | `TryFrom`'un tersi, `a.try_into()` | `a.try_into()` → `Result<T, E>` | `Err` döner |
| `ToString` | Değeri `String` yap | `deger.to_string()` | (Display varsa) hatasız |
| `FromStr` | String'ten değer ayrıştırma | `metin.parse::<T>()` | `Result<T, Err>` |

---

## 6. Pratik İpuçları ve En İyi Yaklaşımlar

- **Kendi türünüz için `From` implemente edin, `Into` otomatik gelir.** Mümkünse `From`'u implemente etmek, hem kodunuzu daha okunur kılar hem de standart kütüphaneyle uyumlu olur.
- **Dönüşüm başarısız olabilirse `TryFrom` kullanın.** `From`'u kayıplı ya da hata üretebilecek durumlar için implemente etmek Rust'ın felsefesine aykırıdır ve beklenmedik hatalara yol açabilir.
- **`Display` ile `ToString` bedavaya gelir.** Yani sadece `Display` implemente ederek `{}` formatlamasını ve `to_string()` metodunu kazanırsınız.
- **Fonksiyon parametrelerinde `Into` veya `TryInto` kullanarak esnek API'ler tasarlayın.** Kullanıcılar farklı türlerle fonksiyonunuzu çağırabilir ve dönüşüm otomatik yapılır.
- **`parse` ile tür belirtmeyi unutmayın.** `"42".parse().unwrap()` yazarsanız Rust hangi türe dönüşeceğini bilemez; ya değişken tipini belirtin ya da turbofish `::<i32>()` kullanın.

---

## 7. Özet

Rust’ta tür dönüşümleri, derleme zamanında tür güvenliğini koruyacak şekilde tasarlanmıştır. Temel prensip:  
- **Her zaman başarılı dönüşüm** → `From`/`Into`  
- **Başarısız olabilen dönüşüm** → `TryFrom`/`TryInto`  
- **String üretimi** → `Display` + `ToString`  
- **String ayrıştırma** → `FromStr` + `parse()`

Bu trait'leri kullanarak kodunuzu hem okunabilir hem de güvenilir kılabilirsiniz. Kendi veri türlerinize bu trait'leri implemente ederek Rust ekosistemiyle kusursuz uyumlu, profesyonel kütüphaneler yazabilirsiniz.

Bugünkü dersimiz burada bitti. Bir sonraki derste, dönüşümlerin `as` anahtar kelimesiyle yapılan ilkel dönüşümlerle farklarını ve hangi durumlarda hangisini tercih etmeniz gerektiğini konuşacağız. Sorularınız varsa bekliyorum!