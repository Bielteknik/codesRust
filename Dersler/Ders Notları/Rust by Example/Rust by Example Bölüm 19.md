# Rust Standart Kütüphanesi (std) - Kapsamlı Ders Notları

## 📚 Giriş: Standart Kütüphene (std) Nedir?

Rust'ın standart kütüphanesi (`std`), dilin temel taşlarını oluşturan ve primitif türlerin ötesine geçen zengin bir tür koleksiyonu sunar [[1]]. Bu kütüphane, Rust programlamanın belkemiği sayılabilecek şu temel yapıları içerir:

- **Büyüyebilir dizgeler** (`String`): `"merhaba dünya"` gibi metinler
- **Büyüyebilir vektörler** (`Vec`): `[1, 2, 3]` gibi dinamik diziler
- **Opsiyonel türler** (`Option<T>`): Değer varlığı veya yokluğu
- **Hata yönetimi türleri** (`Result<T, E>`): Başarı veya başarısızlık
- **Heap'te ayrılan işaretçiler** (`Box<T>`): Bellek yönetimi

Bu ders notlarında, Rust by Example kaynağındaki her bir konuyu **ders anlatır gibi**, kavramsal temelleri, bellek modeliyle ilişkisi ve pratik kullanım senaryolarıyla birlikte inceleyeceğiz.

---

## 1️⃣ Box<T>: Stack ve Heap Ayrımı

### Kavramsal Temel

Rust'ta tüm değerler **varsayılan olarak stack'te** (yığın bellekte) saklanır [[2]]. Ancak bazı durumlarda veriyi **heap'te** (öbek bellekte) tutmak gerekebilir. İşte `Box<T>` tam bu ihtiyacı karşılar: `Box<T>`, heap üzerinde ayrılmış bir `T` türünden değere işaret eden **akıllı bir işaretçidir** (smart pointer) [[2]].

### Bellek Yönetimi

Bir `Box` kapsam dışına çıktığında (scope sonlandığında):
1. Yıkıcısı (destructor) çağrılır
2. İçindeki nesne imha edilir
3. Heap'teki bellek serbest bırakılır

Bu mekanizma, Rust'ın **ownership (mülkiyet)** sisteminin doğal bir uzantısıdır ve bellek sızıntılarını önler.

### Erişim ve Dolaylama

Kutulanmış değerlere `*` operatörü ile erişilir; bu, bir katman dolaylamayı (indirection) kaldırır [[2]].

### Pratik Örnek

```rust
use std::mem;

#[derive(Debug, Clone, Copy)]
struct Point { x: f64, y: f64 }

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Stack'te ayrılmış değişkenler
    let point: Point = Point { x: 0.0, y: 0.0 };
    let rectangle: Rectangle = Rectangle {
        top_left: Point { x: 0.0, y: 0.0 },
        bottom_right: Point { x: 3.0, y: -4.0 }
    };

    // Heap'te ayrılmış dikdörtgen
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        top_left: Point { x: 0.0, y: 0.0 },
        bottom_right: Point { x: 3.0, y: -4.0 },
    });

    // Çift dolaylama: Kutu içinde kutu
    let box_in_a_box: Box<Box<Point>> = Box::new(Box::new(Point { x: 0.0, y: 0.0 }));

    // Bellek boyutlarını karşılaştıralım
    println!("Point stack'te {} byte kaplar", mem::size_of_val(&point));
    println!("Rectangle stack'te {} byte kaplar", mem::size_of_val(&rectangle));
    
    // Kutu boyutu = işaretçi boyutu (64-bit sistemde 8 byte)
    println!("Kutulanmış point stack'te {} byte kaplar", 
             mem::size_of_val(&boxed_rectangle));
    
    // Kutudaki değeri stack'e kopyala
    let unboxed: Point = *Box::new(Point { x: 1.0, y: 2.0 });
}
```

### Ne Zaman Box Kullanmalıyız?

- **Büyük veri yapılarını** fonksiyonlara aktarırken kopyalama maliyetinden kaçınmak için
- **Özyinelemeli (recursive) veri yapıları** oluştururken (derleyici boyut bilmelidir; `Box` sabit boyut sağlar)
- **Trait object**'ler ile çalışırken (`Box<dyn Trait>`)

---

## 2️⃣ Vec<T>: Dinamik Vektörler

### Kavramsal Temel

Vektörler, **boyutu yeniden değiştirilebilen dizilerdir** [[3]]. Dilimlerin (slices) aksine, derleme zamanında boyutları bilinmez; ancak çalışma zamanında büyüyüp küçülebilirler [[3]].

### Üçlü Temsil

Bir vektör içsel olarak üç parametre ile temsil edilir:
1. **Veriye işaretçi** (pointer to data)
2. **Uzunluk** (length) - şu an kaç öğe var
3. **Kapasite** (capacity) - ne kadar bellek ayrılmış

Kapasite, vektör için ayrılmış bellek miktarını belirtir [[3]]. Vektör, uzunluk kapasiteden küçük olduğu sürece büyüyebilir. Bu eşik aşıldığında, vektör **daha büyük bir kapasiteyle yeniden tahsis edilir** (reallocation) [[3]].

