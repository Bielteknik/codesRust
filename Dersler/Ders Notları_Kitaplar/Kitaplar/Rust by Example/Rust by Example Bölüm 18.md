# Rust'ta Hata Yönetimi — Kapsamlı Ders

Rust'ta hata yönetimi, dilin en güçlü ve en karakteristik özelliklerinden biridir. Rust, hataları "görmezden gelmeyi" derleme aşamasında engeller ve sizi hataları açıkça ele almaya zorlar. Bu ders, [Rust By Example](https://doc.rust-lang.org/rust-by-example/error.html) sayfasının tüm alt başlıklarını ders anlatır gibi, detaylı ve örneklerle açıklayacaktır.

---

## 1. Giriş: Hata Yönetimi Nedir?

Hata yönetimi, başarısızlık olasılığını ele alma sürecidir [[1]]. Örneğin bir dosyayı okuyamayıp, o bozuk girdiyi kullanmaya devam etmek açıkça sorunlu olur. Hataları fark edip açıkça yönetmek, programınızı çeşitli tuzaklardan kurtarır.

Rust'ta hataları ele almanın birkaç yolu vardır ve hepsi farklı kullanım durumlarına sahiptir. Temel kural olarak:

| Araç | Ne Zaman Kullanılır? |
|------|----------------------|
| `panic!` | Testler ve **kurtarılamaz** hatalar için. Prototip aşamasında da kullanılabilir. |
| `Option<T>` | Bir değerin **opsiyonel** olduğu durumlar. Değerin yokluğu bir hata değilse. |
| `Result<T, E>` | Bir şeylerin ters gitme ihtimali varsa ve çağırıcının bu sorunla başa çıkması gerekiyorsa. |

> **Not:** `unwrap()` ve `expect()` prototipleme ve testlerde kullanılabilir, ancak üretim kodunda bunlardan kaçınılmalıdır.

---

## 2. `panic!` Makrosu

Göreceğimiz en basit hata yönetimi mekanizması `panic!`'dir [[16]]. Bir hata mesajı yazdırır, yığını (stack) geri sarar (unwind) ve genellikle programı sonlandırır.

```rust
fn drink(beverage: &str) {
    // Çok fazla şekerli içecek içmemelisin.
    if beverage == "lemonade" {
        panic!("AAAaaaaa!!!!");
    }
    println!("Some refreshing {} is all I need.", beverage);
}

fn main() {
    drink("water");       // ✅ Çalışır
    drink("lemonade");    // 💥 PANIC! Program burada durur
    drink("still water"); // ❌ Bu satıra hiç ulaşılmaz
}
```

**Çıktı:**
```
Some refreshing water is all I need.
thread 'main' panicked at 'AAAaaaaa!!!!', src/main.rs:4
```

### `panic!` Ne Zaman Kullanılmalı?

- **Kurtarılamaz hatalar:** Programın devam etmesinin imkansız olduğu durumlar (örn. bellek yetersizliği, kritik bir dosyanın bulunamaması)
- **Testlerde:** Testin bilerek başarısız olması istendiğinde
- **Prototipleme:** Henüz implemente edilmemiş fonksiyonlar için (ancak `unimplemented!` daha açıklayıcıdır)

---

## 3. `abort` ve `unwind` Stratejileri

Bir `panic` oluştuğunda Rust iki farklı strateji kullanabilir [[19]]:

| Strateji | Açıklama |
|----------|----------|
| **`unwind`** | Yığını geri sarar, her fonksiyondaki verileri temizler, destructor'ları çağırır. Varsayılan davranış. |
| **`abort`** | Programı anında sonlandırır. Temizlik yapmaz, daha hızlıdır. |

Bu stratejileri derleme zamanında koşullu olarak kullanabilirsiniz:

```rust
fn drink(beverage: &str) {
    if beverage == "lemonade" {
        if cfg!(panic = "abort") {
            println!("This is not your party. Run!!!!");
        } else {
            println!("Spit it out!!!!");
        }
    } else {
        println!("Some refreshing {} is all I need.", beverage);
    }
}

fn main() {
    drink("water");
    drink("lemonade");
}
```

Derleme sırasında strateji belirlenebilir:
```console
rustc lemonade.rs -C panic=abort
```

