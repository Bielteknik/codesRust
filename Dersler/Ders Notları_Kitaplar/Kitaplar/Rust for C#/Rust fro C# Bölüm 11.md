# 🔗 Rust for .NET Developers: LINQ ve Iterator Sistemi (Sorgulama ve Dönüştürme)

Bu bölüm, C#'ın güçlü **LINQ (Language Integrated Query)** yapısının Rust'taki karşılığını derinlemesine inceler. C# dünyasında `Where`, `Select`, `GroupBy` gibi metodlar ve SQL-benzeri query syntax ile veri manipülasyonuna alışkınız. Rust ise **Iterator trait** tabanlı, **zero-cost abstraction** prensibiyle çalışan çok daha farklı bir felsefe sunar.

> 🎯 **Temel Fark:** C#'ta LINQ, `IEnumerable<T>` üzerinde çalışır ve **runtime overhead** (delegate invocation, virtual dispatch, boxing) içerir. Rust'ta Iterator sistemi **compile-time**'da tamamen elle yazılmış `for` döngüsüne optimize edilir - sıfır maliyet!

---

# 📚 BÖLÜM 1: Iterator Trait'inin Anatomisi ⭐

Rust'taki iterator sisteminin kalbinde tek bir trait vardır:

## 1.1 Iterator Trait Kaynak Kodu

```rust
// Rust standart kütüphanesinden (basitleştirilmiş)
pub trait Iterator {
    type Item;  // Associated type - her iterator'ın ürettiği değer tipi
    
    // Tek zorunlu metod: bir sonraki elemanı döndür
    fn next(&mut self) -> Option<Self::Item>;
    
    // Tüm diğer metodlar (map, filter, sum, vs.) default implementasyonla gelir
    fn map<B, F>(self, f: F) -> Map<Self, F> 
    where 
        F: FnMut(Self::Item) -> B 
    { /* ... */ }
    
    fn filter<P>(self, predicate: P) -> Filter<Self, P> 
    where 
        P: FnMut(&Self::Item) -> bool 
    { /* ... */ }
    
    // ... yüzlerce metod
}
```

> 💡 **Kritik Nokta:** Bir iterator yapmak için **sadece `next()` metodunu implemente etmeniz** yeterlidir. Diğer tüm LINQ-benzeri metodlar (map, filter, fold, vs.) hazır gelir!

## 1.2 C# IEnumerable vs Rust Iterator

| Özellik | C# `IEnumerable<T>` | Rust `Iterator` |
|---|---|---|
| Temel metod | `GetEnumerator()` → `IEnumerator<T>` | `next(&mut self) -> Option<Item>` |
| Sonraki eleman | `MoveNext()` + `Current` | `next()` → `Some(T)` veya `None` |
| Döngü sonu | `MoveNext()` false döner | `next()` `None` döner |
| State management | Class-based (heap allocation) | Enum/struct-based (stack) |
| Zero-cost | ❌ (delegate + virtual call) | ✅ (compile-time inline) |
| Type safety | Runtime | Compile-time |

---

# 📚 BÖLÜM 2: Temel İterasyon - foreach vs for

## 2.1 C# Yaklaşımı

```csharp
var values = new[] { 1, 2, 3, 4, 5 };
var output = new StringBuilder();

foreach (var value in values)
{
    if (output.Length > 0) output.Append(", ");
    output.Append(value);
}

Console.Write(output); // Çıktı: 1, 2, 3, 4, 5
```

## 2.2 Rust Yaklaşımı

```rust
let values = [1, 2, 3, 4, 5];
let mut output = String::new();

for value in values {
    if !output.is_empty() {
        output.push_str(", ");
    }
    use std::fmt::Write;
    write!(output, "{value}").unwrap();
}

println!("{output}"); // Çıktı: 1, 2, 3, 4, 5
```

## 2.3 Arka Planda Ne Oluyor?

Rust derleyicisi `for` döngüsünü şu şekilde açar (desugaring):

```rust
// Yazdığınız kod:
for value in collection {
    // ...
}

// Derleyicinin gördüğü:
let mut iter = IntoIterator::into_iter(collection);
loop {
    match iter.next() {
        Some(value) => {
            // ...
        }
        None => break,
    }
}
```

> 🎯 **Önemli:** `for` döngüsü, koleksiyon üzerinde `IntoIterator::into_iter()` çağırır. Bu metod, koleksiyonun türüne göre farklı iterator'lar döndürür.

---

# 📚 BÖLÜM 3: Ownership ve İterasyon ⭐⭐ (EN KRİTİK BÖLÜM)

Bu bölüm, C# geliştiricilerinin Rust'ta **en çok takıldığı** konudur.

## 3.1 Sorun: Ownership Tüketimi

**C#** (Sorunsuz):
```csharp
var values = new List<int> { 1, 2, 3, 4, 5 };

foreach (var value in values) { sum += value; }
foreach (var value in values) { max = Math.Max(max, value); }
// ✅ İki döngü de çalışır
```

**Rust** (Derlenmez):
```rust
let values = vec![1, 2, 3, 4, 5];

for value in values { sum += value; }
for value in values { /* ... */ }  // ❌ HATA: use of moved value
```

## 3.2 Üç Farklı İterasyon Modu

Rust'ta **3 farklı** iterasyon yöntemi vardır ve her birinin farklı ownership semantiği vardır:

```rust
let values = vec![1, 2, 3];

// 1. into_iter() - Ownership alır, değerleri tüketir
for value in values.into_iter() {
    // value: i32 (değerin kendisi, sahiplik alındı)
    println!("{}", value);
}
// values artık kullanılamaz!

// 2. iter() - Immutable referans (&T)
for value in values.iter() {  // veya: for value in &values
    // value: &i32 (sadece okunabilir referans)
    println!("{}", value);
}
// values hala kullanılabilir

// 3. iter_mut() - Mutable referans (&mut T)
let mut values = vec![1, 2, 3];
for value in values.iter_mut() {  // veya: for value in &mut values
    // value: &mut i32 (değiştirilebilir referans)
    *value *= 10;
}
// values hala kullanılabilir, değerler değiştirildi: [10, 20, 30]
```

## 3.3 IntoIterator Trait

`for` döngüsü aslında `IntoIterator` trait'ini kullanır:

```rust
pub trait IntoIterator 
where
    <Self::IntoIter as Iterator>::Item == Self::Item,
{
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;
    
    fn into_iter(self) -> Self::IntoIter;
}
```

**Vec<T> için 3 farklı implementasyon:**

