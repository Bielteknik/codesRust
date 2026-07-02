# 🦀 Ders Notları: Bölüm 11 - Rust'ta Test Yazma — Kapsamlı Ders Notları

Merhaba! Bu derste Rust programlama dilinin test yazma mekanizmasını sıfırdan, ders anlatır gibi detaylı şekilde inceleyeceğiz. Rust'ın resmi kitabının 11. bölümünü temel alarak, testlerin **neden** yazıldığından **nasıl** yazıldığına, **nasıl çalıştırıldığından** nasıl **organize edildiğine** kadar her şeyi adım adım işleyeceğiz.

---

## 📖 BÖLÜM 1: Test Yazmanın Önemi (Giriş)

### Neden Test Yazmalıyız?

Bilgisayar bilimlerinin öncülerinden **Edsger W. Dijkstra** 1972'de şöyle demiştir:

> *"Program testi, hataların varlığını göstermek için çok etkili bir yol olabilir, ancak hataların yokluğunu göstermek için umutsuzca yetersizdir."*

Yani testler her hatayı yakalayamaz ama yine de elimizdeki en güçlü araçtır!

### Doğruluk (Correctness) Nedir?

Rust, **doğruluk** kavramına büyük önem verir. Tip sistemi (type system) ve borrow checker sayesinde birçok hata derleme zamanında yakalanır. Örneğin:

- Bir fonksiyona `String` yerine `i32` veremezsiniz.
- Geçersiz bir referans kullanamazsınız.

**Ancak** tip sistemi her şeyi yakalayamaz! Şu örneği düşünelim:

```rust
fn add_two(x: i32) -> i32 {
    x + 10  // ❌ Biz +2 istiyorduk ama +10 yazdık!
}
```

Rust bu kodu derlerken **hiçbir hata vermez** çünkü tip sistemi açısından her şey doğru. Ama fonksiyon **niyet ettiğimiz işi yapmıyor**. İşte burada **testler** devreye girer:

```rust
#[test]
fn test_add_two() {
    assert_eq!(add_two(3), 5);  // 3 + 2 = 5 olmalı
}
```

Bu test, fonksiyonun yanlış çalıştığını hemen ortaya çıkarır.

---

## 📖 BÖLÜM 2: Test Nasıl Yazılır? (Writing Tests)

### Test Fonksiyonunun Anatomisi

Rust'ta bir test, temelde üç aşamadan oluşan bir fonksiyondur:

1. **Hazırlık (Setup):** Gerekli veri ve durumları ayarla
2. **Çalıştırma (Exercise):** Test edilecek kodu çalıştır
3. **Doğrulama (Assert):** Sonucun beklendiği gibi olduğunu kontrol et

### `#[test]` Özniteliği

Bir fonksiyonu teste dönüştürmek için `#[test]` özniteliğini (attribute) kullanırız:

```rust
#[test]
fn benim_testim() {
    // Test kodu buraya
}
```

Bu öznitelik, Cargo'ya "bu fonksiyonu test olarak çalıştır" mesajını verir.

### İlk Testimiz: `adder` Projesi

Yeni bir kütüphane projesi oluşturalım:

```bash
cargo new adder --lib
cd adder
```

Cargo otomatik olarak şu şablonu oluşturur:

```rust
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

Burada dikkat edilmesi gereken noktalar:
- `#[cfg(test)]` → Bu modül sadece `cargo test` çalıştırıldığında derlenir
- `use super::*;` → Üst modüldeki (yani `src/lib.rs`'deki) her şeyi kapsama alır
- `assert_eq!` → İki değerin eşit olduğunu doğrular

### Başarılı ve Başarısız Testler

Başarısız bir test ekleyelim:

```rust
#[test]
fn another() {
    panic!("Bu test başarısız olsun!");
}
```

Çıktı şöyle olur:

```
test tests::it_works ... ok
test tests::another ... FAILED

failures:
---- tests::another stdout ----
thread 'tests::another' panicked at src/lib.rs:17:5:
Bu test başarısız olsun!
```

> 💡 **Önemli:** Her test ayrı bir thread'de çalışır. Bir test başarısız olduğunda (panic olduğunda), ana thread bunu algılar ve testi FAILED olarak işaretler.

### Doğrulama Makroları (Assertion Macros)

Rust üç temel doğrulama makrosu sunar:

#### 1. `assert!` — Koşul Kontrolü

Bir ifadenin `true` olduğunu doğrular:

```rust
#[test]
fn greater_than_hundred() {
    let sonuc = buyuk_sayi_uret();
    assert!(sonuc > 100);
}
```

#### 2. `assert_eq!` — Eşitlik Kontrolü

İki değerin eşit olduğunu doğrular. Hata mesajında her iki değeri de gösterir:

```rust
#[test]
fn test_toplama() {
    assert_eq!(add(2, 3), 5);
}
```

Başarısız olursa:
```
assertion `left == right` failed
  left: 6
 right: 5
```

#### 3. `assert_ne!` — Eşitsizlik Kontrolü

İki değerin **eşit olmadığını** doğrular:

```rust
#[test]
fn test_farkli() {
    assert_ne!(add(2, 2), 5);  // 4 != 5, doğru!
}
```

### Özel Hata Mesajları

Makrolara ek mesaj ekleyebilirsiniz:

```rust
#[test]
fn test_deger_kontrolu() {
    let deger = "Merhaba";
    assert!(
        deger.contains("Dünya"),
        "Değer '{}' beklenen 'Dünya' kelimesini içermiyor!",
        deger
    );
}
```

### `should_panic` — Panik Bekleyen Testler

Bazen kodumuzun **hata vermesini (panic)** bekleriz. Örneğin:

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess değeri 1-100 arasında olmalı, {} alındı.", value);
        }
        Guess { value }
    }
}
```

Bu durumda `#[should_panic]` kullanırız:

```rust
#[test]
#[should_panic]
fn greater_than_100() {
    Guess::new(200);  // 200 geçersiz, panic bekliyoruz
}
```

#### `expected` Parametresi

Daha hassas kontrol için beklenen panic mesajını belirtebiliriz:

```rust
#[test]
#[should_panic(expected = "1-100 arasında")]
fn test_hata_mesaji() {
    Guess::new(200);
}
```

Bu sayede, farklı bir nedenden panic olursa test yine başarısız olur.

### `Result<T, E>` Döndüren Testler

Testler panic yerine `Result` döndürebilir. Bu, `?` operatörünü kullanmamıza olanak tanır:

```rust
#[test]
fn fn_returns_result() -> Result<(), String> {
    let sonuc = add(2, 2);
    if sonuc == 4 {
        Ok(())
    } else {
        Err(String::from("İki artı iki dört etmeli!"))
    }
}
```

> ⚠️ **Dikkat:** `Result` döndüren testlerde `#[should_panic]` kullanılamaz!

---

## 📖 BÖLÜM 3: Testlerin Çalıştırılması (Running Tests)

### `cargo test` Komutu

Testleri çalıştırmak için basitçe:

```bash
cargo test
```

Bu komut:
1. Kodu test modunda derler
2. Test binary'sini oluşturur
3. Tüm testleri **paralel** olarak çalıştırır
4. Çıktıları yakalar (sadece başarısız testlerin çıktısını gösterir)

### Paralel Çalıştırma ve Thread Kontrolü

Testler varsayılan olarak **paralel** çalışır. Bu hızlıdır ama paylaşılan durum (shared state) varsa sorun çıkarabilir.

**Örnek sorun:** İki test aynı anda `test-output.txt` dosyasına yazıyorsa, biri diğerinin verisini ezebilir.

Çözüm: Testleri sırayla çalıştır:

```bash
cargo test -- --test-threads=1
```

> 💡 `--` işaretinden önceki argümanlar `cargo test`'e, sonrakiler test binary'sine gider.

### Çıktı Kontrolü

Varsayılan olarak başarılı testlerin `println!` çıktısı gizlenir. Görmek için:

```bash
cargo test -- --show-output
```

### Test Filtreleme

#### Tek Test Çalıştırma

```bash
cargo test one_hundred
```

#### İsim ile Filtreleme

İsminde belirli bir kelime geçen tüm testleri çalıştırır:

```bash
cargo test add    # İçinde "add" geçen tüm testler
```

### `#[ignore]` — Yoksayılan Testler

Bazı testler çok uzun sürebilir (örn. 1 saat). Bunları normal çalıştırmadan hariç tutabiliriz:

```rust
#[test]
#[ignore]
fn cok_uzun_test() {
    // 1 saat sürecek kod
}
```

Normal `cargo test` çalıştırıldığında bu test **atlanır**.

Sadece yoksayılan testleri çalıştırmak için:

```bash
cargo test -- --ignored
```

Hepsini (yoksayılan dahil) çalıştırmak için:

```bash
cargo test -- --include-ignored
```

---

## 📖 BÖLÜM 4: Test Organizasyonu (Test Organization)

Rust topluluğu testleri iki ana kategoriye ayırır:

| Özellik | Birim Testleri (Unit Tests) | Entegrasyon Testleri (Integration Tests) |
|---------|----------------------------|------------------------------------------|
| **Kapsam** | Küçük, odaklı | Büyük, geniş kapsamlı |
| **Konum** | `src/` içinde, ilgili dosyada | `tests/` dizininde |
| **Erişim** | Private (özel) fonksiyonları test edebilir | Sadece public API'yi kullanır |
| **Amaç** | Tek bir modülün doğru çalıştığını doğrular | Birden fazla modülün birlikte çalıştığını doğrular |

### Birim Testleri (Unit Tests)

Her dosyada bir `tests` modülü oluşturulur:

```rust
pub fn add_two(a: u64) -> u64 {
    internal_adder(a, 2)
}

fn internal_adder(left: u64, right: u64) -> u64 {
    left + right  // private fonksiyon!
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        // Private fonksiyonu bile test edebiliriz!
        let result = internal_adder(2, 2);
        assert_eq!(result, 4);
    }
}
```

