# Rust'ta Özel Tipler (Custom Types) — Kapsamlı Ders

Hoş geldiniz! Bu derste Rust'ın **özel veri tipleri** oluşturma mekanizmasını en temelden, bol örneklerle ve satır satır açıklayarak öğreneceğiz. Rust'ta kendi veri tiplerinizi oluşturmanızı sağlayan temel araçlar şunlardır:

| Araç | Görevi |
|------|--------|
| `struct` | Yapı (structure) tanımlar |
| `enum` | Numaralandırma (enumeration) tanımlar |
| `const` / `static` | Sabit (constant) tanımlar |

Hadi başlayalım! 🚀

---

## 1. Structs (Yapılar) 🏗️

### 1.1 Struct Nedir?

`struct` (structure), birden fazla veriyi **tek bir çatı altında** toplamamızı sağlayan veri tipidir. C/C++'taki `struct` yapısına benzer, ama çok daha güçlüdür. Rust'ta **üç farklı struct türü** vardır:

1. **Klasik (Named-Field) Struct** — C'deki klasik struct
2. **Tuple Struct** — İsimlendirilmemiş, sadece türlerle tanımlanan struct
3. **Unit Struct** — Hiç alanı olmayan, boş struct

### 1.2 Kod Örneği ve Satır Satır Açıklama

```rust
// Kullanılmayan kodlar için uyarıları gizle
#![allow(dead_code)]

// Debug trait'ini türetiyoruz ki println!("{:?}", ...) ile yazdırabilelim
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// Birim (unit) struct — hiçbir alanı yok
struct Unit;

// Tuple struct — isim yerine sıra ile erişilir
struct Pair(i32, f32);

// İki alanlı klasik struct
struct Point {
    x: f32,
    y: f32,
}

// Struct'lar başka struct'ların içinde alan olarak kullanılabilir
struct Rectangle {
    // Bir dikdörtgen, sol üst ve sağ alt köşesi ile tanımlanır
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // --- 1. Field Init Shorthand (Kısayol Başlatma) ---
    // Değişken adı ile alan adı aynıysa, sadece isim yeterlidir
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };  // { name: name, age: age } ile aynı

    // Debug formatında yazdır
    println!("{:?}", peter);
    // Çıktı: Person { name: "Peter", age: 27 }

    // --- 2. Klasik Struct Oluşturma ---
    let point: Point = Point { x: 5.2, y: 0.4 };
    let another_point: Point = Point { x: 10.3, y: 0.2 };

    // Alanlara nokta (.) ile erişim
    println!("point coordinates: ({}, {})", point.x, point.y);
    // Çıktı: point coordinates: (5.2, 0.4)

    // --- 3. Struct Update Syntax (.. operatörü) ---
    // Bir struct'ın bazı alanlarını başka struct'tan al, geri kalanını kendin yaz
    let bottom_right = Point { x: 10.3, ..another_point };
    // bottom_right.x = 10.3 (yeni yazdığımız)
    // bottom_right.y = another_point.y'dan alındı (0.2)
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);
    // Çıktı: second point: (10.3, 0.2)

    // --- 4. Destructuring (Yapı Bozma / Ayrıştırma) ---
    // Bir struct'ı parçalarına ayırarak değişkene ata
    let Point { x: left_edge, y: top_edge } = point;
    // x'in değeri left_edge'e, y'nin değeri top_edge'e gitti

    // --- 5. İç İçe Struct Oluşturma ---
    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // --- 6. Unit Struct Oluşturma ---
    let _unit = Unit;  // Parantez yok, çünkü alanı yok

    // --- 7. Tuple Struct Oluşturma ---
    let pair = Pair(1, 0.1);

    // Tuple struct'lara indeks ile erişim (tuple'lar gibi)
    println!("pair contains {:?} and {:?}", pair.0, pair.1);
    // Çıktı: pair contains 1 and 0.1

    // Tuple struct'ı destructuring ile parçala
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
    // Çıktı: pair contains 1 and 0.1
}
```

### 1.3 Önemli Kavramlar

#### 🔹 Struct Update Syntax (`..`)
Mevcut bir struct'ın bazı alanlarını kopyalayıp, bazılarını değiştirmek istediğinizde çok kullanışlıdır:
```rust
let yeni_nokta = Point { x: 99.0, ..eski_nokta };
// x'i değiştirdik, y eski_nokta'dan geldi
```

#### 🔹 Field Init Shorthand
Değişkenin adı ile struct alanının adı aynıysa tekrar yazmaya gerek yok:
```rust
let name = String::from("Ali");
let kisi = Person { name, age: 30 };  // name: name yazmak yerine
```

#### 🔹 Destructuring (Yapı Bozma)
Struct'ın içindeki değerleri doğrudan değişkenlere çıkarır:
```rust
let Point { x, y } = point;  // x ve y artık ayrı değişken
let Point { x: en, y: boy } = point;  // Farklı isim de verebilirsin
```

#### 🔹 Üç Struct Türü Karşılaştırması

| Tür | Tanım | Örnek | Erişim |
|-----|-------|-------|--------|
| Klasik | İsimli alanlar | `Point { x: 1.0, y: 2.0 }` | `p.x` |
| Tuple | İsimsiz, sıralı | `Pair(1, 0.1)` | `p.0`, `p.1` |
| Unit | Alan yok | `Unit` | — |

---

## 2. Enums (Numaralandırmalar) 🎯

### 2.1 Enum Nedir?

`enum` (enumeration), bir değerin **birkaç farklı varyanttan (variant)** biri olabileceğini belirttiğimiz tiptir. C'deki enum'dan çok daha güçlüdür çünkü **her varyant kendi verisini taşıyabilir**.

Rust'ın en meşhur enum'u `Option<T>`'dir:
```rust
enum Option<T> {
    Some(T),   // Bir değer var
    None,      // Değer yok
}
```

### 2.2 Kod Örneği ve Satır Satır Açıklama