```rust
// 1. Vec<T> için - ownership taşır
impl<T> IntoIterator for Vec<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    // ...
}

// 2. &Vec<T> için - immutable referans
impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    // ...
}

// 3. &mut Vec<T> için - mutable referans
impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;
    // ...
}
```

> 💡 **Pratik Kural:** 
> - `for x in collection` → `into_iter()` çağrılır (ownership taşınır)
> - `for x in &collection` → `iter()` çağrılır (ödünç alma)
> - `for x in &mut collection` → `iter_mut()` çağrılır (mutable ödünç alma)

## 3.4 C# vs Rust Karşılaştırma Tablosu

| Senaryo | C# | Rust |
|---|---|---|
| Koleksiyon üzerinde döngü | `foreach (var x in list)` | `for x in &list` |
| Koleksiyon hala kullanılsın | ✅ Her zaman | ✅ `&list` kullan |
| Değerleri değiştir | ❌ foreach'te yapılamaz | ✅ `&mut list` kullan |
| Koleksiyon tüketilsin | ❌ | ✅ `list.into_iter()` |
| Heap allocation | IEnumerator için var | Sıfır (stack-based) |
| GC baskısı | Var | Yok |

---

# 📚 BÖLÜM 4: LINQ → Iterator Adapter Karşılıkları ⭐⭐

## 4.1 Kapsamlı Karşılaştırma Tablosu

| C# LINQ | Rust Iterator | Açıklama |
|---|---|---|
| `Where(pred)` | `filter(pred)` | Filtreleme |
| `Select(sel)` | `map(sel)` | Dönüştürme |
| `SelectMany` | `flat_map` / `flatten` | Flatten + map |
| `OrderBy` | `sort` / `sorted()` | Sıralama |
| `OrderByDescending` | `sort_by` / `sorted_by` | Ters sıralama |
| `GroupBy` | `itertools::group_by` | Gruplama |
| `Aggregate(seed, f)` | `fold(seed, f)` | Seed'li toplama |
| `Aggregate(f)` | `reduce(f)` | Seed'siz toplama |
| `Count()` | `count()` | Eleman sayısı |
| `Sum()` | `sum()` | Toplam |
| `Average()` | `sum() / count()` | Ortalama |
| `Min()` / `Max()` | `min()` / `max()` | Min/Maks |
| `MinBy` / `MaxBy` | `min_by_key` / `max_by_key` | Key'e göre |
| `First()` | `next()` / `nth(0)` | İlk eleman |
| `FirstOrDefault` | `next()` | İlk veya None |
| `Single()` | `exactly_one()` | Tek eleman |
| `SingleOrDefault` | `at_most_one()` | 0 veya 1 eleman |
| `Last()` | `last()` | Son eleman |
| `LastOrDefault` | `last()` | Son veya None |
| `Any(pred)` | `any(pred)` | Herhangi biri |
| `All(pred)` | `all(pred)` | Hepsi |
| `Contains(x)` | `any(\|&i\| i == x)` | İçeriyor mu? |
| `Skip(n)` | `skip(n)` | İlk n'i atla |
| `Take(n)` | `take(n)` | İlk n'i al |
| `SkipWhile` | `skip_while` | Koşul sağlanana kadar atla |
| `TakeWhile` | `take_while` | Koşul sağlanana kadar al |
| `Concat` | `chain` | Birleştir |
| `Zip` | `zip` | İki sekansı birleştir |
| `Reverse` | `rev` | Tersine çevir |
| `Distinct` | `unique()` (itertools) | Benzersiz |
| `ElementAt(n)` | `nth(n)` | n'inci eleman |
| `ToArray/ToList` | `collect::<Vec<_>>()` | Koleksiyona çevir |
| `ToDictionary` | `collect::<HashMap<_,_>>()` | Dictionary'e çevir |
| `SequenceEqual` | `eq` | Eşitlik kontrolü |
| `Append(x)` | `chain(std::iter::once(x))` | Sona ekle |
| `Prepend(x)` | `std::iter::once(x).chain(...)` | Başa ekle |
| `OfType<T>()` | `filter_map(\|x\| x.downcast_ref().copied())` | Tip filtreleme |
| `Cast<T>()` | `map(\|x\| x as T)` | Tip dönüşümü |
| `Chunk(n)` (.NET 6+) | `chunks(n)` (itertools) | Parçalara böl |
| `Except` | `filter` + HashSet | Fark kümesi |
| `Intersect` | `filter` + HashSet | Kesişim |
| `Union` | `chain` + `unique()` | Birleşim |

## 4.2 Detaylı Örnekler

### 4.2.1 Where → filter

**C#**:
```csharp
var ciftSayilar = Enumerable.Range(1, 10)
    .Where(x => x % 2 == 0)
    .ToList();
```

**Rust**:
```rust
let cift_sayilar: Vec<i32> = (1..=10)
    .filter(|x| x % 2 == 0)
    .collect();
```

> 💡 **Dikkat:** Closure parametresi referans alır (`|x|` değil `|&x|` veya `|x|` ile referans üzerinden erişim).

### 4.2.2 Select → map

**C#**:
```csharp
var kareler = new[] { 1, 2, 3, 4, 5 }
    .Select(x => x * x)
    .ToList();
```

**Rust**:
```rust
let kareler: Vec<i32> = [1, 2, 3, 4, 5]
    .iter()
    .map(|&x| x * x)  // &x ile referansı aç
    .collect();
```

### 4.2.3 Aggregate → fold / reduce

**C#**:
```csharp
var toplam = new[] { 1, 2, 3, 4, 5 }
    .Aggregate(0, (acc, x) => acc + x);  // Seed ile
```

**Rust**:
```rust
// Seed ile → fold
let toplam: i32 = [1, 2, 3, 4, 5]
    .iter()
    .fold(0, |acc, &x| acc + x);

// Seed'siz → reduce
let toplam2: Option<i32> = [1, 2, 3, 4, 5]
    .iter()
    .copied()
    .reduce(|acc, x| acc + x);
// Sonuç: Some(15)
```

> 💡 **Fark:** Rust'ta `reduce` boş koleksiyon için `None` döner (C#'ta exception fırlatır).

### 4.2.4 SelectMany → flat_map / flatten

**C#**:
```csharp
var sonuc = new[] { "merhaba", "dünya" }
    .SelectMany(s => s.Split(' '))
    .ToList();
```

**Rust**:
```rust
let sonuc: Vec<&str> = ["merhaba dünya", "rust programlama"]
    .iter()
    .flat_map(|s| s.split_whitespace())
    .collect();
// Sonuç: ["merhaba", "dünya", "rust", "programlama"]
```

