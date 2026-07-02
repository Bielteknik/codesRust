# 🦀 Ders Notları: Bölüm 19 - Rust'ta Patterns (Örüntüler/Desenler) — Kapsamlı Ders

Hoş geldiniz! Bu derste Rust'ın en güçlü ve en zarif özelliklerinden biri olan **Pattern Matching** (Örüntü Eşleştirme) sistemini derinlemesine inceleyeceğiz. Rust kitabının 19. bölümünü temel alarak, konuyu baştan sona, bol örneklerle ve anlaşılır bir şekilde işleyeceğiz.

---

## 📌 Bölüm 1: Patterns Nedir?

Rust'ta **pattern** (örüntü/desen), verinin yapısını (hem basit hem karmaşık tiplerde) eşleştirmek için kullanılan özel bir sözdizimidir. Pattern'ler, `match` ifadesi ve diğer yapılarla birlikte kullanıldığında programın kontrol akışı üzerinde size muazzam bir kontrol sağlar.

Bir pattern şu bileşenlerin bir kombinasyonundan oluşur:
- **Literal'lar** (sabit değerler: `5`, `'a'`, `"merhaba"`)
- **Destructuring** (parçalama): array, enum, struct veya tuple'ları açma
- **Değişkenler**
- **Wildcard'lar** (joker karakterler: `_`)
- **Placeholder'lar** (yer tutucular: `..`)

### Temel Mantık

Bir pattern'i bir değerle karşılaştırırsınız. Eğer değer pattern'in şeklini karşılıyorsa, pattern içindeki isimlendirilmiş parçaları kodunuzda kullanabilirsiniz. Karşılık vermiyorsa, o pattern'e bağlı kod çalışmaz.

Örnek pattern'ler:
- `x` → herhangi bir değeri yakalar
- `(a, 3)` → ikinci elemanı 3 olan bir tuple'ı yakalar
- `Some(Color::Red)` → `Some` içinde `Color::Red` olan bir değeri yakalar

---

## 📌 Bölüm 2: Pattern'lerin Kullanıldığı Yerler

Pattern'ler Rust'ta sandığınızdan çok daha fazla yerde karşınıza çıkar. Gelin hepsini tek tek inceleyelim.

### 2.1 — `match` İfadeleri

`match` ifadesi pattern matching'in en klasik kullanım alanıdır. Yapısı şöyledir:

```rust
match DEGER {
    PATTERN => IFade,
    PATTERN => IFade,
    PATTERN => IFade,
}
```

**Önemli kural:** `match` ifadeleri **exhaustive** (kapsayıcı) olmak zorundadır. Yani, eşleştirilen değerin alabileceği **tüm olasılıklar** pattern'lerde kapsanmalıdır.

```rust
let x = Some(5);

match x {
    None => None,
    Some(i) => Some(i + 1),
}
```

Burada `None` ve `Some(i)` pattern'leri, `Option<i32>`'ün alabileceği her iki durumu da kapsadığı için `match` geçerlidir.

**Catch-all (hepsini yakala) pattern:** Eğer tüm olasılıkları tek tek yazmak istemiyorsanız, son kola `_` pattern'ini koyabilirsiniz:

```rust
match x {
    1 => println!("bir"),
    2 => println!("iki"),
    _ => println!("başka bir şey"),  // geriye kalan her şeyi yakalar
}
```

### 2.2 — `let` İfadeleri

Her `let` kullandığınızda aslında pattern matching yapıyorsunuz, farkında olmasanız bile!

```rust
let x = 5;
```

Burada `x` aslında bir pattern'dir ve "buraya uyan değeri `x` değişkenine bağla" anlamına gelir.

**Tuple destructuring örneği:**

```rust
let (x, y, z) = (1, 2, 3);
// x = 1, y = 2, z = 3
```

Rust, `(1, 2, 3)` değerini `(x, y, z)` pattern'iyle karşılaştırır ve eleman sayısı aynı olduğu için eşleşme başarılı olur.

**Hata durumu:** Eğer eleman sayıları uyuşmazsa derleyici hata verir:

```rust
let (x, y) = (1, 2, 3);  // ❌ HATA!
// error[E0308]: mismatched types
// expected a tuple with 3 elements, found one with 2 elements
```

### 2.3 — `if let` İfadeleri

`if let`, tek bir durumu kontrol etmek için `match`'in kısa yoludur. Ayrıca `else if`, `else if let` ve `else` ile birlikte zincirleme kullanılabilir:

```rust
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Favori rengin olan {color} kullanılıyor");
    } else if is_tuesday {
        println!("Salı günü yeşil günüdür!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Mor arka plan kullanılıyor");
        } else {
            println!("Turuncu arka plan kullanılıyor");
        }
    } else {
        println!("Mavi arka plan kullanılıyor");
    }
}
```

**Dikkat:** `if let` içinde yeni bir değişken tanımlandığında, bu değişken dışarıdaki aynı isimli değişkeni **gölgele**r (shadow):

```rust
let age: Result<u8, _> = "34".parse();

if let Ok(age) = age {
    // Buradaki 'age', Result içindeki değerdir, dışarıdaki Result değil!
    if age > 30 {
        println!("30'dan büyük");
    }
}
```

⚠️ **Dezavantaj:** `if let`'te derleyici exhaustiveness (kapsayıcılık) kontrolü yapmaz. `match`'te yapsa da `if let`'te yapmaz.

### 2.4 — `while let` Döngüleri

`while let`, bir pattern eşleştiği sürece döngüyü çalıştırır:

```rust
fn main() {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for val in [1, 2, 3] {
            tx.send(val).unwrap();
        }
    });

    while let Ok(value) = rx.recv() {
        println!("{value}");
    }
    // rx.recv() Ok döndürdüğü sürece döngü çalışır.
    // Gönderici kapandığında Err döner ve döngü biter.
}
```

Bu kod `1`, `2`, `3` yazar ve sonra döngü sonlanır.

### 2.5 — `for` Döngüleri

`for` döngüsündeki değişken de bir pattern'dir:

```rust
let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() {
    println!("{value} is at index {index}");
}
```

Burada `enumerate()` metodu `(index, value)` şeklinde tuple'lar üretir ve `for` döngüsü bu tuple'ı `(index, value)` pattern'iyle eşleştirir.

### 2.6 — Fonksiyon Parametreleri

Fonksiyon parametreleri de aslında birer pattern'dir:

```rust
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Konum: ({x}, {y})");
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
    // Çıktı: Konum: (3, 5)
}
```

Burada `&(x, y)` pattern'i, `&(3, 5)` değeriyle eşleşir ve `x=3`, `y=5` olur.

---

## 📌 Bölüm 3: Refutability — Çürütülebilirlik

Pattern'ler iki türdür: **refutable** (çürütülebilir) ve **irrefutable** (çürütülemez). Bu kavramı anlamak çok önemlidir.

### 3.1 — Irrefutable (Çürütülemez) Pattern'ler

Herhangi bir değeri **her zaman** yakalayabilen pattern'lerdir. Asla başarısız olmazlar.

**Örnek:** `let x = 5;` ifadesindeki `x` pattern'i irrefutable'dır çünkü herhangi bir değeri kabul eder.

### 3.2 — Refutable (Çürütülebilir) Pattern'ler

Bazı değerler için **eşleşemeyebilecek** pattern'lerdir.

**Örnek:** `if let Some(x) = a_value` ifadesindeki `Some(x)` pattern'i refutable'dır. Eğer `a_value` değişkeni `None` ise, pattern eşleşmez.

### 3.3 — Hangi Yapı Hangi Pattern'i Kabul Eder?

| Yapı | Kabul Ettiği Pattern Türü |
|------|---------------------------|
| `let`, `for`, fonksiyon parametreleri | **Sadece irrefutable** |
| `if let`, `while let`, `let...else` | **Hem refutable hem irrefutable** |

**Hata örneği:**

```rust
fn main() {
    let some_option_value: Option<i32> = None;
    let Some(x) = some_option_value;  // ❌ HATA!
}
```

Derleyici hatası:
```
error[E0005]: refutable pattern in local binding
  |
3 |     let Some(x) = some_option_value;
  |         ^^^^^^^ pattern `None` not covered
```

Çünkü `let` ifadesi, `None` durumunda ne yapacağını bilemez.

**Çözüm — `let...else` kullanımı:**

```rust
fn main() {
    let some_option_value: Option<i32> = None;
    let Some(x) = some_option_value else {
        return;  // Pattern eşleşmezse burası çalışır
    };
    // x burada kullanılabilir
}
```

`let...else` yapısı, pattern eşleşmediğinde çalışacak bir blok sağlar.

---

## 📌 Bölüm 4: Pattern Sözdizimi

Bu bölümde tüm pattern sözdizimini detaylıca inceleyeceğiz.

### 4.1 — Literal Eşleştirme

Doğrudan sabit değerlerle eşleştirme yapabilirsiniz:

```rust
let x = 1;

match x {
    1 => println!("bir"),
    2 => println!("iki"),
    3 => println!("üç"),
    _ => println!("başka bir şey"),
}
// Çıktı: bir
```

### 4.2 — İsimlendirilmiş Değişkenler ve Gölgeleme (Shadowing)

İsimlendirilmiş değişkenler irrefutable pattern'lerdir ve her değeri yakalar. Ancak `match` içinde kullanıldığında **gölgeleme** problemi ortaya çıkabilir:

```rust
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("50 geldi"),
        Some(y) => println!("Eşleşti, y = {y}"),  // ⚠️ Dikkat!
        _ => println!("Varsayılan durum, x = {x:?}"),
    }

    println!("sonunda: x = {x:?}, y = {y}");
}
```

**Ne olur?**
- `Some(y)` pattern'i, dışarıdaki `y = 10` ile karşılaştırma **yapmaz**!
- Bunun yerine, **yeni bir `y` değişkeni** oluşturur ve `Some` içindeki değere bağlar.
- Çıktı: `Eşleşti, y = 5` olur (10 değil!)
- Son satırda: `sonunda: x = Some(5), y = 10` (dışarıdaki `y` değişmedi)

**Çözüm:** Dışarıdaki değişkenle karşılaştırmak için **match guard** kullanın (aşağıda anlatılacak).

### 4.3 — Çoklu Pattern'ler (`|` Operatörü)

`|` (veya/pipe) operatörüyle birden fazla pattern'i birleştirebilirsiniz:

```rust
let x = 1;

match x {
    1 | 2 => println!("bir veya iki"),
    3 => println!("üç"),
    _ => println!("başka bir şey"),
}
// Çıktı: bir veya iki
```

### 4.4 — Aralık Pattern'leri (`..=`)

`..=` sözdizimiyle kapalı aralık belirtilebilir. Sadece sayısal ve `char` tiplerinde kullanılabilir:

```rust
let x = 5;

match x {
    1..=5 => println!("1'den 5'e kadar"),
    _ => println!("başka bir şey"),
}
// Çıktı: 1'den 5'e kadar
```

**`char` aralığı örneği:**

```rust
let x = 'c';

match x {
    'a'..='j' => println!("erken ASCII harfi"),
    'k'..='z' => println!("geç ASCII harfi"),
    _ => println!("başka bir şey"),
}
// Çıktı: erken ASCII harfi
```

⚠️ Derleyici, aralığın boş olmadığını derleme zamanında kontrol eder.

### 4.5 — Struct Destructuring (Yapı Parçalama)

Bir struct'ın alanlarını ayrı değişkenlere çıkarabilirsiniz:

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}
```

**Kısayol:** Değişken isimleri alan isimleriyle aynıysa, kısayol kullanabilirsiniz:

```rust
let Point { x, y } = p;
// x = 0, y = 7
```

**Literal ile birleştirilmiş destructuring:**

```rust
let p = Point { x: 0, y: 7 };

match p {
    Point { x, y: 0 } => println!("x ekseni üzerinde, x = {x}"),
    Point { x: 0, y } => println!("y ekseni üzerinde, y = {y}"),
    Point { x, y } => println!("hiçbir eksen üzerinde değil: ({x}, {y})"),
}
// Çıktı: y ekseni üzerinde, y = 7
```

⚠️ `match` ilk eşleşen kolda durur. `Point { x: 0, y: 0 }` her iki eksende de olsa, sadece ilk kol çalışır.

### 4.6 — Enum Destructuring

Enum varyantlarını, içindeki verilere göre parçalayabilirsiniz:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("Quit varyantının verisi yok.");
        }
        Message::Move { x, y } => {
            println!("x yönünde {x}, y yönünde {y} hareket");
        }
        Message::Write(text) => {
            println!("Metin mesajı: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Renk değiştir: kırmızı {r}, yeşil {g}, mavi {b}");
        }
    }
    // Çıktı: Renk değiştir: kırmızı 0, yeşil 160, mavi 255
}
```

**İç içe (nested) enum destructuring:**

```rust
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    ChangeColor(Color),
    // ...
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("RGB: {r}, {g}, {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("HSV: {h}, {s}, {v}");
        }
        _ => (),
    }
}
```

**Karmaşık destructuring örneği:**

```rust
struct Point { x: i32, y: i32 }

let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
// feet = 3, inches = 10, x = 3, y = -10
```

### 4.7 — Değerleri Görmezden Gelme