```rust
// Bir web olayını sınıflandıran enum
// Her varyant farklı türde veri taşıyabilir
enum WebEvent {
    // Unit-like varyant — veri taşımaz
    PageLoad,
    PageUnload,
    
    // Tuple struct benzeri varyant — tek değer taşır
    KeyPress(char),
    Paste(String),
    
    // C-like struct benzeri varyant — isimli alanlar taşır
    Click { x: i64, y: i64 },
}

// WebEvent alan bir fonksiyon
fn inspect(event: WebEvent) {
    // match ile her varyantı ayrı ayrı ele alıyoruz
    match event {
        WebEvent::PageLoad => println!("sayfa yüklendi"),
        
        WebEvent::PageUnload => println!("sayfa boşaltıldı"),
        
        // Tuple varyanttan değeri çıkar (destructuring)
        WebEvent::KeyPress(c) => println!("'{}' tuşuna basıldı.", c),
        
        WebEvent::Paste(s) => println!("\"{}\" yapıştırıldı.", s),
        
        // Struct varyanttan x ve y'yi çıkar
        WebEvent::Click { x, y } => {
            println!("x={}, y={} konumuna tıklandı.", x, y);
        },
    }
}

fn main() {
    // Her varyanttan birer örnek oluşturalım
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("merhaba dünya".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    // Hepsini inspect fonksiyonuna gönder
    inspect(pressed);   // 'x' tuşuna basıldı.
    inspect(pasted);    // "merhaba dünya" yapıştırıldı.
    inspect(click);     // x=20, y=80 konumuna tıklandı.
    inspect(load);      // sayfa yüklendi
    inspect(unload);    // sayfa boşaltıldı
}
```

### 2.3 Enum Varyant Türleri

Bir enum içinde **üç farklı varyant türü** bir arada bulunabilir:

```rust
enum Mesaj {
    Quit,                           // Unit-like (veri yok)
    Renk(i32, i32, i32),           // Tuple-like (sıralı veri)
    Metin(String),                  // Tuple-like (tek veri)
    Konum { x: f64, y: f64 },      // Struct-like (isimli alanlar)
}
```

> 💡 **Kilit Nokta:** Her varyant **bağımsız bir tip** gibidir. `KeyPress('a')` ile `Paste("abc")` tamamen farklı şeylerdir.

### 2.4 Type Alias (Tür Eş İsmi)

Uzun enum isimlerini kısaltmak için `type` anahtar kelimesini kullanabiliriz:

```rust
enum CokUzunIsimliBirEnum {
    Ekle,
    Cikar,
}

// Kısa bir takma ad oluştur
type Islem = CokUzunIsimliBirEnum;

fn main() {
    let x = Islem::Ekle;  // Uzun ismi yazmaya gerek yok
}
```

### 2.5 `Self` Kullanımı (En Yaygın Kullanım)

`impl` bloğu içinde enum'un kendi ismi yerine `Self` yazmak çok yaygındır:

```rust
enum Islem {
    Ekle,
    Cikar,
}

impl Islem {
    fn calistir(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Ekle => x + y,       // Self = Islem
            Self::Cikar => x - y,
        }
    }
}

fn main() {
    let sonuc = Islem::Ekle.calistir(5, 3);
    println!("Sonuç: {}", sonuc);  // 8
}
```

### 2.6 Enum + Match İlişkisi

Rust'ta enum ve `match` **ayrılmaz bir ikili**dir. Compiler, `match`'te tüm varyantların ele alındığından emin olur. Eğer birini unutursanız **compile hatası** alırsınız. Bu, "eksik durum" hatalarını önler.

```rust
match event {
    WebEvent::PageLoad => println!("yüklendi"),
    WebEvent::PageUnload => println!("boşaltıldı"),
    // KeyPress, Paste, Click eksik! → HATA!
}
```

---

## 3. Constants (Sabitler) 🔒

### 3.1 Sabit Nedir?

Sabitler, bir kere atandığında **değiştirilemeyen** değerlerdir. Rust'ta iki tür sabit vardır:

| Anahtar Kelime | Özellik |
|----------------|---------|
| `const` | Değiştirilemez değer (yaygın kullanım) |
| `static` | `'static` ömürlü, değiştirilebilir olabilir (`unsafe` gerekir) |

Her ikisi de **her kapsamda** (global dahil) tanımlanabilir ve **tür belirtimi zorunludur**.

### 3.2 Kod Örneği ve Satır Satır Açıklama

```rust
// Global sabitler — tüm kapsamın dışında tanımlanır
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

// Bir fonksiyon içinde sabite erişim
fn is_big(n: i32) -> bool {
    n > THRESHOLD  // const'a her yerden erişilebilir
}

fn main() {
    let n = 16;

    // Sabitleri kullan
    println!("Bu dil: {}", LANGUAGE);
    println!("Eşik değeri: {}", THRESHOLD);
    println!("{} sayısı {}", n, if is_big(n) { "büyük" } else { "küçük" });

    // HATA! const değiştirilemez
    THRESHOLD = 5;
    // ^ Bu satırı yorum yapmazsanız compile hatası alırsınız
}
```

### 3.3 `const` vs `let` Farkı

| Özellik | `const` | `let` |
|---------|---------|-------|
| Değiştirilebilirlik | Asla değiştirilemez | `mut` ile değiştirilebilir |
| Kapsam | Global veya lokal | Sadece lokal |
| Tür belirtimi | Zorunlu | Genelde çıkarım yapılır |
| Değer | Compile-time'da bilinmeli | Runtime'da hesaplanabilir |
| Bellek | Her kullanımda kopyalanabilir | Tek bir bellek konumu |

### 3.4 `const` vs `static` Farkı

```rust
const SABIT: i32 = 42;              // Değiştirilemez
static MUTABLE_STATIC: i32 = 100;   // Değiştirilemez (default)
static mut DEGISKEN_STATIC: i32 = 0; // Değiştirilebilir, AMA unsafe!
```

**Temel farklar:**

