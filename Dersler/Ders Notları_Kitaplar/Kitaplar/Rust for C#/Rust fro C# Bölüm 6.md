# 🧪 Rust for .NET Developers: Testing (Test Yazma)

Bu bölüm, Rust'ta test yazmanın temellerinden ileri düzey tekniklere kadar kapsamlı bir rehberdir. C# dünyasında xUnit, NUnit, MSTest gibi framework'ler ve Visual Studio Test Explorer gibi araçlarla test yazmaya alışkınız. Rust ise **derleyiciye gömülü**, harici framework gerektirmeyen minimal ama güçlü bir test sistemi sunar.

> 🎯 **Temel Fark:** C#'ta test yazmak için **harici framework** (xUnit, NUnit) ve **paket** gerekir. Rust'ta test sistemi **standart kütüphanenin bir parçasıdır** - `cargo test` komutu her şeyi yönetir.

---

# 📚 BÖLÜM 1: İlk Testinizi Yazmak

## 1.1 C# Yaklaşımı (xUnit)

**C#**:
```csharp
// Ayrı bir Test projesi gerekir
// xUnit NuGet paketi yüklenmeli
using Xunit;

public class HesapMakinesiTestleri
{
    [Fact]
    public void Toplama_DogruCalisir()
    {
        var hesap = new HesapMakinesi();
        var sonuc = hesap.Topla(2, 3);
        Assert.Equal(5, sonuc);
    }
}
```

## 1.2 Rust Yaklaşımı

**Rust**:
```rust
// Herhangi bir paket gerekmez!
fn topla(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]  // Bu modül sadece test derlemesinde dahil edilir
mod testler {
    use super::*;  // Üst modülden fonksiyonları al
    
    #[test]
    fn toplama_dogru_calisir() {
        assert_eq!(topla(2, 3), 5);
    }
}
```

> 💡 **Kritik Fark:** Rust'ta testler **aynı dosyada** veya `tests/` klasöründe olabilir. Ayrı bir test projesi açmanıza gerek yoktur!

## 1.3 Test Çalıştırma

```bash
# Tüm testleri çalıştır
cargo test

# Sadece belirli bir testi çalıştır
cargo test toplama_dogru_calisir

# Detaylı çıktı
cargo test -- --nocapture

# Tek thread'de çalıştır (sıralı)
cargo test -- --test-threads=1
```

---

# 📚 BÖLÜM 2: Assert Makroları

Rust'ın assert makroları, C#'taki `Assert` sınıfının karşılığıdır.

## 2.1 Temel Assert Makroları

| Rust | C# (xUnit) | Açıklama |
|---|---|---|
| `assert!(koşul)` | `Assert.True(koşul)` | Koşul doğru olmalı |
| `assert_eq!(a, b)` | `Assert.Equal(a, b)` | İki değer eşit olmalı |
| `assert_ne!(a, b)` | `Assert.NotEqual(a, b)` | İki değer farklı olmalı |
| `assert!(cond, "mesaj")` | `Assert.True(cond, "mesaj")` | Özel hata mesajı |

## 2.2 Örnek Kullanımlar

```rust
#[cfg(test)]
mod testler {
    #[test]
    fn temel_assertlar() {
        // Boolean kontrol
        assert!(5 > 3);
        assert!(!false);
        
        // Eşitlik kontrolü
        assert_eq!(10, 5 + 5);
        assert_eq!("merhaba", "merhaba");
        
        // Farklılık kontrolü
        assert_ne!(10, 20);
        
        // Özel hata mesajı
        let sonuc = 42;
        assert!(sonuc > 0, "Sonuç pozitif olmalı, ama {} bulundu", sonuc);
    }
    
    #[test]
    #[should_panic]  // Testin panic yapması BEKLENİR
    fn sifira_bolme_panic_yapar() {
        let x = 5;
        let y = 0;
        let _sonuc = x / y;  // Bu satır panic yapmalı
    }
    
    #[test]
    #[should_panic(expected = "divide by zero")]  // Belirli bir mesaj beklenir
    fn belirli_panic_mesaji() {
        let _sonuc = 1 / 0;
    }
}
```