### Pratik Örnek

```rust
fn main() {
    // İteratörden vektör oluşturma
    let collected: Vec<i32> = (0..10).collect();
    println!("(0..10) toplandı: {:?}", collected);

    // vec! makrosu ile başlatma
    let mut xs = vec![1i32, 2, 3];
    println!("Başlangıç vektörü: {:?}", xs);

    // Sonuna eleman ekleme
    xs.push(4);
    println!("Push sonrası: {:?}", xs);

    // Uzunluk bilgisi
    println!("Vektör uzunluğu: {}", xs.len());

    // İndeksleme (0'dan başlar)
    println!("İkinci eleman: {}", xs[1]);

    // Son elemanı çıkar ve döndür
    println!("Pop edilen: {:?}", xs.pop());

    // Vektör üzerinde yineleme
    for x in xs.iter() {
        println!("> {}", x);
    }

    // İndeks numaralı yineleme
    for (i, x) in xs.iter().enumerate() {
        println!("{} pozisyonunda {} değeri", i, x);
    }

    // Değiştirilebilir yineleme ile değerleri güncelle
    for x in xs.iter_mut() {
        *x *= 3;  // Her değeri 3 ile çarp
    }
    println!("Güncellenmiş vektör: {:?}", xs);
}
```

### Önemli Notlar

- **Sınır dışı indeksleme panic ile sonuçlanır** (güvenli bellek erişimi)
- **Değişmez (immutable) vektörler büyüyemez**
- `vec![değer; adet]` sözdizimi ile belirli sayıda aynı değeri içeren vektör oluşturulabilir

---

## 3️⃣ String ve &str: Metin İşleme

### İki Temel Metin Türü

Rust'ta en çok kullanılan iki metin türü vardır [[4]]:

#### `String`
- Byte vektörü (`Vec<u8>`) olarak saklanır
- **Geçerli UTF-8 dizisi** olma garantisi vardır
- **Heap'te ayrılır**, büyüyebilir
- Null-terminated değildir (C-style sonlandırıcı yok)

#### `&str` (String Slice)
- Bir dilim (`&[u8]`) olarak geçerli UTF-8 dizisine işaret eder
- `String`'in görünümü (view) olarak kullanılabilir
- Tıpkı `&[T]`'nin `Vec<T>`'ye görünümü olması gibi

### Pratik Örnek

```rust
fn main() {
    // Salt okunur bellekte ayrılmış string dilimi
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);

    // Kelimeleri tersten yinele (yeni bellek ayrılmaz)
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    // Karakterleri vektöre topla, sırala, tekrarları kaldır
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    // Büyüyebilir String oluştur
    let mut string = String::new();
    for c in chars {
        string.push(c);        // Karakter ekle
        string.push_str(", "); // Dilim ekle
    }

    // Kırpma işlemi (orijinal stringe dilim döner, yeni bellek yok)
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Kullanılan karakterler: {}", trimmed_str);

    // Heap'te String tahsis et
    let alice = String::from("I like dogs");
    // Yeni bellekte değiştirilmiş string sakla
    let bob: String = alice.replace("dog", "cat");
    println!("Alice: {}", alice);
    println!("Bob: {}", bob);
}
```

### String Literalleri ve Kaçış Karakterleri

Rust, özel karakterler içeren string literalleri yazmak için çeşitli yollar sunar [[4]]:

#### Standart Kaçışlar
```rust
fn main() {
    // Hexadecimal byte değerleri
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("{}", byte_escape);  // "I'm writing Rust!"

    // Unicode kod noktaları
    let unicode = "\u{211D}";  // ℝ (gerçek sayılar kümesi)
    println!("Unicode karakter: {}", unicode);

    // Çok satırlı stringler
    let long_string = "String literals
        can span multiple lines.";
}
```

#### Ham (Raw) String Literalleri
Kaçış karakterlerinin çalışmadığı, olduğu gibi yazılan stringler:
```rust
fn main() {
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // Tırnak işareti için # kullan
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 255'e kadar # kullanılabilir
    let longer = r###"String with "# in it"###;
}
```

#### Byte Stringler
UTF-8 olmayan veriler için:
```rust
fn main() {
    // Bu aslında &str değil, &[u8; N]
    let bytestring: &[u8; 21] = b"this is a byte string";
    println!("Byte string: {:?}", bytestring);

    // Byte kaçışları çalışır ama Unicode kaçışları çalışmaz
    let escaped = b"\x52\x75\x73\x74 as bytes";
    
    // Ham byte string
    let raw_bytes = br"\u{211D} is not escaped here";
    
    // UTF-8'e dönüştürme (başarısız olabilir)
    if let Ok(my_str) = std::str::from_utf8(raw_bytes) {
        println!("Metin olarak: '{}'", my_str);
    }
}
```

---

## 4️⃣ Option<T>: Opsiyonel Değerler

### Kavramsal Temel

Bazen bir programın bazı bölümlerinin başarısızlığını `panic!` çağırmak yerine yakalamak isteriz [[5]]. Bu amaçla `Option` enum'u kullanılır.

`Option<T>` enum'unun iki varyantı vardır:
- **`None`**: Başarısızlık veya değer yokluğunu belirtir
- **`Some(value)`**: `T` türünden bir `value` sarmalayan tuple struct

