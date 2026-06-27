# Rust'ta Modüller — Kapsamlı Ders Anlatımı

Rust'ın modül sistemi, kodunuzu mantıksal birimlere ayırmanızı, bu birimler arasında görünürlük (public/private) yönetimi yapmanızı sağlayan güçlü bir sistemdir. Bu derste, [Rust By Example](https://doc.rust-lang.org/rust-by-example/mod.html) kaynağındaki **Modüller** bölümünü ve tüm alt başlıklarını ders anlatır gibi detaylıca inceleyeceğiz.

---

## 1. Modüllere Giriş (`mod`)

Rust'ta bir **modül**, şu öğelerin bir araya geldiği bir koleksiyondur:
- Fonksiyonlar
- Struct'lar (yapılar)
- Trait'ler (özellikler)
- `impl` blokları
- Hatta başka modüller

Modüller, kodu **hiyerarşik** olarak mantıksal birimlere bölmek ve bu birimler arasındaki görünürlüğü (erişilebilirliği) yönetmek için kullanılır.

### Temel Syntax

```rust
mod modül_adı {
    // Modülün içeriği buraya gelir
    fn bir_fonksiyon() { /* ... */ }
    struct BirStruct { /* ... */ }
}
```

Modüller **iç içe** de tanımlanabilir:

```rust
mod dış_modül {
    mod iç_modül {
        fn bir_fonksiyon() { /* ... */ }
    }
}
```

Modüllerin en önemli özelliği, **aynı isimde** öğelerin farklı modüllerde bulunabilmesi ve böylece isim çakışmalarının önlenmesidir.

---

## 2. Görünürlük (Visibility)

> **Kural:** Rust'ta bir modülün içindeki öğeler **varsayılan olarak private (özel)** görünürlüğe sahiptir.

Yani, bir modülün içindeki fonksiyonlar, struct'lar vs. **sadece o modülün içinden** erişilebilir. Modül dışından erişilebilmesi için `pub` (public) anahtar kelimesi ile işaretlenmeleri gerekir.

### Detaylı Örnek

```rust
// my_mod adında bir modül
mod my_mod {

    // Varsayılan olarak private görünürlük
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    // pub ile public yapılır
    pub fn function() {
        println!("called `my_mod::function()`");
    }

    // Aynı modül içindeki öğeler, private olsalar bile birbirlerine erişebilir
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function(); // ✅ Aynı modül içinde olduğu için erişebilir
    }

    // Modüller iç içe olabilir
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }

        // pub(in path) — yalnızca belirtilen yol içinde görünür
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n> ");
            public_function_in_nested();
        }

        // pub(self) — yalnızca geçerli modülde görünür (private ile aynı)
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested()`");
        }

        // pub(super) — yalnızca üst (parent) modülde görünür
        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_function_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // pub(crate) — yalnızca geçerli crate içinde görünür
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()`");
    }

    // Private modül
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }

        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("called `my_mod::private_nested::restricted_function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