Bazen bir pattern'deki bazı değerleri kullanmak istemezsiniz. Bunun birkaç yolu var:

#### a) `_` Wildcard (Joker) Pattern

Herhangi bir değeri yakalar ama bağlamaz:

```rust
fn foo(_: i32, y: i32) {
    println!("Sadece y parametresi kullanılıyor: {y}");
}

fn main() {
    foo(3, 4);
    // Çıktı: Sadece y parametresi kullanılıyor: 4
}
```

Bu özellikle trait implementasyonlarında, imza gereği bir parametreyi kullanmadığınızda faydalıdır.

#### b) `_` ile Bir Değerin Bir Kısmını Görmezden Gelme

```rust
let mut setting_value = Some(5);
let new_setting_value = Some(10);

match (setting_value, new_setting_value) {
    (Some(_), Some(_)) => {
        println!("Mevcut özelleştirilmiş değer üzerine yazılamaz");
    }
    _ => {
        setting_value = new_setting_value;
    }
}
// Çıktı: Mevcut özelleştirilmiş değer üzerine yazılamaz
```

#### c) Birden Fazla Değeri Görmezden Gelme

```rust
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, _, third, _, fifth) => {
        println!("Bazı sayılar: {first}, {third}, {fifth}");
    }
}
// Çıktı: Bazı sayılar: 2, 8, 32
```

#### d) `_` ile Başlayan Değişken İsmi

Kullanılmayan değişkenler için Rust uyarı verir. `_` ile başlayan isimler bu uyarıyı bastırır:

```rust
fn main() {
    let _x = 5;  // ⚠️ Uyarı yok
    let y = 10;  // ⚠️ "unused variable" uyarısı var
}
```

**Önemli fark:** `_x` değere bağlanır (ownership alır), ama `_` bağlanmaz!

```rust
let s = Some(String::from("Merhaba!"));

if let Some(_s) = s {
    println!("string bulundu");
}
println!("{s:?}");  // ❌ HATA! s, _s'e taşındı (moved)
```

Ama `_` kullanırsanız:

```rust
let s = Some(String::from("Merhaba!"));

if let Some(_) = s {
    println!("string bulundu");
}
println!("{s:?}");  // ✅ Çalışır! s taşınmadı.
```

#### e) `..` ile Kalan Parçaları Görmezden Gelme

```rust
struct Point { x: i32, y: i32, z: i32 }

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
    Point { x, .. } => println!("x = {x}"),
    // y ve z görmezden gelindi
}
```

**Tuple ile kullanım:**

```rust
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, .., last) => {
        println!("Bazı sayılar: {first}, {last}");
    }
}
// Çıktı: Bazı sayılar: 2, 32
```

⚠️ `..` belirsiz kullanılamaz:

```rust
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (.., second, ..) => {  // ❌ HATA!
        println!("{second}")
    }
}
// error: `..` can only be used once per tuple pattern
```

### 4.8 — Match Guard (Eşleştirme Koşulu)

Match guard, pattern'in ardından gelen ek bir `if` koşuludur. Pattern eşleşse bile, guard'ın da `true` olması gerekir. **Sadece `match` ifadelerinde kullanılabilir.**

```rust
let num = Some(4);

match num {
    Some(x) if x % 2 == 0 => println!("{x} çifttir"),
    Some(x) => println!("{x} tektir"),
    None => (),
}
// Çıktı: 4 çifttir
```

**Gölgeleme problemini çözme:**

```rust
let x = Some(5);
let y = 10;

match x {
    Some(50) => println!("50 geldi"),
    Some(n) if n == y => println!("Eşleşti, n = {n}"),
    _ => println!("Varsayılan, x = {x:?}"),
}
// Çıktı: Varsayılan, x = Some(5)
// Çünkü 5 != 10
```

Burada `n` yeni bir değişken, `y` ise dışarıdaki `y`'dir. Gölgeleme yok!

**`|` operatörü ile birlikte kullanım:**

```rust
let x = 4;
let y = false;

match x {
    4 | 5 | 6 if y => println!("evet"),
    _ => println!("hayır"),
}
// Çıktı: hayır
```

⚠️ Öncelik: `if y` koşulu **tüm pattern'lere** uygulanır:
```
(4 | 5 | 6) if y => ...
```
Yani `y` true olsaydı, 4, 5 veya 6 olduğunda "evet" yazacaktı.

### 4.9 — `@` Operatörü

`@` operatörü, bir değeri test ederken aynı zamanda onu bir değişkene bağlamanızı sağlar:

```rust
enum Message {
    Hello { id: i32 },
}

let msg = Message::Hello { id: 5 };

match msg {
    Message::Hello { id: id @ 3..=7 } => {
        println!("Aralıktaki id bulundu: {id}")
    }
    Message::Hello { id: 10..=12 } => {
        println!("Başka bir aralıkta id bulundu")
    }
    Message::Hello { id } => println!("Başka bir id bulundu: {id}"),
}
// Çıktı: Aralıktaki id bulundu: 5
```

**Farklar:**
- İlk kolda: `id @ 3..=7` → Hem aralığı test eder, hem de değeri `id` değişkenine bağlar.
- İkinci kolda: Sadece `10..=12` aralığı test edilir, değer bir değişkene bağlanmaz.
- Üçüncü kolda: Değer `id`'ye bağlanır ama aralık testi yapılmaz.

`@` operatörü, **hem test hem bağlama** işlemini tek pattern'de yapmanızı sağlar.

---

## 📌 Özet ve En İyi Uygulamalar

### Pattern Matching'in Gücü

Rust'ın pattern matching sistemi, veri yapılarını güvenli ve okunaklı bir şekilde işlemek için güçlü bir araçtır:

1. **`match`** → Kapsamlı durum analizi için
2. **`if let` / `while let`** → Tek durum kontrolü için
3. **`let...else`** → Erken dönüş (early return) pattern'leri için
4. **Destructuring** → Karmaşık verileri parçalamak için
5. **Match guard** → Ek koşullar eklemek için
6. **`@` operatörü** → Test ederken bağlamak için

### Sık Yapılan Hatalar

| Hata | Çözüm |
|------|-------|
| `let Some(x) = option_val` | `let Some(x) = option_val else { return; };` kullan |
| `match`'te exhaustiveness hatası | `_ => ()` catch-all kol ekle |
| Gölgeleme problemi | Match guard (`if`) kullan |
| `..` belirsiz kullanımı | Tek bir `..` ile sınırla |
| Kullanılmayan değişken uyarısı | `_` ile başlat: `_x` |

### Son Söz

Pattern matching, Rust'ın "güvenli ve ifade gücü yüksek" felsefesinin mükemmel bir örneğidir. Derleyici, pattern'lerin doğruluğunu kontrol ederek runtime hatalarını compile-time'a taşır. Bu dersteki kavramları ustalıkla kullandığınızda, Rust kodunuz çok daha temiz, güvenli ve okunaklı hale gelecektir.

Bir sonraki konu olarak Rust'ın gelişmiş özelliklerine (Advanced Features) geçebilirsiniz. Başarılar! 🚀

# DeepSeek Yorumu -----------------

Rust programlama dilinde **desenler (patterns)** , veri yapılarının şeklini tanımlamak ve bu şekle göre değerleri eşleştirip parçalarına ayırmak için kullanılan özel bir sözdizimidir. Bugünkü dersimizde, Rust Kitabı’nın 19. bölümünü baştan sona ele alacağız: desenlerin nerelerde kullanılabileceğini, çürütülebilirlik (refutability) kavramını ve en ince ayrıntısına kadar desen sözdizimini öğreneceğiz.

---

## 1. Desenlerin Kullanılabildiği Bütün Yerler

Rust’ta desenler çok sayıda yerde karşımıza çıkar. Her birini tanıyalım.

### `match` Kolları
En bilinen kullanım `match` ifadesidir. Her kol bir desen ve isteğe bağlı bir koruyucu (`if` guard) içerir.

```rust
match x {
    None => println!("Hiçbir şey"),
    Some(v) if v > 5 => println!("5'ten büyük: {}", v),
    Some(v) => println!("Diğer değer: {}", v),
}
```

Burada `None`, `Some(v)` birer desendir. Koruyucu (`if v > 5`) desenin parçası değildir; desen eşleştikten sonra ek bir koşul getirir.

### `if let` İfadeleri
`if let`, bir desenin eşleşmesi durumunda bir blok çalıştırmanın kısa yoludur. Ayrıca `else if let` ve `else` zincirleri de yazılabilir. Dezavantajı, `match`’in sağladığı kapsamlılık (exhaustiveness) kontrolünün yapılmamasıdır.

```rust
if let Some(v) = maybe_value {
    println!("Değer: {}", v);
} else {
    println!("Değer yok");
}
```

### `while let` Koşullu Döngüleri
Bir desen eşleştiği sürece döngüyü sürdürür. Genellikle kuyruk, yığın veya kanal alıcılarında kullanılır.

