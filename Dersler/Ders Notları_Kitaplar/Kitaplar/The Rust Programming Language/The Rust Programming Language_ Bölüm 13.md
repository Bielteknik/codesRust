# 🦀 Ders Notları: Bölüm 13 - Fonksiyonel Özellikler — Kapsamlı Ders

---

## 📌 Giriş: Fonksiyonel Programlama ve Rust

Rust, birçok programlama dilinden ilham almıştır ve bunlardan biri de **fonksiyonel programlama** paradigmasıdır. Bu bölümde Rust'ın fonksiyonel programlamadan aldığı iki temel özelliği öğreneceğiz:

| Özellik | Ne İşe Yarar? |
|---------|---------------|
| **Closures (Kapamalar)** | Bir değişkene atanabilen, başka fonksiyona argüman olarak geçilebilen anonim fonksiyonlar |
| **Iterators (Yineleyiciler)** | Bir dizi eleman üzerinde sırayla işlem yapmamızı sağlayan yapılar |

> 💡 **Ön Bilgi:** Rust'ta bu özellikler "yüksek seviyeli" soyutlamalar gibi görünse de, arka planda **sıfır maliyetli (zero-cost)** olarak çalışırlar. Yani performans kaybı yoktur!

---

## 📚 BÖLÜM 1: Closures (Kapamalar)

### 1.1 Closure Nedir?

Closure, Rust'ta **anonim (isimsiz) bir fonksiyondur** ve şu özel yeteneklere sahiptir:

1. ✅ Bir **değişkene** atanabilir
2. ✅ Başka bir fonksiyona **argüman** olarak geçirilebilir
3. ✅ Tanımlandığı **kapsamdaki (scope) değerleri yakalayabilir** (capture)

> 🔑 **En önemli fark:** Normal fonksiyonlar çevrelerindeki değişkenlere erişemezken, closure'lar tanımlandıkları ortamdaki değişkenleri "yakalayıp" kullanabilirler.

### 1.2 Closure Sözdizimi (Syntax)

Closure'lar dikey çubuklar `||` arasına parametreler alarak tanımlanır:

```rust
// Normal fonksiyon
fn  add_one_v1(x: u32) -> u32 { x + 1 }

// Closure — farklı yazım biçimleri
let add_one_v2 = |x: u32| -> u32 { x + 1 };  // Tam tip belirterek
let add_one_v3 = |x|             { x + 1 };  // Tipleri çıkararak
let add_one_v4 = |x|               x + 1  ;  // Tek satırlık, süslü parantezsiz
```

📌 **Kural:** Tek bir ifade varsa süslü parantez `{}` ve `return` gerekmez.

### 1.3 Pratik Örnek: Tişört Çekilişi

Bir tişört şirketinin promosyon çekilişi senaryosunu düşünelim:

```rust
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // ┌──────────────────────────────────────────────────────┐
        // │ unwrap_or_else:                                      │
        // │  • Some(varsa) → içindeki değeri döndürür            │
        // │  • None(boşsa) → closure'ı çağırır, onun sonucunu    │
        // │    döndürür                                          │
        // └──────────────────────────────────────────────────────┘
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    // Kullanıcı kırmızı istiyor → kırmızı alır
    let giveaway1 = store.giveaway(Some(ShirtColor::Red));
    println!("Kullanıcı 1 şunu aldı: {:?}", giveaway1); // Red

    // Kullanıcının tercihi yok → en çok stoktanan verilir
    let giveaway2 = store.giveaway(None);
    println!("Kullanıcı 2 şunu aldı: {:?}", giveaway2); // Blue (çünkü 2 mavi, 1 kırmızı)
}
```

🔍 **Burada ne oldu?**
- `|| self.most_stocked()` ifadesi bir closure'dır
- Bu closure, `self` referansını **çevresinden yakalar** (capture eder)
- Standart kütüphanedeki `unwrap_or_else` fonksiyonu `Inventory` veya `ShirtColor` tiplerini bilmez; sadece bir closure alır ve gerektiğinde çağırır

### 1.4 Tip Çıkarımı (Type Inference)

Closure'larda tip belirtmek **zorunlu değildir**, çünkü derleyici bunu çıkarabilir:

```rust
fn main() {
    let example_closure = |x| x;

    let s = example_closure(String::from("merhaba")); // İlk çağrı: x = String
    // let n = example_closure(5);                    // ❌ HATA! x artık String'e kilitlendi
}
```

> ⚠️ **Önemli:** Closure ilk çağrıldığında hangi tipi görürse, o tipe **bir kez bağlanır** (monomorphization). Sonrasında farklı tipte çağrılamaz.

