# 🦀 Ders Notları: Bölüm 8 - 📚 Rust'ta Yaygın Koleksiyonlar (Common Collections) — Kapsamlı Ders

---

## 🎯 Bölüm Özeti

Rust'ın standart kütüphanesi, birden fazla değeri tek bir yapıda saklamamızı sağlayan **koleksiyonlar** adı verilen çok kullanışlı veri yapıları sunar. Normal tipler tek bir değeri temsil ederken, koleksiyonlar birden fazla değer içerebilir.

**Koleksiyonların en önemli özelliği:**
- Array (dizi) ve tuple'ların aksine, koleksiyonların işaret ettiği veriler **heap** bellekte saklanır.
- Bu, verinin miktarının derleme zamanında bilinmesinin gerekmediği ve program çalışırken büyüyüp küçülebileceği anlamına gelir.

Bu bölümde üç temel koleksiyonu öğreneceğiz:

| Koleksiyon | Açıklama | Kullanım Alanı |
|------------|----------|----------------|
| **Vec<T>** (Vektör) | Yan yana değerler listesi | Alışveriş listesi, dosya satırları |
| **String** | Karakter koleksiyonu | Metin işleme |
| **HashMap<K,V>** | Anahtar-değer eşleştirmesi | Veritabanı, skor tablosu |

---

## 1️⃣ VEKTÖRLER (Vectors) — `Vec<T>`

### 📖 Nedir?

Vektörler, **birden fazla değeri bellekte yan yana** saklayan koleksiyonlardır. Vektörler yalnızca **aynı tipte** değerler saklayabilir.

```rust
// ✅ Doğru: Hepsi i32
let sayilar = vec![1, 2, 3, 4, 5];

// ❌ Yanlış: Farklı tipler (bu derlenmez)
let karisik = vec![1, "merhaba", 3.14];
```

### 🔨 Oluşturma Yöntemleri

#### Yöntem 1: Boş Vektör Oluşturma — `Vec::new()`

```rust
fn main() {
    let v: Vec<i32> = Vec::new();
    // Tip belirtmek zorundayız çünkü Rust henüz ne saklayacağını bilmiyor
}
```

> ⚠️ **Önemli:** Boş bir vektör oluştururken **tip açıklaması (type annotation)** eklemeliyiz. Çünkü Rust'ın hangi tipte değer saklayacağımızı tahmin etmesi imkansızdır.

#### Yöntem 2: Başlangıç Değerleriyle Oluşturma — `vec!` Makrosu

```rust
fn main() {
    let v = vec![1, 2, 3];
    // Rust, v'nin Vec<i32> olduğunu otomatik anlar
    // Tip belirtmeye gerek yok!
}
```

> 💡 **Pratik Bilgi:** Başlangıç değerleri verildiğinde Rust tipi **çıkarım (inference)** yapar. Bu yüzden `vec!` makrosu en sık kullanılan yöntemdir.

#### Yöntem 3: Eleman Ekleyerek Büyüme — `push()`

```rust
fn main() {
    let mut v = Vec::new();  // mut kullanmak zorundayız!

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    // v şimdi [5, 6, 7, 8]
}
```

> 🔑 **Kural:** Bir vektörü değiştirebilmek için `mut` (mutable/değiştirilebilir) olmalıdır.

### 🔍 Elemanlara Erişim

Vektördeki elemanlara erişmenin **iki yolu** vardır:

#### Yol 1: İndeksleme (`[]`) — Tehlikeli ama Hızlı

```rust
fn main() {
    let v = vec![10, 20, 30, 40, 50];

    let ucuncu = &v[2];  // İndeks 0'dan başlar!
    println!("Üçüncü eleman: {ucuncu}");  // 30
}
```

#### Yol 2: `get()` Metodu — Güvenli

```rust
fn main() {
    let v = vec![10, 20, 30, 40, 50];

    match v.get(2) {
        Some(deger) => println!("Üçüncü eleman: {deger}"),
        None => println!("Üçüncü eleman yok."),
    }
}
```

### 🆚 `[]` vs `get()` — Hangisini Kullanmalı?

| Özellik | `v[indeks]` | `v.get(indeks)` |
|---------|-------------|-----------------|
| Geçersiz indeks | **Panic!** (Program çöker) | `None` döner |
| Dönüş tipi | `&T` (referans) | `Option<&T>` |
| Kullanım yeri | İndeksin geçerli olduğundan eminseniz | İndeks geçersiz olabilir (örn: kullanıcı girdisi) |

**Örnek — Geçersiz Erişim:**

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let x = &v[100];    // 💥 PANIC! Program anında çöker
    let y = v.get(100); // ✅ None döner, program devam eder
}
```

### ⚠️ Borrowing (Ödünç Alma) Kuralları ve Vektörler

Bu kısım çok önemli! Bir vektöre **değişmez (immutable) referans** tutarken vektöre **eleman ekleyemezsiniz**.

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let ilk = &v[0];  // Değişmez referans aldık

    v.push(6);        // ❌ HATA! Derlenmez!

    println!("İlk eleman: {ilk}");
}
```

**Neden?** Vektörler bellekte yan yana yer alır. Yeni eleman eklerken yer kalmazsa:
1. Vektör **yeni bellek alanı** tahsis eder
2. Eski elemanları **kopyalar**
3. Eski bellek **serbest bırakılır**

Eğer `ilk` referansı hala eski belleği işaret ediyorsa, **geçersiz bellek** erişimi olur. Rust'ın borrow checker'ı bunu **derleme zamanında** engeller!

### 🔄 Vektör Üzerinde Döngü

#### Değişmez (Immutable) Döngü