- `const`: Değer **kullanıldığı yere kopyalanır** (inline edilir). Bellekte sabit bir adresi yoktur.
- `static`: Bellekte **gerçek bir adresi** olan sabit bir değişkendir. Tüm program boyunca aynı yerde durur.

> ⚠️ **Dikkat:** `static mut` değiştirmek **`unsafe`** blok gerektirir çünkü veri yarışına (data race) yol açabilir.

```rust
static mut SAYAC: i32 = 0;

fn main() {
    unsafe {
        SAYAC += 1;  // Unsafe olmadan derlenmez!
        println!("{}", SAYAC);
    }
}
```

### 3.5 `'static` Lifetime

`static` anahtar kelimesi ile tanımlanan değişkenler otomatik olarak `'static` lifetime'a sahiptir. Bu, programın **başlangıcından sonuna kadar** geçerli oldukları anlamına gelir.

```rust
static ISIM: &str = "Rust";  // &'static str tipindedir
```

---

## 4. Özet ve Pratik İpuçları 📝

### Ne Zaman Hangi Tipi Kullanmalıyız?

| Durum | Kullanılacak |
|-------|--------------|
| Farklı türde alanları gruplamak istiyorum | `struct` |
| Bir değerin birkaç durumdan birinde olmasını istiyorum | `enum` |
| Program boyunca değişmeyecek sabit bir değer | `const` |
| Global, bellekte sabit adresi olan bir değer | `static` |
| Sadece marker (işaret) olarak bir tip lazım | Unit struct (`struct Marker;`) |
| Tuple gibi ama isimli olsun istiyorum | Tuple struct (`struct Renk(u8, u8, u8);`) |

### Sık Yapılan Hatalar

1. ❌ **const'a değer atamaya çalışmak:**
   ```rust
   const X: i32 = 5;
   X = 10;  // HATA! const değiştirilemez
   ```

2. ❌ **match'te tüm varyantları ele almamak:**
   ```rust
   match event {
       WebEvent::PageLoad => {},
       // Diğer varyantlar eksik → HATA!
   }
   ```

3. ❌ **Struct update syntax'ta ownership sorunu:**
   ```rust
   let p2 = Point { x: 1.0, ..p1 };
   // p1'deki String gibi move gerektiren alanlar varsa p1 artık kullanılamaz
   ```

### Alıştırma Önerileri

1. Bir `Dikdortgen` struct'ı oluşturun ve alanını hesaplayan bir fonksiyon yazın.
2. Bir `Sekil` enum'u oluşturun (Daire, Ucgen, Dikdortgen varyantları olsun).
3. Her şekil için alan hesaplayan bir `match` ifadesi yazın.
4. Global bir `PI` sabiti tanımlayın ve kullanın.

---

## 5. Sonraki Adımlar

Bu bölümde öğrendiklerimiz, Rust'ın tip sisteminin temelini oluşturuyor. Bir sonraki adımda şunlara bakmanızı öneririm:

- **`impl` blokları** — Struct ve enum'lara metod eklemek
- **Generics** — Tipler üzerinde genelleme yapmak
- **Traits** — Davranış paylaşımı (interface benzeri)
- **Pattern Matching** — `match` ile daha derinlemesine yapı bozma

> 🎓 **Unutmayın:** Rust'ın güçlü tip sistemi, hataları **çalışma zamanından derleme zamanına** taşır. `enum` + `match` ikilisi, "imkansız durumları imkansız kılar" (make impossible states impossible).

Başarılar! 🦀
</think>

# Rust'ta Özel Tipler (Custom Types) — Kapsamlı Ders

Hoş geldiniz! Bu derste Rust'ın **özel veri tipleri** oluşturma mekanizmasını en temelden, bol örneklerle ve satır satır açıklayarak öğreneceğiz. Rust'ta kendi veri tiplerinizi oluşturmanızı sağlayan temel araçlar şunlardır:

| Araç | Görevi |
|------|--------|
| `struct` | Yapı (structure) tanımlar |
| `enum` | Numaralandırma (enumeration) tanımlar |
| `const` / `static` | Sabit (constant) tanımlar |

Hadi başlayalım! 🚀

---

## 1. Structs (Yapılar) 🏗️

### 1.1 Struct Nedir?

`struct` (structure), birden fazla veriyi **tek bir çatı altında** toplamamızı sağlayan veri tipidir. C/C++'taki `struct` yapısına benzer, ama çok daha güçlüdür. Rust'ta **üç farklı struct türü** vardır:

1. **Klasik (Named-Field) Struct** — C'deki klasik struct
2. **Tuple Struct** — İsimlendirilmemiş, sadece türlerle tanımlanan struct
3. **Unit Struct** — Hiç alanı olmayan, boş struct

### 1.2 Kod Örneği ve Satır Satır Açıklama