### Pratik Örnek

```rust
// Panic! oluşturmayan tam sayı bölmesi
fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        None  // Başarısızlık None ile temsil edilir
    } else {
        Some(dividend / divisor)  // Sonuç Some ile sarılır
    }
}

// Başarısız olabilecek bölme işlemini yöneten fonksiyon
fn try_division(dividend: i32, divisor: i32) {
    match checked_division(dividend, divisor) {
        None => println!("{} / {} başarısız!", dividend, divisor),
        Some(quotient) => {
            println!("{} / {} = {}", dividend, divisor, quotient)
        },
    }
}

fn main() {
    try_division(4, 2);   // 4 / 2 = 2
    try_division(1, 0);   // 1 / 0 başarısız!

    // None'a bağlanmak tür belirtimi gerektirir
    let none: Option<i32> = None;
    let _equivalent_none = None::<i32>;

    // Some varyantını unwrap ile açma
    let optional_float = Some(0f32);
    println!("{:?} unwrap ile: {:?}", 
             optional_float, optional_float.unwrap());

    // None'ı unwrap etmek panic! oluşturur!
    // println!("{:?} unwrap ile: {:?}", none, none.unwrap());
}
```

### Unwrap ve Güvenli Kullanım

- **`.unwrap()`**: `Some` ise içindeki değeri döndürür, `None` ise **panic!**
- **`.unwrap_or(default)`**: `None` ise varsayılan değer döndürür
- **`.unwrap_or_else(|| default)`**: `None` ise closure çalıştırılır
- **Pattern matching** ile güvenli kullanım tercih edilmelidir

---

## 5️⃣ Result<T, E>: Hata Yönetimi

### Kavramsal Temel

`Option` enum'u başarısızlığı ifade edebilir ama **neden başarısız olduğunu** söyleyemez [[6]]. Bu amaçla `Result` enum'u kullanılır.

`Result<T, E>` enum'unun iki varyantı vardır:
- **`Ok(value)`**: İşlem başarılı, `T` türünden `value` döndürür
- **`Err(why)`**: İşlem başarısız, `E` türünden `why` (neden) döndürür

### Pratik Örnek

```rust
mod checked {
    // Matematiksel hatalar
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    pub type MathResult = Result<f64, MathError>;

    pub fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }

    pub fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    pub fn ln(x: f64) -> MathResult {
        if x <= 0.0 {
            Err(MathError::NonPositiveLogarithm)
        } else {
            Ok(x.ln())
        }
    }
}

// op(x, y) === sqrt(ln(x / y))
fn op(x: f64, y: f64) -> f64 {
    // Üç seviyeli match piramidi!
    match checked::div(x, y) {
        Err(why) => panic!("{:?}", why),
        Ok(ratio) => match checked::ln(ratio) {
            Err(why) => panic!("{:?}", why),
            Ok(ln) => match checked::sqrt(ln) {
                Err(why) => panic!("{:?}", why),
                Ok(sqrt) => sqrt,
            },
        },
    }
}

fn main() {
    println!("{}", op(1.0, 10.0));
}
```

### İç İçe Match Problemi

Yukarıdaki örnekte görüldüğü gibi, `Result`'ları zincirlemek **iç içe match** ifadelerine yol açar ve kod çabuk karmaşıklaşır. Bu sorunun zarif çözümü bir sonraki bölümde...

---

## 6️⃣ `?` Operatörü: Sonuç Zincirleme

### Kavramsal Temel

`match` kullanarak sonuçları zincirlemek oldukça dağınık olabilir; şans eseri `?` operatörü işleri tekrar düzgün hale getirmek için kullanılabilir [[7]].

`?` operatörü, `Result` döndüren bir ifadenin sonuna yerleştirilir ve şu eşdeğerdir:
- **`Err(err)`** dalı → erken `return Err(From::from(err))`
- **`Ok(ok)`** dalı → `ok` ifadesi (değeri açar)

### Pratik Örnek

```rust
mod checked {
    #[derive(Debug)]
    enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    type MathResult = Result<f64, MathError>;

    fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 { Err(MathError::DivisionByZero) }
        else { Ok(x / y) }
    }

    fn sqrt(x: f64) -> MathResult {
        if x < 0.0 { Err(MathError::NegativeSquareRoot) }
        else { Ok(x.sqrt()) }
    }

    fn ln(x: f64) -> MathResult {
        if x <= 0.0 { Err(MathError::NonPositiveLogarithm) }
        else { Ok(x.ln()) }
    }

    // ? operatörü ile temiz zincirleme
    fn op_(x: f64, y: f64) -> MathResult {
        let ratio = div(x, y)?;    // Başarısızsa DivisionByZero döner
        let ln = ln(ratio)?;       // Başarısızsa NonPositiveLogarithm döner
        sqrt(ln)                   // Son sonucu döndür
    }

    pub fn op(x: f64, y: f64) {
        match op_(x, y) {
            Err(why) => panic!("{}", match why {
                MathError::NonPositiveLogarithm 
                    => "negatif sayının logaritması",
                MathError::DivisionByZero 
                    => "sıfıra bölme",
                MathError::NegativeSquareRoot 
                    => "negatif sayının karekökü",
            }),
            Ok(value) => println!("Sonuç: {}", value),
        }
    }
}

fn main() {
    checked::op(1.0, 10.0);
}
```

