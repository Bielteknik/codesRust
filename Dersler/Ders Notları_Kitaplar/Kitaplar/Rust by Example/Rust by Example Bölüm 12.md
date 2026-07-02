# 🦀 Rust Cargo Rehberi — Ders Anlatımı

Hoş geldin! Bu derste, Rust'ın resmi paket yönetim aracı olan **Cargo**'yu derinlemesine öğreneceğiz. Cargo sadece bir bağımlılık yöneticisi değil; aynı zamanda proje oluşturucu, derleyici, test çalıştırıcı ve çok daha fazlası. Hazırsan başlayalım! 🚀

---

## 📚 Bölüm 1: Cargo Nedir?

Cargo, Rust ekosisteminin **kalbidir**. Tıpkı JavaScript'te `npm`, Python'da `pip`, Java'da `Maven` ne ise, Rust'ta da Cargo odur. Ama Cargo bundan çok daha fazlasını yapar:

✅ **Bağımlılık Yönetimi** — `crates.io` (Rust'ın resmi paket kayıt defteri) ile entegre çalışır
✅ **Proje İskeleti Oluşturma** — Tek komutla yeni proje
✅ **Derleme (Build)** — Kodunu derler, sadece değişen kısımları tekrar derler (incremental build)
✅ **Test Çalıştırma** — Unit testler, entegrasyon testleri, benchmarklar
✅ **Dokümantasyon Oluşturma** — `cargo doc` ile otomatik dokümantasyon
✅ **Yayınlama** — Kodunu `crates.io`'ya tek komutla yayınla

> 💡 **Ders Notu:** Cargo'nun kapsamlı dokümantasyonuna [The Cargo Book](https://doc.rust-lang.org/cargo/) adresinden ulaşabilirsin.

---

## 📚 Bölüm 2: Dependencies (Bağımlılıklar)

### 2.1 Yeni Proje Oluşturma

Cargo ile iki tür proje oluşturabiliriz:

```bash
# İkili (binary) proje — çalıştırılabilir bir program
cargo new foo

# Kütüphane (library) projesi — başkalarının kullanacağı bir crate
cargo new --lib bar
```

Bu komutları çalıştırdığında dosya yapısı şöyle oluşur:

```
.
├── bar/
│   ├── Cargo.toml      ← Yapılandırma dosyası
│   └── src/
│       └── lib.rs      ← Kütüphane kök dosyası
└── foo/
    ├── Cargo.toml      ← Yapılandırma dosyası
    └── src/
        └── main.rs     ← Programın giriş noktası
```

### 2.2 Cargo.toml Dosyasını Anlama

`Cargo.toml` dosyası projenin kimlik kartasıdır. İçeriği şöyle görünür:

```toml
[package]
name = "foo"
version = "0.1.0"
authors = ["ahmet"]
edition = "2021"

[dependencies]
```

| Alan | Anlamı |
|------|--------|
| `name` | Projenin adı. `crates.io`'ya yayınlanırsa bu isimle kaydedilir ve derlenen ikili dosyanın adı olur. |
| `version` | **Semantik Sürümleme** (Semantic Versioning) kullanır. `MAJOR.MINOR.PATCH` formatında. |
| `authors` | Projeyi yayınlayan yazarların listesi. |
| `edition` | Rust'ın hangi yıl sürümünü kullanacağını belirtir (2015, 2018, 2021). |

### 2.3 Bağımlılık Ekleme

Diyelim ki harika bir komut satırı arayüzü (CLI) yapmak istiyorsun. Bunun için `clap` crate'ini kullanabiliriz:

```toml
[dependencies]
clap = "2.27.1"
```

İşte bu kadar! Artık kodunda `clap`'ı kullanabilirsin. Cargo, bağımlılıkları otomatik olarak indirir ve derler.

### 2.4 Farklı Bağımlılık Kaynakları

Cargo sadece `crates.io`'dan değil, farklı kaynaklardan da bağımlılık çekebilir:

```toml
[dependencies]
# crates.io'dan (resmi paket kayıt defteri)
clap = "2.27.1"

# GitHub reposundan doğrudan
rand = { git = "https://github.com/rust-lang-nursery/rand" }

# Yerel dosya sisteminden (başka bir projenin yolu)
bar = { path = "../bar" }
```

### 2.5 Derleme ve Çalıştırma

```bash
# Sadece derle (make gibi, sadece değişenleri tekrar derler)
cargo build

# Derle ve çalıştır
cargo run

# Release (optimize edilmiş) modunda derle
cargo build --release
```

> ⚠️ **Önemli:** Bu komutları proje dizinindeki **herhangi bir alt dizinde** bile çalıştırabilirsin. Cargo otomatik olarak kök dizini bulur.

---

## 📚 Bölüm 3: Conventions (Cargo Düzenleri/Kuralları)

### 3.1 Birden Fazla Binary (İkili Dosya)

Standart bir projede `src/main.rs` tek bir binary oluşturur. Peki ya aynı projede **birden fazla çalıştırılabilir dosya** istersen?

Cargo bunu da destekler! `src/bin/` dizinine ek Rust dosyaları koyman yeterli:

```
foo/
├── Cargo.toml
└── src/
    ├── main.rs              ← Varsayılan binary
    └── bin/
        ├── my_other_bin.rs  ← İkinci binary
        └── server.rs        ← Üçüncü binary
```

Her dosya ayrı bir binary olarak derlenir. Belirli bir binary'yi çalıştırmak için `--bin` bayrağını kullan:

```bash
# Varsayılan binary'yi çalıştır
cargo run

# Belirli bir binary'yi çalıştır
cargo run --bin my_other_bin
cargo run --bin server

# Belirli bir binary'yi derle
cargo build --bin server
```

### 3.2 Cargo'nun Desteklediği Diğer Özellikler

Cargo sadece binary'lerle sınırlı değil. Şunları da destekler:

| Özellik | Açıklama |
|---------|----------|
| **Tests** | Test dosyaları |
| **Benchmarks** | Performans ölçümleri |
| **Examples** | Örnek kullanım dosyaları |

> 📌 **Kural:** Cargo, "convention over configuration" (yapılandırmadan çok kurallar) felsefesini benimser. Yani dosyaları doğru yere koyarsan, Cargo onları otomatik tanır.

---

## 📚 Bölüm 4: Tests (Testler)

Test, her yazılımın **ayrılmaz bir parçasıdır** ve Rust bunu birinci sınıf vatandaş olarak destekler! 🎯

### 4.1 Test Türleri ve Dizin Yapısı

Rust'ta üç tür test vardır: **Unit Test**, **Doc Test** ve **Integration Test**.

```
foo/
├── Cargo.toml
├── src/
│   ├── main.rs
│   └── lib.rs          ← Unit testler modüllerin içinde
└── tests/
    ├── my_test.rs      ← Entegrasyon testleri
    └── my_other_test.rs
```

- **Unit testler** → Test ettikleri modülün içine yazılır (genellikle `#[cfg(test)]` ile)
- **Integration testler** → `tests/` dizininde ayrı dosyalarda olur. Her dosya, kütüphaneni sanki dışarıdan bağımlı bir crate çağırıyormuş gibi test eder.

### 4.2 Testleri Çalıştırma

```bash
# Tüm testleri çalıştır
cargo test
```

Çıktı şöyle görünür:

```
$ cargo test
   Compiling blah v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.89 secs
     Running target/debug/deps/blah-d3b32b97275ec472

running 4 tests
test test_bar ... ok
test test_baz ... ok
test test_foo_bar ... ok
test test_foo ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### 4.3 Test Filtreleme

Belirli bir desene uyan testleri çalıştırmak için:

```bash
# Adında "test_foo" geçen testleri çalıştır
cargo test test_foo
```

Çıktı:
```
running 2 tests
test test_foo ... ok
test test_foo_bar ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out
```

### 4.4 ⚠️ Eşzamanlılık Uyarısı (Race Condition)

Çok önemli bir nokta: **Cargo testleri paralel (eşzamanlı) çalıştırır!** Bu yüzden testlerinin birbirleriyle yarışmamasına (race condition) dikkat etmelisin.

**Kötü örnek** — İki test aynı dosyaya yazıyor:

```rust
#[cfg(test)]
mod tests {
    use std::fs::OpenOptions;
    use std::io::Write;

    #[test]
    fn test_file() {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("ferris.txt")
            .expect("Failed to open ferris.txt");

        for _ in 0..5 {
            file.write_all("Ferris\n".as_bytes())
                .expect("Could not write to ferris.txt");
        }
    }

    #[test]
    fn test_file_also() {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("ferris.txt")
            .expect("Failed to open ferris.txt");

        for _ in 0..5 {
            file.write_all("Corro\n".as_bytes())
                .expect("Could not write to ferris.txt");
        }
    }
}
```

**Beklenen sonuç:**
```
Ferris
Ferris
Ferris
Ferris
Ferris
Corro
Corro
Corro
Corro
Corro
```

**Gerçek sonuç (eşzamanlılık yüzünden):**
```
Corro
Ferris
Corro
Ferris
Corro
Ferris
Corro
Ferris
Corro
Ferris
```

> 💡 **Çözüm:** Her testin kendine özgü bir dosya/dosya adı kullanmasını sağla ya da `-- --test-threads=1` bayrağı ile testleri sıralı çalıştır.

---

## 📚 Bölüm 5: Build Scripts (Derleme Betikleri)

### 5.1 Ne Zaman Gerekli?

Bazen standart `cargo build` yeterli olmaz. Projenin derlenmeden önce bazı ön koşullara ihtiyacı olabilir:

🔧 Kod üretimi (code generation)
🔧 Native (C/C++) kodların derlenmesi
🔧 Harici bir sisteme bağlanıp bilgi alma

İşte bu durumlar için **build script** (derleme betiği) kullanırız.

### 5.2 Build Script Nasıl Eklenir?

İki yol var:

**Yol 1 — Varsayılan:** Proje kök dizinine `build.rs` dosyası koy. Cargo otomatik bulur.

**Yol 2 — Özelleştirilmiş:** `Cargo.toml`'de belirt:

```toml
[package]
name = "foo"
version = "0.1.0"
build = "build.rs"   # ← Özel betik dosyası
```

### 5.3 Nasıl Çalışır?

Build script aslında **başka bir Rust dosyasıdır**. Ama bu dosya, projenin geri kalanından **önce** derlenip çalıştırılır. Yani senin asıl kodun derlenmeden önce build script çalışır ve gerekli ön hazırlıkları yapar.

```
┌─────────────────────┐
│  1. build.rs derlenir│
└──────────┬──────────┘
           ▼
┌─────────────────────┐
│  2. build.rs çalışır │  ← Kod üretimi, native derleme vb.
└──────────┬──────────┘
           ▼
┌─────────────────────┐
│  3. Asıl proje derlenir │
└─────────────────────┘
```

### 5.4 Girdi/Çıktı Mekanizması

**Girdi:** Cargo, build script'e **ortam değişkenleri** (environment variables) üzerinden bilgi verir. Örneğin:
- `OUT_DIR` — Betiğin ürettiği dosyaları koyabileceği dizin
- `CARGO_MANIFEST_DIR` — Proje kök dizini
- `TARGET` — Hedef platform

**Çıktı:** Build script, `stdout`'a yazdırır. Tüm çıktılar şuraya yazılır:
```
target/debug/build/<paket-adı>/output
```

**Özel komutlar:** `cargo:` ile başlayan satırlar Cargo tarafından doğrudan yorumlanır. Örneğin:

```rust
// build.rs
fn main() {
    // Derleyiciye "foo" kütüphanesini bağlamasını söyle
    println!("cargo:rustc-link-lib=foo");
    
    // Yeniden derleme tetikleyici dizini bildir
    println!("cargo:rerun-if-changed=src/wrapper.h");
}
```

### 5.5 Kullanım Alanları

| Kullanım Alanı | Örnek |
|----------------|-------|
| C/C++ kütüphanelerini derleme | `cc` crate ile |
| Kod üretimi | Protobuf, GraphQL şemaları |
| Sistem bilgisi toplama | İşletim sistemi, mimari tespiti |
| Kaynak dosya doğrulama | Dosya format kontrolü |

---

## 🎯 Özet Tablosu

| Konu | Komut / Dosya | Açıklama |
|------|---------------|----------|
| Yeni binary proje | `cargo new foo` | Çalıştırılabilir proje |
| Yeni kütüphane | `cargo new --lib bar` | Kütüphane crate'i |
| Bağımlılık ekleme | `Cargo.toml` → `[dependencies]` | Paket ekle |
| Derleme | `cargo build` | Sadece derle |
| Çalıştırma | `cargo run` | Derle ve çalıştır |
| Release modu | `cargo build --release` | Optimize edilmiş derleme |
| Belirli binary | `cargo run --bin name` | Çoklu binary desteği |
| Tüm testler | `cargo test` | Testleri çalıştır |
| Filtreli test | `cargo test pattern` | İsme göre filtrele |
| Build script | `build.rs` | Derleme öncesi betik |

---

## 🎓 Pratik İpuçları

1. **`cargo check`** — Kodunu derlemeden sadece hataları kontrol eder. Çok hızlıdır, geliştirme sırasında sıkça kullan.

2. **`cargo doc --open`** — Projenin (ve bağımlılıklarının) dokümantasyonunu tarayıcıda açar.

3. **`cargo update`** — `Cargo.lock` dosyasını güncelleyerek bağımlılıkları en son uyumlu sürümlere yükseltir.

4. **`cargo clean`** — `target/` dizinini temizler, sıfırdan derleme yapmak için kullanılır.

5. **`cargo tree`** — Bağımlılık ağacını gösterir, hangi paketin hangi bağımlılıkları getirdiğini görmek için harika.

---

## 🏁 Son Söz

Cargo, Rust'ın en güçlü yanlarından biridir. Sadece bir "paket yöneticisi" olarak düşünme — o senin **proje asistanın**. Proje oluşturmadan derlemeye, testten yayınlamaya kadar her adımda yanında. 

Bu derste öğrendiklerimiz Cargo'nun temelleriydi. Daha ileri seviye konular için [The Cargo Book](https://doc.rust-lang.org/cargo/) kaynağına mutlaka göz at. 🦀

Başarılar! 🚀

# DeepSeek Yorumu --------------------

Rust'a dair öğrendiklerinizi pekiştirmenin en iyi yollarından biri, projelerinizi profesyonel bir şekilde yönetmeyi öğrenmektir. İşte bu noktada **Cargo** devreye girer. Bu bölümde, Rust'ın resmi paket yönetim aracı olan Cargo'yu, neden bu kadar önemli olduğunu ve günlük geliştirme akışınızda size nasıl devrim niteliğinde kolaylıklar sağladığını detaylıca öğreneceğiz.

## Cargo Nedir ve Neden Kullanmalıyız?

Bir Rust programcısı olarak Cargo, sizin için bir **proje asistanıdır**. Sadece bir bağımlılık yöneticisi değil, aynı zamanda güçlü bir derleme sistemi, test koşucusu ve belge oluşturucudur. Cargo sayesinde:

*   **Bağımlılıklarınızı (`dependencies`) kolayca yönetebilirsiniz.**
*   **Birim testlerinizi (`unit tests`) çalıştırabilirsiniz.**
*   **Performans testlerinizi (`benchmarks`) koşabilirsiniz.**
*   **Projenizin dokümantasyonunu (`documentation`) otomatik olarak oluşturabilirsiniz.**

Kısacası Cargo, Rust ekosisteminin kalbidir ve onu kullanmak, Rust ile üretken olmanın ilk ve en önemli adımıdır.

## 🚀 Yeni Bir Proje Başlatmak: `cargo new`

Her şey yeni bir proje ile başlar. Terminalinizde aşağıdaki komutu çalıştırarak "hello_cargo" adında yeni bir ikili program (binary) projesi oluşturabilirsiniz:

```bash
cargo new hello_cargo
```

Bu komut, sizin için bir klasör oluşturacak ve içerisine temel bir Rust projesi iskeleti kuracaktır. Oluşan dosya yapısı şuna benzer:

```text
hello_cargo/
├── Cargo.toml
└── src/
    └── main.rs
```

*   **`Cargo.toml`**: Bu dosya, projenizin kalbidir. Proje adı, versiyonu, yazar bilgileri ve tüm bağımlılıklarınız bu dosyada tanımlanır. `Cargo.toml` dosyası, Rust ekosisteminde `Cargo` tarafından okunan bir **manifest dosyasıdır**.
*   **`src/main.rs`**: Uygulamanızın ana kod dosyasıdır. Varsayılan olarak, ekrana "Hello, world!" yazdıran basit bir kod içerir.

## ⚙️ Projeyi Derlemek ve Çalıştırmak

`cargo new` ile oluşturduğumuz projeyi derlemek ve çalıştırmak için Cargo'nun sağladığı iki temel komutu kullanırız:

### 1. `cargo build`

Bu komut, projenizi ve tüm bağımlılıklarını derler. İlk çalıştırmada bağımlılıkları indirip derleyeceği için biraz zaman alabilir, ancak sonraki derlemeler çok daha hızlı olacaktır.

```bash
cargo build
```

Derleme başarılı olduğunda, çalıştırılabilir dosyanız `target/debug/` klasöründe oluşur. Debug modunda derlendiği için hata ayıklama bilgileri içerir ve optimizasyon yapılmaz.

### 2. `cargo run`

Eğer amacınız kodu derleyip hemen çalıştırmaksa, `cargo run` komutu bunu tek bir adımda yapar. Eğer kodda bir değişiklik yoksa, Cargo akıllıca davranarak projeyi yeniden derlemez ve doğrudan çalıştırır.

```bash
cargo run
```

### 3. `cargo check`

Bu komut, belki de geliştirme esnasında en sık kullanacağınız komuttur. `cargo check`, projenizi derlemeden sadece **hata olup olmadığını kontrol eder**. Çıktı dosyası oluşturmadığı için `cargo build`'den çok daha hızlıdır. Kod yazarken sürekli olarak `cargo check` çalıştırarak hataları anında görebilirsiniz.

```bash
cargo check
```

## 📦 Bağımlılıkları Yönetmek: `Cargo.toml`

Rust'ın gücü, zengin ekosisteminden gelir. Projenize harici bir kütüphane (crate) eklemek istediğinizde, bunu `Cargo.toml` dosyasına eklemeniz yeterlidir.

Örneğin, renkli çıktılar için popüler bir kütüphane olan `colored`'ı ekleyelim. `Cargo.toml` dosyanızı açın ve `[dependencies]` bölümünün altına şu satırı ekleyin:

```toml
[dependencies]
colored = "2.1.0"
```

Artık projenizde `colored` kütüphanesini kullanabilirsiniz. Cargo, sizin için bu kütüphaneyi ve onun kendi bağımlılıklarını [crates.io](https://crates.io) adresinden indirip derleyecektir.

> 💡 **İpucu:** `cargo build` dediğinizde, Cargo otomatik olarak `Cargo.toml`'daki tüm bağımlılıkları indirir ve derler. Bağımlılıkları güncellemek için `cargo update` komutunu kullanabilirsiniz.

## 🧪 Test Etmek: `cargo test`

Rust, birim testlerine (unit tests) birinci sınıf bir destek sunar. Cargo da bu testleri çalıştırmak için `cargo test` komutunu sağlar. Projenizdeki tüm test fonksiyonlarını bulur, çalıştırır ve sonuçları raporlar.

```bash
cargo test
```

## 📚 Dokümantasyon Oluşturmak: `cargo doc`

Projenizdeki ve bağımlılıklarınızdaki tüm dokümantasyonu HTML olarak oluşturmak için `cargo doc` komutunu kullanabilirsiniz. Oluşturulan belgeler `target/doc/` klasöründe yer alır.

```bash
cargo doc --open
```

`--open` bayrağı ile oluşturulan dokümantasyonu varsayılan tarayıcınızda açabilirsiniz.

## 📖 Daha Fazlası İçin

Bu bölüm, Cargo'nun temellerine hızlı bir giriş niteliğindedir. Cargo, burada anlatılanların çok ötesinde yeteneklere sahiptir. Örneğin:

*   **Workspace'ler:** Birden fazla ilgili projeyi tek bir çatı altında yönetmek.
*   **Özel Derleme Komutları:** Derleme sürecini özelleştirmek için `build.rs` dosyaları.
*   **Yayınlama (`cargo publish`):** Kendi kütüphanelerinizi [crates.io](https://crates.io)'da yayınlamak.

Cargo hakkında çok daha detaylı ve kapsamlı bilgi için resmi **Cargo Kitabı'nı (The Cargo Book)** incelemenizi şiddetle tavsiye ederim.

## 💎 Özet

Cargo, Rust geliştirme deneyiminin ayrılmaz bir parçasıdır. Proje oluşturma, bağımlılık yönetimi, derleme, test etme ve dokümantasyon gibi kritik görevleri otomatikleştirerek sizin kod yazmaya odaklanmanızı sağlar. Unutmayın, bir Rust projesine başlamak için ihtiyacınız olan tek şey `cargo new` komutudur. Gerisini Cargo halleder!