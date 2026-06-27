# 🦀 Ders Notları: Bölüm 20 - Gelişmiş Özellikler

Harika bir seçim! Rust'ın resmi kitabının 20. Bölümü olan **"Gelişmiş Özellikler" (Advanced Features)**, dilin temel yapı taşlarını (ownership, trait'ler, lifetime'lar vb.) öğrendikten sonra, Rust'ın sunduğu daha niş, karmaşık ama bir o kadar da güçlü özelliklerini inceleyen bir başyapıttır. 

Bu özellikleri her gün standart bir uygulama geliştirirken kullanmayabilirsiniz; ancak bir kütüphane yazarıysanız, sistem seviyesinde bir iş yapıyorsanız veya C gibi dillerle entegrasyon sağlıyorsanız bu araçlar sizin en büyük silahlarınız olacaktır.

Gel, bu bölümü bir sınıf ortamında, ders anlatır gibi ve bol bol örnekle detaylandıralım. Bölüm 5 ana başlıktan oluşuyor:

---

### 1. Unsafe Rust (Güvensiz Rust)
Rust, bellek güvenliği (memory safety) ile ünlüdür. Ancak bazı durumlarda (örneğin C ile entegrasyon veya performansın kritik olduğu düşük seviyeli işlemler) derleyicinin statik kontrollerini devre dışı bırakmanız gerekir. İşte `unsafe` blokları burada devreye girer. `unsafe` kullanmak yanlış bir şey değildir, ancak sorumluluğu derleyiciden size devreder.

`unsafe` ile yapabileceğiniz 5 "süper güç" vardır:

1. **Ham İşaretçileri (Raw Pointers) Kullanmak:**
   Rust'ta `*const T` (değişmez) ve `*mut T` (değişir) şeklinde ham işaretçiler oluşturabilirsiniz. İlginçtir ki, bu işaretçileri *güvenli* kodda oluşturabilirsiniz, ancak onların gösterdiği adresteki veriye erişmek (dereference) için `unsafe` bloğuna ihtiyacınız vardır.
   ```rust
   let mut num = 5;
   let r1 = &raw const num; // *const i32
   let r2 = &raw mut num;   // *mut i32

   unsafe {
       println!("r1 is: {}", *r1);
       println!("r2 is: {}", *r2);
   }
   ```
   *Neden kullanılır?* Genellikle C diliyle haberleşirken veya borrow checker'ın anlayamayacağı karmaşık veri yapıları (örneğin aynı bellek bloğunun farklı parçalarına ait iki mutable referans) oluştururken kullanılır.

2. **Güvenli Olmayan (Unsafe) Fonksiyonları Çağırmak:**
   Tanımından önce `unsafe fn` yazılan fonksiyonlardır. Bunları çağırırken, fonksiyonun gerektirdiği güvenlik sözleşmelerini (contract) yerine getirdiğinizi garanti etmeniz gerekir.
   ```rust
   unsafe fn dangerous() {}
   unsafe { dangerous(); } // unsafe blok içinde çağrılmalı
   ```
   **Güvenli Soyutlama (Safe Abstraction):** Standart kütüphanedeki `split_at_mut` fonksiyonu bunun harika bir örneğidir. Fonksiyonun kendisi güvenlidir (`fn`), ancak içinde ham işaretçilerle çalışmak için `unsafe` blok barındırır. Dışarıya güvenli bir API sunarken, içerideki unsafe kodu gizler.

3. **Yabancı Fonksiyon Arayüzü (FFI - Foreign Function Interface):**
   Başka bir dilde (örneğin C) yazılmış fonksiyonları çağırmak için `extern` blokları kullanılır.
   ```rust
   unsafe extern "C" {
       fn abs(input: i32) -> i32;
   }
   ```
   Rust, C'nin bellek kurallarını uygulayamayacağı için bu çağrılar `unsafe` kabul edilir.

4. **Değişebilir Statik Değişkenlere (Mutable Static Variables) Erişmek:**
   Rust global değişkenlere (`static`) izin verir. Değişmez olanlara erişim güvenli olsa da, `static mut` olarak tanımlanan değişkenlere erişim ve onları değiştirmek `unsafe`'tir çünkü veri yarışlarına (data races) yol açabilir.

