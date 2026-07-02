
Merhaba! Rust programlama dilini öğrenmeye hoş geldin. Bugün, resmi Rust dokümantasyonunun en temel ve en önemli başlangıç noktası olan **"Rust By Example" (Örneklerle Rust)** kaynağının ilk bölümü olan **1. Hello World (Merhaba Dünya)** konusunu ve alt başlıklarını, tıpkı bir sınıf ortamında ders anlatır gibi adım adım, mantığını kavrayarak inceleyeceğiz [[17]].

Rust By Example'ın felsefesi şudur: Kodu oku, anla, derle ve çalıştır [[11]]. Hazırsan, derinlere inelim!

---

### 1. Hello World (Merhaba Dünya)
Her programlama dilinde olduğu gibi Rust'a da ekrana "Merhaba Dünya!" yazdırarak başlıyoruz [[1]]. Peki bu basit programın arkasında neler yatıyor?

```rust
fn main() {
    println!("Hello World!");
}
```

**Öğretmenin Notları:**
1. **`fn main()`:** Rust'ta (ve C/C++ gibi dillerde) çalışan her çalıştırılabilir (binary) programın bir giriş noktası olmalıdır. `main` fonksiyonu, işletim sisteminin programı çalıştırmaya başladığı yerdir.
2. **`println!` Makrosu:** Sonundaki ünlem işaretine (`!`) dikkat et. Rust'ta ünlem işareti, bunun sıradan bir fonksiyon değil, bir **makro (macro)** olduğunu gösterir. Makrolar, derleme zamanında (compile-time) kod üreten güçlü araçlardır. `println!`, metni konsola yazdırmak için kullanılır ve satır sonuna otomatik olarak yeni satır (`\n`) ekler.
3. **Derleme (`rustc`):** Rust, derlenen bir dildir. Kodumuzu `rustc hello.rs` komutuyla derlediğimizde bize `hello` adında çalıştırılabilir bir dosya (binary) verir [[1]]. Bu dosyayı `./hello` komutuyla çalıştırdığımızda çıktıyı alırız.

---

### 1.1. Comments (Yorum Satırları)
Kod yazarken sadece makinenin değil, kodu okuyacak diğer insanların (ve gelecekteki senin) da ne yaptığını anlaması gerekir. Rust'ta temel olarak iki tür yorum satırı bulunur [[6]].

#### A. Regular Comments (Normal Yorum Satırları)
Bunlar derleyici tarafından tamamen görmezden gelinir, koda etki etmezler.
* **Tek Satırlık:** `//` ile başlar, satır sonuna kadar devam eder.
* **Blok Yorumlar:** `/*` ile başlar ve `*/` ile biter. Birden fazla satırı yorum içine almak için kullanışlıdır.

```rust
// Bu tek satırlık bir yorumdur.
/* Bu da 
   birden fazla satıra yayılan
   bir blok yorumdur. */
```

#### B. Documentation Comments (Dokümantasyon Yorumları - Doc Comments)
İşte Rust'ın harika özelliklerinden biri! Bu yorumlar sadece kodu okuyanlara değil, aynı zamanda `rustdoc` aracına hitap eder. `rustdoc`, bu yorumları tarayarak projeniz için otomatik ve şık **HTML dokümantasyonları** üretir [[6]].
* **`///`**: Kendisinden hemen sonra gelen öğeyi (fonksiyon, struct, enum) açıklamak için kullanılır. Genellikle Markdown formatını destekler.
* **`//!`**: İçinde bulunulan modülü veya sandığı (crate) açıklamak için kullanılır.

---

### 1.2. Formatted Print (Biçimlendirilmiş Çıktı)
Ekrana sadece düz metin yazdırmak bir yere kadar yeterlidir. Değişkenlerin değerlerini metinlerin içine gömmek isteriz. Rust'ta bunu C dilindeki `printf` mantığına benzer ama çok daha güvenli ve esnek bir yolla, **format string** (biçimlendirme dizgisi) kullanarak yaparız [[10]].

