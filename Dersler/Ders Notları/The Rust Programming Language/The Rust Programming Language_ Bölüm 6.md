# 🦀   Ders Notları: Bölüm 6 -  Rust'ta Enum'lar (Numaralandırmalar) — Kapsamlı Ders

> **Kaynak:** *The Rust Programming Language* — Bölüm 6  
> Bu ders; enum tanımlama, `Option<T>`, `match` ve `if let` yapılarını adım adım, örneklerle ve derinlemesine açıklamaktadır.

---

## 📖 Bölüm 1 — Enum Nedir? Neden İhtiyacımız Var?

### 1.1 Temel Motivasyon

Diyelim ki bir IP adresi ile çalışıyorsunuz. IP adresleri dünyada sadece **iki standartta** bulunur:

- **IPv4** (ör. `127.0.0.1`)
- **IPv6** (ör. `::1`)

Bir IP adresi **ya IPv4'tür ya da IPv6'dır**; aynı anda ikisi birden olamaz. İşte tam bu noktada **enum** (enumeration = numaralandırma) devreye girer.

> 💡 **Anahtar Fikir:** `struct`'lar birbiriyle ilişkili alanları **bir arada** gruplarken (ör. `Rectangle { width, height }`), `enum`'lar bir değerin **bir dizi olası varyanttan sadece biri** olabileceğini ifade eder.

### 1.2 En Basit Enum Tanımı

```rust
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six  = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

fn route(ip_kind: IpAddrKind) {
    // ...
}
```

Dikkat edilmesi gereken noktalar:

| Özellik | Açıklama |
|---|---|
| `enum` anahtar kelimesi | Numaralandırma tanımlamak için kullanılır |
| `V4`, `V6` | Enum'un **varyantlarıdır** (variant) |
| `IpAddrKind::V4` | Varyantlara erişim için `::` operatörü kullanılır |
| Tip güvenliği | Hem `V4` hem `V6`, aynı `IpAddrKind` tipindedir |

### 1.3 Struct ile Birleştirme (Eski Yöntem)

Varyantlara veri eklemek isterseniz, enum'u struct içine gömebilirsiniz:

```rust
enum IpAddrKind { V4, V6 }

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};
```

Ancak bu **gereksiz dolambaçlı** bir yoldur. Rust bunu çok daha zarif yapmamıza izin verir.

### 1.4 Veriyi Doğrudan Enum Varyantına Gömmek ✨

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home     = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
```

> 🔑 **Çok Önemli Kural:** Enum varyantının adı, aynı zamanda bir **constructor (yapıcı) fonksiyon** olarak otomatik tanımlanır. `IpAddr::V4("...".to_string())` çağrısı, `IpAddr` tipinde bir değer döndürür.

### 1.5 Her Varyant Farklı Tip Taşıyabilir!

Struct ile bunu yapamazsınız, ama enum ile çok kolaydır:

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),   // 4 ayrı byte
    V6(String),            // tek bir string
}

let home     = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

### 1.6 Varyantların İç Yapısı Çok Esnektir

Aynı enum içinde farklı yapılar bir arada olabilir:

```rust
enum Message {
    Quit,                              // veri yok
    Move { x: i32, y: i32 },          // struct gibi isimli alanlar
    Write(String),                     // tek bir String
    ChangeColor(i32, i32, i32),        // üç tane i32
}

fn main() {
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 10, y: 30 };
    let m3 = Message::Write(String::from("Merhaba!"));
    let m4 = Message::ChangeColor(255, 128, 0);
}
```

### 1.7 Enum'lara Metod Tanımlamak (`impl`)

Tıpkı `struct`'larda olduğu gibi, `impl` bloğu ile enum'lara metod ekleyebilirsiniz:

```rust
impl Message {
    fn call(&self) {
        // Bu metod, üzerinde çağrıldığı değeri işler
        println!("Mesaj çağrıldı!");
    }
}