fn main() {
    // Aynı isimli fonksiyonlar farklı modüllerde bulunabilir
    function();
    my_mod::function();

    // Public öğelere modül dışından erişilebilir
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) öğeleri crate'in her yerinden çağrılabilir
    my_mod::public_function_in_crate();

    // ❌ HATA! pub(in path) ile sınırlandırılmış fonksiyon, belirtilen yol dışından erişilemez
    // my_mod::nested::public_function_in_my_mod();

    // ❌ HATA! Private fonksiyonlara modül dışından erişilemez
    // my_mod::private_function();
    // my_mod::nested::private_function();

    // ❌ HATA! Private modüllere erişilemez
    // my_mod::private_nested::function();
    // my_mod::private_nested::restricted_function();
}
```

### Görünürlük Seviyeleri Tablosu

| Anahtar Kelime | Anlamı |
|---|---|
| *(varsayılan)* | Yalnızca tanımlandığı modül içinde erişilebilir |
| `pub` | Herkes erişebilir (modülün kendisi public olduğu sürece) |
| `pub(crate)` | Yalnızca geçerli crate (proje) içinde erişilebilir |
| `pub(self)` | Yalnızca geçerli modülde erişilebilir (private ile aynı) |
| `pub(super)` | Yalnızca üst (parent) modülde erişilebilir |
| `pub(in crate::path)` | Yalnızca belirtilen yol (path) içinde erişilebilir |

> ⚠️ **Önemli Kural:** Bir öğenin public olması, onu kapsayan modülün de public olması gerektiği anlamına gelir. Private bir modülün içindeki `pub` öğeler, modülün kendisi private olduğu sürece dışarıdan erişilemez.

---

## 3. Struct Görünürlüğü (Struct Visibility)

Struct'lar, modül görünürlüğüne ek olarak **alan (field) düzeyinde** bir görünürlük katmanına daha sahiptir.

> **Kural:** Struct alanları varsayılan olarak **private**'dır ve `pub` ile public yapılabilir.

Bu görünürlük, struct modül dışından erişildiğinde devreye girer ve **bilgi gizleme (encapsulation)** amacını taşır.

### Detaylı Örnek

```rust
mod my {
    // Public struct + public alan → Her şey açık
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // Public struct + private alan → Bilgi gizlenmiş
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        // Public constructor (yapıcı metod)
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents: contents,
            }
        }
    }
}

fn main() {
    // ✅ Public alanlı struct doğrudan oluşturulabilir
    let open_box = my::OpenBox { contents: "public information" };
    println!("The open box contains: {}", open_box.contents);

    // ❌ HATA! Private alanlı struct, alan isimleriyle doğrudan oluşturulamaz
    // let closed_box = my::ClosedBox { contents: "classified information" };

    // ✅ Public constructor ile oluşturulabilir
    let _closed_box = my::ClosedBox::new("classified information");

    // ❌ HATA! Private alana dışarıdan erişilemez
    // println!("The closed box contains: {}", _closed_box.contents);
}
```

### Önemli Noktalar

| Durum | Sonuç |
|---|---|
| Struct `pub` + Alan `pub` | Dışarıdan doğrudan oluşturulabilir ve alanlarına erişilebilir |
| Struct `pub` + Alan private | Dışarıdan doğrudan oluşturulamaz, sadece public constructor ile oluşturulabilir. Alanlarına erişilemez |
| Struct private | Dışarıdan hiç erişilemez (struct'ın kendisi private) |

Bu mekanizma, **encapsulation (kapsülleme)** prensibinin Rust'taki temel uygulamasıdır. Veri yapınızın iç detaylarını gizlerken, kontrollü bir arayüz sunmanızı sağlar.

---

## 4. `use` Bildirimi (The `use` Declaration)

`use` anahtar kelimesi, uzun bir yol (path) ifadesini daha kısa bir isimle bağlamak için kullanılır. Kodun okunabilirliğini artırır ve tekrar eden uzun path yazmalarını engeller.

### Temel Kullanım

```rust
use crate::deeply::nested::{
    my_first_function,
    my_second_function,
    AndATraitType
};

fn main() {
    my_first_function();
}
```

### `as` ile Yeniden Adlandırma

Bir import'u farklı bir isimle bağlamak için `as` kullanılır:

```rust
// deeply::nested::function yolunu other_function ismine bağla
use deeply::nested::function as other_function;