5. **Güvenli Olmayan Trait'leri (Unsafe Traits) Uygulamak:**
   Bir trait'in en az bir metodunun derleyicinin doğrulayamayacağı bazı değişmezler (invariants) gerektiriyorsa, trait `unsafe trait` olarak tanımlanır. (Örn: `Send` ve `Sync` trait'leri).

**Miri Aracı:** Unsafe kod yazdığınızda, kodunuzun Undefined Behavior (Tanımsız Davranış) üretip üretmediğini kontrol etmek için Rust'ın resmi dinamik analiz aracı **Miri**'yi kullanabilirsiniz.

---

### 2. Gelişmiş Trait'ler (Advanced Traits)
Trait'leri daha önce görmüştük, şimdi onların gelişmiş kullanım alanlarına ve tip sistemindeki ince ayarlarına bakalım.

1. **İlişkili Türler (Associated Types):**
   Bir trait içinde `type Item;` gibi yer tutucular tanımlamanızı sağlar. En ünlü örnek standart kütüphanedeki `Iterator` trait'idir.
   ```rust
   pub trait Iterator {
       type Item;
       fn next(&mut self) -> Option<Self::Item>;
   }
   ```
   *Generics ile farkı nedir?* Generics kullansaydık (`trait Iterator<T>`), bir tip için `Iterator` trait'ini birden fazla kez (farklı T türleriyle) uygulayabilirdik. Associated type kullandığımızda ise bir tip için trait'i sadece **bir kez** uygulayabiliriz. Bu, `next` metodunu çağırırken her seferinde tür belirtme zorunluluğunu ortadan kaldırır ve API'yi temizler.

2. **Varsayılan Tür Parametreleri ve Operatör Aşırı Yükleme:**
   Generic bir tür parametresine varsayılan bir değer verebilirsiniz. Örneğin `Add` trait'i `Rhs=Self` (Right-hand side) varsayılanına sahiptir. Bu, `+` operatörünü aşırı yüklerken (overload) kolaylık sağlar.
   ```rust
   impl Add for Point {
       type Output = Point;
       fn add(self, other: Point) -> Point { /* ... */ }
   }
   ```
   Eğer farklı türleri toplamak isterseniz (örn: `Millimeters` + `Meters`), `impl Add<Meters> for Millimeters` şeklinde varsayılanı ezerek kullanabilirsiniz.

3. **Tam Nitelikli Sözdizimi (Fully Qualified Syntax):**
   Bir tip, aynı isimde metoda sahip birden fazla trait'i uyguluyorsa (veya tipin kendi metodu da varsa), Rust hangisini çağıracağını bilemez. Bu durumda `<Type as Trait>::method()` sözdizimi kullanılır.
   ```rust
   Pilot::fly(&person); // Pilot trait'indeki fly
   Wizard::fly(&person); // Wizard trait'indeki fly
   ```
   Eğer metod `self` parametresi almıyorsa (associated function), tam nitelikli sözdizimi zorunludur: `<Dog as Animal>::baby_name()`.

4. **Üst Trait'ler (Supertraits):**
   Bir trait'in çalışması için başka bir trait'in de uygulanmış olmasını gerektirebilirsiniz.
   ```rust
   trait OutlinePrint: fmt::Display {
       fn outline_print(&self) {
           let output = self.to_string(); // Display trait'inden gelir
           // ...
       }
   }
   ```

5. **Newtype Deseni (Newtype Pattern):**
   Rust'ın "Orphan Rule" (Yetim Kuralı) gereği, hem trait hem de tip sizin crate'inize ait değilse, o trait'i o tip için uygulayamazsınız. Bunu aşmak için tipi bir tuple struct içine sararsınız (wrapper).
   ```rust
   struct Wrapper(Vec<String>);
   impl fmt::Display for Wrapper { /* ... */ }
   ```

---

### 3. Gelişmiş Türler (Advanced Types)

