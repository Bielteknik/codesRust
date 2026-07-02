# 🎓 Rust'ta Test Etme (Testing) — Kapsamlı Ders

## 📚 Giriş: Rust Neden Test Etmeye Bu Kadar Önem Veriyor?

Rust, **"doğruluk" (correctness)** kavramına son derece önem veren bir programlama dilidir. Derleyicisi (compiler) zaten birçok hatayı derleme zamanında yakalar, ancak kodun *mantıksal doğruluğunu* yalnızca testler kanıtlayabilir. Rust bu yüzden test yazmayı dilin içine yerleşik bir özellik olarak sunar.

Rust'ta **üç temel test türü** vardır:

| Test Türü | Amaç | Kapsam |
|---|---|---|
| **Unit Testing** (Birim Testi) | Tek bir modülü/fonksiyonu izole şekilde test etmek | Küçük, iç (private) kodu test edebilir |
| **Doc Testing** (Belge Testi) | Dokümantasyondaki kod örneklerinin çalıştığını doğrulamak | Kullanım örneklerini test eder |
| **Integration Testing** (Entegrasyon Testi) | Birden fazla modülün birlikte doğru çalıştığını test etmek | Büyük, yalnızca açık (public) arayüzü kullanır |

Ayrıca testler için özel bağımlılıklar (dev-dependencies) tanımlama desteği de vardır.

Şimdi her birini tek tek, bol örneklerle inceleyelim. 🚀

---

## 1️⃣ Unit Testing (Birim Testleri)

### 🎯 Nedir?
Birim testleri, **en küçük testable birimleri** — genellikle tek bir fonksiyonu — izole bir ortamda test eder. Küçük, hızlı ve özeldirler; hatta **private (özel) fonksiyonları bile test edebilirler.**

### 📐 Temel Yapı

Rust'ta birim testleri şu kalıpla yazılır:

```rust
#[cfg(test)]      // ← Bu modül SADECE test derlemesinde dahil edilir
mod tests {
    use super::*;  // ← Üst kapsamdaki isimleri içeri aktar

    #[test]        // ← Bu bir test fonksiyonudur
    fn test_something() {
        // Test kodu buraya
    }
}
```

> 💡 **`#[cfg(test)]` ne işe yarar?** Bu öznitelik (attribute), derleyiciye "bu modülü yalnızca `cargo test` çalıştırıldığında derle" der. Normal derlemede (`cargo build`) bu kod derlenmez, böylece production binary'nize test kodu karışmaz.

### 🛠️ Kullanılan Makrolar

Rust testlerde kullanmak üzere üç temel makro sunar:

| Makro | Açıklama |
|---|---|
| `assert!(ifade)` | İfade `false` ise panic oluşturur (test başarısız olur) |
| `assert_eq!(sol, sağ)` | İki değerin **eşit** olduğunu kontrol eder |
| `assert_ne!(sol, sağ)` | İki değerin **eşit olmadığını** kontrol eder |

> 💡 `assert_eq!` ve `assert_ne!` makroları, test başarısız olduğunda her iki değeri de yazdırır, bu da hata ayıklamayı çok kolaylaştırır. Bu makrolar `PartialEq` trait'ini gerektirir.

### 📝 Örnek: Basit Birim Testleri

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Kötü bir toplama fonksiyonu — örnek olsun diye bilerek yanlış yapıldı
#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    use super::*;  // Üst kapsamdaki add ve bad_add'i içeri aktar

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);  // ✅ Geçer
    }

    #[test]
    fn test_bad_add() {
        // Private fonksiyonlar da test edilebilir!
        assert_eq!(bad_add(1, 2), 3);  // ❌ Başarısız olur (1-2 = -1, 3 değil)
    }
}
```

### 😱 Panik Bekleyen Testler: `#[should_panic]`

Bazen bir fonksiyonun **belirli durumlarda panic oluşturmasını** beklersiniz (örneğin sıfıra bölme). İşte bu durumda `#[should_panic]` özniteliğini kullanırsınız:

