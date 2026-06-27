# 🦀 Rust'ta İlkel Türler (Primitives) — Kapsamlı Ders

Hoş geldiniz! Bu derste Rust programlama dilinin temel yapı taşları olan **ilkel türleri (primitives)** en ince detayına kadar inceleyeceğiz. Rust'ın güçlü tip sistemi, derleme zamanında güvenliği sağlamak için bu türleri sıkı bir şekilde tanımlar. Hazırsanız başlayalım!

---

## 📚 Bölüm 1: Temel İlkel Türler

Rust, bellekte nasıl temsil edildikleri ve hangi aralıkta değer alabildikleri bakımından çeşitli ilkel türler sunar.

### 1.1 — İşaretli Tamsayılar (Signed Integers)

Negatif değer alabilen tam sayılardır. İsimlerindeki "i" harfi "integer" (tam sayı), rakam ise **bit uzunluğunu** temsil eder.

| Tür | Bit Genişliği | Minimum Değer | Maksimum Değer |
|-----|---------------|---------------|----------------|
| `i8` | 8 bit | -128 | 127 |
| `i16` | 16 bit | -32.768 | 32.767 |
| `i32` | 32 bit | ≈ -2,1 milyar | ≈ 2,1 milyar |
| `i64` | 64 bit | çok büyük negatif | çok büyük pozitif |
| `i128` | 128 bit | çok çok büyük | çok çok büyük |
| `isize` | İşletimci mimarisine bağlı (32/64 bit) | — | — |

> 💡 **İpucu:** Rust'ta hiçbir ek belirtme yapmazsanız, tam sayılar **varsayılan olarak `i32`** türündedir. Bu, performans ve taşınabilirlik açısından dengeli bir tercihtir.

```rust
let x: i32 = 42;        // Açıkça i32 olarak belirtildi
let y = -10;            // Tür belirtilmedi, Rust bunu i32 olarak varsayar
let z: isize = 100;     // İşlemci mimarisine göre boyutlanır (64-bit sistemde 64 bit)
```

### 1.2 — İşaretsiz Tamsayılar (Unsigned Integers)

Sadece **sıfır ve pozitif** değerler alabilen tam sayılardır. "u" harfi "unsigned" (işaretsiz) anlamına gelir.

| Tür | Bit Genişliği | Minimum Değer | Maksimum Değer |
|-----|---------------|---------------|----------------|
| `u8` | 8 bit | 0 | 255 |
| `u16` | 16 bit | 0 | 65.535 |
| `u32` | 32 bit | 0 | ≈ 4,2 milyar |
| `u64` | 64 bit | 0 | çok büyük |
| `u128` | 128 bit | 0 | çok çok büyük |
| `usize` | Mimariye bağlı | 0 | — |

> 📌 **`usize` ne zaman kullanılır?** Genellikle dizi indekslemelerinde, koleksiyon boyutlarında ve bellek adresi hesaplamalarında kullanılır. Çünkü bellek adresleme boyutuyla eşleşir.

```rust
let yas: u8 = 25;           // 0-255 arası, yaş için ideal
let dosya_boyutu: u64 = 1_000_000_000;
let indeks: usize = 0;       // Bir dizinin 5. elemanına erişmek için
```

### 1.3 — Kayan Nokta Sayıları (Floating-Point Numbers)

Ondalıklı sayıları temsil ederler. IEEE 754 standardına göre çalışırlar.

| Tür | Bit Genişliği | Açıklama |
|-----|---------------|----------|
| `f32` | 32 bit | Tek hassasiyetli kayan nokta |
| `f64` | 64 bit | Çift hassasiyetli kayan nokta (varsayılan) |

> 💡 **Neden `f64` varsayılan?** Modern işlemcilerde `f64`, `f32` ile neredeyse aynı hızda çalışır ama çok daha fazla hassasiyet sunar.

```rust
let pi: f64 = 3.14159265358979;    // Varsayılan olarak f64
let yaricap: f32 = 2.5_f32;        // Suffix ile f32 olarak belirtildi
let alan = pi * yaricap as f64 * yaricap as f64;
```

### 1.4 — Karakter Türü (`char`)

