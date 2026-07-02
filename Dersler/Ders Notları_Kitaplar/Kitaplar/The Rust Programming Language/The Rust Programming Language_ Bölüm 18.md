# 🦀 Ders Notları: Bölüm 18 - Rust'ta Nesne Yönelimli Programlama (OOP) — Kapsamlı Ders

Hoş geldiniz! Bu derste, Rust programlama dilinin nesne yönelimli programlama (OOP) yaklaşımını, Rust kitabının 18. bölümünü temel alarak adım adım, ders anlatır gibi işleyeceğiz. Rust'ın klasik OOP dillerinden (Java, C++, C#) nasıl farklılaştığını, kendi güçlü özellikleriyle bu paradigmaya nasıl yaklaştığını göreceğiz.

---

## 📚 BÖLÜM 18: GİRİŞ — OOP Nedir, Rust Neden Farklı?

### 1.1 OOP'nin Tarihsel Arka Planı

Nesne yönelimli programlama kavramı ilk kez 1960'larda **Simula** dilinde ortaya çıktı. Alan Kay, nesnelerin birbirlerine mesaj geçtiği bir mimari tasarlayarak 1967'de "object-oriented programming" terimini ortaya attı.

> ⚠️ **Önemli Not:** OOP'nin tek bir doğru tanimi yoktur. Bazı tanımlara göre Rust nesne yönelimlidir, bazılarına göre değildir. Bu bölümde, OOP ile ilişkilendirilen özelliklerin Rust'ta nasıl karşılandığını göreceğiz.

---

## 📚 BÖLÜM 18.1: NESNE YÖNELİMLİ PROGRAMLAMA NEDİR?

OOP topluluğunda, bir dilin "nesne yönelimli" sayılması için hangi özelliklere sahip olması gerektiği konusunda bir fikir birliği yoktur. Ancak genel kabul gören **üç temel özellik** vardır:

1. **Nesneler (Objects)** — Veri + Davranış birleşimi
2. **Kapsülleme (Encapsulation)** — İç detayların gizlenmesi
3. **Kalıtım (Inheritance)** — Üst sınıftan özellik devralma

Hadi bunları tek tek Rust açısından inceleyelim.

---

### 🧱 1.1.1 Nesneler (Objects) — Veri ve Davranışın Birleşimi

> *"Nesne yönelimli programlar nesnelerden oluşur. Bir nesne, hem veriyi hem de bu veri üzerinde işlem yapan prosedürleri paketler. Bu prosedürlere genellikle **metot** veya **işlem** denir."*
> — Gang of Four (Dörtlü Çete) Kitabı

Rust'ta `struct` ve `enum`'lar **veri** tutar, `impl` blokları ise bu verilere **metot** sağlar. Yani Rust, Gang of Four tanımına göre nesne yönelimlidir!

```rust
struct Araba {
    marka: String,
    hiz: f64,
}

impl Araba {
    // Bu bir metottur — davranış
    fn hizlan(&mut self, miktar: f64) {
        self.hiz += miktar;
    }
    
    fn bilgi_goster(&self) {
        println!("{} - {} km/h", self.marka, self.hiz);
    }
}
```

---

### 🔒 1.1.2 Kapsülleme (Encapsulation) — İç Detayları Gizlemek

Kapsülleme, bir nesnenin iç implementasyon detaylarının dışarıdan erişime kapatılması anlamına gelir. Dış kod, nesneyle sadece **public API** üzerinden etkileşime girmelidir.

**Neden Önemli?**
- İç yapıyı istediğiniz zaman değiştirebilirsiniz (refactoring)
- Dış kodu bozmazsınız
- Veri tutarlılığını korursunuz

#### 📖 Örnek: `AveragedCollection` (Ortalama Koleksiyonu)

Bir koleksiyon düşünelim: İçinde `i32` değerleri tutuyor ve ortalamasını hesaplıyor. Ortalama, her istendiğinde yeniden hesaplanmak yerine **önbelleğe (cache)** alınsın.

```rust
pub struct AveragedCollection {
    list: Vec<i32>,      // private (pub yok!)
    average: f64,        // private (pub yok!)
}
```

🔴 **Dikkat:** Alanlar `pub` değil! Yani dışarıdan kimse doğrudan `list`'e ekleme/çıkarma yapamaz. Bu çok önemli çünkü `list` değiştiğinde `average`'ın da güncellenmesi gerekiyor.

#### Metotların Implementasyonu:

```rust
impl AveragedCollection {
    // ✅ Ekleme — hem list hem average güncellenir
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    // ✅ Çıkarma — hem list hem average güncellenir
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    // ✅ Sadece okuma — average değiştirilemez
    pub fn average(&self) -> f64 {
        self.average
    }

    // 🔒 Private metot — dışarıdan çağrılamaz
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
```

#### 🎯 Kapsüllemenin Gücü

Diyelim ki gelecekte `Vec<i32>` yerine `HashSet<i32>` kullanmak istediniz:

```rust
pub struct AveragedCollection {
    list: HashSet<i32>,   // Değişti!
    average: f64,
}
```

**Eğer alanlar private ise:** Sadece `add`, `remove`, `update_average` metotlarını güncellersiniz. Dışarıdaki kod **hiç değişmez!**

**Eğer alanlar public olsaydı:** Dışarıdaki tüm kodlar `HashSet`'in farklı API'sine göre güncellenmek zorunda kalırdı. 💥

---

### 🧬 1.1.3 Kalıtım (Inheritance) — Rust'ta Yok!

Kalıtım, bir nesnenin başka bir nesnenin tanımından veri ve davranışları **devralması** mekanizmasıdır.

> ❌ **Rust'ta kalıtım yoktur.** Bir struct'ın başka bir struct'tan alan ve metot implementasyonlarını miras almasının bir yolu yoktur (macro kullanmadan).

#### Peki Kalıtım Neden Kullanılır?

Genellikle **iki ana sebeple** kalıtıma ihtiyaç duyarız:

#### **Sebep 1: Kod Tekrarını Önlemek (Code Reuse)**

Rust'ta bunu **trait'lerin varsayılan metot implementasyonları** ile çözeriz:

```rust
pub trait Summary {
    // Varsayılan implementasyon
    fn summarize(&self) -> String {
        String::from("(Devamı okunuyor...)")
    }
}

struct Makale {
    baslik: String,
}

// Varsayılan summarize'ı kullanır — kod tekrarı yok!
impl Summary for Makale {}

struct Haber {
    baslik: String,
}

impl Summary for Haber {
    // Varsayılan implementasyonu override ederiz
    fn summarize(&self) -> String {
        format!("Son dakika: {}", self.baslik)
    }
}
```

#### **Sebep 2: Polimorfizm (Alt Tipleri Üst Tip Yerine Kullanma)**

Rust kalıtım yerine **generics + trait bounds** kullanır. Buna **bounded parametric polymorphism** denir.

```rust
// Java/C++ tarzı kalıtım YOK
// Rust tarzı trait ile polimorfizm VAR
fn yazdir(o: &impl Summary) {
    println!("{}", o.summarize());
}
```

#### 🤔 Rust Neden Kalıtımı Reddetti?

- Kalıtım genellikle **gereğinden fazla kod paylaşımına** yol açar
- Alt sınıflar, üst sınıfın **tüm** özelliklerini almak zorundadır (esneklik kaybı)
- Alt sınıfa **uygun olmayan metotlar** çağrılabilir
- Bazı diller sadece **tek kalıtıma** (single inheritance) izin verir

