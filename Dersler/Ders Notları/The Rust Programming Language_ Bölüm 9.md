# 🦀 Ders Notları: Bölüm 9 - Rust'ta Hata Yönetimi (Error Handling) — Kapsamlı Ders

> **Kaynak:** *The Rust Programming Language* — Chapter 9

Rust, hataları ele alma konusunda çoğu dilden farklı, oldukça titiz bir felsefeye sahiptir. Bu derste hata yönetimini **temelden ileri seviyeye**, bol örnek ve benzetmelerle işleyeceğiz. Hazırsanız başlayalım! 🚀

---

## 📚 Bölüm 0: Hatalara Genel Bakış — İki Büyük Kategori

Yazılımda hatalar kaçınılmazdır. Rust bu gerçeği kabul eder ve sizi kod yazarken hataları **düşünmeye zorlar**. Hatta çoğu durumda, bir hatayı göz ardı ederseniz kodunuz **derlenmez bile**. Bu, production'a geçmeden hataları yakalamanızı garanti eder.

Rust hataları **iki ana kategoriye** ayırır:

| Kategori | Açıklama | Rust'taki Karşılığı |
|---|---|---|
| **Kurtarılabilir (Recoverable)** | Dosya bulunamadı gibi, kullanıcıya bildirip işlemi tekrar deneyebileceğiniz durumlar | `Result<T, E>` |
| **Kurtarılamaz (Unrecoverable)** | Dizinin sonundan ötesine erişim gibi, bug belirtisi olan durumlar | `panic!` makrosu |

> ⚠️ **Önemli:** Rust'ta **exception (istisna) mekanizması yoktur.** Bu, Java/Python/C++ gibi dillerden gelenler için büyük bir farktır.

---

## 🔥 Bölüm 1: Kurtarılamaz Hatalar — `panic!` Makrosu

Bazen kodunuzda öyle bir şey olur ki, düzeltebileceğiniz bir şey değildir. İşte o an `panic!` sahneye çıkar.

### 1.1 `panic!` Nasıl Tetiklenir?

İki şekilde panic oluşur:
1. **Bizim yaptığımız bir hata** (örn. dizinin sınırları dışına çıkmak)
2. **Doğrudan `panic!` makrosunu çağırmak**

Basit bir örnek:

```rust
fn main() {
    panic!("crash and burn"); // "Çök ve yan"
}
```

Çıktı:
```
thread 'main' panicked at src/main.rs:2:5:
crash and burn
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Gördüğünüz gibi Rust bize:
- **Hangi dosyada** (`src/main.rs`)
- **Hangi satırda** (2. satır, 5. karakter)
- **Hangi mesajla** ("crash and burn")

panic olduğunu söylüyor.

### 1.2 Gerçek Hayattan Bir Örnek: Sınır Dışı Erişim

```rust
fn main() {
    let v = vec![1, 2, 3];
    v[99]; // Sadece 3 eleman var, 99. indis yok!
}
```

Çıktı:
```
thread 'main' panicked at src/main.rs:4:6:
index out of bounds: the len is 3 but the index is 99
```

> 🎯 **Neden bu önemli?** C gibi dillerde sınır dışı erişim **tanımsız davranış (undefined behavior)** üretir. Saldırganlar bundan faydalanarak bellekteki hassas verileri okuyabilir (**buffer overread**). Rust ise bunu **anında durdurarak** güvenliği garanti eder.

### 1.3 Unwinding vs Abort — Rust'a Nasıl Çökeceğini Söyleyin

Panic olduğunda Rust iki stratejiden birini izler:

| Strateji | Açıklama | Avantaj/Dezavantaj |
|---|---|---|
| **Unwinding** (varsayılan) | Stack'i geri sarar, her fonksiyondaki verileri temizler | Güvenli ama yavaş |
| **Abort** | Programı anında sonlandırır, temizliği OS'a bırakır | Hızlı ama bellek "kirli" kalır |

Eğer **binary boyutunu küçültmek** istiyorsanız, `Cargo.toml` dosyanıza şunu ekleyin:

```toml
[profile.release]
panic = 'abort'
```

### 1.4 Backtrace (Geri İz) — Hatanın Kaynağını Bulmak

Panic mesajındaki `note: run with RUST_BACKTRACE=1` satırını görmüşsünüzdür. Bu, hatanın **nereden geldiğini** gösteren fonksiyon çağrı zinciridir.

```bash
RUST_BACKTRACE=1 cargo run
```

Çıktı (kısaltılmış):
```
stack backtrace:
   0: rust_begin_unwind
   1: core::panicking::panic_fmt
   ...
   6: panic::main           ← BİZİM KODUMUZ!
             at ./src/main.rs:4:6