Temel kuralımız şudur: Süslü parantezler `{}` birer "yer tutucudur" (placeholder).

```rust
let name = "Ali";
let age = 25;
println!("Benim adım {}, yaşım {}.", name, age);
```

Ancak Rust'ın çıktı sistemi iki farklı "Trait" (Özellik/Arayüz) üzerine kuruludur: **Debug** ve **Display**.

#### 1.2.1. Debug (Hata Ayıklama Çıktısı)
Her veri türü doğrudan `{}` ile ekrana basılamaz. Özellikle kendi oluşturduğun yapıları (struct) ekrana yazdırmaya çalıştığında Rust derleyicisi hata verir. Çünkü Rust, veriyi kullanıcıya nasıl sunacağını bilemez. İşte bu noktada **Debug** trait'i devreye girer.

* **Kullanımı:** `{:?}` (veya satırlı ve okunaklı basım için `{:#?}`) kullanılır.
* **Önemi:** Hata ayıklarken (debugging) verinin içeriğini hızlıca görmek için mükemmeldir.
* **Türetme (Deriving):** Kendi struct'ına bu yeteneği kazandırmak için başına sadece `#[derive(Debug)]` eklemen yeterlidir.

```rust
#[derive(Debug)]
struct Point { x: i32, y: i32 }

let p = Point { x: 10, y: 20 };
println!("Normal basım hata verir, Debug basım: {:?}", p); // Çıktı: Point { x: 10, y: 20 }
```

#### 1.2.2. Display (Kullanıcıya Gösterim)
Eğer amacın son kullanıcıya güzel bir çıktı vermekse (Örn: `(10, 20)` yerine `X: 10, Y: 20`), `{}` kullanman gerekir.
* **Standart Türler:** Sayılar (i32, f64), metinler (String, &str) gibi temel türler `Display` trait'ini otomatik olarak barındırır.
* **Özel Türler:** Kendi struct'ların için `std::fmt::Display` trait'ini manuel olarak implemente etmen (uyarlaman) gerekir. Bu, `fmt` fonksiyonunu yazarak verinin nasıl görüneceği üzerinde mutlak hakimiyet kurmanı sağlar.

##### 1.2.2.1. Testcase: List (Örnek Olay: Bir Listenin Biçimlendirilmesi)
Bu bölümdeki klasik bir alıştırmada, içinde sayılar barındıran özel bir `List` (Vektör) yapısının `Display` trait'i nasıl uyarlanır ona bakılır. Amacımız `[1, 2, 3]` şeklindeki vektörü alıp, `[0: 1, 1: 2, 2: 3]` gibi özel bir formatta ekrana yazdırmaktır.

```rust
use std::fmt; // Biçimlendirme kütüphanesini çağırıyoruz.

struct List(Vec<i32>);

// Display trait'ini List struct'ı için uyguluyoruz.
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?; // write! makrosu ile 'f' (formatter) içine yazıyoruz.
        
        // Vektörü iterate ederek (gezerek) özel format uyguluyoruz.
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", count, v)?;
        }
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v); // Çıktı: [0: 1, 1: 2, 2: 3]
}
```
*Öğretmenin Notu:* Buradaki `?` işareti Rust'ın meşhur hata yönetimi (error propagation) mekanizmasıdır. Yazma işlemi başarısız olursa hatayı yukarı fırlatır.

#### 1.2.3. Formatting (Gelişmiş Biçimlendirme Seçenekleri)
`println!` makrosunun içindeki `{}` parantezlerinin arasına çeşitli komutlar yazarak çıktıyı tam istediğin gibi şekillendirebilirsin [[14]].

* **İsimlendirilmiş ve Pozisyonel Argümanlar:**
  ```rust
  println!("{0}, bu {1}. {1} bu {0}!", "Ali", "Veli");
  println!("{isim} {meslek}dir.", isim="Ayşe", meslek="Mühendis");
  ```
* **Sayı Tabanları:**
  * `{:b}` -> İkili (Binary) taban
  * `{:o}` -> Sekizli (Octal) taban
  * `{:x}` veya `{:X}` -> Onaltılı (Hexadecimal) taban