```rust
pub fn divide_non_zero_result(a: u32, b: u32) -> u32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    } else if a < b {
        panic!("Divide result is zero");
    }
    a / b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide() {
        assert_eq!(divide_non_zero_result(10, 2), 5);
    }

    #[test]
    #[should_panic]  // ← Bu testin panic ile başarısız olmasını BEKLİYORUZ
    fn test_any_panic() {
        divide_non_zero_result(1, 0);  // Sıfıra bölme → panic
    }

    #[test]
    #[should_panic(expected = "Divide result is zero")]
    // ↑ Sadece herhangi bir panic değil, BELİRLİ bir panic mesajı bekliyoruz
    fn test_specific_panic() {
        divide_non_zero_result(1, 10);  // 1 < 10 → "Divide result is zero" panic'i
    }

    #[test]
    #[should_panic = "Divide result is zero"]  // ← Kısa gösterim (aynı işlev)
    fn test_specific_panic_shorthand() {
        divide_non_zero_result(1, 10);
    }
}
```

> 🎓 **İpucu:** `expected` parametresi, panic mesajının **içerdiğini** kontrol eder (tam eşleşme değil). Fonksiyonunuz birden fazla şekilde panic oluşturabiliyorsa, doğru panic'i test ettiğinizden emin olmak için bu parametreyi mutlaka kullanın!

### 🏃 Testleri Çalıştırma

```bash
# Tüm testleri çalıştır
cargo test

# Belirli bir testi çalıştır (isim bazlı filtreleme)
cargo test test_add

# İsminde "add" geçen tüm testleri çalıştır
cargo test add

# Yoksayılan testleri de çalıştır
cargo test -- --ignored

# Testlerin stdout çıktısını göster (başarılı testlerde varsayılan olarak gizlenir)
cargo test -- --nocapture
```

### ⏭️ Testleri Yoksayma: `#[ignore]`

Bazı testler yavaştır veya nadiren çalıştırılmaları gerekir. Bunları `#[ignore]` ile işaretleyebilirsiniz:

```rust
#[test]
#[ignore]  // ← Bu test varsayılan olarak çalıştırılmaz
fn expensive_test() {
    // Uzun süren bir test...
}
```

Bu testi çalıştırmak için: `cargo test -- --ignored`

---

## 2️⃣ Doc Testing (Belge Testleri)

### 🎯 Nedir?
Rust'ta dokümantasyon, kaynak kodun içine **yorum satırları** olarak yazılır. Bu yorumlar **CommonMark Markdown** formatındadır ve içlerinde **kod blokları** barındırabilir. İşte Rust'ın büyülü yanı: **bu kod blokları otomatik olarak derlenir ve test edilir!**

Yani dokümantasyonunuzdaki örnekler aslında birer testtir. Dokümanınız güncellenmezse kod değiştiğinde test başarısız olur — böylece **dokümantasyonunuz asla eskimez.**

### 📐 Temel Yapı

```rust
/// Fonksiyonun kısa özeti buraya yazılır.
///
/// Detaylı açıklama buraya. Kod blokları üç ters tırnak ile başlar:
///
/// ```
/// let sonuc = crate_adi::fonksiyon(2, 3);
/// assert_eq!(sonuc, 5);
/// ```
pub fn fonksiyon(a: i32, b: i32) -> i32 {
    a + b
}
```

> 💡 Kod bloklarının içine örtük olarak bir `fn main()` ve `extern crate <cratename>` yerleştirilir.

### 📝 Örnek: Bölümlerle Zenginleştirilmiş Belge Testleri

Rust dokümantasyonunda genellikle üç bölüm bulunur: **Examples**, **Panics**, **Failures**.

````rust
/// İlk satır kısa özettir.
///
/// Sonraki satırlar detaylı dokümantasyonu içerir.
///
/// # Examples
///
/// ```
/// let result = playground::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// İki sayıyı bölen fonksiyon.
///
/// # Examples
///
/// ```
/// let result = playground::div(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// # Panics
///
/// İkinci argüman sıfır ise panic oluşturur.
///
/// ```rust,should_panic
/// // Sıfıra bölmede panic oluşur
/// playground::div(10, 0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }
    a / b
}
````

### 🏃 Belge Testlerini Çalıştırma

```bash
cargo test
```

Çıktı şöyle görünür:

```
   Doc-tests playground

running 3 tests
test src/lib.rs - add (line 7) ... ok
test src/lib.rs - div (line 21) ... ok
test src/lib.rs - div (line 31) ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### 🔮 Gelişmiş: Gizli Satırlar ve `?` Operatörü

Bazen dokümantasyonda göstermek istemediğiniz ama derlenmesi gereken kod satırları olur. Örneğin `Result` döndüren bir fonksiyon için `?` operatörünü kullanmak istersiniz, ancak örtük `main()` fonksiyonu `()` döndürür — bu yüzden `?` kullanamazsınız.

Çözüm: **Gizli satırlar** (`#` ile başlayan satırlar) kullanmak!