Rust bunun yerine **trait nesneleri** ile runtime polimorfizmi sağlar. Şimdi buna geçelim! 👇

---

## 📚 BÖLÜM 18.2: TRAIT NESNELERİ (Trait Objects) ve Polimorfizm

Bu bölüm Rust'ın en önemli OOP özelliklerinden birini içeriyor. Hazırsanız başlayalım!

### 🎯 Problem: Farklı Tipleri Bir Arada Tutma

Bir GUI (Grafik Arayüz) kütüphanesi yazdığınızı düşünün. Ekranda farklı tiplerde bileşenler olacak: `Button`, `TextField`, `Image`, `SelectBox`...

**Sorun:** `Vec` sadece **tek bir tip** tutabilir. Farklı tipleri bir arada nasıl tutacağız?

```rust
// ❌ OLMAZ — Vec tek tip tutar
let components = vec![Button::new(), TextField::new(), Image::new()];
```

### 💡 Çözüm: Trait Nesneleri (Trait Objects)

Trait nesnesi, belirli bir trait'i implement eden **herhangi bir tipe** işaret eden bir pointer'dır. Çalışma zamanında (runtime) hangi metotun çağrılacağını belirler.

#### 🔧 Sözdizimi:

```rust
Box<dyn Traitİsmi>    // Trait nesnesi
&dyn Traitİsmi         // Referans olarak trait nesnesi
```

- `Box<T>` — Heap üzerinde bir pointer
- `dyn` — "dinamik" anlamına gelir, bir trait nesnesi olduğunu belirtir

### 📖 Kapsamlı Örnek: GUI Kütüphanesi

#### Adım 1: `Draw` Trait'ini Tanımla

```rust
pub trait Draw {
    fn draw(&self);
}
```

#### Adım 2: `Screen` Struct'ını Trait Nesnesi ile Tanımla

```rust
pub struct Screen {
    // Farklı tiplerde bileşenleri tutabilir!
    pub components: Vec<Box<dyn Draw>>,
}
```

🔑 **Kritik Nokta:** `Vec<Box<dyn Draw>>`, `Draw` trait'ini implement eden **herhangi bir tip** içerebilir. Hem `Button`, hem `TextField`, hem de kullanıcının sonradan ekleyeceği `SelectBox`!

#### Adım 3: `run` Metodu

```rust
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();  // Her bileşenin kendi draw'ı çağrılır
        }
    }
}
```

### 🆚 Trait Nesnesi vs. Generics

Bu noktada aklınıza şu gelebilir: *"Generics ile de yapabilirdik!"* Haklısınız, ama farklar var:

#### Generic Yaklaşım:

```rust
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}
```

❌ **Sorun:** Bu `Screen` sadece **tek bir tip** tutabilir. Ya hep `Button`, ya hep `TextField`. İkisini bir arada tutamazsınız!

#### Trait Nesnesi Yaklaşımı:

```rust
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
```

✅ **Avantaj:** Tek bir `Screen` içinde hem `Button`, hem `TextField`, hem `Image` bir arada olabilir!

### 📖 Tam Örnek: Button ve SelectBox

```rust
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// Kütüphanenin sağladığı Button
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button çiziliyor: {} ({}x{})", self.label, self.width, self.height);
    }
}

// Kullanıcının eklediği SelectBox (kütüphane yazarı bunu bilmiyordu!)
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("SelectBox çiziliyor: {} seçenek", self.options.len());
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Evet"),
                    String::from("Belki"),
                    String::from("Hayır"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("Tamam"),
            }),
        ],
    };

    screen.run();
}
```

**Çıktı:**
```
SelectBox çiziliyor: 3 seçenek
Button çiziliyor: Tamam (50x10)
```

### 🦆 Ördek Tiplemesi (Duck Typing) ve Rust'ın Güvenliği

Dinamik tipli dillerde "duck typing" vardır:
> *"Eğer ördek gibi yürüyorsa ve ördek gibi vaklıyorsa, o ördektir!"*

Rust'ta trait nesneleri ile benzer bir esneklik elde ederiz, **ama güvenliğimiz var!**

```rust
// ❌ String, Draw trait'ini implement etmiyor — DERLEME HATASI!
let screen = Screen {
    components: vec![Box::new(String::from("Merhaba"))],
};
```

**Hata:**
```
error[E0277]: the trait bound `String: Draw` is not satisfied
```

🎯 **Avantaj:** Dinamik dillerde bu hata runtime'da çöker. Rust'ta ise **derleme zamanında** yakalanır!

### ⚡ Statik vs. Dinamik Dispatch

Bu konu performans açısından çok önemlidir:

#### Statik Dispatch (Generics ile):

```rust
fn cagir<T: Draw>(component: &T) {
    component.draw();
}
```

- Derleyici **hangi metotun çağrılacağını derleme zamanında bilir**
- **Monomorphization** yapılır — her tip için ayrı kod üretilir
- Çok hızlıdır, inline optimizasyonu yapılabilir

#### Dinamik Dispatch (Trait Nesneleri ile):

```rust
fn cagir(component: &dyn Draw) {
    component.draw();
}
```

- Derleyici **hangi metotun çağrılacağını bilemez**
- Runtime'da trait nesnesinin içindeki **vtable**'a (sanal metot tablosu) bakılır
- Küçük bir **runtime maliyeti** vardır
- Inline optimizasyonu yapılamaz

### 📊 Karşılaştırma Tablosu

| Özellik | Generics (`<T: Draw>`) | Trait Nesnesi (`dyn Draw`) |
|---------|------------------------|----------------------------|
| Dispatch | Statik (compile-time) | Dinamik (runtime) |
| Performans | Çok yüksek | Biraz daha yavaş |
| Esneklik | Tek tip | Çoklu tip |
| Binary boyutu | Büyük (monomorphization) | Küçük |
| Kullanım | Homojen koleksiyonlar | Heterojen koleksiyonlar |

---

## 📚 BÖLÜM 18.3: NESNE YÖNELİMLİ TASARIM DESENLERİ — STATE PATTERN

Şimdi Rust'ta klasik bir OOP tasarım deseni olan **State Pattern**'i (Durum Deseni) uygulayacağız.

### 🎯 State Pattern Nedir?

Bir nesnenin davranışı, **durumuna (state)** göre değişir. Her durum, kendi davranışından ve bir sonraki duruma geçiş kurallarından sorumludur.

### 📖 Örnek: Blog Yazısı İş Akışı

Bir blog platformu için şu kuralları implement edelim:

1. Yeni bir yazı **taslak (draft)** olarak başlar
2. Taslak bitince **inceleme (review)** istenir
3. İnceleme onaylanınca **yayınlanır (published)**
4. **Sadece yayınlanmış yazılar** içerik döndürür

```rust
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("Bugün öğle yemeğinde salata yedim");
    assert_eq!("", post.content());  // Henüz taslak, içerik boş

    post.request_review();
    assert_eq!("", post.content());  // İncelemede, içerik boş

    post.approve();
    assert_eq!("Bugün öğle yemeğinde salata yedim", post.content());  // Yayında!
}
```

### 🏗️ YAKLAŞIM 1: Klasik OOP Tarzı State Pattern

#### Adım 1: Temel Yapı

```rust
pub struct Post {
    state: Option<Box<dyn State>>,  // Durum nesnesi
    content: String,
}

trait State {}

struct Draft {}
impl State for Draft {}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),  // Yeni yazı = taslak
            content: String::new(),
        }
    }
}
```