> 💡 **Rust'ın Gücü:** Diğer birçok dilde private fonksiyonları test etmek zorken, Rust'ta modül yapısı sayesinde bu mümkün.

### Entegrasyon Testleri (Integration Tests)

Proje kök dizininde `tests/` klasörü oluşturulur:

```
adder/
├── Cargo.toml
├── src/
│   └── lib.rs
└── tests/
    └── integration_test.rs
```

`tests/integration_test.rs` içeriği:

```rust
use adder::add_two;  // Kütüphaneyi import etmeliyiz!

#[test]
fn it_adds_two() {
    let result = add_two(2);
    assert_eq!(result, 4);
}
```

> 📌 **Dikkat:** `tests/` dizini Cargo tarafından otomatik olarak tanındığı için `#[cfg(test)]` gerekmez.

### `cargo test` Çıktısı

```
Running unittests src/lib.rs
running 1 test
test tests::internal ... ok

Running tests/integration_test.rs
running 1 test
test it_adds_two ... ok

Doc-tests adder
running 0 tests
```

Üç bölüm vardır:
1. **Unit tests** — `src/` içindeki testler
2. **Integration tests** — `tests/` içindeki testler
3. **Doc tests** — Dokümantasyondaki kod örnekleri

> ⚠️ Eğer birim testlerden biri başarısız olursa, entegrasyon testleri **çalıştırılmaz**.

### Belirli Bir Entegrasyon Testini Çalıştırma

```bash
cargo test --test integration_test
```

### Paylaşılan Kod (Common Module)

Birden fazla entegrasyon testinde kullanılacak yardımcı fonksiyonlar için `tests/common/mod.rs` oluşturulur:

```
tests/
├── common/
│   └── mod.rs       ← Paylaşılan kod buraya
└── integration_test.rs
```

`tests/common/mod.rs`:

```rust
pub fn setup() {
    // Test öncesi hazırlık kodu
}
```

`tests/integration_test.rs`:

```rust
use adder::add_two;

mod common;  // common modülünü import et

#[test]
fn it_adds_two() {
    common::setup();  // Ortak hazırlığı çağır
    let result = add_two(2);
    assert_eq!(result, 4);
}
```

> 💡 **Neden `common/mod.rs`?** Eğer sadece `tests/common.rs` olsaydı, Cargo bunu ayrı bir test dosyası olarak görür ve test çıktısında "running 0 tests" olarak gösterirdi. `mod.rs` kullanarak Cargo'ya "bu bir test dosyası değil, paylaşılan bir modül" demiş oluruz.

---

## 📖 BÖLÜM 5: Özet ve En İyi Pratikler

### Hızlı Referans Tablosu

| Komut / Öznitelik | Açıklama |
|-------------------|----------|
| `#[test]` | Fonksiyonu test olarak işaretle |
| `#[cfg(test)]` | Modülü sadece test derlemesinde dahil et |
| `assert!(koşul)` | Koşulun `true` olduğunu doğrula |
| `assert_eq!(a, b)` | İki değerin eşit olduğunu doğrula |
| `assert_ne!(a, b)` | İki değerin farklı olduğunu doğrula |
| `#[should_panic]` | Testin panic atmasını bekle |
| `#[should_panic(expected = "...")]` | Belirli bir mesajla panic atmasını bekle |
| `#[ignore]` | Testi normal çalıştırmada atla |
| `cargo test` | Tüm testleri çalıştır |
| `cargo test test_adi` | Belirli bir testi çalıştır |
| `cargo test -- --test-threads=1` | Testleri sırayla çalıştır |
| `cargo test -- --show-output` | Başarılı testlerin çıktısını göster |
| `cargo test -- --ignored` | Sadece yoksayılan testleri çalıştır |
| `cargo test --test dosya_adi` | Belirli bir entegrasyon test dosyasını çalıştır |

### En İyi Pratikler

✅ **Hem birim hem entegrasyon testleri yazın** — İkisi de önemlidir.
✅ **Testleri bağımsız tutun** — Testler birbirine bağımlı olmamalıdır.
✅ **Paylaşılan durumdan kaçının** — Her test kendi dosyasını/verisini kullanmalı.
✅ **Anlamlı isimler verin** — Test adı, neyi test ettiğini açıklamalı.
✅ **Hata mesajlarını özelleştirin** — `assert!` makrolarına açıklayıcı mesajlar ekleyin.
✅ **Yavaş testleri `#[ignore]` ile işaretleyin** — Günlük geliştirmede zaman kaybetmeyin.

---

## 🎓 Son Söz

Test yazma, Rust'ın güvenilirlik felsefesinin temel taşlarından biridir. Tip sistemi ve borrow checker birçok hatayı engellerken, testler **iş mantığındaki** hataları yakalar. 

Rust'ın test sistemi:
- **Dahili** (dil içinde gömülü, harici framework gerektirmez)
- **Pragmatik** (hem birim hem entegrasyon testini destekler)
- **Güçlü** (paralel çalıştırma, filtreleme, yoksayma gibi özellikler)

Artık Rust projelerinizde profesyonel düzeyde testler yazmaya hazırsınız! 🚀