````rust
/// Gizli `try_main` kullanarak doc testleri.
///
/// ```
/// # // Gizli satırlar `#` ile başlar, derlenirler ama dokümanda görünmezler!
/// # fn try_main() -> Result<(), String> {
/// let res = playground::try_div(10, 2)?;
/// # Ok(())
/// # }
/// # fn main() {
/// #    try_main().unwrap();
/// # }
/// ```
pub fn try_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Divide-by-zero"))
    } else {
        Ok(a / b)
    }
}
````

> 🎓 **`#` kuralı:** Doküman yorumlarında `# ` ile başlayan satırlar derlenir ama oluşturulan HTML dokümantasyonunda **görünmez**. Böylece test altyapısını gizleyip, kullanıcıya yalnızca temiz örneği gösterebilirsiniz.

### 🎨 Kod Bloğu Nitelikleri

Kod bloklarına çeşitli nitelikler ekleyebilirsiniz:

| Nitelik | Anlamı |
|---|---|
| ````rust` | Sözdizimi vurgulama (Rust olarak) |
| ````rust,should_panic` | Kodun panic oluşturması beklenir |
| ````rust,no_run` | Kod derlenir ama çalıştırılmaz |
| ````rust,ignore` | Kod ne derlenir ne çalıştırılır |
| ````text` | Kod değildir, sadece metin |

---

## 3️⃣ Integration Testing (Entegrasyon Testleri)

### 🎯 Nedir?
Entegrasyon testleri, **birim testlerinin tam tersidir:**

- **Birim testleri:** Küçük, izole, private kodu test eder
- **Entegrasyon testleri:** Crate'inizin **dışında** yaşar, yalnızca **public arayüzü** kullanır — tıpkı gerçek bir kullanıcının kütüphanenizi kullanacağı gibi

Amaçları, kütüphanenizin **farklı parçalarının birlikte doğru çalıştığını** doğrulamaktır.

### 📁 Dizin Yapısı

Cargo, entegrasyon testlerini `src` klasörünün yanındaki **`tests`** klasöründe arar:

```
projem/
├── Cargo.toml
├── src/
│   └── lib.rs
└── tests/              ← Entegrasyon testleri burada!
    └── integration_test.rs
```

### 📝 Örnek

**`src/lib.rs`:**
```rust
// `adder` adlı bir crate olduğunu varsayalım
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

**`tests/integration_test.rs`:**
```rust
#[test]
fn test_add() {
    // Dışarıdan bir kullanıcı gibi crate'i kullanıyoruz
    assert_eq!(adder::add(3, 2), 5);
}
```

**Çalıştırma:**
```bash
$ cargo test

running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/integration_test-bcd60824f5fbfe19

running 1 test
test test_add ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests adder

running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

> 💡 Dikkat edin: Cargo her bir testi **ayrı bir crate** olarak derler. Bu yüzden her test dosyası kendi `main()` fonksiyonuna sahiptir.

### 🔗 Ortak Kod Paylaşımı

Birden fazla entegrasyon test dosyanız varsa ve bunların **ortak kurulum (setup) kodunu** paylaşması gerekiyorsa ne yaparsınız?

Çözüm: **`tests/common/mod.rs`** yapısı!

**`tests/common/mod.rs`:**
```rust
pub fn setup() {
    // Gerekli dosyaları oluşturma, sunucuları başlatma vb.
}
```

**`tests/integration_test.rs`:**
```rust
mod common;  // ← common modülünü içeri aktar