### `?` Operatörünün Gücü

- Kod okunabilirliğini dramatik şekilde artırır
- Hata yayılımını (error propagation) otomatikleştirir
- `From` trait'i ile hata türü dönüşümü yapar
- Hem `Result` hem de `Option` ile çalışabilir

---

## 7️⃣ `panic!` Makrosu

### Kavramsal Temel

`panic!` makrosu, bir panik oluşturmak ve stack'i sarmak (unwinding) için kullanılır [[8]]. Sarım sırasında runtime, **thread tarafından sahiplenilen tüm kaynakları** nesnelerin yıkıcılarını çağırarak serbest bırakır [[8]].

Tek thread'li programlarda `panic!`, programın panik mesajını raporlamasına ve çıkmasına neden olur [[8]].

### Pratik Örnek

```rust
// Tam sayı bölmesinin yeniden uygulanması
fn division(dividend: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        panic!("sıfıra bölme hatası");
    } else {
        dividend / divisor
    }
}

fn main() {
    // Heap'te ayrılmış tam sayı
    let _x = Box::new(0i32);

    // Bu işlem thread hatası tetikler
    division(3, 0);

    println!("Bu noktaya ulaşılmaz!");
    // _x bu noktada imha edilmeli
}
```

### Bellek Sızıntısı Yoktur

Valgrind ile test edildiğinde görüleceği üzere, `panic!` durumunda bile tüm heap belleği doğru şekilde serbest bırakılır [[8]]:

```
==4401== HEAP SUMMARY:
==4401==     in use at exit: 0 bytes in 0 blocks
==4401==   total heap usage: 18 allocs, 18 frees, 1,648 bytes allocated
==4401== 
==4401== All heap blocks were freed -- no leaks are possible
```

### Ne Zaman `panic!` Kullanmalı?

- **Kurtarılamaz hatalar** için (programın devam etmesi anlamsız)
- **Prototip geliştirme** aşamasında
- **Testlerde** beklenen durumları doğrulamak için
- **Invariant ihlalleri** için (asla olmaması gereken durumlar)

---

## 8️⃣ HashMap: Anahtar-Değer Depolama

### Kavramsal Temel

Vektörler değerleri tamsayı indeksiyle saklarken, `HashMap`'ler değerleri **anahtar (key)** ile saklar [[9]]. HashMap anahtarları boolean, tamsayı, string veya `Eq` ve `Hash` trait'lerini implement eden herhangi bir tür olabilir [[9]].

Vektörler gibi `HashMap`'ler de büyüyebilir, ancak **fazla alan olduğunda kendilerini küçültebilirler** [[9]].

### Oluşturma Yöntemleri

- `HashMap::new()`: Varsayılan başlangıç kapasitesi (önerilen)
- `HashMap::with_capacity(uint)`: Belirli başlangıç kapasitesi

### Pratik Örnek

```rust
use std::collections::HashMap;

fn call(number: &str) -> &str {
    match number {
        "798-1364" => "Üzgünüz, arama tamamlanamadı.",
        "645-7689" => "Merhaba, Bay Awesome'un Pizzacısı. Adım Fred.",
        _ => "Merhaba! Kim tekrar arıyor?"
    }
}

fn main() {
    let mut contacts = HashMap::new();

    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Robert", "956-1745");

    // Referans alır, Option<&V> döndürür
    match contacts.get(&"Daniel") {
        Some(&number) => println!("Daniel aranıyor: {}", call(number)),
        _ => println!("Daniel'ın numarası yok."),
    }

    // insert() None döndürür (yeni değer) veya Some(eski_değer)
    contacts.insert("Daniel", "164-6743");  // Daniel'ın numarasını güncelle

    match contacts.get(&"Ashley") {
        Some(&number) => println!("Ashley aranıyor: {}", call(number)),
        _ => println!("Ashley'nin numarası yok."),
    }

    contacts.remove(&"Ashley");

    // iter() rastgele sırada (&key, &value) çiftleri döndürür
    for (contact, &number) in contacts.iter() {
        println!("{} aranıyor: {}", contact, call(number));
    }
}
```

### Önemli Metotlar

- **`.insert(key, value)`**: Anahtar-değer çifti ekle
- **`.get(&key)`**: `Option<&V>` döndürür
- **`.remove(&key)`**: Anahtarı ve değerini kaldır
- **`.contains_key(&key)`**: Anahtarın varlığını kontrol et
- **`.iter()`**: Tüm çiftler üzerinde yineleme

---

## 9️⃣ HashSet: Kümeler

### Kavramsal Temel

`HashSet`'i, yalnızca anahtarlarla ilgilendiğimiz bir `HashMap` olarak düşünebilirsiniz [[10]]. Aslında `HashSet`, `HashMap`'in etrafında bir sarmalayıcıdır (wrapper) [[10]].