let m = Message::Write(String::from("selam"));
m.call();
```

---

## 📖 Bölüm 2 — `Option<T>` Enum'u: Null'un Yerine Ne Geçti?

### 2.1 "Milyar Dolarlık Hata" — Null Problemi

Null referansının mucidi **Tony Hoare**, 2009'da şöyle demiştir:

> *"Buna milyar dolarlık hatam diyorum. Null referansı koymaya direnemedim çünkü uygulanması çok kolaydı. Bu, son 40 yılda sayısız hata, güvenlik açığı ve sistem çöküşüne yol açtı."*

**Sorun:** Null olan bir değişkeni, null olmayan bir değişken gibi kullanmaya çalışırsanız → **çökme!**

### 2.2 Rust'ın Çözümü: `Option<T>`

Rust'ta **`null` diye bir şey yoktur.** Bunun yerine standart kütüphanenin tanımladığı şu enum kullanılır:

```rust
enum Option<T> {
    Some(T),   // değer VAR ve içinde T tipinde bir şey taşıyor
    None,      // değer YOK
}
```

> 🎯 Bu enum o kadar önemlidir ki, **prelude**'a (otomatik içe aktarılan modüle) dahil edilmiştir. `Some` ve `None`'u doğrudan, `Option::` ön eki olmadan kullanabilirsiniz.

### 2.3 Tip Güvenliği Nasıl Çalışıyor?

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

// ❌ HATA! Derlenmez:
let sum = x + y;
```

**Hata mesajı:**
```
error[E0277]: cannot add `Option<i8>` to `i8`
```

Çünkü `i8` ile `Option<i8>` **farklı tiplerdir.** Derleyici sizi zorlar:

> "Eğer `Option<i8>` kullanıyorsan, önce `None` olup olmadığını kontrol etmek **zorundasın**."

Bu sayede Rust, **null kontrolü yapmayı unutma ihtimalinizi sıfıra indirir.**

### 2.4 `Option<T>` Kullanım Örnekleri

```rust
fn bul(dizi: &[i32], hedef: i32) -> Option<usize> {
    for (i, &deger) in dizi.iter().enumerate() {
        if deger == hedef {
            return Some(i);  // Buldum, indeksini veriyorum
        }
    }
    None  // Bulamadım
}

fn main() {
    let sayilar = [10, 20, 30, 40];
    
    match bul(&sayilar, 30) {
        Some(i) => println!("30 sayısı {} indeksinde", i),
        None    => println!("30 sayısı dizide yok"),
    }
}
```

---

## 📖 Bölüm 3 — `match`: Rust'ın En Güçlü Kontrol Akışı

### 3.1 `match` Nedir?

`match`, bir değeri bir dizi **desenle (pattern)** karşılaştırır ve eşleşen ilk desene ait kodu çalıştırır.

> 🪙 **Madeni Para Sıralama Makinesi Analojisi:** Bir madeni para bandın üzerinde kayar; boyutuna uygun ilk deliğe düşer. `match` da aynı şekilde, değer sırayla her desene girer ve uyan ilk desende kalır.

### 3.2 Temel Kullanım

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny  => 1,
        Coin::Nickel => 5,
        Coin::Dime   => 10,
        Coin::Quarter => 25,
    }
}
```

**`match` yapısının anatomisi:**
```
match DEGER {
    DESEN => KOD,
    DESEN => KOD,
    ...
}
```

- `if`'ten **en büyük farkı:** `if` sadece `bool` ile çalışır, `match` **herhangi bir tip** ile çalışır.
- Her kol (arm) bir **desen** ve bir **koddan** oluşur.
- Kollar `=>` ile ayrılır, birbirinden `,` ile ayrılır.

### 3.3 Desenden Değer Çıkarma (Binding)

Varyantların içindeki verilere erişmek için desen değişkeni kullanılır:

```rust
#[derive(Debug)]
enum UsState { Alabama, Alaska, /* ... */ }

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