> ⚠️ **Dikkat:** `assert_eq!` ve `assert_ne!`, karşılaştırılan türlerin `PartialEq` ve `Debug` trait'lerini implemente etmesini gerektirir.

---

# 📚 BÖLÜM 3: Test Organizasyonu

## 3.1 Unit Testler (Aynı Dosyada)

Rust'ta unit testler, test edilen kod ile **aynı dosyada** tutulur:

```rust
// src/lib.rs
pub struct HesapMakinesi;

impl HesapMakinesi {
    pub fn topla(&self, a: i32, b: i32) -> i32 {
        a + b
    }
    
    pub fn bol(&self, a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err("Sıfıra bölme hatası".to_string())
        } else {
            Ok(a / b)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_topla() {
        let h = HesapMakinesi;
        assert_eq!(h.topla(2, 3), 5);
    }
    
    #[test]
    fn test_bol_basarili() {
        let h = HesapMakinesi;
        assert_eq!(h.bol(10, 2), Ok(5));
    }
    
    #[test]
    fn test_bol_hatali() {
        let h = HesapMakinesi;
        assert!(h.bol(10, 0).is_err());
    }
}
```

> 💡 **`#[cfg(test)]` Anlamı:** Bu attribute, modülün sadece `cargo test` çalıştırıldığında derlenmesini sağlar. Normal derlemede (`cargo build`) bu kod binary'ye dahil edilmez - bu da production binary'sinin boyutunu küçültür.

## 3.2 Integration Testler (Ayrı Dosyada)

Integration testler, kütüphanenizi **dışarıdan bir kullanıcı gibi** test eder. `tests/` klasöründe tutulurlar:

```
my_project/
├── src/
│   └── lib.rs
├── tests/
│   ├── integration_test.rs
│   └── another_test.rs
└── Cargo.toml
```

**tests/integration_test.rs**:
```rust
use my_project::HesapMakinesi;  // Kütüphaneyi dışarıdan kullan

#[test]
fn integration_topla() {
    let h = HesapMakinesi;
    assert_eq!(h.topla(100, 200), 300);
}

#[test]
fn integration_bol() {
    let h = HesapMakinesi;
    assert_eq!(h.bol(100, 5), Ok(20));
}
```

> 🎯 **Kritik Fark:** 
> - **Unit testler:** Modülün iç yapısına erişebilir (private fonksiyonlar test edilebilir)
> - **Integration testler:** Sadece public API'yi test edebilir

## 3.3 C# vs Rust Test Organizasyonu

| Özellik | C# (xUnit) | Rust |
|---|---|---|
| Test projesi | Ayrı proje | Aynı crate veya `tests/` klasörü |
| Test keşfi | Reflection | `#[test]` attribute |
| Setup/Teardown | `IDisposable`, fixtures | `setup` fonksiyonu veya struct |
| Test kategorileri | `[Trait]`, `[Category]` | `#[ignore]`, modül yapısı |
| Private test | Internal test projesi | Aynı dosyada unit test |

---

# 📚 BÖLÜM 4: Test Filtreleme ve Kontrol

## 4.1 Test İsme Göre Filtreleme

```bash
# İsminde "topla" geçen tüm testler
cargo test topla

# Tam eşleşme
cargo test --exact test_topla

# Modül bazlı filtreleme
cargo test tests::test_topla
```

## 4.2 Ignore (Yoksay) Attribute

Bazı testler yavaş olabilir veya özel durumlar gerektirebilir:

```rust
#[test]
#[ignore]  // Bu test varsayılan olarak çalışmaz
fn yavas_test() {
    // Çok uzun süren bir test
    std::thread::sleep(std::time::Duration::from_secs(60));
    assert!(true);
}
```

```bash
# Sadece ignore edilmiş testleri çalıştır
cargo test -- --ignored

# Tüm testleri çalıştır (ignore dahil)
cargo test -- --include-ignored
```

## 4.3 Parallel vs Sequential Çalıştırma

Rust testleri **varsayılan olarak paralel** çalıştırır:

```bash
# Paralel çalıştır (varsayılan)
cargo test

# Tek thread'de çalıştır (sıralı)
cargo test -- --test-threads=1

# 4 thread kullan
cargo test -- --test-threads=4
```

> ⚠️ **Dikkat:** Testleriniz paylaşılan kaynakları (dosya, veritabanı) değiştiriyorsa, `--test-threads=1` kullanın veya her teste benzersiz kaynak verin.

## 4.4 Çıktı Kontrolü

```bash
# Başarılı testlerin çıktısını da göster
cargo test -- --show-output

# Panic mesajlarını gösterme
cargo test -- --quiet

# Sadece derle, çalıştırma
cargo test --no-run
```

---

# 📚 BÖLÜM 5: Test Setup ve Teardown

Rust'ta xUnit'taki `[Collection]` veya `[Trait]` gibi gelişmiş fixture sistemleri yoktur. Bunun yerine basit struct'lar kullanılır.

## 5.1 Setup Fonksiyonu

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Ortak setup fonksiyonu
    fn setup() -> TestContext {
        TestContext {
            db: Baglanti::new("test_db"),
            kullanici: Kullanici::test_kullanicisi(),
        }
    }
    
    struct TestContext {
        db: Baglanti,
        kullanici: Kullanici,
    }
    
    #[test]
    fn test_kullanici_olustur() {
        let ctx = setup();
        let yeni = ctx.db.kullanici_olustur(&ctx.kullanici);
        assert!(yeni.is_ok());
    }
    
    #[test]
    fn test_kullanici_sil() {
        let ctx = setup();
        let sonuc = ctx.db.kullanici_sil(ctx.kullanici.id);
        assert!(sonuc.is_ok());
    }
}
```

## 5.2 RAII ile Teardown

Rust'ın `Drop` trait'i, teardown işlemleri için mükemmeldir:

```rust
struct TestFixture {
    temp_dosya: String,
}

impl TestFixture {
    fn new() -> Self {
        let dosya = format!("/tmp/test_{}", rand::random::<u32>());
        std::fs::write(&dosya, "test verisi").unwrap();
        TestFixture { temp_dosya: dosya }
    }
}

impl Drop for TestFixture {
    fn drop(&mut self) {
        // Test bitince otomatik temizlik!
        let _ = std::fs::remove_file(&self.temp_dosya);
    }
}

#[test]
fn dosya_testi() {
    let fixture = TestFixture::new();
    let icerik = std::fs::read_to_string(&fixture.temp_dosya).unwrap();
    assert_eq!(icerik, "test verisi");
} // <- fixture burada drop edilir, dosya silinir
```

> 💡 **Güçlü Özellik:** Test panic yapsa bile `drop` çağrılır - bu, C#'taki `finally` bloğuna benzer ama daha güvenlidir.

---

# 📚 BÖLÜM 6: Doc Tests (Dokümantasyon Testleri)

Rust'ın **en sevilen** özelliklerinden biri: Dokümantasyon örnekleri **otomatik olarak test edilir**!

## 6.1 Doc Test Örneği

```rust
/// İki sayıyı toplar
///
/// # Examples
///
/// ```
/// use crate::topla;
/// assert_eq!(topla(2, 3), 5);
/// ```
///
/// Negatif sayılarla da çalışır:
///
/// ```
/// use crate::topla;
/// assert_eq!(topla(-1, 1), 0);
/// ```
pub fn topla(a: i32, b: i32) -> i32 {
    a + b
}
```

```bash
cargo test
# Doc testler de çalışır!
```

## 6.2 Doc Test Kontrolü

```rust
/// Bazı durumlarda doc test çalışmasını istemezsiniz:
///
/// ```ignore
/// // Bu kod derlenmez ama örnek olarak gösterilir
/// let x = bilinmeyen_fonksiyon();
/// ```
///
/// ```no_run
/// // Bu kod derlenir ama ÇALIŞTIRILMAZ
/// // (Örn: ağ bağlantısı gerektiriyorsa)
/// let baglanti = Baglanti::kur("example.com");
/// ```
///
/// ```should_panic
/// // Bu kodun panic yapması beklenir
/// panic!("bilerek panic");
/// ```
///
/// ```compile_fail
/// // Bu kodun DERLENMEMESİ beklenir
/// let x: i32 = "string";  // Tür hatası
/// ```
pub fn ornek_fonksiyon() {}
```

> 🎯 **Avantaj:** Dokümantasyonunuzun her zaman güncel kalmasını sağlar. Kod değişti ama doküman değişmediyse, doc test başarısız olur!

---

# 📚 BÖLÜM 7: Property-Based Testing

Geleneksel testlerde belirli değerlerle test yaparsınız. Property-based testing'de ise **özellikler** (properties) tanımlarsınız ve framework rastgele binlerce değer üretir.

## 7.1 Proptest Kütüphanesi

**Cargo.toml**:
```toml
[dev-dependencies]
proptest = "1.4"
```

**Kod**:
```rust
use proptest::prelude::*;

