# Rust'ta Attribute (Öznitelik) - Kapsamlı Ders

Merhaba! Bu derste Rust programlama dilinin çok güçlü özelliklerinden biri olan **Attribute (Öznitelik)** sistemini detaylıca öğreneceğiz. Bu konu, Rust'ın esnekliğini ve derleyiciyle iletişim kurma yeteneğini gösteren önemli bir konsepttir.

---

## 📚 Bölüm 1: Attribute Nedir?

### Tanım

**Attribute (öznitelik)**, Rust'ta bir modül, crate (kasa/paket) veya herhangi bir öğe (item) üzerine uygulanan **meta verilerdir**. 

Meta veri dediğimiz şey, kodun kendisi değil ama kod hakkında derleyiciye bilgi veren ek açıklamalardır. Tıpkı bir kitapta sayfa kenarına yazılmış notlar gibi düşünebilirsiniz - kitabın içeriği değil ama okuyucuya (bu durumda derleyiciye) ek bilgi verir.

### Attribute'lar Ne İşe Yarar?

Attribute'lar şu amaçlarla kullanılabilir:

| Amaç | Açıklama |
|------|----------|
| **Koşullu derleme** | Belirli koşullara göre kodun derlenip derlenmemesini kontrol etme |
| **Crate bilgileri** | Crate adı, versiyonu ve tipini (binary/library) belirleme |
| **Lint'leri kapatma** | Derleyici uyarılarını devre dışı bırakma |
| **Compiler özellikleri** | Makrolar, glob import gibi özellikleri aktifleştirme |
| **Foreign library** | Harici (C gibi) kütüphanelere bağlanma |
| **Test işaretleme** | Fonksiyonları birim testi olarak işaretleme |
| **Benchmark** | Performans test fonksiyonlarını işaretleme |
| **Makro benzeri** | Attribute-like makrolar oluşturma |

### Attribute Sözdizimi

Rust'ta iki tür attribute vardır:

#### 1️⃣ Dış Attribute: `#[outer_attribute]`

Bu attribute, **hemen ardından gelen öğeye** uygulanır.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
```

Burada `#[derive(Debug)]` sadece `Rectangle` struct'ına uygulanır.

**Nerelerde kullanılır?**
- Fonksiyonlar
- Modül bildirimleri
- Sabitler (constants)
- Struct'lar
- Enum'lar

#### 2️⃣ İç Attribute: `#![inner_attribute]`

Bu attribute, **içinde bulunduğu kapsama** (genellikle modül veya crate) uygulanır.

```rust
#![allow(unused_variables)]

fn main() {
    let x = 3; // Normalde kullanılmayan değişken uyarısı verir
              // Ama #![allow(unused_variables)] bu uyarıyı kapatır
}
```

Burada `#![allow(unused_variables)]` tüm crate'e (veya modüle) uygulanır.

**Farkı anlayalım:**
- `#[...]` → Sadece bir sonraki öğeye
- `#![...]` → İçinde bulunduğu tüm kapsama

### Attribute Argümanları

Attribute'lar farklı sözdizimleriyle argüman alabilir:

```rust
// 1. Eşittir işareti ile
#[attribute = "value"]

// 2. Parantez içinde key-value
#[attribute(key = "value")]

// 3. Parantez içinde sadece değer
#[attribute(value)]

// 4. Birden fazla değer (virgülle ayrılmış)
#[attribute(value1, value2)]

// 5. Çok satıra yayılabilir
#[attribute(value1, value2, value3,
            value4, value5)]
```

---

## 📚 Bölüm 2: Crate Öznitelikleri

Bu bölümde crate seviyesinde kullanılan attribute'ları öğreneceğiz.

### `crate_type` Attribute'u

Bu attribute, derleyiciye crate'in bir **binary** (çalıştırılabilir dosya) mı yoksa **library** (kütüphane) mi olduğunu söyler.

```rust
// Bu crate bir kütüphanedir
#![crate_type = "lib"]

// Kütüphanenin adı "rary" olacak
#![crate_name = "rary"]

pub fn public_function() {
    println!("rary'nin public_function()'ı çağrıldı");
}

fn private_function() {
    println!("rary'nin private_function()'ı çağrıldı");
}

pub fn indirect_access() {
    print!("rary'nin indirect_access()'i çağrıldı, ");
    private_function();
}
```

### `crate_name` Attribute'u

Bu attribute, crate'in adını belirler.

### ⚠️ Önemli Not: Cargo ile Kullanım