Burada `state` değişkeni, `Quarter`'ın içindeki `UsState` değerine **bağlanır (bind).**

### 3.4 `Option<T>` ile `match` Kullanımı

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six  = plus_one(five);     // Some(6)
let none = plus_one(None);     // None
```

### 3.5 ⚠️ Exhaustive (Kapsayıcı) Olma Zorunluluğu

Rust'ın `match`'teki **en güçlü özelliği:** Tüm olası durumları kapsamanız **zorunludur.**

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        // ❌ HATA: None durumu unutuldu!
    }
}
```

**Derleyici hatası:**
```
error[E0004]: non-exhaustive patterns: `None` not covered
```

Bu özellik sayesinde **hiçbir senaryo unutulamaz.** Null kontrolü derleme zamanında garanti altına alınır.

### 3.6 Catch-All (Her Şeyi Yakala) Deseni

Bazen tüm durumları tek tek yazmak istemezsiniz. İki yol var:

#### a) Değişken ile catch-all

```rust
fn roll_zar(deger: u8) {
    match deger {
        3 => hareket_et(3),
        7 => hareket_et(7),
        other => hareket_et(other),  // diğer tüm değerler
    }
}
```

#### b) `_` wildcard (değeri kullanmayacaksanız)

```rust
let para = Coin::Penny;
let mut sayac = 0;

match para {
    Coin::Quarter(state) => println!("Eyalet çeyreği: {:?}", state),
    _ => sayac += 1,  // değeri kullanmıyoruz, sadece sayıyoruz
}
```

> ⚠️ **Sıralama kuralı:** Catch-all kolu **en sona** konmalıdır. Aksi halde sonraki kollar asla çalışmaz ve Rust bunu uyarı olarak bildirir.

---

## 📖 Bölüm 4 — `if let`: `match`'in Kısa Yazımı

### 4.1 Sorun: Sadece Bir Durumla İlgileniyorsanız

Bazen `match` ile sadece **tek bir durumu** kontrol etmek istersiniz, diğerleri önemsizdir:

```rust
let config_max = Some(3u8);

match config_max {
    Some(max) => println!("Maksimum değer: {max}"),
    _ => (),   // 😒 Gereksiz boilerplate
}
```

`_ => ()` yazmak can sıkıcıdır. İşte `if let` burada devreye girer.

### 4.2 `if let` Sözdizimi

```rust
let config_max = Some(3u8);

if let Some(max) = config_max {
    println!("Maksimum değer: {max}");
}
```

**Sözdizimi:**
```
if let DESEN = IFDE {
    // desen eşleşirse çalışır
}
```

- `=` işareti atama değil, **desen eşleştirme** anlamına gelir.
- `if let`, aslında `match`'in **sözdizimsel şekeridir (syntax sugar).**

### 4.3 `if let` + `else`

`match`'teki `_` koluna karşılık olarak `else` kullanabilirsiniz:

```rust
let coin = Coin::Penny;
let mut count = 0;

if let Coin::Quarter(state) = coin {
    println!("Eyalet çeyreği: {:?}!", state);
} else {
    count += 1;
}
```

### 4.4 `let...else`: Modern Rust'ın Güzel Yapısı

Rust, fonksiyonun "mutlu yolu" (happy path) üzerinde kalmak için `let...else` yapısını sunar:

```rust
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;  // eşleşmezse hemen dön
    };
    
    // Buradan sonra `state` kullanıma hazır!
    if state.existed_in(1900) {
        Some(format!("{:?} oldukça eski!", state))
    } else {
        Some(format!("{:?} nispeten yeni.", state))
    }
}
```

> 💎 **Avantajı:** İç içe `if` bloklarına girmeden, erken dönüş (early return) ile kodu düz ve okunaklı tutarsınız.

### 4.5 `match` mi `if let` mi?