1. **Newtype Deseninin Diğer Kullanımları:**
   Sadece orphan rule'u aşmak için değil, birimleri (örn: `Millimeters`, `Meters`) ayırt etmek, API'yi kısıtlamak veya implementasyon detaylarını gizlemek (encapsulation) için de kullanılır.

2. **Tür Eş İsimleri (Type Aliases):**
   Mevcut bir türe yeni bir isim vermek için `type` anahtar kelimesi kullanılır. Bu, yeni bir tür oluşturmaz (newtype'ın aksine), sadece ismi kısaltır.
   ```rust
   type Kilometers = i32; // Tür güvenliği sağlamaz, sadece okunabilirliği artırır
   type Thunk = Box<dyn Fn() + Send + 'static>; // Uzun tipleri kısaltmak için harika
   ```
   Standart kütüphanedeki `std::io::Result<T>` aslında `Result<T, std::io::Error>` için bir type alias'tır.

3. **Asla Türü (The Never Type `!`):**
   Rust'ta `!` özel bir türdür ve "hiçbir değer döndürmeme" anlamına gelir. `panic!()`, `continue` veya sonsuz döngüye giren fonksiyonlar bu türü döndürür. Buna "ıraksayan fonksiyonlar" (diverging functions) denir.

4. **Dinamik Boyutlu Türler (Dynamically Sized Types - DSTs):**
   `str` (string slice değil, kendisi) veya trait'ler DST'dir. Derleme zamanında boyutları bilinmez.
   **Altın Kural:** DST'leri her zaman bir işaretçinin (pointer, referans, `Box` vb.) arkasında tutmalısınız.
   Rust, generic fonksiyonlara örtük olarak `Sized` trait bound'u ekler. Eğer DST'leri de kabul etmek istiyorsanız `?Sized` kullanmalısınız:
   ```rust
   fn generic<T: ?Sized>(t: &T) { /* ... */ }
   ```

---

### 4. Gelişmiş Fonksiyonlar ve Closure'lar

1. **Fonksiyon İşaretçileri (Function Pointers `fn`):**
   Closure'ları (`Fn`, `FnMut`, `FnOnce`) fonksiyonlara parametre olarak verebildiğimizi biliyoruz. Ancak bazen doğrudan bir fonksiyonun kendisini de parametre olarak vermek isteyebilirsiniz. Bunun için `fn` (küçük harf) türü kullanılır.
   ```rust
   fn add_one(x: i32) -> i32 { x + 1 }
   fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 { f(arg) + f(arg) }
   ```
   *Not:* `fn` türü, closure trait'lerini otomatik olarak uygular. Yani `fn` alan bir fonksiyona closure da verebilirsiniz. Ancak C gibi closure'ı olmayan dillerle FFI yaparken sadece `fn` kullanabilirsiniz.

2. **Closure Döndürmek:**
   Closure'ların kendilerine has, isimsiz tipleri vardır. Bu yüzden bir fonksiyondan closure döndürmek istiyorsanız, `impl Trait` sözdizimini kullanmalısınız:
   ```rust
   fn returns_closure() -> impl Fn(i32) -> i32 {
       |x| x + 1
   }
   ```
   Eğer birden fazla farklı closure'ı bir `Vec` içinde tutacaksanız (ki her closure farklı bir tiptir), `impl Trait` işe yaramaz. Bunun yerine Trait Object kullanıp heap'e allocate etmeniz gerekir:
   ```rust
   fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
       Box::new(|x| x + 1)
   }
   ```

---

### 5. Makrolar (Macros)
Makrolar, kod yazan kodlardır (metaprogramming). Fonksiyonlardan farkı; derleme zamanında genişlemeleri, değişken sayıda parametre alabilmeleri ve trait implementasyonu gibi derleme zamanı işlemleri yapabilmeleridir.

1. **Bildirimsel Makrolar (Declarative Macros / `macro_rules!`):**
   `match` ifadelerine benzer bir sözdizimiyle çalışır. Rust kodunun yapısını desenlerle (pattern) eşleştirir ve yeni kod üretir.
   Örneğin `vec!` makrosu:
   ```rust
   macro_rules! vec {
       ( $( $x:expr ),* ) => {
           {
               let mut temp_vec = Vec::new();
               $( temp_vec.push($x); )*
               temp_vec
           }
       };
   }
   ```
   `$x:expr` bir ifadeyi yakalar, `*` sıfır veya daha fazla tekrarı ifade eder.

2. **Prosedürel Makrolar (Procedural Macros):**
   Daha karmaşıktırlar. Bir `TokenStream` girdisi alıp, Rust kodunu manipüle ederek yeni bir `TokenStream` çıktısı verirler. Kendi crate'lerinde tanımlanmaları gerekir. 3 çeşittir:
   - **Custom `#[derive]` Makroları:** Struct ve Enum'lara `#[derive(MyTrait)]` yazarak otomatik trait implementasyonları üretir. (Örn: `serde`, `sqlx` gibi kütüphanelerde çokça kullanılır).
   - **Attribute-like Makrolar:** `#[route(GET, "/")]` gibi yeni attribute'lar tanımlamanızı sağlar.
   - **Function-like Makrolar:** `sql!(SELECT * FROM posts)` gibi fonksiyon çağrısına benzer şekilde çalışan, karmaşık girdileri parse edip kod üreten makrolardır.

---

### Özetle
Bu bölüm, Rust'ın "sınırlarını" zorladığınız yerleri anlatır. `unsafe` ile derleyicinin korumasını devre dışı bırakıp sisteme hükmedebilir; Gelişmiş Trait'ler ve Türler ile tip sisteminin en ince detaylarına inebilir; Makrolar ile kendi mini-dilinizi (DSL) yaratabilirsiniz. Bu araçlar günlük kodlamada her gün elinizin altında olmayabilir, ancak bir kütüphane yazarıysanız veya çok spesifik bir sistem problemiyle karşılaşıyorsanız Rust'ın size sunduğu en güçlü silahlardır.

# DeepSeek Yorumu -------------------------

Merhaba! Şimdi, Rust programlama dilinin en ileri seviye konularını kapsayan “Advanced Features” (İleri Düzey Özellikler) bölümünü, tıpkı bir ders anlatır gibi, adım adım, bol örnekli ve kapsamlı bir şekilde açıklayacağım. İnternet adresinde her ne kadar `ch20-00` yazsa da, şu anki Rust Kitabı’nın güncel sürümünde bu konular 19. bölümde yer alır; içerik tamamen aynıdır. Hazırsan başlayalım.

---

## Giriş: Neden İleri Seviye Özellikler?

Rust’ın temel özellikleri (mülkiyet, ödünç alma, ömürler, trait’ler, enum’lar vb.) çoğu güvenli sistem programlama ihtiyacını karşılar. Ancak bazen dilin güvenlik çitlerinin ardına geçmek, çok daha esnek kod yazmak veya düşük seviyeli donanım erişimi yapmak gerekir. İşte bu bölüm, Rust’ın size sunduğu o “kapıların kilidini açan” ileri seviye mekanizmaları anlatır:

1. **Unsafe Rust** (Güvensiz Rust)
2. **Advanced Traits** (İleri Düzey Trait’ler)
3. **Advanced Types** (İleri Düzey Tipler)
4. **Advanced Functions and Closures** (İleri Düzey Fonksiyonlar ve Closure’lar)
5. **Macros** (Makrolar)

Her birini ustalıkla kullanmayı öğrendiğinde, Rust’ın sınırlarını sonuna kadar zorlayabilirsin.

---

## 1. Unsafe Rust – Güvensiz Rust

Rust’ın derleyicisi, bellek güvenliğini garanti altına almak için sıkı kurallar koyar. Fakat bazı durumlarda yaptığımız işlemin güvenli olduğunu **biz biliriz**, derleyici bunu ispatlayamaz. İşte o zaman `unsafe` anahtar kelimesi ile derleyiciye “Ben sorumluluğu alıyorum, kuralları kontrol etme” deriz.

Önemli not: `unsafe` kod yazmak, illa bellek sızıntısı ya da güvenlik açığı yaratacaksın demek değildir; yalnızca derleyici beş ekstra özelliği kullanmana izin verir. Bu özelliklerin yanlış kullanımı tanımsız davranışa (undefined behavior) yol açar.

`unsafe` blok ya da fonksiyon içinde şunları yapabilirsin:
- Ham işaretçi (raw pointer) tanımlamak ve kullanmak
- `unsafe` bir fonksiyon ya da metodu çağırmak
- Yabancı dil arayüzü (FFI) ile `extern` fonksiyon çağırmak
- `static` değişkenlere erişmek ya da değiştirmek
- `unsafe` bir trait’i implemente etmek

Şimdi her birini ayrıntılı inceleyelim.

### 1.1 Ham İşaretçiler (Raw Pointers)

Rust’ta iki tür ham işaretçi vardır:
- `*const T` (değiştirilemez ham işaretçi)
- `*mut T` (değiştirilebilir ham işaretçi)

Ham işaretçiler referanslardan farklıdır:
- Ödünç alma kurallarına tabi değildirler (aynı anda hem `*mut` hem de `*const` olabilir).
- Geçerli bir belleği göstermek zorunda değillerdir.
- `null` olabilirler.
- Otomatik temizleme (drop) uygulanmaz.

Ham işaretçileri `unsafe` blok olmadan oluşturabilirsin, ancak **dereference etmek** (işaret ettiği yeri okumak/yazmak) için mutlaka `unsafe` blok gerekir.

```rust
let mut sayi = 5;

let r1 = &sayi as *const i32;   // oluşturma güvenli
let r2 = &mut sayi as *mut i32; // oluşturma güvenli

unsafe {
    println!("r1: {}", *r1);
    *r2 = 10;
    println!("r2: {}", *r2);
}
```

**Neden ham işaretçi kullanırız?**
- Donanım ile doğrudan konuşmak (memory-mapped I/O)
- C kütüphaneleriyle çalışırken (FFI)
- Yüksek performans gerektiren veri yapılarında, ödünç alma denetiminden kaçınmak için

### 1.2 Güvensiz Fonksiyonlar ve Metotlar

Fonksiyonun tamamını `unsafe` olarak işaretlersek, onu çağıranın da `unsafe` blok içinde çağırması zorunlu olur. Böylece tehlikeli işlem yapıldığını çağıran kişi açıkça görür.

```rust
unsafe fn tehlikeli() {
    // burada ham işaretçiyle işlem yapılabilir
}

fn main() {
    unsafe {
        tehlikeli();
    }
}
```

Genellikle `unsafe` fonksiyon gövdesinde bellek güvenliğini biz sağlarız, üzerine güvenli bir soyutlama (safe wrapper) inşa ederiz.

### 1.3 `extern` Fonksiyonlar ile FFI

Rust, başka dillerde yazılmış fonksiyonları çağırabilir. Bunun için `extern` bloğu kullanırız ve bu çağrılar her zaman `unsafe` sayılır, çünkü Rust yabancı kodun güvenliğini garanti edemez.

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("C'ye göre -3'ün mutlak değeri: {}", abs(-3));
    }
}
```

Tersi de mümkündür: Rust fonksiyonlarımızı diğer dillerden çağrılabilir hale getirebiliriz.

```rust
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Bu fonksiyon C'den çağrıldı!");
}
```

### 1.4 Değiştirilebilir `static` Değişkenler

Rust’ta sabitler (`const`) ve değiştirilemez statik değişkenler (`static`) güvenlidir. Ancak `static mut` değiştirilebilir statik değişkenler tanımlarsan, bunlara erişmek (okumak ya da yazmak) `unsafe` olur. Çünkü veri yarışına (data race) yol açabilirler.

```rust
static mut SAYAC: u32 = 0;