Veya `Cargo.toml` dosyasında:
```toml
[profile.dev]
panic = "abort"
```

> **Ne zaman `abort` kullanmalı?** Gömülü sistemlerde, WASM'de veya ikili dosya boyutunu küçültmek istediğinizde.

---

## 4. `Option<T>` ve `unwrap`

Bir değerin **mevcut olup olmaması** söz konusuysa, `Option<T>` enum'u kullanılır [[14]]. Bu, "değer yokluğu" durumunu temsil eder ve bir hata değildir.

`Option<T>` iki varyanta sahiptir:
- `Some(T)` — Bir `T` değeri bulundu
- `None` — Değer bulunamadı

```rust
// Yetişkin her içeceği idare edebilir.
// Tüm içecekler `match` ile açıkça ele alınıyor.
fn give_adult(drink: Option<&str>) {
    match drink {
        Some("lemonade") => println!("Yuck! Too sugary."),
        Some(inner)      => println!("{}? How nice.", inner),
        None             => println!("No drink? Oh well."),
    }
}

// Başkaları şekerli içecek içmeden önce panic yapar.
// Tüm içecekler `unwrap` ile dolaylı olarak ele alınıyor.
fn drink(drink: Option<&str>) {
    // `unwrap`, `None` aldığında `panic!` oluşturur.
    let inside = drink.unwrap();
    if inside == "lemonade" {
        panic!("AAAaaaaa!!!!");
    }
    println!("I love {}s!!!!!", inside);
}

fn main() {
    let water    = Some("water");
    let lemonade = Some("lemonade");
    let void     = None;

    give_adult(water);     // "water? How nice."
    give_adult(lemonade);  // "Yuck! Too sugary."
    give_adult(void);      // "No drink? Oh well."

    let coffee  = Some("coffee");
    let nothing = None;

    drink(coffee);   // "I love coffees!!!!!"
    drink(nothing);  // 💥 PANIC!
}
```

### `unwrap()` vs `expect()`

| Metod | Davranış |
|-------|----------|
| `unwrap()` | `Some(v)` → `v` döner. `None` → **panic** (genel mesaj) |
| `expect("mesaj")` | `Some(v)` → `v` döner. `None` → **panic** (özel mesajınızla) |

```rust
let x: Option<i32> = None;
x.unwrap();                      // panic: called `Option::unwrap()` on a `None` value
x.expect("Değer kesinlikle olmalıydı!"); // panic: Değer kesinlikle olmalıydı!
```

> **İpucu:** Her zaman `expect()` kullanın; hata mesajı debug sürecinde çok işinize yarar.

---

## 5. Option ile `map` Kombinörü

`match` kullanmak geçerli bir yöntemdir, ancak tekrar tekrar yazmak yorucu olabilir. `map()` kombinörü, kontrol akışını modüler bir şekilde yönetmenizi sağlar [[30]].

`map()`, `Some → Some` ve `None → None` eşleştirmesi yapar.

```rust
#![allow(dead_code)]

#[derive(Debug)]
enum Food { Apple, Carrot, Potato }

#[derive(Debug)]
struct Peeled(Food);
#[derive(Debug)]
struct Chopped(Food);
#[derive(Debug)]
struct Cooked(Food);

// Soyma. Eğer yiyecek yoksa `None` döner.
fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None       => None,
    }
}

// Doğrama. Yine `match` kullanıyoruz.
fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        Some(Peeled(food)) => Some(Chopped(food)),
        None               => None,
    }
}

// Pişirme. Burada `match` yerine `map()` kullanıyoruz!
fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food)| Cooked(food))
}

// Hepsini tek seferde yapalım — `map()` zinciri!
fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

fn eat(food: Option<Cooked>) {
    match food {
        Some(food) => println!("Mmm. I love {:?}", food),
        None       => println!("Oh no! It wasn't edible."),
    }
}

fn main() {
    let apple  = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;

    let cooked_apple  = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));
    let cooked_potato = process(potato);  // Daha temiz görünüm!

    eat(cooked_apple);   // "Mmm. I love Cooked(Apple)"
    eat(cooked_carrot);  // "Mmm. I love Cooked(Carrot)"
    eat(cooked_potato);  // "Oh no! It wasn't edible."
}
```

