# Rust'ta Türler (Types) — Kapsamlı Ders

Rust By Example'ın "Types" bölümü, Rust'ın **tür sistemini** nasıl yönettiğimizi anlatan dört temel başlıktan oluşur. Bu dersimizde bu başlıkları tek tek, bol örneklerle ve "neden böyle?" sorularını yanıtlayarak inceleyeceğiz. Rust'ın tür sistemi, dilin en güçlü yanlarından biridir; çünkü hataları daha kod çalışmadan, derleme zamanında yakalar.

---

## 1. Casting — Tür Dönüşümleri (`as` Anahtar Kelimesi)

### Temel Kural: Rust'ta "Gizli" Dönüşüm Yoktur

C, C++ veya Java gibi dillere alışkınsanız şunu unutmanız gerekir: **Rust, ilkel türler (primitive types) arasında örtülü (implicit) tür dönüşümü yapmaz.** Yani bir `f32` değerini doğrudan bir `u8` değişkene atayamazsınız. Derleyici size hata verir. Bu, Rust'ın "güvenli kod" felsefesinin bir parçasıdır: *Ne yaptığınızı açıkça belirtmeniz gerekir.*

Ama açık (explicit) dönüşüm yapabilirsiniz. Bunun için **`as`** anahtar kelimesini kullanırız [[1]].

### Basit Bir Örnek

```rust
fn main() {
    let ondalik: f32 = 65.4321;
    
    // HATA! Örtülü dönüşüm yok:
    // let tamsayi: u8 = ondalik; 
    
    // Doğru yol: `as` ile açık dönüşüm
    let tamsayi = ondalik as u8;
    let karakter = tamsayi as char;
    
    println!("Dönüşüm: {} -> {} -> {}", ondalik, tamsayi, karakter);
    // Çıktı: Casting: 65.4321 -> 65 -> A
}
```

Burada `65.4321` önce `u8`'e dönüştürülürken **kesilir** (truncation) → `65`. Sonra `65` sayısının ASCII karşılığı olan `'A'` karakterine dönüşür.

### Dikkat: Doğrudan Float'tan Char'a Dönüşüm Yoktur

```rust
// HATA! Float doğrudan char'a dönüşemez.
let karakter = ondalik as char; 
```

Rust burada sizi korur. Önce tamsayıya, sonra karaktere dönüşüm *iki adımda* yapılmalıdır.

### Büyük Sayıları Küçük Tür Dönüştürmek

Peki `1000` gibi bir sayıyı `u8`'e (en fazla 255 alabilen 8-bit işaretsiz tamsayı) dönüştürürsek ne olur? Rust burada C'nin "tanımsız davranış" (undefined behavior) tuzağına düşmez; davranış **tamamen tanımlıdır** [[1]].

Kural şudur: **Hedef türün `MAX + 1` değeri, sığana kadar kaynaktan eklenir veya çıkarılır.** Pratikte bu, bitlerin kırpılması (truncation) anlamına gelir.

```rust
// 1000 zaten u16'ya sığar
println!("1000 as u16: {}", 1000 as u16);  // 1000

// 1000 = 0b0000_0011_1110_1000 (16 bit)
// u8'e dönüşürken sadece en düşük 8 bit (LSB) korunur:
// 0b1110_1000 = 232
println!("1000 as u8: {}", 1000 as u8);    // 232

// -1'i u8'e dönüştürmek:
// -1 + 256 = 255
println!("-1 as u8: {}", (-1i8) as u8);    // 255
```

Bu davranış, modüler aritmetiğe benzer:
```rust
println!("1000 mod 256: {}", 1000 % 256);  // 232 (aynı sonuç)
```

### İşaretli (Signed) Türlerde Durum