```

> 🔍 **Backtrace okuma kuralı:** Yukarıdan aşağıya doğru okuyun, **kendi yazdığınız dosyayı gördüğünüz yer** sorunlu yerdir. Onun üstündekiler Rust'ın iç kodları, altındakiler ise sizi çağıran kodlardır.

> ⚙️ Backtrace'ın çalışması için **debug symbol'ler** açık olmalı. `cargo build` veya `cargo run` (release olmadan) varsayılan olarak bunları açık tutar.

---

## 🛠️ Bölüm 2: Kurtarılabilir Hatalar — `Result<T, E>`

Hayattaki hataların çoğu programı çökertecek kadar ciddi değildir. Bir dosya açılamadığında programı öldürmek yerine, dosyayı oluşturmak isteyebilirsiniz. İşte `Result` burada devreye girer.

### 2.1 `Result` Enum Yapısı

```rust
enum Result<T, E> {
    Ok(T),   // Başarılı durum, T tipinde değer döner
    Err(E),  // Hata durumu, E tipinde hata döner
}
```

- `T`: Başarı durumunda dönecek değerin tipi
- `E`: Hata durumunda dönecek hatanın tipi

### 2.2 İlk Örnek: Dosya Açmak

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");
}
```

`File::open` fonksiyonunun dönüş tipi `Result<T, E>`'dir:
- `T` = `std::fs::File` (dosya tutamacı)
- `E` = `std::io::Error`

Yani fonksiyon ya başarılı olup bir dosya tutamacı verir, ya da hata bilgisi döner.

### 2.3 `match` ile Result İşleme

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Dosya açma sorunu: {error:?}"),
    };
}
```

Dosya yoksa çıktı:
```
thread 'main' panicked at src/main.rs:8:23:
Problem opening the file: Os { code: 2, kind: NotFound, 
    message: "No such file or directory" }
```

### 2.4 Farklı Hataları Farklı Şekilde İşleme

Ya dosya **bulunamadığı için** değil de, **izin olmadığı için** açılamadıysa? O zaman dosyayı oluşturmak isteriz!

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Dosya oluşturma sorunu: {e:?}"),
            },
            _ => {
                panic!("Dosya açma sorunu: {error:?}");
            }
        },
    };
}
```

> 📝 Burada iç içe üç `match` var! Mantık şu:
> 1. Dosya açılmaya çalışıldı
> 2. Başarısızsa → hata türüne bak
> 3. `NotFound` ise → dosyayı oluştur
> 4. Başka bir hata ise → panic

### 2.5 Closure ile Daha Temiz Kod: `unwrap_or_else`

İç içe `match`'ler okumayı zorlaştırır. Chapter 13'te closure'ları öğrendiğinizde şu写法 çok daha temiz görünecek:

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Dosya oluşturma sorunu: {error:?}");
            })
        } else {
            panic!("Dosya açma sorunu: {error:?}");
        }
    });
}
```

Aynı davranış, ama `match` yok! 🎉

### 2.6 Kısayol Metotları: `unwrap` ve `expect`

#### `unwrap` — "Ya değeri ver ya da çök"

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
}
```

- `Ok` ise → içindeki değeri döner
- `Err` ise → **panic!**

#### `expect` — `unwrap`'ın daha açıklamalı versiyonu

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt bu projede bulunmalı");
}
```

`expect`'in avantajı: **kendi hata mesajınızı** yazabilirsiniz. Production kodlarında çoğu Rustacean `unwrap` yerine `expect` tercih eder çünkü hata ayıklarken daha fazla bağlam sağlar.

> 💡 **İpucu:** `unwrap` "oldu oldu" demek, `expect` ise "olmalıydı, olmadı, işte sebebi" demektir.

### 2.7 Hata Yayma (Propagating Errors)

Bazen bir fonksiyon hatayı **kendisi işlemek yerine**, çağıran koda bırakmak ister. Çünkü çağıran kodun daha fazla bilgisi olabilir.

#### Manuel yöntem (match ile):

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),  // Hatayı çağıran koda iade et
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),     // Başarılı
        Err(e) => Err(e),          // Hatayı iade et
    }
}
```

Dönüş tipi `Result<String, io::Error>`:
- Başarılı → dosyadan okunan kullanıcı adı (`String`)
- Başarısız → `io::Error`

### 2.8 Sihirli Operatör: `?`

Rust, hata yaymayı o kadar sık yaptığımızı fark etmiş ki, bunun için özel bir operatör koymuş: `?`

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```

> ✨ `?` operatörünün yaptığı:
> - `Ok` ise → içindeki değeri çıkar, devam et
> - `Err` ise → **fonksiyondan erken dön**, hatayı çağıran koda iade et

Bu tek satır:
```rust
let mut username_file = File::open("hello.txt")?;
```

Aslında şunun kısaltmasıdır:
```rust
let mut username_file = match File::open("hello.txt") {
    Ok(file) => file,
    Err(e) => return Err(e),
};
```

### 2.9 `?` Operatörünün Gizli Gücü: `From` Trait'i

`?` sadece erken dönüş yapmaz, aynı zamanda **hata tipini dönüştürür**! Standart kütüphanedeki `From` trait'inin `from` fonksiyonunu çağırır.

Yani fonksiyonunuz `OurError` tipinde hata döndürüyorsa ve `impl From<io::Error> for OurError` tanımladıysanız, `?` operatörü `io::Error`'ü otomatik olarak `OurError`'e çevirir.

### 2.10 `?` ile Method Chaining

Kodu daha da kısaltabiliriz:

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
```