```rust
fn main() {
    let v = vec![100, 32, 57];
    
    for i in &v {
        println!("{i}");  // 100, 32, 57
    }
    // v hala kullanılabilir
}
```

#### Değiştirilebilir (Mutable) Döngü

```rust
fn main() {
    let mut v = vec![100, 32, 57];
    
    for i in &mut v {
        *i += 50;  // * (dereference) operatörü gerekli!
    }
    // v şimdi [150, 82, 107]
}
```

> 📌 `*` (dereference) operatörü, referansın gösterdiği değere erişmek için kullanılır.

### 🎭 Enum ile Farklı Tipleri Saklama

Vektörler sadece aynı tipte değer saklayabilir. Ancak **enum** kullanarak farklı tipleri aynı vektörde saklayabiliriz!

```rust
fn main() {
    enum HucreDegeri {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let satir = vec![
        HucreDegeri::Int(42),
        HucreDegeri::Text(String::from("mavi")),
        HucreDegeri::Float(3.14),
    ];
    // ✅ Hepsi HucreDegeri tipinde, derlenir!
}
```

### 🗑️ Bellek Yönetimi (Drop)

Vektörler kapsam dışına çıktığında otomatik olarak serbest bırakılır:

```rust
fn main() {
    {
        let v = vec![1, 2, 3, 4];
        // v ile işlemler...
    } // ← v burada kapsam dışına çıkar ve bellek serbest bırakılır
      // İçindeki tüm elemanlar da temizlenir
}
```

---

## 2️⃣ DİZELER (Strings)

### 📖 Rust'ta String Karmaşası

Yeni Rust kullanıcıları genellikle string'lerle zorlanır. Bunun 3 nedeni var:
1. Rust'ın olası hataları açığa çıkarma eğilimi
2. String'lerin düşünüldüğünden daha karmaşık bir veri yapısı olması
3. UTF-8 kodlaması

### 🔤 İki String Tipi

Rust'ta **iki farklı string tipi** vardır:

| Tip | Açıklama | Özellik |
|-----|----------|---------|
| `&str` (String Slice) | UTF-8 kodlanmış string verisine referans | Salt okunur, sabit boyut |
| `String` | Büyüyebilir, değiştirilebilir, sahip olunan UTF-8 string | Dinamik, heap'te |

> 💡 String literal'lar (örn: `"merhaba"`) programın binary'sinde saklanır ve `&str` tipindedir.

### 🔨 String Oluşturma

#### Yöntem 1: Boş String

```rust
fn main() {
    let mut s = String::new();
}
```

#### Yöntem 2: `to_string()` Metodu

```rust
fn main() {
    let veri = "başlangıç içeriği";
    let s = veri.to_string();
    
    // Veya doğrudan literal üzerinde:
    let s = "başlangıç içeriği".to_string();
}
```

#### Yöntem 3: `String::from()` Fonksiyonu

```rust
fn main() {
    let s = String::from("başlangıç içeriği");
}
```

> 💡 `to_string()` ve `String::from()` aynı işi yapar. Hangisini kullanacağınız zevk meselesi!

### 🌍 UTF-8 ve Çok Dilli Destek

String'ler UTF-8 kodlandığı için herhangi bir dildeki metni saklayabilir:

```rust
fn main() {
    let selam1 = String::from("السلام عليكم");  // Arapça
    let selam2 = String::from("こんにちは");      // Japonca
    let selam3 = String::from("你好");            // Çince
    let selam4 = String::from("مرحبا");           // Türkçe
    let selam5 = String::from("Здравствуйте");    // Rusça
}
```

### ➕ String Güncelleme

#### `push_str()` — String Slice Ekleme

```rust
fn main() {
    let mut s = String::from("mer");
    s.push_str("haba");
    // s artık "merhaba"
}
```

> 💡 `push_str()` bir **string slice** (`&str`) alır, `String` değil. Bu sayede eklenen değerin sahipliğini almaz.

```rust
fn main() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 hala kullanılabilir: {s2}");  // ✅ Çalışır
}
```

#### `push()` — Tek Karakter Ekleme

```rust
fn main() {
    let mut s = String::from("lo");
    s.push('l');  // Tek tırnak kullan!
    // s artık "lol"
}
```

### 🔗 String Birleştirme

#### `+` Operatörü

```rust
fn main() {
    let s1 = String::from("Merhaba, ");
    let s2 = String::from("Dünya!");
    let s3 = s1 + &s2;
    
    // s3 = "Merhaba, Dünya!"
    // ⚠️ s1 artık kullanılamaz (moved!)
    // ✅ s2 hala kullanılabilir
}
```

> ⚠️ **Önemli:** `+` operatörü `s1`'in sahipliğini alır (move), `s2`'nin referansını kullanır.

**Arka Planda Ne Oluyor?**

`+` operatörü aslında `add()` metodunu çağırır:

```rust
fn add(self, s: &str) -> String {
    // ...
}
```

- `self` → `s1`'in sahipliğini alır (bu yüzden `s1` artık kullanılamaz)
- `s: &str` → `s2`'nin referansını alır (bu yüzden `s2` hala kullanılabilir)
- `&String` → `&str`'ye **deref coercion** (otomatik dönüşüm) yapılır

#### `format!` Makrosu — Çoklu Birleştirme

Birden fazla string'i birleştirirken `format!` daha okunaklıdır:

```rust
fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // + ile (okunması zor):
    let s = s1 + "-" + &s2 + "-" + &s3;

    // format! ile (okunması kolay):
    let s = format!("{s1}-{s2}-{s3}");
    // s = "tic-tac-toe"
}
```

> 💡 `format!` makrosu `println!` gibi çalışır ama ekrana yazdırmak yerine bir `String` döner. Hiçbir parametrenin sahipliğini almaz.

