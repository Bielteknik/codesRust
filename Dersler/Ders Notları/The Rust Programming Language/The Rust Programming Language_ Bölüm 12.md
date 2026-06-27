# 🦀 Ders Notları: Bölüm 12 - Bir I/O Projesi (Komut Satırı Aracı Geliştirme)

## 📚 Bölüm Hakkında Genel Bakış

Bu bölüm, Rust'ta şimdiye kadar öğrendiğiniz birçok kavramı **pratik bir proje** ile pekiştireceğiniz kapsamlı bir çalışma. Birlikte **minigrep** adında, klasik Unix `grep` aracının basitleştirilmiş bir versiyonunu geliştireceğiz.

**grep** (Globally search a Regular Expression and Print), bir dosyada belirli bir metni arayan ve eşleşen satırları ekrana yazdıran bir araçtır.

### 🎯 Bu Bölümde Neler Öğreneceğiz?

- ✅ Komut satırı argümanlarını okuma
- ✅ Dosya okuma/yazma işlemleri
- ✅ Hata yönetimi ve iyileştirme
- ✅ Kodu modüler hale getirme (refactoring)
- ✅ Test-driven development (TDD) - Test güdümlü geliştirme
- ✅ Çevre değişkenleri (environment variables) ile çalışma
- ✅ stdout ve stderr farkı

---

## 🚀 Başlayalım: Projeyi Oluşturma

```bash
$ cargo new minigrep
     Created binary (application) `minigrep` project
$ cd minigrep
```

Artık elimizde standart bir Rust projesi var:
```
minigrep/
├── Cargo.toml
└── src/
    └── main.rs
```

---

## 📖 Bölüm 12.1: Komut Satırı Argümanlarını Kabul Etme

### 🎯 Hedef
Programımızı şu şekilde çalıştırabilmek istiyoruz:
```bash
$ cargo run -- arama_metni dosya_yolu.txt
```

### 📌 std::env::args() Kullanımı

Rust'ın standart kütüphanesindeki `std::env::args()` fonksiyonu, programa geçirilen tüm komut satırı argümanlarını bir **iterator** olarak döndürür.

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
```

**Bu kod ne yapıyor?**
- `env::args()` → Argümanları bir iterator olarak döndürür
- `.collect()` → Iterator'ı bir koleksiyona (bizim durumumuzda Vec) dönüştürür
- `dbg!` → Debug formatında yazdırır

### 💡 Önemli Not: İlk Argüman Programın Kendisidir!

```bash
$ cargo run -- needle haystack
```

Çıktı:
```
[src/main.rs:5:5] args = [
    "target/debug/minigrep",  // ← Bu programın kendisi!
    "needle",                  // ← İlk gerçek argüman (index 1)
    "haystack",                // ← İkinci gerçek argüman (index 2)
]
```

### 📝 Argümanları Değişkenlere Atama

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];        // Arama metni
    let file_path = &args[2];    // Dosya yolu

    println!("Aradığımız: {query}");
    println!("Dosya: {file_path}");
}
```

**Çalıştırma:**
```bash
$ cargo run -- test ornek.txt
   Compiling minigrep v0.1.0
     Running `target/debug/minigrep test ornek.txt`
Aradığımız: test
Dosya: ornek.txt
```

### ⚠️ Dikkat Edilmesi Gerekenler

1. **Yetersiz argüman durumu:** Eğer kullanıcı argüman vermezse program **panic** yapar:
```bash
$ cargo run
thread 'main' panicked at 'index out of bounds: the len is 1 but the index is 1'
```

2. **Unicode sorunu:** `std::env::args()` geçersiz Unicode içerirse panic yapar. Alternatif olarak `std::env::args_os()` kullanılabilir.

---

## 📖 Bölüm 12.2: Dosya Okuma

### 🎯 Hedef
Belirtilen dosyayı okuyup içeriğini ekrana yazdırmak.

### 📄 Test Dosyası Oluşturma

`poem.txt` adında Emily Dickinson'ın bir şiirini oluşturalım:

```text
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

### 📝 Dosya Okuma Kodu

```rust
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Aradığımız: {query}");
    println!("Dosya: {file_path}");

    // Dosyayı oku!
    let contents = fs::read_to_string(file_path)
        .expect("Dosya okunabilmeliydi");

    println!("İçerik:\n{contents}");
}
```

**Çalıştırma:**
```bash
$ cargo run -- the poem.txt
Aradığımız: the
Dosya: poem.txt
İçerik:
I'm nobody! Who are you?
Are you nobody, too?
...
```

### 🔍 fs::read_to_string() Nasıl Çalışır?

- Dosyayı açar
- Tüm içeriği okur
- `Result<String>` döndürür
- Başarılıysa `Ok(String)`, hata varsa `Err(io::Error)`

### ⚠️ Mevcut Sorunlar

1. **main() fonksiyonu çok fazla iş yapıyor:**
   - Argüman okuma
   - Dosya okuma
   - Ekrana yazdırma

2. **Hata yönetimi zayıf:** `.expect()` kullanmak yerine daha iyi bir yaklaşım lazım.

---

## 📖 Bölüm 12.3: Hata Yönetimini İyileştirme ve Kodu Yeniden Düzenleme (Refactoring)

### 🎯 Hedef
Kodu daha modüler, test edilebilir ve hataya dayanıklı hale getirmek.

### 🏗️ Refactoring Stratejisi

**main() fonksiyonunu ikiye böleceğiz:**
1. **main()** → Sadece program akışını kontrol eder
2. **run()** → Gerçek iş mantığını çalıştırır

### 📝 Adım 1: Config Struct Oluşturma

Tüm konfigürasyonu tek bir struct'ta toplayalım:

```rust
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Yetersiz argüman sayısı");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
```

**Neden Result döndürüyoruz?**
- Hata durumlarını zarif şekilde yönetmek için
- Programın panic yapmasını engellemek için

### 📝 Adım 2: run() Fonksiyonu Oluşturma

```rust
use std::fs;
use std::error::Error;

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    
    println!("Dosya içeriği:\n{contents}");
    
    Ok(())
}
```

**`Box<dyn Error>` nedir?**
- Dinamik olarak tiplendirilmiş bir hata kutusu
- Farklı hata tiplerini döndürebilmemizi sağlar

**`?` operatörü ne yapıyor?**
- Eğer `Result::Err` ise, hatayı otomatik olarak döndürür
- Eğer `Result::Ok` ise, içindeki değeri çıkarır

### 📝 Adım 3: main() Fonksiyonunu Güncelleme

```rust
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Argüman ayrıştırma sorunu: {err}");
        process::exit(1);
    });

    println!("Aradığımız: {}", config.query);
    println!("Dosya: {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Uygulama hatası: {e}");
        process::exit(1);
    }
}
```

**unwrap_or_else() nedir?**
- `Ok` ise içindeki değeri döndürür
- `Err` ise closure'ı çalıştırır

**process::exit(1) nedir?**
- Programı hata kodu ile sonlandırır
- 0 = başarılı, 1 = hata

### 🎯 Refactoring Sonucu

✅ main() artık sadece program akışını kontrol ediyor  
✅ Hatalar zarif şekilde yönetiliyor  
✅ Kod daha okunabilir ve test edilebilir  
✅ Config struct tüm ayarları tek yerde tutuyor

---

## 📖 Bölüm 12.4: Kütüphane İşlevselliğini Test Etme (TDD)

### 🎯 Hedef
Arama işlevini **Test-Driven Development (TDD)** ile geliştirmek.

### 🔄 TDD Süreci

1. **Başarısız olan bir test yaz**
2. **Testi geçecek kadar kod yaz**
3. **Kodu refactor et, testlerin geçtiğinden emin ol**
4. **1. adıma dön**

### 📝 Adım 1: Başarısız Test Yazma

```rust
// src/lib.rs

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
```

**Lifetime ('a) neden gerekli?**
- Döndürülen vector, `contents`'in string slice'larını referans ediyor
- `query`'i değil, `contents`'i referans etmeli

### 📝 Adım 2: Testi Geçecek Kodu Yazma

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
```