fn ters_cevir(s: &str) -> String {
    s.chars().rev().collect()
}

fn cift_cevir(s: &str) -> String {
    ters_cevir(&ters_cevir(s))
}

proptest! {
    #[test]
    fn cift_ters_orijinal_dondurur(s in ".*") {
        // Herhangi bir string için: ters(ters(s)) == s
        prop_assert_eq!(cift_cevir(&s), s);
    }
    
    #[test]
    fn uzunluk_degismez(s in ".*") {
        // Ters çevirme uzunluğu değiştirmez
        prop_assert_eq!(s.len(), ters_cevir(&s).len());
    }
}
```

> 💡 **Güçlü Özellik:** Proptest, hataya neden olan **en küçük girdiyi** otomatik olarak bulur (shrinking). Bu, edge case'leri bulmanın mükemmel bir yoludur.

## 7.2 Quickcheck Alternatifi

```toml
[dev-dependencies]
quickcheck = "1.0"
quickcheck_macros = "1.0"
```

```rust
use quickcheck_macros::quickcheck;

#[quickcheck]
fn toplama_degisme_ozelligi(a: i32, b: i32) -> bool {
    a + b == b + a  // Toplama değişme özelliği
}
```

---

# 📚 BÖLÜM 8: Mocking

Rust'ta mocking, C#'taki Moq veya NSubstitute gibi framework'lerden farklıdır. Trait'ler ve generic'ler kullanılır.

## 8.1 Trait-Based Mocking

```rust
// Gerçek trait
trait Veritabani {
    fn kullanici_getir(&self, id: i32) -> Option<Kullanici>;
}

// Gerçek implementasyon
struct GercekDB;
impl Veritabani for GercekDB {
    fn kullanici_getir(&self, id: i32) -> Option<Kullanici> {
        // Gerçek DB sorgusu
        todo!()
    }
}

// Test için mock
struct MockDB {
    dondurulecek: Option<Kullanici>,
}
impl Veritabani for MockDB {
    fn kullanici_getir(&self, _id: i32) -> Option<Kullanici> {
        self.dondurulecek.clone()
    }
}

// Test edilen fonksiyon - generic
fn kullanici_var<T: Veritabani>(db: &T, id: i32) -> bool {
    db.kullanici_getir(id).is_some()
}

#[test]
fn test_kullanici_var() {
    let mock = MockDB {
        dondurulecek: Some(Kullanici { ad: "Ali".to_string() }),
    };
    assert!(kullanici_var(&mock, 1));
    
    let mock_bos = MockDB { dondurulecek: None };
    assert!(!kullanici_var(&mock_bos, 999));
}
```

## 8.2 Mockall Kütüphanesi

Daha gelişmiş mocking için:

```toml
[dev-dependencies]
mockall = "0.12"
```

```rust
use mockall::*;

#[automock]
trait MailServisi {
    fn gonder(&self, kime: &str, konu: &str) -> Result<(), String>;
}