### 4.2.5 GroupBy → itertools::group_by

**C#**:
```csharp
var gruplar = kisiler.GroupBy(k => k.Yas);
```

**Rust** (itertools gerekir):
```rust
use itertools::Itertools;

// ÖNEMLİ: Rust'ta group_by sadece ardışık elemanları gruplar!
// Önce sıralamak gerekir.
for (yas, grup) in &kisiler.iter()
    .sorted_by_key(|k| k.yas)
    .group_by(|k| k.yas) 
{
    let adlar: Vec<_> = grup.map(|k| &k.ad).collect();
    println!("Yaş {}: {:?}", yas, adlar);
}
```

> ⚠️ **Kritik Fark:** C# `GroupBy` tüm koleksiyonu gruplar. Rust `group_by` sadece **ardışık (consecutive)** elemanları gruplar. Bu yüzden önce sıralamak gerekir!

### 4.2.6 Zip

**C#**:
```csharp
var birlesik = isimler.Zip(yaslar, (isim, yas) => $"{isim}: {yas}");
```

**Rust**:
```rust
for (isim, yas) in isimler.iter().zip(yaslar.iter()) {
    println!("{}: {}", isim, yas);
}

// Veya map ile birleştir
let sonuc: Vec<String> = isimler.iter()
    .zip(yaslar.iter())
    .map(|(isim, yas)| format!("{}: {}", isim, yas))
    .collect();
```

### 4.2.7 Partition (C#'ta yok, Rust'ta var!)

```rust
let (gecenler, kalanlar): (Vec<_>, Vec<_>) = ogrenciler
    .into_iter()
    .partition(|o| o.not >= 50);
```

### 4.2.8 Scan (C#'ta yok, Rust'ta fold ile yapılır)

```rust
// Kümülatif toplam
let sayilar = vec![1, 2, 3, 4, 5];
let mut kumulatif = Vec::new();
sayilar.iter().fold(0, |acc, &x| {
    let yeni = acc + x;
    kumulatif.push(yeni);
    yeni
});
// kumulatif: [1, 3, 6, 10, 15]

// itertools ile scan
use itertools::Itertools;
let kumulatif: Vec<i32> = sayilar.iter()
    .copied()
    .scan(None, |state, x| {
        *state = Some(state.unwrap_or(0) + x);
        *state
    })
    .collect();
```

---

# 📚 BÖLÜM 5: Lazy Evaluation (Tembel Değerlendirme) ⭐

Hem C# LINQ hem Rust Iterator **tembeldir (lazy)** - son elemana ihtiyaç duyulana kadar **hiçbir işlem yapılmaz**.

## 5.1 Çalışma Prensibi

```rust
let sayilar = 1..=5;

// Hiçbir işlem yapılmadı henüz!
let sorgu = sayilar
    .filter(|x| { println!("Filter: {}", x); *x > 2 })
    .map(|x| { println!("Map: {}", x); x * 2 });

println!("--- Sorgu tanımlandı, henüz çalışmadı ---");

// Şimdi çalışır
for x in sorgu {
    println!("Sonuç: {}", x);
}

/* Çıktı:
--- Sorgu tanımlandı, henüz çalışmadı ---
Filter: 1
Filter: 2
Filter: 3
Map: 3
Sonuç: 6
Filter: 4
Map: 4
Sonuç: 8
Filter: 5
Map: 5
Sonuç: 10
*/
```

## 5.2 Terminal Operations (İşlemi Tetikleyen Metodlar)

Bir iterator sadece **terminal operation** çağrıldığında çalışır:

```rust
// Terminal operations (iterator'ı tüketir):
let _: Vec<_> = iter.collect();     // Koleksiyona çevir
let _: i32 = iter.sum();            // Topla
let _: usize = iter.count();        // Say
let _: Option<_> = iter.next();     // Sonraki eleman
let _: Option<_> = iter.max();      // Maksimum
let _: bool = iter.any(|x| ...);    // Herhangi biri
let _: bool = iter.all(|x| ...);    // Hepsi
iter.for_each(|x| ...);             // Her eleman için çalıştır
let _: Option<_> = iter.find(|x| ...); // Bul
let _: Option<_> = iter.position(|x| ...); // Pozisyon bul
let _: Option<_> = iter.fold(...);  // Toplama
let _: Option<_> = iter.reduce(...); // Seed'siz toplama
```

## 5.3 Early Termination Avantajı

```rust
// İlk 5 çift sayıyı bul
let ilk_5_cift: Vec<i32> = (1..)  // Sonsuz dizi!
    .filter(|x| x % 2 == 0)
    .take(5)  // Sadece 5 eleman al
    .collect();
// Sonuç: [2, 4, 6, 8, 10]
// Sonsuz diziden sadece 10 sayı işlendi!
```

---

# 📚 BÖLÜM 6: `collect` ve Turbofish (`::<>`)

Rust'ta `collect`, LINQ'taki `ToList`, `ToArray`, `ToDictionary` karşılığıdır ama çok daha güçlüdür.

## 6.1 Temel Kullanım

```rust
let sayilar = vec![1, 2, 3, 4, 5];

// Vec'e dönüştür
let vec: Vec<i32> = sayilar.iter().copied().collect();

// HashSet'e
let set: HashSet<i32> = sayilar.iter().copied().collect();

// HashMap'e
let map: HashMap<i32, i32> = sayilar.iter()
    .map(|&x| (x, x * x))
    .collect();

// String'e (char'lardan)
let metin: String = vec!['R', 'u', 's', 't'].into_iter().collect();
```

## 6.2 Turbofish (`::<>`) Sözdizimi

Derleyici bazen hedef türü bilemez:

```rust
// ❌ HATA: "type annotations needed"
let sonuc = (1..=5).collect();

// ✅ Çözüm 1: Tür annotation
let sonuc: Vec<i32> = (1..=5).collect();

// ✅ Çözüm 2: Turbofish
let sonuc = (1..=5).collect::<Vec<i32>>();

// ✅ Çözüm 3: Partial inference (en yaygın)
let sonuc = (1..=5).collect::<Vec<_>>();
```

## 6.3 FromIterator Trait

`collect`, `FromIterator` trait'ini implemente eden **herhangi bir türe** çalışır:

```rust
// Result koleksiyonu - hepsi Ok ise Ok, bir Err varsa Err döner
let sonuclar = vec![Ok(1), Ok(2), Ok(3)];
let toplu_sonuc: Result<Vec<i32>, &str> = sonuclar.into_iter().collect();
// Sonuç: Ok([1, 2, 3])

let hatali = vec![Ok(1), Err("hata"), Ok(3)];
let toplu_hata: Result<Vec<i32>, &str> = hatali.into_iter().collect();
// Sonuç: Err("hata")

// Option koleksiyonu
let opsiyonlar = vec![Some(1), Some(2), Some(3)];
let toplu: Option<Vec<i32>> = opsiyonlar.into_iter().collect();
// Sonuç: Some([1, 2, 3])

// String (char'lardan)
let chars = vec!['M', 'e', 'r', 'h', 'a', 'b', 'a'];
let metin: String = chars.into_iter().collect();
```

---

# 📚 BÖLÜM 7: Custom Iterator Yazma ⭐⭐

Kendi iterator'ınızı yazmak, Rust'ın iterator sistemini anlamanın en iyi yoludur.

## 7.1 Basit Bir Range Iterator

```rust
struct Sayac {
    mevcut: i32,
    bitis: i32,
}

impl Sayac {
    fn new(baslangic: i32, bitis: i32) -> Self {
        Sayac { 
            mevcut: baslangic, 
            bitis 
        }
    }
}

impl Iterator for Sayac {
    type Item = i32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.mevcut < self.bitis {
            let deger = self.mevcut;
            self.mevcut += 1;
            Some(deger)
        } else {
            None
        }
    }
}

fn main() {
    let sayac = Sayac::new(1, 5);
    
    // Tüm iterator metodları kullanılabilir!
    let kareler: Vec<i32> = sayac.map(|x| x * x).collect();
    println!("{:?}", kareler); // [1, 4, 9, 16]
    
    let toplam: i32 = Sayac::new(1, 5).sum();
    println!("Toplam: {}", toplam); // 10
}
```

## 7.2 Fibonacci Iterator

```rust
struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;
    
    fn next(&mut self) -> Option<Self::Item> {
        let sonuc = self.a;
        let yeni = self.a + self.b;
        self.a = self.b;
        self.b = yeni;
        Some(sonuc)
    }
}

fn main() {
    let ilk_10: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("{:?}", ilk_10); // [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
    
    let ilk_cift_5: Vec<u64> = Fibonacci::new()
        .filter(|x| x % 2 == 0)
        .take(5)
        .collect();
    println!("{:?}", ilk_cift_5); // [0, 2, 8, 34, 144]
}
```

## 7.3 Generic Iterator

```rust
struct Tekrarlayan<T> {
    deger: T,
    kalan: usize,
}

impl<T: Clone> Tekrarlayan<T> {
    fn new(deger: T, kac_kez: usize) -> Self {
        Tekrarlayan { deger, kalan: kac_kez }
    }
}

impl<T: Clone> Iterator for Tekrarlayan<T> {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.kalan > 0 {
            self.kalan -= 1;
            Some(self.deger.clone())
        } else {
            None
        }
    }
}

fn main() {
    let tekrar: Vec<String> = Tekrarlayan::new("Merhaba".to_string(), 3)
        .collect();
    println!("{:?}", tekrar); // ["Merhaba", "Merhaba", "Merhaba"]
}
```

## 7.4 DoubleEndedIterator

Hem baştan hem sondan ilerleyebilen iterator:

```rust
struct CiftYonlu {
    veri: Vec<i32>,
    sol: usize,
    sag: usize,
}

impl CiftYonlu {
    fn new(veri: Vec<i32>) -> Self {
        let sag = veri.len();
        CiftYonlu { veri, sol: 0, sag }
    }
}

impl Iterator for CiftYonlu {
    type Item = i32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.sol < self.sag {
            let deger = self.veri[self.sol];
            self.sol += 1;
            Some(deger)
        } else {
            None
        }
    }
}

impl DoubleEndedIterator for CiftYonlu {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.sol < self.sag {
            self.sag -= 1;
            Some(self.veri[self.sag])
        } else {
            None
        }
    }
}

fn main() {
    let iter = CiftYonlu::new(vec![1, 2, 3, 4, 5]);
    
    // Başтан
    let ilk_uc: Vec<i32> = iter.take(3).collect();
    println!("{:?}", ilk_uc); // [1, 2, 3]
    
    // Sondan
    let mut iter = CiftYonlu::new(vec![1, 2, 3, 4, 5]);
    println!("{:?}", iter.next_back()); // Some(5)
    println!("{:?}", iter.next_back()); // Some(4)
}
```

## 7.5 ExactSizeIterator

```rust
impl ExactSizeIterator for Sayac {
    fn len(&self) -> usize {
        (self.bitis - self.mevcut) as usize
    }
}

// Artık .len() kullanılabilir
let sayac = Sayac::new(1, 10);
println!("Uzunluk: {}", sayac.len()); // 9
```

---

# 📚 BÖLÜM 8: `impl Iterator` Return Type ⭐⭐

Fonksiyonlardan iterator döndürmenin en ergonomik yolu.

## 8.1 Temel Kullanım

```rust
// ❌ Kötü: Somut tür döndürmek
fn cift_sayilar() -> std::iter::Filter<std::ops::RangeInclusive<i32>, fn(&i32) -> bool> {
    (1..=10).filter(|x| x % 2 == 0)
}

// ✅ İyi: impl Iterator
fn cift_sayilar() -> impl Iterator<Item = i32> {
    (1..=10).filter(|x| x % 2 == 0)
}

fn main() {
    let sonuc: Vec<i32> = cift_sayilar().collect();
    println!("{:?}", sonuc); // [2, 4, 6, 8, 10]
}
```

## 8.2 Karmaşık Iterator Zincirleri

```rust
fn kullanici_adi_olustur(isim: &str, yas: u8) -> impl Iterator<Item = String> {
    let isim_kucuk = isim.to_lowercase();
    (1..=3).map(move |i| format!("{}{}{}", isim_kucuk, yas, i))
}

fn main() {
    let adlar: Vec<String> = kullanici_adi_olustur("Ali", 25).collect();
    println!("{:?}", adlar); // ["ali251", "ali252", "ali253"]
}
```

## 8.3 Koşullu Iterator Döndürme