| Durum | Tercih |
|---|---|
| Birden fazla durumu kontrol ediyorsanız | ✅ `match` |
| Tüm durumları kapsadığınızdan emin olmak istiyorsanız | ✅ `match` |
| Sadece **tek bir** durumu kontrol ediyorsanız | ✅ `if let` |
| Kodun kısa ve öz olmasını istiyorsanız | ✅ `if let` |
| Erken dönüş (early return) yapıyorsanız | ✅ `let...else` |

---

## 🎯 Bölüm 5 — Özet ve Pratik İpuçları

### 5.1 Enum'ların Gücü

1. **Tip güvenliği:** Derleyici, geçersiz durumları derleme zamanında yakalar.
2. **Veri taşıyabilir:** Her varyant farklı tip ve miktarda veri barındırabilir.
3. **Metod alabilir:** `impl` bloğu ile enum'a metod eklenebilir.
4. **Genel (generic) olabilir:** `Option<T>`, `Result<T, E>` gibi.

### 5.2 Rust'ta "Null" Yoktur

- `null` yerine **`Option<T>`** kullanılır.
- `Some(değer)` → değer var
- `None` → değer yok
- `Option<T>` ile `T` **farklı tiplerdir**, karıştırılamaz.

### 5.3 `match` Kuralları

- **Exhaustive olmak zorundadır** — tüm durumlar kapsanmalı.
- Desenden **değer çıkarılabilir** (binding).
- `_` veya değişken ile **catch-all** yapılabilir.
- Catch-all **en sona** yazılmalıdır.

### 5.4 `if let` ve `let...else`

- `if let` → tek bir desenle ilgilendiğinizde kısa yol.
- `else` bloğu ile diğer durumlar işlenebilir.
- `let...else` → erken dönüş için modern ve temiz çözüm.

---

## 🧪 Bölüm 6 — Pratik Örnek: Adım Motoru Kontrol Enum'u

Kendi projenizden bir örnekle pekiştirelim:

```rust
#[derive(Debug)]
enum MotorKomutu {
    IleriAdim(u32),           // kaç adım ileri
    GeriAdim(u32),            // kaç adım geri
    HizarAyarla(u16),         // RPM değeri
    AcilDurdur,               // veri yok
    PozisyonaGit { x: i32, y: i32 },  // isimli alanlar
}

fn komutu_isle(komut: MotorKomutu) {
    match komut {
        MotorKomutu::IleriAdim(n) => println!("{} adım ileri", n),
        MotorKomutu::GeriAdim(n)  => println!("{} adım geri", n),
        MotorKomutu::HizarAyarla(rpm) => println!("Hız: {} RPM", rpm),
        MotorKomutu::AcilDurdur => println!("🛑 ACİL DURDURMA!"),
        MotorKomutu::PozisyonaGit { x, y } => {
            println!("Hedef pozisyon: ({}, {})", x, y);
        }
    }
}

fn main() {
    let k1 = MotorKomutu::IleriAdim(100);
    let k2 = MotorKomutu::AcilDurdur;
    let k3 = MotorKomutu::PozisyonaGit { x: 500, y: -200 };
    
    komutu_isle(k1);
    komutu_isle(k2);
    komutu_isle(k3);
}
```

---

## 📝 Son Söz

Enum'lar Rust'ın **tip sisteminin kalbidir.** `match` ile birlikte kullanıldığında:

- ✅ Derleme zamanında güvenlik
- ✅ Çalışma zamanında sıfır maliyet
- ✅ Okunabilir ve bakımı kolay kod

sağlarlar. Rust'ı diğer dillerden ayıran en önemli özelliklerden biri, bu **enum + pattern matching** ikilisinin gücüdür.

> 🚀 **Bir sonraki adım:** Bölüm 19'da daha gelişmiş desen eşleştirme tekniklerini (guards, `@` binding, nested patterns vb.) inceleyebilirsiniz.

