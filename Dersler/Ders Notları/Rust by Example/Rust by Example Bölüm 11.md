Merhaba! Bugün Rust programlama dilinin temel yapı taşlarından biri olan **Crates (Sandıklar/Paketler)** konusunu, *Rust By Example* dokümantasyonunu temel alarak, adım adım ve bir ders niteliğinde inceleyeceğiz. Hazırsanız, kahvenizi alın ve başlayalım! 🦀

---

### 1. Bölüm: Crate Nedir? (Temel Kavramlar)

Rust'ta kod yazmaya başladığınızda sıkça duyacağınız ilk terimlerden biri "Crate"tir. Peki, crate tam olarak nedir?

Rust'ta bir **crate, temel derleme birimidir (compilation unit)** [[1]]. Yani derleyici (`rustc`) kodunuzu derlerken dosyaları tek tek değil, bütünsel bir "crate" olarak ele alır. Örneğin, terminalde `rustc some_file.rs` komutunu çalıştırdığınızda, `some_file.rs` dosyası bir **crate dosyası** olarak kabul edilir. 

Eğer bu ana dosyanın içinde `mod` (modül) bildirimleri varsa, derleyici asıl derleme işlemine başlamadan *önce* bu modüllerin içerikleri ana crate dosyasının içine dahil edilir [[1]].
**Buradaki en kritik kural şudur:** Modüller tek başlarına derlenmezler, sadece **crate'ler** derlenir!

Bir crate iki farklı şekilde derlenebilir:
1. **Binary (Çalıştırılabilir):** Varsayılan olarak `rustc` bir crate'i çalıştırılabilir bir programa (binary) dönüştürür [[1]]. Yani `main` fonksiyonu olan ve doğrudan işletim sisteminde çalıştırabileceğiniz bir dosya üretir.
2. **Library (Kütüphane):** Başka programlar tarafından kullanılmak üzere tasarlanmış, kendi başına çalışmayan fonksiyon, struct (yapı) ve trait (özellik) bütünüdür. Varsayılan davranışı değiştirmek için derleyiciye `--crate-type=lib` bayrağı (flag) gönderilir [[1]].

---

### 2. Bölüm: Bir Kütüphane Crate'i Oluşturmak (Creating a Library)

Gelin birlikte basit bir kütüphane yazalım ve onu nasıl derleyeceğimize bakalım. Amacımız başka yerlerde kullanabileceğimiz, içinde bazı fonksiyonlar barındıran `rary` adında bir kütüphane oluşturmak.

`rary.rs` adında bir dosya oluşturalım:

```rust
pub fn public_function() {
    println!("rary'nin `public_function()` fonksiyonu çağrıldı");
}

fn private_function() {
    println!("rary'nin `private_function()` fonksiyonu çağrıldı");
}

pub fn indirect_access() {
    print!("rary'nin `indirect_access()` fonksiyonu çağrıldı, o da şunu yapar:\n> ");
    private_function();
}
```

**Öğretmenin Notu (Görünürlük / Visibility):** 
Burada dikkat etmemiz gereken çok önemli bir Rust kuralı var. Rust'ta her şey varsayılan olarak **özeldir (private)**. Bir fonksiyonun veya yapının crate dışından erişilebilir olmasını istiyorsanız, başına mutlaka `pub` anahtar kelimesini eklemelisiniz.
*   `public_function` ve `indirect_access` başında `pub` olduğu için dışarıya açıktır.
*   `private_function` ise sadece bu dosya içinde kullanılabilir. Ancak gördüğünüz gibi, açık (pub) olan `indirect_access` fonksiyonu, kendi içinden kapalı (private) olan `private_function` fonksiyonunu rahatça çağırabilir.

**Peki bunu nasıl kütüphaneye dönüştüreceğiz?**
Terminali açıyoruz ve `rustc`'ye bunu bir çalıştırılabilir dosya değil, bir kütüphane olarak derlemesini söylüyoruz:

```bash
$ rustc --crate-type=lib rary.rs
```

Bu komutu çalıştırdığınızda dizinde `lib` ile başlayan bir dosya göreceksiniz:
```bash
$ ls lib*
library.rlib
```
Kütüphaneler varsayılan olarak dosya adını alır ve başına otomatik olarak `lib` öneki gelir. Çıkan `.rlib` (Rust Library) uzantılı dosya, bizim derlenmiş kütüphanemizdir [[1]]. Eğer bu varsayılan ismi değiştirmek isterseniz, derleme sırasında `--crate-name` seçeneğini kullanabilirsiniz [[1]].

---

### 3. Bölüm: Kütüphaneyi Kullanmak (Using a Library)

Harika, elimizde artık `library.rlib` adında derlenmiş bir kütüphanemiz var! Peki, başka bir projede bu kütüphaneyi nasıl kullanacağız?