Çok önemli bir nokta: **`crate_type` ve `crate_name` attribute'ları Cargo kullanırken HİÇBİR ETKİYE sahip değildir.**

Rust projelerinin çoğu Cargo kullandığı için, bu attribute'ların gerçek dünya kullanımı oldukça sınırlıdır.

Cargo kullanıyorsanız, bu bilgileri `Cargo.toml` dosyasında belirlersiniz:

```toml
[package]
name = "benim_projem"
version = "0.1.0"

[lib]
name = "benim_kutuphanem"
crate-type = ["lib"]
```

### Ne Zaman Kullanılır?

Bu attribute'lar sadece **doğrudan `rustc`** (Rust derleyicisi) ile derleme yaparken kullanışlıdır:

```shell
# Normalde rustc'ye crate tipini söylemek gerekir
$ rustc --crate-type lib lib.rs

# Ama crate_type attribute'u kullanırsanız gerekmez
$ rustc lib.rs
$ ls lib*
rary.rlib
```

---

## 📚 Bölüm 3: `cfg` - Koşullu Derleme

Bu bölüm, attribute sisteminin en güçlü kullanımlarından biridir: **Koşullu Derleme**.

### İki Farklı Yöntem

Rust'ta yapılandırma koşulları iki farklı operatörle kontrol edilebilir:

| Yöntem | Sözdizimi | Kullanım Yeri |
|--------|-----------|---------------|
| `cfg` attribute | `#[cfg(...)]` | Attribute pozisyonunda |
| `cfg!` makrosu | `cfg!(...)` | Boolean ifadelerde |

### Temel Fark

- **`#[cfg(...)]`** → Koşullu derleme sağlar. Koşul sağlanmazsa kod **tamamen derlenmez** ve binary'ye dahil edilmez.
- **`cfg!(...)`** → Çalışma zamanında (runtime) `true` veya `false` döner. Kod **her zaman derlenir**, sadece değer değişir.

### Örnek: İşletim Sistemi Kontrolü

```rust
// Bu fonksiyon SADECE hedef işletim sistemi Linux ise derlenir
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("Linux kullanıyorsun!");
}

// Bu fonksiyon SADECE hedef işletim sistemi Linux DEĞİLSE derlenir
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("Linux kullanmıyorsun!");
}

fn main() {
    are_you_on_linux();
    
    println!("Emin misin?");
    
    // cfg! makrosu runtime'da çalışır
    if cfg!(target_os = "linux") {
        println!("Evet. Kesinlikle Linux!");
    } else {
        println!("Evet. Kesinlikle Linux DEĞİL!");
    }
}
```

### Yaygın `cfg` Koşulları

Rust, birçok yerleşik koşul sağlar:

```rust
// İşletim sistemi
#[cfg(target_os = "linux")]
#[cfg(target_os = "windows")]
#[cfg(target_os = "macos")]

// Mimari
#[cfg(target_arch = "x86_64")]
#[cfg(target_arch = "aarch64")]

// Derleme profili
#[cfg(debug_assertions)]  // Debug build
#[cfg(not(debug_assertions))]  // Release build

// Feature flag'ler
#[cfg(feature = "ozellik_adi")]
```

### `cfg!` Makrosunun Önemli Özelliği

`cfg!` makrosu kodu kaldırmaz, sadece `true` veya `false` döner. Bu yüzden:

```rust
// Her iki dal da geçerli Rust kodu olmalı
if cfg!(target_os = "linux") {
    println("Linux");  // ✓ Geçerli
} else {
    println("Not Linux");  // ✓ Geçerli
}
```

Ama `#[cfg]` ile:

```rust
// Koşul sağlanmazsa bu kod DERLENMEZ
#[cfg(target_os = "linux")]
{
    println("Linux");
}
// Eğer Linux değilse, bu kod binary'de hiç olmaz
```

---

## 📚 Bölüm 4: Özel (Custom) cfg Koşulları

### Yerleşik vs Özel Koşullar

`target_os` gibi bazı koşullar `rustc` tarafından otomatik sağlanır. Ancak **özel koşullar** için derleyiciye `--cfg` flag'i ile bilgi vermeniz gerekir.

### Örnek

```rust
#[cfg(some_condition)]
fn conditional_function() {
    println!("Koşul sağlandı!");
}

fn main() {
    conditional_function();
}
```

### Derleme