### 1.5 Çevreden Değer Yakalama (Capturing)

Closure'lar çevrelerindeki değerleri **3 farklı şekilde** yakalayabilir:

#### a) Değişmez (Immutable) Borç Alma

```rust
fn main() {
    let list = vec![1, 2, 3];
    println!("Closure öncesi: {:?}", list);

    let only_borrows = || println!("Closure içinden: {:?}", list);
    //                └─── list'i değişmez referansla yakalar

    println!("Closure çağrısı öncesi: {:?}", list); // ✅ Hâlâ kullanabiliriz
    only_borrows();
    println!("Closure çağrısı sonrası: {:?}", list); // ✅ Hâlâ kullanabiliriz
}
```

#### b) Değişebilir (Mutable) Borç Alma

```rust
fn main() {
    let mut list = vec![1, 2, 3];
    println!("Öncesi: {:?}", list);

    let mut borrows_mutably = || list.push(7);
    //                       └─── list'i değişebilir referansla yakalar

    borrows_mutably();
    println!("Sonrası: {:?}", list); // [1, 2, 3, 7]
}
```

#### c) Mülkiyet Alma (Ownership) — `move` Anahtar Kelimesi

```rust
use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Öncesi: {:?}", list);

    // move → list'in mülkiyetini closure'a taşır
    thread::spawn(move || println!("Thread içinden: {:?}", list))
        .join()
        .unwrap();
}
```

> 🎯 **`move` ne zaman kullanılır?** Genellikle yeni bir thread'e closure gönderirken, verinin o thread'e ait olmasını sağlamak için kullanılır.

### 1.6 Fn Trait Ailesi: FnOnce, FnMut, Fn

Closure'lar, çevrelerindeki değerleri nasıl ele aldıklarına göre otomatik olarak şu trait'leri implemente ederler:

```
┌─────────────────────────────────────────────────────────────────────┐
│                    CLOSURE TRAIT HİYERARŞİSİ                        │
│                                                                     │
│   FnOnce ───── Tüm closure'lar en az bunu implement eder           │
│     │           (Bir kez çağrılabilir)                              │
│     │                                                               │
│     ▼                                                               │
│   FnMut ────── Yakalanan değerleri değiştirebilir                  │
│     │           (Birden fazla çağrılabilir)                        │
│     │                                                               │
│     ▼                                                               │
│   Fn ───────── Yakalanan değerleri ne değiştirir ne taşır          │
│                 (Birden fazla güvenle çağrılabilir)                │
└─────────────────────────────────────────────────────────────────────┘
```

#### Detaylı Açıklama:

| Trait | Ne Zaman? | Açıklama |
|-------|-----------|----------|
| **`FnOnce`** | Closure, yakaladığı değeri **mülküne alırsa** (move) | Sadece **bir kez** çağrılabilir. Çünkü değer artık taşınmıştır. |
| **`FnMut`** | Closure, yakaladığı değeri **değiştiriyorsa** (mutate) | Birden fazla çağrılabilir ama her seferinde ortamı değiştirir. |
| **`Fn`** | Closure, yakaladığı değeri **değiştirmiyor ve taşımıyorsa** | Birden fazla güvenle çağrılabilir. En kısıtlı ama en güvenli olan. |

#### `unwrap_or_else` Örneği — `FnOnce` Kullanımı:

```rust
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T  // ← Closure sadece bir kez çağrılabilir
    {
        match self {
            Some(x) => x,   // Some varsa closure çağrılmaz
            None => f(),    // None ise closure bir kez çağrılır
        }
    }
}
```

#### `sort_by_key` Örneği — `FnMut` Kullanımı:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    // sort_by_key, closure'ı her eleman için çağırır → FnMut gerekir
    list.sort_by_key(|r| r.width);
    println!("{list:#?}");
}
```

#### ❌ Hatalı Kullanım — `FnOnce` ile `FnMut` Beklenen Yerde:

```rust
fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut sort_operations = vec![];
    let value = String::from("closure çağrıldı");

    list.sort_by_key(|r| {
        sort_operations.push(value); // ❌ value taşınıyor → FnOnce olur!
        r.width
    });
}
```

**Hata:** `sort_by_key`, `FnMut` bekler ama closure `FnOnce` implement ediyor çünkü `value` her çağrıda taşınıyor.

#### ✅ Doğru Kullanım:

```rust
fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1; // ✅ Sadece mutable referans → FnMut
        r.width
    });
    println!("{list:#?}, {num_sort_operations} işlemde sıralandı");
}
```

---

## 📚 BÖLÜM 2: Iterators (Yineleyiciler)

### 2.1 Iterator Nedir?

Iterator, bir **dizi eleman üzerinde sırayla dolaşma** işlemini yöneten yapıdır. Rust'ta iterator'lar:

- **Tembeldir (Lazy):** Tüketici bir metod çağırılana kadar hiçbir şey yapmazlar
- `Iterator` trait'ini implement ederler
- Her elemanı birer birer `Option` olarak döndürürler

### 2.2 Iterator Trait'i

```rust
pub trait Iterator {
    type Item;  // İlişkili tip (associated type)