`HashSet`'in benzersiz özelliği, **tekrar eden elemanlar içermemesinin garanti** edilmesidir [[10]]. Eğer `HashSet`'te zaten bulunan bir değer eklerseniz, yeni değer eskisinin yerini alır.

### Küme Operasyonları

Kümelerin 4 temel operasyonu vardır (hepsi iteratör döndürür):
- **`union`**: Her iki kümedeki tüm benzersiz elemanlar
- **`difference`**: İlk kümede olup ikinci kümede olmayanlar
- **`intersection`**: Her iki kümede de olanlar
- **`symmetric_difference`**: Bir kümede veya diğerinde olan ama **her ikisinde birden olmayanlar**

### Pratik Örnek

```rust
use std::collections::HashSet;

fn main() {
    let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = vec![2i32, 3, 4].into_iter().collect();

    assert!(a.insert(4));         // true döner (yeni eklendi)
    assert!(a.contains(&4));      // true

    // Eğer değer zaten varsa insert() false döner
    assert!(b.insert(4), "Değer 4 zaten B kümesinde!");
    
    b.insert(5);

    println!("A: {:?}", a);  // {1, 2, 3, 4}
    println!("B: {:?}", b);  // {2, 3, 4, 5}

    // Birleşim: [1, 2, 3, 4, 5] (sıra rastgele)
    println!("Birleşim: {:?}", 
             a.union(&b).collect::<Vec<&i32>>());

    // Fark: [1]
    println!("Fark: {:?}", 
             a.difference(&b).collect::<Vec<&i32>>());

    // Kesişim: [2, 3, 4]
    println!("Kesişim: {:?}", 
             a.intersection(&b).collect::<Vec<&i32>>());

    // Simetrik Fark: [1, 5]
    println!("Simetrik Fark: {:?}", 
             a.symmetric_difference(&b).collect::<Vec<&i32>>());
}
```

---

## 🔟 Rc<T>: Referans Sayımı (Tek Thread)

### Kavramsal Temel

Çoklu sahiplik gerektiğinde `Rc` (Reference Counting) kullanılabilir [[11]]. `Rc`, `Rc` içine sarılan değerin sahiplerinin sayısı anlamına gelen referans sayısını takip eder [[11]].

### Davranış Kuralları

- Bir `Rc` klonlandığında referans sayısı **1 artar**
- Klonlanmış bir `Rc` kapsam dışına çıktığında referans sayısı **1 azalır**
- Referans sayısı **sıfır olduğunda** (artık sahip yok), hem `Rc` hem de değer imha edilir
- `Rc` klonlamak **derin kopya (deep copy) yapmaz**; sadece sarılan değere başka bir işaretçi oluşturur ve sayacı artırır [[11]]

### Pratik Örnek

```rust
use std::rc::Rc;

fn main() {
    let rc_examples = "Rc örnekleri".to_string();
    {
        println!("--- rc_a oluşturuldu ---");
        
        let rc_a: Rc<String> = Rc::new(rc_examples);
        println!("rc_a Referans Sayısı: {}", Rc::strong_count(&rc_a));
        
        {
            println!("--- rc_a, rc_b'ye klonlandı ---");
            
            let rc_b: Rc<String> = Rc::clone(&rc_a);
            println!("rc_b Referans Sayısı: {}", Rc::strong_count(&rc_b));
            println!("rc_a Referans Sayısı: {}", Rc::strong_count(&rc_a));
            
            // İki Rc, iç değerleri eşitse eşittir
            println!("rc_a ve rc_b eşit mi: {}", rc_a.eq(&rc_b));
            
            // Değerin metotlarını doğrudan kullanabiliriz
            println!("rc_a içindeki değerin uzunluğu: {}", rc_a.len());
            
            println!("--- rc_b kapsam dışına çıktı ---");
        }
        
        println!("rc_a Referans Sayısı (rc_b düştü): {}", 
                 Rc::strong_count(&rc_a));
        
        println!("--- rc_a kapsam dışına çıktı ---");
    }
    
    // Hata! rc_examples zaten rc_a'ya taşındı
    // println!("rc_examples: {}", rc_examples);
}
```

### Önemli Notlar

- `Rc` **tek thread** içindir (thread-safe değildir)
- Çoklu thread için `Arc` kullanılmalıdır
- `Rc` içindeki değer **değişmezdir** (immutable)

---

## 1️⃣1️⃣ Arc<T>: Atomik Referans Sayımı (Thread-Safe)

### Kavramsal Temel

Thread'ler arasında **paylaşımlı sahiplik** gerektiğinde `Arc` (Atomically Reference Counted) kullanılabilir [[12]]. Bu yapı, `Clone` implementasyonu yoluyla belleğin heap'indeki bir değerin konumu için referans işaretçisi oluşturur ve referans sayacını artırır [[12]].

`Arc`, sahipliği thread'ler arasında paylaştığından, bir değere son referans işaretçisi kapsam dışına çıktığında değişken imha edilir [[12]].

### Pratik Örnek

```rust
use std::time::Duration;
use std::sync::Arc;
use std::thread;

fn main() {
    // Değerin belirlendiği yer
    let apple = Arc::new("the same apple");

    for _ in 0..10 {
        // Bellekteki referansa işaretçi, değer belirtimi yok
        let apple = Arc::clone(&apple);
        
        thread::spawn(move || {
            // Arc sayesinde thread'ler aynı heap verisine erişebilir
            println!("{:?}", apple);
        });
    }

    // Tüm Arc örneklerinin yazdırılmasını bekle
    thread::sleep(Duration::from_secs(1));
}
```