Rust'ta `char` türü **tek bir Unicode karakteri** temsil eder ve **4 byte** yer kaplar. Bu, yalnızca ASCII değil, dünya üzerindeki tüm dillerin karakterlerini, emojileri ve matematiksel sembolleri destekleyebileceğiniz anlamına gelir!

```rust
let harf: char = 'a';
let yunanca: char = 'α';      // Yunan alfa
let sonsuzluk: char = '∞';    // Matematiksel sembol
let emoji: char = '🦀';       // Rust yengeci!
let turkce: char = 'ş';       // Türkçe karakter sorunsuz çalışır
```

> ⚠️ **Dikkat:** `char` tek tırnak `' '` ile, string (metin) çift tırnak `" "` ile yazılır!

### 1.5 — Boolean (`bool`)

Mantıksal değerlerdir. Sadece iki değeri olabilir: `true` (doğru) veya `false` (yanlış). Bellekte **1 byte** yer kaplar.

```rust
let rust_seviyor_mu: bool = true;
let bugun_cuma: bool = false;

if rust_seviyor_mu {
    println!("Harika, doğru yoldasın! 🦀");
}
```

### 1.6 — Unit Türü `()`

Rust'a özgü ilginç bir türdür. **Değeri olmayan** durumu temsil eder. Tek bir geçerli değeri vardır: `()` (boş parantez).

> 🤔 **Ne işe yarar?** Hiçbir şey döndürmeyen fonksiyonlar aslında `()` döndürür. Bu, Rust'ta "void" kavramının karşılığıdır.

```rust
fn selam_ver() {
    println!("Merhaba!");
    // Bu fonksiyon aslında () döndürür
}

let sonuc = selam_ver();  // sonuc değişkeni () türündedir
assert_eq!(sonuc, ());     // () sadece kendine eşittir
```

---

## 📚 Bölüm 2 — Literaller (Değişmezler)

Literaller, kaynak kodda **sabit olarak yazılan** değerlerdir. Rust, literalleri çok esnek bir şekilde yazmanıza olanak tanır.

### 2.1 — Sayı Sistemleri

Rust'ta tamsayıları farklı sayı sistemlerinde yazabilirsiniz:

```rust
let onluk     = 42;          // Onluk (decimal) sistem
let on_altili = 0x2A;        // On altılı (hexadecimal) — 0x öneki
let sekizli   = 0o52;        // Sekizli (octal) — 0o öneki
let ikili     = 0b101010;    // İkili (binary) — 0b öneki
```

### 2.2 — Okunabilirlik İçin Alt Çizgi

Büyük sayıları okumak zordur. Rust, sayıların içine **alt çizgi `_`** koymanıza izin verir. Derleyici bunları tamamen yok sayar.

```rust
let bir_milyon = 1_000_000;          // 1000000 ile aynı
let ondalikli  = 0.000_001;          // 0.000001 ile aynı
let hex_buyuk  = 0xFFFF_FFFF;        // Okuması çok daha kolay
```

### 2.3 — Bilimsel Gösterim (E-Notation)

Çok büyük veya çok küçük kayan nokta sayıları için bilimsel gösterim kullanabilirsiniz. Bu durumda tür **otomatik olarak `f64`** olur.

```rust
let avogadro = 6.022e23;     // 6.022 × 10²³
let kucuk    = 7.6e-4;       // 0.00076
let buyuk    = 1e6;          // 1.000.000 (f64 olarak)
```

### 2.4 — Tür Sonekleri (Type Suffixes)

Derleyiciye literallerin türünü açıkça söylemek için sonek kullanabilirsiniz:

```rust
let x = 42u32;          // İşaretsiz 32-bit tam sayı
let y = -100i64;        // İşaretli 64-bit tam sayı
let z = 3.14f32;        // 32-bit kayan nokta
```

### 2.5 — Operatör Önceliği

Rust'taki operatör önceliği, C-benzeri dillere çok benzer. Örneğin çarpma/bölme, toplama/çıkarmadan önce yapılır:

```rust
let sonuc = 2 + 3 * 4;   // 14 olur, 20 değil!
let dogru = 2 + (3 * 4); // Parantez ile açıkça belirtmek her zaman iyidir
```

---