```rust
// Kullanılmayan kodlar için uyarıları gizle
#![allow(dead_code)]

// Debug trait'ini türetiyoruz ki println!("{:?}", ...) ile yazdırabilelim
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// Birim (unit) struct — hiçbir alanı yok
struct Unit;

// Tuple struct — isim yerine sıra ile erişilir
struct Pair(i32, f32);

// İki alanlı klasik struct
struct Point {
    x: f32,
    y: f32,
}

// Struct'lar başka struct'ların içinde alan olarak kullanılabilir
struct Rectangle {
    // Bir dikdörtgen, sol üst ve sağ alt köşesi ile tanımlanır
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // --- 1. Field Init Shorthand (Kısayol Başlatma) ---
    // Değişken adı ile alan adı aynıysa, sadece isim yeterlidir
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };  // { name: name, age: age } ile aynı

    // Debug formatında yazdır
    println!("{:?}", peter);
    // Çıktı: Person { name: "Peter", age: 27 }

    // --- 2. Klasik Struct Oluşturma ---
    let point: Point = Point { x: 5.2, y: 0.4 };
    let another_point: Point = Point { x: 10.3, y: 0.2 };

    // Alanlara nokta (.) ile erişim
    println!("point coordinates: ({}, {})", point.x, point.y);
    // Çıktı: point coordinates: (5.2, 0.4)

    // --- 3. Struct Update Syntax (.. operatörü) ---
    // Bir struct'ın bazı alanlarını başka struct'tan al, geri kalanını kendin yaz
    let bottom_right = Point { x: 10.3, ..another_point };
    // bottom_right.x = 10.3 (yeni yazdığımız)
    // bottom_right.y = another_point.y'dan alındı (0.2)
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);
    // Çıktı: second point: (10.3, 0.2)

    // --- 4. Destructuring (Yapı Bozma / Ayrıştırma) ---
    // Bir struct'ı parçalarına ayırarak değişkene ata
    let Point { x: left_edge, y: top_edge } = point;
    // x'in değeri left_edge'e, y'nin değeri top_edge'e gitti

    // --- 5. İç İçe Struct Oluşturma ---
    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // --- 6. Unit Struct Oluşturma ---
    let _unit = Unit;  // Parantez yok, çünkü alanı yok

    // --- 7. Tuple Struct Oluşturma ---
    let pair = Pair(1, 0.1);

    // Tuple struct'lara indeks ile erişim (tuple'lar gibi)
    println!("pair contains {:?} and {:?}", pair.0, pair.1);
    // Çıktı: pair contains 1 and 0.1

    // Tuple struct'ı destructuring ile parçala
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
    // Çıktı: pair contains 1 and 0.1
}
```

### 1.3 Önemli Kavramlar

#### 🔹 Struct Update Syntax (`..`)
Mevcut bir struct'ın bazı alanlarını kopyalayıp, bazılarını değiştirmek istediğinizde çok kullanışlıdır:
```rust
let yeni_nokta = Point { x: 99.0, ..eski_nokta };
// x'i değiştirdik, y eski_nokta'dan geldi
```

#### 🔹 Field Init Shorthand
Değişkenin adı ile struct alanının adı aynıysa tekrar yazmaya gerek yok:
```rust
let name = String::from("Ali");
let kisi = Person { name, age: 30 };  // name: name yazmak yerine
```

#### 🔹 Destructuring (Yapı Bozma)
Struct'ın içindeki değerleri doğrudan değişkenlere çıkarır:
```rust
let Point { x, y } = point;  // x ve y artık ayrı değişken
let Point { x: en, y: boy } = point;  // Farklı isim de verebilirsin
```

#### 🔹 Üç Struct Türü Karşılaştırması

| Tür | Tanım | Örnek | Erişim |
|-----|-------|-------|--------|
| Klasik | İsimli alanlar | `Point { x: 1.0, y: 2.0 }` | `p.x` |
| Tuple | İsimsiz, sıralı | `Pair(1, 0.1)` | `p.0`, `p.1` |
| Unit | Alan yok | `Unit` | — |

---

## 2. Enums (Numaralandırmalar) 🎯

### 2.1 Enum Nedir?

`enum` (enumeration), bir değerin **birkaç farklı varyanttan (variant)** biri olabileceğini belirttiğimiz tiptir. C'deki enum'dan çok daha güçlüdür çünkü **her varyant kendi verisini taşıyabilir**.

Rust'ın en meşhur enum'u `Option<T>`'dir:
```rust
enum Option<T> {
    Some(T),   // Bir değer var
    None,      // Değer yok
}
```

### 2.2 Kod Örneği ve Satır Satır Açıklama

```rust
// Bir web olayını sınıflandıran enum
// Her varyant farklı türde veri taşıyabilir
enum WebEvent {
    // Unit-like varyant — veri taşımaz
    PageLoad,
    PageUnload,
    
    // Tuple struct benzeri varyant — tek değer taşır
    KeyPress(char),
    Paste(String),
    
    // C-like struct benzeri varyant — isimli alanlar taşır
    Click { x: i64, y: i64 },
}

// WebEvent alan bir fonksiyon
fn inspect(event: WebEvent) {
    // match ile her varyantı ayrı ayrı ele alıyoruz
    match event {
        WebEvent::PageLoad => println!("sayfa yüklendi"),
        
        WebEvent::PageUnload => println!("sayfa boşaltıldı"),
        
        // Tuple varyanttan değeri çıkar (destructuring)
        WebEvent::KeyPress(c) => println!("'{}' tuşuna basıldı.", c),
        
        WebEvent::Paste(s) => println!("\"{}\" yapıştırıldı.", s),
        
        // Struct varyanttan x ve y'yi çıkar
        WebEvent::Click { x, y } => {
            println!("x={}, y={} konumuna tıklandı.", x, y);
        },
    }
}

fn main() {
    // Her varyanttan birer örnek oluşturalım
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("merhaba dünya".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    // Hepsini inspect fonksiyonuna gönder
    inspect(pressed);   // 'x' tuşuna basıldı.
    inspect(pasted);    // "merhaba dünya" yapıştırıldı.
    inspect(click);     // x=20, y=80 konumuna tıklandı.
    inspect(load);      // sayfa yüklendi
    inspect(unload);    // sayfa boşaltıldı
}
```

### 2.3 Enum Varyant Türleri

Bir enum içinde **üç farklı varyant türü** bir arada bulunabilir:

```rust
enum Mesaj {
    Quit,                           // Unit-like (veri yok)
    Renk(i32, i32, i32),           // Tuple-like (sıralı veri)
    Metin(String),                  // Tuple-like (tek veri)
    Konum { x: f64, y: f64 },      // Struct-like (isimli alanlar)
}
```

> 💡 **Kilit Nokta:** Her varyant **bağımsız bir tip** gibidir. `KeyPress('a')` ile `Paste("abc")` tamamen farklı şeylerdir.

### 2.4 Type Alias (Tür Eş İsmi)

Uzun enum isimlerini kısaltmak için `type` anahtar kelimesini kullanabiliriz:

```rust
enum CokUzunIsimliBirEnum {
    Ekle,
    Cikar,
}

// Kısa bir takma ad oluştur
type Islem = CokUzunIsimliBirEnum;

fn main() {
    let x = Islem::Ekle;  // Uzun ismi yazmaya gerek yok
}
```