### Rc vs Arc Karşılaştırması

| Özellik | Rc | Arc |
|---------|-----|-----|
| Thread-safe | ❌ Hayır | ✅ Evet |
| Performans | Daha hızlı | Atomik işlemler nedeniyle yavaş |
| Kullanım alanı | Tek thread | Çoklu thread |
| Bellek modeli | Tek thread referans sayımı | Atomik referans sayımı |

---

## 🎓 Özet ve En İyi Uygulamalar

### Tür Karar Matrisi

| İhtiyaç | Kullanılacak Tür |
|---------|------------------|
| Sabit boyutlu, stack'te veri | Primitif türler, array |
| Dinamik dizi | `Vec<T>` |
| Metin (sahiplenilen, büyüyebilir) | `String` |
| Metin (görünüm, referans) | `&str` |
| Heap'te büyük veri | `Box<T>` |
| Olabilecek/olamayacak değer | `Option<T>` |
| Başarılı/başarısız işlem | `Result<T, E>` |
| Anahtar-değer eşleme | `HashMap<K, V>` |
| Benzersiz elemanlar kümesi | `HashSet<T>` |
| Tek thread çoklu sahiplik | `Rc<T>` |
| Çoklu thread çoklu sahiplik | `Arc<T>` |

### Hata Yönetimi Stratejisi

1. **Kurtarılabilir hatalar** → `Result<T, E>` ve `?` operatörü
2. **Opsiyonel değerler** → `Option<T>` ve pattern matching
3. **Kurtarılamaz hatalar** → `panic!`
4. **Beklenen durumlar** → `unwrap_or()`, `unwrap_or_else()`

### Bellek Yönetimi İlkeleri

- **Mümkün olduğunda stack tercih edin** (daha hızlı)
- **Büyük veriler için `Box` kullanın** (stack overflow önleme)
- **`Rc`/`Arc` ile paylaşımlı sahiplik** yönetin
- **Scope sonlandığında otomatik temizlik** (RAII prensibi)

---

## 📖 Sonraki Adımlar

Bu kapsamlı ders notları, Rust standart kütüphanesinin temel yapı taşlarını kapsamaktadır. Daha ileri seviye konular için:

- **Iterator'lar ve Closure'lar** (fonksiyonel programlama)
- **Concurrency** (eşzamanlılık: threads, channels, mutex)
- **Unsafe Rust** (düşük seviye bellek kontrolü)
- **Macro sistemi** (meta-programlama)
- **Async/Await** (asenkron programlama)

Rust'ın standart kütüphanesi hakkında daha fazla bilgi için resmi dokümantasyon: https://doc.rust-lang.org/std/

---

*Bu ders notları, Rust by Example kaynağının std bölümü temel alınarak hazırlanmıştır. Tüm kod örnekleri Rust 2021 edition ile uyumludur.*

# DeepSeek Yorumu --------------------

Rust Standart Kütüphanesi (`std`), dilin temel ilkel türlerinin (primitives) çok ötesine geçen zengin bir tür ve işlevsellik koleksiyonu sunar. Bu bölüm, Rust'ın günlük programlamada en sık kullanılan yapı taşlarından bazılarını tanıtmayı amaçlar.

Rust'ta her değer varsayılan olarak **stack** (yığın) üzerinde saklanır. Ancak `std` kütüphanesi, heap (yığın) üzerinde veri saklamak, boyutu dinamik olarak değişen koleksiyonlar oluşturmak ve hata yönetimini güvenli bir şekilde ele almak için ihtiyacımız olan araçları sağlar.

Şimdi, Rust By Example'ın `std` bölümünün alt başlıklarını tek tek inceleyelim.

---

### 1. Box, Stack ve Heap (`Box, stack and heap`)

Rust'ta tüm değerler varsayılan olarak stack üzerinde saklanır. Stack, LIFO (Last In, First Out) prensibiyle çalışan hızlı bir bellek alanıdır. Ancak, boyutu derleme anında bilinmeyen veya çok büyük olan veriler için stack yetersiz kalabilir. İşte bu noktada **Heap** devreye girer.

**`Box<T>`** , Rust'ın en temel heap allocate edilmiş (yığın üzerinde tahsis edilmiş) akıllı işaretçisidir (smart pointer).

*   **Ne işe yarar?** `Box<T>` oluşturduğunuzda, `T` türündeki bir değer stack yerine heap üzerine yerleştirilir. `Box`'un kendisi ise stack üzerinde kalır ve heap'teki bu veriyi işaret eder.
*   **Neden kullanırız?**
    1.  **Boyutu derleme anında bilinmeyen türleri kullanmak:** Örneğin, özyinelemeli (recursive) bir veri yapısı olan bağlı liste (linked list) oluştururken, bir düğümün kendi türünden bir başka düğümü işaret etmesi gerekir. Bu durumda türün boyutu sonsuz olacağından derlenemez. `Box` ile bu sorunu çözeriz.
    2.  **Büyük miktarda veriyi taşımak:** Büyük bir veriyi (örneğin büyük bir dizi) fonksiyonlar arasında taşırken, tüm veriyi kopyalamak yerine sadece `Box`'ı (yani işaretçiyi) taşımak çok daha verimlidir.
    3.  **Bir değerin sahipliğini (ownership) devretmek:** Sahipliği net bir şekilde belirtmek ve bir değerin belirli bir kapsamın (scope) sonunda otomatik olarak temizlenmesini sağlamak için kullanılır.

