# 🦀   Ders Notları: Bölüm 5 -  'ta Structs (Yapılar) — Kapsamlı Ders Anlatımı

Merhaba! Bu derste  'ın en temel ve en güçlü veri yapılandırma mekanizması olan **Structs (Yapılar)** konusunu baştan sona, örneklerle ve detaylı açıklamalarla inceleyeceğiz.   kitabının 5. bölümünü temel alarak ilerleyeceğiz.

---

## 📌 Bölüm 1: Struct Nedir? Neden İhtiyacımız Var?

### 1.1 Struct Kavramı

**Struct (yapı)**, birden fazla ilgili değeri bir araya getirip onlara anlamlı bir isim vermenizi sağlayan **özel bir veri tipidir**. Eğer nesne yönelimli (OOP) bir dilden geliyorsanız, struct'ları bir nesnenin **veri alanları (data attributes)** gibi düşünebilirsiniz.

### 1.2 Tuple'dan Struct'a Geçiş — Neden Struct?

Diyelim ki bir dikdörtgenin alanını hesaplayan bir program yazıyoruz. İlk yaklaşımımız ayrı değişkenler kullanmak olsun:

``` 
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("Dikdörtgenin alanı: {} piksel kare.", area(width1, height1));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

Bu çalışır ama bir sorun var: `area` fonksiyonunun imzasına baktığınızda, bu iki parametrenin **birbirleriyle ilişkili olduğunu** anlamak mümkün değil. Belki bunlar iki farklı dikdörtgenin genişlik ve yükseklikleri?

#### Tuple ile Gruplama

``` 
fn main() {
    let rect1 = (30, 50);

    println!("Dikdörtgenin alanı: {} piksel kare.", area(rect1));
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

Tuple ile gruplama yaptık ama şimdi de **elemanların ne anlama geldiği kayboldu**. `dimensions.0` genişlik mi, yükseklik mi? Kodu okuyan biri bunu bilemez.

#### Struct ile Anlamlı Gruplama

İşte tam bu noktada struct devreye girer:

``` 
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Dikdörtgenin alanı: {} piksel kare.", area(&rect1));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

Artık:
- ✅ `width` ve `height` birbiriyle ilişkili ve aynı yapıda
- ✅ Alan adı açık ve anlaşılır
- ✅ Kod okunabilirliği çok yüksek

---

## 📌 Bölüm 2: Struct Tanımlama ve Kullanma

### 2.1 Struct Tanımlama Sözdizimi

``` 
struct KullaniciAdi {
    alan1: Tip1,
    alan2: Tip2,
    alan3: Tip3,
}
```

Örnek: Bir kullanıcı hesabını temsil eden bir struct:

``` 
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {}
```

**Önemli Not:** Struct ismi, içindeki verilerin ne anlama geldiğini yansıtmalıdır.

### 2.2 Struct Örneği (Instance) Oluşturma

Bir struct tanımladıktan sonra, onu kullanmak için **örnek (instance)** oluştururuz:

``` 
fn main() {
    let user1 = User {
        active: true,
        username: String::from("ahmet_yilmaz"),
        email: String::from("ahmet@example.com"),
        sign_in_count: 1,
    };
}
```

**Dikkat:** Alanları tanımlandıkları sırada vermek zorunda değilsiniz. Sıralama tamamen serbest.

### 2.3 Alanlara Erişim — Dot Notation

Bir struct örneğinin belirli bir alanına erişmek için **nokta notasyonu** kullanırız:

``` 
fn main() {
    let user1 = User {
        active: true,
        username: String::from("ahmet_yilmaz"),
        email: String::from("ahmet@example.com"),
        sign_in_count: 1,
    };

    println!("Kullanıcı e-postası: {}", user1.email);
}
```

### 2.4 Değer Değiştirme — Mutable Struct

Eğer struct örneği **mutable (değiştirilebilir)** ise, alanların değerlerini değiştirebiliriz:

``` 
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("ahmet_yilmaz"),
        email: String::from("ahmet@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("yeni_email@example.com");
}
```

⚠️ **Önemli:**  'ta **tüm örnek mutable olmalıdır**. Belirli alanları mutable yapıp diğerlerini immutable bırakamazsınız.

### 2.5 Struct Döndüren Fonksiyonlar

``` 
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
```

---

## 📌 Bölüm 3: Kolaylık Sağlayan Sözdizimleri

### 3.1 Field Init Shorthand (Alan Başlatma Kısayolu)

Yukarıdaki örnekte `username: username` ve `email: email` tekrarları can sıkıcı. Eğer **parametre adı ile alan adı aynıysa**, kısayol kullanabiliriz:

``` 
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,  // username: username yerine
        email,     // email: email yerine
        sign_in_count: 1,
    }
}
```

Bu tamamen aynı işi yapar ama daha temiz görünür.

### 3.2 Struct Update Syntax (Güncelleme Sözdizimi)

Bazen mevcut bir örnekten **bazı alanları değiştirerek** yeni bir örnek oluşturmak isteriz. Bunu geleneksel yolla yapmak çok uzun sürer:

``` 
let user2 = User {
    active: user1.active,
    username: user1.username,
    email: String::from("yeni@example.com"),  // sadece bu değişecek
    sign_in_count: user1.sign_in_count,
};
```

**Struct Update Syntax** ile bunu çok daha kısa yazabiliriz:

``` 
let user2 = User {
    email: String::from("yeni@example.com"),
    ..user1  // geri kalan tüm alanlar user1'den alınsın
};
```

`..user1` ifadesi, açıkça belirtilmeyen alanların `user1`'den alınacağını söyler.

⚠️ **Dikkat — Move Semantiği:**
Update syntax `=` gibi çalışır ve veriyi **taşıyabilir (move)**. Eğer `username` gibi `String` tipindeki alanlar `user2`'ye taşınırsa, `user1` artık kullanılamaz. Ama `active` ve `sign_in_count` gibi `Copy` trait'ine sahip tipler taşınmaz, kopyalanır.

### 3.3 Tuple Struct (Demet Yapıları)

Bazen alanlara isim vermek yerine sadece **tip bilgisi** yeterli olabilir. Bu durumda tuple struct kullanırız:

``` 
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