* **Hizalama ve Dolgu (Padding):**
  * `{:>5}` -> 5 karakterlik alan ayır, sağa yasla.
  * `{:<5}` -> 5 karakterlik alan ayır, sola yasla.
  * `{:^5}` -> 5 karakterlik alan ayır, ortala.
  * `{:0>5}` -> 5 karakterlik alan ayır, sağa yasla ve boşlukları '0' ile doldur.
* **Hassasiyet (Ondalık Sayılar):**
  * `{:.2}` -> Virgülden sonra sadece 2 basamak göster.

```rust
println!("{:b} - {:o} - {:x}", 10, 10, 10); // 1010 - 12 - a
println!("|{:<5}| |{:^5}| |{:>5}|", "sol", "orta", "sag");
println!("Pi sayısı yaklaşık: {:.2}", 3.14159); // Pi sayısı yaklaşık: 3.14
```

---
### Dersin Özeti ve Tavsiyeler
Bugün Rust'ın dünyasına ilk adımımızı attık. Sadece ekrana yazı yazdırmakla kalmadık, Rust'ın arkasındaki güçlü **Trait (Özellik)** sistemine (Debug ve Display), derleyicinin (rustc) nasıl çalıştığına ve metinleri bir sanat eseri gibi nasıl biçimlendirebileceğimize dokunduk. 

**Ödev:** Yukarıdaki `Display` kodunu al, kendi struct'ını yarat ve onu ekrana tam senin istediğin formatta basmasını sağla. Kodu kırmaktan korkma, Rust derleyicisi sana en iyi öğretmen gibi nerede hata yaptığını uzun uzun anlatacaktır! 

İyi kodlamalar! 🦀

# DeepSeek Yorumu ----------------------

Rust By Example, Rust programlama dilini örneklerle öğreten resmî bir kaynaktır. `hello.html` sayfası, “Hello World” (Merhaba Dünya) başlıklı ilk bölümün giriş niteliğindeki sayfasıdır. Bu bölümün alt başlıkları olan **Hello World**, **Comments (Yorumlar)** ve **Formatted Print (Biçimlendirilmiş Çıktı)** konularını, sıfırdan başlayan bir ders anlatımıyla, tüm ayrıntılarıyla ele alacağım.

---

## 1. Hello World – İlk Rust Programı

Her programlama diline başlarken yazdığımız klasik program, ekrana “Merhaba Dünya” yazdırmaktır. Rust’ta bu işlem şu şekilde yapılır:

```rust
fn main() {
    println!("Merhaba Dünya!");
}
```

Bu kodu `main.rs` adlı bir dosyaya kaydedip terminalde `rustc main.rs` komutuyla derleyebilir, ardından `./main` (Linux/macOS) veya `main.exe` (Windows) ile çalıştırabilirsiniz.

Kodu adım adım inceleyelim:

- **`fn main()`** : Rust’ta her çalıştırılabilir programın bir `main` fonksiyonuna ihtiyacı vardır. `fn` anahtar kelimesi fonksiyon tanımlar. Bu fonksiyon programın giriş noktasıdır; program başladığında ilk olarak `main` çalışır.

- **`{ }`** : Fonksiyonun gövdesini sınırlayan süslü parantezlerdir. Tüm kod bu blok içinde yer alır.

- **`println!("Merhaba Dünya!");`** : Bu bir **makro** çağrısıdır. Makro olduğunu sondaki `!` işaretinden anlarız. `println!` makrosu, verilen metni ekrana yazdırır ve sonuna yeni bir satır karakteri (`\n`) ekler. Çift tırnak içindeki kısım bir string literal’dir. İfadenin sonundaki `;` ise bu satırın bir ifade olduğunu belirtir (Rust’ta çoğu ifade noktalı virgülle biter).

Eğer yeni satır eklenmesini istemiyorsak `print!` makrosu kullanılabilir, ancak `println!` çok daha yaygındır.