### ❌ String İndeksleme — Neden Çalışmaz?

Diğer birçok dilde `s[0]` ile ilk karaktere erişebilirsiniz. **Rust'ta bu çalışmaz!**

```rust
fn main() {
    let s = String::from("merhaba");
    let h = s[0];  // ❌ HATA! Derlenmez
}
```

**Neden?** Çünkü Rust string'leri **UTF-8** olarak saklar ve:

1. **Her karakter aynı boyutta değil:**
   - `"Hola"` → 4 byte (her harf 1 byte)
   - `"Здравствуйте"` → 24 byte (her harf 2 byte)

2. **İndeksleme O(1) olmalı** ama UTF-8'de bu imkansız. Rust'ın karakter sayısını bulmak için baştan taraması gerekir.

3. **Hangi değeri döndüreceği belirsiz:** Byte mı? Karakter mi? Grapheme cluster mı?

### 🔍 String Üzerinde Gezinme Yöntemleri

#### Yöntem 1: `chars()` — Unicode Karakterler

```rust
fn main() {
    for c in "Зд".chars() {
        println!("{c}");
    }
    // Çıktı:
    // З
    // д
}
```

#### Yöntem 2: `bytes()` — Ham Baytlar

```rust
fn main() {
    for b in "Зд".bytes() {
        println!("{b}");
    }
    // Çıktı:
    // 208
    // 151
    // 208
    // 180
}
```

#### Yöntem 3: String Slices (Dilimleme)

```rust
fn main() {
    let hello = "Здравствуйте";
    let s = &hello[0..4];  // İlk 4 byte
    println!("{s}");  // "Зд"
}
```

> ⚠️ **Dikkat:** Geçersiz byte sınırlarında dilimleme **panic**'e neden olur:

```rust
let hello = "Здравствуйте";
let s = &hello[0..1];  // 💥 PANIC! 'З' karakterinin ortasında kesildi
```

### 📊 String'in 3 Yorumlanması

Rust string'lere 3 farklı şekilde bakabilir:

| Yorumlama | Açıklama | Örnek (नमस्ते) |
|-----------|----------|----------------|
| **Bytes** | Ham baytlar | `[224, 164, 168, ...]` (18 byte) |
| **Scalar Values** | Unicode karakterler | `['न', 'म', 'स', '्', 'त', 'े']` (6 char) |
| **Grapheme Clusters** | İnsanların gördüğü "harfler" | `["न", "म", "स्", "ते"]` (4 harf) |

> 💡 Grapheme cluster'lar için standart kütüphane yeterli değil, crates.io'dan ek kütüphaneler gerekebilir.

---

## 3️⃣ HASH MAP'LERİ — `HashMap<K, V>`

### 📖 Nedir?

Hash map'ler, **anahtar-değer (key-value)** çiftleri saklayan koleksiyonlardır. Veriye indeks yerine bir **anahtar** ile erişirsiniz.

Diğer dillerde farklı isimlerle bilinir:
- Hash, Map, Object, Hash Table, Dictionary, Associative Array

**Kullanım Alanları:**
- Skor tablosu (takım adı → skor)
- Telefon rehberi (isim → numara)
- Kelime frekansı (kelime → kaç kez geçti)

### 🔨 Oluşturma ve Eleman Ekleme

```rust
use std::collections::HashMap;

fn main() {
    let mut skorlar = HashMap::new();

    skorlar.insert(String::from("Mavi"), 10);
    skorlar.insert(String::from("Sarı"), 50);
}
```

> ⚠️ `HashMap` prelude'da (otomatik içe aktarılan) olmadığı için `use` ile içe aktarmalıyız.

### 🔍 Değerlere Erişim

#### `get()` Metodu

```rust
use std::collections::HashMap;

fn main() {
    let mut skorlar = HashMap::new();
    skorlar.insert(String::from("Mavi"), 10);
    skorlar.insert(String::from("Sarı"), 50);

    let takim = String::from("Mavi");
    let skor = skorlar.get(&takim).copied().unwrap_or(0);
    
    println!("Mavi takımın skoru: {skor}");  // 10
}
```

**Nasıl Çalışıyor?**
1. `get(&takim)` → `Option<&i32>` döner
2. `.copied()` → `Option<i32>` yapar (referansı kaldırır)
3. `.unwrap_or(0)` → `Some(değer)` ise değeri, `None` ise 0 döner

#### `for` Döngüsü ile Tüm Çiftleri Gezinme

```rust
use std::collections::HashMap;

fn main() {
    let mut skorlar = HashMap::new();
    skorlar.insert(String::from("Mavi"), 10);
    skorlar.insert(String::from("Sarı"), 50);

    for (anahtar, deger) in &skorlar {
        println!("{anahtar}: {deger}");
    }
    // Çıktı (sıra rastgele):
    // Sarı: 50
    // Mavi: 10
}
```

> ⚠️ Hash map'te elemanlar **rastgele sırada** döner!

### 🔐 Sahiplik (Ownership) Kuralları

#### `Copy` Trait'i Uygulayan Tipler (i32, f64, vb.)

```rust
let sayi = 42;
map.insert("anahtar".to_string(), sayi);
// sayi hala kullanılabilir (kopyalandı)
```

#### Sahiplik Alan Tipler (String, vb.)

```rust
use std::collections::HashMap;

fn main() {
    let alan_adi = String::from("Favori renk");
    let alan_degeri = String::from("Mavi");

    let mut map = HashMap::new();
    map.insert(alan_adi, alan_degeri);
    
    // ❌ alan_adi ve alan_degeri artık kullanılamaz!
    // HashMap sahipliği aldı
}
```