fn sayaci_arttir() {
    unsafe {
        SAYAC += 1;
    }
}
```

### 1.5 Güvensiz Trait’ler

Bir trait’in tüm implementasyonlarının belli bellek düzeni ya da güvenlik şartlarını sağlaması gerekiyorsa, o trait’i `unsafe trait` olarak tanımlarız. Implemente ederken de `unsafe impl` kullanılır.

Örnek: `Send` ve `Sync` trait’leri, iş parçacıkları arasında taşınabilirlik ve paylaşılabilirlik garantisi verir; bu yüzden derleyici sadece uygun durumlarda otomatik implemente eder. İhtiyaç halinde biz de sorumluluğu alarak `unsafe impl` ile implemente edebiliriz.

```rust
unsafe trait TehlikeliTrait {
    // ...
}

unsafe impl TehlikeliTrait for i32 {
    // ...
}
```

### 1.6 Union’lar

Rust’ta `union` veri yapısı, aynı bellek alanını farklı türlerle yorumlamak için kullanılır. Alanlarına erişim her zaman `unsafe`’dir, çünkü hangi türün aktif olduğunu bilmek programcının sorumluluğundadır.

```rust
union SayiVeFloat {
    sayi: u32,
    float: f32,
}

let birlik = SayiVeFloat { sayi: 1 };
unsafe {
    println!("float olarak: {}", birlik.float); // tanımsız olabilir!
}
```

---

## 2. Advanced Traits – İleri Düzey Trait’ler

Trait’ler Rust’ın polimorfizm ve kod paylaşım mekanizmasının temelidir. İleri düzey kullanımlar, daha karmaşık senaryoların üstesinden gelmeye yarar.

### 2.1 İlişkilendirilmiş Tipler (Associated Types)

Trait tanımında, implementasyon sırasında belirlenen bir tip yer tutucusu tanımlamak için kullanılır. Standart kütüphanedeki `Iterator` trait’ini düşün: `type Item;` sayesinde her yineleyici, hangi türü döndüreceğini kendi belirler.

```rust
trait Graph {
    type Node;          // ilişkilendirilmiş tip
    fn add_node(&mut self, node: Self::Node);
}

