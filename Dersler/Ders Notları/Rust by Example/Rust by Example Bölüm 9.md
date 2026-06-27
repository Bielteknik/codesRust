# 🦀 Rust'ta Fonksiyonlar — Kapsamlı Ders Anlatımı

Rust By Example'ın "Functions" bölümü, Rust'taki fonksiyon kavramını dört ana başlıkta ele alır:

1. **Temel Fonksiyonlar** (`fn`)
2. **Metotlar** (Associated Functions & Methods)
3. **Closure'lar** (Kapamalar)
4. **Yüksek Dereceli Fonksiyonlar** (Higher Order Functions)

Hadi tek tek, ders anlatır gibi inceleyelim.

---

## 📘 Bölüm 1: Temel Fonksiyonlar (`fn`)

### 1.1 Fonksiyon Tanımlama Kuralları

Rust'ta fonksiyonlar `fn` anahtar kelimesi ile tanımlanır. C veya Python'dan farklı olarak Rust'ta **her parametrenin tipi açıkça belirtilmek zorundadır** ve dönüş tipi (varsa) `->` oku ile gösterilir.

```rust
// Basit bir fonksiyon
fn merhaba() {
    println!("Merhaba dünya!");
}

// Parametreli ve dönüş tipli fonksiyon
fn topla(a: i32, b: i32) -> i32 {
    a + b   // Noktalı virgül YOK! Bu bir ifade (expression) ve dönüş değeridir.
}
```

### 1.2 Expression vs. Statement Farkı

Bu konu Rust'ta çok kritiktir:

- **Statement (İfade/Deyim):** Bir şey yapar ama değer döndürmez. Örn: `let x = 5;`
- **Expression (İfade):** Bir değer üretir. Örn: `5 + 3`, `{ x + 1 }`

```rust
fn kare(x: i32) -> i32 {
    x * x   // ← Noktalı virgül yoksa, bu fonksiyonun dönüş değeridir.
}

fn kare_noktali(x: i32) -> i32 {
    x * x;  // ← Noktalı virgül varsa, bu bir statement olur ve () döner!
            //    Derleyici hata verir çünkü i32 bekliyor ama () buluyor.
}
```

### 1.3 `return` Anahtar Kelimesi

Fonksiyonun sonundaki expression otomatik olarak döndürülür. Ama erken dönmek istersen `return` kullanabilirsin:

```rust
fn mutlak_deger(x: i32) -> i32 {
    if x < 0 {
        return -x;   // Erken dönüş
    }
    x   // Normal dönüş
}
```

### 1.4 FizzBuzz Örneği — Fonksiyonlarla

```rust
fn fizzbuzz(n: u32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => String::from("FizzBuzz"),
        (0, _) => String::from("Fizz"),
        (_, 0) => String::from("Buzz"),
        _      => n.to_string(),
    }
}

fn fizzbuzz_to(n: u32) {
    for i in 1..=n {
        println!("{}", fizzbuzz(i));
    }
}

fn main() {
    fizzbuzz_to(20);
}
```

### 1.5 `diverge` — Dönmeyen Fonksiyonlar

Bazı fonksiyonlar asla geri dönmez (örneğin `panic!` veya sonsuz döngü). Bunların dönüş tipi "boş tip" anlamına gelen **ünlem işareti `!`** ile belirtilir:

```rust
fn sonsuz_dongu() -> ! {
    loop {
        println!("Sonsuza kadar...");
    }
}

fn panic_yapan() -> ! {
    panic!("Bir şeyler çok kötü gitti!");
}
```

> 🎓 **Not:** `!` tipi "bu fonksiyon asla bir değer döndürmez" demektir. Bu tip, her tipe dönüştürülebildiği için derleyiciye "burada herhangi bir tip bekleyebilirsin" mesajı verir.

### 1.6 Fonksiyon Aşırı Yükleme (Overloading) Yoktur

Rust'ta aynı isimde farklı parametreli fonksiyonlar tanımlayamazsınız. Bunun yerine trait'ler kullanılır.

---

## 📘 Bölüm 2: Metotlar (Methods)

### 2.1 Associated Functions vs. Methods