### 2.5 `Self` Kullanımı (En Yaygın Kullanım)

`impl` bloğu içinde enum'un kendi ismi yerine `Self` yazmak çok yaygındır:

```rust
enum Islem {
    Ekle,
    Cikar,
}

impl Islem {
    fn calistir(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Ekle => x + y,       // Self = Islem
            Self::Cikar => x - y,
        }
    }
}

fn main() {
    let sonuc = Islem::Ekle.calistir(5, 3);
    println!("Sonuç: {}", sonuc);  // 8
}
```

### 2.6 Enum + Match İlişkisi

Rust'ta enum ve `match` **ayrılmaz bir ikili**dir. Compiler, `match`'te tüm varyantların ele alındığından emin olur. Eğer birini unutursanız **compile hatası** alırsınız. Bu, "eksik durum" hatalarını önler.

```rust
match event {
    WebEvent::PageLoad => println!("yüklendi"),
    WebEvent::PageUnload => println!("boşaltıldı"),
    // KeyPress, Paste, Click eksik! → HATA!
}
```

---

## 3. Constants (Sabitler) 🔒

### 3.1 Sabit Nedir?

Sabitler, bir kere atandığında **değiştirilemeyen** değerlerdir. Rust'ta iki tür sabit vardır:

| Anahtar Kelime | Özellik |
|----------------|---------|
| `const` | Değiştirilemez değer (yaygın kullanım) |
| `static` | `'static` ömürlü, değiştirilebilir olabilir (`unsafe` gerekir) |

Her ikisi de **her kapsamda** (global dahil) tanımlanabilir ve **tür belirtimi zorunludur**.

### 3.2 Kod Örneği ve Satır Satır Açıklama

```rust
// Global sabitler — tüm kapsamın dışında tanımlanır
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

// Bir fonksiyon içinde sabite erişim
fn is_big(n: i32) -> bool {
    n > THRESHOLD  // const'a her yerden erişilebilir
}

fn main() {
    let n = 16;

    // Sabitleri kullan
    println!("Bu dil: {}", LANGUAGE);
    println!("Eşik değeri: {}", THRESHOLD);
    println!("{} sayısı {}", n, if is_big(n) { "büyük" } else { "küçük" });

    // HATA! const değiştirilemez
    THRESHOLD = 5;
    // ^ Bu satırı yorum yapmazsanız compile hatası alırsınız
}
```

### 3.3 `const` vs `let` Farkı

| Özellik | `const` | `let` |
|---------|---------|-------|
| Değiştirilebilirlik | Asla değiştirilemez | `mut` ile değiştirilebilir |
| Kapsam | Global veya lokal | Sadece lokal |
| Tür belirtimi | Zorunlu | Genelde çıkarım yapılır |
| Değer | Compile-time'da bilinmeli | Runtime'da hesaplanabilir |
| Bellek | Her kullanımda kopyalanabilir | Tek bir bellek konumu |

### 3.4 `const` vs `static` Farkı

```rust
const SABIT: i32 = 42;              // Değiştirilemez
static MUTABLE_STATIC: i32 = 100;   // Değiştirilemez (default)
static mut DEGISKEN_STATIC: i32 = 0; // Değiştirilebilir, AMA unsafe!
```

**Temel farklar:**

- `const`: Değer **kullanıldığı yere kopyalanır** (inline edilir). Bellekte sabit bir adresi yoktur.
- `static`: Bellekte **gerçek bir adresi** olan sabit bir değişkendir. Tüm program boyunca aynı yerde durur.

> ⚠️ **Dikkat:** `static mut` değiştirmek **`unsafe`** blok gerektirir çünkü veri yarışına (data race) yol açabilir.

```rust
static mut SAYAC: i32 = 0;

fn main() {
    unsafe {
        SAYAC += 1;  // Unsafe olmadan derlenmez!
        println!("{}", SAYAC);
    }
}
```

### 3.5 `'static` Lifetime

`static` anahtar kelimesi ile tanımlanan değişkenler otomatik olarak `'static` lifetime'a sahiptir. Bu, programın **başlangıcından sonuna kadar** geçerli oldukları anlamına gelir.

```rust
static ISIM: &str = "Rust";  // &'static str tipindedir
```

---

## 4. Özet ve Pratik İpuçları 📝

### Ne Zaman Hangi Tipi Kullanmalıyız?

| Durum | Kullanılacak |
|-------|--------------|
| Farklı türde alanları gruplamak istiyorum | `struct` |
| Bir değerin birkaç durumdan birinde olmasını istiyorum | `enum` |
| Program boyunca değişmeyecek sabit bir değer | `const` |
| Global, bellekte sabit adresi olan bir değer | `static` |
| Sadece marker (işaret) olarak bir tip lazım | Unit struct (`struct Marker;`) |
| Tuple gibi ama isimli olsun istiyorum | Tuple struct (`struct Renk(u8, u8, u8);`) |

### Sık Yapılan Hatalar

1. ❌ **const'a değer atamaya çalışmak:**
   ```rust
   const X: i32 = 5;
   X = 10;  // HATA! const değiştirilemez
   ```

2. ❌ **match'te tüm varyantları ele almamak:**
   ```rust
   match event {
       WebEvent::PageLoad => {},
       // Diğer varyantlar eksik → HATA!
   }
   ```

3. ❌ **Struct update syntax'ta ownership sorunu:**
   ```rust
   let p2 = Point { x: 1.0, ..p1 };
   // p1'deki String gibi move gerektiren alanlar varsa p1 artık kullanılamaz
   ```

### Alıştırma Önerileri

1. Bir `Dikdortgen` struct'ı oluşturun ve alanını hesaplayan bir fonksiyon yazın.
2. Bir `Sekil` enum'u oluşturun (Daire, Ucgen, Dikdortgen varyantları olsun).
3. Her şekil için alan hesaplayan bir `match` ifadesi yazın.
4. Global bir `PI` sabiti tanımlayın ve kullanın.

