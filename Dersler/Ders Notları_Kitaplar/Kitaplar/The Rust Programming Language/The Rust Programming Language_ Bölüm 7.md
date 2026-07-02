# 🦀 Ders Notları: Bölüm 7 - Rust'ta Modül Sistemi — Ders Notları

> **Bölüm 7: Paketler, Crateler ve Modüller ile Büyüyen Projeleri Yönetmek**

---

## 🎯 Giriş: Neden Bu Sisteme İhtiyacımız Var?

Arkadaşlar, şimdiye kadar yazdığımız Rust programları tek bir dosyada, tek bir modül içindeydi. Ama projeler büyüdükçe bu yaklaşım sürdürülemez hale gelir. Tıpkı kütüphanedeki binlerce kitabı rastgele bir odaya yığmak gibi — ne aradığınızı bulabilirsiniz ne de düzeni koruyabilirsiniz.

Rust'ın **modül sistemi** size şu olanakları sağlar:

| Kavram | Görevi |
|--------|--------|
| **Paket (Package)** | Crateleri derlemek, test etmek ve paylaşmak |
| **Crate** | Modüllerden oluşan bir ağaç; kütüphane ya da çalıştırılabilir çıktı üretir |
| **Modül (Module)** | Kodun organizasyonunu, kapsamını ve gizliliğini kontrol eder |
| **Yol (Path)** | Bir struct, fonksiyon veya modüle isimlendirme yoluyla erişim sağlar |

Bu kavramlar birbirine sıkı sıkıya bağlıdır. Bir binayı düşünün:
- **Paket** → Bina
- **Crate** → Daireler (ya da iş yerleri)
- **Modül** → Odalar
- **Yol** → Adres tarifi ("3. daire, 2. oda, masa üstü")

---

## 📦 Bölüm 7.1 — Paketler ve Crateler

### Crate Nedir?

**Crate**, Rust derleyicisinin dikkate aldığı **en küçük kod birimidir**. Tek bir `.rs` dosyası bile bir cratedir. Crateler iki türlüdür:

#### 1️⃣ Binary Crate (İkili/Çalıştırılabilir Crate)
- Derlendiğinde **çalıştırılabilir bir dosya** üretir (komut satırı aracı, sunucu vb.)
- Mutlaka bir `main` fonksiyonu içermelidir
- `cargo new` ile oluşturduğunuz projeler default olarak binary cratedir

```rust
// src/main.rs — bir binary crate'in kökü
fn main() {
    println!("Merhaba dünya!");
}
```

#### 2️⃣ Library Crate (Kütüphane Crate'i)
- `main` fonksiyonu **yoktur** ve çalıştırılabilir dosya üretmez
- Başka projelerin **kullanması için** fonksiyonellik tanımlar
- `cargo new --lib` ile oluşturulur
- Örneğin `rand` crate'i bir library crate'tir

```rust
// src/lib.rs — bir library crate'in kökü
pub fn selamla(isim: &str) -> String {
    format!("Merhaba, {}!", isim)
}
```

> 💡 **Rustçılar "crate" dediklerinde genelde library crate'i kastediyorlar.**

### Crate Root (Crate Kökü) Nedir?

Derleyicinin **başlangıç noktası** olan kaynak dosyadır. Crate'in kök modülünü oluşturur:
- Binary crate için → `src/main.rs`
- Library crate için → `src/lib.rs`

### Package (Paket) Nedir?

**Paket**, bir veya birden fazla crate'i bir arada tutan ve işlevsellik sağlayan bir bütündür. `Cargo.toml` dosyası paketin nasıl derleneceğini tanımlar.

#### Paket Kuralları:
```
my-project/
├── Cargo.toml      ← Bu dosya bir pakettir
└── src/
    ├── main.rs     ← Binary crate kökü (paket adıyla aynı isimde)
    └── lib.rs      ← Library crate kökü (paket adıyla aynı isimde)
```

- Bir paket **en az bir crate** içermelidir (binary veya library)
- Bir paket **birden fazla binary crate** içerebilir
- Bir paket **en fazla bir library crate** içerebilir

#### Paket Oluşturma:

```bash
$ cargo new my-project
     Created binary (application) `my-project` package
```

Bu komut şunları yapar:
1. `Cargo.toml` → Paket dosyası
2. `src/main.rs` → Binary crate kökü (Cargo bunu otomatik bilir)