    fn next(&mut self) -> Option<Self::Item>;
    //           │
    //           ├─ Some(item) → bir sonraki eleman
    //           └─ None       → dizi bitti
}
```

#### Manuel Iterator Kullanımı:

```rust
fn main() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();  // Iterator oluştur

    assert_eq!(v1_iter.next(), Some(&1));  // İlk eleman
    assert_eq!(v1_iter.next(), Some(&2));  // İkinci eleman
    assert_eq!(v1_iter.next(), Some(&3));  // Üçüncü eleman
    assert_eq!(v1_iter.next(), None);      // Dizi bitti!
}
```

### 2.3 Iterator Metodları: İki Ana Kategori

```
┌─────────────────────────────────────────────────────────────┐
│              ITERATOR METODLARI                              │
│                                                             │
│  1. Tüketen Adaptörler (Consuming Adapters)                 │
│     ─── Iterator'ı tüketir, sonuç üretir                    │
│     Örnekler: sum(), count(), last(), collect()             │
│                                                             │
│  2. Yineleyici Adaptörleri (Iterator Adapters)              │
│     ─── Iterator'ı tüketmez, yeni bir iterator döndürür     │
│     Örnekler: map(), filter(), take(), skip(), zip()        │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

#### a) Tüketen Adaptörler (Consuming Adapters)

```rust
fn main() {
    let v1 = vec![1, 2, 3, 4, 5];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();
    // sum() iterator'ı tüketir, toplamı döndürür

    println!("Toplam: {}", total); // 15

    // v1_iter artık kullanılamaz çünkü tüketildi!
    // let again = v1_iter.sum(); // ❌ HATA!
}
```

#### b) Yineleyici Adaptörleri (Iterator Adapters)

```rust
fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    // map() her elemana bir işlem uygular, yeni iterator döndürür
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    //                                   └───────┘
    //                                   collect() ile sonuca dönüştür!

    println!("{:?}", v2); // [2, 3, 4]
}
```

> ⚠️ **Dikkat:** Iterator adaptörleri **tembel (lazy)** olduğu için `collect()` gibi tüketen bir metod çağırmazsanız hiçbir şey olmaz! Derleyici uyarı verir.

#### `filter` Metodu:

```rust
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)  // Sadece belirtilen bedendekileri tut
        .collect()                         // Vec'e topla
}

fn main() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_size(shoes, 10);
    println!("{:?}", in_my_size);
    // [Shoe { size: 10, style: "sneaker" }, Shoe { size: 10, style: "boot" }]
}
```

### 2.4 Kendi Iterator'ümüzü Yazma

`Iterator` trait'ini implement ederek kendi iterator'ümüzü oluşturabiliriz:

```rust
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let counter = Counter::new(5);

    // Tüm iterator adaptörlerini kullanabiliriz!
    let sum: u32 = counter
        .filter(|x| x % 2 == 0)  // Sadece çift sayılar
        .map(|x| x * x)          // Karelerini al
        .sum();                  // Topla

    println!("Sonuç: {}", sum); // 2² + 4² = 4 + 16 = 20
}
```

### 2.5 Yaygın Iterator Metodları Tablosu

| Metod | Kategori | Açıklama |
|-------|----------|----------|
| `next()` | Temel | Sonraki elemanı döndürür |
| `collect()` | Tüketen | Iterator'ı bir koleksiyona dönüştürür |
| `sum()` | Tüketen | Tüm elemanları toplar |
| `count()` | Tüketen | Eleman sayısını döndürür |
| `map(closure)` | Adaptör | Her elemana closure uygular |
| `filter(closure)` | Adaptör | Closure'u sağlayanları tutar |
| `fold(init, closure)` | Adaptör | Birikimli işlem (reduce) |
| `enumerate()` | Adaptör | (index, value) çiftleri üretir |
| `zip(other)` | Adaptör | İki iterator'ı birleştirir |
| `take(n)` | Adaptör | İlk n elemanı alır |
| `skip(n)` | Adaptör | İlk n elemanı atlar |
| `chain(other)` | Adaptör | İki iterator'ı arka arkaya ekler |
| `flat_map(closure)` | Adaptör | Her elemanı iterator'a çevirip düzleştirir |
| `any(closure)` | Tüketen | En az bir eleman sağlıyorsa `true` |
| `all(closure)` | Tüketen | Tüm elemanlar sağlıyorsa `true` |
| `find(closure)` | Tüketen | Koşulu sağlayan ilk elemanı bulur |
| `position(closure)` | Tüketen | Koşulu sağlayan ilk elemanın indeksini bulur |