İşaretli bir türe dönüşüm yapılırken, önce ilgili işaretsiz türe dönüştüğünüz varsayılır. Sonra **en anlamlı bit (MSB)** 1 ise, değer negatif olarak yorumlanır (two's complement).

```rust
// 128, i16'ya rahatça sığar
println!("128 as i16: {}", 128 as i16);    // 128

// Ama 8-bit two's complement'te 128 = -128'dir!
println!("128 as i8: {}", 128 as i8);      // -128

// 232 (ki bu 1000 as u8'den geliyor), i8 olarak -24'tür
println!("232 as i8: {}", 232 as i8);      // -24
```

### Float → Int Dönüşümünde "Saturating Cast" (Rust 1.45+)

Rust 1.45'ten itibaren, `as` ile float'tan int'e dönüşüm yaparken **doyurma (saturating)** yapılır. Yani değer hedef türün sınırlarını aşıyorsa, **sınır değer** döndürülür [[1]].

```rust
// 300.0, u8'in maksimumu olan 255'i aşıyor → 255 döner
println!("300.0 as u8: {}", 300.0_f32 as u8);   // 255

// -100.0, u8'in minimumu olan 0'ın altında → 0 döner
println!("-100.0 as u8: {}", -100.0_f32 as u8); // 0

// NaN (Not a Number) → 0 döner
println!("NaN as u8: {}", f32::NAN as u8);      // 0
```

### `unsafe` ile "Unchecked" Dönüşüm

Eğer bu doyurma kontrolünün getirdiği küçük çalışma zamanı maliyetinden kaçınmak isterseniz, `unsafe` blok içinde `to_int_unchecked::<T>()` metodunu kullanabilirsiniz. Ancak dikkat: **taşma olursa "sound" (geçerli) olmayan değerler elde edersiniz.** Bu, Rust'ın güvenlik garantilerini bozar [[1]].

```rust
unsafe {
    // 300.0 → 44 (bitler olduğu gibi yorumlanır, taşma var!)
    println!("300.0 as u8 (unchecked): {}", 300.0_f32.to_int_unchecked::<u8>());
}
```

**Dersin Özeti:** Rust'ta tür dönüşümleri asla gizli olmaz. `as` anahtar kelimesi ile açıkça belirtirsiniz. Davranışlar tanımlıdır ama taşma, kırpılma ve doyurma kurallarını bilmeniz gerekir.

---

## 2. Literals — Değişmez Değerlerin Türlerini Belirlemek

### Sonek (Suffix) Kullanımı

Rust'ta sayısal değişmez değerlerin (literal) türünü, sonuna bir **sonek** ekleyerek belirtebilirsiniz [[2]].

```rust
let x = 1u8;     // x'in türü u8
let y = 2u32;    // y'nin türü u32
let z = 3f32;    // z'nin türü f32
let w = 4.0f64;  // w'nin türü f64
```

Sonekler: `i8`, `i16`, `i32`, `i64`, `i128`, `isize` (işaretli); `u8`, `u16`, `u32`, `u64`, `u128`, `usize` (işaretsiz); `f32`, `f64` (kayan nokta).

### Soneksiz Değerlerde Varsayılan Türler

Eğer sonek kullanmazsanız, derleyici **kullanım yerine göre** türü belirler. Hiçbir kısıt yoksa:
- **Tamsayılar için varsayılan: `i32`**
- **Kayan noktalılar için varsayılan: `f64`**

```rust
fn main() {
    // Sonekli — türleri baştan belli
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Soneksiz — türleri kullanıma göre belirlenir
    let i = 1;     // i32 oldu (varsayılan)
    let f = 1.0;   // f64 oldu (varsayılan)

    // Boyutlarını kontrol edelim:
    println!("x: {} byte", std::mem::size_of_val(&x)); // 1
    println!("y: {} byte", std::mem::size_of_val(&y)); // 4
    println!("z: {} byte", std::mem::size_of_val(&z)); // 4
    println!("i: {} byte", std::mem::size_of_val(&i)); // 4 (i32)
    println!("f: {} byte", std::mem::size_of_val(&f)); // 8 (f64)
}
```

### `std::mem::size_of_val` Hakkında Kısa Not

Bu fonksiyon, bir değişkenin **bayt cinsinden boyutunu** döndürür. `std::mem::size_of_val` şeklinde "tam yol" (full path) ile çağrılmıştır. Rust'ta kod, **modüller** halinde düzenlenir; `size_of_val` fonksiyonu `mem` modülünde, `mem` modülü de `std` **crate**'inde (paketinde) tanımlıdır [[2]].

### Neden Önemli?

Bu mekanizma, özellikle **sistem programlama** yaparken kritik. Örneğin bir donanım register'ına yazarken değerinizin tam olarak 8 bit mi yoksa 32 bit mi olması gerektiğini bilmek zorundasınız. Sonekler bunu açıkça belirtmenizi sağlar.

---

## 3. Type Inference — Tür Çıkarımı (Derleyicinin Zekâsı)

### Sadece İlk Değere Bakmaz, Kullanıma da Bakar

Rust'ın tür çıkarım motoru (type inference engine) oldukça akıllıdır. Bir değişkenin türünü belirlerken **sadece başlangıç değerine bakmaz; değişkenin daha sonra nasıl kullanıldığına da bakar** [[3]].

### Klasik Vec Örneği

```rust
fn main() {
    // elem'in u8 olduğunu derleyici biliyor
    let elem = 5u8;

    // Boş bir vektör oluşturuyoruz
    let mut vec = Vec::new();
    // Bu noktada derleyici vec'in tam türünü bilmiyor.
    // Sadece "bir şeylerin vektörü" olduğunu biliyor: Vec<_>

    // elem'i vektöre ekliyoruz
    vec.push(elem);
    // İşte şimdi derleyici anladı: vec, Vec<u8> olmalı!

    println!("{:?}", vec);  // [5]
}
```

### Büyünün Arkasında Ne Var?

1. `Vec::new()` çağrıldığında, derleyici vektörün içinde ne tutacağını bilmiyor. Bu yüzden geçici olarak `Vec<_>` (türü bilinmeyen vektör) olarak işaretliyor.
2. `vec.push(elem)` satırında, `push` metodunun imzası `fn push(&mut self, value: T)` şeklindedir. `elem`'in türü `u8` olduğuna göre, `T = u8` olmalı.
3. Derleyici bu bilgiyi geriye doğru yayarak `vec`'in türünü `Vec<u8>` olarak kesinleştirir.

Eğer `vec.push(elem)` satırını yorum yapsaydınız, derleyici `vec`'in türünü belirleyemezdi ve **hata verirdi**. Çünkü vektörün türünü çıkaracak hiçbir ipucu kalmazdı.

### Bu Neden Güzel?

Çünkü **hiçbir tür belirtimi (type annotation) yapmanıza gerek kalmadı**. Hem derleyici mutlu, hem de programcı! Kod temiz, okunabilir ve hâlâ tamamen tür-güvenli (type-safe).

---

## 4. Aliasing — Tür Takma Adları

### `type` Anahtar Kelimesi ile Yeni İsim

Rust'ta `type` ifadesi, **mevcut bir türe yeni bir isim** vermek için kullanılır [[4]]. Bu, tamamen aynı türe farklı bir isim takmaktan ibarettir; yeni bir tür oluşturmaz.

```rust
// NanoSecond, Inch ve U64, hepsi aslında u64'ün yeni isimleri
type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

fn main() {
    let nanoseconds: NanoSecond = 5 as u64;
    let inches: Inch = 2 as U64;

    // Tür eşlenikleri ekstra tür güvenliği SAĞLAMAZ!
    // Çünkü bunlar yeni türler değil, sadece takma ad.
    // nanoseconds + inches toplamayı mümkün kılar.
    println!("{} ns + {} inç = {} birim?",
             nanoseconds,
             inches,
             nanoseconds + inches);
    // Çıktı: 5 nanoseconds + 2 inches = 7 unit?
}
```

### Önemli Uyarı: Takma Adlar Tür Güvenliği Sağlamaz!

Yukarıdaki örnekte `NanoSecond` ve `Inch` farklı kavramlar olsa da, ikisi de `u64` olduğu için **birbirleriyle toplanabilirler**. Bu mantıksız! Eğer gerçek tür güvenliği istiyorsanız, "New Type Idiom" denilen bir desen kullanmalısınız (bu, `struct NanoSecond(u64);` gibi bir sarmalayıcı tür oluşturmayı gerektirir).

### İsimlendirme Kuralı

Tür takma adları **UpperCamelCase** (BüyükDeveKasası) şeklinde isimlendirilmelidir, aksi halde derleyici uyarı verir. İstisna: `usize`, `f32` gibi ilkel türlerin kendileri [[4]].

### Asıl Kullanım Amacı: Boilerplate'i Azaltmak

Takma adların asıl gücü, **karmaşık türleri basitleştirmektedir**. Standart kütüphaneden mükemmel bir örnek:

```rust
// std::io modülünde şöyle bir takma ad var:
type Result<T> = std::result::Result<T, std::io::Error>;
```

Dosya I/O işlemlerinde her seferinde `Result<T, io::Error>` yazmak yerine, sadece `io::Result<T>` yazabilirsiniz. Aynı türe işaret ederler ama yazımı çok daha kolaydır [[4]].

Kendi projelerinizde de karmaşık generic türleri basitleştirmek için kullanabilirsiniz:

```rust
use std::collections::HashMap;

// Uzun bir türü kısaltalım
type PlayerInventory = HashMap<String, Vec<Item>>;

fn show_inventory(inv: &PlayerInventory) {
    // ...
}
```

---

## Genel Özet ve Dersin Çıkarımları

| Konu | Anahtar Kelime / Mekanizma | Ne İşe Yarar? |
|------|----------------------------|---------------|
| **Casting** | `as` | İlkel türler arasında açık dönüşüm. Örtülü dönüşüm yoktur. |
| **Literals** | Sonekler (`42i32`, `3.14f64`) | Değişmez değerlerin türünü baştan belirtmek. Varsayılanlar: `i32` ve `f64`. |
| **Inference** | Derleyicinin kendisi | Değişkenin kullanım yerine bakarak türünü otomatik belirler. |
| **Aliasing** | `type` | Mevcut türe yeni bir isim vermek. Boilerplate'i azaltır. |

### Rust'ın Tür Felsefesi

Bu dört konuyu birleştiren ortak bir tema var: **Açıklık ve Güvenlik**.

- Rust, sizin ne yaptığınızı *bilerek* yapmanızı ister (bu yüzden `as` gerekir).
- Ama aynı zamanda sizi gereksiz yazımlardan kurtarır (tür çıkarımı sayesinde).
- Karmaşık türleri okunabilir kılar (takma adlar sayesinde).
- Sayıların boyutunu netleştirir (sonekler sayesinde).

Bu ders, Rust'ın temelini oluşturan "tür güvenliği" anlayışının küçük ama önemli bir parçası. İleride `struct`, `enum`, `trait` gibi özel türleri öğrendikçe, burada öğrendiğiniz ilkelerin aynı şekilde geçerli olduğunu göreceksiniz.

Bir sonraki adımda **Custom Types** (özel türler) bölümüne geçerek `struct` ve `enum` tanımlamayı öğrenebilirsiniz. Orada, bu dersteki "tür çıkarımı" ve "takma ad" bilgileriniz de işe yarayacak.

Başarılar! 🦀

# DeepSeek Yorumu --------------------------

Merhaba! Rust'ın tür sistemi, dilin en güçlü yanlarından biridir. Bu sistem sayesinde bellek güvenliği ve yüksek performans bir arada sunulur. Şimdi `Types` (Türler) ana başlığını ve `Casting`, `Literals`, `Inference`, `Aliasing` alt başlıklarını, tıpkı bir sınıfta anlatıyormuşum gibi, örneklerle ve kapsamlı bir şekilde ele alalım.

---

## 1. Giriş: Rust Tür Sisteminin Temelleri

Rust, **statik olarak tiplendirilmiş** bir dildir. Bu, her değişkenin, ifadenin ve fonksiyon dönüş değerinin türünün derleme zamanında bilinmesi gerektiği anlamına gelir. Peki her seferinde türü biz mi yazmak zorundayız? Hayır, devreye **tür çıkarımı (type inference)** girer ve çoğu durumda derleyici türü bizim için belirler. Ancak bazı durumlarda türü açıkça belirtmemiz veya dönüştürmemiz gerekir. İşte bu bölümde bunları öğreneceğiz.

---

## 2. Casting (Tür Dönüşümü)

Rust, türler arası dönüşümde örtülü (implicit) dönüşüme izin vermez. Bu, C/C++ gibi dillerde sıkça karşılaşılan ve hatalara yol açabilen otomatik tür genişletme/daraltma gibi sürprizlerin önüne geçer. Bunun yerine, açık (explicit) dönüşüm için `as` anahtar sözcüğünü kullanırız.

### Temel Kurallar:
- Sadece **sayısal türler** ve **işaretçi türleri** arasında `as` ile dönüşüm yapılabilir.
- `as` ifadesi **güvenli olmayabilir**; örneğin büyük bir tam sayıyı daha küçük bir türe dönüştürürken taşma (overflow) olursa Rust bunu sessizce keser (truncate). Bu davranış bilinçli bir tercihtir, çünkü C'deki gibi tanımsız davranış (undefined behavior) oluşturmaz, ancak yine de veri kaybına yol açabilir.
- Bellek adresleriyle ilgili ham işaretçi (raw pointer) dönüşümlerinde de `as` kullanılır.

### Örneklerle Açıklayalım:

```rust
fn main() {
    let ondalikli: f64 = 3.1415;
    // f64'ü i32'ye dönüştürelim. Ondalık kısım atılır (truncation), yuvarlama yapılmaz!
    let tamsayi = ondalikli as i32;
    println!("{} -> {}", ondalikli, tamsayi); // 3.1415 -> 3

    // Büyük bir u16 değerini u8'e dönüştürelim.
    let buyuk: u16 = 300;
    let kucuk = buyuk as u8;
    // 300, u8'in maksimum değeri olan 255'ten büyük. 
    // Rust bu durumda alt bitleri alır: 300 % 256 = 44
    println!("300 as u8 = {}", kucuk); // 44

    // İşaretli ve işaretsiz arasında dönüşüm:
    let negatif: i8 = -1;
    // -1'i u8'e çevirelim. Bellekteki bit temsili (ikiye tümleme) korunur ve işaretsiz olarak yorumlanır.
    let isaretsiz = negatif as u8; 
    println!("-1 as u8 = {}", isaretsiz); // 255 (çünkü 0b11111111)

    // İşaretçi dönüşümü (güvensiz kod bloğu gerekmez, ama kullanımı güvensiz olabilir)
    let sayi: u32 = 42;
    let ham_isaretci = &sayi as *const u32 as *const u8; // u32 işaretçisini u8 işaretçisine çevirdik.
    // Artık ham_isaretci, sayi'nin bellekteki ilk byte'ını gösterir.
}
```

Burada dikkat edilmesi gereken en kritik nokta: **`as` dönüşümü, veri kaybına veya mantıksal değişime yol açabilir.** Derleyici sizi uyarmaz! Bu yüzden dönüşümün sonuçlarını öngörebiliyor olmanız gerekir.

Güvenli dönüşümler için `From`/`Into` traitleri veya `try_from`/`try_into` (sonuç `Result` döndüren) gibi mekanizmalar da vardır, ancak bu bölümün konusu `as` ile yapılan temel dönüşümlerdir.

---

## 3. Literals (Sabit Değerler)

Rust, farklı türlerde sabit değerler yazmamıza olanak tanır. Okunabilirliği artırmak ve türü belirtmek için çeşitli sonekler (suffix) ve ayraçlar kullanabiliriz.

### Sayısal Sabitler

| Tür | Örnek | Açıklama |
|-----|-------|----------|
| Tam sayı (varsayılan `i32`) | `42`, `0xFF`, `0o77`, `0b1010` | Ondalık, onaltılık, sekizlik, ikilik sistemde yazılabilir. |
| Kayan nokta (varsayılan `f64`) | `3.14`, `2.0e5` | Bilimsel gösterim (e-notation) desteklenir. |
| Tür belirteci (suffix) | `42u8`, `3.14f32`, `1_000_000u64` | Değerin sonuna tür yazılarak tam olarak hangi tür olduğu belirtilir. |
| Görsel ayraç `_` | `1_000_000`, `0b1111_0000` | `_` derleyici tarafından yok sayılır, sadece okunabilirliği artırır. |

### Byte ve Karakter Sabitleri
- **Byte sabiti:** `b'A'` → `u8` türündedir, yani 65 değerini verir. Sadece ASCII karakterler için geçerlidir.
- **Karakter sabiti:** `'A'` → `char` türündedir, Unicode skalar değeri tutar (4 byte).

### String Sabitleri
- `"Merhaba"` → `&str` (string slice) türündedir. Çift tırnakla yazılır.
- **Ham string (raw string):** `r#"Burada "tırnak" işaretleri var"#` şeklinde yazılır. Kaçış karakterlerine (escape) gerek kalmaz.

### Boolean
- `true`, `false` → `bool` türü.

### Örnek Kod:

```rust
fn main() {
    // Tür belirteçleri
    let x = 42u16;          // u16 türünde
    let y = 3.14f32;        // f32 türünde
    let popülasyon = 7_500_000_000u64; // alt çizgi okunabilirlik için

    // Farklı sayı sistemleri
    let hex = 0x1A;        // 26 ondalık
    let octal = 0o52;      // 42 ondalık
    let binary = 0b1101;   // 13 ondalık

    // Byte ve karakter
    let byte_a = b'A';      // u8: 65
    let char_a = 'A';       // char: 'A'
    let kalp = '❤';       // char, Unicode

    // String
    let normal = "Satır başı: \n";
    let ham = r#"Burada \n yeni satır olmaz, "çift tırnak" da var"#;

    println!("x={}, y={}, popülasyon={}", x, y, popülasyon);
    println!("hex={}, octal={}, binary={}", hex, octal, binary);
    println!("byte_a: {}, char_a: {}", byte_a, char_a);
    println!("{}", ham);
}
```

Gördüğünüz gibi, özellikle sayısal sabitlerle çalışırken türü netleştirmek için sonek kullanmak oldukça pratiktir. Aksi halde derleyici tam sayılar için `i32`, kayan noktalı sayılar için `f64` varsayar.

---

## 4. Inference (Tür Çıkarımı)

Rust derleyicisinin tür çıkarım motoru oldukça güçlüdür. Sadece değişkenin ilk kullanıldığı yere bakarak değil, bağlamın geneline bakarak türü belirleyebilir. Buna rağmen bazen türü açıkça belirtmemiz gerekir.

### Varsayılan Türler
Eğer derleyici türü çıkaramazsa ve biz de belirtmezsek, varsayılan türleri kullanır:
- Tam sayılar: `i32`
- Kayan nokta: `f64`

### Tür Çıkarımı Nasıl Çalışır?

Basit bir örnek:
```rust
let sayi = 10;      // 10 tam sayısı, varsayılan i32 olur
let pi = 3.14;      // varsayılan f64
```

Ancak tür daha sonraki kullanımdan da çıkarılabilir:
```rust
let mut veri = Vec::new();  // Boş vektör, türü henüz belli değil.
veri.push(42u8);            // Artık veri: Vec<u8> olduğu çıkarılır.
```

`Vec::new()` boş bir vektör oluşturur, ancak içine ne tür eleman konacağı belli değildir. `push(42u8)` çağrısıyla derleyici vektörün `Vec<u8>` olduğunu anlar. Eğer bu `push` işlemi olmasaydı, derleyici hata verirdi: "type annotations needed".

### Karmaşık Tür Çıkarımı Örneği:

```rust
fn main() {
    let sayilar = vec![1, 2, 3, 4, 5];  // i32 vektörü
    let toplam: u64 = sayilar.iter().sum(); // sum() genellikle aynı türü döner ama biz u64 istedik.
    // .sum() metodu, sayilar'ın eleman türüne (i32) göre çalışır, 
    // fakat biz toplam değişkeninin türünü u64 belirttiğimiz için 
    // Rust, sum() çıktısını u64'e çevirir. (sum aslında FromIterator trait'i ile çalışır)
    
    // Aşağıdaki durumda tür çıkarımı belirsizdir ve hata verir:
    // let belirsiz = sayilar.into_iter().collect(); // collect ne toplayacağını bilemez!
    // Çözüm: tür belirtmek (türbini kullanabiliriz)
    let belirsiz: Vec<u32> = sayilar.into_iter().map(|x| x as u32).collect();
    println!("{:?}", belirsiz);
}
```

`collect()` metodu, dönüştürüleceği koleksiyon türünü bilmek zorundadır. Bu yüzden genelde "turbofish" söz dizimi (`::<>`) veya değişken türü belirtilerek yardım edilir: `let v: Vec<_> = ...` veya `let v = ...::<Vec<i32>>()`.

### Dikkat: Gölgeleme (Shadowing) ve Tür Değişimi
Rust'ta aynı isimde yeni bir değişken tanımlayarak türü değiştirebiliriz. Buna gölgeleme denir. Bu, tür çıkarımıyla birleşince kullanışlıdır:

```rust
let sayi = "42";
let sayi: u32 = sayi.parse().expect("Sayı değil!"); // Önceki &str'i gölgeledi, şimdi u32
```

---

## 5. Aliasing (Tür Takma Adı - `type`)

Büyük projelerde, özellikle uzun veya karmaşık tür isimlerini daha okunabilir kılmak için `type` anahtar sözcüğü ile **tür takma adı (alias)** tanımlayabiliriz. Bu, yeni bir tür oluşturmaz; sadece mevcut türe verilen ikinci bir isimdir. Yani takma ad ile orijinal tür tamamen aynı kabul edilir.

### Neden Kullanırız?
- Kodun anlamını güçlendirmek (örneğin `u64` yerine `NanoSecond`).
- Uzun tür ifadelerini kısaltmak (`HashMap<String, Vec<u8>>` yerine `KayitTablosu`).
- Jenerik tür parametrelerini belirli bir bağlamda sabitlemek.

### Söz Dizimi:
```rust
type YeniIsim = VarolanTur;
```

### Örnekler:

```rust
// Anlamsal takma adlar: birimleri ifade eder.
type NanoSecond = u64;
type Inch = u32;
type Metre = f64;

fn lazer_mesafesi() -> NanoSecond {
    // Lazerin geri dönüş süresini nanosaniye cinsinden döndürelim
    15_000_000_000u64 // 15 saniye = 15 milyar nanosaniye
}

fn uzunluk_cevir(inch: Inch) -> Metre {
    (inch as f64) * 0.0254
}

// Karmaşık türleri kısaltma
use std::collections::HashMap;
type OgrenciNotlari = HashMap<String, Vec<u8>>;

fn main() {
    let sure: NanoSecond = lazer_mesafesi();
    println!("Lazer süresi: {} ns", sure);

    let ekran_boyutu: Inch = 24;
    let metre = uzunluk_cevir(ekran_boyutu);
    println!("{} inç = {:.2} metre", ekran_boyutu, metre);

    // Takma adı kullanarak HashMap oluşturma
    let mut sinif: OgrenciNotlari = HashMap::new();
    sinif.insert("Ali".to_string(), vec![90, 85, 92]);
    // sinif'in türü aslında HashMap<String, Vec<u8>> ile aynıdır.
}
```

Bu örnekte `NanoSecond` ve `Inch` ayrı birer tür değildir; hâlâ `u64` ve `u32`'dir. Yani bir fonksiyon `NanoSecond` bekliyorsa, `u64` gönderebilirsiniz. Tür güvenliğini artırmak için ayrı türler istiyorsanız `newtype` pattern'ini (örneğin `struct NanoSecond(u64);`) kullanmanız gerekir, ancak bu `Types` sayfasının değil, ileri seviye konuların kapsamındadır.

Ayrıca, jenerik türlerle de takma ad kullanabiliriz:
```rust
type Result<T> = std::result::Result<T, std::io::Error>;
// Artık IO işlemleri için Result<String> yazabiliriz, 
// bu da std::result::Result<String, std::io::Error> anlamına gelir.
```
Bu tür kullanım Rust standart kütüphanesinde ve birçok kütüphanede yaygındır (örneğin `io::Result<T>`).

---

## Özet ve İpuçları

- **Casting (`as`):** Dönüşümler açık olmalı; veri kaybı olabileceğini unutma. `as` sadece sayısal ve işaretçi türler arasında çalışır.
- **Literals:** Sayıları okunabilir yazmak için `1_000_000`, tür belirtmek için `42u8` gibi sonekler kullan. Ham stringler (`r#"..."#`) kaçış karakteri derdini bitirir.
- **Inference:** Çoğu yerde tür yazmana gerek kalmaz. Ama `collect()` gibi belirsizlik durumlarında derleyiciye yardım etmen gerekir. Varsayılan sayısal türler `i32` ve `f64`'tür.
- **Aliasing:** `type` ile mevcut türlere anlamlı isimler ver. Bu yeni bir tür değil, sadece takma addır. Daha sıkı tür güvenliği için `struct` sarmalayıcıları (newtype) kullan.

Bu temel tür mekanizmaları, Rust'ta güvenli ve okunabilir kod yazmanın yapı taşlarıdır. Bir sonraki adımda, `From`/`Into` ve `TryFrom`/`TryInto` gibi daha güvenli dönüşüm trait'lerini inceleyerek tür dönüşümlerindeki kontrolü artırabilirsin.