Eğer `src/lib.rs` de eklerseniz, paket hem binary hem de library crate'e sahip olur.

#### Birden Fazla Binary Crate:

`src/bin/` dizinine koyduğunuz **her dosya** ayrı bir binary crate olur:

```
my-project/
├── Cargo.toml
└── src/
    ├── main.rs          ← Ana binary (paket adıyla aynı)
    ├── lib.rs           ← Library crate (opsiyonel)
    └── bin/
        ├── araci1.rs    ← İkinci binary crate
        └── araci2.rs    ← Üçüncü binary crate
```

---

## 🏗️ Bölüm 7.2 — Modülleri Tanımlama: Kapsam ve Gizlilik Kontrolü

### Modül Ne İşe Yarar?

Modüller, bir crate içindeki kodu **okunabilirlik** ve **yeniden kullanılabilirlik** için düzenler. Aynı zamanda **gizlilik (privacy)** kontrolü sağlar — bazı şeyleri dış dünyaya kapatır, bazılarını açar.

### Restoran Metaforu 🍽️

Rust kitabının harika bir metaforu var: Bir restoran düşünün.

- **Front of house (Salon kısmı):** Hostesler, garsonlar, kasiyerler → Müşterilerin gördüğü kısım
- **Back of house (Mutfak kısmı):** Şefler, bulaşıkçılar, müdürler → Arka plan, gizli kısım

Bu mantığı koda uygulayalım:

```rust
// src/lib.rs
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
```

### Modül Ağacı (Module Tree)

Yukarıdaki kod şu ağacı oluşturur:

```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

> 💡 **Dosya sistemi metaforu:** Modüller = klasörler, fonksiyonlar = dosyalar. Tıpkı dizin ağacı gibi!

### Temel Kurallar (Hızlı Referans)

| Kural | Açıklama |
|-------|----------|
| **Crate kökünden başla** | Derleyici önce `src/lib.rs` veya `src/main.rs`'e bakar |
| **Modül bildirimi** | `mod bahce;` → Derleyici kodu `src/bahce.rs` veya `src/bahce/mod.rs` dosyasında arar |
| **Alt modül bildirimi** | `src/bahce.rs` içinde `mod sebzeler;` → Kod `src/bahce/sebzeler.rs` dosyasında aranır |
| **Varsayılan gizlilik** | Tüm öğeler **private**'dır (sadece kendi modülünden ve alt modüllerinden erişilebilir) |
| **`pub` anahtar kelimesi** | Bir öğeyi **public** yapar |

### Gizlilik Kuralları — Çok Önemli! ⚠️

Rust'ta **her şey varsayılan olarak private'dır.**

```rust
mod on_yuz {
    mod hostes {
        fn siraya_ekle() {}  // ← PRIVATE!
    }
}

pub fn restoranda_yemek_ye() {
    on_yuz::hostes::siraya_ekle();  // ❌ HATA! hostes modülü private
}
```

**Gizlilik kuralları şöyle çalışır:**
- 🔒 **Child → Parent:** Alt modüller, üst modüllerdeki **private** öğelere erişebilir
- 🔒 **Parent → Child:** Üst modüller, alt modüllerdeki **private** öğelere **erişemez** (ancak `pub` ile açılabilirse)
- 🔒 **Sibling → Sibling:** Kardeş modüller birbirinin private öğelerine erişemez

> 💡 **Restoran benzetmesi:** Mutfaktaki (back of house) detaylar müşterilerden gizlidir, ama müdür mutfağın her şeyini görebilir. Mutfak personeli de restoranın genelini görebilir.

---

## 🛤️ Bölüm 7.3 — Modül Ağacında Bir Öğeye Başvurmak: Yollar (Paths)

Bir fonksiyonu çağırmak için onun **adresini (path)** bilmemiz gerekir. İki tür yol vardır:

### 1️⃣ Absolute Path (Mutlak Yol)
- **Crate kökünden** başlar
- `crate::` ile başlar (mevcut crate için)
- Dosya sistemindeki `/` gibidir: `/home/kullanici/belge.txt`

### 2️⃣ Relative Path (Göreceli Yol)
- **Mevcut modülden** başlar
- Modül adıyla, `self` veya `super` ile başlar
- Dosya sistemindeki `./` veya `../` gibidir

### Örnek:

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Mutlak yol: crate kökünden başlıyor
    crate::front_of_house::hosting::add_to_waitlist();

    // Göreceli yol: mevcut modülden başlıyor
    front_of_house::hosting::add_to_waitlist();
}
```