```rust
// ❌ Bu çalışmaz - farklı türler
fn filtrele(sayilar: Vec<i32>, cift: bool) -> impl Iterator<Item = i32> {
    if cift {
        sayilar.into_iter().filter(|x| x % 2 == 0)
    } else {
        sayilar.into_iter().filter(|x| x % 2 != 0)  // ❌ Farklı tür!
    }
}

// ✅ Çözüm 1: Box<dyn Iterator>
fn filtrele(sayilar: Vec<i32>, cift: bool) -> Box<dyn Iterator<Item = i32>> {
    if cift {
        Box::new(sayilar.into_iter().filter(|x| x % 2 == 0))
    } else {
        Box::new(sayilar.into_iter().filter(|x| x % 2 != 0))
    }
}

// ✅ Çözüm 2: Either pattern
use either::Either;

fn filtrele(sayilar: Vec<i32>, cift: bool) -> impl Iterator<Item = i32> {
    if cift {
        Either::Left(sayilar.into_iter().filter(|x| x % 2 == 0))
    } else {
        Either::Right(sayilar.into_iter().filter(|x| x % 2 != 0))
    }
}
```

---

# 📚 BÖLÜM 9: `itertools` Crate - LINQ'un Eksik Parçaları ⭐⭐

Standart kütüphanede olmayan birçok LINQ metodu **itertools** crate'inde bulunur.

## 9.1 Kurulum

```toml
[dependencies]
itertools = "0.12"
```

## 9.2 En Sık Kullanılan Metodlar

### 9.2.1 unique - Distinct

```rust
use itertools::Itertools;

let benzersiz: Vec<i32> = vec![1, 2, 2, 3, 3, 3].into_iter()
    .unique()
    .collect();
// Sonuç: [1, 2, 3]

// Key'e göre unique
struct Kisi { ad: String, yas: u8 }
let kisiler = vec![
    Kisi { ad: "Ali".into(), yas: 25 },
    Kisi { ad: "Veli".into(), yas: 25 },
    Kisi { ad: "Ali".into(), yas: 30 },
];

let benzersiz_isimler: Vec<_> = kisiler.into_iter()
    .unique_by(|k| k.ad.clone())
    .collect();
```

### 9.2.2 chunks - Bölümlere Ayırma

```rust
use itertools::Itertools;

let chunks: Vec<Vec<i32>> = (1..=10).chunks(3).into_iter()
    .map(|chunk| chunk.collect())
    .collect();
// Sonuç: [[1,2,3], [4,5,6], [7,8,9], [10]]
```

### 9.2.3 tuple_windows - Ardışık Pencereler

```rust
use itertools::Itertools;

let sayilar = vec![1, 2, 3, 4, 5];

// İkili pencereler
let windows: Vec<(i32, i32)> = sayilar.iter()
    .tuple_windows()
    .copied()
    .collect();
// Sonuç: [(1,2), (2,3), (3,4), (4,5)]

// Üçlü pencereler
let triples: Vec<(i32, i32, i32)> = sayilar.iter()
    .tuple_windows::<(_, _, _)>()
    .copied()
    .collect();
// Sonuç: [(1,2,3), (2,3,4), (3,4,5)]
```

### 9.2.4 combinations ve permutations

```rust
use itertools::Itertools;

// Kombinasyonlar (sıra önemli değil)
let combos: Vec<Vec<i32>> = vec![1, 2, 3, 4].into_iter()
    .combinations(2)
    .collect();
// Sonuç: [[1,2], [1,3], [1,4], [2,3], [2,4], [3,4]]

// Permütasyonlar (sıra önemli)
let perms: Vec<Vec<i32>> = vec![1, 2, 3].into_iter()
    .permutations(2)
    .collect();
// Sonuç: [[1,2], [1,3], [2,1], [2,3], [3,1], [3,2]]
```

### 9.2.5 interleave - Sıralı Birleştirme

```rust
use itertools::Itertools;

let a = vec![1, 2, 3];
let b = vec![10, 20, 30];

let inter: Vec<i32> = a.into_iter().interleave(b).collect();
// Sonuç: [1, 10, 2, 20, 3, 30]
```

### 9.2.6 kmerge - Sıralı Koleksiyonları Birleştir

```rust
use itertools::Itertools;

let a = vec![1, 3, 5];
let b = vec![2, 4, 6];
let c = vec![7, 8, 9];

let birlesik: Vec<i32> = vec![a, b, c].into_iter()
    .kmerge()
    .collect();
// Sonuç: [1, 2, 3, 4, 5, 6, 7, 8, 9] (sıralı)
```

### 9.2.7 minmax - Aynı Anda Min ve Max

```rust
use itertools::Itertools;
use itertools::MinMaxResult::{MinMax, NoElements, OneElement};

let sayilar = vec![3, 1, 4, 1, 5, 9, 2, 6];

match sayilar.iter().minmax() {
    MinMax(min, max) => println!("Min: {}, Max: {}", min, max),
    OneElement(e) => println!("Tek eleman: {}", e),
    NoElements => println!("Boş"),
}
// Çıktı: Min: 1, Max: 9
```

### 9.2.8 positions - Koşula Uyan İndeksler

```rust
use itertools::Itertools;

let sayilar = vec![1, 2, 3, 4, 5, 6];
let cift_indeksler: Vec<usize> = sayilar.iter()
    .positions(|&x| x % 2 == 0)
    .collect();
// Sonuç: [1, 3, 5] (0-based indeks)
```

### 9.2.9 group_by (Ardışık Gruplama)

```rust
use itertools::Itertools;

let veriler = vec![1, 1, 2, 2, 2, 3, 1, 1];

for (key, grup) in &veriler.into_iter().group_by(|&x| x) {
    let elemanlar: Vec<i32> = grup.collect();
    println!("{}: {:?}", key, elemanlar);
}
/* Çıktı:
1: [1, 1]
2: [2, 2, 2]
3: [3]
1: [1, 1]
*/
```

---

# 📚 BÖLÜM 10: Borrow Checker ve Iterator Etkileşimi ⭐⭐

Iterator'lar ile borrow checker arasındaki etkileşim, Rust'ın en karmaşık konularından biridir.

## 10.1 Temel Problem

```rust
let mut v = vec![1, 2, 3];

// ❌ HATA: v ödünç alınmışken değiştiremezsiniz
for x in &v {
    v.push(*x * 2);  // ❌ cannot borrow `v` as mutable
}
```

## 10.2 Çözümler

### Çözüm 1: Önce Topla, Sonra Ekle

```rust
let mut v = vec![1, 2, 3];
let eklenecekler: Vec<i32> = v.iter().map(|&x| x * 2).collect();
v.extend(eklenecekler);
// v: [1, 2, 3, 2, 4, 6]
```

### Çözüm 2: İndeks ile Döngü