**`map()` nasıl çalışır?**
```
Some(x) → Some(f(x))
None    → None
```

Yani zincirin herhangi bir yerinde `None` varsa, tüm zincir `None` olarak sonuçlanır. Bu, "kısadevre" (short-circuit) davranışıdır.

---

## 6. Option ile `and_then` Kombinörü

`map()` kullanırken dikkat edilmesi gereken bir nokta vardır: Eğer `map`'e verdiğiniz fonksiyon kendisi bir `Option` döndürürse, sonuç `Option<Option<T>>` olur — iç içe geçmiş bir yapı!

İşte burada `and_then()` (bazı dillerde `flatmap` olarak bilinir) devreye girer [[31]].

```rust
#![allow(dead_code)]

#[derive(Debug)]
enum Food { CordonBleu, Steak, Sushi }
#[derive(Debug)]
enum Day { Monday, Tuesday, Wednesday }

// Sushi için malzememiz yok.
fn have_ingredients(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        _           => Some(food),
    }
}

// Cordon Bleu'nün tarifi yok.
fn have_recipe(food: Food) -> Option<Food> {
    match food {
        Food::CordonBleu => None,
        _                => Some(food),
    }
}

// ❌ match ile uzun yol:
fn cookable_v1(food: Food) -> Option<Food> {
    match have_recipe(food) {
        None        => None,
        Some(food)  => have_ingredients(food),
    }
}

// ✅ and_then ile kısa yol:
fn cookable_v3(food: Food) -> Option<Food> {
    have_recipe(food).and_then(have_ingredients)
}

// ⚠️ map + flatten ile alternatif:
fn cookable_v2(food: Food) -> Option<Food> {
    have_recipe(food).map(have_ingredients).flatten()
}

fn eat(food: Food, day: Day) {
    match cookable_v3(food) {
        Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
        None       => println!("Oh no. We don't get to eat on {:?}?", day),
    }
}

fn main() {
    let (cordon_bleu, steak, sushi) =
        (Food::CordonBleu, Food::Steak, Food::Sushi);

    eat(cordon_bleu, Day::Monday);    // Oh no. (tarif yok)
    eat(steak,       Day::Tuesday);   // Yay! (hem tarif hem malzeme var)
    eat(sushi,       Day::Wednesday); // Oh no. (malzeme yok)
}
```

### `map` vs `and_then` Karşılaştırması

| Metod | Fonksiyon İmzası | Sonuç |
|-------|-------------------|-------|
| `map(f)` | `f: T → U` | `Option<U>` |
| `and_then(f)` | `f: T → Option<U>` | `Option<U>` (düzleştirilmiş) |

**Kural:** Eğer fonksiyonunuz `Option` döndürüyorsa `and_then`, aksi halde `map` kullanın.

---

## 7. Option ile `?` Operatörü

`match` kullanarak `Option` açabilirsiniz, ancak `?` operatörü çok daha kolaydır [[38]]. Eğer `x` bir `Option` ise:
- `x?` → `x` `Some` ise içindeki değeri döner
- `x` `None` ise fonksiyonu sonlandırır ve `None` döndürür

```rust
fn next_birthday(current_age: Option<u8>) -> Option<String> {
    // `current_age` None ise, bu satır None döndürür (fonksiyon biter).
    // `current_age` Some ise, içindeki u8 değeri + 1 yapılır.
    let next_age: u8 = current_age? + 1;
    Some(format!("Next year I will be {}", next_age))
}
```

### Zincirleme `?` Kullanımı

`?` operatörünün gerçek gücü, iç içe yapıları düzleştirmesinde ortaya çıkar:

```rust
struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u16>,
    number: u32,
}

impl Person {
    // Kişinin iş telefonunun alan kodunu al (varsa).
    fn work_phone_area_code(&self) -> Option<u16> {
        // ? operatörü olmadan bu, birçok iç içe match gerektirirdi!
        self.job?.phone_number?.area_code
    }
}

fn main() {
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 4392222,
            }),
        }),
    };

    assert_eq!(p.work_phone_area_code(), Some(61));
}
```

**`?` olmadan bu kod nasıl görünürdü?**
```rust
fn work_phone_area_code(&self) -> Option<u16> {
    match self.job {
        Some(job) => match job.phone_number {
            Some(phone) => phone.area_code,
            None => None,
        },
        None => None,
    }
}
```