---

## 📚 BÖLÜM 3: Pratik Uygulama — I/O Projesini Geliştirme

Bölüm 12'deki `minigrep` projemizi iterator'larla nasıl daha temiz hale getirebileceğimizi görelim.

### 3.1 `Config::build` Fonksiyonunu İyileştirme

**Önceki (Verimsiz) Versiyon:**

```rust
impl Config {
    // ❌ clone() kullanılıyor, gereksiz bellek kopyalaması
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Yeterli argüman yok");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}
```

**Yeni (Iterator Tabanlı) Versiyon:**

```rust
impl Config {
    // ✅ Iterator alıyor, clone yok, mülkiyet transferi var
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // Program adını atla

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Arama sorgusu belirtilmedi"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Dosya yolu belirtilmedi"),
        };

        Ok(Config { query, file_path })
    }
}
```

**`main` Fonksiyonu:**

```rust
use std::env;
use std::process;

fn main() {
    // env::args() bir iterator döndürür!
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Yapılandırma sorunu: {err}");
        process::exit(1);
    });

    // ...
}
```

🔍 **Kazanımlar:**
- `clone()` çağrıları ortadan kalktı → daha az bellek kullanımı
- Kod daha okunabilir: `next()` ile sırayla eleman alıyoruz
- Hata mesajları daha detaylı

### 3.2 `search` Fonksiyonunu İyileştirme

**Önceki Versiyon (for döngüsü ile):**

```rust
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
```

**Yeni Versiyon (Iterator ile):**

```rust
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
```

🎯 **Farklar:**

| Eski (for) | Yeni (iterator) |
|-------------|-----------------|
| `mut results` değişkeni var | Mutable durum yok |
| Döngü mantığı kodda gizli | Amaç açık: "satırları filtrele" |
| 7 satır | 4 satır |
| Paralelleştirme zor | Paralelleştirmeye uygun |

### 3.3 Daha Da İleri: Iterator Döndürme

```rust
// collect() kaldırıldı, return tipi iterator
fn search<'a>(query: &str, contents: &'a str) -> impl Iterator<Item = &'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
}
```

> 💡 **Avantaj:** Sonuçlar hazırda bekletilmez, her eşleşen satır bulundukça işlenir. Bu da **tembel değerlendirme (lazy evaluation)** sayesinde büyük dosyalarda büyük performans kazandırır.

---

## 📚 BÖLÜM 4: Performans — Sıfır Maliyetli Soyutlamalar

### 4.1 Temel Soru: Döngü mü, Iterator mü?

Sezgilerimiz "düşük seviyeli for döngüsü daha hızlıdır" der. Ama Rust'ta bu doğru değil!

### 4.2 Benchmark Sonuçları

Sherlock Holmes romanı üzerinde yapılan arama testi:

```
test bench_search_for   ... bench: 19,620,300 ns/iter (+/- 915,700)
test bench_search_iter  ... bench: 19,234,900 ns/iter (+/- 657,200)
```

📊 **Sonuç:** İki implementasyon **neredeyse aynı performansta!** Hatta iterator versiyonu biraz daha hızlı!

### 4.3 Sıfır Maliyetli Soyutlama (Zero-Cost Abstraction) Nedir?