Yeni bir dosya oluşturalım: `executable.rs`

```rust
// extern crate rary; // Rust 2015 sürümü veya öncesi için gerekebilir. 
// Modern Rust'ta (2018+) genellikle doğrudan kullanılır.

fn main() {
    // Kütüphaneyi bir modül gibi kullanarak içindeki public fonksiyonu çağırıyoruz.
    rary::public_function();

    // HATA! `private_function` özeldir, dışarıdan doğrudan erişilemez.
    // rary::private_function(); 

    // Ancak public bir fonksiyon aracılığıyla private olana dolaylı yoldan erişebiliriz.
    rary::indirect_access();
}
```

Bu kodu derlerken `rustc`'ye bizim derlenmiş kütüphanemizi nereden bulacağını ve ona hangi isimle bağlanacağını söylememiz gerekir. Bunun için `--extern` bayrağını kullanırız:

```bash
$ rustc executable.rs --extern rary=library.rlib && ./executable
```
*(Burada `rary=library.rlib` diyerek derleyiciye "rary ismindeki modülün kodları library.rlib dosyasının içindedir" demiş oluyoruz.)*

**Çıktımız şu şekilde olacaktır:**
```text
rary'nin `public_function()` fonksiyonu çağrıldı
rary'nin `indirect_access()` fonksiyonu çağrıldı, o da şunu yapar:
> rary'nin `private_function()` fonksiyonu çağrıldı
```

**Buradan Çıkarmamız Gereken Dersler:**
1. **Modül Gibi Davranır:** `--extern` ile bağladığınız kütüphane, kodunuzun içinde sanki alt bir modülmüş gibi davranır [[2]]. `rary::fonksiyon_adi` şeklinde erişim sağlarsınız.
2. **Gizlilik Sınırları Keskindir:** `rary::private_function()` satırının yorum satırından çıkarıp derlemeye çalışsaydık, Rust derleyicisi bizi *"Bu fonksiyon özeldir (private), erişemezsin!"* diyerek durdururdu. Rust'ın güvenlik felsefesi tam olarak burada devreye girer: Geliştirici neyi dışarıya açacağını (`pub`) kendisi belirler [[2]].
3. **Dolaylı Erişim:** `private_function`'a dışarıdan kimse dokunamaz, ancak kütüphanenin kendi içindeki `pub` fonksiyonlar (örneğin `indirect_access`) bu gizli hazineleri kullanabilir ve dışarıya bir servis sunabilir.

---

### Kapanış ve Modern Rust (Cargo) Notu 🚀

Bu dersimizde işin "mutfağını", yani `rustc` derleyicisinin arka planda neler yaptığını görmek için komutları manuel olarak yazdık. Ancak gerçek dünya projelerinde bu uzun terminal komutlarını (`--extern`, `--crate-type` vb.) tek tek yazmak zorunda değilsiniz! 

Rust'ın resmi paket yöneticisi olan **Cargo**, bu işleri sizin için arka planda otomatik olarak halleder. `Cargo.toml` dosyanıza bir bağımlılık (dependency) eklediğinizde, Cargo kütüphaneyi crates.io üzerinden indirir, doğru crate türünde derler ve `--extern` bayraklarını sizin yerinize ayarlar.

Umarım bu detaylı anlatım, Rust'taki Crate mantığını, derleme birimlerini ve görünürlük (visibility) ilişkisini zihninizde netleştirmiştir. Başka bir Rust dersinde görüşmek üzere, iyi kodlamalar!

# DeepSeek Yorumu --------------------

## Rust'ta Crate (Paket/Kasa) Kavramı

Rust'ta **crate**, derlemenin en temel birimidir. Programınız ne kadar büyük olursa olsun, Rust derleyicisi (`rustc`) her seferinde bir **crate**'i derler. Modüller (`mod`) tek başına derlenmez; modül dosyalarının içeriği, `mod` bildirimlerinin bulunduğu yerlere yerleştirildikten sonra tüm crate bir bütün olarak derlenir.

---

### 1. Crate Türleri: Binary ve Library

Bir crate iki farklı çıktı türüne dönüştürülebilir:

| Çıktı Türü | Açıklama |
|------------|----------|
| **Binary (İkili)** | Çalıştırılabilir bir program. Varsayılan davranış budur. |
| **Library (Kütüphane)** | Başka projelerin kullanabileceği bir kod kütüphanesi. |

```bash
# Varsayılan olarak rustc binary üretir
$ rustc main.rs          # main çalıştırılabilir dosyasını oluşturur

# --crate-type ile kütüphane olarak derlenebilir
$ rustc --crate-type lib lib.rs   # liblib.rlib kütüphane dosyasını oluşturur
```