**Bu kod ne yapıyor?**
1. `contents.lines()` → Her satırı iterator olarak döndürür
2. `line.contains(query)` → Satırda arama metni var mı kontrol eder
3. Varsa `results` vector'ına ekler
4. Sonuçları döndürür

### 🧪 Testleri Çalıştırma

```bash
$ cargo test
running 1 test
test tests::one_result ... ok

test result: ok. 1 passed; 0 failed
```

### 📝 Adım 3: run() Fonksiyonunu Güncelleme

```rust
use minigrep::search;

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    
    for line in search(&config.query, &contents) {
        println!("{line}");
    }
    
    Ok(())
}
```

### 🎯 Gerçek Dünya Testleri

```bash
$ cargo run -- frog poem.txt
How public, like a frog

$ cargo run -- body poem.txt
I'm nobody! Who are you?
Are you nobody, too?
How dreary to be somebody!

$ cargo run -- monomorphization poem.txt
(Hiçbir çıktı yok - eşleşme bulunamadı)
```

---

## 📖 Bölüm 12.5: Çevre Değişkenleriyle Çalışma

### 🎯 Hedef
Büyük/küçük harf duyarlılığını çevre değişkeni ile kontrol edilebilir yapmak.

### 💡 Neden Çevre Değişkeni?

- Kullanıcı bir kez ayarlar, tüm oturum boyunca geçerli olur
- Komut satırı argümanı her seferinde yazılmak zorunda değil

### 📝 Adım 1: Yeni Test Ekleme

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
```

### 📝 Adım 2: search_case_insensitive() Fonksiyonu

```rust
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
```

**to_lowercase() ne yapıyor?**
- String'i küçük harfe çevirir
- Yeni bir `String` oluşturur (String slice değil!)
- Bu yüzden `&query` kullanıyoruz

### 📝 Adım 3: Config'e ignore_case Ekleme

```rust
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,  // ← Yeni alan
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Yetersiz argüman");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        
        // Çevre değişkenini kontrol et
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
```

**env::var() nasıl çalışır?**
- `IGNORE_CASE` değişkeni varsa → `Ok(değer)`
- Yoksa → `Err`
- `is_ok()` → Sadece var olup olmadığını kontrol eder

### 📝 Adım 4: run() Fonksiyonunu Güncelleme

```rust
use minigrep::{search, search_case_insensitive};

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
```

### 🧪 Test Etme

**Büyük/küçük harf duyarlı (varsayılan):**
```bash
$ cargo run -- to poem.txt
Are you nobody, too?
How dreary to be somebody!
```

**Büyük/küçük harf duyarsız:**
```bash
# Linux/macOS
$ IGNORE_CASE=1 cargo run -- to poem.txt

# Windows PowerShell
$ $Env:IGNORE_CASE=1; cargo run -- to poem.txt
```

Çıktı:
```
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

**"To" ile başlayan satırlar da geldi!** 🎉

---

## 📖 Bölüm 12.6: stdout Yerine stderr'e Yazma

### 🎯 Hedef
Hata mesajlarını standart çıktı (stdout) yerine standart hata (stderr) akışına yazmak.

### 💡 stdout ve stderr Farkı Nedir?

- **stdout** → Normal çıktı (dosyaya yönlendirilebilir)
- **stderr** → Hata mesajları (ekranda görünür kalır)

### 🔍 Mevcut Sorunu Gösterme

```bash
$ cargo run > output.txt
```

**Beklenen:** Hata mesajı ekranda görünmeli  
**Gerçekleşen:** Hata mesajı `output.txt` içine yazıldı!

```bash
$ cat output.txt
Problem parsing arguments: not enough arguments
```

