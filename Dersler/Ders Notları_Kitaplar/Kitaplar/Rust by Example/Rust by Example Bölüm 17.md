# Rust Makroları - Kapsamlı Ders

Rust'ın en güçlü özelliklerinden biri olan **makro sistemi**, metaprogramlama yapmanıza olanak tanır. Gelin bu konuyu adım adım, ders anlatır gibi inceleyelim.

---

## 1. Makrolar Nedir?

Makrolar, fonksiyonlara benzer ancak isimlerinin sonunda bir ünlem işareti (`!`) bulunur. Örneğin: `println!`, `vec!`, `assert_eq!`.

### Makrolar vs Fonksiyonlar

```rust
// Bu bir fonksiyon çağrısı
fonksiyon_adi(argüman);

// Bu bir makro çağrısı
makro_adi!(argüman);
```

**Temel fark:** Makrolar derleme zamanında **kaynak koda genişletilir** (expand). Yani derleyici, makroyu gördüğünde onu gerçek Rust koduna dönüştürür ve sonra derler.

### C/C++ Makrolarından Farkı

C ve C++'taki makrolar **string preprocessing** (metin ön işleme) yapar. Bu, beklenmedik öncelik hatalarına yol açabilir:

```c
// C makrosu - tehlikeli!
#define Kare(x) x * x
int sonuc = Kare(3 + 2);  // 3 + 2 * 3 + 2 = 11 (!) Beklenen: 25
```

Rust makroları ise **Abstract Syntax Tree (AST)** üzerinde çalışır. Bu, öncelik hatalarını ortadan kaldırır çünkü makro, sözdizimi ağacı seviyesinde genişletilir.

### Makrolar Neden Faydalıdır?