**Özel cfg flag'i olmadan:**
```shell
$ rustc custom.rs
$ ./custom
# Hata! conditional_function bulunamadı
# Çünkü some_condition sağlanmadı, fonksiyon derlenmedi
```

**Özel cfg flag'i ile:**
```shell
$ rustc --cfg some_condition custom.rs
$ ./custom
Koşul sağlandı!
```

### Pratik Kullanım

Bu özellik, özellikle şu durumlarda kullanışlıdır:

1. **Feature flag'ler** - Farklı özellik setlerini açma/kapama
2. **Environment-specific kod** - Geliştirme, test, production ortamları
3. **Platform-specific optimizasyonlar** - Farklı donanımlar için optimize edilmiş kod

```rust
// Cargo.toml'da tanımlanan feature'lar
#[cfg(feature = "advanced_logging")]
fn advanced_log() {
    // Sadece "advanced_logging" feature'ı aktifse derlenir
}

#[cfg(feature = "experimental")]
fn experimental_feature() {
    // Deneysel özellik
}
```

---

## 📚 Bölüm 5: `dead_code` ve Lint'leri Kapatma

### Lint Nedir?

**Lint**, derleyicinin kodunuzdaki potansiyel sorunları tespit etmesine yardımcı olan bir kontroldür. Rust derleyicisi birçok lint'e sahiptir ve varsayılan olarak birçok uyarı verir.

### `dead_code` Lint'i

`dead_code` lint'i, **kullanılmayan fonksiyonlar** hakkında uyarı verir.

```rust
fn used_function() {
    println!("Bu fonksiyon kullanılıyor");
}

#[allow(dead_code)]
fn unused_function() {
    println!("Bu fonksiyon kullanılmıyor ama uyarı vermeyecek");
}

fn noisy_unused_function() {
    println!("Bu fonksiyon da kullanılmıyor ve uyarı verecek");
}

fn main() {
    used_function();
}
```

**Çıktı:**
```
warning: function is never used: `noisy_unused_function`
```

### `#[allow(dead_code)]` Kullanımı

`#[allow(dead_code)]` attribute'u, belirli bir öğe için `dead_code` uyarısını kapatır.

**Neden kullanılır?**
- Kütüphane geliştirme: Public API'de henüz kullanılmayan ama gelecekte kullanılabilecek fonksiyonlar
- Test kodları: Test yardımcı fonksiyonları
- Örnek kodlar: Eğitim materyallerinde

### Diğer Yaygın Lint Attribute'ları

```rust
// Tüm uyarıları kapat
#![allow(warnings)]

// Belirli bir uyarıyı kapat
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
#[allow(non_snake_case)]

// Uyarıyı hata'ya yükselt
#[deny(warnings)]

// Uyarıyı bir not olarak işaretle
#[warn(unused_variables)]
```

### ⚠️ Önemli Uyarı

**Gerçek programlarda, dead code'u (ölü kodu) ortadan kaldırmalısınız.** 

Eğer bir fonksiyon gerçekten kullanılmıyorsa, onu silmek en iyi pratiktir. `#[allow(dead_code)]` sadece istisnai durumlarda kullanılmalıdır.

---

## 📚 Bölüm 6: Pratik Örnekler ve Kullanım Senaryoları

### Örnek 1: Platform-Specific Kod

```rust
#[cfg(target_os = "windows")]
fn get_path_separator() -> &'static str {
    "\\"
}

#[cfg(not(target_os = "windows"))]
fn get_path_separator() -> &'static str {
    "/"
}

fn main() {
    println!("Path separator: {}", get_path_separator());
}
```

### Örnek 2: Debug/Release Farklı Davranışlar

```rust
#[cfg(debug_assertions)]
fn log(message: &str) {
    println!("[DEBUG] {}", message);
}

#[cfg(not(debug_assertions))]
fn log(_message: &str) {
    // Release build'de logging kapalı
}

fn main() {
    log("Uygulama başlatıldı");
}
```

### Örnek 3: Feature Flag'ler

```rust
// Cargo.toml'da:
// [features]
// experimental = []

#[cfg(feature = "experimental")]
fn experimental_feature() {
    println!("Deneysel özellik aktif!");
}

#[cfg(not(feature = "experimental"))]
fn experimental_feature() {
    println!("Deneysel özellik kapalı.");
}
```

### Örnek 4: Test ve Benchmark İşaretleme