*   **Bellek Yönetimi:** `Box` kapsam dışına çıktığında, yıkıcısı (destructor) otomatik olarak çağrılır. Bu yıkıcı, heap üzerinde tutulan iç nesneyi yok eder ve ilgili heap bellek alanını serbest bırakır. Bu sayede Rust, bellek sızıntılarını (memory leak) önler.

**Örnek Kullanım:**
```rust
fn main() {
    // 'x' stack üzerinde saklanır
    let x = 5;
    // 'b' heap üzerinde saklanan 5 değerini işaret eden bir Box'tır.
    let b = Box::new(5);
    println!("b = {}", b);
}
```

---

### 2. Vektörler (`Vectors`)

Dizi (array) türü, sabit bir boyuta sahiptir. Oysa **`Vec<T>`** , boyutu dinamik olarak değişebilen, yani büyüyüp küçülebilen bir koleksiyondur.

*   **Temsili:** Bir vektör, aslında 3 kelimeden (word) oluşan bir yapıdır:
    1.  **Veri İşaretçisi (Pointer):** Heap üzerinde vektörün elemanlarının saklandığı yeri gösterir.
    2.  **Uzunluk (Length):** Vektörün o anki eleman sayısıdır.
    3.  **Kapasite (Capacity):** Vektörün, yeniden tahsis (reallocation) yapmadan kaç eleman saklayabileceğidir. Kapasite her zaman uzunluktan büyük veya eşittir.

*   **Nasıl Çalışır?** Vektöre yeni bir eleman eklendiğinde (`push` metoduyla), eğer mevcut kapasite yetersiz kalırsa, vektör tüm verilerini daha büyük bir heap alanına kopyalar (yeniden tahsis). Bu işlem maliyetli olabilir, bu nedenle vektör, eklemeleri verimli hale getirmek için kapasitesini katlanarak artırır.

**Örnek Kullanım:**
```rust
fn main() {
    // Tür açıklaması eklenebilir
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    // Makro ile daha kolay oluşturma
    let v2 = vec![1, 2, 3];
    println!("İlk vektör: {:?}, İkinci vektör: {:?}", v, v2);
}
```

---

### 3. String'ler (`Strings`)

Rust'ta iki ana string türü vardır: `str` ve `String`. Bu bölümde odaklanılan tür, **`String`** türüdür.

*   **`str` (String Slice):** Genellikle `&str` olarak kullanılır ve sabit uzunlukta, değiştirilemez (immutable) bir UTF-8 byte dizisine işaret eder. Bu tür, doğrudan programın ikili dosyasına gömülü string'ler (string literal) için veya mevcut bir `String`'in bir bölümünü göstermek için kullanılır.
*   **`String`:** `Vec<u8>`'e benzer şekilde, heap üzerinde tahsis edilmiş, büyüyebilir, değiştirilebilir (mutable), UTF-8 ile kodlanmış bir string türüdür. `push_str` veya `push` gibi metotlarla içeriği değiştirilebilir.

**Örnek Kullanım:**
```rust
fn main() {
    // String oluşturmanın birkaç yolu
    let mut s = String::new();
    s.push_str("Merhaba");
    s.push('!');

    let s2 = String::from("Dünya");
    let s3 = "Rust".to_string(); // &str'den String'e dönüşüm

    println!("{} {} {}", s, s2, s3);
}
```

---

### 4. Option (`Option`)

Rust'da null (boş) değer yoktur. Bunun yerine, bir değerin **var olma veya olmama** durumunu temsil etmek için **`Option<T>`**  enum'ı kullanılır.

`Option<T>` iki varyanta sahiptir:
*   `None`: Değer yok.
*   `Some(T)`: `T` türünde bir değer var.

Bu, dilin tür sisteminin bir parçasıdır. Bir fonksiyon `Option<T>` döndürüyorsa, çağıran taraf bu iki durumu da ele almak zorundadır. Bu, null referans hatalarını (NullPointerException) tamamen derleme zamanında önler.

**Örnek Kullanım:**
```rust
fn main() {
    let some_number = Some(5);
    let no_number: Option<i32> = None;

    // `unwrap` kullanmak tehlikeli olabilir, eğer None ise program panikler.
    // Güvenli yaklaşım `match` veya `if let` kullanmaktır.
    match some_number {
        Some(i) => println!("Sayı: {}", i),
        None => println!("Sayı yok"),
    }
}
```

---

### 5. Result (`Result`)

Hata yönetimi Rust'ın temel taşlarından biridir. **`Result<T, E>`**  enum'ı, bir işlemin başarılı veya başarısız olabileceği durumları temsil etmek için kullanılır.