1. **DRY (Don't Repeat Yourself - Kendini Tekrar Etme):** Farklı tipler için benzer işlevselliği tekrar tekrar yazmak yerine makro kullanabilirsiniz.
2. **Domain-Specific Languages (Etki Alanına Özgü Diller):** Belirli bir amaç için özel söz dizimi tanımlayabilirsiniz.
3. **Variadic Interfaces (Değişken Sayıda Argüman Alan Arayüzler):** `println!` gibi, format dizgisine bağlı olarak değişken sayıda argüman alabilen arayüzler tanımlayabilirsiniz.

---

## 2. Makro Söz Dizimi (Syntax)

Rust'ta makrolar `macro_rules!` makrosu kullanılarak oluşturulur.

### Temel Yapı

```rust
macro_rules! makro_adi {
    // Desen (pattern) => { genişletme (expansion) }
    (desen) => {
        // Genişletilmiş kod
    };
}
```

### Basit Bir Örnek

```rust
macro_rules! selam {
    () => {
        println!("Merhaba Dünya!");
    };
}

fn main() {
    selam!();  // "Merhaba Dünya!" yazdırır
}
```

**Açıklama:**
- `()` boş bir argüman listesi deseni
- `=>` desen eşleştiğinde sağ tarafın genişletileceğini belirtir
- `{ ... }` genişletilecek kodu içerir

### Üç Temel Kavram

Makroları anlamak için üç temel kavramı öğrenmemiz gerekiyor:
1. **Patterns and Designators** (Desenler ve Tanımlayıcılar)
2. **Overloading** (Aşırı Yükleme)
3. **Repetition** (Tekrar)

---

## 3. Designators (Tanımlayıcılar)

Makro argümanları dolar işareti (`$`) ile başlar ve bir **designator** ile tip belirtilir.

### Söz Dizimi

```rust
macro_rules! makro_adi {
    ($arguman_adi:designator_tipi) => {
        // $arguman_adi burada kullanılabilir
    };
}
```

### Örnek

```rust
macro_rules! yazdir {
    ($deger:expr) => {
        println!("Değer: {}", $deger);
    };
}

fn main() {
    yazdir!(42);        // "Değer: 42"
    yazdir!("Merhaba"); // "Değer: Merhaba"
    yazdir!(3.14);      // "Değer: 3.14"
}
```

Burada `$deger` argüman adı, `:expr` ise bunun bir **ifade** (expression) olduğunu belirten designator'dır.

### Yaygın Designator Tipleri

| Designator | Açıklama | Örnek |
|------------|----------|-------|
| `block` | Blok ifadesi | `{ ... }` |
| `expr` | İfade | `2 + 2`, `x`, `foo()` |
| `ident` | Değişken/fonksiyon adı | `degisken`, `fonksiyon_adi` |
| `item` | Öğe (fonksiyon, struct, vb.) | `fn foo() {}` |
| `literal` | Sabit değer | `42`, `"merhaba"`, `3.14` |
| `pat` | Desen | `Some(x)`, `Ok(val)` |
| `path` | Yol | `std::collections::HashMap` |
| `stmt` | İfade (statement) | `let x = 5;` |
| `tt` | Token tree (tek token veya parantez içi grup) | Herhangi bir token |
| `ty` | Tip | `i32`, `String`, `Vec<u8>` |
| `vis` | Görünürlük nitelendiricisi | `pub`, `pub(crate)` |

### Detaylı Örnekler

```rust
// ident kullanımı - değişken ismi alır
macro_rules! degisken_olustur {
    ($isim:ident, $deger:expr) => {
        let $isim = $deger;
    };
}

// ty kullanımı - tip alır
macro_rules! tip_kontrol {
    ($deger:expr, $tip:ty) => {
        let _: $tip = $deger;  // Tip uyumluluğunu kontrol eder
    };
}

// literal kullanımı - sabit değer alır
macro_rules! sabit_topla {
    ($a:literal, $b:literal) => {
        $a + $b
    };
}

fn main() {
    degisken_olustur!(sayi, 42);
    println!("{}", sayi);  // 42
    
    tip_kontrol!(100, i32);
    
    let toplam = sabit_topla!(5, 10);
    println!("{}", toplam);  // 15
}
```

---

## 4. Overloading (Aşırı Yükleme)

Makrolar, farklı argüman kombinasyonlarını kabul etmek için **aşırı yüklenebilir**. Bu açıdan `macro_rules!`, bir `match` bloğu gibi çalışır.

### Nasıl Çalışır?

```rust
macro_rules! makro_adi {
    // İlk desen
    (desen_1) => {
        // Birinci genişletme
    };
    // İkinci desen
    (desen_2) => {
        // İkinci genişletme
    };
    // Üçüncü desen
    (desen_3) => {
        // Üçüncü genişletme
    };
}
```

Derleyici, makro çağrısını sırayla desenlerle eşleştirmeye çalışır ve ilk eşleşen deseni kullanır.

### Örnek: Farklı Argüman Sayıları

```rust
macro_rules! hesapla {
    // Hiç argüman yoksa
    () => {
        println!("Hiç argüman yok");
    };
    
    // Tek argüman varsa
    ($a:expr) => {
        println!("Tek argüman: {}", $a);
    };
    
    // İki argüman varsa
    ($a:expr, $b:expr) => {
        println!("İki argüman: {} ve {}", $a, $b);
    };
    
    // Üç argüman varsa
    ($a:expr, $b:expr, $c:expr) => {
        println!("Üç argüman: {}, {} ve {}", $a, $b, $c);
    };
}

fn main() {
    hesapla!();              // "Hiç argüman yok"
    hesapla!(1);             // "Tek argüman: 1"
    hesapla!(1, 2);          // "İki argüman: 1 ve 2"
    hesapla!(1, 2, 3);       // "Üç argüman: 1, 2 ve 3"
}
```

### Örnek: Farklı Desenler

```rust
macro_rules! matematik {
    // Toplama işlemi
    ($a:expr + $b:expr) => {
        println!("Toplama: {} + {} = {}", $a, $b, $a + $b);
    };
    
    // Çarpma işlemi
    ($a:expr * $b:expr) => {
        println!("Çarpma: {} * {} = {}", $a, $b, $a * $b);
    };
    
    // Hem toplama hem çarpma
    ($a:expr + $b:expr * $c:expr) => {
        println!("Karmaşık: {} + {} * {} = {}", $a, $b, $c, $a + $b * $c);
    };
}

fn main() {
    matematik!(5 + 3);       // "Toplama: 5 + 3 = 8"
    matematik!(4 * 7);       // "Çarpma: 4 * 7 = 28"
    matematik!(2 + 3 * 4);   // "Karmaşık: 2 + 3 * 4 = 14"
}
```

### Örnek: Tip Bazlı Aşırı Yükleme

```rust
macro_rules! yazdir {
    // String için
    ($deger:expr, str) => {
        println!("String: {}", $deger);
    };
    
    // Sayı için
    ($deger:expr, num) => {
        println!("Sayı: {}", $deger);
    };
    
    // Otomatik tespit
    ($deger:expr) => {
        println!("Değer: {}", $deger);
    };
}

fn main() {
    yazdir!("Merhaba", str);  // "String: Merhaba"
    yazdir!(42, num);         // "Sayı: 42"
    yazdir!(3.14);            // "Değer: 3.14"
}
```

**Önemli:** Desenler sırayla kontrol edilir. İlk eşleşen desen kullanılır, bu yüzden daha spesifik desenleri önce koymak genellikle daha iyidir.

---

## 5. Repetition (Tekrar)

Makrolar, argüman listesinde **tekrarı** ifade etmek için `+` ve `*` sembollerini kullanabilir.

### Tekrar Operatörleri

- `+`: Argüman **en az bir kez** tekrar etmelidir (1 veya daha fazla)
- `*`: Argüman **sıfır veya daha fazla kez** tekrar edebilir (0 veya daha fazla)

### Söz Dizimi

```rust
macro_rules! makro_adi {
    // $(...),+ : Bir veya daha fazla, virgülle ayrılmış
    ($($arguman:expr),+) => {
        // Her $arguman için kod üret
    };
    
    // $(...),* : Sıfır veya daha fazla, virgülle ayrılmış
    ($($arguman:expr),*) => {
        // Her $arguman için kod üret
    };
}
```

### Örnek: Değişken Sayıda Argüman

```rust
macro_rules! toplam {
    // Bir veya daha fazla ifade, virgülle ayrılmış
    ($($sayi:expr),+) => {
        {
            let mut t = 0;
            $(
                t += $sayi;  // Her sayı için bu satır genişletilir
            )+
            t
        }
    };
}

fn main() {
    println!("{}", toplam!(1));           // 1
    println!("{}", toplam!(1, 2));        // 3
    println!("{}", toplam!(1, 2, 3));     // 6
    println!("{}", toplam!(1, 2, 3, 4));  // 10
}
```

**Nasıl Çalışır?**
- `$(...),+` deseni, virgülle ayrılmış bir veya daha fazla ifadeyi yakalar
- Genişletme kısmında `$(...)+` kullanıldığında, yakalanan her öğe için kod üretilir
- `$sayi` her yinelemede farklı bir değeri alır

### Örnek: Vec Oluşturma

```rust
macro_rules! vektor {
    // Sıfır veya daha fazla öğe
    ($($eleman:expr),*) => {
        {
            let mut v = Vec::new();
            $(
                v.push($eleman);
            )*
            v
        }
    };
}

fn main() {
    let bos = vektor![];
    let tek = vektor![1];
    let coklu = vektor![1, 2, 3, 4, 5];
    
    println!("{:?}", bos);     // []
    println!("{:?}", tek);     // [1]
    println!("{:?}", coklu);   // [1, 2, 3, 4, 5]
}
```

### Örnek: Ayırıcı ile Tekrar

```rust
macro_rules! yazdir_hepsini {
    // Bir veya daha fazla ifade, virgülle ayrılmış
    ($($deger:expr),+) => {
        $(
            print!("{} ", $deger);
        )+
        println!();
    };
    
    // Noktalı virgülle ayrılmış
    ($($deger:expr);+) => {
        $(
            print!("{} ", $deger);
        )+
        println!("(noktalı virgül ile)");
    };
}

fn main() {
    yazdir_hepsini!(1, 2, 3, 4);      // "1 2 3 4"
    yazdir_hepsini!(5; 6; 7; 8);      // "5 6 7 8 (noktalı virgül ile)"
}
```

### Örnek: Son Elemanda Opsiyonel Noktalı Virgül

```rust
macro_rules! esnek {
    // Son noktalı virgül opsiyonel
    ($($deger:expr),+ $(;)?) => {
        $(
            println!("{}", $deger);
        )+
    };
}

fn main() {
    eslek!(1, 2, 3);     // Noktalı virgül yok - çalışır
    eslek!(1, 2, 3;);    // Noktalı virgül var - çalışır
}
```

---

## 6. DRY (Don't Repeat Yourself)

Makrolar, fonksiyonların ve/veya test paketlerinin ortak kısımlarını çıkararak **DRY kodu** yazmanıza olanak tanır.

### Problem: Kod Tekrarı

```rust
use std::ops::AddAssign;
use std::ops::SubAssign;
use std::ops::MulAssign;

// Her operatör için benzer kodu tekrarlamak zorunda kalmak
impl AddAssign for Vec<i32> {
    fn add_assign(&mut self, other: Self) {
        assert_eq!(self.len(), other.len());
        for (a, b) in self.iter_mut().zip(other.iter()) {
            *a += b;
        }
    }
}

impl SubAssign for Vec<i32> {
    fn sub_assign(&mut self, other: Self) {
        assert_eq!(self.len(), other.len());
        for (a, b) in self.iter_mut().zip(other.iter()) {
            *a -= b;
        }
    }
}

impl MulAssign for Vec<i32> {
    fn mul_assign(&mut self, other: Self) {
        assert_eq!(self.len(), other.len());
        for (a, b) in self.iter_mut().zip(other.iter()) {
            *a *= b;
        }
    }
}
```

### Çözüm: Makro Kullanımı

```rust
macro_rules! impl_op_assign {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait for Vec<i32> {
            fn $method(&mut self, other: Self) {
                assert_eq!(self.len(), other.len());
                for (a, b) in self.iter_mut().zip(other.iter()) {
                    *a $op b;
                }
            }
        }
    };
}

// Tek satırla her operatörü implement et
impl_op_assign!(AddAssign, add_assign, +=);
impl_op_assign!(SubAssign, sub_assign, -=);
impl_op_assign!(MulAssign, mul_assign, *=);
```

### Test Kodunda DRY

Makrolar özellikle test kodlarında tekrar eden kalıpları ortadan kaldırmak için mükemmeldir:

```rust
#[cfg(test)]
mod testler {
    use super::*;
    
    macro_rules! test_op {
        ($test_adi:ident, $op:tt, $beklenen:expr) => {
            #[test]
            fn $test_adi() {
                let mut a = vec![1, 2, 3];
                let b = vec![4, 5, 6];
                a $op b;
                assert_eq!(a, $beklenen);
            }
        };
    }
    
    test_op!(toplama_testi, +=, vec![5, 7, 9]);
    test_op!(cikarma_testi, -=, vec![-3, -3, -3]);
    test_op!(carpma_testi, *=, vec![4, 10, 18]);
}
```

**Çıktı:**
```
running 3 tests
test test::carpma_testi ... ok
test test::toplama_testi ... ok
test test::cikarma_testi ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured
```

### Gerçek Dünya Örneği

```rust
macro_rules! impl_display {
    ($struct_adi:ident, $($alan:ident),+) => {
        impl std::fmt::Display for $struct_adi {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{} {{ ", stringify!($struct_adi))?;
                $(
                    write!(f, "{}: {:?}, ", stringify!($alan), self.$alan)?;
                )+
                write!(f, "}}")
            }
        }
    };
}

struct Kullanici {
    isim: String,
    yas: u32,
    email: String,
}

impl_display!(Kullanici, isim, yas, email);

fn main() {
    let k = Kullanici {
        isim: "Ahmet".to_string(),
        yas: 30,
        email: "ahmet@example.com".to_string(),
    };
    println!("{}", k);
    // Çıktı: Kullanici { isim: "Ahmet", yas: 30, email: "ahmet@example.com", }
}
```

---

## 7. DSL (Domain-Specific Language - Etki Alanına Özgü Dil)

DSL, bir Rust makrosuna gömülü mini bir "dil"dir. Makro sistemi normal Rust yapılarına genişlediği için tamamen geçerli Rust'tır, ancak küçük bir dil gibi görünür. Bu, belirli işlevsellik için özlü veya sezgisel söz dizimi tanımlamanıza olanak tanır.

### Basit Hesap Makinesi DSL'i

```rust
macro_rules! hesapla {
    // İfade al ve sonucu yazdır
    ($sol:expr, $op:tt, $sag:expr) => {
        {
            let sonuc = $sol $op $sag;
            println!("{} {} {} = {}", $sol, stringify!($op), $sag, sonuc);
            sonuc
        }
    };
}

fn main() {
    hesapla!(1 + 2);           // "1 + 2 = 3"
    hesapla!((1 + 2) * (3 / 4)); // "(1 + 2) * (3 / 4) = 0"
}
```

### Daha Karmaşık DSL Örneği

```rust
macro_rules! sql_benzeri {
    // SELECT kolonlar FROM tablo WHERE koşul
    (SELECT $($kolon:ident),+ FROM $tablo:ident WHERE $kosul:expr) => {
        {
            println!("SELECT {} FROM {} WHERE {}", 
                stringify!($($kolon),+),
                stringify!($tablo),
                stringify!($kosul)
            );
            // Burada gerçek SQL sorgusu oluşturulabilir
        }
    };
    
    // SELECT * FROM tablo
    (SELECT * FROM $tablo:ident) => {
        {
            println!("SELECT * FROM {}", stringify!($tablo));
        }
    };
}

fn main() {
    sql_benzeri!(SELECT isim, yas FROM kullanicilar WHERE yas > 18);
    sql_benzeri!(SELECT * FROM urunler);
}
```

### HTML Benzeri DSL

```rust
macro_rules! html {
    // Etiket: içerik
    ($etiket:ident : $icerik:expr) => {
        format!("<{}>{}</{}>", stringify!($etiket), $icerik, stringify!($etiket))
    };
    
    // Etiket(atribut=deger): içerik
    ($etiket:ident ($attr:ident = $val:expr) : $icerik:expr) => {
        format!("<{} {}=\"{}\">{}</{}>", 
            stringify!($etiket),
            stringify!($attr),
            $val,
            $icerik,
            stringify!($etiket)
        )
    };
}

fn main() {
    println!("{}", html!(p : "Merhaba Dünya"));
    // <p>Merhaba Dünya</p>
    
    println!("{}", html!(a (href = "https://example.com") : "Tıkla"));
    // <a href="https://example.com">Tıkla</a>
}
```

### Gerçek Dünya DSL Örnekleri

Rust ekosisteminde birçok popüler kütüphane DSL makroları kullanır:

- **`lazy_static!`**: Statik değişkenleri tembel başlatma ile tanımlar
- **`clap!`**: Komut satırı argümanlarını tanımlamak için DSL sağlar
- **`serde_json::json!`**: JSON yapısını Rust'ta tanımlamak için DSL sunar
- **`vec!`**: Vektörleri başlatmak için özel söz dizimi sağlar

```rust
// serde_json::json! örneği
use serde_json::json;

let veri = json!({
    "isim": "Ahmet",
    "yas": 30,
    "diller": ["Rust", "Python", "JavaScript"]
});
```

### DSL Tasarım İpuçları

1. **Sezgisel Olmalı:** DSL, hedef kitle için doğal ve anlaşılır olmalı
2. **Rust Söz Dizimine Uygun Olmalı:** Rust'ın sözdizimi kurallarını ihlal etmemeli
3. **İyi Hata Mesajları Vermeli:** Derleme hataları açıklayıcı olmalı
4. **Performanslı Olmalı:** Genişletilmiş kod verimli çalışmalı

---

## Özet ve En İyi Uygulamalar

### Makro Yazarken Dikkat Edilmesi Gerekenler

1. **Mümkün Olduğunda Fonksiyon Kullanın:** Makrolar güçlüdür ama debug edilmesi zordur. Eğer bir fonksiyon işinizi görüyorsa, onu kullanın.

2. **`tt` (Token Tree) Kullanın:** `tt` en esnek designator'dır ve neredeyse her şeyi kabul eder. Ancak çok genel olduğu için dikkatli kullanılmalıdır.

3. **Hijacking'den Kaçının:** Makronuzun içindeki değişken isimlerinin, kullanıcının kodundaki değişkenlerle çakışmamasına dikkat edin.

4. **Hygiene (Hijyen):** Rust makroları "hygienic"tir, yani makro içindeki değişkenler dışarıdaki değişkenlerle çakışmaz.

5. **Dokümantasyon:** Makrolarınızı iyi dokümante edin, çünkü IDE'ler makroları fonksiyonlar kadar iyi analiz edemez.

### Ne Zaman Makro Kullanmalı?

✅ **Kullanın:**
- Kod tekrarını önlemek için (özellikle trait implementasyonlarında)
- DSL oluşturmak için
- Değişken sayıda argüman alan arayüzler için
- Derleme zamanında kod üretmek için

❌ **Kullanmayın:**
- Basit bir fonksiyon işinizi görecekse
- Runtime'da değişen davranışlar için
- Çok karmaşık mantık için (bunun yerine procedural macro düşünün)

### Sonraki Adımlar

Bu derste `macro_rules!` makrolarını öğrendik. Rust'ta ayrıca **procedural macros** (prosedürel makrolar) da vardır:
- `#[derive(...)]` makroları
- Attribute makrolar
- Function-like makrolar

Prosedürel makrolar daha karmaşıktır ama daha da güçlüdür. `macro_rules!` ile başlayıp, ihtiyaç duydukça prosedürel makrolara geçmek iyi bir stratejidir.

---

Artık Rust makrolarının temellerini anladınız! Pratik yaparak bu bilgileri pekiştirmenizi öneririm. Küçük makrolarla başlayıp, zamanla daha karmaşık DSL'lere geçebilirsiniz.

# DeepSeek Yorumu ------------------

Rust By Example'ın Macros sayfası, Rust'ın güçlü makro sistemine kapsamlı bir giriş sunar. Bu bölüm, makroların ne olduğunu, nasıl çalıştığını ve neden bu kadar kullanışlı olduklarını adım adım açıklar. İsterseniz bu sayfayı ve alt başlıklarını birlikte inceleyelim.

### 📖 Makrolara Giriş: `macro_rules!`

Sayfa, Rust'ın `macro_rules!` ile tanımlanan makro sistemini tanıtarak başlar. Bu sistem, **metaprogramlama** (program yazan program) yapmamızı sağlar.

*   **Makrolar Ne İşe Yarar?** Makrolar, fonksiyonlara benzerler ancak önemli bir farkları vardır. Bir fonksiyon çağrısı (`foo()`) çalıştırıldığında kod çalıştırılırken, bir makro çağrısı (`foo!()`) **derleme zamanında** çalıştırılır ve yeni bir kaynak kodu üretir. Bu üretilen kod, programın geri kalanıyla birlikte derlenir.

*   **C ve Rust Makroları Arasındaki Fark:** C ve diğer dillerdeki makrolar genellikle **metin bazlı** (string preprocessing) çalışır ve bu, beklenmedik öncelik (precedence) hatalarına yol açabilir. Rust'ın makroları ise **soyut sözdizimi ağaçları** (Abstract Syntax Trees - AST) üzerinde çalışır. Bu sayede makro genişlemesi daha güvenlidir ve hatalara karşı daha az hassastır.

Sayfadaki ilk örnek, oldukça basit bir makroyu gösteriyor:

```rust
// Bu, "say_hello" adında basit bir makrodur.
macro_rules! say_hello {
    // `()` makronun hiçbir argüman almadığını belirtir.
    () => {
        // Makro, bu bloktaki içeriğe genişleyecektir.
        println!("Hello!")
    };
}

fn main() {
    // Bu çağrı, `println!("Hello!")` koduna genişleyecektir.
    say_hello!();
}
```

### 💡 Makrolar Neden Kullanışlıdır?

Sayfa, makroların üç temel kullanım alanını vurguluyor:

1.  **Kendini Tekrar Etme (DRY - Don't Repeat Yourself):** Farklı türler için benzer işlevselliği tekrar tekrar yazmak zorunda kalmamak, makroların en büyük avantajlarından biridir. Örneğin, farklı sayı türleri (`i32`, `f64`) için aynı işlemi yapan bir kod bloğunu makro ile tek bir seferde tanımlayıp kullanabilirsiniz.
2.  **Alan Özel Diller (DSL - Domain-Specific Languages):** Makrolar, belirli bir amaç için özel bir sözdizimi (syntax) tanımlamanıza olanak tanır. Bu, kodunuzu belirli bir alana (örneğin, web uygulaması, veritabanı işlemleri) özel olarak daha okunabilir ve ifade edici hale getirebilir.
3.  **Değişken Sayıda Argüman (Variadic Interfaces):** Bazen bir fonksiyonun veya makronun değişken sayıda argüman almasını isteyebilirsiniz. `println!` makrosu bunun en bilinen örneğidir; format string'ine bağlı olarak herhangi bir sayıda argüman alabilir.

### 🗺️ Sayfanın Alt Başlıkları

Rust By Example'daki makrolar bölümü, bu temel kavramların üzerine inşa edilmiş birkaç önemli alt başlık daha içerir:

*   **Designators (Belirleyiciler):** Makroların argüman olarak ne tür girdiler (ifadeler, türler, tanımlayıcılar vb.) beklediğini belirtmek için kullanılan işaretlerdir. Örneğin, `:expr` bir ifadeyi, `:ty` bir türü, `:ident` ise bir tanımlayıcıyı (değişken/fonksiyon adı gibi) belirtir. Bu, makroların tür güvenliği açısından önemlidir.
*   **Overload (Aşırı Yükleme):** Makrolar, farklı sayıda veya türde argüman alan birden fazla "kolu" (arm) destekleyecek şekilde tanımlanabilir. Bu, aynı makro adının farklı durumlarda farklı şekillerde davranmasını sağlar. Örneğin, bir `vec!` makrosu hiç argüman almadan boş bir vektör oluşturabileceği gibi, birden fazla eleman alarak onlarla dolu bir vektör de oluşturabilir.
*   **Repeat (Tekrarlama):** Makroların en güçlü özelliklerinden biri, belirli kalıpları tekrarlama yeteneğidir. `$(...),*` veya `$(...)+` sözdizimi ile bir kalıbın sıfır veya daha fazla (`*`) veya bir veya daha fazla (`+`) kez tekrarlanacağını belirtebilirsiniz. Bu, `println!` gibi değişken sayıda argüman alan makroların temelini oluşturur.

---

**Özetle:** Rust'ın `macro_rules!` sistemi, derleme zamanında güvenli ve esnek kod üretimi sağlayan güçlü bir metaprogramlama aracıdır. DRY ilkesini uygulamak, alana özel diller oluşturmak ve değişken sayıda argümanla çalışmak için biçilmiş kaftandır. Designators, overload ve repeat gibi kavramlar ise bu sistemi daha da esnek ve ifade edici hale getirir.

Bu konulara daha derinlemesine dalmak veya aklınıza takılan başka bir nokta varsa, lütfen sormaktan çekinmeyin!