## 📚 Bölüm 3 — Tür Çıkarımı (Type Inference)

Rust, **akıllı bir derleyiciye** sahiptir ve çoğu zaman türünüzü sizin yerinize kendisi tahmin eder. Buna "type inference" denir.

```rust
// Aşağıdaki üç satır da aynı anlama gelir:
let a = 42;               // Derleyici i32 olarak çıkarır (varsayılan)
let b: i32 = 42;          // Açıkça belirtildi
let c = 42i32;            // Sonek ile belirtildi

// Bağlamdan çıkarım:
let mut sayilar = Vec::new();
sayilar.push(42);          // Derleyici, push'tan Vec<i32> olduğunu anlar!
```

> ⚠️ **Dikkat:** Eğer derleyici türü çıkaracak yeterli bağlam bulamazsa, derleme hatası alırsınız. Bu durumda ya açıkça tür belirtmeli ya da sonek kullanmalısınız.

---

## 📚 Bölüm 4 — Tuple'lar (Demetler)

Tuple, **farklı türlerde** değerleri bir arada tutabilen sabit uzunluklu bir koleksiyondur. Parantez `()` ile oluşturulur.

### 4.1 — Tuple Oluşturma ve Erişim

```rust
let kişi: (&str, i32, f64) = ("Ahmet", 28, 1.75);

// Elemanlara indeks ile erişim (0'dan başlar)
println!("İsim: {}", kişi.0);    // "Ahmet"
println!("Yaş: {}", kişi.1);     // 28
println!("Boy: {}", kişi.2);     // 1.75
```

### 4.2 — Tuple Pattern Matching (Yapı Çözme)

Tuple'ları değişkenlere "açmak" (destructure) çok kolaydır:

```rust
let (isim, yas, boy) = kişi;
println!("{} {} yaşında ve {:.2} metre.", isim, yas, boy);
```

### 4.3 — Tuple ile Çoklu Değer Döndürme

Rust fonksiyonları sadece bir değer döndürebilir, ama bir tuple döndürerek **birden fazla değer** döndürebilirsiniz:

```rust
fn sayilari_topla_cikar(a: i32, b: i32) -> (i32, i32) {
    (a + b, a - b)   // Toplam ve farkı tuple olarak döndür
}

let (toplam, fark) = sayilari_topla_cikar(10, 3);
println!("Toplam: {}, Fark: {}", toplam, fark);  // 13, 7
```

### 4.4 — Unit Tuple `()`

Hiç elemanı olmayan tuple'a "unit tuple" denir ve türü `()`'dir. Bu, Rust'ta "hiçbir şey döndürmeme" anlamına gelir.

```rust
fn bir_ise_yara() -> () {
    println!("Bu fonksiyon bir şey döndürmez.");
}
// Aslında -> () yazmaya gerek yoktur, Rust bunu varsayar.
```

### 4.5 — Debug Yazdırma

Tuple'lar `{:?}` (debug formatı) ile kolayca yazdırılabilir:

```rust
let nokta = (3, 5);
println!("Nokta: {:?}", nokta);   // Nokta: (3, 5)
```

---

## 📚 Bölüm 5 — Diziler (Arrays) ve Dilimler (Slices)

### 5.1 — Diziler

Dizi (array), **aynı türden** elemanların **bitişik bellekte** saklandığı bir koleksiyondur. Köşeli parantez `[]` ile oluşturulur.

> 🔑 **Önemli:** Rust'ta dizilerin uzunluğu **türünün bir parçasıdır**. Yani `[i32; 3]` ile `[i32; 5]` **farklı türlerdir!**

```rust
let sayilar: [i32; 5] = [10, 20, 30, 40, 50];
//        ^^^^  ^
//        tür   uzunluk

// Elemanlara erişim
println!("İlk eleman: {}", sayilar[0]);   // 10
println!("Son eleman: {}", sayilar[4]);   // 50

// Uzunluk
println!("Uzunluk: {}", sayilar.len());   // 5
```

### 5.2 — Tekrarlı Dizi Oluşturma

Aynı değeri birden çok kez tekrarlamak isterseniz kısa bir sözdizimi vardır:

```rust
let sifirlar = [0; 5];     // [0, 0, 0, 0, 0]
let birler: [u8; 3] = [1; 3];   // [1, 1, 1]
```

### 5.3 — Dilimler (Slices)

Diziler sabit uzunlukludur, ama bazen bir dizinin **bir kısmına** bakmak istersiniz. İşte burada **slice** devreye girer.

Slice, derleme zamanında uzunluğu bilinmeyen, **bir veri bloğunun referansıdır**. İki kelimelik (two-word) bir nesnedir:
1. Veriye işaret eden bir **pointer**
2. Dilimin **uzunluğu**

Tür gösterimi: `&[T]` (T türünden bir dilime referans)

```rust
let dizi = [10, 20, 30, 40, 50];

// Dilim oluşturma (başlangıç..bitiş) — bitiş dahil DEĞİL
let dilim1: &[i32] = &dizi[1..4];    // [20, 30, 40]
let dilim2: &[i32] = &dizi[..3];     // [10, 20, 30] (baştan 3'e kadar)
let dilim3: &[i32] = &dizi[2..];     // [30, 40, 50] (2'den sona kadar)
let hepsi: &[i32] = &dizi[..];       // Tüm dizi

// Dilimi yazdırma
println!("Dilim: {:?}", dilim1);
```

### 5.4 — Dizi vs. Slice Karşılaştırması

| Özellik | Dizi `[T; N]` | Dilim `&[T]` |
|---------|---------------|--------------|
| Uzunluk | Derleme zamanında bilinir | Çalışma zamanında bilinir |
| Bellek | Kendi verisini tutar | Başka bir yere işaret eder |
| Boyut | Sabit | Dinamik (referans edilen kısma göre) |
| Kullanım | Veri sahipliği | Veri ödünç alma (borrowing) |

### 5.5 — Dizi ile İlgili Yaygın Hata

Rust, dizi sınırlarını **çalışma zamanında** kontrol eder. Sınır dışı erişimde program **panik** yapar (güvenlik!):

```rust
let dizi = [1, 2, 3];
// println!("{}", dizi[10]);  // ⚠️ PANIC! index out of bounds
```

---

## 📚 Bölüm 6 — Pratik Örnek: Hepsini Birleştirelim

Şimdi öğrendiklerimizi birleştiren kapsamlı bir örnek:

```rust
fn main() {
    // === Temel İlkel Türler ===
    let isaretli: i32 = -42;
    let isaretsiz: u8 = 255;
    let kayan_nokta: f64 = 3.14159;
    let karakter: char = '🦀';
    let dogru_mu: bool = true;
    let birim: () = ();

    // === Literaller ===
    let hex = 0xFF_u32;           // 255
    let okunabilir = 1_000_000i64;
    let bilimsel = 6.022e23;

    // === Tuple ===
    let ogrenci: (&str, u8, f64) = ("Zeynep", 20, 3.45);
    let (isim, yas, gno) = ogrenci;
    println!("{} — Yaş: {}, GNO: {:.2}", isim, yas, gno);

    // === Dizi ve Dilim ===
    let notlar: [f64; 5] = [85.5, 90.0, 78.5, 92.0, 88.0];
    let ilk_uc: &[f64] = &notlar[..3];
    
    println!("Tüm notlar: {:?}", notlar);
    println!("İlk üç not: {:?}", ilk_uc);
    println!("Ortalama: {:.2}", ortalama(&notlar));
}

fn ortalama(sayilar: &[f64]) -> f64 {
    let toplam: f64 = sayilar.iter().sum();
    toplam / sayilar.len() as f64
}
```

---

## 🎯 Özet Tablosu

| Tür | Açıklama | Örnek | Varsayılan |
|-----|----------|-------|------------|
| `i8`..`i128` | İşaretli tam sayılar | `-42i32` | `i32` |
| `u8`..`u128` | İşaretsiz tam sayılar | `255u8` | — |
| `isize`/`usize` | Mimariye göre boyut | `0usize` | — |
| `f32`/`f64` | Kayan nokta | `3.14f64` | `f64` |
| `char` | Unicode karakter | `'🦀'` | — |
| `bool` | Mantıksal değer | `true` | — |
| `()` | Unit (boş) tür | `()` | — |
| `[T; N]` | Sabit uzunluklu dizi | `[1, 2, 3]` | — |
| `(T1, T2, ...)` | Tuple (farklı türler) | `(1, "a", true)` | — |
| `&[T]` | Dilim (slice) | `&arr[1..3]` | — |