`Result<T, E>` iki varyanta sahiptir:
*   `Ok(T)`: İşlem başarılı, `T` türünde bir sonuç döndü.
*   `Err(E)`: İşlem başarısız, `E` türünde bir hata döndü.

Tıpkı `Option` gibi, `Result` da çağıranı her iki durumu da ele almaya zorlar. Bu, hataların göz ardı edilmesini engeller ve programların daha sağlam olmasını sağlar.

---

#### 5.1. ? Operatörü (`?`)

`Result` türleriyle çalışmayı kolaylaştıran önemli bir operatördür.

*   **Ne işe yarar?** Bir `Result` döndüren fonksiyonun çağrısından sonra `?` kullanılırsa:
    *   Eğer sonuç `Ok(T)` ise, `T` değerini döndürür (devam eder).
    *   Eğer sonuç `Err(E)` ise, hatayı otomatik olarak içinde bulunduğu fonksiyondan `return` eder.

Bu operatör, hata yönetimi kodunu çok daha temiz ve okunabilir hale getirir. `?` sadece `Result` döndüren fonksiyonlar içinde kullanılabilir.

**Örnek Kullanım:**
```rust
use std::num::ParseIntError;

fn multiply(first: &str, second: &str) -> Result<i32, ParseIntError> {
    // `?` ile hata varsa otomatik olarak return edilir.
    let first_number = first.parse::<i32>()?;
    let second_number = second.parse::<i32>()?;
    Ok(first_number * second_number)
}

fn main() {
    let result = multiply("10", "2");
    println!("Sonuç: {:?}", result);

    let error_result = multiply("t", "2");
    println!("Hata Sonucu: {:?}", error_result);
}
```

---

### 6. panic! (`panic!`)

Rust'ta geri dönülemez (unrecoverable) bir hata oluştuğunda program panikler. **`panic!`**  makrosu, programı kasıtlı olarak panikletmek ve bir hata mesajı ile sonlandırmak için kullanılır.

Panik genellikle aşağıdaki durumlarda gerçekleşir:
*   Bir dizinin sınırlarının dışına erişmek.
*   `Option` üzerinde `unwrap()` çağrıldığında `None` olması.
*   `Result` üzerinde `unwrap()` çağrıldığında `Err` olması.
*   `assert!` veya `assert_eq!` makrolarının başarısız olması.

`panic!` makrosu, bir yazılımın beklemediği bir duruma geldiğinde ve güvenli bir şekilde devam edemeyeceğinde kullanılır. Panik anında Rust, yığını geri sarar (stack unwinding) ve tüm kaynakları temizlemeye çalışır.

**Örnek Kullanım:**
```rust
fn main() {
    let v = vec![1, 2, 3];
    // Bu satır panikletecektir çünkü indeks 100 geçersiz.
    // v[100];

    panic!("Kritik bir hata oluştu!");
}
```

---

### 7. HashMap (`HashMap`)

**`HashMap<K, V>`** , anahtar-değer (key-value) çiftlerini saklayan bir veri yapısıdır. Her benzersiz anahtar (`K`), bir değere (`V`) eşlenir.

*   **Kullanım Alanları:** Bir sözlük, önbellek (cache) veya herhangi bir eşleştirme gerektiren durumlarda sıklıkla kullanılır.
*   **Özellikler:**
    *   Veriler hash fonksiyonu kullanılarak organize edilir, bu nedenle aramalar, eklemeler ve silmeler ortalama olarak O(1) karmaşıklığındadır (sabit zaman).
    *   `HashMap`'in anahtarları `Eq` (eşitlik) ve `Hash` (hashlenebilir) trait'lerini uygulamalıdır. Rust'ın ilkel türleri ve standart kütüphanedeki çoğu tür bu trait'leri zaten uygular.

**Örnek Kullanım:**
```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // Değer ekleme
    scores.insert(String::from("Mavi"), 10);
    scores.insert(String::from("Sarı"), 50);

    // Değere erişme
    let team_name = String::from("Mavi");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Mavi Takımının Skoru: {}", score);

    // Iterasyon
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
```

---

### Özet

Rust Standart Kütüphanesi (`std`), dilin temelini oluşturan ilkel türlerin üzerine inşa edilmiş, güçlü ve çeşitli bir araç setidir.

*   **`Box<T>`** ile heap üzerinde veri saklayabilir, bellek yönetimini otomatikleştirebiliriz.
*   **`Vec<T>`** dinamik boyutlu diziler, **`String`** ise dinamik metin işleme için vazgeçilmezdir.
*   **`Option<T>`** ve **`Result<T, E>`** , null ve hata yönetimini derleme zamanında güvence altına alarak, programlarımızı çok daha sağlam hale getirir.
*   **`panic!`** geri dönülemez hataları ele alırken, **`HashMap<K, V>`** ise verimli anahtar-değer eşleştirmeleri sağlar.

Bu yapı taşları, Rust ile güvenli, eşzamanlı ve verimli yazılımlar geliştirmenin temelini oluşturur. Bu bölümde öğrendikleriniz, ilerleyen konularda (dosya işlemleri, thread'ler, vs.) sürekli karşınıza çıkacaktır.