```rust
let mut v = vec![1, 2, 3];
let baslangic_uzunlugu = v.len();
for i in 0..baslangic_uzunlugu {
    let yeni = v[i] * 2;
    v.push(yeni);
}
```

### Çözüm 3: drain ile

```rust
let mut v = vec![1, 2, 3];
let yeni: Vec<i32> = v.drain(..).map(|x| x * 2).collect();
v.extend(yeni);
```

## 10.3 Self-Referential Struct Problemi

```rust
// ❌ Bu çalışmaz - Rust'ta self-referential struct yasak
struct SelfRef {
    veri: String,
    referans: &str,  // veri'ye referans
}

// ✅ Çözüm: Ya indeks kullan ya da Rc/Arc
struct SelfRef<'a> {
    veri: String,
    referans: &'a str,  // Lifetime annotation gerekli
}
```

---

# 📚 BÖLÜM 11: Streaming Iterator ve Bellek Verimliliği ⭐

Büyük veri setleriyle çalışırken, tüm veriyi belleğe almak yerine **streaming** yaklaşımı kullanılır.

## 11.1 Dosya Satırlarını Streaming Olarak İşleme

```rust
use std::fs::File;
use std::io::{BufRead, BufReader};

fn log_analiz_et(dosya_yolu: &str) -> std::io::Result<()> {
    let dosya = File::open(dosya_yolu)?;
    let reader = BufReader::new(dosya);
    
    // Tüm dosyayı belleğe almadan, satır satır işle
    let (error_sayisi, warn_sayisi) = reader.lines()
        .filter_map(|line| line.ok())
        .fold((0, 0), |(e, w), line| {
            if line.contains("ERROR") {
                (e + 1, w)
            } else if line.contains("WARN") {
                (e, w + 1)
            } else {
                (e, w)
            }
        });
    
    println!("ERROR: {}, WARN: {}", error_sayisi, warn_sayisi);
    Ok(())
}
```

## 11.2 HTTP Response Streaming

```rust
async fn buyuk_dosya_indir(url: &str) -> Result<u64, reqwest::Error> {
    let response = reqwest::get(url).await?;
    
    // Tüm body'yi belleğe almadan, parça parça işle
    let mut stream = response.bytes_stream();
    let mut toplam_byte = 0u64;
    
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        toplam_byte += chunk.len() as u64;
        // Chunk'ı burada işle (örn: diske yaz)
    }
    
    Ok(toplam_byte)
}
```

## 11.3 Embedded için Streaming

```rust
// RP2354B'de sensor verisi işleme
fn ortalama_hesapla_streaming(
    mut sensor: impl Iterator<Item = f32>,
    ornek_sayisi: usize
) -> f32 {
    let (toplam, sayac) = sensor
        .take(ornek_sayisi)
        .fold((0.0f32, 0usize), |(t, s), x| (t + x, s + 1));
    
    if sayac > 0 { toplam / sayac as f32 } else { 0.0 }
}
```

---

# 📚 BÖLÜM 12: Kompleks Örnekler

## 12.1 Web API'den Veri Çekme ve İşleme

```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct Urun {
    id: u32,
    ad: String,
    fiyat: f64,
    kategori: String,
    stok: u32,
}

async fn urunleri_isle() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get("https://api.example.com/urunler").await?;
    let urunler: Vec<Urun> = response.json().await?;
    
    // 1. Stokta olan ürünleri filtrele
    let stoktakiler: Vec<_> = urunler.iter()
        .filter(|u| u.stok > 0)
        .collect();
    
    // 2. Kategori bazında grupla
    use itertools::Itertools;
    let kategori_ozet: Vec<_> = stoktakiler.iter()
        .sorted_by_key(|u| &u.kategori)
        .group_by(|u| &u.kategori)
        .into_iter()
        .map(|(kategori, grup)| {
            let urunler: Vec<_> = grup.collect();
            let toplam_fiyat: f64 = urunler.iter().map(|u| u.fiyat).sum();
            let ortalama = toplam_fiyat / urunler.len() as f64;
            (kategori.to_string(), urunler.len(), ortalama)
        })
        .collect();
    
    // 3. En pahalı 5 ürün
    let en_pahali_5: Vec<_> = stoktakiler.iter()
        .sorted_by(|a, b| b.fiyat.partial_cmp(&a.fiyat).unwrap())
        .take(5)
        .collect();
    
    Ok(())
}
```

## 12.2 SQL-benzeri Sorgular

```rust
use itertools::Itertools;

struct Siparis {
    musteri_id: u32,
    urun_id: u32,
    miktar: u32,
    birim_fiyat: f64,
}

fn musteri_harcama_raporu(siparisler: &[Siparis]) -> Vec<(u32, f64, usize)> {
    siparisler.iter()
        .group_by(|s| s.musteri_id)
        .into_iter()
        .map(|(musteri_id, grup)| {
            let siparisler: Vec<_> = grup.collect();
            let toplam_harcama: f64 = siparisler.iter()
                .map(|s| s.miktar as f64 * s.birim_fiyat)
                .sum();
            let siparis_sayisi = siparisler.len();
            (musteri_id, toplam_harcama, siparis_sayisi)
        })
        .sorted_by(|a, b| b.1.partial_cmp(&a.1).unwrap())  // Harcamaya göre sırala
        .collect()
}
```

## 12.3 Matrix İşlemleri

```rust
fn matrix_carpimi(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let satir = a.len();
    let sutun = b[0].len();
    let ortak = b.len();
    
    (0..satir).map(|i| {
        (0..sutun).map(|j| {
            (0..ortak).map(|k| a[i][k] * b[k][j]).sum()
        }).collect()
    }).collect()
}

fn matrix_transpozu(m: &[Vec<f64>]) -> Vec<Vec<f64>> {
    if m.is_empty() { return vec![]; }
    let satir = m.len();
    let sutun = m[0].len();
    
    (0..sutun).map(|j| {
        (0..satir).map(|i| m[i][j]).collect()
    }).collect()
}
```

---

# 📚 BÖLÜM 13: Performans Karşılaştırması ⭐

## 13.1 C# LINQ - Overhead Var

```csharp
// LINQ versiyonu
var toplam = sayilar.Where(x => x % 2 == 0).Select(x => x * 2).Sum();

// Manuel loop versiyonu
int toplam = 0;
foreach (var x in sayilar)
{
    if (x % 2 == 0) toplam += x * 2;
}
```