#### Referans Eklemek

```rust
let alan_adi = String::from("Favori renk");
let alan_degeri = String::from("Mavi");

let mut map = HashMap::new();
map.insert(&alan_adi, &alan_degeri);  // Referans ekle

// ✅ alan_adi ve alan_degeri hala kullanılabilir
// Ama map'in ömrü, referansların ömründen kısa olamaz!
```

### 🔄 Değer Güncelleme

#### Senaryo 1: Eski Değeri Değiştir

```rust
use std::collections::HashMap;

fn main() {
    let mut skorlar = HashMap::new();
    skorlar.insert(String::from("Mavi"), 10);
    skorlar.insert(String::from("Mavi"), 25);  // Üzerine yazar

    println!("{skorlar:?}");  // {"Mavi": 25}
}
```

#### Senaryo 2: Sadece Anahtar Yoksa Ekle — `entry()` API'si

```rust
use std::collections::HashMap;

fn main() {
    let mut skorlar = HashMap::new();
    skorlar.insert(String::from("Mavi"), 10);

    skorlar.entry(String::from("Sarı")).or_insert(50);  // Ekler (yeni)
    skorlar.entry(String::from("Mavi")).or_insert(50);  // Eklemez (zaten var)

    println!("{skorlar:?}");  // {"Sarı": 50, "Mavi": 10}
}
```

**`entry()` Nasıl Çalışır?**
- `Entry` enum'u döner: `Occupied` (dolu) veya `Vacant` (boş)
- `or_insert(değer)` → Anahtar varsa mevcut değerin mutable referansını, yoksa yeni değeri ekleyip referansını döner

#### Senaryo 3: Eski Değere Göre Güncelle

```rust
use std::collections::HashMap;

fn main() {
    let metin = "merhaba dünya harika dünya";
    let mut map = HashMap::new();

    for kelime in metin.split_whitespace() {
        let sayac = map.entry(kelime).or_insert(0);
        *sayac += 1;  // Mutable referans üzerinden güncelle
    }

    println!("{map:?}");
    // {"dünya": 2, "merhaba": 1, "harika": 1}
}
```

### 🔒 Hash Fonksiyonu ve Güvenlik

Varsayılan olarak `HashMap`, **SipHash** adı verilen bir hash fonksiyonu kullanır:

| Özellik | Açıklama |
|---------|----------|
| **Güvenlik** | Hash table DoS saldırılarına karşı dirençli |
| **Performans** | En hızlı algoritma değil, ama güvenli |
| **Değiştirilebilir** | Farklı bir hasher kullanılabilir (`BuildHasher` trait) |

> 💡 Performans kritikse, crates.io'dan farklı hasher kütüphaneleri eklenebilir.

---

## 📋 Özet Karşılaştırma Tablosu

| Özellik | Vec<T> | String | HashMap<K,V> |
|---------|--------|--------|--------------|
| **Saklama** | Yan yana değerler | UTF-8 baytlar | Anahtar-değer çiftleri |
| **Oluşturma** | `Vec::new()`, `vec![]` | `String::new()`, `String::from()`, `to_string()` | `HashMap::new()` |
| **Eleman Ekleme** | `push()` | `push_str()`, `push()`, `+`, `format!` | `insert()` |
| **Erişim** | `v[i]`, `v.get(i)` | `chars()`, `bytes()`, dilimleme | `map.get(&key)` |
| **Döngü** | `for i in &v` | `for c in s.chars()` | `for (k,v) in &map` |
| **Tip Kısıtlaması** | Tek tip | UTF-8 | Anahtarlar tek tip, değerler tek tip |
| **Bellek** | Heap | Heap (Vec<u8> wrapper) | Heap |
| **Sıralama** | Sıralı | Sıralı | Sırasız |

---

## 🎓 Sık Yapılan Hatalar ve Çözümleri

### Hata 1: Vektörde Farklı Tipler Saklamaya Çalışmak

```rust
// ❌ Yanlış
let v = vec![1, "merhaba", 3.14];

// ✅ Doğru: Enum kullan
enum Deger {
    Int(i32),
    Text(String),
    Float(f64),
}
let v = vec![Deger::Int(1), Deger::Text("merhaba".into()), Deger::Float(3.14)];
```

### Hata 2: String İndeksleme

```rust
// ❌ Yanlış
let s = String::from("merhaba");
let c = s[0];

// ✅ Doğru
let c = s.chars().nth(0).unwrap();
```

### Hata 3: Vektör Referansı Var İken Değiştirme

```rust
// ❌ Yanlış
let mut v = vec![1, 2, 3];
let ilk = &v[0];
v.push(4);  // HATA!

// ✅ Doğru
let mut v = vec![1, 2, 3];
v.push(4);
let ilk = &v[0];  // Referansı push'tan SONRA al
```

### Hata 4: HashMap'te Olmayan Anahtara Erişim

```rust
// ❌ Yanlış
let skor = skorlar["Kırmızı"];  // Panic!

// ✅ Doğru
let skor = skorlar.get("Kırmızı").copied().unwrap_or(0);
```

---

## 🚀 Sonraki Adımlar

Bu bölümde öğrendikleriniz:
- ✅ Vektör oluşturma, erişim, döngü
- ✅ String oluşturma, birleştirme, UTF-8 anlama
- ✅ HashMap oluşturma, erişim, güncelleme

**Pratik Yapmak İçin Öneriler:**
1. Bir todo list uygulaması yazın (Vec kullanın)
2. Metin dosyasındaki kelime frekanslarını sayın (HashMap kullanın)
3. Farklı dillerdeki selamlaşmaları saklayan bir program yazın (String + UTF-8)

