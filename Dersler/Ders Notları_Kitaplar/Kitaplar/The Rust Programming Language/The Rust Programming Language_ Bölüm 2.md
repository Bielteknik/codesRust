# 🦀 Ders Notları: Bölüm 2 - Guessing Game Tutorial

# 🎓 Rust Dersleri: Bölüm 2 - Tahmin Oyunu Eğitimi (Guessing Game Tutorial)

Hoş geldiniz! Bu derste Rust programlama diline, interaktif bir "Tahmin Oyunu" yaparak adım adım giriş yapacağız. Bu ders; **değişkenler**, **match ifadesi**, **metotlar**, **ilişkili fonksiyonlar**, **dış kasa kullanımı (crate)**, **hata yönetimi** ve **döngüler** gibi temel kavramları uygulamalı olarak öğretecek.

---

## 📋 Dersin Akışı
1. [Proje Kurulumu](#1-proje-kurulumu)
2. [Kullanıcı Girdisi Alma](#2-kullanıcı-girdisi-alma)
3. [Rastgele Sayı Üretme (rand Crate)](#3-rastgele-sayı-üretme)
4. [Sayıları Karşılaştırma (Ordering ve match)](#4-sayıları-karşılaştırma)
5. [Tip Dönüşümü ve Shadowing](#5-tip-dönüşümü-ve-shadowing)
6. [Döngü ile Tekrarlama (loop)](#6-döngü-ile-tekrarlama)
7. [Hata Yönetimi (match ile Result)](#7-hata-yönetimi)
8. [Son Kod ve Özet](#8-son-kod-ve-özet)

---

## 🎯 Oyunun Mantığı Nedir?

Program 1 ile 100 arasında rastgele bir sayı üretir. Oyuncu bir tahmin girer. Program tahminin **çok küçük**, **çok büyük** veya **doğru** olduğunu söyler. Doğru tahmin edilirse oyun biter.

---

## 1. Proje Kurulumu

İlk olarak `cargo` ile yeni bir proje oluşturalım:

```bash
$ cargo new guessing_game
$ cd guessing_game
```

Bu komutlar bize şu yapıyı verir:

**Cargo.toml** (Proje yapılandırma dosyası):
```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2024"

[dependencies]
```

**src/main.rs** (Başlangıç kodu):
```rust
fn main() {
    println!("Hello, world!");
}
```

Projenin çalıştığını doğrulamak için:
```bash
$ cargo run
```

> 💡 **Ders Notu:** `cargo new` otomatik olarak "Hello, world!" programı oluşturur. `cargo run` hem derler hem de çalıştırır.

---

## 2. Kullanıcı Girdisi Alma

Oyuncunun tahminini alabilmek için `src/main.rs` dosyasını güncelleyelim:

```rust
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

### 🔍 Satır Satır İnceleme

### 📌 `use std::io;`
Rust'ta standart kütüphanedeki (`std`) modülleri kullanabilmek için `use` anahtar kelimesiyle **kapsama (scope)** almamız gerekir. `io` modülü giriş/çıkış işlemleri için gereklidir.

> 🎓 **Prelude Nedir?** Rust, sık kullanılan tipleri otomatik olarak her modüle dahil eder (prelude). Ancak `io` gibi bazı modüller prelude'da değildir, bu yüzden `use` ile manuel eklenmelidir.

### 📌 `let mut guess = String::new();`
- **`let`**: Yeni bir değişken tanımlar.
- **`mut`**: Rust'ta değişkenler varsayılan olarak **değişmezdir (immutable)**. `mut` ekleyerek onu **değişken (mutable)** yaparız. Çünkü kullanıcının girdisini bu değişkene yazacağız.
- **`String::new()`**: `String` tipinde yeni, boş bir string oluşturur. `::new` ile tipin **ilişkili fonksiyonunu (associated function)** çağırıyoruz.

### 📌 `io::stdin().read_line(&mut guess)`
- **`io::stdin()`**: Standart giriş (klavye) tutamacını döndürür.
- **`.read_line(&mut guess)`**: Kullanıcının girdiği satırı `guess` değişkenine yazar.
- **`&mut guess`**: **Referans (reference)** kullanıyoruz. Böylece `read_line` fonksiyonu, `guess` değişkenini **doğrudan değiştirebilir**. `&` referans, `mut` ise değişken referansı anlamına gelir.

### 📌 `.expect("Failed to read line")`
`read_line` bir **`Result`** tipinde değer döndürür. `Result` bir **enum** (sıralama) olup iki varyanta sahiptir:
- **`Ok`**: İşlem başarılı.
- **`Err`**: İşlem başarısız.

`expect` metodu, eğer sonuç `Err` ise programı çökertir ve verdiğiniz mesajı gösterir. `Ok` ise içindeki değeri döndürür.

> 💡 **Neden önemli?** Rust'ta hataları görmezden gelemeyiz. `expect` kullanmak, "Bu hata olursa program dursun ve bana bunu söyle" demektir.

---

## 3. Rastgele Sayı Üretme

Rust'ın standart kütüphanesinde rastgele sayı üretme yoktur. Bunun için **`rand`** adında harici bir **crate** (paket) kullanacağız.

### 📦 Crate Nedir?
- **Binary crate**: Çalıştırılabilir program (bizim projemiz).
- **Library crate**: Başka programlarda kullanılmak üzere yazılmış kod kütüphanesi (`rand` gibi).

### 🔧 Cargo.toml'a Bağımlılık Ekleme

```toml
[dependencies]
rand = "0.8.5"
```

Bu satırı ekledikten sonra `cargo build` çalıştırırsak Cargo, `rand` kütüphanesini [crates.io](https://crates.io) üzerinden indirir.

> 🎓 **Sürüm Yönetimi:** `"0.8.5"` yazmak, Cargo'ya "0.8.5 ile uyumlu en son sürümü kullan" der. 0.9.0 gibi büyük sürümler API uyumsuzluğu içerebileceğinden otomatik alınmaz.

### 🎲 Rastgele Sayı Kodu

```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");
}
```

### 🔍 Açıklama:
- **`use rand::Rng;`**: `Rng` bir **trait**'tir (özellik). Rastgele sayı üreticilerinin uygulaması gereken metotları tanımlar. Bu trait'i kapsama almadan `gen_range` kullanamayız.
- **`rand::thread_rng()`**: Mevcut iş parçacığına (thread) özel, işletim sistemi tarafından beslenen bir rastgele sayı üreticisi verir.
- **`gen_range(1..=100)`**: 1 ile 100 arasında (her iki sınır dahil) rastgele bir sayı üretir. `..=` operatörü **kapalı aralık** anlamına gelir.

---

## 4. Sayıları Karşılaştırma

Şimdi kullanıcının tahmini ile gizli sayıyı karşılaştıralım:

```rust
use std::cmp::Ordering;

// ... main içinde:
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
}
```

### 🔍 Kavramlar:

### 📌 `std::cmp::Ordering`
Bu bir **enum** olup üç varyantı vardır:
- `Less` (küçük)
- `Greater` (büyük)
- `Equal` (eşit)

### 📌 `.cmp()` Metodu
Her karşılaştırılabilir tipin `cmp` metodu vardır. İki değeri karşılaştırır ve `Ordering` enum'undan bir varyant döndürür.

### 📌 `match` İfadesi
`match`, Rust'ın en güçlü yapılarından biridir. Bir değeri çeşitli **kalıplarla (pattern)** eşleştirir ve eşleşen kolu çalıştırır:

```
match <değer> {
    <kalıp1> => <kod1>,
    <kalıp2> => <kod2>,
    ...
}
```

> ⚠️ **Dikkat:** Bu aşamada kod **derlenmez!** Çünkü `guess` bir `String`, `secret_number` ise bir sayıdır. Rust **güçlü ve statik tipli** bir dil olduğundan farklı tipleri karşılaştıramazsınız.

---

## 5. Tip Dönüşümü ve Shadowing

String'i sayıya dönüştürmek için:

```rust
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

### 🔍 Satır Satır:

### 📌 `let guess: u32`
- Yeni bir `guess` değişkeni tanımlıyoruz.
- **`: u32`** ile tipini **işaretsiz 32-bit tam sayı** olarak belirtiyoruz.
- Aynı isimde bir değişken zaten var! Rust buna izin verir, buna **Shadowing (Gölgeleme)** denir. Yeni değişken, eskisini gölgeler.

> 💡 **Shadowing Neden Kullanılır?** Tip dönüşümlerinde aynı ismi korumak kodu temiz tutar. `guess_str` ve `guess` gibi iki farklı isim kullanmak yerine, tek isimle devam ederiz.

### 📌 `.trim()`
String'in başındaki ve sonundaki **boşlukları** siler. Kullanıcı Enter'a bastığında string'e `\n` (veya Windows'ta `\r\n`) eklenir. `trim` bunu temizler.

```
"5\n"  →  "5"
```

### 📌 `.parse()`
String'i başka bir tipe çevirir. `: u32` ile Rust'a "bu sayıyı `u32` olarak yorumla" demiş oluyoruz. Bu bilgi `secret_number`'ın da `u32` olarak çıkarımlanmasını sağlar. Böylece **artık iki değeri karşılaştırabiliriz!**

### 📌 `.expect()`
Eğer kullanıcı sayı yerine "abc" yazarsa `parse` başarısız olur (`Err` döner). `expect` bu durumda programı çökertir.

---

## 6. Döngü ile Tekrarlama (loop)

Oyuncu sadece bir kez tahmin yapmasın, bildiği kadar denesin!

```rust
loop {
    println!("Please input your guess.");
    // ... girdi alma ve karşılaştırma kodları ...
    
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;  // Döngüden çık!
        }
    }
}
```

### 🔍 Kavramlar:
- **`loop`**: Sonsuz döngü oluşturur.
- **`break`**: Döngüden çıkar. Oyuncu bildiğinde oyunu bitirmek için kullanılır.

---

## 7. Hata Yönetimi

Şu anda kullanıcı sayı yerine harf yazarsa program çöküyor. Bunu düzeltelim:

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

### 🔍 Yeni Yaklaşım:

Artık `expect` yerine **`match`** kullanıyoruz:
- **`Ok(num) => num`**: Parse başarılıysa, içindeki sayıyı al ve `guess` değişkenine ata.
- **`Err(_) => continue`**: Parse başarısızsa, hatayı görmezden gel (`_` her şeyi yakalar) ve **`continue`** ile döngünün başına dön.

> 💡 **`continue`**: Döngünün o iterasyonunu atlar, bir sonraki iterasyona geçer. Kullanıcıya "Lütfen tekrar dene" fırsatı verir.

Artık kullanıcı "foo" yazsa bile program çökmez, sadece yeni bir tahmin ister!

---

## 8. Son Kod ve Özet

İşte tamamlanmış tahmin oyunumuz:

```rust
use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

### 🎮 Örnek Çalışma:
```
$ cargo run
Guess the number!
Please input your guess.
10
You guessed: 10
Too small!
Please input your guess.
99
You guessed: 99
Too big!
Please input your guess.
foo
Please input your guess.
61
You guessed: 61
You win!
```

---

## 📚 Bu Derste Öğrendiklerimiz

| Kavram | Açıklama |
|--------|----------|
| `use` | Modülleri kapsama alma |
| `let mut` | Değişken (mutable) değişken tanımlama |
| `String::new()` | Yeni boş string oluşturma |
| `io::stdin().read_line()` | Kullanıcıdan girdi alma |
| `&mut` | Değişken referansı geçirme |
| `Result`, `Ok`, `Err` | Hata yönetimi için enum |
| `expect()` | Hata durumunda çökme |
| `crate` | Harici paket (Cargo ile yönetilir) |
| `rand::Rng` trait | Rastgele sayı üretme |
| `gen_range(1..=100)` | Belirli aralıkta rastgele sayı |
| `std::cmp::Ordering` | Karşılaştırma sonuçları |
| `cmp()` | İki değeri karşılaştırma |
| `match` | Kalıp eşleştirme |
| `trim()` | Boşlukları silme |
| `parse()` | String'i sayıya çevirme |
| `: u32` | Tip belirtme (type annotation) |
| **Shadowing** | Aynı isimle yeni değişken |
| `loop` | Sonsuz döngü |
| `break` | Döngüden çıkma |
| `continue` | Sonraki iterasyona geçme |

---

## 🚀 Sonraki Adımlar

Bu ders Rust'ın temellerini pratik bir şekilde gösterdi. İlerleyen bölümlerde:
- **Bölüm 3**: Değişkenler, veri tipleri ve fonksiyonlar (detaylı)
- **Bölüm 4**: **Ownership** (Mülkiyet) - Rust'ı diğer dillerden ayıran temel özellik
- **Bölüm 5**: Struct'lar ve metot sözdizimi
- **Bölüm 6**: Enum'ların derinlemesine incelenmesi

> 🎯 **Ödev:** Bu kodu çalıştırın, `gen_range` aralığını değiştirin, farklı hata mesajları ekleyin ve `match` kollarına yeni durumlar ekleyerek deneyler yapın!

Başarılar! 🦀

# DeepSeek Yorumu ------------------------

Herkese merhaba! Bugün sizlerle Rust programlama dilinin resmi kitabı olan “The Rust Programming Language” (genelde “the book” diye anılır) 2. bölümünü, yani “Tahmin Oyunu” (Guessing Game) eğitimini, adeta bir sınıfta ders anlatır gibi en ince ayrıntısına kadar işleyeceğiz. Bu bölüm, Rust’ın birçok temel konseptini pratik bir proje üzerinden öğretmek için özel olarak tasarlanmıştır. Hazırsanız başlayalım.

---

## Bölüme Genel Bakış

Bu bölümde, klasik bir “sayı tahmin oyunu” yazacağız. Program rastgele bir tam sayı üretecek, kullanıcıdan tahminler alacak, tahminin doğru olup olmadığını kontrol edecek ve doğru bilene kadar tekrar tahmin isteyecek. Bu basit oyun sayesinde şu Rust özelliklerini canlı olarak göreceğiz:

- `let`, `match` gibi temel sözdizimi,
- standart kütüphaneden girdi/çıktı (`std::io`),
- harici kütüphane (crate) kullanımı (`rand`),
- değişkenler ve değişebilirlik (`mut`),
- hata yönetimi (`Result`, `expect`, `match`),
- döngüler (`loop`, `break`, `continue`),
- tür dönüşümleri ve karşılaştırmalar.

Amacımız sadece çalışan bir program yazmak değil, Rust’ın felsefesini anlamak. Hemen kolları sıvayalım.

---

## 1. Yeni Bir Proje Oluşturmak (Setting Up a New Project)

Önce terminali açalım ve yeni bir Rust projesi oluşturalım. Rust’ın paket yöneticisi ve derleme sistemi olan **Cargo**’yu kullanacağız.

```bash
cargo new guessing_game
cd guessing_game
```

Bu komut, `guessing_game` adında bir klasör oluşturur. İçinde `Cargo.toml` (proje bilgilerini ve bağımlılıkları tutan dosya) ve `src/main.rs` (ana kaynak kod dosyası) bulunur. `cargo new` aynı zamanda bir Git deposu başlatır ve `.gitignore` ekler. Projeyi derleyip çalıştırmak için:

```bash
cargo run
```

Şu an ekrana “Hello, world!” yazdıran bir şablonumuz var. `main.rs` dosyasını açalım ve içini aşağıdaki kodla değiştirelim (ama şimdilik sadece ilk kısmı):

```rust
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

Şimdi bu kodu satır satır anlatalım.

---

## 2. Tahmini İşlemek (Processing a Guess)

Yukarıdaki kod parçası, bir tahmin almanın ilkel halidir. Her bir parçayı masaya yatıralım.

### `use std::io;`
`std::io` Rust’ın standart girdi/çıktı kütüphanesidir. Kullanıcıdan veri almak için `stdin()` fonksiyonunu buradan kullanacağız. Eğer `use` ile kapsama almazsak, her seferinde `std::io::stdin()` yazmamız gerekir. Şimdilik sadece ihtiyacımız olanı içe aktardık.

### `fn main() { ... }`
Programın giriş noktası `main` fonksiyonudur. Rust’ta `fn` anahtar kelimesi fonksiyon tanımlar. Parantezler boşsa parametre yok demektir, gövde süslü parantezlerle başlar.

### `println!("Guess the number!");`
`println!` bir **makro**dur. Makrolar normal fonksiyonlara benzer ama isimlerinin sonunda `!` bulunur. Makrolar derleme zamanında kod üretir; `println!` biçimlendirilmiş çıktıyı standart çıkışa yazar. İlk mesajımız oyunun başladığını belirtiyor.

### `println!("Please input your guess.");`
Kullanıcıdan bir tahmin girmesini istiyoruz.

### `let mut guess = String::new();`
İşte kritik bir satır:
- `let` bir değişken tanımlar. Rust’ta değişkenler **varsayılan olarak değişmezdir** (immutable). Yani bir kere değer atadıktan sonra değiştirmeye çalışırsak derleme hatası alırız.
- `mut` anahtar kelimesi değişkeni **değişebilir** (mutable) yapar. İleride kullanıcının girdisini `guess` değişkenine yazacağımız için onu değiştirebilmemiz gerekiyor.
- `String::new()` boş, büyüyebilir bir UTF-8 karakter dizisi oluşturur. `::new()` bir türün ilişkili fonksiyonudur (statik fonksiyon gibi düşünebiliriz); `String` türü için yeni bir örnek döndürür.
- `guess` değişkenine bu yeni string’i bağlarız.

### `io::stdin().read_line(&mut guess).expect("Failed to read line");`
Bu satırı parçalara ayıralım:
- `io::stdin()` standart girdi için bir işleyici (handle) döndürür. Eğer başta `use std::io` yapmış olmasaydık `std::io::stdin()` yazardık.
- `.read_line(&mut guess)` bu işleyici üzerinde bir metot çağrısıdır. Metot, parametre olarak verilen mutable string referansına kullanıcının girdiği satırı ekler. `&mut guess` ile `guess` değişkenine **değişebilir bir referans** (mutable reference) veririz. `read_line` satır sonundaki `\n` karakterini de string’e katar.
- `read_line` işleminin bir dönüş tipi vardır: `io::Result`. Rust’ta birçok fonksiyon hata durumlarını belirtmek için `Result` tipini döndürür. `Result` iki varyantı olan bir enum’dır: `Ok` ve `Err`. `Ok` başarıyı ve içindeki değeri, `Err` ise hatayı ve hata bilgisini taşır.
- `.expect("Failed to read line")` bu `Result` üzerinde çağrılır. Eğer değer `Err` ise program `expect` içinde verdiğimiz mesajı göstererek **panikler** (panic) ve sonlanır. Eğer `Ok` ise, `expect` `Ok` içindeki değeri (bu örnekte okunan bayt sayısı) döndürür, ama biz onu kullanmadığımız için görmezden geliriz. Bu, basit hata yönetimidir; ileride daha sofistike yöntemler göreceğiz.
- Noktalı virgül (;) ifadeyi sonlandırır.

### `println!("You guessed: {}", guess);`
`println!` makrosunun biçimlendirme gücünü kullanıyoruz. Süslü parantezler `{}` yer tutucudur. İlk `{}` yerine `guess` değişkeninin değeri yazılır. Burada `guess` string’ini ekrana basıyoruz.

Programı `cargo run` ile çalıştırdığımızda, bir tahmin girip enter’a basınca girilen değeri geri yazdıracaktır. Ama henüz bir karşılaştırma yok.

---

## 3. Gizli Sayıyı Üretmek (Generating a Secret Number)

Şimdi programımızın kalbi olan rastgele sayı üretimini ekleyelim. Bunun için Rust ekosistemindeki `rand` **crate**’ini kullanacağız. Crate, Rust’ta bir kütüphane paketidir. `rand` kütüphanesi rastgele sayı üretimi için pek çok araç sağlar.

`Cargo.toml` dosyasını açalım ve `[dependencies]` bölümüne şu satırı ekleyelim:

```toml
[dependencies]
rand = "0.8.5"
```

`0.8.5` semantik sürümleme numarasıdır. Cargo, uyumlu en son sürümü otomatik indirir. `cargo build` (veya `cargo run`) çalıştırdığımızda Cargo, `rand` crate’ini ve onun bağımlılıklarını indirip derler. İndirilen kütüphaneler `Cargo.lock` dosyasında kilitlenir, böylece tekrarlanabilir derlemeler elde ederiz.

Şimdi `src/main.rs` dosyasını güncelleyelim. En üste `use rand::Rng;` ekleyelim ve `main` içine aşağıdaki gibi bir bölüm yazalım:

```rust
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}"); // Geliştirme amaçlı, sonra sileceğiz

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

### `use rand::Rng;`
`Rng` (Rastgele sayı üreteci) bir **trait**’tir. Trait’ler Rust’ta belirli bir davranışı tanımlar. `rand` crate’i içindeki birçok fonksiyonu kullanabilmek için `Rng` trait’inin kapsamda olması gerekir. `gen_range` metodu bu trait üzerinden çağrılır.

### `let secret_number = rand::thread_rng().gen_range(1..=100);`
- `rand::thread_rng()` bize o anki iş parçacığı için yerel, kriptografik olarak güvenli olmayan ama oyun için yeterli olan bir rastgele sayı üreteci verir.
- `.gen_range(1..=100)` `Rng` trait’inin bir metodudur. `1..=100` aralığı, 1 ile 100 **dahil** (inclusive) anlamına gelir. `1..100` yazsaydık 100 hariç olurdu. Bu metot, belirtilen aralıkta rastgele bir tam sayı üretir.
- `secret_number` değişkeni `mut` değil, çünkü değerini bir kere atadıktan sonra değiştirmeyeceğiz. Rust bunu zorunlu kılar.

### Gizli sayıyı ekrana yazdırdık:
`println!("The secret number is: {secret_number}");` satırı sadece test içindir, oyunun sonunda sileceğiz. Şimdilik tahminimizi kontrol etmeye başlamak için işimize yarayacak.

Programı çalıştırdığımızda (`cargo run`) her seferinde farklı bir sayı üretildiğini görebiliriz.

---

## 4. Tahmini Gizli Sayı ile Karşılaştırmak (Comparing the Guess to the Secret Number)

Artık elimizde bir kullanıcı tahmini ve bir gizli sayı var. Bunları karşılaştırmak istiyoruz. Ancak şöyle bir sorun var: `guess` değişkeni bir `String`, `secret_number` ise bir tam sayı (varsayılan olarak `i32`). Karşılaştırma yapmadan önce `guess` string’ini bir sayıya dönüştürmemiz gerek.

Kodu şöyle genişletelim (önceki `println!("You guessed: ...")` satırından sonra):

```rust
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // ... önceki kod ...

    println!("You guessed: {guess}");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

Bakalım neler olmuş.

### `use std::cmp::Ordering;`
`Ordering`, `cmp` (compare) modülünden bir enum’dır. Varyantları `Less`, `Greater`, `Equal`. İki değeri karşılaştırdığımızda dönen sonucu bu varyantlarla eşleştireceğiz.

### `let guess: u32 = guess.trim().parse().expect("Please type a number!");`
Bu satırda **gölgeleme** (shadowing) yapıyoruz. Rust aynı isimde yeni bir değişken tanımlamamıza izin verir; bu, eski `guess` (String) değişkenini gölgeler. Yeni `guess`, bu sefer `u32` türünde. Gölgeleme sayesinde `guess_str` gibi farklı isimler düşünmek zorunda kalmayız.

- `guess.trim()`: String’in başındaki ve sonundaki boşlukları ve özellikle `read_line` ile eklenen `\n` karakterini siler.
- `.parse()`: Bu metot, bir string’i başka bir türe dönüştürmeye çalışır. Dönüş tipi `Result`’tır. Burada `: u32` ile parse’ın hangi türe dönüşeceğini belirtiyoruz (tür çıkarımı olmadan `parse` hangi türe dönüşeceğini bilemez, bu yüzden açıkça belirtmek iyi).
- `.expect("Please type a number!")`: Eğer dönüştürme başarısız olursa (kullanıcı sayı dışında bir şey girerse) program panikler ve verdiğimiz mesajı yazdırır. Başarılı olursa `Ok` içindeki `u32` değeri `guess` değişkenine bağlanır.

### `match guess.cmp(&secret_number) { ... }`
- `guess.cmp(&secret_number)`: `guess` bir `u32`, `secret_number` da bir `u32`. `cmp` metodu iki değeri karşılaştırır ve `Ordering` enum’ı döndürür. Karşılaştırma için `secret_number`’ın referansını geçiyoruz (`&secret_number`).
- `match` ifadesi, bir değeri desenlere göre dallandırır. `Ordering::Less`, `::Greater`, `::Equal` varyantlarına karşılık gelen kollar yazarız. Her kol `=>` ile başlar ve bir ifade (veya blok) ile devam eder. İlk eşleşen kol çalışır.
- `Ordering::Less` durumunda `"Too small!"`, `Greater`’da `"Too big!"`, `Equal`’da `"You win!"` yazdırırız.

Programı çalıştırdığımızda artık tahminimize göre ipucu verir ve doğru tahminde tebrik eder.

---

## 5. Döngü ile Birden Çok Tahmine İzin Vermek (Allowing Multiple Guesses with Looping)

Şu an program sadece bir tahmin alıp sonlanıyor. Oyunu oynanabilir kılmak için doğru tahmine kadar tekrar tekrar sormalıyız. Bunun için `loop` anahtar kelimesini kullanırız.

`loop` Rust’ta sonsuz döngü oluşturur. Biz doğru tahminde `break` ile döngüden çıkacağız.

Kodu düzenleyelim: `println!("Guess the number!");` ile `secret_number` oluşturma kısmı döngü dışında kalmalı, çünkü gizli sayıyı bir kere üretiyoruz. Tahmin alma ve karşılaştırma kısmını `loop { ... }` bloğuna alalım. Kazanma durumunda `break;` ekleyelim.

Güncellenmiş `main` fonksiyonu (gizli sayıyı yazdıran satırı silelim):

```rust
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

Artık program doğru tahmin edilene kadar döngüde kalır. Kullanıcı yanlış tahminlerde ipucu alır, doğru tahminde oyun biter.

---

## 6. Geçersiz Girdiyi Ele Almak (Handling Invalid Input)

Oyun şu an oldukça iyi, ama bir eksiği var: Kullanıcı sayı dışında bir şey (örneğin “abc”) girerse `parse().expect(...)` programı panikletir ve sonlandırır. Bu, kötü bir kullanıcı deneyimi. Gerçek bir oyunda geçersiz girdi görmezden gelinmeli ve tekrar sorulmalıdır. `expect` yerine `match` kullanarak `parse`’ın dönüşündeki `Result`’ı işleyebiliriz.

`guess.trim().parse()` bir `Result<u32, _>` döndürür. Bunu `match` ile iki kola ayıralım:

- `Ok(num)` => sayıyı al, `num` değişkenine bağla, karşılaştırma yap.
- `Err(_)` => hatayı yazdır, `continue` ile döngünün bir sonraki iterasyonuna geç (tekrar tahmin iste).

Ayrıca kullanıcıya neyi yanlış yaptığını belirten bir mesaj verelim. Kod şöyle olur:

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => {
        println!("Please type a number!");
        continue;
    }
};
```

- `match` ifadesinin döndürdüğü değeri doğrudan `guess` değişkenine atayabiliriz.
- `Ok(num)` durumunda `num` değerini döndürürüz.
- `Err(_)` durumunda `_` kullanarak hata detaylarını yakalamayız, çünkü kullanıcıya genel bir mesaj yeterli. Mesajı yazdırıp `continue` ile döngünün başına döneriz. `continue` döngünün o iterasyonunu sonlandırır ve bir sonraki iterasyonu başlatır.
- `break` ifadesi de aynı şekilde `Ordering::Equal` kolunda kalır.

Programın son hali (gizli sayıyı gösteren `println!`'i tamamen kaldıralım):

```rust
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

Bu haliyle oyun tam anlamıyla bir “tahmin oyunu”dur. Kullanıcı geçersiz girdiğinde kibarca uyarılır ve döngü devam eder.

---

## Bölümün Özeti (Summary)

Bu bölümde, sıfırdan bir Rust projesi oluşturup, harici bir kütüphane kullanarak işlevsel bir oyun yazdık. Yol boyunca şunları öğrendik:

- **`let` ve `mut`**: Değişken tanımlama ve değişebilirliği kontrol etme.
- **`io::stdin` ve `read_line`**: Kullanıcıdan girdi alma.
- **`Result` ve `expect` / `match`**: Hata yönetiminin iki farklı seviyesi.
- **Gölgeleme (shadowing)**: Aynı isimle yeni değişken oluşturma.
- **`trim` ve `parse`**: String temizleme ve tip dönüşümü.
- **`match` ve `Ordering`**: Karşılaştırma sonucuna göre dallanma.
- **`loop`, `break`, `continue`**: Akış kontrolü.
- **Cargo ve dış crate kullanımı**: Bağımlılık ekleme, `Cargo.toml`, `Cargo.lock`.

Bu temeller, Rust’ın sahiplik (ownership), referanslar, ömürler (lifetimes) gibi daha derin konularına geçmeden önce sağlam bir zemin oluşturur. Özellikle `match` ve `Result` kullanımı Rust’taki hata yönetim felsefesinin çekirdeğidir: hataları görmezden gelmeyip açıkça ele alırız.

Bir sonraki bölümlerde bu kavramları çok daha derinlemesine inceleyeceğiz. Ama şimdilik elinizde çalışan bir oyun var. Deneyin, sayıyı değiştirin, mesela 1 ile 1000 arasında yapın, hatta tahmin hakkı sınırı koymayı deneyin. Rust’ı öğrenmenin en iyi yolu denemektir.

Umarım bu ders faydalı olmuştur. Sorularınız varsa çekinmeden sorabilirsiniz. Rust yolculuğunuzda başarılar!