```rust
// Birim testi olarak işaretle
#[test]
fn test_addition() {
    assert_eq!(2 + 2, 4);
}

// Panic bekleyen test
#[test]
#[should_panic(expected = "index out of bounds")]
fn test_panic() {
    let v = vec![1, 2, 3];
    v[10]; // Panic oluşturacak
}

// Benchmark olarak işaretle
#[bench]
fn bench_something(b: &mut test::Bencher) {
    b.iter(|| {
        // Benchmark yapılacak kod
    });
}
```

### Örnek 5: Derive Makroları

```rust
// Debug, Clone, PartialEq trait'lerini otomatik türet
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1.clone();
    
    println!("{:?}", p1);  // Debug
    println!("Equal: {}", p1 == p2);  // PartialEq
}
```

### Örnek 6: Foreign Function Interface (FFI)

```rust
// Harici C kütüphanesine bağlanma
#[link(name = "m")]  // Math kütüphanesi
extern "C" {
    fn sqrt(x: f64) -> f64;
    fn pow(x: f64, y: f64) -> f64;
}

fn main() {
    unsafe {
        println!("sqrt(16) = {}", sqrt(16.0));
        println!("pow(2, 3) = {}", pow(2.0, 3.0));
    }
}
```

---

## 📚 Bölüm 7: Attribute Makroları

Attribute'lar aynı zamanda makro gibi de çalışabilir. Bunlara **attribute-like macros** denir.

### Örnek: Özel Attribute Makrosu

```rust
// Bu, bir procedural macro'dur
// Ayrı bir crate'te tanımlanması gerekir
use my_macros::my_attribute;

#[my_attribute]
fn my_function() {
    println!("Bu fonksiyon özel bir attribute'a sahip");
}
```

Attribute makroları, derleme zamanında kodu değiştirmek veya eklemek için kullanılır. Bu ileri seviye bir konudur ve procedural macro yazmayı gerektirir.

---

## 📚 Bölüm 8: En İyi Pratikler

### ✅ Yapılması Gerekenler

1. **Cargo kullanın** - `crate_type` ve `crate_name` yerine `Cargo.toml` kullanın
2. **Feature flag'leri** - Büyük projelerde özellikleri açmak/kapatmak için kullanın
3. **Platform kodu** - Farklı işletim sistemleri için `#[cfg]` kullanın
4. **Test işaretleri** - `#[test]` ve `#[should_panic]` kullanın
5. **Derive makroları** - Trait implementasyonlarını otomatikleştirin

### ❌ Yapılmaması Gerekenler

1. **Gereksiz `allow` kullanımı** - Uyarıları kapatmak yerine kodu düzeltin
2. **Karmaşık cfg koşulları** - Çok fazla koşullu kod okunabilirliği azaltır
3. **Dead code bırakma** - Kullanılmayan kodu silin
4. **Global allow kullanımı** - `#![allow(warnings)]` tüm uyarıları kapatır, dikkatli kullanın

---

## 📚 Bölüm 9: Özet ve Hızlı Referans

### Attribute Türleri

| Tür | Sözdizimi | Kapsam |
|-----|-----------|--------|
| Dış Attribute | `#[attr]` | Sadece bir sonraki öğe |
| İç Attribute | `#![attr]` | İçinde bulunduğu kapsam |

### Yaygın Attribute'lar

```rust
// Derive (Trait türetme)
#[derive(Debug, Clone, PartialEq)]

// Koşullu derleme
#[cfg(target_os = "linux")]
#[cfg(feature = "ozellik")]
#[cfg(debug_assertions)]

// Lint kontrolü
#[allow(dead_code)]
#[allow(unused_variables)]
#[deny(warnings)]

// Test
#[test]
#[should_panic(expected = "mesaj")]
#[bench]

// Crate bilgileri (Cargo kullanmıyorsanız)
#![crate_type = "lib"]
#![crate_name = "isim"]

// FFI
#[link(name = "kutuphane")]
extern "C" { }
```

### cfg! vs #[cfg]

| Özellik | `#[cfg(...)]` | `cfg!(...)` |
|---------|---------------|-------------|
| Tip | Attribute | Makro |
| Zaman | Derleme zamanı | Çalışma zamanı |
| Kod | Koşul sağlanmazsa derlenmez | Her zaman derlenir |
| Dönüş | - | `bool` |
| Kullanım | Fonksiyon, struct vb. | `if` ifadeleri |

---

## 🎯 Sonuç

Attribute'lar, Rust'ın derleyiciyle iletişim kurmanın güçlü bir yoludur. Doğru kullanıldığında:

- ✅ Platform-specific kod yazabilirsiniz
- ✅ Feature flag'leri yönetebilirsiniz
- ✅ Test ve benchmark'ları işaretleyebilirsiniz
- ✅ Derleyici uyarılarını kontrol edebilirsiniz
- ✅ Kodunuzu daha esnek ve sürdürülebilir hale getirebilirsiniz

Ancak unutmayın: **Güçlü araçlar sorumluluk gerektirir.** Attribute'ları dikkatli ve amaçlı kullanın, gereksiz karmaşıklıktan kaçının.

Bir sonraki derslerde procedural macro yazmayı ve özel attribute makroları oluşturmayı öğrenebilirsiniz. Şimdilik bu temel attribute'ları pratik yaparak pekiştirmenizi öneririm!

---

**Sorularınız varsa sormaktan çekinmeyin!** 🚀

# DeepSeek Yorumu ------------------
# Rust'ta Attribute'ler (Ek Özellikler) - Kapsamlı Rehber

Rust'ta **attribute'ler** (nitelikler/ek özellikler), modüllere, crate'lere veya diğer öğelere (item) uygulanan **meta verilerdir** (metadata). Bu meta veriler, derleyiciye ek bilgi sağlayarak kodun nasıl davranacağını, derleneceğini veya test edileceğini kontrol etmenizi sağlar.

---

## 1. Attribute'ler Ne İşe Yarar?

Attribute'ler çeşitli amaçlar için kullanılır:

| Amaç | Açıklama |
|------|----------|
| **Koşullu derleme** | Belli platformlarda veya koşullarda kodun derlenmesini sağlamak |
| **Crate bilgileri** | Crate adı, sürümü ve türünü (binary veya library) belirlemek |
| **Lint'leri kapatmak** | Derleyici uyarılarını (warnings) devre dışı bırakmak |
| **Derleyici özellikleri** | Makrolar, glob import'lar gibi özellikleri etkinleştirmek |
| **Dış kütüphane bağlantısı** | Foreign (C/C++) kütüphanelere bağlanmak |
| **Test ve benchmark** | Fonksiyonları birim test veya benchmark olarak işaretlemek |
| **Attribute benzeri makrolar** | Özel attribute benzeri makrolar tanımlamak |

---

## 2. İki Tür Attribute: Dış ve İç

Attribute'ler iki farklı sözdizimiyle yazılır:

### 2.1 Dış Attribute: `#[...]`

**`#[attribute]`** şeklinde yazılır ve **kendisinden hemen sonra gelen öğeye (item)** uygulanır.

Bir öğe (item), Rust'ta şunlardan biri olabilir:
- Bir fonksiyon
- Bir modül bildirimi
- Bir sabit (constant)
- Bir struct (yapı)
- Bir enum

**Örnek - `#[derive(Debug)]`**:
```rust
#[derive(Debug)]  // Bu attribute, hemen altındaki struct'a uygulanır
struct Rectangle {
    width: u32,
    height: u32,
}
```
Bu örnekte `#[derive(Debug)]`, `Rectangle` struct'ına `Debug` özelliğini otomatik olarak türetmesini söyler.

### 2.2 İç Attribute: `#![...]`

**`#![attribute]`** şeklinde yazılır ve **içinde bulunduğu kapsama (enclosing item)** uygulanır. Tipik olarak bir modül veya crate'in tamamına etki eder.

**Örnek - `#![allow(unused_variables)]`**:
```rust
#![allow(unused_variables)]  // Bu attribute, tüm crate'e uygulanır

fn main() {
    let x = 3;  // Normalde kullanılmayan değişken uyarısı verir,
                // ancak bu attribute sayesinde uyarı bastırılır
}
```
Bu örnekte `#![allow(unused_variables)]`, tüm crate boyunca kullanılmayan değişken uyarılarını devre dışı bırakır.

---

## 3. Attribute'lere Argüman Verme

Attribute'ler farklı sözdizimleriyle argüman alabilir:

| Sözdizimi | Örnek | Açıklama |
|-----------|-------|----------|
| `#[attribute = "value"]` | `#[crate_name = "my_app"]` | Tek bir değer ataması |
| `#[attribute(key = "value")]` | `#[cfg(target_os = "linux")]` | Anahtar-değer çifti |
| `#[attribute(value)]` | `#[derive(Debug)]` | Doğrudan değer |

Birden fazla argüman da verilebilir ve satırlara bölünebilir:
```rust
#[attribute(value, value2)]
#[attribute(value, value2, value3, value4, value5)]
```