Bu davranış, `rustc`'ye `--crate-type` bayrağı geçilerek değiştirilebilir.

---

### 2. `crate_type` ve `crate_name` Öznitelikleri (Attributes)

Rust, derleyiciye doğrudan kaynak kodunun içinden talimat vermek için **öznitelikleri** (`attributes`) kullanır. Crate düzeyinde iki önemli öznitelik vardır:

#### `crate_type`

Crate'in nasıl derleneceğini belirtir:

```rust
// Bu crate bir kütüphane olarak derlenecek
#![crate_type = "lib"]
```

#### `crate_name`

Crate'in adını belirler:

```rust
// Bu crate'in adı "rary" olacak
#![crate_name = "rary"]
```

Bu iki öznitelik bir arada kullanıldığında, `rustc`'ye ayrıca `--crate-type` veya `--crate-name` bayrağı geçmeye gerek kalmaz.

---

### 3. Tam Örnek: Kütüphane Crate'i Oluşturma

Aşağıda, `crate_type` ve `crate_name` özniteliklerinin birlikte kullanıldığı bir örnek verilmiştir:

```rust
// Bu crate bir kütüphanedir
#![crate_type = "lib"]

// Kütüphanenin adı "rary"dir
#![crate_name = "rary"]

// Dışarıdan erişilebilen (public) fonksiyon
pub fn public_function() {
    println!("called rary's `public_function()`");
}

// Dışarıdan erişilemeyen (private) fonksiyon
fn private_function() {
    println!("called rary's `private_function()`");
}

// Public fonksiyon içinden private fonksiyonu çağıran bir arabirim
pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");
    private_function();
}
```

Bu dosyayı `lib.rs` olarak kaydedip derlediğinizde:

```bash
$ rustc lib.rs
$ ls lib*
library.rlib   # Kütüphane dosyası oluşur
```

---

### 4. Cargo Kullanımında Özniteliklerin Durumu

**Çok önemli bir nokta:** `crate_type` ve `crate_name` öznitelikleri, Rust'ın resmî paket yöneticisi **Cargo** kullanıldığında **hiçbir etki göstermez**.

Bunun nedeni, Cargo'nun `Cargo.toml` dosyasındaki ayarlara göre crate türünü ve adını otomatik olarak yönetmesidir:

```toml
# Cargo.toml
[package]
name = "my_project"   # crate adı buradan gelir

[[bin]]
name = "my_bin"       # binary adı

[lib]
name = "my_lib"       # kütüphane adı
crate-type = ["lib"]  # kütüphane türü
```

Günümüzde çoğu Rust projesi Cargo kullandığı için, `crate_type` ve `crate_name` öznitelikleri gerçek dünya projelerinde nadiren doğrudan kullanılır.

---

### 5. Crate ve Modül İlişkisi

Bir crate, birden fazla modülden oluşabilir. `mod` bildirimleri, derleyiciye diğer dosyaların nerede bulunacağını söyler:

```
src/
├── main.rs          # Ana crate dosyası (binary)
├── lib.rs           # Ana crate dosyası (library)
├── module1.rs       # Bir modül
└── module2/
    └── mod.rs       # Alt modül
```

```rust
// main.rs
mod module1;          // module1.rs'yi dahil eder
mod module2;          // module2/mod.rs'yi dahil eder

fn main() {
    // ...
}
```

Derleyici, tüm modülleri ana crate dosyasına yerleştirdikten sonra **tek bir crate** olarak derleme yapar.

---

### 6. Özet

| Kavram | Açıklama |
|--------|----------|
| **Crate** | Rust'ta en küçük derleme birimi. |
| **Binary Crate** | Çalıştırılabilir program üretir (varsayılan). |
| **Library Crate** | Başka projelerin kullanabileceği kütüphane üretir. |
| **`crate_type`** | Crate'in türünü belirten öznitelik. |
| **`crate_name`** | Crate'in adını belirten öznitelik. |
| **Cargo** | `crate_type` ve `crate_name` özniteliklerini geçersiz kılar; bunun yerine `Cargo.toml` kullanılır. |

Rust'ta crate'ler, kodunuzu modüller halinde düzenlemenize ve bu modülleri tek bir derleme biriminde birleştirmenize olanak tanır. Küçük projelerde doğrudan `rustc` ile çalışırken `crate_type` ve `crate_name` kullanışlı olabilir; ancak gerçek dünya projelerinde Cargo bu işleri sizin için otomatik olarak halleder.

> 🔗 **İleri Okuma:** Rust By Example'daki [Cargo](https://doc.rust-lang.org/rust-by-example/cargo.html) bölümü, crate'leri yönetmenin modern ve önerilen yolunu ayrıntılı olarak anlatır.