Başarılar! 🦀
</parameter
</think>

# 🦀 Rust'ta Enum'lar (Numaralandırmalar) — Kapsamlı Ders

> **Kaynak:** *The Rust Programming Language* — Bölüm 6  
> Bu ders; enum tanımlama, `Option<T>`, `match` ve `if let` yapılarını adım adım, örneklerle ve derinlemesine açıklamaktadır.

---

## 📖 Bölüm 1 — Enum Nedir? Neden İhtiyacımız Var?

### 1.1 Temel Motivasyon

Diyelim ki bir IP adresi ile çalışıyorsunuz. IP adresleri dünyada sadece **iki standartta** bulunur:

- **IPv4** (ör. `127.0.0.1`)
- **IPv6** (ör. `::1`)

Bir IP adresi **ya IPv4'tür ya da IPv6'dır**; aynı anda ikisi birden olamaz. İşte tam bu noktada **enum** (enumeration = numaralandırma) devreye girer.

> 💡 **Anahtar Fikir:** `struct`'lar birbiriyle ilişkili alanları **bir arada** gruplarken (ör. `Rectangle { width, height }`), `enum`'lar bir değerin **bir dizi olası varyanttan sadece biri** olabileceğini ifade eder.

### 1.2 En Basit Enum Tanımı

```rust
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six  = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

fn route(ip_kind: IpAddrKind) {
    // ...
}
```

Dikkat edilmesi gereken noktalar:

| Özellik | Açıklama |
|---|---|
| `enum` anahtar kelimesi | Numaralandırma tanımlamak için kullanılır |
| `V4`, `V6` | Enum'un **varyantlarıdır** (variant) |
| `IpAddrKind::V4` | Varyantlara erişim için `::` operatörü kullanılır |
| Tip güvenliği | Hem `V4` hem `V6`, aynı `IpAddrKind` tipindedir |

### 1.3 Struct ile Birleştirme (Eski Yöntem)

Varyantlara veri eklemek isterseniz, enum'u struct içine gömebilirsiniz:

```rust
enum IpAddrKind { V4, V6 }

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};
```

Ancak bu **gereksiz dolambaçlı** bir yoldur. Rust bunu çok daha zarif yapmamıza izin verir.

### 1.4 Veriyi Doğrudan Enum Varyantına Gömmek ✨

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home     = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
```

> 🔑 **Çok Önemli Kural:** Enum varyantının adı, aynı zamanda bir **constructor (yapıcı) fonksiyon** olarak otomatik tanımlanır. `IpAddr::V4("...".to_string())` çağrısı, `IpAddr` tipinde bir değer döndürür.

### 1.5 Her Varyant Farklı Tip Taşıyabilir!

Struct ile bunu yapamazsınız, ama enum ile çok kolaydır:

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),   // 4 ayrı byte
    V6(String),            // tek bir string
}

let home     = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

### 1.6 Varyantların İç Yapısı Çok Esnektir

Aynı enum içinde farklı yapılar bir arada olabilir:

```rust
enum Message {
    Quit,                              // veri yok
    Move { x: i32, y: i32 },          // struct gibi isimli alanlar
    Write(String),                     // tek bir String
    ChangeColor(i32, i32, i32),        // üç tane i32
}

fn main() {
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 10, y: 30 };
    let m3 = Message::Write(String::from("Merhaba!"));
    let m4 = Message::ChangeColor(255, 128, 0);
}
```

### 1.7 Enum'lara Metod Tanımlamak (`impl`)

Tıpkı `struct`'larda olduğu gibi, `impl` bloğu ile enum'lara metod ekleyebilirsiniz:

```rust
impl Message {
    fn call(&self) {
        // Bu metod, üzerinde çağrıldığı değeri işler
        println!("Mesaj çağrıldı!");
    }
}