> ⚠️ **C# Durumu:** LINQ genellikle manuel loop'tan **2-10x yavaş** olabilir. JIT compiler bazı optimizasyonlar yapar ama delegate invocation overhead'i kalır.

## 13.2 Rust Iterator - Zero-Cost Abstraction

```rust
// Iterator versiyonu
let toplam: i32 = sayilar.iter()
    .filter(|x| *x % 2 == 0)
    .map(|x| x * 2)
    .sum();

// Manuel loop versiyonu
let mut toplam = 0;
for &x in &sayilar {
    if x % 2 == 0 {
        toplam += x * 2;
    }
}
```

> 🎯 **Rust Durumu:** Derleyici iterator zincirini **tam olarak** manuel loop'a optimize eder! İki versiyonun assembly çıktısı **neredeyse aynıdır**.

## 13.3 Benchmark Sonuçları

```rust
use criterion::{criterion_group, criterion_main, Criterion, black_box};

fn manuel_loop(sayilar: &[i32]) -> i32 {
    let mut toplam = 0;
    for &x in sayilar {
        if x % 2 == 0 {
            toplam += x * 2;
        }
    }
    toplam
}

fn iterator_chain(sayilar: &[i32]) -> i32 {
    sayilar.iter()
        .filter(|x| *x % 2 == 0)
        .map(|x| x * 2)
        .sum()
}

fn bench_comparison(c: &mut Criterion) {
    let data: Vec<i32> = (0..10_000).collect();
    
    c.bench_function("manuel_loop", |b| {
        b.iter(|| manuel_loop(black_box(&data)))
    });
    
    c.bench_function("iterator_chain", |b| {
        b.iter(|| iterator_chain(black_box(&data)))
    });
}

criterion_group!(benches, bench_comparison);
criterion_main!(benches);
```

**Tipik Sonuç:** İki metodun performansı **neredeyse aynıdır** (~%2-3 fark).

---

# 📚 BÖLÜM 14: Embedded Sistemlerde Iterator (RP2354B) 🎯

## 14.1 no_std Ortamında Iterator

```rust
#![no_std]

// Iterator'lar no_std'de de çalışır (core::iter)
let toplam: i32 = [1, 2, 3, 4, 5].iter().sum();
```

## 14.2 Step Motor Projesi İçin Iterator Örnekleri

```rust
// Hız profili oluşturma
fn hiz_profili_olustur(toplam_adim: u32, max_hiz: u16, ivme: u16) -> Vec<u16> {
    (0..toplam_adim)
        .map(|i| {
            let hiz = (ivme as u32 * i).min(max_hiz as u32);
            hiz as u16
        })
        .collect()
}

// Çoklu motor paralel kontrol
fn motorlari_hareket_ettir(motorlar: &mut [Motor], hedefler: &[i32]) {
    motorlar.iter_mut()
        .zip(hedefler.iter())
        .for_each(|(motor, &hedef)| {
            motor.hareket_et(hedef);
        });
}

// Sensör verisi filtreleme
fn sensor_filtrele(oku_malar: &[f32], esik: f32) -> Vec<f32> {
    oku_malar.iter()
        .filter(|&&x| x > esik)
        .copied()
        .collect()
}

// Hata tespiti
fn hata_var_mi(hatalar: &[Hata]) -> bool {
    hatalar.iter().any(|h| h.kritik)
}

// Son 10 okumanın ortalaması
fn ortalama_hesapla(oku_malar: &[f32]) -> f32 {
    let son_10: Vec<f32> = oku_malar.iter()
        .rev()
        .take(10)
        .copied()
        .collect();
    
    if son_10.is_empty() {
        0.0
    } else {
        son_10.iter().sum::<f32>() / son_10.len() as f32
    }
}
```

## 14.3 Bellek-Efficient Iterator

```rust
// ❌ Kötü: Tüm veriyi belleğe al
let buyuk_veri: Vec<u8> = sensor_oku(1000);
let ort = buyuk_veri.iter().sum::<u32>() / 1000;

// ✅ İyi: Streaming ile düşük bellek
let ort: u32 = (0..1000)
    .map(|_| sensor_oku_anlik())
    .sum::<u32>() / 1000;
```

---

# 📚 BÖLÜM 15: Common Pitfalls ve Best Practices

## 15.1 ❌ Yaygın Hatalar

### Hata 1: Iterator'ı Birden Fazla Kez Kullanmak

```rust
// ❌ YANLIŞ
let iter = vec![1, 2, 3].into_iter();
let ilk: Vec<_> = iter.take(2).collect();
let ikinci: Vec<_> = iter.collect();  // ❌ iter tüketildi!

// ✅ DOĞRU
let v = vec![1, 2, 3];
let ilk: Vec<_> = v.iter().take(2).collect();
let ikinci: Vec<_> = v.iter().skip(2).collect();
```

### Hata 2: collect Türünü Belirtmemek

```rust
// ❌ YANLIŞ
let sonuc = (1..=5).collect();  // Derlenmez

// ✅ DOĞRU
let sonuc: Vec<i32> = (1..=5).collect();
// veya
let sonuc = (1..=5).collect::<Vec<_>>();
```

### Hata 3: Lazy Iterator'ı Unutmak

```rust
// ❌ YANLIŞ - hiçbir işlem yapılmaz!
sayilar.iter().filter(|x| **x > 5).map(|x| x * 2);

// ✅ DOĞRU - terminal operation ekle
let sonuc: Vec<_> = sayilar.iter()
    .filter(|x| **x > 5)
    .map(|x| x * 2)
    .collect();
```

### Hata 4: group_by ile Sıralamayı Unutmak

```rust
// ❌ YANLIŞ - sadece ardışık elemanlar gruplanır
use itertools::Itertools;
for (key, grup) in &veriler.iter().group_by(|x| x.kategori) {
    // Eksik gruplar olabilir!
}

// ✅ DOĞRU - önce sırala
for (key, grup) in &veriler.iter()
    .sorted_by_key(|x| &x.kategori)
    .group_by(|x| x.kategori) 
{
    // Tüm elemanlar doğru gruplanır
}
```

### Hata 5: Borrow Checker ile Çakışma

```rust
// ❌ YANLIŞ
let mut v = vec![1, 2, 3];
for x in &v {
    v.push(*x * 2);  // ❌ v ödünç alınmışken değiştiremezsin
}

// ✅ DOĞRU
let mut v = vec![1, 2, 3];
let eklenecek: Vec<_> = v.iter().map(|&x| x * 2).collect();
v.extend(eklenecek);
```

## 15.2 ✅ İyi Pratikler