> 💡 **Hangisini kullanmalı?** Genellikle **mutlak yol** tercih edilir, çünkü kodun yerini değiştirdiğinizde daha az şey bozulur.

### `pub` Anahtar Kelimesi — Gizliliği Açmak

Tüm öğeler varsayılan olarak private'dır. Dışarıdan erişilmesini istiyorsanız `pub` kullanmalısınız.

#### Modülleri Public Yapmak:

```rust
mod front_of_house {
    pub mod hosting {  // ← modülü public yaptık
        fn add_to_waitlist() {}  // ← ama fonksiyon hala private!
    }
}
```

⚠️ **Dikkat:** Bir modülü `pub` yapmak, **içindeki öğeleri otomatik olarak public yapmaz!** Her öğeyi ayrı ayrı `pub` yapmalısınız:

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}  // ← Fonksiyonu da public yaptık ✅
    }
}
```

### `super` ile Göreceli Yol

`super`, dosya sistemindeki `..` (üst dizin) gibidir. **Üst modüle** çıkmak için kullanılır:

```rust
fn deliver_order() {}  // crate kökünde

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();  // ← Üst modüle çık ve deliver_order'ı çağır
    }

    fn cook_order() {}
}
```

> 💡 **Ne zaman `super` kullanmalı?** Alt modül, üst modülle yakından ilişkiliyse ve ikisinin birlikte taşınma ihtimali yüksekse `super` kullanmak iyidir. Böylece modül ağacını yeniden düzenlediğinizde daha az şeyi güncellemeniz gerekir.

### Struct ve Enum'lar için `pub`

#### Struct'lar:
```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,           // ← public alan
        seasonal_fruit: String,      // ← private alan (müşteri seçemez!)
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");  // ✅ toast public, değiştirebiliriz
    // meal.seasonal_fruit = String::from("blueberries");  // ❌ private!
}
```

> ⚠️ **Struct public olsa bile alanları (fields) private kalır!** Her alanı ayrı ayrı `pub` yapmalısınız.

#### Enum'lar:
```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;   // ✅ Varyantlar otomatik public
    let order2 = back_of_house::Appetizer::Salad;  // ✅
}
```

> 💡 **Enum public ise tüm varyantları otomatik olarak public'tir.** Struct'ın aksine, enum'lar varyantları public olmadan kullanışsız olacağı için Rust bu şekilde tasarlanmıştır.

---

## 🚀 Bölüm 7.4 — `use` Anahtar Kelimesi ile Yolları Kapsama Dahil Etme

Her seferinde uzun yollar yazmak (`crate::front_of_house::hosting::add_to_waitlist`) yorucu değil mi? `use` anahtar kelimesi tam burada devreye giriyor!

### Temel Kullanım

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;  // ← Hosting modülünü scope'a getirdik

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();  // ← Artık kısa yazabiliriz! ✅
}
```

> 💡 **Dosya sistemi benzetmesi:** `use`, dosya sisteminde **sembolik link** oluşturmak gibidir. Uzun yolu kısayola bağlarsınız.

### Fonksiyonlar vs. Diğer Öğeler — İdiomatic Kullanım

#### Fonksiyonlar için:
Fonksiyonları scope'a getirirken **üst modülü** getirmek daha idiomatiktir:

```rust
// ✅ İdiomatik: Modülü getir, fonksiyonu modülle birlikte çağır
use crate::front_of_house::hosting;
hosting::add_to_waitlist();

// ❌ İdiomatik değil: Fonksiyonu direkt getir
use crate::front_of_house::hosting::add_to_waitlist;
add_to_waitlist();  // ← Nereden geldiği belli değil!
```

> **Neden?** Fonksiyon çağrılırken üst modülün görünmesi, onun yerel olarak tanımlanmadığını açıkça gösterir.

#### Struct, Enum ve Diğer Öğeler için:
Tam yolunu getirmek idiomatiktir:

```rust
// ✅ İdiomatik: Tam yol
use std::collections::HashMap;

let mut map = HashMap::new();
```