Gördüğünüz gibi `?` operatörü kodu inanılmaz derecede kısaltıyor!

---

## 8. `Result<T, E>`

`Result`, `Option`'ın daha zengin bir versiyonudur — olası bir **hatayı** tanımlar, olası bir **yokluğu** değil [[5]].

İki varyantı vardır:
- `Ok(T)` — Bir `T` elemanı bulundu (başarılı)
- `Err(E)` — Bir `E` hatası bulundu (başarısız)

Genel kural: Beklenen sonuç `Ok`, beklenmeyen sonuç `Err`'dir.

```rust
fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    // unwrap() kullanalım. Bizi ısırır mı?
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number * second_number
}

fn main() {
    let twenty = multiply("10", "2");
    println!("double is {}", twenty);   // ✅ "double is 20"

    let tt = multiply("t", "2");        // 💥 PANIC!
    println!("double is {}", tt);
}
```

`"t"` bir sayıya ayrıştırılamaz, bu yüzden `parse()` bir `Err` döndürür ve `unwrap()` panic yapar.

### `main` Fonksiyonunda `Result` Kullanımı

`main` fonksiyonu `Result` döndürebilir! Bir hata oluşursa, hata kodu döndürülür ve hatanın debug temsili yazdırılır [[8]].

```rust
use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(number)  => number,
        Err(e)      => return Err(e),
    };
    println!("{}", number);
    Ok(())
}
```

---

## 9. `Result` ile `map`

`Result` üzerinde de `map` kullanabiliriz. Ancak `Option`'ın aksine, `Err` durumunda hata olduğu gibi iletilir.

### Uzun Yol (Nested `match`)

```rust
use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    match first_number_str.parse::<i32>() {
        Ok(first_number) => {
            match second_number_str.parse::<i32>() {
                Ok(second_number) => Ok(first_number * second_number),
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let twenty = multiply("10", "2");
    print(twenty);  // "n is 20"

    let tt = multiply("t", "2");
    print(tt);      // "Error: invalid digit found in string"
}
```

### Kısa Yol (Kombinatörler)

```rust
use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str.parse::<i32>()
        .and_then(|first_number| {
            second_number_str.parse::<i32>()
                .map(|second_number| first_number * second_number)
        })
}
```

### `Result` ile Kullanılan Önemli Metodlar

| Metod | Açıklama |
|-------|----------|
| `map(f)` | `Ok(x)` → `Ok(f(x))`, `Err(e)` → `Err(e)` |
| `and_then(f)` | `Ok(x)` → `f(x)` (ki bu da Result döndürür), `Err(e)` → `Err(e)` |
| `map_err(f)` | `Ok(x)` → `Ok(x)`, `Err(e)` → `Err(f(e))` |
| `or_else(f)` | `Ok(x)` → `Ok(x)`, `Err(e)` → `f(e)` |

---

## 10. `Result` ile `?` Operatörü

Bazen `unwrap`'ın sadeliğini isteriz ama `panic` riski olmadan. İşte `?` tam da bu amaçla vardır [[3]].

`?` operatörü neredeyse `unwrap` ile aynıdır, ancak `Err` durumunda **panic yapmak yerine** fonksiyondan **return** eder.

```rust
use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = first_number_str.parse::<i32>()?;   // Hata varsa return Err(...)
    let second_number = second_number_str.parse::<i32>()?; // Hata varsa return Err(...)
    Ok(first_number * second_number)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));  // "n is 20"
    print(multiply("t", "2"));   // "Error: invalid digit found in string"
}
```

### `?` Operatörünün Gerçek Anlamı

`?` aslında tam olarak şunun kısaltmasıdır:

```rust
// x? ifadesi şuna eşdeğerdir:
match x {
    Ok(val)  => val,
    Err(err) => return Err(From::from(err)),
    //                     ^^^^^^^^^^
    //                     Otomatik tip dönüşümü!
}
```

Yani `?`, hatayı `From::from` kullanarak dönüşüm tipine otomatik olarak çevirir!

### Eski `try!` Makrosu

`?` operatöründen önce aynı işlev `try!` makrosu ile sağlanırdı. Artık `?` tercih edilmektedir, ancak eski kodlarda `try!` görebilirsiniz:

```rust
// Eski yol (artık önerilmiyor)
let first_number = try!(first_number_str.parse::<i32>());

// Yeni yol (önerilen)
let first_number = first_number_str.parse::<i32>()?;
```

---

## 11. Birden Fazla Hata Tipi

Önceki örneklerde her şey çok uygundu: `Result`'lar `Result`'larla, `Option`'lar `Option`'larla etkileşime giriyordu. Ancak bazen bir `Option` ile bir `Result`'ın etkileşime girmesi gerekir [[2]].

```rust
fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap();          // Hata Tipi 1: Option (vektör boşsa)
    2 * first.parse::<i32>().unwrap()          // Hata Tipi 2: Result (parse edilemezse)
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty   = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("{}", double_first(numbers));  // 84
    println!("{}", double_first(empty));    // 💥 panic: empty vector
    println!("{}", double_first(strings));  // 💥 panic: parse error
}
```

Burada iki farklı hata türü var. Bunları nasıl yöneteceğiz?

---

## 12. Hataları `Box`'lama (`Box<dyn Error>`)

Farklı hata tiplerini tek bir tipe sormanın basit bir yolu, onları `Box<dyn Error>` olarak "kutulamaktır" [[4]]. Dezavantajı, temel hata tipinin yalnızca çalışma zamanında bilinmesidir (statik değil).

```rust
use std::error;
use std::fmt;

// Result alias'ımızı Box<dyn Error> kullanacak şekilde değiştiriyoruz.
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        .ok_or_else(|| EmptyVec.into())  // Into trait ile Box'a çevrilir
        .and_then(|s| {
            s.parse::<i32>()
                .map_err(From::from)     // From::from ile Box'a çevrilir
                .map(|i| 2 * i)
        })
}

fn print(result: Result<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty   = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));  // "The first doubled is 84"
    print(double_first(empty));    // "Error: invalid first item to double"
    print(double_first(strings));  // "Error: invalid digit found in string"
}
```

### `?` ile Daha Temiz Hale Getirme

`Box<dyn Error>` kullandığımızda, `?` operatörü `From::from` sayesinde hataları otomatik olarak dönüştürür:

```rust
fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(EmptyVec)?;  // Otomatik Box'a çevrilir
    let parsed = first.parse::<i32>()?;         // Otomatik Box'a çevrilir
    Ok(2 * parsed)
}
```

Artık `map_err` çağrılarına gerek yok!

---

## 13. Kendi Hata Tipini Tanımlama

Bazen tüm farklı hataları tek bir tipe dönüştürmek kodu basitleştirir. Rust kendi hata tiplerimizi tanımlamamıza izin verir [[4]].

İyi bir hata tipi:
- Farklı hataları aynı tip altında temsil eder
- Kullanıcıya güzel hata mesajları sunar
- Diğer tiplerle kolay karşılaştırılabilir
- Hata hakkında bilgi taşıyabilir
- Diğer hatalarla iyi bir şekilde birleşir

```rust
use std::fmt;

type Result<T> = std::result::Result<T, DoubleError>;

#[derive(Debug, Clone)]
struct DoubleError;

// Hatanın gösterimi, hata tipinden tamamen ayrı.
impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        .ok_or(DoubleError)              // Hata tipimizi kullan
        .and_then(|s| {
            s.parse::<i32>()
                .map_err(|_| DoubleError)  // Parse hatasını da DoubleError'a çevir
                .map(|i| 2 * i)
        })
}
```

**Dezavantaj:** Hata hakkında bilgi taşımıyoruz. Hangi string'in parse edilemediğini bilmiyoruz.

---

## 14. Hataları Sarmalama (Wrapping Errors)

Hataları kutulamanın bir alternatifi, onları kendi hata tipimiz içinde **sarmalamaktır**. Bu yaklaşım daha fazla boilerplate gerektirir ama daha güçlüdür:

```rust
use std::error;
use std::error::Error;
use std::num::ParseIntError;
use std::fmt;

type Result<T> = std::result::Result<T, DoubleError>;

#[derive(Debug)]
enum DoubleError {
    EmptyVec,
    // Parse hatasını içine gömüyoruz (sarmalıyoruz).
    Parse(ParseIntError),
}

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DoubleError::EmptyVec =>
                write!(f, "please use a vector with at least one element"),
            DoubleError::Parse(..) =>
                write!(f, "the provided string could not be parsed as int"),
        }
    }
}

impl error::Error for DoubleError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            DoubleError::EmptyVec => None,
            // Altta yatan hataya erişim sağlar
            DoubleError::Parse(ref e) => Some(e),
        }
    }
}

// From trait'i sayesinde ? operatörü otomatik dönüşüm yapabilir
impl From<ParseIntError> for DoubleError {
    fn from(err: ParseIntError) -> DoubleError {
        DoubleError::Parse(err)
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(DoubleError::EmptyVec)?;
    // Burada From trait'i otomatik olarak çalışır!
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => {
            println!("Error: {}", e);
            if let Some(source) = e.source() {
                println!("  Caused by: {}", source);
            }
        }
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty   = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    // "The first doubled is 84"

    print(double_first(empty));
    // "Error: please use a vector with at least one element"

    print(double_first(strings));
    // "Error: the provided string could not be parsed as int"
    // "  Caused by: invalid digit found in string"
}
```

### Bu Yaklaşımın Avantajları

1. **Tip güvenliği:** Tüm hata tipleri derleme zamanında bilinir
2. **Zengin bilgi:** `source()` ile orijinal hataya erişilebilir
3. **Ergonomik:** `From` trait'i sayesinde `?` operatörü otomatik dönüşüm yapar
4. **Özelleştirilebilir:** Her hata varyantı için özel mesajlar

---

## 15. Özet Tablosu

| Yaklaşım | Avantaj | Dezavantaj | Kullanım Yeri |
|----------|---------|------------|---------------|
| `panic!` | Basit | Programı öldürür | Kurtarılamaz hatalar, testler |
| `Option<T>` | "Yokluk" durumunu temsil eder | Hata bilgisi taşımaz | Opsiyonel değerler |
| `Result<T, E>` | Hata bilgisi taşır | Verbose olabilir | Hata olması muhtemel işlemler |
| `?` operatörü | Çok temiz kod | Fonksiyon imzasını değiştirir | Tüm Result/Option zincirleri |
| `Box<dyn Error>` | Tek tip, kolay | Runtime'da tip belirlenir | Genel amaçlı, küçük projeler |
| Özel Error enum | Tip güvenli, zengin | Boilerplate çok | Büyük, ciddi projeler |
| Crate'ler (`anyhow`, `thiserror`) | Minimum boilerplate | Ekstra bağımlılık | Üretim kodu |

---

## 16. Pratik İpuçları

### 1. Prototipleme Aşaması
```rust
// Hızlı prototip için unwrap/expect kullanabilirsiniz
let value = some_result.expect("Bu kesinlikle çalışmalı!");
```

### 2. Üretim Kodu
```rust
// Her zaman ? operatörü kullanın
let value = some_result?;
```

### 3. Uygulama Seviyesinde Hata Yönetimi
```rust
// En üst seviyede (main veya CLI handler) hataları yakalayın
fn main() -> Result<(), Box<dyn Error>> {
    // Tüm ? operatörleri burada toplanır
    run_app()?;
    Ok(())
}
```

### 4. Kütüphane Geliştirme
```rust
// Kütüphanelerde her zaman özel hata tipleri tanımlayın
pub enum MyLibError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    Custom(String),
}
```

### 5. Popüler Crate'ler
- **`anyhow`** — Uygulama geliştirme için ergonomik `Box<dyn Error>` alternatifi
- **`thiserror`** — Kütüphane geliştirme için derive makrosuyla özel hata tipleri

---

## 17. Sonuç

Rust'ın hata yönetimi sistemi, ilk bakışta karmaşık görünebilir, ancak mantıklı bir hiyerarşiye sahiptir:

1. **Basit durumlar** → `Option<T>` ve `Result<T, E>`
2. **Kısa kod** → `?` operatörü, `map()`, `and_then()`
3. **Birden fazla hata tipi** → `Box<dyn Error>` veya özel `enum`
4. **Ciddi projeler** → `thiserror` / `anyhow` gibi crate'ler

Unutmayın: Rust'ın felsefesi **"hataları görmezden gelemeyeceğiniz bir dil"** olmaktır. Bu başta zor gelse de, uzun vadede çok daha güvenilir yazılımlar üretmenizi sağlar.