let m = Message::Write(String::from("selam"));
m.call();
```

---

## 📖 Bölüm 2 — `Option<T>` Enum'u: Null'un Yerine Ne Geçti?

### 2.1 "Milyar Dolarlık Hata" — Null Problemi

Null referansının mucidi **Tony Hoare**, 2009'da şöyle demiştir:

> *"Buna milyar dolarlık hatam diyorum. Null referansı koymaya direnemedim çünkü uygulanması çok kolaydı. Bu, son 40 yılda sayısız hata, güvenlik açığı ve sistem çöküşüne yol açtı."*

**Sorun:** Null olan bir değişkeni, null olmayan bir değişken gibi kullanmaya çalışırsanız → **çökme!**

### 2.2 Rust'ın Çözümü: `Option<T>`

Rust'ta **`null` diye bir şey yoktur.** Bunun yerine standart kütüphanenin tanımladığı şu enum kullanılır:

```rust
enum Option<T> {
    Some(T),   // değer VAR ve içinde T tipinde bir şey taşıyor
    None,      // değer YOK
}
```

> 🎯 Bu enum o kadar önemlidir ki, **prelude**'a (otomatik içe aktarılan modüle) dahil edilmiştir. `Some` ve `None`'u doğrudan, `Option::` ön eki olmadan kullanabilirsiniz.

### 2.3 Tip Güvenliği Nasıl Çalışıyor?

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

// ❌ HATA! Derlenmez:
let sum = x + y;
```

**Hata mesajı:**
```
error[E0277]: cannot add `Option<i8>` to `i8`
```

Çünkü `i8` ile `Option<i8>` **farklı tiplerdir.** Derleyici sizi zorlar:

> "Eğer `Option<i8>` kullanıyorsan, önce `None` olup olmadığını kontrol etmek **zorundasın**."

Bu sayede Rust, **null kontrolü yapmayı unutma ihtimalinizi sıfıra indirir.**

### 2.4 `Option<T>` Kullanım Örnekleri

```rust
fn bul(dizi: &[i32], hedef: i32) -> Option<usize> {
    for (i, &deger) in dizi.iter().enumerate() {
        if deger == hedef {
            return Some(i);  // Buldum, indeksini veriyorum
        }
    }
    None  // Bulamadım
}

fn main() {
    let sayilar = [10, 20, 30, 40];
    
    match bul(&sayilar, 30) {
        Some(i) => println!("30 sayısı {} indeksinde", i),
        None    => println!("30 sayısı dizide yok"),
    }
}
```

---

## 📖 Bölüm 3 — `match`: Rust'ın En Güçlü Kontrol Akışı

### 3.1 `match` Nedir?

`match`, bir değeri bir dizi **desenle (pattern)** karşılaştırır ve eşleşen ilk desene ait kodu çalıştırır.

> 🪙 **Madeni Para Sıralama Makinesi Analojisi:** Bir madeni para bandın üzerinde kayar; boyutuna uygun ilk deliğe düşer. `match` da aynı şekilde, değer sırayla her desene girer ve uyan ilk desende kalır.

### 3.2 Temel Kullanım

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny  => 1,
        Coin::Nickel => 5,
        Coin::Dime   => 10,
        Coin::Quarter => 25,
    }
}
```

**`match` yapısının anatomisi:**
```
match DEGER {
    DESEN => KOD,
    DESEN => KOD,
    ...
}
```

- `if`'ten **en büyük farkı:** `if` sadece `bool` ile çalışır, `match` **herhangi bir tip** ile çalışır.
- Her kol (arm) bir **desen** ve bir **koddan** oluşur.
- Kollar `=>` ile ayrılır, birbirinden `,` ile ayrılır.

### 3.3 Desenden Değer Çıkarma (Binding)

Varyantların içindeki verilere erişmek için desen değişkeni kullanılır:

```rust
#[derive(Debug)]
enum UsState { Alabama, Alaska, /* ... */ }

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