```rust
let mut stack = Vec::new();
stack.push(1);
stack.push(2);

while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

### `for` Döngüleri
`for` döngüsünde, `in` anahtar kelimesinden sonra gelen desen doğrudan desenin kendisidir. Çoğu zaman basit bir değişken ismi görsek de, aslında `for` bir desen bekler. Bu sayede demetleri doğrudan parçalayabiliriz.

```rust
let noktalar = vec![(0, 0), (1, 2), (3, 4)];
for (x, y) in noktalar {
    println!("x: {}, y: {}", x, y);
}
```

### `let` İfadeleri
`let` ile değişken tanımlarken aslında sürekli desen kullanırız. `let x = 5;` yazdığımızda `x` bir desendir; bu desen her zaman eşleşir (çürütülemez). Daha karmaşık desenler de kullanabiliriz:

```rust
let (a, b, c) = (1, 2, 3);
let (x, y, _) = (10, 20, 30);
```

Ancak `let` her zaman **çürütülemez (irrefutable)** bir desen talep eder. Yani desen asla başarısız olmamalıdır. Aşağıdaki kod derlenmez:

```rust
let Some(deger) = option_bir_degisken; // HATA: `None` gelirse eşleşmez!
```

### Fonksiyon Parametreleri
Fonksiyon parametreleri de birer desendir. Demetleri doğrudan parametre konumunda parçalamak mümkündür.

```rust
fn koordinatlari_yazdir((x, y): (i32, i32)) {
    println!("({}, {})", x, y);
}
```

Bu sayede kapama (closure) parametrelerinde de desen kullanabiliriz; örneğin `iter().map(|(x, y)| x + y)`.

---

## 2. Çürütülebilirlik: Bir Desen Eşleşmeme İhtimali Taşır mı?

Desenler iki ana gruba ayrılır:

- **Çürütülemez (Irrefutable)** desenler: Her durumda eşleşir. Örnek: `let x = 5;` (her değer `x` ile eşleşir), demet parçalama `(a, b) = (1, 2);`.
- **Çürütülebilir (Refutable)** desenler: Eşleşmeme ihtimali vardır. Örnek: `Some(v)` bir `Option` değeri ile eşleşirken `None` durumunda başarısız olur.

Rust, hangi bağlamda hangi tür deseni istediğimize karışır:

- `let`, fonksiyon parametreleri ve `for` döngüleri yalnızca **çürütülemez** desen kabul eder. Çünkü bu yapılar başarısızlığı ele alacak bir kontrol akışına sahip değildir.
- `if let`, `while let` ve `match` kolları ise **çürütülebilir** desenlerle çalışır (çürütülemez desen de kullanılabilir ama anlamsız olur; derleyici uyarı verir).

Örneğin:

```rust
// HATA: `let` çürütülemez desen bekler.
let Some(x) = option_bir_degisken;

// Çözüm: `if let` ile çürütülebilir desen kullanın.
if let Some(x) = option_bir_degisken {
    // ...
}

// Veya `match` ile kapsamlı bir eşleme yapın.
match option_bir_degisken {
    Some(x) => { /* ... */ }
    None => { /* ... */ }
}
```

Eğer `let` ile `Some(x)` yazmak istersek, `option_bir_degisken`’in her zaman `Some` olduğunu garanti edemeyiz. Derleyici bunu yakalar ve çürütülebilir bir deseni `let`’te kullanamayacağımızı söyler. Benzer şekilde `if let`’e `x` gibi çürütülemez bir desen verirsek, derleyici “desen her zaman eşleşecek, `if let` gereksiz” diyerek bizi uyarır.

Bu kavram, desenleri doğru bağlamda kullanmamızı sağlayan temel bir güvenlik mekanizmasıdır. `match`’te ise kolların toplamı tüm olasılıkları kapsamalıdır (exhaustiveness). Bu yüzden `match`’te çürütülebilir desenleri sıralarız, son kol genellikle çürütülemez bir yakalayıcı olur (`_ => {}`).

---

## 3. Desen Sözdizimi – Bütün İmkanlar

Şimdi Rust’ın desen dünyasındaki bütün araçları teker teker inceleyelim.

### 3.1. Değişmez Değerlerle Eşleme (Literals)
Doğrudan bir sabit değerle eşleşme yapabilirsiniz.

```rust
let x = 1;
match x {
    1 => println!("bir"),
    2 => println!("iki"),
    _ => println!("diğer"),
}
```

### 3.2. Adlandırılmış Değişkenler
Bir değişken ismi, her değerle eşleşen çürütülemez bir desendir. Ancak `match` içinde bu değişkenler **yeni bir bağlanma (binding)** oluşturur; dışarıdaki değişkeni gölgeler.

```rust
let x = Some(5);
let y = 10;
match x {
    Some(50) => println!("50"),
    Some(y) => println!("Eşleşti, y = {}", y), // Buradaki y yeni bir değişken, dıştaki y=10'u gölgeliyor.
    _ => println!("Varsayılan"),
}
println!("x = {:?}, y = {}", x, y); // x değişmedi, y hala 10.
```

### 3.3. Çoklu Desen ( `|` operatörü)
Aynı kol için birden fazla deseni `|` ile birleştirebiliriz.

```rust
let x = 1;
match x {
    1 | 2 => println!("bir veya iki"),
    _ => println!("diğer"),
}
```

### 3.4. Aralıklar `..=` ile Eşleme
`..=` sözdizimi, kapalı bir aralıktaki değerlerle eşleşir. Sayısal ve `char` türleriyle kullanılabilir.

```rust
let x = 5;
match x {
    1..=5 => println!("1 ile 5 arasında"),
    _ => println!("diğer"),
}