Bu noktada bilmeniz gereken önemli bir detay: Rust’ta string’ler UTF-8 kodlamalıdır, dolayısıyla Türkçe karakterler (ç, ğ, ı, ö, ş, ü) sorunsuzca kullanılabilir.

---

## 2. Yorumlar (Comments)

Yorumlar, kodun içine yazdığımız, derleyici tarafından tamamen yok sayılan açıklayıcı notlardır. Rust’ta iki temel yorum türü vardır:

### Satır Yorumları (Line Comments)

`//` ile başlayan yorumlardır. Bu işaretten sonra satırın sonuna kadar her şey yorum olarak kabul edilir.

```rust
fn main() {
    // Bu bir satır yorumudur. Derleyici burayı görmez.
    println!("Merhaba Dünya!"); // Satırın bu kısmı da yorumdur.
}
```

### Blok Yorumları (Block Comments)

`/*` ile başlar ve `*/` ile biter. Birden çok satıra yayılabilir, hatta kod satırının ortasında bile kullanılabilir.

```rust
fn main() {
    /* Bu çok satırlı bir yorum örneğidir.
       İkinci satırda da yorum devam eder. */
    println!("Merhaba Dünya!");
    
    println!("Bir artı bir eşittir: {}", /* 1 + 1 */ 2);
    // Kodun ortasında bir ifadeyi geçici olarak devre dışı bırakmak için kullanılabilir.
}
```

Blok yorumları iç içe (nested) yazılamaz; yani bir `/*` … `*/` bloğunun içine bir başkasını koyamazsınız. Bu sizi büyük blokları yorumlamaktan alıkoymamalıdır; zaten Rust’ta dokümantasyon için ayrı bir yorum türü (`///` ve `//!`) bulunur, ancak bu bölümün konusu değildir.

Yorumlar kodun okunabilirliğini artırmak, karmaşık mantığı açıklamak veya belirli bir bölümü geçici olarak devre dışı bırakmak için kullanılır.

---

## 3. Biçimlendirilmiş Çıktı (Formatted Print)

Artık sadece sabit bir metni değil, değişkenleri, sayıları, yapıları belirli bir düzende ekrana yazdırmak isteyeceğiz. Rust bu iş için zengin bir biçimlendirme sistemi sunar. Bu bölümde kullanacağımız başlıca makrolar ve özellikler şunlardır:

### Temel Makrolar

| Makro           | Açıklaması                                                                 |
|-----------------|----------------------------------------------------------------------------|
| `print!`        | Verilen biçimlendirilmiş metni standart çıktıya yazar, satır sonu eklemez. |
| `println!`      | Aynı işlemi yapar ve çıktının sonuna yeni satır karakteri (`\n`) ekler.    |
| `eprint!`       | Biçimlendirilmiş metni standart hataya (stderr) yazar, satır sonu eklemez. |
| `eprintln!`     | Standart hataya yazar ve satır sonu ekler.                                 |
| `format!`       | Biçimlendirilmiş metni `String` olarak döndürür, ekrana yazdırmaz.         |

Hemen bir örnekle görelim:

```rust
fn main() {
    let isim = "Ahmet";
    let yas = 30;
    
    print!("Merhaba, ");
    println!("benim adım {} ve {} yaşındayım.", isim, yas);
}
```

Çıktı: `Merhaba, benim adım Ahmet ve 30 yaşındayım.` (tek satırda, çünkü `print!` satır sonu eklemedi, `println!` ise cümlenin sonunda satır başı yaptı).

### Yer Tutucular (Placeholders)

Biçimlendirilmiş çıktıda süslü parantezler `{}` birer **yer tutucu** olarak görev yapar. Makroya verdiğimiz ek argümanlar sırayla bu yer tutucuların yerine yerleştirilir. Örnek:

```rust
let x = 10;
let y = 20;
println!("x = {}, y = {}", x, y);  // x = 10, y = 20
```

Burada ilk `{}` yerine `x` (10), ikinci `{}` yerine `y` (20) gelir.

### Konumsal Argümanlar (Positional Arguments)