### Aynı İsimli İki Öğeyi Scope'a Getirmek

Rust aynı scope'ta aynı isimde iki öğe olmasına izin vermez. İki çözüm var:

#### Çözüm 1: Üst modülle birlikte kullanmak

```rust
use std::fmt::Result;
use std::io::Result;

fn main() {
    let r1: Result = Ok(());  // std::fmt::Result
    let r2: Result = Ok(());  // std::io::Result — HATA! Hangi Result?
}
```

⚠️ Bu çalışmaz çünkü iki `Result` çakışır.

#### Çözüm 2: `as` ile takma ad (alias) vermek

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn main() {
    let r1: Result = Ok(());      // std::fmt::Result
    let r2: IoResult = Ok(());    // std::io::Result ✅
}
```

### `pub use` — Yeniden Dışa Aktarma (Re-exporting)

Bazen bir öğeyi kendi scope'unuza getirip, aynı zamanda dış dünyaya da açmak istersiniz:

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;  // ← Re-export!

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

Artık dışarıdaki kod şu yolların **her ikisini** de kullanabilir:

```rust
// Normal yol (iç yapıyı biliyorsa)
restaurant::front_of_house::hosting::add_to_waitlist();

// Re-export edilmiş kısa yol
restaurant::hosting::add_to_waitlist();  // ✅
```

> 💡 **Ne zaman kullanılır?** İç yapınız ile kullanıcıların düşündüğü yapı farklıysa `pub use` ile **farklı bir API yüzeyi** sunabilirsiniz.

### Nested Paths — İç İçe Yollar

Aynı prefix'e sahip birden fazla öğeyi tek satırda getirebilirsiniz:

```rust
// ❌ Uzun yol
use std::cmp::Ordering;
use std::io;

// ✅ Kısa yol (nested path)
use std::{cmp::Ordering, io};
```

Daha karmaşık örnekler:

```rust
use std::io::{self, Write};  // io ve io::Write'ı aynı anda getir
use std::fs::{self, File, OpenOptions};
```

### Glob Operatörü `*`

Bir yoldaki **tüm public öğeleri** scope'a getirmek için:

```rust
use std::collections::*;

// Artık HashMap, HashSet, BTreeMap vb. hepsi scope'ta
```

⚠️ **Dikkat:** Glob operatörü hangi isimlerin scope'ta olduğunu belirsizleştirir ve bağımlılık güncellemelerinde çakışmalara yol açabilir. **Spare kullanın!**

---

## 📁 Bölüm 7.5 — Modülleri Farklı Dosyalara Ayırmak

Şimdiye kadar tüm modülleri tek dosyada tanımladık. Modüller büyüdükçe onları **ayrı dosyalara** taşımak gerekir.

### Adım Adım: Modülü Dosyaya Taşımak

#### Başlangıç durumu (her şey `src/lib.rs` içinde):

```rust
// src/lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

#### Adım 1: `front_of_house` modülünü ayır

`src/lib.rs`:
```rust
mod front_of_house;  // ← Gövde yok, sadece bildirim

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

`src/front_of_house.rs` (yeni dosya):
```rust
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

> 💡 **Önemli:** `mod` bir "include" değildir! Derleyici `mod front_of_house;` gördüğünde, `src/front_of_house.rs` dosyasını arar ve bulursa o dosyadaki kodu **modül ağacına yerleştirir.**

#### Adım 2: `hosting` alt modülünü de ayır

`src/front_of_house.rs`:
```rust
pub mod hosting;  // ← Alt modül bildirimi
```

`src/front_of_house/hosting.rs` (yeni dosya):
```rust
pub fn add_to_waitlist() {}
```

### Dosya Yapısı:

```
restaurant/
├── Cargo.toml
├── Cargo.lock
└── src/
    ├── lib.rs                         ← Crate kökü
    ├── front_of_house.rs              ← front_of_house modülü
    └── front_of_house/
        └── hosting.rs                 ← hosting alt modülü
```

### Dosya Bulma Kuralları

Derleyici bir modül için şu dosyalara bakar:

| Modül | Aranacak Dosyalar |
|-------|-------------------|
| `mod garden;` (crate kökünde) | `src/garden.rs` veya `src/garden/mod.rs` |
| `mod vegetables;` (garden modülünde) | `src/garden/vegetables.rs` veya `src/garden/vegetables/mod.rs` |