### 2.11 En Kısa Yolu: `fs::read_to_string`

Dosyayı string olarak okumak çok yaygın bir iş olduğu için standart kütüphane bunu tek satırda yapar:

```rust
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

> 🎓 Öğrenme notu: Uzun yolu önce görmek, kısayolun neden var olduğunu anlamak için önemlidir.

### 2.12 `?` Operatörünün Kullanım Kısıtlamaları

`?` operatörünü **her yerde** kullanamazsınız. Sadece dönüş tipi uyumlu fonksiyonlarda çalışır:

- `Result` değeri üzerinde `?` → fonksiyon `Result` dönmeli
- `Option` değeri üzerinde `?` → fonksiyon `Option` dönmeli

❌ **Yanlış örnek:**

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")?; // HATA!
}
```

Hata mesajı:
```
error[E0277]: the `?` operator can only be used in a function 
that returns `Result` or `Option`
```

### 2.13 `Option<T>` Üzerinde `?` Kullanımı

```rust
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() {
    assert_eq!(
        last_char_of_first_line("Hello, world\nHow are you?"),
        Some('d')
    );
    assert_eq!(last_char_of_first_line(""), None);
    assert_eq!(last_char_of_first_line("\nhi"), None);
}
```

Mantık:
- `text.lines().next()` → ilk satır yoksa `None` döner, `?` fonksiyondan erken döner
- Varsa → `chars().last()` ile son karakteri al

> ⚠️ `Result` ve `Option` arasında **otomatik dönüşüm YOKTUR**. Gerekirse `.ok()` veya `.ok_or()` metodlarıyla manuel dönüştürmelisiniz.

### 2.14 `main` Fonksiyonunda `?` Kullanımı

`main` fonksiyonu özel bir fonksiyondur ama `Result` dönebilir:

```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;
    Ok(())
}
```

Buradaki `Box<dyn Error>` → **"herhangi bir hata tipi"** anlamına gelir (trait object).

> 💻 **Çıkış kodları:**
> - `Ok(())` dönerse → program `0` ile biter (başarılı)
> - `Err(_)` dönerse → program `0` olmayan bir sayı ile biter (hatalı)
>
> Bu, C/C++ ile uyumluluk içindir.

---

## 🎯 Bölüm 3: `panic!` mı, `Result` mü? — Karar Rehberi

Peki ne zaman hangisini kullanmalıyız? İşte altın kurallar:

### ❌ `panic!` Kullanın, Eğer:
- Örnek kod / prototip yazıyorsanız
- Test yazıyorsanız (hızlı başarısızlık için)
- **Kesinlikle** hata olamayacağından eminseniz (örn. sabit bir dizinin elemanına erişim)
- Derleyiciyi ikna edemediğiniz durumlar (compiler'ı susturmak için)

### ✅ `Result` Kullanın, Eğer:
- Hatanın **beklenebilir** olduğu durumlar (dosya yok, ağ hatası, kullanıcı girdisi)
- Çağıran kodun hatayı işleyebileceği durumlar
- Kütüphane yazıyorsanız (kullanıcıya seçim hakkı bırakın)
- Production kalitesinde kod yazıyorsanız

### 🧠 Pratik Bir Kural:

> **Kötü bir şey olabileceğini ÖNCEDE BİLİYORSANIZ → `Result`**
> **Kötü bir şey olabileceğini SONRADAN FARK EDİYORSANIZ → `panic!`**

Örnek:
- `File::open()` → Dosya olmayabilir, bunu biliyoruz → `Result`
- `arr[100]` → Dizi 100 elemanlıysa sorun yok, ama değilse bug → `panic!`

---

## 📊 Özet Tablosu

| Özellik | `panic!` | `Result<T, E>` |
|---|---|---|
| Hata tipi | Kurtarılamaz | Kurtarılabilir |
| Davranış | Programı çökertir | Hatayı işlemenizi ister |
| Kullanım | Bug'lar, imkansız durumlar | Dosya/ağ/kullanıcı hataları |
| Kısayollar | — | `unwrap`, `expect`, `?` |
| `main`'de | Her zaman | `Result<(), E>` dönüş tipi ile |

---

## 🏁 Son Söz

Rust'ın hata yönetimi felsefesi şudur: **"Hataları görmezden gelemeyiz, onlarla yüzleşmeliyiz."** 

- `panic!` ile "Bu bir bug, dursun dünya" dersiniz.
- `Result` ile "Bu bir olasılık, hazırlıklı olalım" dersiniz.
- `?` operatörü ile "Hata yaymayı kolaylaştıralım" dersiniz.

Bu yaklaşım başta zahmetli gelse de, uzun vadede **çok daha sağlam, güvenilir ve bakımı kolay** kod yazmanızı sağlar. Rust'ın sizi derleme zamanında zorlaması, production'da sizi mutlu eder. 🦀✨

Bir sonraki adımda **generic tipler (Chapter 10)** ve **trait'ler** ile `Result`'ın gücünü daha da artırabilirsiniz. Başarılar! 🚀