Her ikisi de üç `i32` içerse bile, `Color` ve `Point` **farklı tiplerdir**. Bir fonksiyon `Color` bekliyorsa `Point` veremezsiniz.

**Erişim:** İndeks ile yapılır:
``` 
let r = black.0;  // 0
```

**Destructuring (Yapı Çözme):**
``` 
let Point(x, y, z) = origin;
```

### 3.4 Unit-Like Struct (Birim Yapılar)

Hiç alanı olmayan struct'lar da tanımlayabilirsiniz. Bunlar `()` (unit tip) gibi davranır:

``` 
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

Bu tip yapılar genellikle **trait implementasyonu** gerektiğinde ama saklanacak veri olmadığında kullanılır (Bölüm 10'da trait'leri göreceğiz).

---

## 📌 Bölüm 4: Struct ve Ownership (Sahiplik)

### 4.1 Owned Tipler vs Referanslar

Struct tanımlarken, verinin **sahipliğini (ownership)** struct'ın almasını istersiniz:

``` 
struct User {
    active: bool,
    username: String,   // ✅ Owned tip
    email: String,       // ✅ Owned tip
    sign_in_count: u64,
}
```

Neden `&str` yerine `String` kullandık? Çünkü struct'ın tüm verilerinin sahipliğini almasını ve struct geçerli olduğu sürece verilerin de geçerli olmasını istiyoruz.

### 4.2 Struct'larda Referans Kullanımı — Lifetimes

Eğer bir struct'ta başka bir yere ait veriye **referans** saklamak isterseniz, **lifetime (ömür)** belirtmeniz gerekir:

``` 
// ❌ Bu çalışmaz!
struct User {
    username: &str,  // HATA: missing lifetime specifier
    email: &str,
}
```

Hata mesajı:
```
error[E0106]: missing lifetime specifier
```

Doğru kullanım (Bölüm 10'da detaylı anlatılacak):
``` 
struct User<'a> {
    username: &'a str,
    email: &'a str,
}
```

Şimdilik struct'larda referans yerine **owned tipler** (`String`, `Vec<T>` vb.) kullanmak en güvenli yoldur.

---

## 📌 Bölüm 5: Struct'ları Yazdırma — Debug Trait

### 5.1 println! ile Yazdırma Sorunu

``` 
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1: {}", rect1);  // ❌ HATA!
}
```

Hata:
```
error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
```

 , struct'ların nasıl yazdırılacağını **bilemez**. Kaç alan var? Virgül kullanılsın mı? Süslü parantezler olsun mu? Bu belirsizlikleri   tahmin etmez.

### 5.2 Debug Trait ile Çözüm

İki yol var:

**1. `{:?}` format belirteci ile Debug yazdırma:**
``` 
println!("rect1: {:?}", rect1);  // Hala hata verir
```

**2. `#[derive(Debug)]` attribute eklemek:**
``` 
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1: {:?}", rect1);
    // Çıktı: rect1: Rectangle { width: 30, height: 50 }
}
```