**Daha Fazla Kaynak:**
- [Vec<T> API Dokümantasyonu](https://doc.rust-lang.org/std/vec/struct.Vec.html)
- [String API Dokümantasyonu](https://doc.rust-lang.org/std/string/struct.String.html)
- [HashMap API Dokümantasyonu](https://doc.rust-lang.org/std/collections/struct.HashMap.html)

---

*Bu ders notu, Rust Programming Language kitabının 8. bölümünden hazırlanmıştır. Sorularınız için Rust topluluğuna (users.rust-lang.org) başvurabilirsiniz.* 🦀
</think>

# 📚 Rust'ta Yaygın Koleksiyonlar — Kapsamlı Ders Notu

---

## 🎯 Bölüme Giriş

Rust'ın standart kütüphanesi, birden fazla değeri tek bir yapıda saklamamızı sağlayan **koleksiyonlar** adı verilen çok kullanışlı veri yapıları sunar. Normal tipler tek bir değeri temsil ederken, koleksiyonlar birden fazla değer içerebilir.

### 📌 Koleksiyonların Temel Özelliği

Array (dizi) ve tuple'ların aksine, koleksiyonların işaret ettiği veriler **heap** bellekte saklanır. Bu şu anlama gelir:

- Verinin miktarının **derleme zamanında bilinmesi gerekmez**
- Program çalışırken veri **büyüyüp küçülebilir**

### 📋 Bu Bölümde Öğreneceklerimiz

| Koleksiyon | Tip | Açıklama |
|------------|-----|----------|
| **Vector** | `Vec<T>` | Yan yana değerler listesi |
| **String** | `String` | Karakter koleksiyonu |
| **Hash Map** | `HashMap<K, V>` | Anahtar-değer eşleştirmesi |

---

## 1️⃣ VEKTÖRLER (`Vec<T>`)

### 📖 Nedir?

Vektörler, **birden fazla değeri bellekte yan yana** saklayan koleksiyonlardır. Vektörler yalnızca **aynı tipte** değerler saklayabilir.

> **Kullanım Alanları:** Dosyadaki metin satırları, alışveriş sepetindeki ürün fiyatları, sayı listeleri...

---

### 🔨 Vektör Oluşturma

#### Yöntem 1: Boş Vektör — `Vec::new()`

```rust
fn main() {
    let v: Vec<i32> = Vec::new();
}
```

> ⚠️ **Önemli:** Boş vektör oluştururken **tip açıklaması (type annotation)** vermek zorundayız. Çünkü Rust, içine henüz değer koymadığımız için hangi tipte değer saklayacağımızı bilemez.

#### Yöntem 2: Başlangıç Değerleriyle — `vec!` Makrosu

```rust
fn main() {
    let v = vec![1, 2, 3];
}
```

> 💡 **Avantaj:** Başlangıç değerleri verdiğimizde Rust tipi **çıkarım (inference)** yapabilir. Bu yüzden `Vec<i32>` yazmaya gerek yoktur.

#### Yöntem 3: Eleman Ekleyerek Büyüme — `push()`

```rust
fn main() {
    let mut v = Vec::new();
    
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}
```

> 🔑 **Kural:** Değiştirilebilir (mutable) olmalı, bu yüzden `mut` anahtar kelimesini kullanmak zorundayız.

---

### 🔍 Elemanlara Erişim

İki farklı yöntem vardır:

#### Yöntem 1: İndeksleme (`[]`)

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    
    let ucuncu: &i32 = &v[2];  // İndeks 0'dan başlar!
    println!("Üçüncü eleman: {ucuncu}");
}
```

#### Yöntem 2: `get()` Metodu

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    
    let ucuncu: Option<&i32> = v.get(2);
    match ucuncu {
        Some(deger) => println!("Üçüncü eleman: {deger}"),
        None => println!("Üçüncü eleman yok."),
    }
}
```

---

### 🆚 `[]` vs `get()` — Hangisini Ne Zaman Kullanmalı?

| Özellik | `v[indeks]` | `v.get(indeks)` |
|---------|-------------|-----------------|
| Geçersiz indeks | 💥 **Panic!** (Program çöker) | `None` döner |
| Dönüş tipi | `&T` (referans) | `Option<&T>` |
| Kullanım yeri | İndeksin **kesinlikle geçerli** olduğundan eminseniz | İndeks **geçersiz olabilir** (örn: kullanıcı girdisi) |

#### Örnek: Geçersiz Erişim

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    
    // Yöntem 1: Panic!
    let x = &v[100];  // 💥 Program çöker!
    
    // Yöntem 2: Güvenli
    let y = v.get(100);  // None döner, program devam eder
}
```

> 💡 **Pratik Bilgi:** Kullanıcıdan gelen bir indeks gibi belirsiz durumlarda `get()` kullanmak daha güvenlidir. Kullanıcıya "Lütfen 1-5 arasında bir sayı girin" deme şansınız olur.

---

### ⚠️ Borrowing (Ödünç Alma) Kuralları ve Vektörler

Bu kısım çok önemli! Bir vektöre **değişmez (immutable) referans** tutarken vektöre **eleman ekleyemezsiniz**:

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    
    let first = &v[0];  // Değişmez referans aldık
    
    v.push(6);  // ❌ HATA! Derlenmez!
    
    println!("İlk eleman: {first}");
}
```

**Hata Mesajı:**
```
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
```

#### 🤔 Neden Çalışmıyor?

Vektörler bellekte **yan yana** yer alır. Yeni eleman eklerken:

1. Eğer mevcut alanda yer yoksa → **Yeni bellek alanı** tahsis edilir
2. Eski elemanlar **yeni alana kopyalanır**
3. Eski bellek **serbest bırakılır**

Eğer `first` referansı hala eski belleği işaret ediyorsa → **Geçersiz bellek erişimi!**

> 🛡️ Rust'ın borrow checker'ı bunu **derleme zamanında** engelleyerek güvenliğinizi sağlar.

---

### 🔄 Vektör Üzerinde Döngü

#### Değişmez (Immutable) Döngü — Sadece Okuma

```rust
fn main() {
    let v = vec![100, 32, 57];
    
    for i in &v {
        println!("{i}");
    }
    // Çıktı: 100, 32, 57
}
```

#### Değiştirilebilir (Mutable) Döngü — Değiştirme

```rust
fn main() {
    let mut v = vec![100, 32, 57];
    
    for i in &mut v {
        *i += 50;  // Her elemana 50 ekle
    }
    // v şimdi [150, 82, 107]
}
```

> 📌 `*` (dereference) operatörü, referansın gösterdiği **gerçek değere** erişmek için kullanılır.

---

### 🎭 Enum ile Farklı Tipleri Saklama

Vektörler sadece **aynı tipte** değer saklayabilir. Peki farklı tipleri nasıl saklarız? **Enum** kullanarak!

```rust
fn main() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
```

> 💡 **Mantık:** Tüm enum varyantları **aynı tiptedir** (SpreadsheetCell). Bu yüzden vektörde saklanabilirler.

---

### 🗑️ Bellek Yönetimi (Drop)

Vektörler kapsam dışına çıktığında otomatik olarak serbest bırakılır:

```rust
fn main() {
    {
        let v = vec![1, 2, 3, 4];
        // v ile işlemler...
    } // ← v burada kapsam dışına çıkar ve bellek serbest bırakılır
      // İçindeki tüm elemanlar da temizlenir
}
```

---

## 2️⃣ DİZELER (Strings)

### 📖 Rust'ta String Karmaşası

Yeni Rust kullanıcıları genellikle string'lerle zorlanır. Bunun 3 temel nedeni var:

1. **Rust'ın hata yakalama eğilimi** — Olası hataları erkenden gösterir
2. **String'lerin karmaşıklığı** — Sandığınızdan daha karmaşık bir veri yapısı
3. **UTF-8 kodlaması** — Farklı diller, farklı byte uzunlukları

---

### 🔤 İki String Tipi

Rust'ta **iki farklı string tipi** vardır:

| Tip | Açıklama | Özellikler |
|-----|----------|------------|
| **`&str`** (String Slice) | UTF-8 kodlanmış string verisine referans | Genellikle `&str` şeklinde görülür |
| **`String`** | Büyüyebilir, değiştirilebilir, sahip olunan UTF-8 string | Standart kütüphaneden gelir |

> 📌 **String literal'lar** (örn: `"merhaba"`) programın binary'sinde saklanır ve `&str` tipindedir.

---

### 🔨 String Oluşturma

#### Yöntem 1: Boş String

```rust
fn main() {
    let mut s = String::new();
}
```

#### Yöntem 2: `to_string()` Metodu

```rust
fn main() {
    let data = "initial contents";
    let s = data.to_string();
    
    // Doğrudan literal üzerinde de çalışır:
    let s = "initial contents".to_string();
}
```

#### Yöntem 3: `String::from()` Fonksiyonu

```rust
fn main() {
    let s = String::from("initial contents");
}
```

> 💡 `to_string()` ve `String::from()` **aynı işi yapar**. Hangisini kullanacağınız zevk meselesi!

---

### 🌍 UTF-8 ve Çok Dilli Destek

String'ler UTF-8 kodlandığı için herhangi bir dildeki metni saklayabilir:

```rust
fn main() {
    let hello_arabic = String::from("السلام عليكم");
    let hello_czech = String::from("Dobrý den");
    let hello_english = String::from("Hello");
    let hello_hebrew = String::from("שלום");
    let hello_hindi = String::from("नमस्ते");
    let hello_japanese = String::from("こんにちは");
    let hello_korean = String::from("안녕하세요");
    let hello_chinese = String::from("你好");
    let hello_portuguese = String::from("Olá");
    let hello_russian = String::from("Здравствуйте");
    let hello_spanish = String::from("Hola");
}
```

---

### ➕ String Güncelleme

#### `push_str()` — String Slice Ekleme

```rust
fn main() {
    let mut s = String::from("foo");
    s.push_str("bar");
    // s artık "foobar"
}
```

> 💡 `push_str()` bir **string slice** (`&str`) alır, `String` değil. Bu sayede eklenen değerin sahipliğini almaz:

```rust
fn main() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");  // ✅ s2 hala kullanılabilir!
}
```

#### `push()` — Tek Karakter Ekleme

```rust
fn main() {
    let mut s = String::from("lo");
    s.push('l');  // Tek tırnak kullan!
    // s artık "lol"
}
```

---

### 🔗 String Birleştirme

#### `+` Operatörü

```rust
fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    
    // s3 = "Hello, world!"
    // ⚠️ s1 artık kullanılamaz (moved!)
    // ✅ s2 hala kullanılabilir
}
```

**Arka Planda Ne Oluyor?**

`+` operatörü aslında `add()` metodunu çağırır:

```rust
fn add(self, s: &str) -> String {
    // ...
}
```

- `self` → `s1`'in sahipliğini alır (bu yüzden `s1` artık kullanılamaz)
- `s: &str` → `s2`'nin referansını alır (bu yüzden `s2` hala kullanılabilir)
- `&String` → `&str`'ye **deref coercion** (otomatik dönüşüm) yapılır

> 💡 Bu implementasyon **kopyalama yapmaz**, `s1`'in mevcut belleğini kullanarak `s2`'nin içeriğini ekler. Bu yüzden verimlidir.

#### `format!` Makrosu — Çoklu Birleştirme

Birden fazla string'i birleştirirken `format!` daha okunaklıdır:

```rust
fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    
    // + ile (okunması zor):
    let s = s1 + "-" + &s2 + "-" + &s3;
    
    // format! ile (okunması kolay):
    let s = format!("{s1}-{s2}-{s3}");
    // s = "tic-tac-toe"
}
```

> 💡 `format!` makrosu `println!` gibi çalışır ama ekrana yazdırmak yerine bir `String` döner. Hiçbir parametrenin sahipliğini almaz.

---

### ❌ String İndeksleme — Neden Çalışmaz?

Diğer birçok dilde `s[0]` ile ilk karaktere erişebilirsiniz. **Rust'ta bu çalışmaz!**

```rust
fn main() {
    let s1 = String::from("hi");
    let h = s1[0];  // ❌ HATA! Derlenmez
}
```

**Hata Mesajı:**
```
error[E0277]: the type `str` cannot be indexed by `{integer}`
```

#### 🤔 Neden Çalışmıyor?

Çünkü Rust string'leri **UTF-8** olarak saklar. İşte 3 temel neden:

**1. Her karakter aynı boyutta değil:**

```rust
let hello = String::from("Hola");
// len = 4 byte (her harf 1 byte)

let hello = String::from("Здравствуйте");
// len = 24 byte (her harf 2 byte)
```

**2. Byte indeksi her zaman geçerli bir karaktere karşılık gelmez:**

```rust
let hello = "Здравствуйте";
let answer = &hello[0];  // İlk byte = 208, ama bu geçerli bir karakter değil!
```

**3. İndeksleme O(1) olmalı** ama UTF-8'de bu imkansız. Rust'ın karakter sayısını bulmak için baştan taraması gerekir.

---

### 🔍 String Üzerinde Gezinme Yöntemleri

Rust string'lere 3 farklı şekilde bakabilir:

#### Yöntem 1: `chars()` — Unicode Karakterler

```rust
for c in "Зд".chars() {
    println!("{c}");
}
// Çıktı:
// З
// д
```

#### Yöntem 2: `bytes()` — Ham Baytlar

```rust
for b in "Зд".bytes() {
    println!("{b}");
}
// Çıktı:
// 208
// 151
// 208
// 180
```

#### Yöntem 3: String Slices (Dilimleme)

```rust
let hello = "Здравствуйте";
let s = &hello[0..4];  // İlk 4 byte
// s = "Зд"
```

> ⚠️ **Dikkat:** Geçersiz byte sınırlarında dilimleme **panic**'e neden olur:

```rust
let hello = "Здравствуйте";
let s = &hello[0..1];  // 💥 PANIC! 'З' karakterinin ortasında kesildi
```

---

### 📊 String'in 3 Yorumlanması

Rust string'lere 3 farklı şekilde bakabilir:

| Yorumlama | Açıklama | Örnek (नमस्ते) |
|-----------|----------|----------------|
| **Bytes** | Ham baytlar | `[224, 164, 168, ...]` (18 byte) |
| **Scalar Values** | Unicode karakterler | `['न', 'म', 'स', '्', 'त', 'े']` (6 char) |
| **Grapheme Clusters** | İnsanların gördüğü "harfler" | `["न", "म", "स्", "ते"]` (4 harf) |

> 💡 Grapheme cluster'lar için standart kütüphane yeterli değil, crates.io'dan ek kütüphaneler gerekebilir.

---

## 3️⃣ HASH MAP'LERİ (`HashMap<K, V>`)

### 📖 Nedir?

Hash map'ler, **anahtar-değer (key-value)** çiftleri saklayan koleksiyonlardır. Veriye indeks yerine bir **anahtar** ile erişirsiniz.

> **Diğer Dillerdeki İsimleri:** hash, map, object, hash table, dictionary, associative array

**Kullanım Alanları:**
- Oyunlarda takım skorları (takım adı → skor)
- Telefon rehberi (isim → numara)
- Kelime frekansı sayacı (kelime → kaç kez geçti)

---

### 🔨 Hash Map Oluşturma

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}
```

> ⚠️ `HashMap` prelude'da (otomatik içe aktarılan) olmadığı için `use` ile içe aktarmalıyız.

> 📌 Vektörler gibi hash map'ler de veriyi **heap'te** saklar ve **homojendir**: Tüm anahtarlar aynı tipte, tüm değerler aynı tipte olmalı.

---

### 🔍 Değerlere Erişim

#### `get()` Metodu

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    
    // score = 10
}
```