🔑 **Neden `Option`?** State'i `take()` ile alıp yeni bir state ile değiştireceğiz. Rust'ta bir struct alanını taşıyamayız (move), bu yüzden `Option::take()` ile geçici olarak `None` yaparız.

#### Adım 2: `add_text` Metodu

```rust
impl Post {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
```

Bu metot durumdan bağımsızdır — her durumda metin eklenebilir.

#### Adım 3: `content` Metodu (Geçici)

```rust
impl Post {
    pub fn content(&self) -> &str {
        ""  // Şimdilik her zaman boş — sonra düzelteceğiz
    }
}
```

#### Adım 4: State Transition — `request_review`

```rust
impl Post {
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})  // Draft → PendingReview
    }
}

struct PendingReview {}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self  // Zaten incelemede, değişme
    }
}
```

🎯 **Önemli:** `self: Box<Self>` sözdizimi, metot sadece `Box` içinde çağrıldığında geçerli demektir. Bu, eski state'i **tüketir (consume)** ve yeni bir state döndürür.

#### Adım 5: `approve` Metodu

```rust
impl Post {
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
}

impl State for Draft {
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self  // Draft direkt onaylanamaz
    }
}

impl State for PendingReview {
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})  // PendingReview → Published
    }
}

struct Published {}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> { self }
    fn approve(self: Box<Self>) -> Box<dyn State> { self }
}
```

#### Adım 6: `content` Metodunu State'e Devret

```rust
impl Post {
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
}

trait State {
    // ... diğer metotlar
    
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""  // Varsayılan: boş döndür
    }
}

struct Published {}
impl State for Published {
    // ... diğer metotlar
    
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content  // Sadece Published içerik döndürür!
    }
}
```

### 📊 State Pattern'in Avantajları

✅ **Post** metotları durumları bilmez — tüm kurallar state nesnelerinde  
✅ Yeni bir durum eklemek kolay — sadece yeni struct ve trait implementasyonu  
✅ Kod tekrarı azalır — `match` ifadeleri gerekmez  
✅ Kurallar tek bir yerde toplanır  

### ⚠️ Dezavantajları