---

## 🧠 Öğrenme Kontrol Listesi

Bu dersi tam olarak anladığınızı doğrulamak için kendinize şu soruları sorun:

- [ ] `i32` ile `u32` arasındaki fark nedir?
- [ ] Rust'ta bir tam sayının varsayılan türü nedir?
- [ ] `char` türü kaç byte yer kaplar ve neden?
- [ ] `0x`, `0o`, `0b` önekleri ne anlama gelir?
- [ ] Tuple ile dizinin farkı nedir?
- [ ] Slice nedir, neden kullanılır?
- [ ] `()` türü ne zaman karşımıza çıkar?
- [ ] `1_000_000` ifadesi ne anlama gelir?

---

## 🚀 Sonraki Adımlar

İlkel türleri öğrendiğinize göre, artık Rust'ın daha gelişmiş konularına geçebilirsiniz:
- **Type Inference** (Tür Çıkarımı) — Derleyicinin türleri nasıl tahmin ettiği
- **Mutability** (Değişkenlik) — `mut` anahtar kelimesi
- **Shadowing** (Gölgeleme) — Aynı isimle yeni değişken oluşturma
- **std::fmt** — Türleri nasıl biçimlendirip yazdıracağınız

Rust yolculuğunuzda başarılar! 🦀✨

# DeepSeek Yorumu -----------------

Bugün Rust programlama dilinin temellerini oluşturan **ilkel veri tiplerini** (primitives) en ince ayrıntısına kadar öğreneceğiz. Rust By Example kitabının `primitives` bölümü ve onun alt sayfaları olan *Literals and Operators*, *Tuples* ve *Arrays and Slices* konularını bir ders bütünlüğü içinde işleyeceğiz. Hazırsanız Rust’ın tip sisteminin kalbine doğru bir yolculuğa çıkalım.

---

## 1. Giriş – Neden İlkel Tiplerle Başlıyoruz?

Rust, **statik ve güçlü tip sistemine** sahip bir dildir. Bu, her değişkenin türünün derleme zamanında bilindiği ve belleğin nasıl yönetileceğinin garanti altına alındığı anlamına gelir. İlkel tipler (primitive types), dilin üzerine inşa edildiği en küçük yapı taşlarıdır. Bu tipler belleğin **stack** bölgesinde saklanır, kopyalanmaları ucuzdur ve sabit bir boyuta sahiptirler.

Rust’ta ilkel veri tiplerini dört ana grupta inceleyebiliriz:

- **Skaler (Scalar) Tipler**: Tek bir değeri temsil ederler.
  - Tam sayılar (integers)
  - Kayan noktalı sayılar (floats)
  - Boolean (doğru/yanlış)
  - Karakter (char)
- **Bileşik (Compound) Tipler**: Birden fazla değeri bir arada tutarlar.
  - Demetler (tuples)
  - Diziler (arrays)
  - Dilimler (slices) – teknik olarak bir ilkel tip olmasa da bellek görünümü (view) sağlayan temel bir referans türüdür ve bu bölümde anlatılır.

Şimdi ilk olarak en basit yapı taşları olan skaler tipleri tanıyalım.

---

## 2. Skaler Tipler (Scalar Types)

### 2.1. Tam Sayılar (Integer Types)

Rust, her biri farklı boyut ve işaret durumuna sahip tam sayı tipleri sunar. Uzunlukları bit cinsinden isminden anlaşılır:

| Tip      | İşaretli | İşaretsiz | Boyut (bit) |
|----------|----------|-----------|-------------|
| 8-bit    | `i8`     | `u8`      | 8           |
| 16-bit   | `i16`    | `u16`     | 16          |
| 32-bit   | `i32`    | `u32`     | 32          |
| 64-bit   | `i64`    | `u64`     | 64          |
| 128-bit  | `i128`   | `u128`    | 128         |
| Mimarîye bağlı | `isize` | `usize`  | pointer boyutu |