**Nasıl Çalışıyor?**
1. `get(&team_name)` → `Option<&i32>` döner
2. `.copied()` → `Option<i32>` yapar (referansı kaldırır)
3. `.unwrap_or(0)` → `Some(değer)` ise değeri, `None` ise 0 döner

#### `for` Döngüsü ile Tüm Çiftleri Gezinme

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    // Çıktı (sıra rastgele):
    // Yellow: 50
    // Blue: 10
}
```

> ⚠️ Hash map'te elemanlar **rastgele sırada** döner!

---

### 🔐 Sahiplik (Ownership) Kuralları

#### `Copy` Trait'i Uygulayan Tipler (i32, f64, vb.)

Değerler hash map'e **kopyalanır**:

```rust
let sayi = 42;
map.insert("anahtar".to_string(), sayi);
// sayi hala kullanılabilir
```

#### Sahiplik Alan Tipler (String, vb.)

Değerler hash map'e **taşınır** (move):

```rust
use std::collections::HashMap;

fn main() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    
    // ❌ field_name ve field_value artık kullanılamaz!
    // HashMap sahipliği aldı
}
```

#### Referans Eklemek

```rust
let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(&field_name, &field_value);  // Referans ekle

// ✅ field_name ve field_value hala kullanılabilir
// Ama map'in ömrü, referansların ömründen kısa olamaz!
```

---

### 🔄 Değer Güncelleme

Bir anahtar zaten varsa, nasıl güncelleyeceğimize karar vermemiz gerekir:

#### Senaryo 1: Eski Değeri Değiştir

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);  // Üzerine yazar
    
    println!("{scores:?}");  // {"Blue": 25}
}
```