### 📝 Çözüm: eprintln! Kullanma

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Argüman ayrıştırma sorunu: {err}");  // ← eprintln!
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Uygulama hatası: {e}");  // ← eprintln!
        process::exit(1);
    }
}
```

**println! vs eprintln!**
- `println!` → stdout'a yazar
- `eprintln!` → stderr'e yazar

### 🧪 Doğru Davranışı Test Etme

**Hata durumu:**
```bash
$ cargo run > output.txt
Problem parsing arguments: not enough arguments
```

✅ Hata mesajı ekranda göründü!  
✅ `output.txt` boş!

**Başarılı durum:**
```bash
$ cargo run -- to poem.txt > output.txt
```

✅ Ekranda hiçbir şey yok  
✅ `output.txt` içinde sonuçlar var:
```
Are you nobody, too?
How dreary to be somebody!
```

---

## 🎉 Tamamlanmış minigrep Projesi

### 📦 Proje Yapısı

```
minigrep/
├── Cargo.toml
├── poem.txt
└── src/
    ├── main.rs    (Program akışı)
    └── lib.rs     (İş mantığı ve testler)
```

### 📝 src/lib.rs (Tam Kod)

```rust
use std::fs;
use std::error::Error;

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
```

### 📝 src/main.rs (Tam Kod)

```rust
use std::env;
use std::error::Error;
use std::fs;
use std::process;

use minigrep::{search, search_case_insensitive};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Argüman ayrıştırma sorunu: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Uygulama hatası: {e}");
        process::exit(1);
    }
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Yetersiz argüman sayısı");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
```

---

## 🎓 Bu Bölümde Öğrendiklerimiz

### ✅ Temel Kavramlar

1. **Komut Satırı Argümanları**
   - `std::env::args()` ile argüman okuma
   - Iterator'ları `collect()` ile Vec'e dönüştürme

2. **Dosya I/O**
   - `fs::read_to_string()` ile dosya okuma
   - `Result` ve `?` operatörü ile hata yönetimi

3. **Hata Yönetimi**
   - `Result<T, E>` kullanımı
   - `unwrap_or_else()` ile zarif hata yönetimi
   - `Box<dyn Error>` ile dinamik hata tipleri

4. **Kod Organizasyonu**
   - `main()` ve `run()` ayrımı
   - `Config` struct ile konfigürasyon yönetimi
   - `lib.rs` ve `main.rs` ayrımı

5. **Test-Driven Development (TDD)**
   - Önce test yaz, sonra kodla
   - `#[test]` attribute kullanımı
   - `assert_eq!` ile doğrulama

6. **Çevre Değişkenleri**
   - `env::var()` ile değişken okuma
   - `is_ok()` ile varlık kontrolü

7. **stdout vs stderr**
   - `println!` vs `eprintln!`
   - Çıktı yönlendirme (`>`)

### 🚀 Bir Sonraki Adım

Bu bölümde closure'lar ve iterator'lara giriş yaptık. Bir sonraki bölümde (Bölüm 13) bu konuları derinlemesine öğreneceğiz!

---

## 💡 Pratik İpuçları

### 1. Hata Mesajlarını İyileştirme
```rust
// Kötü
.expect("Dosya okunamadı")

// İyi
.map_err(|e| format!("Dosya okuma hatası: {}", e))?
```

### 2. Daha İyi Argüman Yönetimi
Gerçek uygulamalarda `clap` gibi kütüphaneler kullanılır:
```rust
use clap::Parser;

#[derive(Parser)]
struct Config {
    query: String,
    file_path: String,
    #[arg(short, long)]
    ignore_case: bool,
}
```

### 3. Performans İyileştirmesi
Büyük dosyalar için satır satır okuma:
```rust
use std::io::BufRead;

let file = File::open(path)?;
let reader = BufReader::new(file);

for line in reader.lines() {
    let line = line?;
    if line.contains(query) {
        println!("{}", line);
    }
}
```

---

## 🎯 Özet

Bu bölümde **minigrep** projesi ile:
- ✅ Gerçek bir komut satırı aracı geliştirdik
- ✅ Dosya I/O işlemlerini öğrendik
- ✅ Hata yönetimini iyileştirdik
- ✅ TDD ile test yazmayı pratik ettik
- ✅ Çevre değişkenleri ile çalıştık
- ✅ stdout/stderr farkını anladık

Bu proje, Rust'ın **güvenli**, **hızlı** ve **modüler** yapısını gösteren mükemmel bir örnek! 🦀

**Tebrikler! Artık kendi komut satırı araçlarınızı geliştirebilirsiniz!** 🎉