#[test]
fn test_mail_gonderimi() {
    let mut mock = MockMailServisi::new();
    mock.expect_gonder()
        .with(eq("ali@test.com"), eq("Merhaba"))
        .times(1)
        .returning(|_, _| Ok(()));
    
    let sonuc = mock.gonder("ali@test.com", "Merhaba");
    assert!(sonuc.is_ok());
}
```

## 8.3 C# vs Rust Mocking

| Özellik | C# (Moq) | Rust (Mockall) |
|---|---|---|
| Syntax | `mock.Setup(x => x.Method()).Returns()` | `mock.expect_method().returning()` |
| Doğrulama | `mock.Verify()` | `.times(n)` ile otomatik |
| Runtime vs Compile | Runtime reflection | Compile-time macro |
| Interface/Abstract | Interface veya virtual | Trait |

---

# 📚 BÖLÜM 9: Benchmark Testing

Performans testleri için Rust'ın yerleşik `test` feature'ı veyaCriterion kütüphanesi kullanılır.

## 9.1 Yerleşik Benchmark (Nightly)

```rust
#![feature(test)]

#[cfg(test)]
mod bench {
    extern crate test;
    use test::Bencher;
    
    #[bench]
    fn bench_topla(b: &mut Bencher) {
        b.iter(|| {
            let mut toplam = 0;
            for i in 0..1000 {
                toplam += i;
            }
            toplam
        });
    }
}
```

## 9.2 Criterion (Stable - Önerilen)

```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "benchmarklar"
harness = false
```

**benches/benchmarklar.rs**:
```rust
use criterion::{criterion_group, criterion_main, Criterion};
use my_project::topla;

fn bench_topla(c: &mut Criterion) {
    c.bench_function("topla 2+3", |b| {
        b.iter(|| topla(2, 3))
    });
}

fn bench_buyuk_toplama(c: &mut Criterion) {
    c.bench_function("1000 sayi toplami", |b| {
        b.iter(|| {
            (0..1000).sum::<i32>()
        })
    });
}

criterion_group!(benches, bench_topla, bench_buyuk_toplama);
criterion_main!(benches);
```

```bash
cargo bench
# HTML rapor: target/criterion/report/index.html
```

> 💡 **Avantaj:** Criterion, istatistiksel olarak anlamlı sonuçlar verir, regression detection yapar ve detaylı HTML raporlar üretir.

---

# 📚 BÖLÜM 10: Snapshot Testing

UI veya karmaşık çıktılar için snapshot testing kullanılır:

```toml
[dev-dependencies]
insta = "1.34"
```

```rust
use insta::assert_snapshot;

#[test]
fn test_rapor_olustur() {
    let rapor = rapor_olustur(&veriler);
    assert_snapshot!(rapor);  // İlk çalıştırmada snapshot oluşturur
}
```

```bash
cargo test
cargo insta review  # Snapshot'ları gözden geçir
```

---

# 📚 BÖLÜM 11: Test Coverage (Kapsam)

```bash
# cargo-tarpaulin ile coverage
cargo install cargo-tarpaulin
cargo tarpaulin --out Html

# cargo-llvm-cov ile (daha hızlı)
cargo install cargo-llvm-cov
cargo llvm-cov --html
```

**Hedef Coverage:**
- Unit testler: %80+
- Kritik kod (güvenlik, finans): %95+
- Integration testler: Ana senaryolar %100

---

# 📚 BÖLÜM 12: Embedded Testing (RP2354B için) 🎯

Step motor projeniz gibi embedded sistemlerde test yazmak özel teknikler gerektirir.

## 12.1 Host-Target Ayrımı

```rust
// Sadece host'ta çalışacak testler
#[cfg(test)]
#[cfg(not(target_arch = "arm"))]
mod host_tests {
    #[test]
    fn hareket_hesapla() {
        // RP2354B'ye ihtiyaç duymayan saf mantık
        let adim_sayisi = hesapla_adim(100.0, 1.8);
        assert_eq!(adim_sayisi, 200);
    }
}
```

## 12.2 Hardware Abstraction Layer (HAL) ile Test

```rust
// Trait ile donanım soyutlama
trait StepMotorDriver {
    fn adim_at(&mut self);
    fn yon_ayarla(&mut self, yon: Yon);
}