#[test]
fn test_add() {
    common::setup();  // ← Ortak kurulum kodunu kullan
    assert_eq!(adder::add(3, 2), 5);
}
```

> ⚠️ **Önemli Uyarı:** Paylaşılan modülü `tests/common.rs` olarak oluşturursanız, Cargo bunu **bir test crate'i** olarak algılar ve içinde test aramaya çalışır. Bu yüzden **mutlaka `tests/common/mod.rs`** yapısını kullanın!

### 📁 Gelişmiş Dizin Yapısı

Büyük projelerde şu yapı yaygındır:

```
tests/
├── common/
│   └── mod.rs          ← Paylaşılan yardımcı kodlar
├── api_tests.rs        ← API entegrasyon testleri
├── db_tests.rs         ← Veritabanı entegrasyon testleri
└── cli_tests.rs        ← CLI entegrasyon testleri
```

---

## 4️⃣ Dev-Dependencies (Geliştirme Bağımlılıkları)

### 🎯 Nedir?
Bazen testlerinizde, production kodunuzun ihtiyaç duymadığı **ek kütüphanelere** ihtiyaç duyarsınız. Örneğin:
- **`proptest`** gibi property-based test kütüphaneleri
- **`mockall`** gibi mock oluşturma kütüphaneleri
- **`tempfile`** gibi geçici dosya/dizin oluşturma araçları

Bu bağımlılıkları normal `[dependencies]` bölümüne eklerseniz, production binary'niz gereksiz yere şişer. Bunun yerine **`[dev-dependencies]`** bölümünü kullanırsınız:

### 📝 `Cargo.toml` Örneği

```toml
[package]
name = "projem"
version = "0.1.0"

[dependencies]
serde = "1.0"           # Production'da da kullanılır

[dev-dependencies]
tempfile = "3"          # SADECE testlerde kullanılır
proptest = "1.0"        # SADECE testlerde kullanılır
```

> 💡 `[dev-dependencies]`'deki kütüphaneler yalnızca `cargo test`, `cargo build --examples` gibi geliştirme komutları çalıştırıldığında derlenir. Normal `cargo build` veya `cargo install` sırasında derlenmezler.

---

## 🧩 Özet Karşılaştırma Tablosu

| Özellik | Unit Test | Doc Test | Integration Test |
|---|---|---|---|
| **Konum** | `src/` içinde, `#[cfg(test)]` modülü | Kaynak kodundaki `///` yorumları | `tests/` klasörü |
| **Erişim** | Private + Public kod | Public API | Yalnızca Public API |
| **Hız** | Çok hızlı | Hızlı | Yavaş olabilir |
| **Amaç** | Tek bir birimin doğruluğu | Dokümantasyonun doğruluğu | Parçaların birlikte çalışması |
| **Çalıştırma** | `cargo test` | `cargo test` | `cargo test` |
| **Kime hitap eder?** | Geliştiriciye | Kütüphane kullanıcısına | Son kullanıcıya/sisteme |

---

## 🎓 Altın Kurallar

1. **🔬 Birim testleri yazın** — her public fonksiyon için en az bir test
2. **📖 Dokümantasyonunuzu test edin** — örnekleriniz çalışır durumda kalmalı
3. **🔗 Entegrasyon testleri yazın** — özellikle kütüphane geliştiriyorsanız
4. **🚀 Testlerinizi sık çalıştırın** — `cargo test` refleksiniz olmalı
5. **🙈 Yavaş testleri `#[ignore]` ile işaretleyin** — CI/CD'de ayrı çalıştırın
6. **📦 Dev-dependencies kullanın** — production binary'nizi şişirmeyin

---

## 🏁 Son Söz

Rust'ın test ekosistemi, dilin felsefesiyle mükemmel uyumludur: **"Çalışan kod, doğru koddur."** Derleyici tip güvenliğini sağlarken, testler de mantıksal doğruluğu sağlar. Bu üç test türünü birlikte kullanarak, **hem kendinizin hem de kullanıcılarınızın güvenebileceği** sağlam yazılımlar geliştirebilirsiniz.

> 🎯 **Unutmayın:** Test yazmak bir yük değil, **yatırımdır.** Bugün yazdığınız her test, yarın karşılaşacağınız bir bug'ı önler.

Başarılar! 🦀✨

# DeepSeek yorumu --------------

# Rust'ta Test Etme (Testing)

Rust, doğruluğa (correctness) büyük önem veren bir programlama dilidir ve bu nedenle yazılım testlerini doğrudan dilin kendi bünyesinde destekler. Rust'ta test yazmak, kodunuzun beklendiği gibi çalıştığından emin olmanın en temel yoludur. Testler, hataları erken yakalamanıza, kodunuzu yeniden düzenlerken (refactor) güvenle ilerlemenize ve kod tabanınızın kalitesini artırmanıza yardımcı olur.

Rust'ta testler üç ana stile ayrılır:

1. **Birim Testleri (Unit Testing)**
2. **Dokümantasyon Testleri (Doc Testing)**
3. **Entegrasyon Testleri (Integration Testing)**

Ayrıca Rust, testler için ek bağımlılıklar belirleme desteği de sunar. Şimdi bu başlıkları tek tek detaylandıralım.