struct MyGraph;

impl Graph for MyGraph {
    type Node = String;
    fn add_node(&mut self, node: String) {
        // ...
    }
}
```

İlişkilendirilmiş tipler, jenerik parametrelerden farklıdır: jenerik ile `Graph<N>` gibi her tür için birden çok implementasyon yapabilirsin, oysa ilişkilendirilmiş tipte her tip için yalnızca tek bir implementasyon olur. Bu, tip sistemini daha temiz ve anlamlı kılar.

### 2.2 Varsayılan Jenerik Tip Parametreleri

Trait tanımında jenerik parametresine varsayılan bir tip atayabiliriz.

```rust
trait Topla<Rhs = Self> {
    type Output;
    fn topla(self, rhs: Rhs) -> Self::Output;
}
```

`Rhs = Self` yazınca, implementasyon yaparken ikinci operand tipini belirtmezsek `Self` kabul edilir. Bu, operatör aşırı yüklemede çok kullanılır.

### 2.3 Tam Yetkili Sözdizimi (Fully Qualified Syntax)

Aynı isimde iki method farklı trait’lerden geldiğinde ya da method ismiyle türün doğrudan bir methodu çakıştığında, hangisini çağıracağımızı belirtmek için tam yetkili sözdizimini kullanırız:

```rust
<Tür as Trait>::fonksiyon(argumanlar);
```

Örnek:

```rust
trait Pilot {
    fn ucur(&self);
}