// Gerçek donanım implementasyonu (RP2354B)
#[cfg(target_arch = "arm")]
struct GercekMotor {
    step_pin: Pin<Output>,
    dir_pin: Pin<Output>,
}

#[cfg(target_arch = "arm")]
impl StepMotorDriver for GercekMotor {
    fn adim_at(&mut self) { /* GPIO toggle */ }
    fn yon_ayarla(&mut self, yon: Yon) { /* GPIO write */ }
}

// Mock implementasyon (test için)
struct MockMotor {
    adim_sayisi: usize,
    son_yon: Option<Yon>,
}

impl StepMotorDriver for MockMotor {
    fn adim_at(&mut self) { self.adim_sayisi += 1; }
    fn yon_ayarla(&mut self, yon: Yon) { self.son_yon = Some(yon); }
}

// Test edilebilir mantık
fn hareket_et<M: StepMotorDriver>(motor: &mut M, hedef: i32) {
    if hedef > 0 {
        motor.yon_ayarla(Yon::Ileri);
        for _ in 0..hedef { motor.adim_at(); }
    } else {
        motor.yon_ayarla(Yon::Geri);
        for _ in 0..-hedef { motor.adim_at(); }
    }
}

#[test]
fn test_ileri_hareket() {
    let mut motor = MockMotor { adim_sayisi: 0, son_yon: None };
    hareket_et(&mut motor, 100);
    assert_eq!(motor.adim_sayisi, 100);
    assert_eq!(motor.son_yon, Some(Yon::Ileri));
}

#[test]
fn test_geri_hareket() {
    let mut motor = MockMotor { adim_sayisi: 0, son_yon: None };
    hareket_et(&mut motor, -50);
    assert_eq!(motor.adim_sayisi, 50);
    assert_eq!(motor.son_yon, Some(Yon::Geri));
}
```

> 🎯 **Altın Kural:** Donanım bağımlı kodu trait'ler arkasına saklayın. Böylece mantığı host'ta test edebilirsiniz.

## 12.3 Hardware-in-the-Loop (HIL) Testing

Gelişmiş embedded testler için:

```rust
// defmt ve probe-rs ile gerçek donanımda test
#[defmt_test::tests]
mod tests {
    #[test]
    fn test_gpio_cikis() {
        let mut pin = gpio_pin();
        pin.set_high();
        assert!(pin.is_high());
    }
}
```

---

# 📚 BÖLÜM 13: Test Best Practices

## 13.1 ✅ İyi Pratikler

1. **AAA Pattern (Arrange-Act-Assert):**
```rust
#[test]
fn test_kullanici_olustur() {
    // Arrange
    let db = setup_db();
    let kullanici = Kullanici::yeni("Ali");
    
    // Act
    let sonuc = db.olustur(&kullanici);
    
    // Assert
    assert!(sonuc.is_ok());
}
```

2. **Test isimlendirmesi:**
```rust
// ✅ İyi: Ne test edildiği ve beklenen sonuç
#[test]
fn sifir_bolme_hata_dondurur() { }

#[test]
fn bos_liste_ekle_ilk_elemani_ekler() { }

// ❌ Kötü: Anlamsız isimler
#[test]
fn test1() { }
```

3. **Bir test, bir şeyi test etsin:**
```rust
// ❌ Kötü: Çok fazla assertion
#[test]
fn her_seyi_test_et() {
    assert_eq!(topla(1, 2), 3);
    assert_eq!(topla(-1, 1), 0);
    assert_eq!(topla(0, 0), 0);
    assert_eq!(topla(100, 200), 300);
}

// ✅ İyi: Her senaryo ayrı test
#[test]
fn pozitif_sayilar_toplami() { assert_eq!(topla(1, 2), 3); }

#[test]
fn negatif_sayilar_toplami() { assert_eq!(topla(-1, 1), 0); }
```

## 13.2 ❌ Anti-Patterns

```rust
// ❌ Testler arası bağımlılık
static mut GLOBAL_SAYAC: i32 = 0;

#[test]
fn test_a() { unsafe { GLOBAL_SAYAC += 1; } }