- **Varsayılan tam sayı tipi** `i32`’dir. Eğer açıkça belirtmezseniz ve derleyici bağlamdan çıkaramazsa, tam sayı literalleri `i32` olarak kabul edilir.
- `isize` ve `usize` özellikle dizi indeksleme ve bellek adresleme işlemlerinde kullanılır; 64 bit bir sistemde 64 bit, 32 bitte 32 bit boyutundadır.

**Örnek:**
```rust
let tam_sayi = 42;           // tür çıkarımı -> i32
let negatif: i64 = -500;     // açık tür belirtme
let isaretsiz: u8 = 255;     // 0..255 arası
```

### 2.2. Kayan Noktalı Sayılar (Floating-Point Types)

Kesirli sayılar için iki tip vardır: `f32` (tek duyarlıklı) ve `f64` (çift duyarlıklı). **Varsayılan float tipi** `f64`’tür çünkü modern işlemcilerde hız kaybı olmaksızın daha yüksek hassasiyet sunar.

```rust
let pi = 3.14159;            // varsayılan f64
let tek: f32 = 2.71828;      // f32
```

### 2.3. Boolean Tipi

Mantıksal ifadeler için sadece iki değer alabilir: `true` veya `false`. `bool` tipi 1 byte yer kaplar.

```rust
let dogru_mu: bool = true;
let yanlis = false;          // tür çıkarımı
```

### 2.4. Karakter Tipi

Rust’ta `char` tipi, Unicode bir **skaler değer** saklar. 4 byte boyutundadır (C’deki gibi 1 byte değildir). Tek tırnak ile ifade edilir.

```rust
let harf: char = 'A';
let emoji = '🚀';            // char Unicode emoji de tutabilir
```

> **Not:** Karakter tiplerini stringlerle karıştırmayın. `"Merhaba"` bir string slice (`&str`), `'M'` ise `char`’dır.

---

## 3. Değişken Tanımlama, Tip Çıkarımı ve Gölgeleme

Bir değişkeni `let` anahtar kelimesiyle tanımlarız. Rust’ta değişkenler **varsayılan olarak değişmez (immutable)** dir. Değiştirilebilir olması için `mut` eklenmelidir.

```rust
let x = 5;          // immutable i32
// x = 6;           // HATA! değiştirilemez
let mut y = 10;     // mutable i32
y = 15;             // geçerli
```

### Tip Çıkarımı (Type Inference)
Rust, birçok durumda tipi belirtmeseniz bile bağlamdan çıkarabilir. Özellikle sayısal literallerde esnek davranır; ancak gerekli durumlarda açıkça belirtmek en iyisidir.

```rust
let sayi = 100;                // i32
let float_sayi = 1.5;          // f64
let metin = "Merhaba";         // &str
```

### Gölgeleme (Shadowing)
Aynı isimle yeni bir değişken tanımladığınızda, önceki değişken **gölgelenir**. Bu, değişkenin türünü değiştirmek ya da değerini dönüştürmek için kullanışlı bir yöntemdir. `mut`’tan farkı, aslında yeni bir bağ oluşturmanızdır.

```rust
let uzunluk = 10;
let uzunluk = uzunluk + 5;      // 15, yeni i32
let uzunluk = "on beş";         // şimdi &str oldu
```

Şimdiye kadar skaler tiplerin temellerini gördük. Şimdi bu değerleri nasıl yazdığımızı (literals) ve onlarla nasıl işlem yaptığımızı inceleyelim.

---

## 4. Literaller ve Operatörler (Literals and Operators)

Bir değerin kaynak kodda yazılış biçimine **literal** denir. Rust, sayısal literallerde esnek gösterimler sunar.

### 4.1. Tam Sayı Literalleri

- **Ondalık**: `123` veya `1_000_000` (alt çizgi okunabilirlik içindir, derleyici yok sayar)
- **Onaltılık (hex)**: `0xff` (255)
- **Sekizlik (octal)**: `0o77` (63)
- **İkilik (binary)**: `0b1111_0000` (240)
- **Byte literal (sadece u8)**: `b'A'` (65)