---

## 1. Birim Testleri (Unit Testing)

Birim testleri, kodunuzun küçük parçalarının (genellikle tek bir fonksiyon veya modül) doğru çalışıp çalışmadığını doğrulayan test fonksiyonlarıdır. Birim testleri genellikle:

- Bazı hazırlık işlemleri yapar (setup),
- Test edilmek istenen kodu çalıştırır,
- Sonuçların beklenen değerlerle uyuşup uyuşmadığını kontrol eder (assertion).

### Test Modülü ve Nitelikler (Attributes)

Birim testleri, genellikle `tests` adlı bir modül içine yazılır ve bu modül `#[cfg(test)]` niteliği ile işaretlenir. Bu nitelik, derleyiciye bu modülün sadece test modunda derlenmesi gerektiğini söyler. Böylece test kodları, normal derleme sırasında devreye girmez ve üretim ikili dosyalarının (binary) boyutunu artırmaz.

Test fonksiyonlarının üzerine ise `#[test]` niteliği eklenir. Bu nitelik, Rust'a bu fonksiyonun bir test olduğunu ve `cargo test` komutuyla çalıştırılması gerektiğini belirtir.

### Assertion Makroları (Assert Macros)

Rust, testlerde kullanılmak üzere bazı yardımcı makrolar sunar:

- **`assert!(expression)`**: Verilen ifade `false` olarak değerlendiğinde testi başarısız kılar (panik oluşturur).
- **`assert_eq!(left, right)`**: Soldaki ve sağdaki ifadelerin eşit olup olmadığını kontrol eder. Eşit değillerse test başarısız olur.
- **`assert_ne!(left, right)`**: Soldaki ve sağdaki ifadelerin eşit *olmadığını* kontrol eder. Eşitlerse test başarısız olur.

### Örnek: Basit Bir Birim Testi

Aşağıda, iki sayıyı toplayan bir fonksiyon ve ona ait birim testleri gösterilmektedir:

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Bu fonksiyon kasıtlı olarak hatalıdır, testin başarısız olmasını göstermek için.
#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    // Dış kapsamdaki (super) isimleri test modülüne getirir.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_bad_add() {
        // Bu assert tetiklenir ve test başarısız olur.
        // Özel fonksiyonlar da test edilebilir!
        assert_eq!(bad_add(1, 2), 3);
    }
}
```

Testleri `cargo test` komutu ile çalıştırabiliriz. Çıktı, hangi testlerin geçtiğini, hangilerinin başarısız olduğunu ve başarısızlık durumunda hangi satırda panik oluştuğunu gösterir:

```shell
$ cargo test
running 2 tests
test tests::test_bad_add ... FAILED
test tests::test_add ... ok

failures:

---- tests::test_bad_add stdout ----
thread 'tests::test_bad_add' panicked at 'assertion failed: `(left == right)`
  left: `-1`,
 right: `3`', src/lib.rs:21:8
note: Run with `RUST_BACKTRACE=1` for a backtrace.

failures:
    tests::test_bad_add

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

### Testler ve `?` Operatörü

Rust 2018 sürümünden itibaren, birim testleri `Result<()>` döndürebilir. Bu sayede test fonksiyonları içinde `?` operatörünü kullanarak hata yönetimini daha kısa ve okunabilir hale getirebiliriz.

```rust
fn sqrt(number: f64) -> Result<f64, String> {
    if number >= 0.0 {
        Ok(number.powf(0.5))
    } else {
        Err("negatif sayıların karekökü yoktur".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt() -> Result<(), String> {
        let x = 4.0;
        assert_eq!(sqrt(x)?.powf(2.0), x);
        Ok(())
    }
}
```

Bu yaklaşım, özellikle `?` operatörünün kullanılabildiği durumlarda test kodlarını daha temiz hale getirir.

### Panik Testleri (Testing Panics)

Bazı fonksiyonlar belirli koşullar altında panik yapmak üzere tasarlanmıştır. Bu tür fonksiyonları test etmek için `#[should_panic]` niteliğini kullanırız.

```rust
pub fn divide_non_zero_result(a: u32, b: u32) -> u32 {
    if b == 0 {
        panic!("Sıfıra bölme hatası");
    } else if a < b {
        panic!("Bölüm sonucu sıfır");
    }
    a / b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Sıfıra bölme hatası")]
    fn test_divide_by_zero() {
        divide_non_zero_result(10, 0);
    }

    #[test]
    #[should_panic(expected = "Bölüm sonucu sıfır")]
    fn test_divide_result_zero() {
        divide_non_zero_result(5, 10);
    }
}
```