1. **Iterator'ları Zincirleyin:**
```rust
// ✅ İyi - okunabilir ve performanslı
let sonuc = veriler.iter()
    .filter(|x| x.aktif)
    .map(|x| &x.ad)
    .sorted()
    .collect::<Vec<_>>();
```

2. **`peekable` ile İleriye Bakın:**
```rust
let mut iter = (1..5).peekable();
while let Some(&x) = iter.peek() {
    if x < 3 {
        iter.next();
    } else {
        break;
    }
}
```

3. **`inspect` ile Debug Yapın:**
```rust
let sonuc: Vec<_> = (1..=10)
    .inspect(|x| println!("Girdi: {}", x))
    .filter(|x| x % 2 == 0)
    .inspect(|x| println!("Filtre geçti: {}", x))
    .collect();
```

4. **`fold` ile State Yönetin:**
```rust
// Birden fazla değeri aynı anda topla
let (toplam, sayim, max) = veriler.iter().fold(
    (0, 0, i32::MIN),
    |(t, s, m), &x| (t + x, s + 1, m.max(x))
);
```

5. **Büyük Veriler için `par_iter()` Kullanın (Rayon):**
```rust
use rayon::prelude::*;

let sonuc: i64 = buyuk_veri
    .par_iter()
    .filter(|x| x.aktif)
    .map(|x| x.deger)
    .sum();
```

6. **`copied()` ve `cloned()` Kullanın:**
```rust
// ❌ Kötü
let degerler: Vec<i32> = referanslar.iter().map(|&x| x).collect();

// ✅ İyi
let degerler: Vec<i32> = referanslar.iter().copied().collect();
```

7. **`flatten()` ile İç İçe Koleksiyonları Düzleştir:**
```rust
let matris = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
let duz: Vec<i32> = matris.into_iter().flatten().collect();
// Sonuç: [1, 2, 3, 4, 5, 6]
```

---

# 🎯 ÖZET: LINQ vs Iterator Kontrol Listesi

| Özellik | C# LINQ | Rust Iterator |
|---|---|---|
| Temel interface | `IEnumerable<T>` | `Iterator` trait |
| Lazy evaluation | ✅ Deferred execution | ✅ Zero-cost lazy |
| Query syntax | ✅ SQL-like | ❌ Yok (method chaining) |
| Method chaining | ✅ Extension methods | ✅ Adapters |
| Performance | ⚠️ Runtime overhead | ✅ Zero-cost (compile-time) |
| Grouping | `GroupBy` (tüm koleksiyon) | `group_by` (ardışık) |
| Partitioning | Manuel | `partition` |
| Infinite sequences | `yield return` | `(0..)` range |
| Distinct | `Distinct()` | `itertools::unique` |
| Chunk | `.Chunk(n)` (.NET 6+) | `itertools::chunks` |
| Parallel | `AsParallel()` (PLINQ) | `par_iter()` (Rayon) |
| Collect | `ToList()`, `ToArray()` | `collect::<Vec<_>>()` |
| Type safety | ⚠️ Runtime | ✅ Compile-time |
| Custom iterator | Class + yield | `impl Iterator` |
| Borrow checking | ❌ Yok | ✅ Güçlü |
| Memory allocation | Heap (IEnumerator) | Stack (zero-cost) |

---

# 🚀 Son Tavsiyeler

1. **Her Zaman Referans Kullanın:** Koleksiyonu birden fazla kez döngüye alacaksanız `&koleksiyon` veya `.iter()` kullanın.

2. **`collect` Türünü Belirtin:** Turbofish (`::<>`) veya tür annotation kullanın.

3. **`itertools` Crate'ini Kullanın:** LINQ'taki eksik metodlar için şarttır.

4. **Terminal Operation'ı Unutmayın:** Iterator'lar tembeldir, `collect`, `sum`, `count` gibi bir terminal operation olmadan çalışmazlar.

5. **Lazy Evaluation'ı Avantajınıza Kullanın:** Sonsuz diziler ve early termination için ideal.

6. **Performans İçin Endişelenmeyin:** Rust iterator'ları zero-cost'tur, manuel loop kadar hızlıdır.

7. **Rayon ile Paralel Hale Getirin:** Büyük veri setlerinde `par_iter()` ile ciddi hızlanma sağlayın.

8. **Embedded Sistemlerde Dikkat:** `no_std` ortamında iterator'lar çalışır ama `collect` için heap gerekir. Mümkün olduğunda streaming kullanın.

9. **Custom Iterator Yazın:** Kendi iterator'larınızı yazmak, Rust'ın iterator sistemini anlamanın en iyi yoludur.

10. **`impl Iterator` Return Type Kullanın:** Fonksiyonlardan iterator döndürmek için en ergonomik yol.

11. **Borrow Checker ile Dost Olun:** Iterator ve mutable referans çakışmalarını erken fark edin.

12. **Step Motor Projeniz İçin:**
    ```rust
    // Hız profili üretimi - streaming yaklaşım
    let hiz_profili: Vec<u16> = (0..toplam_adim)
        .map(|i| ivme_hesapla(i, max_hiz, ivme))
        .collect();
    
    // Sensör verisi filtreleme
    let temiz_veri: Vec<f32> = sensor_oku_malar.iter()
        .filter(|&&x| x > esik && x < ust_sinir)
        .copied()
        .collect();
    ```

13. **Debug İçin `inspect` Kullanın:** Iterator zincirinde her adımı loglamak için `inspect(|x| println!("{:?}", x))` kullanın.

14. **Karmaşık Toplamalar için `fold` Kullanın:** Birden fazla değer döndürmek istiyorsanız `fold` idealdir.

15. **C# Query Syntax Yok:** Rust'ta SQL-benzeri query syntax yoktur, sadece method chaining vardır. Bu, aslında daha güçlü ve type-safe bir yaklaşımdır.

> 🦀 **Unutmayın:** Rust'ın iterator sistemi, C#'ın LINQ'undan **daha hızlı, daha güvenli ve daha esnektir**. Zero-cost abstraction sayesinde, elle yazılmış döngüler kadar performanslıdır. RP2354B projenizde iterator'lar, bellek verimliliği ve performans açısından idealdir. Özellikle `no_std` ortamında streaming işlemler için vazgeçilmezdir. Iterator'ları iyi anlamak, Rust'ta **idiomatic** kod yazmanın anahtarıdır! Bir sonraki adımda, async iterator'lar ve `Stream` trait'ini inceleyerek bu bilgiyi daha da derinleştirebilirsiniz.