Burada `state` değişkeni, `Quarter`'ın içindeki `UsState` değerine **bağlanır (bind).**

### 3.4 `Option<T>` ile `match` Kullanımı

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six  = plus_one(five);     // Some(6)
let none = plus_one(None);     // None
```

### 3.5 ⚠️ Exhaustive (Kapsayıcı) Olma Zorunluluğu

Rust'ın `match`'teki **en güçlü özelliği:** Tüm olası durumları kapsamanız **zorunludur.**

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        // ❌ HATA: None durumu unutuldu!
    }
}
```

**Derleyici hatası:**
```
error[E0004]: non-exhaustive patterns: `None` not covered
```

Bu özellik sayesinde **hiçbir senaryo unutulamaz.** Null kontrolü derleme zamanında garanti altına alınır.

### 3.6 Catch-All (Her Şeyi Yakala) Deseni

Bazen tüm durumları tek tek yazmak istemezsiniz. İki yol var:

#### a) Değişken ile catch-all

```rust
fn roll_zar(deger: u8) {
    match deger {
        3 => hareket_et(3),
        7 => hareket_et(7),
        other => hareket_et(other),  // diğer tüm değerler
    }
}
```

#### b) `_` wildcard (değeri kullanmayacaksanız)

```rust
let para = Coin::Penny;
let mut sayac = 0;

match para {
    Coin::Quarter(state) => println!("Eyalet çeyreği: {:?}", state),
    _ => sayac += 1,  // değeri kullanmıyoruz, sadece sayıyoruz
}
```

> ⚠️ **Sıralama kuralı:** Catch-all kolu **en sona** konmalıdır. Aksi halde sonraki kollar asla çalışmaz ve Rust bunu uyarı olarak bildirir.

---

## 📖 Bölüm 4 — `if let`: `match`'in Kısa Yazımı

### 4.1 Sorun: Sadece Bir Durumla İlgileniyorsanız

Bazen `match` ile sadece **tek bir durumu** kontrol etmek istersiniz, diğerleri önemsizdir:

```rust
let config_max = Some(3u8);

match config_max {
    Some(max) => println!("Maksimum değer: {max}"),
    _ => (),   // 😒 Gereksiz boilerplate
}
```

`_ => ()` yazmak can sıkıcıdır. İşte `if let` burada devreye girer.

### 4.2 `if let` Sözdizimi

```rust
let config_max = Some(3u8);

if let Some(max) = config_max {
    println!("Maksimum değer: {max}");
}
```

**Sözdizimi:**
```
if let DESEN = IFDE {
    // desen eşleşirse çalışır
}
```

- `=` işareti atama değil, **desen eşleştirme** anlamına gelir.
- `if let`, aslında `match`'in **sözdizimsel şekeridir (syntax sugar).**

### 4.3 `if let` + `else`

`match`'teki `_` koluna karşılık olarak `else` kullanabilirsiniz:

```rust
let coin = Coin::Penny;
let mut count = 0;

if let Coin::Quarter(state) = coin {
    println!("Eyalet çeyreği: {:?}!", state);
} else {
    count += 1;
}
```

### 4.4 `let...else`: Modern Rust'ın Güzel Yapısı

Rust, fonksiyonun "mutlu yolu" (happy path) üzerinde kalmak için `let...else` yapısını sunar:

```rust
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;  // eşleşmezse hemen dön
    };
    
    // Buradan sonra `state` kullanıma hazır!
    if state.existed_in(1900) {
        Some(format!("{:?} oldukça eski!", state))
    } else {
        Some(format!("{:?} nispeten yeni.", state))
    }
}
```

> 💎 **Avantajı:** İç içe `if` bloklarına girmeden, erken dönüş (early return) ile kodu düz ve okunaklı tutarsınız.

### 4.5 `match` mi `if let` mi?