---

## 4. Koşullu Derleme: `cfg`

`cfg` attribute'ü, kodun **yalnızca belirli koşullar altında derlenmesini** sağlar. İki şekilde kullanılır:

### 4.1 `#[cfg(...)]` - Attribute Olarak

Derleme **zamanında** koşulu değerlendirir ve koşul sağlanmıyorsa ilgili kodu **tamamen kaldırır**.

```rust
// Yalnızca hedef işletim sistemi Linux ise derlenir
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

// Yalnızca hedef işletim sistemi Linux DEĞİL ise derlenir
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!");
}

fn main() {
    are_you_on_linux();  // Hangi fonksiyonun çağrılacağı derleme zamanında bellidir
}
```

### 4.2 `cfg!` - Makro Olarak

`cfg!(...)` makrosu, **çalışma zamanında** `true` veya `false` değeri döndürür. Koşul sağlanmasa bile tüm kod blokları geçerli olmalıdır.

```rust
fn main() {
    are_you_on_linux();
    println!("Are you sure?");

    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}
```

### 4.3 Özel Koşullar (`--cfg`)

Rustc, `target_os` gibi bazı koşulları otomatik olarak sağlar. Ancak **özel koşullar** tanımlamak isterseniz, derleme sırasında `--cfg` bayrağını kullanmalısınız.

```rust
// custom.rs
#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!");
}

fn main() {
    conditional_function();
}
```

Bu dosyayı özel koşulla derlemek için:
```bash
$ rustc --cfg some_condition custom.rs && ./custom
# Çıktı: condition met!
```

Eğer `--cfg` bayrağı olmadan derlerseniz, `conditional_function` derlenmez ve çağrılamaz.

---

## 5. Crate Ayarları: `crate_type` ve `crate_name`

Bu attribute'ler, crate'in türünü ve adını belirlemek için kullanılır.

### 5.1 `crate_type`

Crate'in **binary mi yoksa library mi** olduğunu belirtir.

```rust
// Bu crate bir library'dir
#![crate_type = "lib"]

// Library'nin adı "rary" olarak ayarlanır
#![crate_name = "rary"]

pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");
    private_function();
}
```

Bu attribute'ler kullanıldığında, `rustc`'ye `--crate-type` bayrağını geçmeye gerek kalmaz:
```bash
$ rustc lib.rs
$ ls lib*
library.rlib   # Çıktı: .rlib uzantılı library dosyası
```

### 5.2 Önemli Uyarı

**`crate_type` ve `crate_name` attribute'leri, Cargo kullanıldığında hiçbir etki göstermez**. Rust projelerinin büyük çoğunluğu Cargo kullandığı için, bu attribute'lerin gerçek dünya kullanımı sınırlıdır. Cargo'da bu ayarlar `Cargo.toml` dosyası üzerinden yapılır.

---

## 6. Diğer Yaygın Attribute'ler

Rust'da sıkça kullanılan bazı attribute'ler:

| Attribute | Kullanımı |
|-----------|-----------|
| `#[derive(...)]` | Belirtilen trait'leri otomatik türetir (Debug, Clone, Copy, PartialEq, vb.) |
| `#[test]` | Fonksiyonu birim test olarak işaretler |
| `#[should_panic]` | Testin paniklemesi beklendiğini belirtir |
| `#[allow(...)]` / `#[deny(...)]` | Lint seviyelerini kontrol eder |
| `#[macro_export]` | Makroları dışa aktarır |
| `#[repr(...)]` | Veri düzenini (memory layout) belirler |
| `#[inline]` / `#[inline(always)]` | Fonksiyonun inline edilmesini önerir |

---

## 7. Özet

| Konu | Açıklama |
|------|----------|
| **Dış Attribute (`#[...]`)** | Hemen altındaki öğeye uygulanır |
| **İç Attribute (`#![...]`)** | Bulunduğu kapsama (modül/crate) uygulanır |
| **`cfg`** | Koşullu derleme sağlar (`#[cfg]` derleme zamanı, `cfg!` çalışma zamanı) |
| **`crate_type` / `crate_name`** | Crate türünü ve adını belirler (Cargo'da etkisiz) |
| **Özel koşullar** | `rustc --cfg` ile tanımlanır |

Attribute'ler, Rust'ın güçlü meta-programlama araçlarından biridir. Kodunuzu daha esnek, taşınabilir ve okunabilir hale getirmek için doğru attribute'leri doğru yerde kullanmak önemlidir.