> *"Kullanmadığın şey için ödemezsin. Ve kullandığın şey için, elle yazabileceğinden daha iyisini yazamazsın."*
> — Bjarne Stroustrup (C++'ın tasarımcısı)

Rust'ta iterator'lar **sıfır maliyetli soyutlamadır**:

```
┌─────────────────────────────────────────────────────────────┐
│  Yüksek Seviyeli Kod (Iterator)                             │
│  contents.lines().filter(|l| l.contains(q)).collect()       │
│                         │                                   │
│                         ▼                                   │
│  Rust Derleyicisi (Monomorphization + Optimizations)        │
│                         │                                   │
│                         ▼                                   │
│  Düşük Seviyeli Assembly Kodu                               │
│  • Döngü açma (loop unrolling)                              │
│  • Sınır kontrolü kaldırma (bounds check elimination)       │
│  • Inline etme                                              │
│  • Vektörizasyon                                            │
│                         │                                   │
│                         ▼                                   │
│  Elle yazılmış for döngüsüyle AYNI assembly!                │
└─────────────────────────────────────────────────────────────┘
```

### 4.4 Neden Bu Kadar Hızlı?

Rust derleyicisi **monomorphization** sayesinde:

1. **Generic kodu somutlaştırır:** Her iterator zinciri, kullanılan tiplere özel koda dönüştürülür
2. **Inline eder:** Closure çağrıları doğrudan koda gömülür, fonksiyon çağrı maliyeti olmaz
3. **Döngü açma (loop unrolling):** Döngüleri açarak pipeline verimliliğini artırır
4. **Sınır kontrollerini kaldırır:** Derleyici, indeks kontrollerinin gereksiz olduğunu kanıtlarsa kaldırır

### 4.5 Altın Kural

> ✅ **Iterator ve closure kullanmaktan korkmayın!**
> Yüksek seviyeli görünseler de çalışma zamanında **hiçbir ek maliyet** oluşturmazlar.
> Hem okunabilir hem de hızlı kod yazabilirsiniz.

---

## 📝 Bölüm Özeti

```
┌──────────────────────────────────────────────────────────────────────┐
│                    BÖLÜM 13 — GENEL ÖZET                            │
├──────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  CLOSURES (Kapamalar)                                                │
│  ├── Anonim fonksiyonlar: || body                                    │
│  ├── Çevreden değer yakalayabilirler                                 │
│  ├── 3 yakalama yöntemi:                                             │
│  │   ├── FnOnce → Mülkiyet alır (1 kez çağrılır)                    │
│  │   ├── FnMut  → Değiştirir (çok kez çağrılır)                     │
│  │   └── Fn     → Sadece okur (çok kez güvenle çağrılır)            │
│  └── move keyword → Mülkiyeti zorla taşır                           │
│                                                                      │
│  ITERATORS (Yineleyiciler)                                           │
│  ├── Iterator trait: next() -> Option<Item>                         │
│  ├── Tembeldir (lazy) → tüketilene kadar çalışmaz                   │
│  ├── Tüketen adaptörler: sum(), collect(), count()                  │
│  ├── Iterator adaptörleri: map(), filter(), zip()                   │
│  └── Kendi iterator'ümüzü yazabiliriz                               │
│                                                                      │
│  PERFORMANS                                                          │
│  ├── Sıfır maliyetli soyutlama (zero-cost abstraction)              │
│  ├── Iterator'lar, for döngüsüyle aynı assembly'ye derlenir         │
│  └── Okunabilirlikten ödün vermeden performans!                     │
│                                                                      │
└──────────────────────────────────────────────────────────────────────┘
```

---

## 🎓 Öğrenme Kontrol Listesi

Bu bölümü gerçekten anladığınızı test etmek için şu soruları yanıtlayabilmeniz gerekiyor:

- [ ] Closure ile normal fonksiyon arasındaki farkları sayabilir misiniz?
- [ ] `Fn`, `FnMut`, `FnOnce` trait'lerinin ne zaman kullanıldığını açıklayabilir misiniz?
- [ ] `move` anahtar kelimesinin ne işe yaradığını ve ne zaman gerektiğini biliyor musunuz?
- [ ] Iterator'ın "tembel" (lazy) olması ne anlama geliyor?
- [ ] Tüketen adaptör ile iterator adaptörü arasındaki farkı biliyor musunuz?
- [ ] `map()`, `filter()`, `collect()` metodlarını kullanabiliyor musunuz?
- [ ] `Iterator` trait'ini implement ederek kendi iterator'ünüzü yazabilir misiniz?
- [ ] Rust'ta "sıfır maliyetli soyutlama" ne demek, açıklayabilir misiniz?

---

Bu bölüm, Rust'ta **idiomatik (doğal, deyim yerinde)** kod yazmanın temel taşlarından biridir. Closure ve iterator'ları iyi öğrenmek, Rust ekosistemindeki kütüphanelerin kodlarını okuyabilmeniz ve kendi projelerinizde temiz, performanslı kod yazabilmeniz için kritik öneme sahiptir. 🚀

Bir sonraki bölümde (Bölüm 14) **Cargo ve crates.io** ile Rust'ın paket yönetim sistemini öğreneceğiz!