### Eski Stil: `mod.rs`

Rust, eski bir dosya yapısı stilini de destekler:

```
src/
├── lib.rs
├── front_of_house/
│   ├── mod.rs           ← Eski stil (hala destekleniyor)
│   └── hosting.rs
```

⚠️ **Öneri:** Yeni projelerde `mod.rs` yerine **düz dosya adını** tercih edin (`front_of_house.rs`). Çünkü birden fazla `mod.rs` dosyası editörde kafa karıştırıcı olabilir.

### Tam Örnek: `backyard` Crate'i

```
backyard/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── garden.rs
    └── garden/
        └── vegetables.rs
```

`src/main.rs`:
```rust
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
```

`src/garden.rs`:
```rust
pub mod vegetables;
```

`src/garden/vegetables.rs`:
```rust
#[derive(Debug)]
pub struct Asparagus {}
```

---

## 🎓 Özet: Modül Sisteminin Mantığı

```
┌─────────────────────────────────────────────────────────────┐
│                        PACKAGE                               │
│  ┌───────────────────────────────────────────────────────┐  │
│  │                    CRATE                                │  │
│  │  ┌─────────────────────────────────────────────────┐  │  │
│  │  │              MODULE TREE                         │  │  │
│  │  │  crate                                           │  │  │
│  │  │   └── front_of_house (modül)                    │  │  │
│  │  │       ├── hosting (modül)                       │  │  │
│  │  │       │   └── add_to_waitlist (fonksiyon)      │  │  │
│  │  │       └── serving (modül)                       │  │  │
│  │  │           └── take_order (fonksiyon)            │  │  │
│  │  └─────────────────────────────────────────────────┘  │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

### Anahtar Kelimeler Cheat Sheet:

| Anahtar Kelime | Görevi |
|----------------|--------|
| `mod` | Modül tanımlar veya bildirir |
| `pub` | Bir öğeyi public yapar |
| `use` | Bir yolu mevcut scope'a getirir |
| `pub use` | Bir öğeyi re-export eder |
| `crate` | Mevcut crate'in kökünü ifade eder (mutlak yol için) |
| `self` | Mevcut modülü ifade eder |
| `super` | Üst modülü ifade eder |
| `as` | Bir öğeye takma ad verir |
| `*` | Glob operatörü — tüm public öğeleri getirir |

### Gizlilik Kuralları Özeti:

```
Varsayılan: HER ŞEY PRIVATE 🔒

✅ Child modüller, parent'ın private öğelerini GÖREBİLİR
❌ Parent modüller, child'ın private öğelerini GÖREMEZ (pub yoksa)
❌ Kardeş modüller birbirinin private öğelerini GÖREMEZ
```

### Dosya Yapısı Kuralları:

```
src/
├── main.rs (veya lib.rs)       ← Crate kökü
├── modül_adı.rs                ← Üst seviye modül
└── modül_adı/
    ├── alt_modül.rs            ← Alt modül
    └── başka_alt_modül.rs
```

---

## 🧪 Pratik İpuçları

1. **Küçük başlayın:** Başlangıçta her şeyi `main.rs` veya `lib.rs`'te tutun. Kod büyüdükçe modüllere ayırın.

2. **`pub`'ı minimum tutun:** Sadece gerçekten dışarıdan erişilmesi gereken şeyleri public yapın. Bu, gelecekte kodu değiştirmenizi kolaylaştırır.

3. **Mutlak yol tercih edin:** `crate::modül::öğe` şeklinde mutlak yol kullanmak, kodu taşıdığınızda daha az kırılmaya neden olur.

4. **`use` ile okunabilirliği artırın:** Uzun yolları tekrar tekrar yazmak yerine `use` ile kısayol oluşturun.

5. **Glob operatöründen kaçının:** `use std::collections::*` gibi kullanımlar hangi isimlerin scope'ta olduğunu belirsizleştirir.

---

Bu bölüm Rust'ın en temel ve en güçlü özelliklerinden birini oluşturuyor. Modül sistemini iyi anlamak, büyük ve sürdürülebilir Rust projeleri yazmanın anahtarıdır. 🦀

Bir sonraki adımda **Chapter 14: Cargo Workspaces** ile birden fazla crate'in birlikte nasıl yönetileceğini öğrenebilirsiniz. Başarılar!