---

## 5. Sonraki Adımlar

Bu bölümde öğrendiklerimiz, Rust'ın tip sisteminin temelini oluşturuyor. Bir sonraki adımda şunlara bakmanızı öneririm:

- **`impl` blokları** — Struct ve enum'lara metod eklemek
- **Generics** — Tipler üzerinde genelleme yapmak
- **Traits** — Davranış paylaşımı (interface benzeri)
- **Pattern Matching** — `match` ile daha derinlemesine yapı bozma

> 🎓 **Unutmayın:** Rust'ın güçlü tip sistemi, hataları **çalışma zamanından derleme zamanına** taşır. `enum` + `match` ikilisi, "imkansız durumları imkansız kılar" (make impossible states impossible).

Başarılar! 🦀

# DeepSeek Yorumu -------------------

Harika bir konu seçimi! Rust'ın en güçlü olduğu alanlardan biri olan özel veri türlerini (Custom Types) en ince detayına kadar, adeta bir ders anlatır gibi işleyelim. Bu sayfayı ve tüm alt başlıklarını kapsamlı bir öğreticiye dönüştüreceğiz.

Hazırsan, Rust'ın tip sisteminin kalbine doğru bir yolculuğa başlıyoruz.

---

### Rust'ta Özel Veri Türleri: Programcının Kil Yontma Sanatı

Bir programlama dilinde, dilin sunduğu temel veri türleri ( `i32`, `f64`, `bool`, `char`, `str` vb.) vardır. Bunlar atomdur, tuğladır. Ancak hiçbir gerçek dünya problemi, sadece atomları yan yana koyarak modellenemez. Bir kullanıcıyı, bir veritabanı bağlantısını veya bir oyun karakterini düşün. Bunlar, birden fazla temel türün anlamlı bir bütünlük içinde bir araya gelmesiyle oluşur.

İşte Rust'ta `struct` (yapı) ve `enum` (sıralama/çeşit) bu amaca hizmet eder. Kendi veri türlerini yaratarak, kodunun hem derleyici tarafından daha sıkı denetlenmesini sağlar hem de insanlar tarafından çok daha kolay anlaşılmasına olanak tanırsın. Hadi başlayalım.

---

### 1. Yapılar (`struct`): Verileri Kutsal Bir Bütünlükte Birleştirmek

`struct`, İngilizce "structure" (yapı) kelimesinden gelir. Adı üstünde, birden fazla veri parçasını tek bir çatı altında toplayan bir yapıdır. Bir nevi, kendi özel veri kapsülünü yaratırsın. Rust'ta üç farklı struct çeşidi bulunur, her birinin kendine özgü kullanım alanı vardır.

#### 1.1. İsimlendirilmiş Alanlı Yapılar (C Struct'ları / Named Fields)

Bu, en klasik ve en yaygın kullanılan struct türüdür. Her bir veri parçasına (alana) bir isim verirsin.

**Ne zaman kullanırız?**
Kapsadığı verilerin ne anlama geldiği açıkça belirtilmek istendiğinde. Kod okunabilirliğini zirveye taşır.

**Ders Anlatımı:**
Bir kullanıcıyı modellemek istediğimizi düşünelim. Bir kullanıcının bir adı, bir e-postası ve bir yaşı olabilir. `struct` anahtar kelimesiyle başlıyoruz, yapımıza bir isim veriyoruz (`User`) ve süslü parantezler içinde alanları ve türlerini `alan_adi: Tür` şeklinde sıralıyoruz.

```rust
// Bir kullanıcıyı tanımlayan özel veri türümüz.
struct User {
    // Kullanıcı adı. String türünde, çünkü değişebilir ve sahipli bir metin.
    username: String,
    // E-posta adresi. Yine String.
    email: String,
    // Kullanıcının yaşı. Pozitif bir tam sayı olacağı için u8 (0-255 arası) ideal.
    age: u8,
    // Hesap aktif mi? Mantıksal bir değer.
    active: bool,
}
```
Artık `User` diye bir veri türümüz var! Bunu kullanarak yeni bir `User` örneği (instance) oluşturalım.

```rust
fn main() {
    // Tüm alanlara değer atayarak bir User örneği oluşturuyoruz.
    let user1 = User {
        email: String::from("ornek@email.com"),
        username: String::from("rustacean42"),
        age: 30,
        active: true,
    };

    // Nokta notasyonu ile alanlara erişebiliriz.
    println!("Kullanıcı adı: {}", user1.username);
}
```

**Derleyici Gücü:**
Eğer `user1` örneğini `mut` (değişebilir) olarak tanımlarsak, alanlarını değiştirebiliriz. Ama dikkat: Rust'ta bir örneğin tamamı ya değişebilirdir ya da değildir. Tek bir alanı değişebilir yapamazsın.

#### 1.2. Demet Yapılar (`tuple structs`): İsimsiz Kahramanlar

Bazen alanlara isim vermek gereksiz olabilir. Özellikle de türün kendi ismi zaten alanların ne olduğunu yeterince açıklıyorsa. Örneğin, bir RGB rengi. `red`, `green`, `blue` alanlarına sahip bir struct oluşturmak yerine, bunların zaten her zaman sırasıyla Kırmızı, Yeşil ve Mavi olduğunu biliriz.

İşte burada tuple struct'lar devreye girer. Sözdizimi, demet (tuple) oluşturmaya çok benzer, ancak başında bir isim vardır.

**Ne zaman kullanırız?**
Türüne isim vererek tip güvenliği sağlamak istediğimiz, ancak alanlara isim vermenin laf kalabalığı yaratacağı durumlarda. Bir noktanın X ve Y koordinatları gibi.