fn function() {
    println!("called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

fn main() {
    // deeply::nested::function'a kolay erişim
    other_function();

    println!("Entering block");
    {
        // Bu, `use deeply::nested::function as function` ile aynıdır.
        // Bu function(), dıştakini gölgeler (shadow).
        use crate::deeply::nested::function;

        // use bağlamaları yerel kapsama (scope) sahiptir.
        // Bu durumda function() gölgelemesi sadece bu blok içindedir.
        function();

        println!("Leaving block");
    }

    // Blok dışına çıkınca tekrar orijinal function() çağrılır
    function();
}
```

### `pub use` ile Yeniden Dışa Aktarma (Re-export)

`pub use`, bir modüldeki öğeyi o modülün **public arayüzü** üzerinden yeniden dışa aktarmak için kullanılır. Bu, kütüphane geliştirirken çok sık kullanılan bir kalıptır.

```rust
mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

mod cool {
    // deeply::nested::function'ı cool modülünün public arayüzünden sun
    pub use crate::deeply::nested::function;
}

fn main() {
    // Artık cool::function() olarak çağrılabilir!
    cool::function();
}
```

### `use` Bildiriminin Özellikleri

| Özellik | Açıklama |
|---|---|
| `use yol::ismi` | Uzun yolu kısa isimle bağlar |
| `use yol::isim as yeni_isim` | Farklı bir isimle bağlar |
| `use yol::{a, b, c}` | Aynı yoldan birden fazla öğe import eder |
| `pub use` | Öğeyi modülün public arayüzünden yeniden dışa aktarır |
| Kapsam (scope) | `use` bağlamaları **yerel kapsamlıdır** — sadece tanımlandıkları scope içinde geçerlidir |

---

## 5. `super` ve `self` Anahtar Kelimeleri

`super` ve `self` anahtar kelimeleri, modül hiyerarşisinde gezinmek ve belirsizlikleri gidermek için kullanılır.

- **`self`** → Geçerli modülü (current module) ifade eder
- **`super`** → Üst modülü (parent module) ifade eder

Bu anahtar kelimeler, yolları (path'leri) **sabit kodlamak (hardcode)** zorunda kalmadan, modül yapısı değiştiğinde bile çalışacak esnek kod yazmayı sağlar.

### Detaylı Örnek

```rust
fn function() {
    println!("called `function()`");
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

mod my {
    fn function() {
        println!("called `my::function()`");
    }

    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }

    pub fn indirect_call() {
        // Bu scope'taki tüm "function" isimli fonksiyonlara erişelim!
        print!("called `my::indirect_call()`, that\n> ");

        // self → geçerli modül (my)
        // self::function() ile function() aynı kapıya çıkar
        self::function();
        function();

        // self ile my içindeki başka bir modüle erişim
        self::cool::function();

        // super → üst scope (my modülünün dışı)
        super::function();

        // crate scope'undaki cool::function'a erişim
        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    my::indirect_call();
}
```

### Çıktı

```
called `my::indirect_call()`, that
> called `my::function()`
called `my::function()`
called `my::cool::function()`
called `function()`
called `cool::function()`
```

### `self` vs `super` Karşılaştırması

| Anahtar Kelime | Ne Anlama Gelir? | Ne Zaman Kullanılır? |
|---|---|---|
| `self` | Geçerli modül | Mevcut modüldeki öğelere açıkça erişmek için |
| `super` | Üst (parent) modül | Bir üst seviyedeki modüldeki öğelere erişmek için |
| `crate` | Crate'in kök (root) modülü | Crate'in en üst seviyesindeki öğelere erişmek için |

### Neden Önemli?

Diyelim ki modül yapınızı yeniden düzenlediniz ve bir modülü başka bir yere taşıdınız. Eğer yolları `super::` ve `self::` ile yazdıysanız, taşıma işlemi sırasında çok daha az değişiklik yapmanız gerekir. Hardcoded yollar (örn. `crate::a::b::c::fonksiyon`) ise her taşıma işleminde güncellenmek zorundadır.

---

## 6. Dosya Hiyerarşisi (File Hierarchy)

Modüller, dosya ve dizin hiyerarşisine **eşlenebilir**. Bu, büyük projelerde kodu organize etmenin temel yoludur.

### Dosya Yapısı

```
$ tree .
.
├── my
│   ├── inaccessible.rs
│   └── nested.rs
├── my.rs
└── split.rs
```

### `split.rs` (Ana Dosya / Crate Root)

```rust
// Bu bildirim, "my.rs" dosyasını arar ve içeriğini
// bu scope altında "my" adlı bir modüle yerleştirir
mod my;

fn function() {
    println!("called `function()`");
}

fn main() {
    my::function();
    function();
    my::indirect_access();
    my::nested::function();
}
```

### `my.rs` (my Modülü)

```rust
// "inaccessible.rs" ve "nested.rs" dosyalarını bulup
// respective modüllerinin içine yerleştirir
mod inaccessible;
pub mod nested;

pub fn function() {
    println!("called `my::function()`");
}

fn private_function() {
    println!("called `my::private_function()`");
}

pub fn indirect_access() {
    print!("called `my::indirect_access()`, that\n> ");
    private_function();
}
```

### `my/nested.rs` (my::nested Modülü)

```rust
pub fn function() {
    println!("called `my::nested::function()`");
}

#[allow(dead_code)]
fn private_function() {
    println!("called `my::nested::private_function()`");
}
```

### `my/inaccessible.rs` (my::inaccessible Modülü)

```rust
#[allow(dead_code)]
pub fn public_function() {
    println!("called `my::inaccessible::public_function()`");
}
```

### Derleme ve Çalıştırma

```shell
$ rustc split.rs && ./split
called `my::function()`
called `function()`
called `my::indirect_access()`, that
> called `my::private_function()`
called `my::nested::function()`
```

### Dosya Eşleme Kuralları

| Modül Tanımı | Aranan Dosya |
|---|---|
| `mod foo;` | `foo.rs` veya `foo/mod.rs` |
| `mod foo;` (bir `bar` modülünün içindeyse) | `bar/foo.rs` veya `bar/foo/mod.rs` |

> **Modern Rust (2018 edition ve sonrası):** `foo/mod.rs` yerine `foo.rs` kullanımı tercih edilir. Bu, dosya yapısını daha anlaşılır kılar.

---

## Genel Özet ve En İyi Pratikler

### Modül Sisteminin Temel Prensipleri

1. **Her şey varsayılan olarak private'dır.** Rust'ta gizlilik (privacy) varsayılandır. Bir öğeyi dışarıya açmak istiyorsanız, açıkça `pub` yazmanız gerekir.

2. **Görünürlük hiyerarşiktir.** Private bir modülün içindeki `pub` öğe, modülün kendisi private olduğu sürece dışarıdan erişilemez.

3. **Struct alanlarının kendi görünürlüğü vardır.** Struct'ı `pub` yapmak, alanlarını otomatik olarak `pub` yapmaz. Her alan için ayrı ayrı `pub` belirtmek gerekir.

4. **`use` yerel kapsamlıdır.** Bir `use` bildirimi, sadece tanımlandığı scope (kapsam) içinde geçerlidir.

5. **`super` ve `self` esneklik sağlar.** Modül yapısı değiştiğinde kodun bozulmasını engeller.

6. **Dosya hiyerarşisi modül hiyerarşisini yansıtır.** `mod foo;` bildirimi, `foo.rs` dosyasını o modülün içeriği olarak yükler.

### Pratik İpuçları

- 🎯 **Küçük başlayın:** Başlangıçta her şeyi `main.rs`'te tutun. Proje büyüdükçe modüllere ayırın.
- 🎯 **`pub`'ı minimum tutun:** Sadece gerçekten dışarıya açılması gereken öğeleri `pub` yapın.
- 🎯 **Re-export kullanın:** Kütüphane yazıyorsanız, `pub use` ile kullanıcılarınıza daha temiz bir API sunun.
- 🎯 **Dosya yapısını düzenli tutun:** Her modül için ayrı dosya kullanın ve dosya isimlendirmesinde tutarlı olun.

---

Bu dersin sonunda Rust'ın modül sisteminin temel yapı taşlarını — modül tanımlama, görünürlük yönetimi, struct görünürlüğü, `use` bildirimleri, `super`/`self` navigasyonu ve dosya hiyerarşisi — tam olarak anlamış olmanız gerekiyor. Bu kavramlar, Rust'ta büyük ve sürdürülebilir projeler geliştirmenin temelini oluşturur. 🚀

# DeepSeek Yorumu ----------------

# Rust'ta Modül Sistemi (Modules)

Rust, kodunuzu mantıksal birimlere (modüllere) hiyerarşik olarak bölmenize ve bu birimler arasındaki görünürlüğü (public/private) yönetmenize olanak tanıyan güçlü bir modül sistemine sahiptir. Bir modül, fonksiyonlar, struct'lar, trait'ler, `impl` blokları ve hatta diğer modüller gibi öğelerin bir koleksiyonudur.

Bu derste, Rust'ın modül sistemini adım adım, sıfırdan başlayarak ve bol örnekle ele alacağız.

---

## 1. Modül Nedir ve Neden Kullanılır?

Modüller, kodunuzu düzenlemenin temel yoludur. Büyük projelerde kodun karmaşıklığını yönetmek, isim çakışmalarını önlemek ve kodun hangi bölümlerinin dışarıya açık olduğunu kontrol etmek için modüller kullanılır.

Bir modülü `mod` anahtar sözcüğü ile tanımlarsınız:

```rust
mod my_mod {
    // Bu modülün içeriği
}
```

Modüller iç içe (nested) tanımlanabilir, böylece kodunuzu hiyerarşik bir şekilde organize edebilirsiniz.

---

## 2. Görünürlük (Visibility)

Rust'ta modül içindeki öğeler varsayılan olarak **private** (özel) görünürlüğe sahiptir. Yani, bir modülün içindeki bir fonksiyon, struct veya başka bir öğe, o modülün dışından erişilemez. Bu varsayılan davranışı `pub` (public) anahtar sözcüğü ile değiştirebilirsiniz.

### Temel Kullanım

```rust
mod my_mod {
    // Varsayılan olarak private
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    // `pub` ile public yapıldı
    pub fn function() {
        println!("called `my_mod::function()`");
    }

    // Private öğelere aynı modül içinden erişilebilir
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function(); // Private fonksiyona erişim
    }
}
```

Yukarıdaki örnekte:
- `private_function` yalnızca `my_mod` modülü içinden çağrılabilir.
- `function` ve `indirect_access` public olduğu için dışarıdan erişilebilir.

### `pub` ile Görünürlük Kapsamları

Rust, görünürlüğü daha hassas kontrol etmek için çeşitli `pub` varyantları sunar:

| Kullanım | Anlamı |
|----------|--------|
| `pub` | Her yerde görünür |
| `pub(crate)` | Yalnızca mevcut crate içinde görünür |
| `pub(self)` | Yalnızca mevcut modül içinde görünür (private ile aynı) |
| `pub(super)` | Yalnızca üst modül (parent) içinde görünür |
| `pub(in path)` | Yalnızca belirtilen path içinde görünür |

**Örnek:**

```rust
pub mod nested {
    // Sadece `crate::my_mod` içinden görünür
    pub(in crate::my_mod) fn public_function_in_my_mod() {
        println!("called `my_mod::nested::public_function_in_my_mod()`");
    }

    // Sadece mevcut modül içinde görünür
    pub(self) fn public_function_in_nested() {
        println!("called `my_mod::nested::public_function_in_nested()`");
    }

    // Sadece üst modülde (parent) görünür
    pub(super) fn public_function_in_super_mod() {
        println!("called `my_mod::nested::public_function_in_super_mod()`");
    }
}
```

**Önemli Not:** Bir alt modül içindeki public öğe bile, eğer üst modül private ise dışarıdan erişilemez. Görünürlük her zaman yukarıdan aşağıya doğru sınırlanır.

---

## 3. `use` Bildirimi ile İthalat

Her seferinde tam modül yolunu (`crate::deeply::nested::function`) yazmak yerine, `use` bildirimi ile bir yolu yeni bir isme bağlayarak daha kolay erişim sağlayabiliriz.

### Temel Kullanım

```rust
use crate::deeply::nested::function;

fn main() {
    function(); // Tam yol olmadan çağrı
}
```

### `as` ile Yeniden Adlandırma

Çakışmaları önlemek veya daha kısa isimler kullanmak için `as` anahtar sözcüğü ile import edilen öğeyi yeniden adlandırabilirsiniz:

```rust
use deeply::nested::function as other_function;

fn function() {
    println!("called `function()`");
}

fn main() {
    other_function(); // `deeply::nested::function` çağrılır
}
```

### `use` Bağlamalarının Kapsamı

`use` bildirimleri bulundukları kapsam (scope) ile sınırlıdır. Bir blok içinde yapılan `use`, o blok dışında geçerli değildir:

```rust
fn main() {
    // Dışarıdaki fonksiyon
    function();

    {
        // Bu blok içinde `function` ismi `deeply::nested::function`'a bağlanır
        use crate::deeply::nested::function;
        function(); // `deeply::nested::function` çağrılır
        // Blok sona erdiğinde bu bağlama geçersiz olur
    }

    function(); // Tekrar dışarıdaki `function` çağrılır
}
```

### `pub use` ile Yeniden Dışa Aktarma (Re-export)

Bir modül içindeki öğeyi, o modülün public arayüzü üzerinden dışa aktarmak için `pub use` kullanabilirsiniz. Bu, başka bir modüldeki öğeyi sanki o modülün kendi öğesiymiş gibi sunar:

```rust
mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

mod cool {
    // `deeply::nested::function`'ı `cool` modülü üzerinden dışa aktar
    pub use crate::deeply::nested::function;
}

fn main() {
    cool::function(); // Sanki `cool`'un kendi fonksiyonuymuş gibi
}
```

---

## 4. `super` ve `self` ile Yol Belirleme

Bazen aynı isimde birden fazla öğe olduğunda veya yolları gereksiz yere uzun yazmamak için `super` ve `self` anahtar sözcükleri kullanılır.

### `self`

`self`, **mevcut modülü** ifade eder. `self::function()` ile `function()` doğrudan çağrısı aynı sonucu verir, çünkü ikisi de aynı modüldeki fonksiyonu işaret eder:

```rust
mod my {
    fn function() {
        println!("called `my::function()`");
    }

    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }

    pub fn indirect_call() {
        // `self::function()` ile `function()` aynıdır
        self::function();
        function();

        // `self` ile başka bir alt modüle erişim
        self::cool::function();
    }
}
```

### `super`

`super`, **üst modülü (parent scope)** ifade eder. Mevcut modülün dışındaki bir öğeye erişmek için kullanılır:

```rust
fn function() {
    println!("called `function()`");
}

mod my {
    pub fn indirect_call() {
        // Üst modüldeki (crate kökü) `function`'a erişim
        super::function();
    }
}
```

### Crate Köküne Erişim

`crate::` ile crate'in kök modülüne erişebilirsiniz. `super::` ile üst modüle çıktıktan sonra `crate::` ile en üste çıkabilirsiniz:

```rust
mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

mod my {
    pub fn indirect_call() {
        // `super::` ile üst modüle çık, oradan `crate::` ile köke eriş
        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}
```

---

## 5. Dosya Hiyerarşisi ile Modüller (Modülü Dosyalara Bölme)

Rust, modülleri dosya ve dizin hiyerarşisine göre haritalandırmanıza olanak tanır. Bu, büyük projelerde modülleri ayrı dosyalara ayırarak daha düzenli kod yazmanızı sağlar.

### Temel Kural

- `mod my;` bildirimi, aynı dizinde `my.rs` dosyasını arar ve içeriğini `my` modülü olarak ekler.
- `mod my;` bildirimi ayrıca `my/mod.rs` dosyasını da arar (eski stil, ancak hala desteklenir).
- İç içe modüller için, `my` dizini altında `nested.rs` dosyası `mod nested;` ile eklenir.

### Örnek Dosya Yapısı

Aşağıdaki dosya yapısını ele alalım:

```
.
├── my
│   ├── inaccessible.rs
│   └── nested.rs
├── my.rs
└── split.rs
```

**split.rs (ana dosya):**

```rust
// `my.rs` dosyasını `my` modülü olarak ekle
mod my;

fn function() {
    println!("called `function()`");
}

fn main() {
    my::function();
    function();
    my::indirect_access();
    my::nested::function();
}
```

**my.rs:**

```rust
// `inaccessible.rs` dosyasını `inaccessible` modülü olarak ekle
mod inaccessible;
// `nested.rs` dosyasını `nested` modülü olarak ekle (public)
pub mod nested;

pub fn function() {
    println!("called `my::function()`");
}

fn private_function() {
    println!("called `my::private_function()`");
}

pub fn indirect_access() {
    print!("called `my::indirect_access()`, that\n> ");
    private_function();
}
```

**my/nested.rs:**

```rust
pub fn function() {
    println!("called `my::nested::function()`");
}

#[allow(dead_code)]
fn private_function() {
    println!("called `my::nested::private_function()`");
}
```

**my/inaccessible.rs:**

```rust
#[allow(dead_code)]
pub fn public_function() {
    println!("called `my::inaccessible::public_function()`");
}
```

### Derleme ve Çalıştırma

```bash
$ rustc split.rs && ./split
called `my::function()`
called `function()`
called `my::indirect_access()`, that
> called `my::private_function()`
called `my::nested::function()`
```

Bu örnekte:
- `split.rs` içindeki `mod my;` ifadesi `my.rs` dosyasını yükler.
- `my.rs` içindeki `mod nested;` ifadesi `my/nested.rs` dosyasını yükler.
- Modüller arası görünürlük kuralları dosyalar arasında da aynen geçerlidir.

---

## 6. Modül Sistemi ile İlgili Sık Yapılan Hatalar

| Hata | Çözüm |
|------|-------|
| `mod my;` bildirimi yapıldı ama `my.rs` dosyası bulunamıyor | Dosya adını ve konumunu kontrol edin |
| Private bir öğeye dışarıdan erişmeye çalışmak | `pub` ekleyin veya erişim kapsamını genişletin |
| `use` ile import edilen öğe bulunamıyor | Yolun doğru olduğundan ve modülün public olduğundan emin olun |
| Aynı isimli iki öğe arasında çakışma | `as` ile yeniden adlandırın veya tam yol kullanın |

---

## Özet

| Konu | Açıklama |
|------|----------|
| **mod** | Yeni bir modül tanımlar veya bir dosyayı modül olarak ekler |
| **pub** | Bir öğeyi public yapar (varsayılan private) |
| **pub(crate)** | Öğeyi yalnızca mevcut crate içinde görünür yapar |
| **pub(super)** | Öğeyi yalnızca üst modül içinde görünür yapar |
| **pub(self)** | Öğeyi yalnızca mevcut modül içinde görünür yapar (private ile aynı) |
| **use** | Bir yolu kısa bir isme bağlar |
| **as** | `use` ile import edilen öğeyi yeniden adlandırır |
| **pub use** | Bir öğeyi yeniden dışa aktarır |
| **self** | Mevcut modülü ifade eder |
| **super** | Üst modülü ifade eder |
| **crate** | Crate kökünü ifade eder |

Rust'ın modül sistemi, kodunuzu temiz, düzenli ve güvenli bir şekilde organize etmenin anahtarıdır. Private görünürlük varsayılanı sayesinde, hangi API'lerin dışa açık olduğunu kontrol edebilir, iç implementasyon detaylarını gizleyerek güçlü soyutlamalar oluşturabilirsiniz.