Rust'ta OOP'deki "class method" ve "instance method" kavramlarının karşılıkları vardır:

| Kavram | Açıklama | Çağrılma Şekli |
|---|---|---|
| **Associated Function** | Bir türe (type) bağlı ama instance gerektirmeyen fonksiyon | `Ture::fonksiyon()` |
| **Method** | Bir türe bağlı ve bir instance üzerinde çağrılan fonksiyon | `ornek.fonksiyon()` |

### 2.2 `impl` Bloğu

Metotlar, `impl` bloğu içinde tanımlanır:

```rust
struct Dikdortgen {
    genislik: f64,
    yukseklik: f64,
}

impl Dikdortgen {
    // Associated function (constructor gibi)
    // self parametresi YOK
    fn yeni(genislik: f64, yukseklik: f64) -> Dikdortgen {
        Dikdortgen { genislik, yukseklik }
    }

    // Method — &self ile instance'a salt okunur erişim
    fn alan(&self) -> f64 {
        self.genislik * self.yukseklik
    }

    // Method — &mut self ile instance'ı değiştirme
    fn olcekile(&mut self, oran: f64) {
        self.genislik *= oran;
        self.yukseklik *= oran;
    }

    // Method — self ile instance'ı sahiplenme (consume)
    fn aciklama(self) -> String {
        format!("{}x{} boyutunda dikdörtgen", self.genislik, self.yukseklik)
    }
}

fn main() {
    let mut d = Dikdortgen::yeni(10.0, 5.0);   // Associated function çağrısı
    println!("Alan: {}", d.alan());              // Method çağrısı
    
    d.olcekile(2.0);
    println!("Yeni alan: {}", d.alan());
    
    println!("{}", d.aciklama());   // d artık kullanılamaz (moved)
    // println!("{}", d.alan());    // ❌ HATA: d taşındı!
}
```

### 2.3 `self`, `&self`, `&mut self` Farkı

Bu üçü Rust'ın **mülkiyet (ownership)** sisteminin metotlardaki yansımasıdır:

```rust
impl T {
    fn sahiplenen(self)        { /* self'i tüketir, artık çağrılan yerde kullanılamaz */ }
    fn salt_okunan(&self)      { /* sadece okur, orijinal değer korunur */ }
    fn degistiren(&mut self)   { /* orijinal değeri değiştirir */ }
}
```

### 2.4 Statik Metotlar

Rust'ta `static` kelimesi yoktur. `self` almayan associated function'lar zaten statik metotlardır:

```rust
impl Dikdortgen {
    fn birim_kare() -> Dikdortgen {
        Dikdortgen::yeni(1.0, 1.0)
    }
}
```

---

## 📘 Bölüm 3: Closure'lar (Kapamalar)

### 3.1 Closure Nedir?

Closure, **çevresindeki değişkenleri yakalayabilen** (capture) anonim bir fonksiyondur. Fonksiyonel programlamanın temel taşlarından biridir.

```rust
fn main() {
    let x = 10;
    
    // Closure tanımlama — || içinde parametreler
    let topla_x = |val| val + x;
    
    println!("{}", topla_x(5));   // 15 (5 + 10)
}
```

### 3.2 Closure Sözdizimi

```rust
// Temel closure
let selamla = || println!("Merhaba!");

// Parametreli closure
let topla = |a: i32, b: i32| a + b;

// Blok gövdeli closure (birden fazla ifade varsa {} zorunlu)
let islem = |x: i32| -> i32 {
    let sonuc = x * 2;
    sonuc + 1
};
```

### 3.3 Closure'ların Özellikleri

| Özellik | Açıklama |
|---|---|
| `||` kullanımı | Parametreler `()` yerine `||` arasında |
| Tip çıkarımı | Parametre ve dönüş tipleri genellikle çıkarılır |
| Tek satırda `{}` opsiyoneldir | `|x| x + 1` geçerlidir |
| Çevre değişkenlerini yakalar | Kapsamındaki değişkenlere erişebilir |

### 3.4 Yakalama Modları (Capturing)