> **Altın kural:** `unwrap()` ve `expect()` sadece testlerde ve prototiplerde kullanılmalıdır. Üretim kodunda her zaman `?` operatörünü tercih edin!

# DeepSeek Yorumu ------------------

Rust'ta hata yönetimi, programların karşılaşabileceği beklenmedik durumları ele almak için kritik bir öneme sahiptir. Örneğin, bir dosyayı okumaya çalışıp başarısız olmak veya kullanıcıdan alınan geçersiz bir girdiyi işlemek gibi durumlar, programın geri kalanını olumsuz etkileyebilir. Rust, bu tür hataları yönetmek için farklı araçlar sunar ve bu araçların her biri farklı kullanım senaryolarına ve inceliklere sahiptir.

Aşağıda, Rust By Example'ın [Error Handling](https://doc.rust-lang.org/rust-by-example/error.html) bölümünde ele alınan temel kavramları ve alt başlıkları detaylı bir şekilde bulabilirsiniz.

---

### 📘 Giriş: Hata Yönetimine Genel Bakış

Hata yönetimi, bir programda başarısızlık olasılığını ele alma sürecidir. Rust'ta bu amaçla kullanılan başlıca yaklaşımlar şunlardır:

*   **`panic!`**: Genellikle testlerde ve kurtarılamaz hataları bildirmek için kullanılır. Prototipleme aşamasında, henüz implemente edilmemiş fonksiyonlar için geçici bir çözüm olarak da tercih edilebilir (ancak bunun yerine `unimplemented!` makrosu daha açıklayıcıdır).
*   **`Option`**: Bir değerin **var olup olmadığını** (yokluğun bir hata durumu olmadığı) senaryolar için kullanılır. Örneğin, bir dizinin üst dizinini sorgulamak: `/` veya `C:` dizininin bir üst dizini yoktur, bu bir hata değil, `None` ile ifade edilen geçerli bir durumdur.
*   **`Result`**: Bir işlemin **başarılı (`Ok`)** veya **başarısız (`Err`)** olabileceği, yani hatanın bir olasılık olduğu durumlar için kullanılır. Çağıran tarafın bu hatayı ele alması beklenir.

`unwrap()` ve `expect()` metodları, `Option` ve `Result` türleri üzerinde hızlı sonuç almak için kullanılabilir. Ancak, `unwrap()` hata durumunda programı panikletir ve genellikle **sadece testlerde veya hızlı prototiplerde** kullanılması önerilir. `expect()` ise hata durumunda özelleştirilmiş bir mesaj göstermeye olanak tanır.

---

### 📂 Alt Başlıklar ve Detaylı Açıklamaları

#### 1. `panic!` (Panik)

`panic!`, Rust'taki en basit hata yönetim mekanizmasıdır. Bir hata mesajı yazdırır, yığını (stack) geri sarar (unwind) ve genellikle programı sonlandırır.

**Örnek Kullanım:**
```rust
fn drink(beverage: &str) {
    if beverage == "lemonade" {
        panic!("AAAaaaaa!!!!");
    }
    println!("Some refreshing {} is all I need.", beverage);
}

fn main() {
    drink("water");
    drink("lemonade"); // Bu çağrı panikler ve program sonlanır.
    drink("still water"); // Bu satıra asla ulaşılmaz.
}
```
Bu örnekte, limonata içmeye çalışmak kasıtlı olarak bir paniğe neden olur ve programın geri kalanının çalışmasını engeller.

#### 2. `Option` ve `unwrap`

`Option` enum'ı, bir değerin **var olabileceği (`Some(T)`)** veya **olmayabileceği (`None`)** durumları temsil eder.

*   **Açık İşleme (`match`)**: Her iki durumu da ele alarak güvenli bir şekilde yönetebilirsiniz.
*   **Örtük İşleme (`unwrap`)**: `Some` ise içindeki değeri döndürür, `None` ise panikler.