Bazen argümanların sırası dışında bir eşleştirme yapmak isteyebiliriz. Yer tutucu içine argümanın sıra numarası (0’dan başlar) yazılabilir:

```rust
println!("{0} ve {1} birbirini sever, ama {0} {1}'den daha yaşlı.", "Ali", "Ayşe");
// Çıktı: Ali ve Ayşe birbirini sever, ama Ali Ayşe'den daha yaşlı.
```

`{0}` ilk argümanı (`"Ali"`), `{1}` ise ikincisini (`"Ayşe"`) temsil eder. Bu yöntem aynı argümanı birden çok kez kullanmada çok işe yarar.

### İsimli Argümanlar (Named Arguments)

Kod okunabilirliğini artırmak için argümanları isimlendirebiliriz:

```rust
println!("{ad} {soyad}, {yas} yaşında.", ad="Emre", soyad="Yılmaz", yas=25);
// Emre Yılmaz, 25 yaşında.
```

Burada `{ad}` isimli parametre `"Emre"` ile eşleşir. İsimli ve konumsal argümanlar karıştırılmamalıdır.

### Biçimlendirme Belirteçleri (Format Specifiers)

`{}` içine ek işaretler koyarak çıktının nasıl görüneceğini özelleştirebiliriz. En sık kullanılanlar:

- **`{:?}`** : Debug (hata ayıklama) biçimini kullanır. Bir tipin `Debug` trait’ini implemente etmesi gerekir.
- **`{:#?}`** : Pretty-print (güzel biçimlendirilmiş) Debug çıktısı. İç içe yapıları girintili gösterir.
- **`{}`** : Display biçimi. Tipin `Display` trait’ini implemente etmesi gerekir. Daha çok son kullanıcıya gösterilecek çıktılar içindir.
- **Sayı tabanları** : `{:b}` ikili (binary), `{:o}` sekizli, `{:x}` onaltılı (küçük harf), `{:X}` onaltılı (büyük harf), `{:e}` bilimsel gösterim, `{:p}` pointer adresi gibi.
- **Hizalama ve genişlik** : `{:5}` sağa hizalı 5 karakterlik alan, `{:<5}` sola hizalı, `{:^5}` ortalanmış, `{:0>5}` sıfır ile doldurarak 5 haneye tamamlama (ör. `00123`). `{:.*}` gibi dinamik genişlik de mümkündür.

Örnekler:

```rust
let sayı = 42;
println!("onlu: {}, onaltılı: {:x}, ikili: {:b}", sayı, sayı, sayı);
// onlu: 42, onaltılı: 2a, ikili: 101010

println!("{:>8} numara", 123);  // "     123 numara"
println!("{:0>5}", 42);        // "00042"
```

### Debug ve Display Trait’leri

Rust, kullanıcı tanımlı tiplerimizin nasıl yazdırılacağını iki farklı **trait** (özellik) ile kontrol eder:

- **`std::fmt::Debug`** : Geliştiriciye yönelik, hata ayıklama amaçlı çıktılar üretir. Elle implemente edilebileceği gibi, birçok durumda `#[derive(Debug)]` özniteliği ile otomatik türetilebilir. `{:?}` yer tutucusuyla kullanılır.

- **`std::fmt::Display`** : Son kullanıcıya gösterilecek, okunması kolay çıktılar üretir. **Otomatik türetilemez**, mutlaka bizim tarafımızdan implemente edilmelidir. `{}` yer tutucusuyla kullanılır.

Basit bir struct tanımlayıp her iki biçimi de deneyelim:

```rust
// Debug otomatik türetiliyor
#[derive(Debug)]
struct Kisi {
    ad: String,
    yas: u8,
}

fn main() {
    let k = Kisi { ad: String::from("Zeynep"), yas: 28 };
    
    // Debug çıktısı
    println!("{:?}", k);   // Kisi { ad: "Zeynep", yas: 28 }
    println!("{:#?}", k);  // Pretty-print:
                          // Kisi {
                          //     ad: "Zeynep",
                          //     yas: 28,
                          // }
    
    // Display çıktısı istersek? Hata verir çünkü Display implemente edilmedi.
    // println!("{}", k); // derleme hatası
}
```