Closure'lar çevresindeki değişkenleri üç şekilde yakalayabilir. Derleyici, closure'ın değişkeni nasıl kullandığına göre otomatik karar verir:

#### a) Referans ile Yakalama (`&T` — Borrow)
Closure değişkeni **sadece okuyorsa** referans alır:

```rust
let renk = String::from("mavi");
let yazdir = || println!("Renk: {}", renk);
yazdir();
println!("{}", renk);   // ✅ Hala kullanılabilir, çünkü sadece borrow edildi
```

#### b) Değişken Referans ile Yakalama (`&mut T` — Mutable Borrow)
Closure değişkeni **değiştiriyorsa** mutable referans alır:

```rust
let mut sayi = 0;
let mut artir = || {
    sayi += 1;
    println!("Sayı: {}", sayi);
};
artir();   // 1
artir();   // 2
```

#### c) Değer ile Yakalama (`T` — Move)
Closure değişkeni **sahipleniyorsa** (örneğin başka bir scope'a taşıyorsa), değeri move eder:

```rust
let isim = String::from("Ahmet");
let tasi = move || {
    println!("İsim: {}", isim);
};
tasi();
// println!("{}", isim);   // ❌ HATA: isim taşındı!
```

> 🎓 **`move` anahtar kelimesi**, closure'ı değişkenleri zorla sahiplenmeye iter. Özellikle thread'ler arası veri gönderirken çok kullanılır.

### 3.5 Closure'ları Fonksiyona Parametre Olarak Geçirme

Closure'lar, fonksiyonlara parametre olarak aktarılabilir. Burada `Fn`, `FnMut`, `FnOnce` trait'leri devreye girer:

| Trait | Anlamı | Closure Çevreyi Nasıl Kullanır? |
|---|---|---|
| `Fn` | Salt okunur referans alır | `&T` (birçok kez çağrılabilir) |
| `FnMut` | Değişken referans alır | `&mut T` (değiştirir) |
| `FnOnce` | Sahiplenir | `T` (sadece bir kez çağrılabilir) |

```rust
fn uygulanmis<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

fn main() {
    let kare = |x| x * x;
    println!("{}", uygulanmis(kare, 5));   // 25
}
```

### 3.6 Closure'lardan Closure Döndürme

Bir fonksiyondan closure döndürmek istediğimizde, dönüş tipini `Box<dyn Fn(...)>` olarak belirtmeliyiz (çünkü closure'ın tipi anonimdir — `impl Fn` de kullanılabilir):

```rust
fn carpici_yap(carpan: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x * carpan)
}

fn main() {
    let uc_ile_carp = carpici_yap(3);
    println!("{}", uc_ile_carp(10));   // 30
}
```

> 🎓 **Neden `move`?** Çünkü `carpici_yap` fonksiyonu bittiğinde `carpan` değişkeni stack'ten kalkacak. Closure'ın bu değeri sahiplenmesi gerekiyor ki hayatta kalsın.

---

## 📘 Bölüm 4: Yüksek Dereceli Fonksiyonlar (Higher Order Functions — HOF)

### 4.1 Tanım

Yüksek dereceli fonksiyon, **bir veya daha fazla fonksiyon alan** ve/veya **dönüş değeri olarak fonksiyon döndüren** fonksiyondur. Rust'ın fonksiyonel programlama tarafını oluşturan temel yapıdır.

### 4.2 Iterator Üzerinde HOF'lar

Rust'ta en yaygın HOF kullanımı iterator'ler üzerindedir:

```rust
fn main() {
    let sayilar = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // map — her elemana dönüşüm uygula
    let kareler: Vec<_> = sayilar.iter().map(|x| x * x).collect();
    println!("{:?}", kareler);

    // filter — koşulu sağlayanları seç
    let ciftler: Vec<_> = sayilar.iter().filter(|x| *x % 2 == 0).collect();
    println!("{:?}", ciftler);

    // fold — birikimli işlem (reduce gibi)
    let toplam = sayilar.iter().fold(0, |acc, x| acc + x);
    println!("Toplam: {}", toplam);

    // any — herhangi biri koşulu sağlıyor mu?
    let var_mi = sayilar.iter().any(|&x| x > 5);
    println!("5'ten büyük var mı? {}", var_mi);

    // find — koşulu sağlayan ilk eleman
    let bulunan = sayilar.iter().find(|&&x| x > 7);
    println!("Bulunan: {:?}", bulunan);   // Some(8)

    // chain — iki iterator'u birleştir
    let a = [1, 2, 3];
    let b = [4, 5, 6];
    let birlesik: Vec<_> = a.iter().chain(b.iter()).collect();
    println!("{:?}", birlesik);

    // zip — iki iterator'u eşleştir
    let isimler = vec!["Ali", "Veli"];
    let yaslar = vec![25, 30];
    let eslesmis: Vec<_> = isimler.iter().zip(yaslar.iter()).collect();
    println!("{:?}", eslesmis);
}
```

### 4.3 `Option` ve `Result` Üzerinde HOF'lar

```rust
fn main() {
    let deger: Option<i32> = Some(42);

    // map — Some içindeki değeri dönüştürür
    let sonuc = deger.map(|x| x * 2);
    println!("{:?}", sonuc);   // Some(84)

    // and_then — flatMap gibi, Option<Option<_>> yerine Option<_> döner
    let sonuc2 = deger.and_then(|x| {
        if x > 10 { Some(x + 1) } else { None }
    });

    // unwrap_or — None durumunda varsayılan değer
    let yok: Option<i32> = None;
    println!("{}", yok.unwrap_or(0));   // 0

    // Result üzerinde de benzer HOF'lar var
    let r: Result<i32, &str> = Ok(10);
    let r2 = r.map(|x| x * 3).unwrap_or(0);
    println!("{}", r2);   // 30
}
```

### 4.4 Kendi HOF'unu Yazmak

```rust
// Bir fonksiyon alan ve onu iki kez çağıran HOF
fn iki_kez<F: Fn()>(f: F) {
    f();
    f();
}

// Hem fonksiyon alan hem de fonksiyon döndüren HOF
fn kompose_edici<F, G>(f: F, g: G) -> impl Fn(i32) -> i32
where
    F: Fn(i32) -> i32,
    G: Fn(i32) -> i32,
{
    move |x| f(g(x))
}

fn main() {
    iki_kez(|| println!("Merhaba!"));

    let kare = |x| x * x;
    let ikiye_katla = |x| x * 2;
    let once_kare_sonra_iki_katla = kompose_edici(ikiye_katla, kare);
    
    println!("{}", once_kare_sonra_iki_katla(3));   // (3²) * 2 = 18
}
```

### 4.5 Lazy Evaluation (Tembel Değerlendirme)

Rust'taki iterator HOF'ları **tembeldir (lazy)**. Yani `collect()` gibi bir tüketici çağrılmadıkça hiçbir işlem yapılmaz:

```rust
fn main() {
    let sonuc = (1..)
        .filter(|x| x % 2 == 0)
        .filter(|x| x % 3 == 0)
        .take(5)
        .collect::<Vec<_>>();
    
    println!("{:?}", sonuc);   // [6, 12, 18, 24, 30]
}
```

> 🎓 Bu, sonsuz iterator'lerle bile çalışabilmenizi sağlar çünkü sadece ihtiyaç duyulan kadar eleman işlenir.

---

## 🎯 Özet Tablosu

| Kavram | Anahtar Kelime / Yapı | Temel Özellik |
|---|---|---|
| **Fonksiyon** | `fn` | Tip belirtilmeli, expression ile dönüş |
| **Metot** | `impl` bloğu, `self`/`&self`/`&mut self` | Instance'a bağlı fonksiyon |
| **Closure** | `\|x\| ...` | Çevre değişkenlerini yakalar, anonim |
| **HOF** | `Fn`, `FnMut`, `FnOnce` trait'leri | Fonksiyon alan/döndüren fonksiyon |
| **Diverging** | `-> !` | Asla dönmeyen fonksiyon |

---

## 💡 Pratik İpuçları

1. **Closure seçiminde:** Mümkün olan en zayıf yakalamayı tercih edin. `&` > `&mut` > `move` şeklinde düşünün.
2. **HOF parametrelerinde:** `Fn` kullanıyorsanız closure'ı birden fazla çağırabilirsiniz. `FnOnce` ise sadece bir kez.
3. **`move` closure:** Thread spawn ederken veya closure'ın ömrü çevresinden uzun olacaksa mutlaka kullanın.
4. **Metotlarda:** Sadece okuyorsanız `&self`, değiştiriyorsanız `&mut self`, tüketiyorsanız `self` kullanın.

---

Başka bir bölüm veya detay istersen (örneğin closure'ların `std` içindeki kullanımları, iterator adaptor'ları vs.) sorabilirsin! 🚀

# DeepSeek Yorumu ---------------

Rust programlama dilinde fonksiyonlar (functions), kodun yapı taşlarıdır. Bu dersimizde, Rust By Example kitabının `fn` bölümünü ve tüm alt başlıklarını, bir öğretici titizliğiyle adım adım inceleyeceğiz. Amacımız, fonksiyonların sadece temellerini değil; metodları, closure’ları, yakalama modellerini, girdi/çıktı olarak kullanımlarını ve ileri düzey konuları (diverge fonksiyonlar, yüksek dereceli fonksiyonlar) kapsamlı biçimde öğrenmek.

---

## 1. Fonksiyonlar (Functions)

Rust’ta `fn` anahtar kelimesiyle fonksiyon tanımlanır. Temel yapı şöyledir:

```rust
fn fonksiyon_adi(parametre: Tip) -> DönüşTipi {
    // gövde
    ifade // veya return ifade;
}
```

- Parametrelerin tipi **zorunludur**, tip çıkarımı yapılmaz.
- Dönüş tipi `->` ile belirtilir; eğer fonksiyon değer döndürmüyorsa, dönüş tipi yazılmaz (aslında boş demet `()` döner).
- Son ifadenin sonuna `;` konulmazsa, o ifade **dönüş değeri** olur. `return` anahtar kelimesi erken çıkış için kullanılır.

Örnek:

```rust
fn topla(a: i32, b: i32) -> i32 {
    a + b   // noktalı virgül yok, bu ifade döndürülür
}

fn selam_ver() {
    println!("Merhaba!");
}
```

Fonksiyon çağırma, diğer dillerdeki gibidir.

---

## 2. İlişkili Fonksiyonlar ve Metodlar (Associated functions & Methods)

Bir `struct` veya `enum` ile ilişkilendirilen fonksiyonlar ikiye ayrılır:

- **İlişkili fonksiyonlar (associated functions):** İlk parametre olarak `self` almayan, genellikle yapıcı (constructor) gibi davranan fonksiyonlardır. `::` ile çağrılırlar. Örnek: `String::new()`.
- **Metodlar (methods):** İlk parametre olarak `self` (veya `&self`, `&mut self`) alan fonksiyonlardır. `.` ile çağrılırlar.

Metodlar `impl` bloğu içinde tanımlanır.

```rust
struct Dikdortgen {
    genislik: u32,
    yukseklik: u32,
}

impl Dikdortgen {
    // İlişkili fonksiyon (self yok)
    fn kare(boyut: u32) -> Dikdortgen {
        Dikdortgen {
            genislik: boyut,
            yukseklik: boyut,
        }
    }

    // Metod (self alır)
    fn alan(&self) -> u32 {
        self.genislik * self.yukseklik
    }

    // Değiştirici metod (&mut self)
    fn olcekle(&mut self, faktor: f64) {
        self.genislik = (self.genislik as f64 * faktor) as u32;
        self.yukseklik = (self.yukseklik as f64 * faktor) as u32;
    }

    // Tüketen metod (self - ownership alır)
    fn yok_et(self) {
        // self'in mülkiyeti alınır, fonksiyon sonunda düşürülür
        println!("Dikdörtgen yok edildi.");
    }
}

fn main() {
    let mut d = Dikdortgen::kare(10);   // ilişkili fonksiyon
    println!("Alan: {}", d.alan());    // metod
    d.olcekle(2.0);
    d.yok_et();
    // d artık kullanılamaz.
}
```

**Not:** Rust'ta `self` kullanımı otomatik referanslama/dereferanslama ile çalışır; `d.alan()` yazarken `&self` metodunu çağırmak için `&d` almaya gerek yoktur.

---

## 3. Closure’lar (Closures)

Closure’lar, anonim fonksiyonlardır ve bulundukları ortamdan değişken yakalayabilirler. Kısa sözdizimi: `|parametreler| { gövde }`. Tek ifadeli gövdelerde süslü parantez zorunlu değildir.

```rust
let iki_kat = |x| x * 2;
println!("{}", iki_kat(5)); // 10
```

Closure’ların tipi derleyici tarafından atanır, ancak isterlerse belirtilebilir:

```rust
let iki_kat: fn(i32) -> i32 = |x| x * 2; // fonksiyon işaretçisi olarak
// veya
let iki_kat = |x| -> i32 { x * 2 };
```

Önemli fark: Closure’lar, normal fonksiyonlardan farklı olarak **ortam yakalama** yeteneğine sahiptir. Hangi değişkenleri nasıl yakaladıkları, derleyici tarafından closure’ın nasıl kullanıldığına göre belirlenir.

---

## 4. Yakalama (Capturing)

Closure’lar, tanımlandıkları kapsamdaki değişkenleri üç şekilde yakalayabilir:

- **Referansla (`&T`)** – Borrowing
- **Değişken referansla (`&mut T`)** – Mutable borrowing
- **Değerle (`T`)** – Ownership (taşıma)

Derleyici, closure’ın gövdesinde değişkenin nasıl kullanıldığına bakarak en az kısıtlayıcı yakalama modunu seçer. Eğer closure sadece değeri okuyorsa, `&T` ile yakalar. Eğer değiştiriyorsa, `&mut T`; eğer closure ortamından daha uzun yaşayacaksa veya `move` anahtar kelimesi kullanılmışsa, ownership alınır (`T`).

Örnek:

```rust
let mut selam = String::from("Merhaba");

// Sadece okuma -> &T yakalama
let yazdir = || println!("{}", selam);
yazdir(); // &selam geçici olarak ödünç alınır

// Değiştirme -> &mut T yakalama
let mut degistir = || selam.push_str(" dünya!");
degistir(); // &mut selam alınır
println!("{}", selam); // selam artık "Merhaba dünya!" olur

// Ownership ile taşıma (move)
let tasi = move || {
    println!("Yakalanan: {}", selam);
    // selam buraya taşındı, dışarıda kullanılamaz
};
tasi();
// println!("{}", selam); // HATA: selam taşınmış
```

`move` anahtar kelimesi, closure’ın ortamdaki tüm değişkenleri ownership ile yakalamasını zorlar. Genellikle closure’lar farklı bir iş parçacığına gönderileceğinde (spawn) kullanılır.

---

## 5. Girdi Parametresi Olarak Closure’lar (As input parameters)

Bir fonksiyona closure geçmek istediğimizde, closure’ın tipini belirtmemiz gerekir. Rust, closure’lar için üç özel trait sağlar ve fonksiyonlar bu trait’leri generic sınır olarak kullanır:

- `Fn` – yalnızca `&self` alır, ortamı değiştirmez, referansla yakalar.
- `FnMut` – `&mut self` alır, ortamı değiştirebilir, mutable referansla yakalar.
- `FnOnce` – `self` alır, ortamı ownership ile yakalar, yalnızca bir kez çağrılabilir.

Hiyerarşi: `Fn` trait’ini uygulayan her tip `FnMut` ve `FnOnce`’u da uygular; `FnMut` uygulayan her tip `FnOnce`’u uygular. Yani `Fn` en güçlü sınırlamadır (en az yetki).

Bir fonksiyonun girdi olarak closure kabul etmesini şöyle yapabiliriz:

```rust
// FnOnce kullanımı (en genel, her closure'u kabul eder)
fn uygula_once<F>(f: F) where F: FnOnce() {
    f();
}

// FnMut ile mutable yakalayan closure
fn uygula_mut<F>(mut f: F) where F: FnMut() {
    f();
    f(); // birden çok kez çağrılabilir
}

// Fn ile sadece referans yakalayan closure
fn uygula<F>(f: F) where F: Fn() {
    f();
    f();
}

fn main() {
    let s = String::from("merhaba");

    // Fn örneği
    let yaz = || println!("{}", s);
    uygula(yaz);

    // FnMut örneği
    let mut say = 0;
    let mut artir = || say += 1;
    uygula_mut(artir);

    // FnOnce örneği
    let tuket = || drop(s);
    uygula_once(tuket);
}
```

Alternatif olarak generic tip parametresi yerine doğrudan `dyn` trait nesnesi kullanılabilir, ancak bu dinamik dağıtıma (dynamic dispatch) neden olur.

---

## 6. Tip Anonimliği (Type anonymity)

Closure’ların her biri **kendine özgü anonim bir tipe** sahiptir. İki closure aynı imzaya sahip olsa da tipleri farklıdır. Bu nedenle bir vektörde farklı closure’ları tutamazsınız (trait nesnesi kullanmadıkça). Anonim tip, derleme zamanında monomorfizasyon ile optimize edilir.

```rust
let f1 = |x| x + 1;
let f2 = |x| x + 1;
// f1 ve f2 farklı tiptedir, birbirine atanamaz.
// let f3: fn(i32) -> i32 = f1; // fn işaretçisi olarak coerce edilebilir (yakalama yoksa)
```

Eğer closure hiçbir değişken yakalamıyorsa, fonksiyon işaretçisine (`fn` tipine) zorlanabilir (coerce). Yakalama varsa bu mümkün değildir. O zaman trait nesnesi (`Box<dyn Fn()>`) kullanılabilir.

---

## 7. Girdi Fonksiyonları (Input functions)

Bazı durumlarda closure yerine normal bir fonksiyon işaretçisi (`fn` tipi) almak isteyebiliriz. Bu, closure’ların aksine ortam yakalamaz, sadece statik fonksiyonları kabul eder. `fn` tipi, `Fn`, `FnMut` ve `FnOnce` trait’lerini otomatik olarak uygular. Dolayısıyla closure bekleyen bir generic parametreye `fn` tipini de geçebiliriz.

```rust
fn topla(a: i32, b: i32) -> i32 { a + b }

fn uygula<F>(f: F, x: i32, y: i32) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    f(x, y)
}

fn main() {
    let sonuc = uygula(topla, 2, 3);
    // closure da olur
    let carp = |a, b| a * b;
    let sonuc2 = uygula(carp, 2, 3);
}
```

Bu esneklik sayesinde API’ler hem normal fonksiyonları hem de closure’ları kabul eder.

---

## 8. Çıktı Olarak Closure’lar (As output parameters)

Bir closure’ı fonksiyondan döndürmek, tip anonimliği nedeniyle zordur. Anonim tipi doğrudan yazamayız. Çözüm, `impl Trait` sözdizimi (Rust 1.26+) veya `Box<dyn Trait>` kullanmaktır.

### 8.1 `impl Trait` ile döndürme

Fonksiyon imzasında `-> impl Fn(i32) -> i32` gibi bir dönüş tipi yazarak, derleyiciye “bu trait’i uygulayan bir tip döndüreceğim ama tipi gizli” demiş oluruz. Fonksiyon gövdesi tek bir somut tip döndürmelidir.

```rust
fn olustur_artirici(sayi: i32) -> impl Fn(i32) -> i32 {
    move |x| x + sayi
}

fn main() {
    let artir_5 = olustur_artirici(5);
    println!("{}", artir_5(10)); // 15
}
```

Burada `move` kullanarak `sayi`’yi closure içine taşıdık, closure `sayi`’yi ownership ile yakalar.

### 8.2 `Box<dyn Trait>` ile döndürme

Eğer farklı closure tipleri döndürmeniz gerekiyorsa, kalıtımsal polimorfizm (trait object) kullanılır. `Box<dyn Fn()>` döndürmek, dinamik dağıtım yapar.

```rust
fn olustur(isaret: bool) -> Box<dyn Fn(i32) -> i32> {
    if isaret {
        Box::new(|x| x + 1)
    } else {
        Box::new(|x| x - 1)
    }
}
```

Bu yöntem, küçük bir performans maliyeti getirir ama esneklik sağlar.

---

## 9. Yüksek Dereceli Fonksiyonlar (Higher Order Functions - HOF)

Rust, fonksiyonel programlamanın temel taşlarından olan yüksek dereceli fonksiyonları destekler. Bunlar, fonksiyonları parametre alan veya fonksiyon döndüren fonksiyonlardır. `Iterator` trait’i üzerinden `map`, `filter`, `fold` gibi metodlar buna örnektir.

```rust
fn kare(x: i32) -> i32 { x * x }

fn main() {
    let sayilar = vec![1, 2, 3, 4];
    let kareler: Vec<i32> = sayilar.iter().map(|&x| kare(x)).collect();
    println!("{:?}", kareler); // [1, 4, 9, 16]

    // Süslü kullanım: zincirleme
    let sonuc: i32 = (1..=10)
        .filter(|&x| x % 2 == 0)
        .map(|x| x * x)
        .sum();
    println!("Çift sayıların kareleri toplamı: {}", sonuc);
}
```

Burada `map` ve `filter`, birer closure alır. Rust’ın sıfır maliyetli soyutlamaları sayesinde bu zincirleme çağrılar derleme zamanında optimize edilerek düşük seviyeli döngülere dönüştürülür.

---

## 10. Diverge Fonksiyonlar (Diverging functions)

Diverge (ıraksayan) fonksiyonlar, asla geri dönmeyen fonksiyonlardır. Dönüş tipi olarak `!` (never tipi) kullanılır. Bu fonksiyonlar ya sonsuza kadar çalışır, ya programı sonlandırır ya da panic ile çıkar.

En bilinen örnek `panic!` makrosudur. Kendi diverge fonksiyonumuzu yazabiliriz:

```rust
fn sonlan() -> ! {
    panic!("Bu fonksiyon asla dönmez!");
}

fn sonsuz_dongu() -> ! {
    loop {
        // sonsuz döngü
    }
}
```

`!` tipi, herhangi bir tipe zorlanabilir (coerce). Bu sayede `match` kollarında `panic!` veya `unimplemented!` kullanılabilir:

```rust
let deger = match bazi_option {
    Some(v) => v,
    None => panic!("Beklenmeyen None!"),
};
```

Diverge fonksiyonlar, tip sistemi tarafından “bir değer üretmez” olarak bilindiği için kodun devamı ölü kod (dead code) kabul edilir ve bir anlam ifade eder.

---

## 11. Gelişmiş: Fonksiyon İşaretçileri ve Trait Nesneleri

Kapanış konusu olarak, `fn` tipi ile `Fn` trait’lerinin farkını pekiştirelim:

| Özellik                    | `fn(i32) -> i32` (fonksiyon işaretçisi) | `Fn(i32) -> i32` (trait)               |
|----------------------------|-----------------------------------------|----------------------------------------|
| Yakalama                   | Yok                                     | Var (referans, mutable ref, ownership) |
| Tip bilgisi                | Somut, bilinen                         | Generic veya `dyn` olabilir            |
| Bellek temsili             | Tek bir pointer boyutunda              | Trait nesnesinde çift pointer (data+vtable) |
| Ne zaman kullanılır?       | C-FFI, geri çağrılar, basit durumlar   | Esnek closure kabulü, fonksiyonel API’ler |

---

Bu dersle birlikte, Rust’ın fonksiyonlar alemini baştan sona taradık. Artık `fn`, metod, closure, yakalama modelleri, girdi/çıktı olarak kullanım, HOF’lar ve diverge fonksiyonlar hakkında derinlemesine bilgi sahibisiniz. Fonksiyonları kodunuzun yapı taşları olarak etkin biçimde kullanabilir, güvenli ve ifade gücü yüksek API’ler tasarlayabilirsiniz.

Bir sonraki aşamada, bu bilgileri `Iterator`, `Option`, `Result` gibi standart kütüphane bileşenleriyle birleştirerek Rust’ın ifade gücünü tam anlamıyla deneyimleyebilirsiniz.