trait Buyucu {
    fn ucur(&self);
}

struct Insan;

impl Pilot for Insan {
    fn ucur(&self) { println!("Pilot uçuyor"); }
}

impl Buyucu for Insan {
    fn ucur(&self) { println!("Büyücü uçuyor"); }
}

impl Insan {
    fn ucur(&self) { println!("İnsan koşuyor"); }
}

fn main() {
    let kisi = Insan;
    kisi.ucur();                        // Insan::ucur çağrılır
    Pilot::ucur(&kisi);                // Pilot trait'inden
    Buyucu::ucur(&kisi);              // Büyücü trait'inden
    <Insan as Pilot>::ucur(&kisi);    // tam yetkili (aynısı)
}
```

### 2.4 Süper Trait’ler (Supertraits)

Bir trait’in başka bir trait’i gerektirmesini sağlarız. Mesela bir `AnaTrait` tanımlarken `trait AnaTrait: std::fmt::Display` dersek, bu trait’i implemente eden her tipin `Display`’i de implemente etmesi zorunlu olur.

```rust
trait AnaTrait: std::fmt::Display {
    fn bilgi(&self) -> String {
        format!("({})", self) // self.to_string() çağrılabilir
    }
}
```

### 2.5 Newtype Deseni (Sarmalayıcı Tip)

Daha önce “yabancı tip için yabancı trait implemente edemezsin” (orphan rule) demiştik. İşte bu kısıtlamayı aşmak için, bir tipi tek alanlı bir tuple struct içine sararız; böylece yerel bir tip olur ve istediğimiz trait’i implemente edebiliriz.

```rust
struct Sarici(Vec<String>);