İsteğe bağlı olarak **tip soneki** ekleyerek türü netleştirebilirsiniz:
```rust
let a = 42u8;         // u8 tipinde 42
let b = 1_000i64;     // i64 tipinde 1000
let c = 0xffu16;      // u16 tipinde 255
```

### 4.2. Kayan Noktalı Literaller

`f32` veya `f64` soneki kullanılabilir. Bilimsel gösterim de desteklenir:

```rust
let d = 3.14f32;      // f32 tipinde
let e = 2.5e-3;       // 0.0025, varsayılan f64
let f = 1E6f64;       // f64 tipinde 1.000.000
```

### 4.3. Boolean ve Karakter Literalleri

Boolean: `true`, `false`
Karakter: `'A'`, `'🦀'`, `'\n'` (kaçış dizileri desteklenir)

### 4.4. Operatörler

Rust, C benzeri operatörleri destekler:

- **Aritmetik**: `+`, `-`, `*`, `/`, `%` (mod alma)
- **Karşılaştırma**: `==`, `!=`, `<`, `>`, `<=`, `>=`
- **Mantıksal**: `&&` (ve), `||` (veya), `!` (değil)
- **Bit düzeyinde**: `&`, `|`, `^`, `<<`, `>>`

> **Önemli**: Aritmetik operatörlerde türler uyuşmalıdır. Rust otomatik tür dönüşümü yapmaz; örneğin `i32` ile `u8`’i toplamak isterseniz dönüşümü açıkça yapmalısınız.

```rust
let toplam = 5u8 + 10u8;       // 15u8
let fark = 10i32 - 3i32;       // 7i32
let carpim = 3.0 * 4.0;        // 12.0 (f64)
let kalan = 10 % 3;            // 1
let dogru = (5 > 2) && (1 == 1); // true
```

---

## 5. Demetler (Tuples)

Demetler, **farklı türden** birden çok değeri tek bir bileşik tür altında gruplamaya yarar. Parantez `()` ile oluşturulur.

```rust
let demet: (i32, f64, char) = (42, 3.14, 'π');
```

### 5.1. Elemanlara Erişim

İki yöntem vardır:

1. **Nokta indeksleme**: `demet.0`, `demet.1` ... (sıfırdan başlar)
2. **Destructuring (yapı çözümü)**:
```rust
let (x, y, z) = demet;
println!("x = {}, y = {}, z = {}", x, y, z);
```

İhtiyaç duyulmayan elemanlar `_` ile atlanabilir:
```rust
let (a, _, c) = demet;
```

### 5.2. Birim (Unit) Tipi `()`

Hiç elemanı olmayan özel bir demet türüne **unit** denir ve `()` ile gösterilir. Hem tipi hem de değeri aynı şekilde yazılır. Genellikle bir fonksiyonun geri dönüş değeri yoksa döndürdüğü tiptir (void karşılığı).

```rust
let bos: () = ();
fn hicbir_se_yapma() -> () {
    // ...
}
// ya da kısaca:
fn hicbir_se_yapma() {}
```

Unit tipinin boyutu sıfırdır, bellekte yer kaplamaz.

### 5.3. Demetleri Fonksiyonlarda Kullanma

Demetler, bir fonksiyondan birden fazla değer döndürmek için idealdir:

```rust
fn boyutlar() -> (u32, u32) {
    (1920, 1080)
}
let (en, yukseklik) = boyutlar();
```

---

## 6. Diziler ve Dilimler (Arrays and Slices)

### 6.1. Diziler (Arrays)

Dizi, **aynı türden** sabit sayıda elemanı bellekte ardışık olarak tutan bir koleksiyondur. Derleme zamanında boyutu bellidir ve stack’te yaşar.

Tanımlama biçimleri:
```rust
let dizi1: [i32; 5] = [1, 2, 3, 4, 5];
let dizi2 = [3; 4];        // [3, 3, 3, 3] -> 4 elemanlı, hepsi 3
let dizi3: [u8; 3] = [0; 3]; // [0, 0, 0]
```

- `dizi2`’deki `[3; 4]` sözdizimi, `3` değerini `4` kez tekrarlar.
- Diziye indeksle `dizi1[0]` şeklinde erişilir. Sınırlar aşılırsa **panic** oluşur (çalışma zamanı hatası).