#[test]
fn test_b() { 
    unsafe { 
        assert_eq!(GLOBAL_SAYAC, 1);  // ❌ test_a'nın çalışmasına bağlı!
    } 
}

// ❌ Ağ veya DB'ye bağımlı unit test
#[test]
fn test_gercek_api() {
    let client = reqwest::blocking::get("https://api.example.com").unwrap();
    // ❌ Bu unit test değil, integration test!
}
```

---

# 🎯 ÖZET: Testing Kontrol Listesi

| Özellik | C# (xUnit) | Rust |
|---|---|---|
| Test framework | Harici paket | Yerleşik (`#[test]`) |
| Test keşfi | Reflection | Attribute |
| Assertion | `Assert.Equal()` | `assert_eq!()` |
| Setup/Teardown | Constructor/Dispose | Struct + Drop |
| Unit test yeri | Ayrı proje | Aynı dosya (`#[cfg(test)]`) |
| Integration test | Ayrı proje | `tests/` klasörü |
| Doc test | XML doc (test yok) | Otomatik test edilir |
| Mocking | Moq, NSubstitute | Trait + Mockall |
| Benchmark | BenchmarkDotNet | Criterion |
| Coverage | Coverlet | cargo-tarpaulin |
| Property-based | FsCheck | Proptest, Quickcheck |
| Snapshot | Verify | insta |
| Parallel | Varsayılan | Varsayılan |
| Embedded test | Yok | cfg + HAL |

---

# 🚀 Son Tavsiyeler

1. **Testleri Kodla Birlikte Yazın:** `#[cfg(test)] mod tests { }` bloğunu her modülde tutun.

2. **Doc Testleri Ciddye Alın:** Dokümantasyon örnekleri test edilecekse, her zaman güncel kalır.

3. **Property-Based Testing Kullanın:** Edge case'leri bulmanın en iyi yolu. Özellikle matematiksel fonksiyonlar için ideal.

4. **Mocking için Trait Kullanın:** Donanım ve servis bağımlılıklarını trait'ler arkasına saklayın.

5. **Embedded Kodu Test Edilebilir Tasarlayın:** 
   - HAL (Hardware Abstraction Layer) kullanın
   - Saf mantığı donanım bağımsız yazın
   - Host'ta unit test, hedefte integration test

6. **Step Motor Projeniz İçin Test Stratejisi:**
   ```
   ├── src/
   │   ├── motor.rs          # Saf mantık (host'ta test edilir)
   │   ├── driver.rs         # Trait tanımı
   │   ├── driver_real.rs    # RP2354B implementasyonu
   │   └── driver_mock.rs    # Test mock'u
   └── tests/
       ├── motor_test.rs     # Unit testler
       └── integration.rs    # HIL testler
   ```

7. **CI/CD'da Test Çalıştırın:**
   ```yaml
   # .github/workflows/test.yml
   - run: cargo test --all-features
   - run: cargo clippy -- -D warnings
   - run: cargo fmt --check
   ```

8. **Coverage Hedefi Belirleyin:**
   - Kritik kod (motor kontrol): %95+
   - Genel kod: %80+
   - Test edilmeyen kodu `#[cfg(not(tarpaulin_include))]` ile hariç tutun

9. **`cargo nextest` Kullanın:** Daha hızlı ve güvenilir test çalıştırıcı:
   ```bash
   cargo install cargo-nextest
   cargo nextest run
   ```

10. **Test Piramidi:**
    - **Taban:** Çok sayıda unit test (hızlı, ucuz)
    - **Orta:** Integration test (API, DB)
    - **Tepe:** E2E test (gerçek donanım, yavaş)

> 🦀 **Unutmayın:** Rust'ın güçlü tip sistemi ve ownership kuralları, zaten birçok hatayı derleme zamanında yakalar. Testler, **iş mantığı hatalarını** ve **beklenmedik senaryoları** yakalamak için vardır. Rust'ta "derleniyorsa çalışır" demek mümkün değildir, ama "derlenmiyorsa kesinlikle çalışmaz" demek mümkündür! Testler, bu ikinci kategori dışındaki hataları yakalar.