impl std::fmt::Display for Sarici {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
```

Bu desen aynı zamanda tip güvenliğini artırmak için de kullanılır (örneğin `Saat(u32)` ile `Dakika(u32)` birbirine karışmasın diye).

---

## 3. Advanced Types – İleri Düzey Tipler

Rust’ın tip sistemi derleme zamanında son derece kuvvetli güvenceler sağlar. İleri düzey tip kullanımları bu gücü daha da artırır.

### 3.1 Tip Takma Adları (Type Aliases)

Uzun veya karmaşık tip isimlerini kısaltmak için `type` anahtar kelimesiyle takma ad oluştururuz.

```rust
type Kilometre = i32;
type Thunk = Box<dyn Fn() + Send + 'static>;
```

Bu tipin yeni bir tip yaratmadığının altını çizelim; sadece mevcut tip için bir kısaltmadır. Yani `Kilometre` ile `i32` tamamen aynı tiptir.

### 3.2 Asla Tipi: `!`

Hiçbir zaman değer döndürmeyen ifadelerin tipidir. Örneğin `panic!()` makrosu, sonsuz döngüler veya `std::process::exit()` çağrıları `!` tipiyle sonlanır.

```rust
fn bar() -> ! {
    panic!("Bu fonksiyon hiçbir zaman normal dönmez");
}
```

Asla tipi, tip çıkarımında çok faydalıdır. Bir `match` kolunda `continue` (ki `!` tipindedir) varsa, diğer kolların ürettiği değerlerle uyumlu olması için zorlama yapmaz. Böylece kod derlenir.

### 3.3 Dinamik Boyutlu Tipler (DST)

Boyutu derleme zamanında bilinmeyen tiplerdir. Örneğin `str` (string slice’ı) dinamik boyutludur; bu yüzden onu her zaman bir referansın arkasında (`&str`, `Box<str>`) kullanırız. `Sized` trait’ini implemente etmeyen her tip DST’dir.

`?Sized` jenerik kısıtı ile, hem boyutu bilinen hem de bilinmeyen türlerle çalışabilen fonksiyonlar yazabiliriz. Varsayılan olarak tüm jenerik tipler `Sized`’dır; `?Sized` ile bu kısıtı gevşetiriz.

```rust
fn generic<T: ?Sized>(t: &T) { /* ... */ }
```

---

## 4. Advanced Functions and Closures – İleri Düzey Fonksiyonlar ve Closure’lar

### 4.1 Fonksiyon İşaretçileri (Function Pointers)

Closure’ların yanı sıra, normal fonksiyonları da argüman olarak geçirebiliriz. Bunun için `fn` tipini kullanırız. `fn` tipli parametre, fonksiyon işaretçisidir.

```rust
fn ekle_bir(x: i32) -> i32 {
    x + 1
}

fn iki_kere_f(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(f(arg))
}

fn main() {
    let cevap = iki_kere_f(ekle_bir, 5);
    println!("{}", cevap); // 7
}
```

Fonksiyon işaretçileri closure’ların üç trait’ini (`Fn`, `FnMut`, `FnOnce`) de implemente ederler; bu yüzden closure bekleyen yerlere fonksiyon işaretçisi gönderebilirsin.

### 4.2 Closure Döndürmek

Closure’lar, derleyici tarafından anonim bir tip ile temsil edilir. Bu tipi doğrudan yazamayız. Bir closure döndürmek için `Box<dyn Fn()>` gibi trait nesnelerini kullanırız.

```rust
fn dondur() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```

Artık bu fonksiyon, heap üzerinde bir closure tahsis edip döndürebilir.

---

## 5. Macros – Makrolar

Makrolar, Rust kodunu yazan koddur; yani **metaprogramlama** yapmamızı sağlar. Tekrar eden kod kalıplarını azaltır, dilin sözdizimini özelleştirir.

Rust’ta iki ana makro türü vardır:
- **Deklaratif makrolar** (`macro_rules!`)
- **Prosedürel makrolar** (derive, attribute-like, function-like)

### 5.1 Deklaratif Makrolar: `macro_rules!`

En çok kullanılan ve tanıması en kolay olan makro türüdür. `match` ifadesine benzer bir eşleşme ile kod üretir.

En basit örnek: Kendi `vec!` makromuzu yazalım.

```rust
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

**Açıklama:**
- `( $( $x:expr ),* )` kısmı desen: virgülle ayrılmış sıfır veya daha fazla ifadeyi yakalar.
- `$( ... )*` tekrar grubu; her `$x` için içteki kodu tekrar eder.
- Makro çağrıldığında, `vec![1,2,3]` yazdığımızda oluşan kod adeta bizim yukarıdaki bloğu yazmışız gibi olur.

Deklaratif makrolar, desen eşleme ve kod şablonlarıyla çalışır; derleme anında açılırlar.

### 5.2 Prosedürel Makrolar

Daha karmaşık kod dönüşümleri için, derleyiciye bir Rust programı gibi çalışan makrolardır. Token akışı (TokenStream) alıp yeni bir token akışı döndürürler. Üç çeşidi vardır:

#### a) `#[derive]` Makroları

Bir struct veya enum’a otomatik olarak trait implemente eden kod üretir.

```rust
use proc_macro::TokenStream;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // input'u işleyip trait implementasyonu üret
}
```

Kullanımı:

```rust
#[derive(HelloMacro)]
struct MyStruct;
```

#### b) Attribute-like Makrolar

Yeni öznitelikler yaratmaya yarar. `#[route(GET, "/")]` gibi.

```rust
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    // öznitelik ve fonksiyon tanımını işleyip yeni kod üret
}
```

#### c) Function-like Makrolar

Fonksiyon çağrısına benzeyen ama esnek sözdizimi olan makrolardır. `sql!` veya `json!` gibi.

```rust
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    // input'u ayrıştır ve kod üret
}
```

Prosedürel makrolar yazmak için ayrı bir `proc-macro` crate’i oluşturmak gerekir; çünkü bunlar derleyici tarafından yüklenen paylaşımlı kütüphanelerdir.

---

## Sonuç: İleri Seviye Özellikler Ne Zaman Kullanılır?

Bu bölümde anlattığım konular, günlük Rust programlamada çoğu zaman ihtiyaç duyulmayan ama ihtiyaç duyulduğunda vazgeçilmez olan araçlardır:

- **Unsafe Rust**, işletim sistemi çekirdeği, gömülü sistemler, C kütüphanesi sarmalayıcıları veya yüksek performanslı veri yapıları yazarken.
- **İleri Trait’ler**, kütüphane tasarımında esnek ve güçlü arayüzler sunarken.
- **İleri Tipler**, tip güvenliğini sonuna kadar kullanıp soyutlama seviyesini artırırken.
- **Fonksiyon işaretçileri ve closure döndürme**, fonksiyonel programlama tekniklerinde.
- **Makrolar**, tekrar eden kodları ortadan kaldırmak, DSL (domain-specific language) oluşturmak veya yeni öznitelikler tanımlamak için.

Her birini öğrendikten sonra, Rust’ın güvenlik felsefesinden ödün vermeden ne kadar derin seviyelere inebileceğini göreceksin. Unutma: büyük güç, büyük sorumluluk getirir. Özellikle `unsafe` kod yazarken, güvenlik garantilerini senin sağlaman gerektiğini aklından çıkarma.

Artık ileri seviye Rust’a bir kapı araladık. Bu konuları projelerinde deneyimleyerek pekiştirmen, tam yetkinlik için çok önemli. Başarılar!