### 6.2. Dilimler (Slices)

Dilim, bir koleksiyonun (dizi veya vektör gibi) bir bölümüne erişmemizi sağlayan **bir görünümdür (view)**. Bir dilim, verinin kendisine sahip değildir; bellekteki orijinal veriyi referans alır. Dilimin tipi `&[T]` biçimindedir (örneğin `&[i32]`).

Bir diziden dilim almak:
```rust
let dizi = [10, 20, 30, 40, 50];
let dilim: &[i32] = &dizi[1..4];   // 20, 30, 40 (indeks 1 dahil, 4 hariç)
```

Dilim aralıkları `başlangıç..bitiş` (bitiş hariç) sözdizimiyle verilir. 
- `..` tüm diziyi kapsar
- `2..` indeks 2’den sona kadar
- `..3` baştan indeks 3’e kadar (hariç)

### 6.3. Dizi ve Dilim Metotları

Diziler de dilimler de bazı ortak metotlara sahiptir. En sık kullanılanlar:

- `len()`: eleman sayısını döndürür.
- `is_empty()`: boş mu?
- `first()`, `last()`: ilk/son elemanı `Option` ile döner.

```rust
let d = [1, 2, 3];
assert_eq!(d.len(), 3);
println!("İlk eleman: {:?}", d.first()); // Some(1)
```

### 6.4. String Dilimi (`&str`)

String verileri de aslında bir tür dilimdir. `str` tipi, bellekte UTF-8 kodlanmış bir string dilimidir. Genelde `&str` şeklinde ödünç alınmış referans olarak kullanılır. String literal’lerin tipi `&str`’dir:

```rust
let selam: &str = "Merhaba Rust!";
```

String dilimleri de indislenebilir, ancak dilim alırken **UTF-8 karakter sınırlarına** dikkat edilmelidir, aksi halde program panic verir.

```rust
let s = "Rustacean";
let parcasi = &s[0..4];   // "Rust"
```

---

## 7. Kapanış – Pratik Uygulama ve Önemli Noktalar

Şimdiye kadar gördüklerimizi birleştiren küçük bir örnek yapalım:

```rust
fn main() {
    // Skalerler
    let yas: u8 = 30;
    let agirlik: f64 = 72.5;
    let evli: bool = false;
    let kan_grubu: char = 'A';

    // Literal çeşitleri ve operatörler
    let mesafe_km = 1_500u32;
    let hex_renk = 0xFF5733u32;
    let toplam_yas = yas + 5; // 35 (u8, ama tür dönüşümü yok)

    // Demet
    let kisi: (&str, u8, f64, bool, char) = ("Ali", yas, agirlik, evli, kan_grubu);
    let (isim, _, _, medeni_durum, _) = kisi;
    println!("{} evli mi? {}", isim, medeni_durum);

    // Dizi
    let notlar = [85, 92, 78, 90];
    let ilk_iki: &[i32] = &notlar[..2];
    println!("İlk iki not: {:?}", ilk_iki);

    // String dilimi
    let rust_mottosu: &str = "Hızlı, güvenli, üretken.";
    println!("Motto: {}", &rust_mottosu[0..5]); // "Hızlı"
}
```

### Unutulmaması Gerekenler

1. Rust’ta türler katıdır; otomatik genişletme/daraltma yoktur. İhtiyaç halinde `as` ile dönüşüm yapılır (ileride detaylandıracağız).
2. Diziler sabit uzunlukludur; dinamik diziler için `Vec<T>` kullanılır (standart kütüphane konusu).
3. Dilimler sahiplik almaz; bu sayede bellek yönetimi güvendedir.
4. Demetler farklı türleri bir arada tutabilirken, diziler homojendir.
5. Tüm bu ilkel tipler stack’te yaşar, bu da performansı artırır.

Bu bölümle birlikte Rust’ın temel veri yapılarını ve onların nasıl kullanılacağını derinlemesine öğrenmiş olduk. Bundan sonraki konularda (kontrol akışı, fonksiyonlar, sahiplik) bu yapıları sürekli kullanacağız. Eliniz alışana kadar küçük denemeler yapmanızı tavsiye ederim. Bir sonraki derste görüşmek üzere!