**Örnek Kullanım:**
```rust
fn give_adult(drink: Option<&str>) {
    match drink {
        Some("lemonade") => println!("Yuck! Too sugary."),
        Some(inner) => println!("{}? How nice.", inner),
        None => println!("No drink? Oh well."),
    }
}

fn drink(drink: Option<&str>) {
    let inside = drink.unwrap(); // None ise panikler.
    if inside == "lemonade" {
        panic!("AAAaaaaa!!!!");
    }
    println!("I love {}s!!!!!", inside);
}
```
Bu örnek, `match` ile açık işlemenin daha kontrollü olduğunu, `unwrap`'ın ise daha az anlamlı hata mesajlarına yol açabileceğini gösterir. `expect` ile panik mesajını özelleştirebilirsiniz.

#### 3. `Result`

`Result` enum'ı, bir işlemin **başarılı (`Ok(T)`)** veya **başarısız (`Err(E)`)** olabileceği durumları temsil eder. `Option`'a göre daha zengin bir yapıdır çünkü hata hakkında bilgi taşıyabilir.

*   **`unwrap()`**: `Ok` ise değeri döndürür, `Err` ise panikler.
*   **`parse()`**: Bir string'i başka bir türe dönüştürmeye çalışan yaygın bir `Result` döndüren metottur.

**Örnek Kullanım:**
```rust
fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number * second_number
}
```
Geçersiz bir string (`"t"`) parse edilmeye çalışılırsa, `unwrap` panikler ve program sonlanır.

**`main`'de `Result` Kullanımı:**
`main` fonksiyonu da `Result` döndürebilir. Bu durumda, hata oluşursa program bir hata koduyla çıkar ve hatanın debug çıktısını yazdırır.
```rust
use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(number) => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);
    Ok(())
}
```

#### 4. Erken Dönüş (`?` Operatörü)

`?` operatörü, `Result` veya `Option` türleriyle çalışmayı kolaylaştırır. Eğer değer `Ok` veya `Some` ise içindeki değeri döndürür; `Err` veya `None` ise hatayı çağrıldığı fonksiyondan erken döndürür. Bu, hata yönetimini daha okunabilir hale getirir. (Bu konu, `result/early_returns.html` sayfasında detaylandırılmıştır).

#### 5. Birden Fazla Hata Türü (Multiple Error Types)

Gerçek dünya uygulamalarında, bir fonksiyon farklı türlerde hatalar döndürebilir. Bu durumu yönetmek için:
*   Kendi hata türünüzü tanımlayabilirsiniz.
*   Farklı hata türlerini tek bir türde birleştirmek için kutulama (boxing) veya `From` trait'i gibi yöntemler kullanabilirsiniz.

#### 6. Hataları Kutulama (Boxing Errors)

Hata türlerini `Box<dyn Error>` olarak kutulayarak, farklı hata türlerini tek bir `Result` türü altında toplayabilirsiniz. Bu, özellikle hata türünün önemsiz olduğu durumlarda kullanışlıdır.

#### 7. `Result` Üzerinde Yineleme (Iterating over `Result`)

`Result` döndüren bir yineleyici (iterator) üzerinde işlem yaparken, `collect()` gibi metodlar kullanarak tüm sonuçları toplayabilir veya ilk hatada durarak işlemi sonlandırabilirsiniz.

#### 8. Özel Hata Türü Tanımlama (Defining an Error Type)

Kendi hata türünüzü tanımlayarak, uygulamanıza özel hata durumlarını daha anlamlı ve yönetilebilir hale getirebilirsiniz. Bu genellikle `std::error::Error` trait'ini implemente ederek yapılır.

---

### 💡 Özet ve En İyi Uygulamalar

*   **`panic!`**: Kurtarılamaz hatalar ve testler için.
*   **`Option`**: Bir değerin yokluğunun geçerli bir durum olduğu senaryolar için.
*   **`Result`**: Başarısızlık olasılığının olduğu ve çağıranın bu durumu ele alması gereken senaryolar için.
*   **`unwrap()` ve `expect()`**: Sadece prototipleme veya testlerde kullanılmalıdır. Gerçek kodlarda mümkün olduğunca kaçınılmalıdır.
*   **`?` Operatörü**: Hata yönetimini sadeleştirir ve okunabilirliği artırır.

Rust'ta hata yönetimi konusunda daha derinlemesine bilgi için [Rust Resmi Kitabı'nın ilgili bölümüne](https://doc.rust-lang.org/book/ch09-00-error-handling.html) başvurabilirsiniz.