```rust
// İki farklı tuple struct tanımlayalım.
struct Color(i32, i32, i32); // Sırasıyla R, G, B değerleri
struct Point(f64, f64, f64); // Sırasıyla X, Y, Z koordinatları

fn main() {
    // Siyah rengi oluşturalım. (Kırmızı=0, Yeşil=0, Mavi=0)
    let black = Color(0, 0, 0);

    // Bir noktayı oluşturalım.
    let origin = Point(0.0, 0.0, 0.0);

    // Değerlere, demetlerde olduğu gibi indis ile erişiriz (0'dan başlar).
    let kirmizi_kanal = black.0;
    println!("Siyah rengin kırmızı kanalı: {}", kirmizi_kanal);
}
```

**Derleyici Gücü:**
Burada can alıcı nokta şu: `Color` ve `Point` aynı iç veri türlerine sahip olsalar bile (üç tane `i32`), Rust için bunlar **tamamen farklı ve birbiriyle karıştırılamaz** türlerdir. Bir fonksiyon `Color` bekliyorsa, ona yanlışlıkla `Point` gönderemezsin. Derleyici bunu anında yakalar ve pat diye suratına vurur (elbette sevgiyle). Bu, tip güvenliğinin (type safety) en güzel örneklerindendir.

#### 1.3. Birimsiz Yapılar (`unit-like structs`): Sıfır Boyutlu Varlıklar

Bu, en garip görünen ama en havalı olanıdır. Hiçbir alanı olmayan bir struct'tır. Evet, yanlış duymadın, bomboş bir yapı.

**Ne zaman kullanırız?**
Bir türün sadece varlığının bile bir anlam ifade ettiği, herhangi bir veri taşımasına gerek olmadığı durumlarda. Özellikle **trait'ler** ile birlikte kullanıldığında inanılmaz güçlüdür. "Bu tür, şu davranışı sergileyebilir" demenin bir yoludur.

```rust
// Bir işlemin "hazır" durumda olduğunu belirten bir tür.
struct Ready;

// Bir başka işlem "işleniyor" durumunda olsun.
struct Processing;

// Ve bir diğeri "tamamlandı" durumunda.
struct Completed;
```
Bu türlerin bellekte hiç yer kaplamadığını (sıfır boyutlu tip - ZST) bilmek önemli. Bunları daha çok, genel (generic) programlamada ve durum makinelerinde (state machines) kısıtlayıcı olarak kullanacağız. Örneğin, bir fonksiyon sadece `Ready` durumundaki bir yapıyı kabul edebilir.

---

### 2. Sıralamalar (`enum`): Rust'ın Tür Sisteminin Mücevheri

Gelelim asıl büyüye. Birçok dilde `enum`'lar, sadece sabit tam sayı değerlerinin isimlendirilmiş hali olarak bulunur (Pazartesi=1, Salı=2 gibi). Rust'ta ise `enum` çok daha fazlasıdır. Bir değerin, **birbirinden farklı çeşitlerden (varyantlardan) sadece biri** olabileceğini belirtmek için kullanılır. Bu, "sum type" (toplam türü) veya "tagged union" (etiketli birleşim) olarak bilinen kavramın Rust'taki karşılığıdır.

#### 2.1. Basit Sıralamalar (C-Tarzı Enum'lar)

En temel haliyle, bir dizi sabit değeri gruplandırır.

```rust
// Bir IP adresinin iki ana versiyonu olabilir.
enum IpAddrKind {
    V4, // Versiyon 4
    V6, // Versiyon 6
}

fn main() {
    let four = IpAddrKind::V4; // Çeşitler, `EnumAdı::Çeşit` şeklinde kullanılır.
    let six = IpAddrKind::V6;
}
```
Artık `IpAddrKind` diye bir türümüz var ve bu türden bir değişken, **ya `V4` ya da `V6`** olabilir, ikisi aynı anda olamaz.

#### 2.2. Veri Taşıyan Sıralamalar: Asıl Güç Burada Başlıyor

Rust `enum`'larının asıl gücü, her bir varyantın kendine özgü veri taşıyabilmesidir. Tıpkı bir `struct` gibi! Bu, "şu türdeki bir veri, şu çeşitlerden biridir ve her çeşit farklı türde ve sayıda veri saklayabilir" dememizi sağlar.

**Ders Anlatımı:**
Bir ağ mesajı protokolü yazdığını düşün. Bir mesaj şunlardan biri olabilir:
- Bir bağlantı isteği (Connect): Hiçbir veri taşımasına gerek yok.
- Bir metin mesajı (Text): İçeriği bir `String` olarak taşır.
- Bir kullanıcı ayrılma bildirimi (Disconnect): Ayrılan kullanıcının ID'sini (`u64`) taşır.