`#[should_panic]` niteliğine `expected` parametresi vererek, panik mesajının belirli bir metni içermesini zorunlu kılabiliriz. Bu, fonksiyonun farklı şekillerde panik yapabildiği durumlarda, testin doğru panik türünü yakaladığından emin olmamızı sağlar. Kısa yazım şekli olan `#[should_panic = "mesaj"]` da kullanılabilir.

---

## 2. Dokümantasyon Testleri (Documentation Testing)

Rust projelerini belgelemenin birincil yolu, kaynak koduna açıklama satırları eklemektir. Dokümantasyon yorumları [CommonMark Markdown](https://commonmark.org/) formatında yazılır ve kod bloklarını destekler. Rust, doğruluğa verdiği önem nedeniyle, bu kod bloklarını derler ve dokümantasyon testleri olarak kullanır.

### Temel Kullanım

Dokümantasyon yorumları `///` ile başlar. İçlerine yazılan Markdown kod blokları, `cargo test` çalıştırıldığında otomatik olarak test edilir.

```rust
/// İlk satır, fonksiyonun kısa bir özetidir.
///
/// Sonraki satırlar detaylı dokümantasyon içerir.
/// Kod blokları üç ters tırnak ile başlar ve içlerinde otomatik olarak `fn main()` bulunur.
///
/// ```
/// let result = playground::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Yukarıdaki örnekte, dokümantasyon içindeki kod bloğu `cargo test` ile test edilecektir.

### Örnekler, Panikler ve Başarısızlıklar

Dokümantasyon yorumları genellikle "Examples" (Örnekler), "Panics" (Panik Durumları) ve "Failures" (Başarısızlık Durumları) gibi bölümler içerir.

```rust
/// Bu fonksiyon iki sayıyı böler.
///
/// # Örnekler
///
/// ```
/// let result = playground::div(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// # Panikler
///
/// İkinci argüman sıfır ise fonksiyon panik yapar.
///
/// ```rust,should_panic
/// // sıfıra bölme paniğe yol açar
/// playground::div(10, 0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Sıfıra bölme hatası");
    }
    a / b
}
```

Burada dikkat edilmesi gereken nokta, ikinci kod bloğunun `rust,should_panic` olarak işaretlenmiş olmasıdır. Bu, testin panik yapmasının beklendiğini belirtir.

`cargo test` komutu çalıştırıldığında, hem normal testler hem de dokümantasyon testleri birlikte çalıştırılır:

```shell
$ cargo test
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests playground
running 3 tests
test src/lib.rs - add (line 7) ... ok
test src/lib.rs - div (line 21) ... ok
test src/lib.rs - div (line 31) ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Dokümantasyon Testlerinin Arkasındaki Motivasyon

Dokümantasyon testlerinin temel amacı, API'nin nasıl kullanılacağını gösteren çalışan örnekler sunmaktır. Bu örneklerin derlenebilir ve çalışabilir olması, dokümantasyonun her zaman güncel ve doğru kalmasını sağlar.

Ancak, bazen örneklerde `?` operatörü kullanmak isteyebiliriz. `?` operatörü bir fonksiyonun `Result` döndürmesini gerektirir, ancak dokümantasyon testlerinde kod bloğu otomatik olarak `fn main()` içine sarıldığı için `Result` döndüremez. Bu sorunu aşmak için, kodun bazı satırlarını dokümantasyonda gizleyip (`#` ile başlayan satırlar) gerçek test kodunu çalıştırabiliriz.