#### Senaryo 2: Sadece Anahtar Yoksa Ekle — `entry()` API'si

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    
    scores.entry(String::from("Yellow")).or_insert(50);  // Ekler (yeni)
    scores.entry(String::from("Blue")).or_insert(50);    // Eklemez (zaten var)
    
    println!("{scores:?}");  // {"Yellow": 50, "Blue": 10}
}
```

**`entry()` Nasıl Çalışır?**
- `Entry` enum'u döner: `Occupied` (dolu) veya `Vacant` (boş)
- `or_insert(değer)` → Anahtar varsa mevcut değerin mutable referansını, yoksa yeni değeri ekleyip referansını döner

#### Senaryo 3: Eski Değere Göre Güncelle

```rust
use std::collections::HashMap;

fn main() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;  // Mutable referans üzerinden güncelle
    }
    
    println!("{map:?}");
    // {"world": 2, "hello": 1, "wonderful": 1}
}
```

---

### 🔒 Hash Fonksiyonu ve Güvenlik

Varsayılan olarak `HashMap`, **SipHash** adı verilen bir hash fonksiyonu kullanır:

| Özellik | Açıklama |
|---------|----------|
| **Güvenlik** | Hash table DoS saldırılarına karşı dirençli |
| **Performans** | En hızlı algoritma değil, ama güvenli |
| **Değiştirilebilir** | Farklı bir hasher kullanılabilir (`BuildHasher` trait) |

> 💡 Performans kritikse, crates.io'dan farklı hasher kütüphaneleri eklenebilir.

---

## 📋 Özet Karşılaştırma Tablosu

| Özellik | `Vec<T>` | `String` | `HashMap<K,V>` |
|---------|----------|----------|----------------|
| **Saklama** | Yan yana değerler | UTF-8 baytlar | Anahtar-değer çiftleri |
| **Oluşturma** | `Vec::new()`, `vec![]` | `String::new()`, `String::from()`, `to_string()` | `HashMap::new()` |
| **Eleman Ekleme** | `push()` | `push_str()`, `push()`, `+`, `format!` | `insert()` |
| **Erişim** | `v[i]`, `v.get(i)` | `chars()`, `bytes()`, dilimleme | `map.get(&key)` |
| **Döngü** | `for i in &v` | `for c in s.chars()` | `for (k,v) in &map` |
| **Tip Kısıtlaması** | Tek tip | UTF-8 | Anahtarlar tek tip, değerler tek tip |
| **Bellek** | Heap | Heap (Vec<u8> wrapper) | Heap |
| **Sıralama** | Sıralı | Sıralı | Sırasız |
| **İndeksleme** | ✅ Var | ❌ Yok (UTF-8 nedeniyle) | Anahtar ile erişim |

---

## 🎓 Sık Yapılan Hatalar ve Çözümleri

### Hata 1: Vektörde Farklı Tipler Saklamaya Çalışmak

```rust
// ❌ Yanlış
let v = vec![1, "merhaba", 3.14];