| Durum | Tercih |
|---|---|
| Birden fazla durumu kontrol ediyorsanız | ✅ `match` |
| Tüm durumları kapsadığınızdan emin olmak istiyorsanız | ✅ `match` |
| Sadece **tek bir** durumu kontrol ediyorsanız | ✅ `if let` |
| Kodun kısa ve öz olmasını istiyorsanız | ✅ `if let` |
| Erken dönüş (early return) yapıyorsanız | ✅ `let...else` |

---

## 🎯 Bölüm 5 — Özet ve Pratik İpuçları

### 5.1 Enum'ların Gücü

1. **Tip güvenliği:** Derleyici, geçersiz durumları derleme zamanında yakalar.
2. **Veri taşıyabilir:** Her varyant farklı tip ve miktarda veri barındırabilir.
3. **Metod alabilir:** `impl` bloğu ile enum'a metod eklenebilir.
4. **Genel (generic) olabilir:** `Option<T>`, `Result<T, E>` gibi.

### 5.2 Rust'ta "Null" Yoktur

- `null` yerine **`Option<T>`** kullanılır.
- `Some(değer)` → değer var
- `None` → değer yok
- `Option<T>` ile `T` **farklı tiplerdir**, karıştırılamaz.

### 5.3 `match` Kuralları

- **Exhaustive olmak zorundadır** — tüm durumlar kapsanmalı.
- Desenden **değer çıkarılabilir** (binding).
- `_` veya değişken ile **catch-all** yapılabilir.
- Catch-all **en sona** yazılmalıdır.

### 5.4 `if let` ve `let...else`

- `if let` → tek bir desenle ilgilendiğinizde kısa yol.
- `else` bloğu ile diğer durumlar işlenebilir.
- `let...else` → erken dönüş için modern ve temiz çözüm.

---

## 🧪 Bölüm 6 — Pratik Örnek: Adım Motoru Kontrol Enum'u

Kendi projenizden bir örnekle pekiştirelim:

```rust
#[derive(Debug)]
enum MotorKomutu {
    IleriAdim(u32),           // kaç adım ileri
    GeriAdim(u32),            // kaç adım geri
    HizarAyarla(u16),         // RPM değeri
    AcilDurdur,               // veri yok
    PozisyonaGit { x: i32, y: i32 },  // isimli alanlar
}

fn komutu_isle(komut: MotorKomutu) {
    match komut {
        MotorKomutu::IleriAdim(n) => println!("{} adım ileri", n),
        MotorKomutu::GeriAdim(n)  => println!("{} adım geri", n),
        MotorKomutu::HizarAyarla(rpm) => println!("Hız: {} RPM", rpm),
        MotorKomutu::AcilDurdur => println!("🛑 ACİL DURDURMA!"),
        MotorKomutu::PozisyonaGit { x, y } => {
            println!("Hedef pozisyon: ({}, {})", x, y);
        }
    }
}

fn main() {
    let k1 = MotorKomutu::IleriAdim(100);
    let k2 = MotorKomutu::AcilDurdur;
    let k3 = MotorKomutu::PozisyonaGit { x: 500, y: -200 };
    
    komutu_isle(k1);
    komutu_isle(k2);
    komutu_isle(k3);
}
```

---

## 📝 Son Söz

Enum'lar Rust'ın **tip sisteminin kalbidir.** `match` ile birlikte kullanıldığında:

- ✅ Derleme zamanında güvenlik
- ✅ Çalışma zamanında sıfır maliyet
- ✅ Okunabilir ve bakımı kolay kod

sağlarlar. Rust'ı diğer dillerden ayıran en önemli özelliklerden biri, bu **enum + pattern matching** ikilisinin gücüdür.

> 🚀 **Bir sonraki adım:** Bölüm 19'da daha gelişmiş desen eşleştirme tekniklerini (guards, `@` binding, nested patterns vb.) inceleyebilirsiniz.

Başarılar! 🦀