**Daha okunabilir çıktı için `{:#?}` kullanın:**
``` 
println!("rect1: {:#?}", rect1);
// Çıktı:
// rect1: Rectangle {
//     width: 30,
//     height: 50,
// }
```

### 5.3 dbg! Makrosu

`dbg!` makrosu, debug bilgilerini **stderr**'e yazdırır ve dosya/satır numarasını da gösterir:

``` 
fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),  // İfadenin değerini yazdırır ve döndürür
        height: 50,
    };

    dbg!(&rect1);  // Struct'ı yazdırır
}
```

Çıktı:
```
[src/main.rs:10:16] 30 * scale = 60
[src/main.rs:14:5] &rect1 = Rectangle {
    width: 60,
    height: 50,
}
```

⚠️ `dbg!` sahipliği alır, bu yüzden referans (`&`) kullanmak gerekebilir.

---

## 📌 Bölüm 6: Method Syntax (Metod Sözdizimi)

### 6.1 Metod vs Fonksiyon

**Metodlar**, fonksiyonlara benzer ama bir struct (veya enum, trait object) **bağlamında** tanımlanırlar. İlk parametreleri her zaman **`self`**'tir ve struct örneğini temsil eder.

### 6.2 Metod Tanımlama — impl Bloğu

``` 
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("Alan: {} piksel kare", rect1.area());
}
```

**Açıklama:**
- `impl Rectangle` bloğu, `Rectangle` tipiyle ilişkili tüm fonksiyonları içerir
- `&self` aslında `self: &Self` kısaltmasıdır
- `Self`, `impl` bloğunun ait olduğu tipi temsil eder (burada `Rectangle`)

### 6.3 self Parametresinin Çeşitleri

| Parametre | Anlamı | Ne Zaman Kullanılır? |
|-----------|--------|----------------------|
| `&self` | Immutable borrow (değişmez ödünç alma) | Sadece okuma yapılacaksa |
| `&mut self` | Mutable borrow (değişebilir ödünç alma) | Veri değiştirilecekse |
| `self` | Ownership alma (sahiplik) | Nadir; genellikle dönüşüm yapıp orijinali kullanılmaz hale getirecekse |

Örnek — Mutable metod:
``` 
impl Rectangle {
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
}
```

### 6.4 Otomatik Referans ve Dereferans

 'ta `->` operatörü yoktur. Bunun yerine **otomatik referans/dereferans** vardır:

``` 
let p1 = Point { x: 0.0, y: 0.0 };
let p2 = Point { x: 5.0, y: 6.5 };

p1.distance(&p2);       // ✅ Temiz görünüm
(&p1).distance(&p2);    // ✅ Aynı şey, ama gereksiz
```

 , metodun imzasına bakarak otomatik olarak `&`, `&mut` veya `*` ekler. Bu, ownership'i ergonomik hale getirir.

### 6.5 Birden Fazla Parametre Alan Metodlar

``` 
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("rect1, rect2'yi tutabilir mi? {}", rect1.can_hold(&rect2));  // true
    println!("rect1, rect3'ü tutabilir mi? {}", rect1.can_hold(&rect3));  // false
}
```

### 6.6 Alan ve Metod Aynı İsimde Olabilir

``` 
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    
    if rect1.width() {  // Parantez var → metod çağrısı
        println!("Dikdörtgenin sıfırdan büyük genişliği var: {}", rect1.width);  // Parantez yok → alan erişimi
    }
}
```