```rust
enum Message {
    // Boş bir varyant, birimsiz yapı gibidir.
    Connect,

    // Bir String verisi taşıyan varyant.
    Text(String),

    // Bir u64 verisi taşıyan varyant.
    Disconnect(u64),

    // Hatta birden fazla, isimlendirilmiş alan taşıyan varyant!
    Move { x: i32, y: i32 }, // Tıpkı normal bir struct!
}
```
Bu inanılmaz! Aynı `Message` türü, içerisinde birbirinden tamamen farklı veri yapılarını barındırabiliyor. Bu, geleneksel nesne yönelimli dillerdeki (Java, C#) kalıtım (inheritance) ve polimorfizme (çok biçimlilik) meydan okuyan, çok daha güvenli ve kontrollü bir soyutlama şeklidir.

Bu türü kullanmak için yine `::` operatörünü kullanırız:
```rust
let m1 = Message::Text(String::from("Merhaba!"));
let m2 = Message::Move { x: 10, y: -5 };
let m3 = Message::Disconnect(42);
```

#### 2.3. `Option` ile Boş Değerin (null) Yok Edilişi

Enum'ların en meşhur ve en hayati kullanımı, Rust'ta **null değer olmaması** problemini çözmektir. Rust'ta `null` yoktur. Bunun yerine, standart kütüphanede (ön yükleme ile gelen, `use` etmeye gerek olmayan) `Option<T>` adında bir enum vardır.

```rust
// Option<T> enum'ının tanımı aynen şöyledir:
enum Option<T> {
    None,     // Değerin yokluğunu belirtir.
    Some(T),  // Bir T değerinin varlığını belirtir.
}
```
Bu, "bir değer olabilir de olmayabilir de" durumunu tür sisteminin içine işlemiştir. Eğer bir fonksiyon `Option<String>` döndürüyorsa, o fonksiyonu kullanan kişi **her iki durumu da** (`Some` ve `None`) ele almak zorundadır. Derleyici, "yoksa bir değer gelirse ne olacak?" sorusunu sormana gerek kalmadan, bu durumu unutmana asla izin vermez. Bu, milyarlarca dolarlık "null pointer" hatalarının önüne geçen en büyük Rust siperidir.

```rust
fn bolme(dividend: f64, divisor: f64) -> Option<f64> {
    if divisor == 0.0 {
        None // Sonuç tanımsız, değer yok.
    } else {
        Some(dividend / divisor) // Sonuç var, işte burada.
    }
}

fn main() {
    let sonuc = bolme(10.0, 2.0);
    // Pattern matching ile iki durumu da ele almak ZORUNDAYIZ.
    match sonuc {
        Some(x) => println!("Sonuç: {}", x),
        None => println!("Sıfıra bölme hatası!"),
    }
}
```

#### 2.4. `use` ile Kapsamı Kolaylaştırma

Her seferinde `Message::Text(...)` yazmak can sıkıcı olabilir. `use` anahtar kelimesi, enum varyantlarını doğrudan kapsama almana olanak tanır.

```rust
enum Color {
    Red,
    Green,
    Blue,
}

// Tüm varyantları içeri aktaralım.
use Color::*;

fn main() {
    let goz_rengi = Green; // Color::Green yazmaya gerek kalmadı!
}
```
Dikkatli kullanılmalıdır; eğer birden fazla enum'dan aynı isimli varyantı içeri aktarırsan isim çakışması yaşanabilir. Genelde kütüphane kodlarında ve `Option::Some`, `Option::None` için sıkça kullanılır.

#### 2.5. C-Tarzı Enum'lar ve Sayısal Değerler

Bazen enum varyantlarının belirli tam sayı değerlerine karşılık gelmesini isteyebiliriz (örneğin, donanım register'ları veya bir dosya formatı ile çalışırken). `as` anahtar kelimesi ile kolayca dönüşüm yapabiliriz.

```rust
// Sayısal değerler açıkça atanabilir.
enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    InternalServerError = 500,
}

// Atama yapılmazsa 0'dan başlar ve birer birer artar.
enum SmallNumbers {
    Zero, // 0
    One,  // 1
    Two,  // 2
}

fn main() {
    let durum = HttpStatus::NotFound;
    println!("Hata kodu: {}", durum as i32); // Çıktı: Hata kodu: 404
}
```

---

### 3. Kısaltmalar (`type aliases`): Karmaşık İsimlere Takma Ad Vermek

Tür isimleri bazen çok uzun ve karmaşık olabilir. Özellikle generic (jenerik) türlerle ve uzun kütüphane yollarıyla çalışırken. `type` anahtar kelimesi, mevcut bir türe yeni bir **takma ad (alias)** vermenizi sağlar. Bu, yeni bir tür **yaratmaz**; sadece aynı türe ikinci bir isim verir, ikisi tamamen eşdeğerdir.

**Ders Anlatımı:**
Diyelim ki bir sunucu uygulaması yazıyorsun ve her bağlantı için benzersiz bir oturum (session) ID'si kullanıyorsun. Bu ID `u64` türünde olsun. Kodun her yerinde `u64` demek, bu sayının ne olduğunu hemen anlamamızı zorlaştırır. Ona güzel bir isim verelim.

```rust
// `SessionId`, `u64` türü için bir takma addır.
type SessionId = u64;

fn yeni_oturum_olustur() -> SessionId {
    // Özel bir ID üretme algoritması...
    42 // Şimdilik sabit dönüyor.
}

fn oturumu_temizle(id: SessionId) {
    println!("{} ID'li oturum temizleniyor...", id);
}

fn main() {
    let oturum_no: SessionId = yeni_oturum_olustur();
    // `oturum_no` değişkeni aslında bir u64'tür.
    // Onunla tüm u64 işlemlerini yapabiliriz.
    let sonraki_id = oturum_no + 1;
    oturumu_temizle(sonraki_id);
}
```
Bu, kodun belgelenmesini ve okunabilirliğini inanılmaz derecede artırır. Bir diğer yaygın kullanım alanı da uzun generic türleri kısaltmaktır:
`type Result<T> = std::result::Result<T, std::io::Error>;`

---

### Büyük Resim ve Sonraki Adımlar

İşte bu bölümde öğrendiklerimizle, Rust'ın veri modelleme konusundaki çekirdek felsefesini anlamış oldun:

1.  **`struct`**: İlgili verileri "ve" mantığıyla bir araya getirir. Bir Kullanıcının adı **ve** e-postası **ve** yaşı vardır.
2.  **`enum`**: Bir verinin farklı durumlarını "veya" mantığıyla modeller. Bir IP adresi **ya** V4'tür **ya da** V6'dır. Bir mesaj **ya** bir metindir **ya** bir bağlantı isteğidir.
3.  **`type`**: Karmaşık tür isimlerine anlamlı ve kısa takma adlar vererek kodun okunabilirliğini artırır.

Bu üç silah, Rust'ta sağlam, hataya yer bırakmayan ve kendini ifade eden API'ler ve programlar yazmanın temelidir. Özellikle `Option` ve ileride göreceğin `Result` enum'ları sayesinde, hata yönetimi zorunlu ve çok güçlü bir hale gelir.

Şimdi sıradaki en önemli konuya geçme vaktin geldi: **Akış Kontrolü (Flow of Control)** ve özellikle bu enum'ları ve struct'ları işlemek için kullanacağımız en güçlü aracımız olan **`match` (Desen Eşleme)**. Hazırsan, Rust'ın en büyüleyici bölümlerinden birine geçelim!