❌ State'ler birbirine **bağlı (coupled)** — yeni state eklerken diğerleri değişebilir  
❌ Bazı kod tekrarları var (`request_review` ve `approve`'daki `take` mantığı)  
❌ Trait nesneleri dyn-compatibility kurallarına takılabilir  

---

### 🚀 YAKLAŞIM 2: Rust'ın Gücünü Kullanan Tip-Tabanlı Yaklaşım

Şimdi Rust'ın **tip sistemini** kullanarak çok daha güvenli bir yaklaşım görelim. Burada durumlar, **farklı tipler** ile temsil edilir!

#### Temel Fikir:

- `DraftPost` → `content()` metodu **YOK**!
- `PendingReviewPost` → `content()` metodu **YOK**!
- `Post` (yayınlanmış) → `content()` metodu **VAR**!

```rust
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    
    // Dikkat: content() metodu YOK!
}
```

#### Tip Dönüşümleri:

```rust
impl DraftPost {
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
```

🔑 **Kritik:** `self` ile ownership alınıyor — eski tip **tüketiliyor**, yeni tip üretiliyor!

#### Kullanım:

```rust
fn main() {
    let mut post = Post::new();  // DraftPost döner

    post.add_text("Bugün salata yedim");
    // post.content();  // ❌ DERLEME HATASI! DraftPost'ta content() yok

    let post = post.request_review();  // PendingReviewPost döner
    // post.content();  // ❌ DERLEME HATASI!

    let post = post.approve();  // Post döner
    assert_eq!("Bugün salata yedim", post.content());  // ✅ Artık çalışır!
}
```

### 🎉 Bu Yaklaşımın Muhteşem Avantajı

**Geçersiz durumlar derleme zamanında engellenir!**

- Taslak yazının içeriğini okumaya çalışmak → **Derleme hatası**
- Onaylanmamış yazıyı yayınlamak → **Derleme hatası**
- Üretim ortamında (production) hata olma ihtimali → **SIFIR!**

### 📊 İki Yaklaşımın Karşılaştırması

| Özellik | Klasik State Pattern | Tip-Tabanlı Yaklaşım |
|---------|---------------------|----------------------|
| Runtime hataları | Mümkün | İmkansız |
| Esneklik | Yüksek (runtime'da durum değişir) | Düşük (tip dönüşümü gerekir) |
| Performans | Dinamik dispatch maliyeti | Statik dispatch — çok hızlı |
| Kod karmaşıklığı | Yüksek | Düşük |
| OOP'ye benzerlik | Yüksek | Düşük |
| Rust idiomatic | Orta | Yüksek ✅ |

---

## 🎓 DERSTEN ÇIKARILACAK TEMEL DERSLER

### ✅ Rust OOP'de Neleri Destekler?

| OOP Özelliği | Rust Karşılığı |
|--------------|----------------|
| Nesneler | `struct` + `impl` blokları |
| Kapsülleme | `pub` / private alanlar |
| Kod paylaşımı | Trait'lerin varsayılan metotları |
| Polimorfizm | Trait nesneleri (`dyn Trait`) + Generics |
| Kalıtım | ❌ YOK — trait'ler ve kompozisyon ile değiştirilir |

### 🎯 Rust'ın Felsefesi

Rust, klasik OOP kalıplarını **taklit etmek yerine**, kendi güçlü özelliklerini (ownership, tip sistemi, trait'ler) kullanarak **daha güvenli** çözümler sunar.

> 💡 **Altın Kural:** OOP desenlerini Rust'a olduğu gibi aktarmaya çalışmak yerine, Rust'ın sunduğu araçlarla problemi yeniden düşünün!

### 🔑 Trait Nesneleri Ne Zaman Kullanılmalı?

✅ **Kullan:**
- Runtime'da farklı tipleri bir arada tutmanız gerekiyorsa
- Heterojen koleksiyonlar (farklı tipler aynı `Vec`'te)
- Plugin sistemi, strateji deseni

❌ **Kullanma:**
- Sadece tek tip varsa (generics daha hızlı)
- Performans kritikse (statik dispatch tercih edin)

---

## 🏁 SONUÇ

Bu derste Rust'ın nesne yönelimli programlamaya nasıl yaklaştığını öğrendik:

1. **Rust "saf" bir OOP dili değildir**, ancak OOP'nin önemli özelliklerini (kapsülleme, polimorfizm) kendi yoluyla sağlar.
2. **Kalıtım yerine trait'ler** kullanırız — daha esnek, daha güvenli.
3. **Trait nesneleri** ile runtime polimorfizmi elde ederiz, ancak performans maliyeti vardır.
4. **State pattern** gibi OOP desenleri Rust'ta implement edilebilir, ama Rust'ın tip sistemiyle daha güvenli çözümler üretmek genellikle daha iyidir.

🦀 **Unutmayın:** "Rust OOP dili midir?" tartışması önemli değil. Önemli olan, Rust'ın güçlü özelliklerini kullanarak **güvenli, sürdürülebilir ve performanslı** kod yazabilmektir!

Bir sonraki konuda görüşmek üzere — Mutlu kodlamalar! 🚀
</think>

# 🦀 Rust'ta Nesne Yönelimli Programlama (OOP) — Kapsamlı Ders

Hoş geldiniz! Bu derste, Rust programlama dilinin nesne yönelimli programlama (OOP) yaklaşımını, Rust kitabının 18. bölümünü temel alarak adım adım, ders anlatır gibi işleyeceğiz. Rust'ın klasik OOP dillerinden (Java, C++, C#) nasıl farklılaştığını, kendi güçlü özellikleriyle bu paradigmaya nasıl yaklaştığını göreceğiz.

---

## 📚 BÖLÜM 18: GİRİŞ — OOP Nedir, Rust Neden Farklı?

### 1.1 OOP'nin Tarihsel Arka Planı

Nesne yönelimli programlama kavramı ilk kez 1960'larda **Simula** dilinde ortaya çıktı. Alan Kay, nesnelerin birbirlerine mesaj geçtiği bir mimari tasarlayarak 1967'de "object-oriented programming" terimini ortaya attı.

> ⚠️ **Önemli Not:** OOP'nin tek bir doğru tanımı yoktur. Bazı tanımlara göre Rust nesne yönelimlidir, bazılarına göre değildir. Bu bölümde, OOP ile ilişkilendirilen özelliklerin Rust'ta nasıl karşılandığını göreceğiz.

---

## 📚 BÖLÜM 18.1: NESNE YÖNELİMLİ PROGRAMLAMA NEDİR?

OOP topluluğunda, bir dilin "nesne yönelimli" sayılması için hangi özelliklere sahip olması gerektiği konusunda bir fikir birliği yoktur. Ancak genel kabul gören **üç temel özellik** vardır:

1. **Nesneler (Objects)** — Veri + Davranış birleşimi
2. **Kapsülleme (Encapsulation)** — İç detayların gizlenmesi
3. **Kalıtım (Inheritance)** — Üst sınıftan özellik devralma

Hadi bunları tek tek Rust açısından inceleyelim.

---

### 🧱 1.1.1 Nesneler (Objects) — Veri ve Davranışın Birleşimi

> *"Nesne yönelimli programlar nesnelerden oluşur. Bir nesne, hem veriyi hem de bu veri üzerinde işlem yapan prosedürleri paketler. Bu prosedürlere genellikle **metot** veya **işlem** denir."*
> — Gang of Four (Dörtlü Çete) Kitabı

Rust'ta `struct` ve `enum`'lar **veri** tutar, `impl` blokları ise bu verilere **metot** sağlar. Yani Rust, Gang of Four tanımına göre nesne yönelimlidir!

```rust
struct Araba {
    marka: String,
    hiz: f64,
}

impl Araba {
    // Bu bir metottur — davranış
    fn hizlan(&mut self, miktar: f64) {
        self.hiz += miktar;
    }
    
    fn bilgi_goster(&self) {
        println!("{} - {} km/h", self.marka, self.hiz);
    }
}
```

---

### 🔒 1.1.2 Kapsülleme (Encapsulation) — İç Detayları Gizlemek

Kapsülleme, bir nesnenin iç implementasyon detaylarının dışarıdan erişime kapatılması anlamına gelir. Dış kod, nesneyle sadece **public API** üzerinden etkileşime girmelidir.

**Neden Önemli?**
- İç yapıyı istediğiniz zaman değiştirebilirsiniz (refactoring)
- Dış kodu bozmazsınız
- Veri tutarlılığını korursunuz

#### 📖 Örnek: `AveragedCollection` (Ortalama Koleksiyonu)

Bir koleksiyon düşünelim: İçinde `i32` değerleri tutuyor ve ortalamasını hesaplıyor. Ortalama, her istendiğinde yeniden hesaplanmak yerine **önbelleğe (cache)** alınsın.

```rust
pub struct AveragedCollection {
    list: Vec<i32>,      // private (pub yok!)
    average: f64,        // private (pub yok!)
}
```

🔴 **Dikkat:** Alanlar `pub` değil! Yani dışarıdan kimse doğrudan `list`'e ekleme/çıkarma yapamaz. Bu çok önemli çünkü `list` değiştiğinde `average`'ın da güncellenmesi gerekiyor.

#### Metotların Implementasyonu:

```rust
impl AveragedCollection {
    // ✅ Ekleme — hem list hem average güncellenir
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    // ✅ Çıkarma — hem list hem average güncellenir
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    // ✅ Sadece okuma — average değiştirilemez
    pub fn average(&self) -> f64 {
        self.average
    }

    // 🔒 Private metot — dışarıdan çağrılamaz
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
```

#### 🎯 Kapsüllemenin Gücü

Diyelim ki gelecekte `Vec<i32>` yerine `HashSet<i32>` kullanmak istediniz:

```rust
pub struct AveragedCollection {
    list: HashSet<i32>,   // Değişti!
    average: f64,
}
```

**Eğer alanlar private ise:** Sadece `add`, `remove`, `update_average` metotlarını güncellersiniz. Dışarıdaki kod **hiç değişmez!**

**Eğer alanlar public olsaydı:** Dışarıdaki tüm kodlar `HashSet`'in farklı API'sine göre güncellenmek zorunda kalırdı. 💥

---

### 🧬 1.1.3 Kalıtım (Inheritance) — Rust'ta Yok!

Kalıtım, bir nesnenin başka bir nesnenin tanımından veri ve davranışları **devralması** mekanizmasıdır.

> ❌ **Rust'ta kalıtım yoktur.** Bir struct'ın başka bir struct'tan alan ve metot implementasyonlarını miras almasının bir yolu yoktur (macro kullanmadan).

#### Peki Kalıtım Neden Kullanılır?

Genellikle **iki ana sebeple** kalıtıma ihtiyaç duyarız:

#### **Sebep 1: Kod Tekrarını Önlemek (Code Reuse)**

Rust'ta bunu **trait'lerin varsayılan metot implementasyonları** ile çözeriz:

```rust
pub trait Summary {
    // Varsayılan implementasyon
    fn summarize(&self) -> String {
        String::from("(Devamı okunuyor...)")
    }
}

struct Makale {
    baslik: String,
}

// Varsayılan summarize'ı kullanır — kod tekrarı yok!
impl Summary for Makale {}

struct Haber {
    baslik: String,
}

impl Summary for Haber {
    // Varsayılan implementasyonu override ederiz
    fn summarize(&self) -> String {
        format!("Son dakika: {}", self.baslik)
    }
}
```

#### **Sebep 2: Polimorfizm (Alt Tipleri Üst Tip Yerine Kullanma)**

Rust kalıtım yerine **generics + trait bounds** kullanır. Buna **bounded parametric polymorphism** denir.

```rust
// Java/C++ tarzı kalıtım YOK
// Rust tarzı trait ile polimorfizm VAR
fn yazdir(o: &impl Summary) {
    println!("{}", o.summarize());
}
```

#### 🤔 Rust Neden Kalıtımı Reddetti?

- Kalıtım genellikle **gereğinden fazla kod paylaşımına** yol açar
- Alt sınıflar, üst sınıfın **tüm** özelliklerini almak zorundadır (esneklik kaybı)
- Alt sınıfa **uygun olmayan metotlar** çağrılabilir
- Bazı diller sadece **tek kalıtıma** (single inheritance) izin verir

Rust bunun yerine **trait nesneleri** ile runtime polimorfizmi sağlar. Şimdi buna geçelim! 👇

---

## 📚 BÖLÜM 18.2: TRAIT NESNELERİ (Trait Objects) ve Polimorfizm

Bu bölüm Rust'ın en önemli OOP özelliklerinden birini içeriyor. Hazırsanız başlayalım!

### 🎯 Problem: Farklı Tipleri Bir Arada Tutma

Bir GUI (Grafik Arayüz) kütüphanesi yazdığınızı düşünün. Ekranda farklı tiplerde bileşenler olacak: `Button`, `TextField`, `Image`, `SelectBox`...

**Sorun:** `Vec` sadece **tek bir tip** tutabilir. Farklı tipleri bir arada nasıl tutacağız?

```rust
// ❌ OLMAZ — Vec tek tip tutar
let components = vec![Button::new(), TextField::new(), Image::new()];
```

### 💡 Çözüm: Trait Nesneleri (Trait Objects)

Trait nesnesi, belirli bir trait'i implement eden **herhangi bir tipe** işaret eden bir pointer'dır. Çalışma zamanında (runtime) hangi metotun çağrılacağını belirler.

#### 🔧 Sözdizimi:

```rust
Box<dyn Traitİsmi>    // Trait nesnesi
&dyn Traitİsmi         // Referans olarak trait nesnesi
```

- `Box<T>` — Heap üzerinde bir pointer
- `dyn` — "dinamik" anlamına gelir, bir trait nesnesi olduğunu belirtir

### 📖 Kapsamlı Örnek: GUI Kütüphanesi

#### Adım 1: `Draw` Trait'ini Tanımla

```rust
pub trait Draw {
    fn draw(&self);
}
```

#### Adım 2: `Screen` Struct'ını Trait Nesnesi ile Tanımla

```rust
pub struct Screen {
    // Farklı tiplerde bileşenleri tutabilir!
    pub components: Vec<Box<dyn Draw>>,
}
```

🔑 **Kritik Nokta:** `Vec<Box<dyn Draw>>`, `Draw` trait'ini implement eden **herhangi bir tip** içerebilir. Hem `Button`, hem `TextField`, hem de kullanıcının sonradan ekleyeceği `SelectBox`!

#### Adım 3: `run` Metodu

```rust
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();  // Her bileşenin kendi draw'ı çağrılır
        }
    }
}
```

### 🆚 Trait Nesnesi vs. Generics

Bu noktada aklınıza şu gelebilir: *"Generics ile de yapabilirdik!"* Haklısınız, ama farklar var:

#### Generic Yaklaşım:

```rust
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}
```

❌ **Sorun:** Bu `Screen` sadece **tek bir tip** tutabilir. Ya hep `Button`, ya hep `TextField`. İkisini bir arada tutamazsınız!

#### Trait Nesnesi Yaklaşımı:

```rust
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
```

✅ **Avantaj:** Tek bir `Screen` içinde hem `Button`, hem `TextField`, hem `Image` bir arada olabilir!

### 📖 Tam Örnek: Button ve SelectBox

```rust
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// Kütüphanenin sağladığı Button
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button çiziliyor: {} ({}x{})", self.label, self.width, self.height);
    }
}

// Kullanıcının eklediği SelectBox (kütüphane yazarı bunu bilmiyordu!)
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("SelectBox çiziliyor: {} seçenek", self.options.len());
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Evet"),
                    String::from("Belki"),
                    String::from("Hayır"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("Tamam"),
            }),
        ],
    };

    screen.run();
}
```

**Çıktı:**
```
SelectBox çiziliyor: 3 seçenek
Button çiziliyor: Tamam (50x10)
```

### 🦆 Ördek Tiplemesi (Duck Typing) ve Rust'ın Güvenliği

Dinamik tipli dillerde "duck typing" vardır:
> *"Eğer ördek gibi yürüyorsa ve ördek gibi vaklıyorsa, o ördektir!"*

Rust'ta trait nesneleri ile benzer bir esneklik elde ederiz, **ama güvenliğimiz var!**

```rust
// ❌ String, Draw trait'ini implement etmiyor — DERLEME HATASI!
let screen = Screen {
    components: vec![Box::new(String::from("Merhaba"))],
};
```

**Hata:**
```
error[E0277]: the trait bound `String: Draw` is not satisfied
```

🎯 **Avantaj:** Dinamik dillerde bu hata runtime'da çöker. Rust'ta ise **derleme zamanında** yakalanır!

### ⚡ Statik vs. Dinamik Dispatch

Bu konu performans açısından çok önemlidir:

#### Statik Dispatch (Generics ile):

```rust
fn cagir<T: Draw>(component: &T) {
    component.draw();
}
```

- Derleyici **hangi metotun çağrılacağını derleme zamanında bilir**
- **Monomorphization** yapılır — her tip için ayrı kod üretilir
- Çok hızlıdır, inline optimizasyonu yapılabilir

#### Dinamik Dispatch (Trait Nesneleri ile):

```rust
fn cagir(component: &dyn Draw) {
    component.draw();
}
```

- Derleyici **hangi metotun çağrılacağını bilemez**
- Runtime'da trait nesnesinin içindeki **vtable**'a (sanal metot tablosu) bakılır
- Küçük bir **runtime maliyeti** vardır
- Inline optimizasyonu yapılamaz

### 📊 Karşılaştırma Tablosu

| Özellik | Generics (`<T: Draw>`) | Trait Nesnesi (`dyn Draw`) |
|---------|------------------------|----------------------------|
| Dispatch | Statik (compile-time) | Dinamik (runtime) |
| Performans | Çok yüksek | Biraz daha yavaş |
| Esneklik | Tek tip | Çoklu tip |
| Binary boyutu | Büyük (monomorphization) | Küçük |
| Kullanım | Homojen koleksiyonlar | Heterojen koleksiyonlar |

---

## 📚 BÖLÜM 18.3: NESNE YÖNELİMLİ TASARIM DESENLERİ — STATE PATTERN

Şimdi Rust'ta klasik bir OOP tasarım deseni olan **State Pattern**'i (Durum Deseni) uygulayacağız.

### 🎯 State Pattern Nedir?

Bir nesnenin davranışı, **durumuna (state)** göre değişir. Her durum, kendi davranışından ve bir sonraki duruma geçiş kurallarından sorumludur.

### 📖 Örnek: Blog Yazısı İş Akışı

Bir blog platformu için şu kuralları implement edelim:

1. Yeni bir yazı **taslak (draft)** olarak başlar
2. Taslak bitince **inceleme (review)** istenir
3. İnceleme onaylanınca **yayınlanır (published)**
4. **Sadece yayınlanmış yazılar** içerik döndürür

```rust
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("Bugün öğle yemeğinde salata yedim");
    assert_eq!("", post.content());  // Henüz taslak, içerik boş

    post.request_review();
    assert_eq!("", post.content());  // İncelemede, içerik boş

    post.approve();
    assert_eq!("Bugün öğle yemeğinde salata yedim", post.content());  // Yayında!
}
```

### 🏗️ YAKLAŞIM 1: Klasik OOP Tarzı State Pattern

#### Adım 1: Temel Yapı

```rust
pub struct Post {
    state: Option<Box<dyn State>>,  // Durum nesnesi
    content: String,
}

trait State {}

struct Draft {}
impl State for Draft {}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),  // Yeni yazı = taslak
            content: String::new(),
        }
    }
}
```

🔑 **Neden `Option`?** State'i `take()` ile alıp yeni bir state ile değiştireceğiz. Rust'ta bir struct alanını taşıyamayız (move), bu yüzden `Option::take()` ile geçici olarak `None` yaparız.

#### Adım 2: `add_text` Metodu

```rust
impl Post {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
```

Bu metot durumdan bağımsızdır — her durumda metin eklenebilir.

#### Adım 3: `content` Metodu (Geçici)

```rust
impl Post {
    pub fn content(&self) -> &str {
        ""  // Şimdilik her zaman boş — sonra düzelteceğiz
    }
}
```

#### Adım 4: State Transition — `request_review`

```rust
impl Post {
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})  // Draft → PendingReview
    }
}

struct PendingReview {}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self  // Zaten incelemede, değişme
    }
}
```

🎯 **Önemli:** `self: Box<Self>` sözdizimi, metot sadece `Box` içinde çağrıldığında geçerli demektir. Bu, eski state'i **tüketir (consume)** ve yeni bir state döndürür.

#### Adım 5: `approve` Metodu

```rust
impl Post {
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
}

impl State for Draft {
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self  // Draft direkt onaylanamaz
    }
}

impl State for PendingReview {
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})  // PendingReview → Published
    }
}

struct Published {}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> { self }
    fn approve(self: Box<Self>) -> Box<dyn State> { self }
}
```

#### Adım 6: `content` Metodunu State'e Devret

```rust
impl Post {
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
}

trait State {
    // ... diğer metotlar
    
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""  // Varsayılan: boş döndür
    }
}

struct Published {}
impl State for Published {
    // ... diğer metotlar
    
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content  // Sadece Published içerik döndürür!
    }
}
```

### 📊 State Pattern'in Avantajları

✅ **Post** metotları durumları bilmez — tüm kurallar state nesnelerinde  
✅ Yeni bir durum eklemek kolay — sadece yeni struct ve trait implementasyonu  
✅ Kod tekrarı azalır — `match` ifadeleri gerekmez  
✅ Kurallar tek bir yerde toplanır  

### ⚠️ Dezavantajları

❌ State'ler birbirine **bağlı (coupled)** — yeni state eklerken diğerleri değişebilir  
❌ Bazı kod tekrarları var (`request_review` ve `approve`'daki `take` mantığı)  
❌ Trait nesneleri dyn-compatibility kurallarına takılabilir  

---

### 🚀 YAKLAŞIM 2: Rust'ın Gücünü Kullanan Tip-Tabanlı Yaklaşım

Şimdi Rust'ın **tip sistemini** kullanarak çok daha güvenli bir yaklaşım görelim. Burada durumlar, **farklı tipler** ile temsil edilir!

#### Temel Fikir:

- `DraftPost` → `content()` metodu **YOK**!
- `PendingReviewPost` → `content()` metodu **YOK**!
- `Post` (yayınlanmış) → `content()` metodu **VAR**!

```rust
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    
    // Dikkat: content() metodu YOK!
}
```

#### Tip Dönüşümleri:

```rust
impl DraftPost {
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
```

🔑 **Kritik:** `self` ile ownership alınıyor — eski tip **tüketiliyor**, yeni tip üretiliyor!

#### Kullanım:

```rust
fn main() {
    let mut post = Post::new();  // DraftPost döner

    post.add_text("Bugün salata yedim");
    // post.content();  // ❌ DERLEME HATASI! DraftPost'ta content() yok

    let post = post.request_review();  // PendingReviewPost döner
    // post.content();  // ❌ DERLEME HATASI!

    let post = post.approve();  // Post döner
    assert_eq!("Bugün salata yedim", post.content());  // ✅ Artık çalışır!
}
```

### 🎉 Bu Yaklaşımın Muhteşem Avantajı

**Geçersiz durumlar derleme zamanında engellenir!**

- Taslak yazının içeriğini okumaya çalışmak → **Derleme hatası**
- Onaylanmamış yazıyı yayınlamak → **Derleme hatası**
- Üretim ortamında (production) hata olma ihtimali → **SIFIR!**

### 📊 İki Yaklaşımın Karşılaştırması

| Özellik | Klasik State Pattern | Tip-Tabanlı Yaklaşım |
|---------|---------------------|----------------------|
| Runtime hataları | Mümkün | İmkansız |
| Esneklik | Yüksek (runtime'da durum değişir) | Düşük (tip dönüşümü gerekir) |
| Performans | Dinamik dispatch maliyeti | Statik dispatch — çok hızlı |
| Kod karmaşıklığı | Yüksek | Düşük |
| OOP'ye benzerlik | Yüksek | Düşük |
| Rust idiomatic | Orta | Yüksek ✅ |

---

## 🎓 DERSTEN ÇIKARILACAK TEMEL DERSLER

### ✅ Rust OOP'de Neleri Destekler?

| OOP Özelliği | Rust Karşılığı |
|--------------|----------------|
| Nesneler | `struct` + `impl` blokları |
| Kapsülleme | `pub` / private alanlar |
| Kod paylaşımı | Trait'lerin varsayılan metotları |
| Polimorfizm | Trait nesneleri (`dyn Trait`) + Generics |
| Kalıtım | ❌ YOK — trait'ler ve kompozisyon ile değiştirilir |

### 🎯 Rust'ın Felsefesi

Rust, klasik OOP kalıplarını **taklit etmek yerine**, kendi güçlü özelliklerini (ownership, tip sistemi, trait'ler) kullanarak **daha güvenli** çözümler sunar.

> 💡 **Altın Kural:** OOP desenlerini Rust'a olduğu gibi aktarmaya çalışmak yerine, Rust'ın sunduğu araçlarla problemi yeniden düşünün!

### 🔑 Trait Nesneleri Ne Zaman Kullanılmalı?

✅ **Kullan:**
- Runtime'da farklı tipleri bir arada tutmanız gerekiyorsa
- Heterojen koleksiyonlar (farklı tipler aynı `Vec`'te)
- Plugin sistemi, strateji deseni

❌ **Kullanma:**
- Sadece tek tip varsa (generics daha hızlı)
- Performans kritikse (statik dispatch tercih edin)

---

## 🏁 SONUÇ

Bu derste Rust'ın nesne yönelimli programlamaya nasıl yaklaştığını öğrendik:

1. **Rust "saf" bir OOP dili değildir**, ancak OOP'nin önemli özelliklerini (kapsülleme, polimorfizm) kendi yoluyla sağlar.
2. **Kalıtım yerine trait'ler** kullanırız — daha esnek, daha güvenli.
3. **Trait nesneleri** ile runtime polimorfizmi elde ederiz, ancak performans maliyeti vardır.
4. **State pattern** gibi OOP desenleri Rust'ta implement edilebilir, ama Rust'ın tip sistemiyle daha güvenli çözümler üretmek genellikle daha iyidir.

🦀 **Unutmayın:** "Rust OOP dili midir?" tartışması önemli değil. Önemli olan, Rust'ın güçlü özelliklerini kullanarak **güvenli, sürdürülebilir ve performanslı** kod yazabilmektir!

Bir sonraki konuda görüşmek üzere — Mutlu kodlamalar! 🚀

# DeepSeek Yorumu ---------------------

Merhaba! Şimdi Rust programlama dilinin resmî kitabı olan *The Rust Programming Language* içindeki **“Object Oriented Programming Features of Rust”** (Rust’ın Nesne Yönelimli Programlama Özellikleri) başlıklı bölümü (bazı basımlarda 18. bölüm, güncel kitapta 17. bölüm) baştan sona, bir ders anlatır gibi detaylıca ele alacağız. Anlatımı Türkçe yapacağım, örnek kod parçalarını da kitaptaki sırayla ve açıklamalarla birlikte vereceğim. Hazırsan başlayalım.

---

## Bölümün Amacı

Bu bölüm, Rust’ın nesne yönelimli programlamanın (OOP) yaygın olarak bilinen kavramlarını nasıl karşıladığını, nerelerde geleneksel OOP kalıplarından ayrıldığını ve Rust’ın kendine özgü araçlarıyla (trait’ler, trait nesneleri, enum’lar) bu kalıpları nasıl daha güvenli veya farklı şekilde gerçekleştirebileceğini öğretir. Temel olarak üç ana başlık altında ilerler:

1. **Characteristics of Object-Oriented Languages** (Nesne Yönelimli Dillerin Özellikleri)  
2. **Using Trait Objects That Allow for Values of Different Types** (Farklı Türlerden Değerlere İzin Veren Trait Nesnelerinin Kullanımı)  
3. **Implementing an Object-Oriented Design Pattern** (Nesne Yönelimli Bir Tasarım Desenini Uygulamak)

Bu başlıkları sırayla ayrıntılandıracağız.

---

## 1. Nesne Yönelimli Dillerin Özellikleri (Characteristics of Object-Oriented Languages)

OOP denince genellikle üç ana kavram akla gelir:

- **Nesneler** (Objects)
- **Kapsülleme** (Encapsulation)
- **Kalıtım ve çok biçimlilik** (Inheritance / Polymorphism)

Kitap önce Rust’ın bu kavramlarla ilişkisini sorguluyor.

### Nesneler (Objects) ve Rust

Birçok tanıma göre nesne, **veriyi** ve bu **veri üzerinde işlem yapan metotları** bir arada tutan yapıdır. Rust’ta `struct` (veya `enum`) ile veriyi tanımlar, bu yapıya `impl` bloğuyla metot ekleriz. Dolayısıyla Rust’taki `struct` + `impl` ikilisi klasik anlamda birer nesnedir.

Örnek:
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```
Burada `Rectangle` veriyi, `area` ise davranışı temsil eder; yani bir nesnemiz var.

### Kapsülleme (Encapsulation)

Kapsülleme, nesnenin iç detaylarının dışarıdan gizlenmesi ve yalnızca belirli bir arayüzle erişilmesidir. Rust’ta **görünürlük** `pub` anahtar kelimesi ile kontrol edilir. Bir modül içinde `pub` olmayan öğeler dışarıdan erişilemez; yapılandırıcı (struct) alanları da varsayılan olarak gizlidir, metotlar ise `pub` yapılıp yapılmayacağına göre arayüz tanımlanır.

Örneğin bir `AveragedCollection` yapısı düşünelim:
```rust
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}
```
Bu haliyle `list` ve `average` dışarıdan erişilebilir (çünkü alanlar `pub` yapılmamış ama `struct` `pub` olsa da alanlar varsayılan olarak gizlidir). Aslında Rust’ta bir `struct` `pub` ise alanlar aksi belirtilmedikçe gizlidir. Kitapta tam tersi bir örnek verilir: `pub struct AveragedCollection` içinde alanlar `list` ve `average` özel kalır, dış dünya yalnızca `add`, `remove`, `average` gibi `pub` metotlar üzerinden erişebilir. Böylece kapsülleme sağlanır.

Rust’ta kapsülleme isteğe bağlı olarak modül sınırlarında çalışır; istersen her şeyi açabilirsin, ancak iyi bir tasarım genellikle iç detayları gizler.

### Kalıtım (Inheritance)

Rust’ta **yapısal kalıtım (bir struct’ın başka bir struct’ı miras alması) yoktur**. Bunun yerine kod paylaşımı ve çok biçimlilik için **trait’ler** ve **generics** kullanılır.

Peki neden kalıtım yok? Kitap iki ana kullanım alanından bahseder:

- **Kod paylaşımı (code reuse):** Rust’ta bunu varsayılan metotlu trait’ler, generic yapılar ve kompozisyon ile sağlarsın.
- **Alt tipleme ve çok biçimlilik (subtyping & polymorphism):** Aynı anda birden çok türü aynı yerde kullanabilme. Rust bunu **trait nesneleri** (`dyn Trait`) veya **enum** ve `match` ile çözer.

Sonuç: Rust, klasik OOP’deki kalıtım ağaçları yerine daha esnek ve güvenli alternatifler sunar. Kitap bu bölümde özellikle **trait nesneleri** üzerine yoğunlaşacağını söyler.

---

## 2. Farklı Türlerden Değerlere İzin Veren Trait Nesnelerinin Kullanımı (Using Trait Objects)

Bu alt bölüm, çok biçimliliği (polymorphism) Rust’ta nasıl **dinamik gönderim** (dynamic dispatch) ile yapabileceğimizi anlatır.

### Neden Trait Nesnelerine İhtiyaç Duyarız?

Bir GUI kütüphanesi yazdığımızı hayal edelim. `Button`, `TextField` gibi bileşenlerin hepsi `draw` metotuna sahip olsun ve bir listede karışık tutulabilsin. Klasik OOP’de ortak bir ata sınıfı (örneğin `Component`) yapar, `draw` metodunu sanal (virtual) tanımlar ve her alt sınıf onu ezerdi (override).

Rust’ta bu senaryoyu iki şekilde yapabiliriz:

- **Generic ve statik gönderim:** `fn draw_all<T: Draw>(components: &[T])` gibi. Bu derleme zamanında her tür için ayrı kod üretir (monomorfizasyon), ancak listenin **tek bir türden** oluşmasını zorunlu kılar. Farklı türleri aynı listede tutamayız.
- **Trait nesneleri ve dinamik gönderim:** `Vec<Box<dyn Draw>>` kullanarak farklı türleri aynı koleksiyona koyabiliriz. Çalışma zamanında hangi `draw` metodunun çağrılacağına karar verilir.

İşte trait nesneleri bu ikinci yolu açar.

### Trait Nesnesi Nedir?

Bir trait nesnesi, belirli bir trait’i uygulayan herhangi bir türün örneğini işaret eden bir işaretçidir (genellikle `Box<dyn Trait>` veya `&dyn Trait`). `dyn` anahtar kelimesi “dinamik” olduğunu vurgular.

Örneğin:

```rust
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw(); // dinamik gönderim
        }
    }
}
```

`Screen` yapısı `Box<dyn Draw>` listesini tutar. `Box` ile heap’te yer alan türleri sahipleniriz. `dyn Draw` ise “Draw trait’ini uygulayan herhangi bir tür” anlamına gelir. `run` metodu içinde `component.draw()` çağrısı çalışma zamanında doğru `draw` implementasyonuna yönlendirilir.

### Dinamik Gönderim ile Statik Gönderim Karşılaştırması

Aynı problemi generic ile çözseydik:

```rust
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw(); // statik gönderim
        }
    }
}
```

Bu yapıda `Screen<Button>` ve `Screen<TextField>` ayrı ayrı oluşturulur, aynı ekranda ikisi bir arada olamaz. Generic yapı tek bir tür üzerine kilitlenir, trait nesnesi ise heterojen koleksiyona izin verir.

Performans açısından: statik gönderimde derleyici hangi fonksiyonun çağrılacağını bilir, dolayısıyla ek bir yönlendirme maliyeti yoktur; hatta inline edilebilir. Dinamik gönderimde ise bir **vtable** (sanal fonksiyon tablosu) üzerinden dolaylı çağrı yapılır, bu da hafif bir çalışma zamanı maliyeti getirir. Ancak esneklik kazanırız.

### Trait Nesnelerinde Nesne Güvenliği (Object Safety)

Her trait, trait nesnesi olarak kullanılamaz. Kullanılabilmesi için trait’in **nesne güvenli** (object safe) olması gerekir. Bir trait’in nesne güvenli olması için:

- `Self` dönüş tipi olan metotlar içermemelidir (örneğin `fn clone(&self) -> Self`).
- Metotlarının hiçbiri generic parametre almamalıdır.

Örneğin `Clone` trait’i object safe değildir çünkü `clone` metodu `-> Self` döner. Dolayısıyla `Box<dyn Clone>` oluşturamazsınız.

Kitap burada `Clone` örneğini verir: `pub trait Clone { fn clone(&self) -> Self; }` nesne güvenli değildir. Eğer bir trait nesnesini klonlamak isteseydik, farklı bir tasarım gerekirdi.

Neyse ki çoğu trait nesne güvenlidir. Basit arayüzlerde (`draw` gibi) sorun çıkmaz.

---

## 3. Nesne Yönelimli Bir Tasarım Desenini Uygulamak (Implementing an Object-Oriented Design Pattern)

Bu uzun bölüm, OOP’de meşhur **State (Durum) tasarım desenini** Rust’ta trait nesneleriyle ve alternatif olarak Rust’a özgü yaklaşımlarla nasıl gerçekleyebileceğimizi adım adım gösterir. Amaç, Rust’ın OOP desenlerini uygularken fırsatlarını ve takasları (trade-off) göstermektir.

### Durum Deseni (State Pattern) Nedir?

Bir blog yazısının akışını düşünelim: Taslak (Draft) → İncelemede (PendingReview) → Yayınlandı (Published). Durum deseni, bir nesnenin iç durumuna bağlı olarak davranışının değişmesini sağlar. Durumlar ayrı sınıflar/struct’lar olarak temsil edilir ve ana nesne (context) mevcut durumu bir durum nesnesine devreder.

Rust’ta bunu trait nesneleriyle modelleyelim.

### İlk Tasarım: Trait Nesneleri ile State Pattern

Bir `Post` yapımız olacak; içinde `state: Option<Box<dyn State>>` ve `content: String` tutacak. `State` trait’i de `request_review` ve `approve` gibi metotlar tanımlayacak. Durumlar arası geçişlerde `state` alanı yeni bir durum nesnesi ile değiştirilecek.

`State` trait’i:
```rust
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str { "" }
}
```
Burada önemli bir ayrıntı: metotlar `self: Box<Self>` alır. Bu, metot çağrıldığında `Box` içindeki `self`'i tüketip yerine yeni bir `Box<dyn State>` dönebilmemizi sağlar. Eski durum yok edilir, yerine yenisi gelir. `content` varsayılan olarak boş string döner.

`Draft` durumu:
```rust
struct Draft;

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self // Taslak onaylanamaz, kendini döner
    }
}
```

`PendingReview` durumu:
```rust
struct PendingReview;

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self // Zaten incelemede
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}
```

`Published` durumu:
```rust
struct Published;

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
```

`Post` yapısı ve metotları:
```rust
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}
```

Nasıl işler? `request_review` çağrıldığında `self.state.take()` ile `Option` içinden `Box<dyn State>`'i alırız (yerine `None` gelir). Sonra mevcut durumun `request_review` metodu kendini tüketerek yeni bir durum döner, biz de onu `state`'e yerleştiririz. `Draft` durumunda `PendingReview` döner, böylece geçiş tamamlanır. `content` metodu ise duruma göre ya boş string (`Draft`, `PendingReview`) ya da `post.content` dilimini (`Published`) döner.

Bu yaklaşım klasik State desenine oldukça yakındır; davranış durum nesnelerine dağıtılmıştır.

### Rust’ın Önerdiği Alternatif: Durumu Tip Sistemine Kodlamak

Kitap ikinci yarıda aynı blog iş akışını **farklı bir Rustacean yaklaşımla** yeniden yazar: durumları farklı **struct türleri** olarak kodlar ve tip sistemini kullanarak geçersiz durum geçişlerini derleme zamanında engeller. Burada trait nesnelerine ihtiyaç kalmaz.

Yeni tasarımda `Post` yerine `DraftPost`, `PendingReviewPost` ve nihayet `Post` gibi türler olur. Kullanıcı sadece `Post` üzerinden `content` görebilir. Geçişler sahiplik transferiyle (ownership) yapılır.

Örnek iskelet:

```rust
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
```

Kullanım:
```rust
let mut post = Post::new();          // DraftPost
post.add_text("Merhaba dünya");
let post = post.request_review();    // PendingReviewPost
let post = post.approve();           // Post
assert_eq!("Merhaba dünya", post.content());
```

Bu tasarımda `DraftPost` üzerinde `approve` diye bir metot yoktur, derleme hatası alırsınız. Aynı şekilde `PendingReviewPost`'un `content` metodu yoktur. Geçişler türler arası dönüşümlerle garanti altına alınır. Bu, **tür durum deseni** (typestate pattern) olarak bilinir ve Rust’ta sıfır maliyetli bir soyutlama sağlar; ekstra `Option` veya `Box` gerekmez, dinamik gönderim olmadığı için daha hızlıdır, yanlış kullanımı derleme zamanında yakalar.

### İki Yaklaşımın Karşılaştırması

Kitap son olarak bir karşılaştırma yapar:

- **Trait nesneli State deseni:** Klasik OOP’ye benzer, durumların tamamen ayrı struct’lar olmasını sağlar, yeni durum eklemek kolaydır (yeni bir struct yaz, trait’i uygula). Ancak çalışma zamanı maliyeti vardır ve bazı hatalar ancak çalışma zamanında fark edilir (örneğin yanlış durumda içerik istemek). Ayrıca trait’in object safe olması gerekir.
- **Tür durum (Typestate) yaklaşımı:** Derleme zamanında güvenlik, daha hızlı performans sunar. Fakat durum geçiş mantığı farklı türler arasında dağıldığı için bazen kod tekrarı olabilir ve daha katıdır. Hangi yöntemin seçileceği probleme bağlıdır.

Kitap, Rust’ın size OOP desenlerini farklı şekillerde uygulama özgürlüğü verdiğini, hatta geleneksel OOP dillerinde mümkün olmayan tip güvencelerini sağlayabileceğinizi vurgular.

---

## Özet

Bu bölümden öğrendiklerimiz:

- Rust’ta **struct + impl** ile nesne benzeri yapılar kurulabilir, **kapsülleme** `pub` ile kontrol edilir.
- Rust **kalıtım** yerine **trait ve generics** kullanır.
- Çok biçimlilik (polymorphism) için **trait nesneleri** (`dyn Trait`) ile dinamik gönderim yapılabilir; bu sayede farklı türler aynı koleksiyonda tutulabilir.
- Trait nesneleri **nesne güvenli** olmalıdır (`Self` döndüren veya generic metot içermez).
- State deseni hem trait nesneleriyle (dinamik) hem de **tür durum** (typestate) ile statik olarak gerçeklenebilir; her birinin avantajları ve takasları vardır.
- Rust, OOP kalıplarını uygularken daha fazla esneklik ve güvenlik sağlayan araçlar sunar.

Böylece Rust’ın OOP özelliklerini kapsamlı bir şekilde incelemiş olduk. Soruların olursa memnuniyetle cevaplarım.