**Getter (Erişimci) Metotlar:** Bazı diller otomatik getter oluşturur ama   bunu yapmaz. Manuel yazmanız gerekir. Bu, alanı **private** tutup metodu **public** yaparak kontrollü erişim sağlar (Bölüm 7'de göreceğiz).

---

## 📌 Bölüm 7: Associated Functions (İlişkili Fonksiyonlar)

### 7.1 self Olmayan Fonksiyonlar

`impl` bloğunda tanımlanan ama `self` parametresi olmayan fonksiyonlara **associated function** denir. Bunlar struct örneğine ihtiyaç duymaz.

En yaygın kullanım: **Constructor (yapıcı) fonksiyonlar**

``` 
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```

**Çağırma:** `::` syntax ile struct ismi üzerinden:
``` 
let sq = Rectangle::square(3);
```

`Self`, `impl` bloğundaki tipin (`Rectangle`) kısaltmasıdır.

### 7.2 String::from — Bildiğimiz Bir Örnek

Daha önce kullandığımız `String::from` da bir associated function'dır:
``` 
let s = String::from("merhaba");
```

### 7.3 Birden Fazla impl Bloğu

Bir struct için birden fazla `impl` bloğu tanımlayabilirsiniz:

``` 
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

Bu tamamen geçerlidir. Genellikle **farklı trait'ler** implemente ederken birden fazla `impl` bloğu kullanılır (Bölüm 10).

---

## 📌 Bölüm 8: Pratik Örnek — Gömülü Sistem İçin Struct

Kullanıcının step motor projesine uygun bir örnek:

``` 
#[derive(Debug)]
struct StepMotor {
    motor_id: u8,
    steps_per_revolution: u32,
    current_position: i32,
    is_enabled: bool,
}

impl StepMotor {
    // Constructor (associated function)
    fn new(motor_id: u8, steps_per_revolution: u32) -> Self {
        Self {
            motor_id,
            steps_per_revolution,
            current_position: 0,
            is_enabled: false,
        }
    }

    // Metod — motoru etkinleştir
    fn enable(&mut self) {
        self.is_enabled = true;
        println!("Motor {} etkinleştirildi", self.motor_id);
    }

    // Metod — belirli adım kadar hareket ettir
    fn move_steps(&mut self, steps: i32) {
        if !self.is_enabled {
            println!("Motor {} etkin değil!", self.motor_id);
            return;
        }
        self.current_position += steps;
        println!("Motor {} {} adım hareket etti. Yeni konum: {}", 
                 self.motor_id, steps, self.current_position);
    }

    // Metod — mevcut konumu döndür
    fn get_position(&self) -> i32 {
        self.current_position
    }

    // Metod — sıfıra döndür
    fn home(&mut self) {
        self.current_position = 0;
        println!("Motor {} sıfır konumuna döndü", self.motor_id);
    }
}

fn main() {
    // Struct örneği oluştur
    let mut motor = StepMotor::new(1, 200);
    
    println!("Motor bilgisi: {:?}", motor);
    
    motor.enable();
    motor.move_steps(100);
    motor.move_steps(-50);
    
    println!("Son konum: {}", motor.get_position());
    
    motor.home();
}
```

---

## 📌 Bölüm 9: Özet ve En İyi Pratikler

### ✅ En İyi Pratikler

1. **Owned tipler tercih edin:** Struct alanlarında `String`, `Vec<T>` gibi owned tipler kullanın
2. **Anlamlı isimler verin:** Struct ismi ve alan isimleri açıklayıcı olmalı
3. **Field init shorthand kullanın:** Parametre ve alan adları aynıysa kısayoldan faydalanın
4. **Struct update syntax ile kod tekrarını azaltın:** Mevcut örnekten yeni örnek oluştururken `..` kullanın
5. **`#[derive(Debug)]` ekleyin:** Debugging için her struct'a ekleyin
6. **Metodları `impl` bloğunda toplayın:** Bir tipe ait tüm davranışlar birlikte olmalı
7. **`&self` varsayılanınız olsun:** Sadece okuma yapılacaksa immutable borrow kullanın
8. **Constructor için `new` kullanın:** Bu bir gelenektir (zorunlu değil)

### ❌ Yaygın Hatalar

1. **Lifetime belirtmeden referans kullanmak:** `&str` yerine `String` kullanın
2. **Mutable alan ayrı tutmaya çalışmak:** Tüm struct mutable olmalı
3. **Move semantiğini göz ardı etmek:** Update syntax sonrası orijinal struct kullanılamayabilir
4. **Display ile Debug karıştırmak:** `{}` Display, `{:?}` Debug içindir

---

## 📌 Bölüm 10: Sıradaki Adım

Struct'lar,  'ta **kendi tiplerinizi oluşturmanın** temel yoludur. Bir sonraki konuda **Enum**'ları inceleyeceğiz. Enum'lar da struct'lar gibi özel tiplerdir ama farklı amaçlara hizmet ederler. İkisini birlikte kullanarak  'ın güçlü **tip sisteminden** tam anlamıyla faydalanabilirsiniz.

---

## 📚 Hızlı Referans Tablosu

| Özellik | Sözdizim | Örnek |
|---------|----------|-------|
| Struct tanımlama | `struct Name { field: Type }` | `struct User { name: String }` |
| Örnek oluşturma | `Name { field: value }` | `User { name: String::from("Ali") }` |
| Alan erişimi | `instance.field` | `user.name` |
| Field init shorthand | `field` (name aynıysa) | `User { name }` |
| Update syntax | `..other` | `User { email: new, ..user1 }` |
| Tuple struct | `struct Name(Type)` | `struct Point(i32, i32)` |
| Unit struct | `struct Name;` | `struct Marker;` |
| Metod tanımlama | `impl Type { fn name(&self) }` | `impl User { fn greet(&self) }` |
| Associated function | `fn name() -> Self` | `fn new() -> Self` |
| Debug yazdırma | `{:?}` veya `{:#?}` | `println!("{:?}", user)` |

---

# 📚 Detaylı Hızlı Referans Tablosu

Aşağıdaki tablo,   struct'ları ile ilgili tüm temel kavramları, kullanım senaryolarını ve önemli detayları içerir:

| Özellik | Ne İşe Yarar? | Sözdizim | Somut Örnek | Önemli Notlar & İpuçları | Ne Zaman Kullanılır? |
|---------|---------------|----------|-------------|-------------------------|----------------------|
| **Struct Tanımlama** | İlgili verileri gruplayarak yeni bir tip oluşturur | `struct Isim { alan: Tip }` | ``` struct Ogrenci {    ad: String,    numara: u32,    ortalama: f32}``` | • Struct ismi **PascalCase** (BüyükHarfle)• Alan isimleri **snake_case** (küçük_harf)• Her alanın tipi belirtilmeli• Noktalı virgül `;` ile biter | Birden fazla ilişkili veriyi (ad, soyad, yaş vb.) tek bir mantıksal birimde toplamak istediğinizde |
| **Örnek (Instance) Oluşturma** | Tanımlanan struct'tan somut bir değer yaratır | `Isim { alan: değer }` | ``` let ogrenci1 = Ogrenci {    ad: String::from("Ali"),    numara: 12345,    ortalama: 3.75};``` | • Tüm alanlar belirtilmeli (eksik olursa hata)• Sıralama önemli değil• `String` için `String::from()` veya `.to_string()` kullanın• Değişmez (`let`) veya değişken (`mut`) olabilir | Struct tipinden gerçek bir veri nesnesi oluşturmak istediğinizde |
| **Alan Erişimi (Field Access)** | Struct içindeki belirli bir değeri okur veya değiştirir | `ornek.alan_adi` | ``` println!("{}", ogrenci1.ad);ogrenci1.ortalama = 4.0;``` | • Dot notation (nokta gösterimi)• Mutable struct'ta değer değiştirilebilir• Immutable struct'ta sadece okunabilir• Tüm struct mutable olmalı, tek alan değil | Struct'ın içindeki belirli bir bilgiye ihtiyacınız olduğunda veya güncellemek istediğinizde |
| **Field Init Shorthand** | Parametre adı ile alan adı aynıysa tekrarı önler | `alan_adi` (sadece isim) | ``` fn yeni_ogrenci(ad: String, numara: u32) -> Ogrenci {    Ogrenci {        ad,  // ad: ad yerine        numara,  // numara: numara yerine        ortalama: 0.0    }}``` | • Sadece **parametre adı == alan adı** olduğunda çalışır• Kodu temizleştirir• Okunabilirliği artırır• Tüm alanlar için kullanılmak zorunda değil | Fonksiyon parametrelerini doğrudan struct alanlarına aktarırken |
| **Struct Update Syntax** | Mevcut struct'tan yeni struct oluşturur, bazı alanları değiştirir | `..eski_ornek` | ``` let ogrenci2 = Ogrenci {    ad: String::from("Ayşe"),    ..ogrenci1};// ogrenci2: ad="Ayşe", numara ve ortalama ogrenci1'den``` | • `..` operatörü **kalan tüm alanları** kopyalar/taşır• Belirtilen alanlar önce yazılmalı• **Move semantiği**: `String` gibi tipler taşınır, `Copy` tipler kopyalanır• Taşınan struct artık kullanılamaz | Bir struct'ın çoğu alanı aynı ama birkaçı farklı olan yeni bir örnek oluştururken (örn: klonlama + değişiklik) |
| **Tuple Struct** | Alanlara isim vermeden, sadece tip ile struct oluşturur | `struct Isim(Tip1, Tip2);` | ``` struct Renk(i32, i32, i32);struct Koordinat(f32, f32);let siyah = Renk(0, 0, 0);let nokta = Koordinat(10.5, 20.3);let kirmizi = siyah.0;  // 0let x = nokta.0;  // 10.5``` | • Alanlara **indeks ile** erişilir (`.0`, `.1`)• Farklı tuple struct'lar **farklı tiplerdir**• `Renk(0,0,0)` ≠ `Koordinat(0,0)`• Destructuring: `let Renk(r, g, b) = siyah;` | Alan isimleri önemli değilse ama **tip güvenliği** istiyorsanız (örn: renk vs koordinat karışmasın) |
| **Unit-Like Struct** | Hiç alanı olmayan, marker/etiket olarak kullanılan struct | `struct Isim;` | ``` struct Isaretleyici;fn main() {    let marker = Isaretleyici;}``` | • Hiç veri saklamaz• Genellikle **trait implementasyonu** için• Type state pattern'de kullanılır• Boyutu 0 byte'tır | Veri saklamaya ihtiyacınız yoksa ama bir tip oluşturmanız gerekiyorsa (örn: trait implementasyonu, tip seviyesi programlama) |
| **Method Tanımlama** | Struct'a ait davranışları (fonksiyonları) tanımlar | `impl Isim { fn metod(&self) { } }` | ``` impl Ogrenci {    fn not_harfi(&self) -> String {        match self.ortalama {            3.5..=4.0 => "AA".to_string(),            3.0..=3.4 => "BA".to_string(),            _ => "Düşük".to_string()        }    }}``` | • `impl` bloğu içinde tanımlanır• İlk parametre **`self`** olmalı• `&self` = immutable borrow• `&mut self` = mutable borrow• `self` = ownership alma (nadir) | Struct verisi üzerinde işlem yapan, hesaplayan veya veriyi değiştiren fonksiyonlar yazarken |
| **`&self` Parametresi** | Metoda struct'ın immutable referansını verir | `fn metod(&self)` | ``` impl Ogrenci {    fn bilgileri_goster(&self) {        println!("{} - {}", self.ad, self.numara);        // self.ortalama = 4.0; ❌ HATA!    }}``` | • Struct'ı **değiştirmez**• Sadece okuma yapar• En yaygın kullanım• Otomatik referans/dereferans vardır• `self.bilgileri_goster()` şeklinde çağrılır | Metot struct verisini değiştirmeyecekse (getter, hesaplayıcı, görüntüleyici) |
| **`&mut self` Parametresi** | Metoda struct'ın mutable referansını verir | `fn metod(&mut self)` | ``` impl Ogrenci {    fn ortalama_guncelle(&mut self, yeni_ortalama: f32) {        self.ortalama = yeni_ortalama;    }}``` | • Struct'ı **değiştirebilir**• Mutable struct örneği gerekir• `let mut ogrenci = ...`• Veri güncelleme, ekleme, silme işlemleri için | Metot struct'ın içindeki veriyi değiştirecekse (setter, güncelleme, sıfırlama) |
| **Associated Function** | Struct'a ait ama `self` almayan fonksiyon (static benzeri) | `fn fonksiyon() -> Self` | ``` impl Ogrenci {    fn yeni(ad: String, numara: u32) -> Self {        Self {            ad,            numara,            ortalama: 0.0        }    }}``` | • `self` parametresi **yok**• `::` ile çağrılır: `Ogrenci::yeni(...)`• Genellikle **constructor** olarak kullanılır• `Self` = struct isminin kısaltması• Birden fazla constructor olabilir | Struct örneği oluştururken (factory pattern, constructor, utility fonksiyonları) |
| **Debug Trait** | Struct'ı debug formatında yazdırır | `#[derive(Debug)]` + `{:?}` | ``` #[derive(Debug)]struct Ogrenci {    ad: String,    numara: u32}let ogrenci = Ogrenci { ... };println!("{:?}", ogrenci);// Ogrenci { ad: "Ali", numara: 12345 }println!("{:#?}", ogrenci);// Pretty print (çok satırlı)``` | • `#[derive(Debug)]` eklenmeli• `{:?}` = kompakt• `{:#?}` = güzel formatlı• `dbg!()` makrosu da kullanılabilir• Production'da kullanmayın | Debugging yaparken, struct içeriğini hızlıca görmek istediğinizde |
| **Multiple `impl` Blokları** | Bir struct için birden fazla impl bloğu tanımlar | `impl Isim { }` (birden fazla) | ``` impl Ogrenci {    fn yeni(...) -> Self { ... }}impl Ogrenci {    fn ortalama_hesapla(&self) -> f32 { ... }}impl Ogrenci {    fn mezun_olabilir_mi(&self) -> bool { ... }}``` | • Tamamen geçerli• Genellikle **farklı trait'ler** için kullanılır• Kod organizasyonu için faydalı• Her blokta farklı metod grupları olabilir | Büyük struct'larda kodu mantıksal gruplara ayırmak veya farklı trait'ler implemente etmek için |

---

## 🎯 Ek Bilgiler

### Otomatik Türetilen Trait'ler (Derive Macros)

``` 
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}
```

| Trait | Ne Sağlar? | Ne Zaman Kullanılır? |
|-------|------------|----------------------|
| `Debug` | `{:?}` ile yazdırma | Debugging için (neredeyse her zaman) |
| `Clone` | `.clone()` ile derin kopya | Struct'ı kopyalamanız gerektiğinde |
| `Copy` | Otomatik kopya (stack tipler) | Sadece tüm alanlar `Copy` ise |
| `PartialEq` | `==` ve `!=` operatörleri | İki struct'ı karşılaştırmak için |
| `Eq` | Tam eşitlik (reflexive, symmetric, transitive) | `PartialEq` + ek garantiler |
| `Hash` | Hash fonksiyonu | `HashMap` key'i olarak kullanacaksanız |
| `Default` | `.default()` ile varsayılan değer | Varsayılan değerler mantıklıysa |

### Ownership ve Move Semantiği - Özet

``` 
struct Veri {
    s: String,      // Heap'te, owned
    n: i32,         // Stack'te, Copy
}

fn main() {
    let v1 = Veri { s: String::from("merhaba"), n: 42 };
    
    let v2 = v1;  // MOVE! v1 artık kullanılamaz
    
    // println!("{}", v1.s);  // ❌ HATA: value borrowed here after move
    
    let v3 = Veri { n: 100, ..v2 };  // v2.s taşınır, v2.n kopyalanır
    // v2 artık kullanılamaz (s taşındı)
    
    println!("{}", v3.s);  // ✅ Çalışır
}
```

### Metod Çağırma - Otomatik Referans/Dereferans

``` 
struct Dikdortgen { alan: u32 }

impl Dikdortgen {
    fn buyut(&mut self, faktor: u32) {
        self.alan *= faktor;
    }
    
    fn alan(&self) -> u32 {
        self.alan
    }
}

fn main() {
    let mut d = Dikdortgen { alan: 10 };
    
    d.buyut(2);        //   otomatik &mut alır
    (&mut d).buyut(2); // Aynı şey, açık yazım
    
    let a = d.alan();  //   otomatik & alır
    let a = (&d).alan(); // Aynı şey
}
```

---

Bu tablo,   struct'ları ile çalışırken ihtiyaç duyacağınız tüm temel bilgileri pratik örneklerle birlikte sunar. Her bir özelliği ne zaman ve neden kullanmanız gerektiğini artık daha iyi anlıyorsunuz! 🚀