```rust
/// Doküman testlerinde gizli `try_main` kullanımı.
///
/// ```
/// # // '#' ile başlayan satırlar dokümanda gizlenir, ancak derlenmeye devam eder!
/// # fn try_main() -> Result<(), String> {
/// let res = playground::try_div(10, 2)?;
/// # Ok(())
/// # }
/// # fn main() {
/// # try_main().unwrap();
/// # }
/// ```
pub fn try_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Sıfıra bölme"))
    } else {
        Ok(a / b)
    }
}
```

Bu teknik, örneklerde `?` kullanımına izin verirken, dokümantasyonun okunabilirliğini korur.

---

## 3. Entegrasyon Testleri (Integration Testing)

Birim testleri, modülleri tek başına ve izole bir şekilde test ederken, entegrasyon testleri, crate'inizin dışından, yani yalnızca genel (public) arayüzünü kullanarak test eder. Amaçları, kütüphanenizin birçok parçasının birlikte doğru çalışıp çalışmadığını kontrol etmektir.

### Entegrasyon Testlerinin Yapısı

Cargo, entegrasyon testlerini `src` dizininin yanında bulunan `tests` dizini içinde arar.

Örnek bir kütüphane (`adder`):

`src/lib.rs`:
```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Entegrasyon testi dosyası: `tests/integration_test.rs`:
```rust
#[test]
fn test_add() {
    assert_eq!(adder::add(3, 2), 5);
}
```

`cargo test` ile çalıştırıldığında, Cargo entegrasyon testlerini ayrı bir süreçte derler ve çalıştırır:

```shell
$ cargo test
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

Running target/debug/deps/integration_test-bcd60824f5fbfe19
running 1 test
test test_add ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests adder
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Entegrasyon Testleri Arasında Kod Paylaşımı

Her Rust kaynak dosyası `tests` dizininde ayrı bir crate olarak derlenir. Bu nedenle, testler arasında ortak kod paylaşmak istediğimizde, `tests/common/mod.rs` gibi bir modül oluşturabiliriz.

`tests/common/mod.rs`:
```rust
pub fn setup() {
    // Bazı hazırlık işlemleri: dosya/klasör oluşturma, sunucu başlatma vb.
}
```

`tests/integration_test.rs`:
```rust
// common modülünü içe aktar
mod common;

#[test]
fn test_add() {
    common::setup();
    assert_eq!(adder::add(3, 2), 5);
}
```

`tests/common.rs` dosyası da çalışır, ancak test çalıştırıcısı bu dosyayı da bir test sandığı olarak algılayıp içindeki testleri çalıştırmaya çalışacağı için önerilmez. Bu nedenle `common/mod.rs` yapısı tercih edilmelidir.

---

## 4. Geliştirme Bağımlılıkları (Development Dependencies)

Bazen testler (veya örnekler, benchmark'lar) için yalnızca geliştirme aşamasında ihtiyaç duyulan bağımlılıklar olabilir. Bu tür bağımlılıklar `Cargo.toml` dosyasında `[dev-dependencies]` bölümü altında belirtilir. Bu bağımlılıklar, paketinize bağımlı olan diğer paketlere aktarılmaz.

Örnek olarak, standart `assert_eq!` ve `assert_ne!` makrolarını renkli farklar (diff) gösterecek şekilde genişleten [`pretty_assertions`](https://docs.rs/pretty_assertions/) kütüphanesini ele alalım.

`Cargo.toml`:
```toml
[dev-dependencies]
pretty_assertions = "1"
```

`src/lib.rs`:
```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq; // Bu crate yalnızca testlerde kullanılır.

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
```

`pretty_assertions` yalnızca test modunda kullanıldığı için, normal derlemede bu bağımlılık devreye girmez ve üretim kodunu etkilemez.

---

## Özet ve Sonuç

Rust'ta test etme, dilin doğruluk odaklı felsefesinin merkezinde yer alır. Üç ana test türü olan birim testleri, dokümantasyon testleri ve entegrasyon testleri, farklı düzeylerde ve farklı amaçlarla kod kalitesini güvence altına alır:

- **Birim Testleri**: Küçük, izole ve hızlı; özel (private) kodları da test edebilir.
- **Dokümantasyon Testleri**: API dokümantasyonundaki örneklerin çalıştığından emin olur.
- **Entegrasyon Testleri**: Crate'in genel arayüzünü dışarıdan test eder; birden çok parçanın birlikte çalışmasını doğrular.

Ayrıca, `[dev-dependencies]` ile testler için özel bağımlılıklar tanımlayabilir, böylece üretim bağımlılıklarını gereksiz yere şişirmeden test sürecini zenginleştirebiliriz.

Bu üç test türünü birlikte kullanmak, Rust projelerinde sağlam, güvenilir ve iyi belgelenmiş bir kod tabanı oluşturmanın temel taşlarından biridir.

### Ek Kaynaklar

- [Rust Kitabı (The Book) - Test Etme Bölümü](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [API Kılavuzları - Dokümantasyon Testleri](https://rust-lang-nursery.github.io/api-guidelines/documentation.html)