let c = 'Z';
match c {
    'a'..='z' => println!("küçük harf"),
    'A'..='Z' => println!("büyük harf"),
    _ => println!("harf değil"),
}
```

### 3.5. Yapıları Parçalama (Destructuring)

#### Demetleri (Tuple) Parçalama
```rust
let tup = (500, 6.4, 1);
let (x, y, z) = tup; // x=500, y=6.4, z=1

match tup {
    (0, y, z) => println!("İlk eleman sıfır, y={}, z={}", y, z),
    (x, ..)  => println!("İlk eleman {}, diğerleri önemsiz", x),
}
```

#### Yapıları (Struct) Parçalama
Alan isimlerini kullanarak parçalarız. `..` ile kalan alanları yok sayabiliriz.

```rust
struct Nokta {
    x: i32,
    y: i32,
}

let p = Nokta { x: 0, y: 7 };
let Nokta { x, y } = p; // x ve y değişkenleri oluşur
// veya farklı isimle bağlamak:
let Nokta { x: x_ekseni, y: y_ekseni } = p;

match p {
    Nokta { x, y: 0 } => println!("x ekseni üzerinde, x={}", x),
    Nokta { x: 0, y } => println!("y ekseni üzerinde, y={}", y),
    Nokta { x, y }     => println!("({}, {})", x, y),
}
```

#### Enum’ları Parçalama
Varyantın içindeki verileri çıkarırız.

```rust
enum Mesaj {
    Cik,
    Tasin { x: i32, y: i32 },
    Yaz(String),
    RenkDegistir(i32, i32, i32),
}