Eğer `Display` trait’ini kendimiz yazarsak:

```rust
use std::fmt;

struct Kisi {
    ad: String,
    yas: u8,
}

impl fmt::Display for Kisi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({} yaşında)", self.ad, self.yas)
    }
}

fn main() {
    let k = Kisi { ad: String::from("Zeynep"), yas: 28 };
    println!("{}", k);   // Zeynep (28 yaşında)
}
```

Artık `{}` ile anlamlı bir çıktı alabiliriz. `write!` makrosu, `format!`’a benzer ancak bir formatter’a yazma işlemi yapar.

### format! ile String Oluşturma

Ekrana yazdırmak yerine biçimlendirilmiş bir `String` oluşturmak için `format!` makrosu kullanılır. Kullanımı `println!` ile aynıdır:

```rust
let ad = "Ali";
let yas = 22;
let metin = format!("Ben {} ve {} yaşındayım.", ad, yas);
assert_eq!(metin, "Ben Ali ve 22 yaşındayım.");
```

Bu `String` daha sonra dosyaya yazılabilir, ağa gönderilebilir veya bir değişkende saklanabilir.

### Hata Çıktıları (eprint! / eprintln!)

Normal program çıktısıyla hata mesajlarını ayırmak için standart hata akışı (stderr) kullanılır. Rust’ta bu iş için `eprint!` ve `eprintln!` makroları vardır. Kullanımları `print!` ve `println!` ile tamamen aynıdır, yalnızca çıktıyı stderr’a gönderir:

```rust
eprintln!("Bir hata oluştu: dosya bulunamadı.");
```

Terminalde normal çıktı ve hata çıktısı farklı yönlendirilebildiği için bu önemlidir.

### Biçimlendirme Sistemiyle İlgili Önemli Notlar

- Yer tutucu `{}` kullanıldığında, argümanın tipinin `Display` trait’ini implemente etmesi zorunludur. Temel tipler (i32, f64, bool, &str, String vb.) `Display`’i hazır olarak sunar.
- `{:?}` için `Debug` trait’i gerekir. Temel tipler ve standart kütüphanedeki çoğu tip `Debug` implementasyonuna sahiptir.
- Yer tutucu içinde özel karakterleri yazdırmak için çift süslü parantez kullanılır: `{{` → `{`, `}}` → `}`. Örneğin: `println!("Bu bir süslü parantez {{ ve }}");` çıktısı: `Bu bir süslü parantez { ve }`.
- Tüm bu makrolar derleme zamanında kontrol edilir; yanlış sayıda argüman veya uyumsuz tip kullanımı derleyici hatasına yol açar. Bu, çalışma zamanı sürprizlerini engeller.

---

## Özet ve İpuçları

- Rust’ta giriş noktası `fn main()` fonksiyonudur.
- Ekrana çıktı için `println!`, `print!`; hataya çıktı için `eprintln!`, `eprint!`; string oluşturmak için `format!` kullanılır.
- Yorumlar `//` (satır) ve `/* */` (blok) ile yazılır.
- Biçimlendirilmiş çıktıda `{}` yer tutucuları kullanılır, konum ve isim bazlı eşleştirme yapılabilir.
- Kullanıcı tipleri için `Debug` genellikle `#[derive(Debug)]` ile otomatik türetilir, `Display` ise elle yazılır.
- Biçimlendirme belirteçleri ile sayı tabanları, hizalama, genişlik gibi detaylar kontrol edilir.

Buraya kadar anlattıklarım, Rust By Example’ın “Hello World” bölümünün kapsamlı bir ders halidir. Bir sonraki bölüm olan “Primitives” (İlkel Tipler) ile Rust’ın temel veri yapılarına geçiş yapabilirsiniz. Şimdilik öğrendiklerinizle bile terminalde okunaklı, düzenli çıktılar üreten programlar yazmaya başlayabilirsiniz. Denemekten çekinmeyin; rustc ile hemen derleyip çalıştırabilirsiniz.