// ✅ Doğru: Enum kullan
enum Deger {
    Int(i32),
    Text(String),
    Float(f64),
}
let v = vec![Deger::Int(1), Deger::Text("merhaba".into()), Deger::Float(3.14)];
```

### Hata 2: String İndeksleme

```rust
// ❌ Yanlış
let s = String::from("merhaba");
let c = s[0];

// ✅ Doğru
let c = s.chars().nth(0).unwrap();
```

### Hata 3: Vektör Referansı Var İken Değiştirme

```rust
// ❌ Yanlış
let mut v = vec![1, 2, 3];
let ilk = &v[0];
v.push(4);  // HATA!

// ✅ Doğru
let mut v = vec![1, 2, 3];
v.push(4);
let ilk = &v[0];  // Referansı push'tan SONRA al
```

### Hata 4: HashMap'te Olmayan Anahtara Erişim

```rust
// ❌ Yanlış
let skor = skorlar["Kırmızı"];  // Panic!

// ✅ Doğru
let skor = skorlar.get("Kırmızı").copied().unwrap_or(0);
```

---

## 🚀 Sonraki Adımlar

Bu bölümde öğrendikleriniz:
- ✅ Vektör oluşturma, erişim, döngü
- ✅ String oluşturma, birleştirme, UTF-8 anlama
- ✅ HashMap oluşturma, erişim, güncelleme

**Pratik Yapmak İçin Öneriler:**
1. Bir todo list uygulaması yazın (Vec kullanın)
2. Metin dosyasındaki kelime frekanslarını sayın (HashMap kullanın)
3. Farklı dillerdeki selamlaşmaları saklayan bir program yazın (String + UTF-8)

**Daha Fazla Kaynak:**
- [Vec<T> API Dokümantasyonu](https://doc.rust-lang.org/std/vec/struct.Vec.html)
- [String API Dokümantasyonu](https://doc.rust-lang.org/std/string/struct.String.html)
- [HashMap API Dokümantasyonu](https://doc.rust-lang.org/std/collections/struct.HashMap.html)