let m = Mesaj::RenkDegistir(255, 0, 0);
match m {
    Mesaj::Cik => println!("Çık"),
    Mesaj::Tasin { x, y } => println!("({}, {}) konumuna taşı", x, y),
    Mesaj::Yaz(metin) => println!("Mesaj: {}", metin),
    Mesaj::RenkDegistir(r, g, b) => println!("R:{}, G:{}, B:{}", r, g, b),
}
```

İsimli alanlı varyantlarda `..` ile alan atlayabiliriz: `Mesaj::Tasin { x, .. }`.

#### İç İçe Yapıları Parçalama
Bir enum’ın içindeki struct’ı, demeti vb. derinlemesine parçalayabiliriz.

```rust
enum Renk {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
enum Mesaj {
    Cik,
    RenkDegistir(Renk),
}

let m = Mesaj::RenkDegistir(Renk::Hsv(180, 100, 50));
match m {
    Mesaj::RenkDegistir(Renk::Rgb(r, g, b)) => println!("RGB: {}, {}, {}", r, g, b),
    Mesaj::RenkDegistir(Renk::Hsv(h, s, v)) => println!("HSV: {}, {}, {}", h, s, v),
    _ => (),
}
```

### 3.6. Değerleri Yok Sayma

#### `_` ile tüm değeri yok sayma
Herhangi bir değerle eşleşir, ama bağlama yapmaz. `match`’in son kolunda sık kullanılır.

```rust
match some_value {
    Some(_) => println!("var ama değeri umursamıyorum"),
    None => (),
}
```

#### İç içe `_` kullanımı
Bir parçayı yok saymak için de kullanılır: `(x, _, z)`.

#### `_x` ile kullanılmayan değişken
Değişkeni bağlar ama kullanmayacağımızı derleyiciye söyleriz. `_`’dan farkı, `_x` isimlendirilmiş bir bağlanma oluşturur ve değerin sahipliğini alabilir. `_` ise sahiplik transferi yapmaz, değeri bağlamaz.

```rust
let (x, _y, z) = (1, 2, 3); // _y bağlanır, değer 2 taşınır.
```

Eğer `_` kullansaydık, `2` değeri hareket ettirilmezdi (bind yapılmazdı), bu yüzden `_` ile `_y` arasında ince bir fark vardır. Genelde değer bırakmak istemiyorsak `_` kullanırız.

#### `..` ile kalanı yok sayma
Bir demet veya struct’ta kalan alanları yok saymak için `..` kullanılır. Demet için `(first, .., last)` yazılabilir (Rust 2021’den itibaren `..` demet deseninde istenilen yerde kullanılabilir, önceki sürümlerde sadece sonda veya başta olabilirdi). Struct’ta `..` sonda olmalıdır.

```rust
struct UcBoyutlu { x: i32, y: i32, z: i32 }
let nokta = UcBoyutlu { x: 1, y: 2, z: 3 };
match nokta {
    UcBoyutlu { x, .. } => println!("x: {}", x),
}

let tup = (1, 2, 3, 4, 5);
match tup {
    (first, .., last) => println!("ilk: {}, son: {}", first, last),
}
```

### 3.7. Koruyucular ile Ek Koşullar (Match Guards)
Bir desenin ardından `if` koşulu ekleyerek eşleşmeyi daraltabiliriz. Bu, desenin kendisinden daha karmaşık mantığı ifade etmeye yarar.

```rust
let x = Some(4);
match x {
    Some(n) if n % 2 == 0 => println!("çift sayı: {}", n),
    Some(n) => println!("tek sayı: {}", n),
    None => (),
}
```

Koruyucu, desenin bağladığı değişkenleri kullanabilir. `|` ile birden fazla desende koruyucu kullanırsak, koruyucu bütün alternatiflere uygulanır:

```rust
let x = 4;
let y = false;
match x {
    4 | 5 | 6 if y => println!("evet"),
    _ => println!("hayır"),
}
```
Burada `x` 4,5,6’dan biri **ve** `y` true ise ilk kol çalışır; aksi halde ikinci kol.

### 3.8. `@` Bağlamaları
Bir desenin hem eşleşmesini isteyip hem de eşleşen değeri bir değişkene bağlamak için `@` kullanırız.

```rust
enum Mesaj {
    Selam { id: i32 },
}

let m = Mesaj::Selam { id: 5 };
match m {
    Mesaj::Selam {
        id: id_degiskeni @ 3..=7,
    } => println!("id {} aralıkta bulundu", id_degiskeni),
    Mesaj::Selam { id: 10..=12 } => {
        println!("id 10-12 arasında") // id değerine erişim yok!
    }
    Mesaj::Selam { id } => println!("id: {}", id),
}
```

İlk kolda `id_degiskeni @ 3..=7` sayesinde hem `id`’nin 3..7 aralığında olup olmadığını kontrol ettik, hem de eşleşen değeri `id_degiskeni` olarak yakaladık. İkinci kolda ise doğrudan aralık belirttik ama değeri bir değişkene bağlamadık, bu yüzden id’ye erişemeyiz.

---

## Özet

Rust’ın desen sistemi, veriyi ayrıştırmak ve kontrol akışını yönlendirmek için son derece güçlü araçlar sunar. Bu derste öğrendiklerimiz:

- Desenler `match`, `if let`, `while let`, `for`, `let` ve fonksiyon parametrelerinde kullanılır.
- Her bağlam, desenin **çürütülemez** mi yoksa **çürütülebilir** mi olması gerektiğine dair kurallara sahiptir. `let` çürütülemez ister; `match` kolları çürütülebilir olabilir.
- Desen sözdizimi; değişmezler, adlandırılmış değişkenler, `|`, aralıklar `..=`, demet/struct/enum parçalama, `_` ve `..` ile yok sayma, koruyucu `if` koşulları ve `@` bağlamaları ile zenginleştirilmiştir.

Bu araçları doğru yerde kullanarak hem okunaklı hem de güvenli kod yazabilirsiniz. Desenler Rust’ın ifade gücünün temel taşlarından biridir; onlara ne kadar hâkim olursanız, kodunuz o kadar temiz ve hatasız olur.

Şimdi öğrendiklerinizi kendi projelerinizde deneme zamanı!