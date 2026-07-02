# 🦀 Ders Notları: Bölüm 22 - 📚 Rust Programlama Dili — Ekler (Appendix) Bölümü Detaylı Anlatım

Rust kitabının **Appendix (Ekler)** bölümü, dilin referans niteliğindeki teknik detaylarını bir araya getiren bir başvuru kaynağıdır. Bu bölüm, kitabın ana akışını tamamlamakla birlikte, Rust yolculuğunuzda sık sık döneceğiniz bir başucu kaynağıdır. Şimdi her bir eki ders anlatır gibi, örneklerle ve detaylı açıklamalarla inceleyelim.

---

## 🅰️ Ek A: Anahtar Kelimeler (Keywords)

Rust'ta **anahtar kelimeler (keywords)**, dilin kendisi tarafından rezerve edilmiş özel isimlerdir. Bu kelimeleri; fonksiyon, değişken, parametre, struct alanı, modül, crate, sabit, macro, static değer, attribute, type, trait ya da lifetime isimlendirmelerinde **doğrudan kullanamazsınız**.

### 1. Aktif (Strict) Anahtar Kelimeler

Bunlar Rust'ın şu anda aktif olarak kullandığı, her birinin belirli bir görevi olan kelimelerdir:

| Anahtar Kelime | Görevi |
|---|---|
| `as` | İlkel (primitive) tür cast'ı yapar; `use` ifadelerinde yeniden adlandırma yapar; belirli bir trait'i işaret eder. |
| `async` | Bir fonksiyonun `Future` döndürmesini sağlar, thread'i bloklamaz. |
| `await` | Bir `Future`'ın sonucu hazır olana kadar yürütmeyi askıya alır. |
| `break` | Döngüyü anında sonlandırır. |
| `const` | Sabit (constant) öğe ya da sabit ham pointer tanımlar. |
| `continue` | Döngünün bir sonraki iterasyonuna geçer. |
| `crate` | Modül yolunda crate kökünü (root) temsil eder. |
| `dyn` | Trait objesine dinamik dispatch (gönderim) yapar. |
| `else` | `if` ve `if let` yapıları için alternatif dal. |
| `enum` | Enumerasyon (sıralama) tanımlar. |
| `extern` | Harici bir fonksiyon ya da değişkene bağlanır. |
| `false` | Boolean `false` değişmezi. |
| `fn` | Fonksiyon ya da fonksiyon pointer türü tanımlar. |
| `for` | Iterator'dan öğeleri döngüyle alır; trait implement eder; higher-ranked lifetime belirtir. |
| `if` | Koşullu ifadeye göre dallanma yapar. |
| `impl` | Inherent ya da trait işlevselliğini implement eder. |
| `in` | `for` döngüsü sözdiziminin bir parçası. |
| `let` | Bir değişken bağlar (bind). |
| `loop` | Koşulsuz sonsuz döngü. |
| `match` | Bir değeri pattern'lara (desenlere) göre eşler. |
| `mod` | Modül tanımlar. |
| `move` | Closure'un tüm yakaladıklarını sahipliğini almasını sağlar. |
| `mut` | Referanslarda, ham pointer'larda ya da pattern bağlamalarında değişkenliği (mutability) belirtir. |
| `pub` | Struct alanlarında, `impl` bloklarında ya da modüllerde genel görünürlüğü belirtir. |
| `ref` | Referans ile bağlama yapar. |
| `return` | Fonksiyondan dönüş yapar. |
| `Self` | Tanımladığımız ya da implement ettiğimiz tür için bir takma addır (type alias). |
| `self` | Metodun öznesi ya da mevcut modül. |
| `static` | Global değişken ya da programın tamamı boyunca süren lifetime. |
| `struct` | Yapı (structure) tanımlar. |
| `super` | Mevcut modülün üst (parent) modülü. |
| `trait` | Trait tanımlar. |
| `true` | Boolean `true` değişmezi. |
| `type` | Type alias ya da associated type tanımlar. |
| `union` | Union tanımlar (yalnızca union bildiriminde anahtar kelimedir). |
| `unsafe` | Unsafe (güvenli olmayan) kod, fonksiyon, trait ya da implementasyonları belirtir. |
| `use` | Sembolleri scope'a getirir. |
| `where` | Türü kısıtlayan (constrain) clause'ları belirtir. |
| `while` | İfadenin sonucuna göre koşullu döngü. |

### 2. Rezerve (Geleceğe Ayrılmış) Anahtar Kelimeler

Bunlar henüz bir işlevi olmayan ama Rust tarafından **gelecekteki kullanım için ayrılmış** kelimelerdir:

`abstract`, `become`, `box`, `do`, `final`, `gen`, `macro`, `override`, `priv`, `try`, `typeof`, `unsized`, `virtual`, `yield`

### 3. Ham Tanımlayıcılar (Raw Identifiers)

Peki ya bir kütüphanede `try` adında bir fonksiyon varsa ve siz 2018 edition kullanıyorsanız? İşte burada **ham tanımlayıcılar** devreye girer. Bir anahtar kelimenin önüne `r#` ekleyerek onu sıradan bir isim gibi kullanabilirsiniz:

```rust
// 'match' normalde bir anahtar kelimedir, ama r# öneki ile fonksiyon adı yapılabilir:
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

fn main() {
    assert!(r#match("foo", "foobar")); // Kullanım
}
```

Bu özellikle farklı edition'larda yazılmış kütüphanelerle çalışırken kritik öneme sahiptir. Örneğin `try`, 2015 edition'da anahtar kelime değilken 2018 ve sonrasında anahtar kelimedir.

---

## 🅱️ Ek B: Operatörler ve Semboller

Rust'ın sözdizimindeki tüm operatörler ve semboller, farklı bağlamlarda (paths, generics, trait bounds, macros, attributes, comments, tuples, brackets) kullanılır. Bu ek, adeta Rust'ın **sözdizimi sözlüğü** gibidir.

### 1. Operatörler (Operators)

Her operatörün **aşırı yüklenebilir (overloadable)** olup olmadığı ve hangi trait ile yüklendiği belirtilir:

#### Aritmetik Operatörler
| Operatör | Örnek | Açıklama | Aşırı Yüklenebilir Trait |
|---|---|---|---|
| `+` | `a + b` | Toplama | `Add` |
| `-` | `a - b` | Çıkarma | `Sub` |
| `*` | `a * b` | Çarpma | `Mul` |
| `/` | `a / b` | Bölme | `Div` |
| `%` | `a % b` | Mod (kalan) | `Rem` |
| `-expr` | `-x` | Negatif yapma | `Neg` |

#### Atama ile Birleşik Operatörler
`+=` (`AddAssign`), `-=` (`SubAssign`), `*=` (`MulAssign`), `/=` (`DivAssign`), `%=` (`RemAssign`)

#### Karşılaştırma Operatörleri
| Operatör | Açıklama | Trait |
|---|---|---|
| `==` | Eşitlik | `PartialEq` |
| `!=` | Eşitsizlik | `PartialEq` |
| `<`, `>`, `<=`, `>=` | Sıralama | `PartialOrd` |

#### Mantıksal ve Bitwise Operatörler
| Operatör | Açıklama | Trait |
|---|---|---|
| `!` | Mantıksal/bitwise değil | `Not` |
| `&` | Bitwise AND | `BitAnd` |
| `\|` | Bitwise OR | `BitOr` |
| `^` | Bitwise XOR | `BitXor` |
| `<<` | Sola kaydırma | `Shl` |
| `>>` | Sağa kaydırma | `Shr` |
| `&&` | Kısa devre (short-circuit) AND | — |
| `\|\|` | Kısa devre OR | — |

#### Diğer Önemli Operatörler
- `!ident!(...)` → Macro çağrısı
- `*expr` → Dereference (referansı çözme) → `Deref` trait'i
- `&expr` / `&mut expr` → Borrowing (ödünç alma)
- `expr?` → Hata yayma (error propagation)
- `@` → Pattern binding (ör. `x @ 1..=5`)
- `..` ve `..=` → Range (aralık) ifadeleri
- `.` → Alan erişimi, metod çağrısı, tuple indeksleme

### 2. Bağımsız Semboller (Stand-alone Syntax)

| Sembol | Açıklama |
|---|---|
| `'ident` | İsimlendirilmiş lifetime ya da döngü etiketi |
| `42u8`, `3.14f64` | Belirli türde sayısal değişmez |
| `"..."` | String değişmezi |
| `r"..."`, `r#"..."#` | Ham (raw) string — kaçış karakterleri işlenmez |
| `b"..."` | Byte string (string yerine byte dizisi) |
| `'a'` | Karakter değişmezi |
| `b'a'` | ASCII byte değişmezi |
| `\|...\| expr` | Closure (kapama) |
| `!` | Sonsuz döndüren (diverging) fonksiyonlar için "bottom type" |
| `_` | "Yok sayılan" pattern binding; sayıları okunabilir yapar (örn: `1_000_000`) |

### 3. Yol (Path) İlişkili Semboller

| Sembol | Açıklama |
|---|---|
| `ident::ident` | Namespace yolu |
| `::path` | Crate köküne göre mutlak yol |
| `self::path` | Mevcut modüle göre göreli yol |
| `super::path` | Üst modüle göre yol |
| `<type as trait>::ident` | Belirli bir trait üzerinden ilişkilendirilmiş öğe |

### 4. Generic İlişkili Semboller

| Sembol | Açıklama |
|---|---|
| `path<...>` | Generic tür parametreleri (örn: `Vec<u8>`) |
| `path::<...>` | **Turbofish** sözdizimi (örn: `"42".parse::<i32>()`) |
| `fn ident<...>` | Generic fonksiyon tanımı |
| `for<...> type` | Higher-ranked lifetime bound |

### 5. Trait Bound Kısıtlamaları

| Sembol | Açıklama |
|---|---|
| `T: U` | `T`, `U` trait'ini implement etmeli |
| `T: 'a` | `T`, `'a` lifetime'ından uzun yaşamalı |
| `T: 'static` | `T`, `'static` dışında borrowed referans içermemeli |
| `T: ?Sized` | Dinamik boyutlu türe izin verir |

### 6. Macro ve Attribute Sembolleri
- `#[meta]` → Outer attribute (dış öznitelik)
- `#![meta]` → Inner attribute (iç öznitelik)
- `$ident` → Macro substitüsyonu
- `$(...)...` → Macro tekrarı
- `ident!(...)` → Macro çağrısı

### 7. Yorum (Comment) Sembolleri
| Sembol | Açıklama |
|---|---|
| `//` | Satır yorumu |
| `///` | Dış dokümantasyon yorumu |
| `//!` | İç dokümantasyon yorumu |
| `/* ... */` | Blok yorumu |
| `/** ... */` | Dış blok dokümantasyon yorumu |
| `/*! ... */` | İç blok dokümantasyon yorumu |

### 8. Parantez Kullanımları
- `()` → Boş tuple (unit) — hem literal hem tür
- `(expr,)` → Tek elemanlı tuple
- `expr(expr, ...)` → Fonksiyon çağrısı
- `{...}` → Blok ifadesi
- `Type {...}` → Struct literal
- `[...]` → Array literal
- `[expr; len]` → `len` kopya içeren array
- `expr[idx]` → Koleksiyon indeksleme (`Index`, `IndexMut` trait'leri ile aşırı yüklenebilir)

---

## 🅲 Ek C: Türetilebilir Trait'ler (Derivable Traits)

Rust'ta `#[derive(...)]` attribute'u ile standart kütüphanedeki bazı trait'leri otomatik olarak implement edebilirsiniz. Bu bölüm, bu trait'lerin her birinin:
- Hangi operatör/metodları sağladığını,
- `derive` implementasyonunun ne yaptığını,
- Bu trait'in tür hakkında ne anlama geldiğini,
- Hangi koşullarda implement edilip edilemeyeceğini,
- Hangi durumlarda gerekli olduğunu açıklar.

> **Önemli Not:** `Display` gibi bazı trait'ler türetilemez çünkü bunlar son kullanıcıya yönelik özel formatlama gerektirir. Derive edilebilen trait'ler yalnızca standart kütüphanedekilerle sınırlı değildir; kütüphaneler kendi trait'leri için procedural macro kullanarak `derive` desteği sunabilir.

### 1. `Debug` Trait'i

- **Ne sağlar?** Format string'lerinde `{:?}` ile debug formatlaması.
- **Ne yapar?** Türün bir örneğini debug amaçlı yazdırılabilir hale getirir.
- **Nerede gerekir?** `assert_eq!` macrosu başarısız olduğunda değerleri yazdırabilmek için.

```rust
#[derive(Debug)]
struct Point { x: i32, y: i32 }

fn main() {
    let p = Point { x: 3, y: 7 };
    println!("{:?}", p); // Point { x: 3, y: 7 }
}
```

### 2. `PartialEq` Trait'i

- **Ne sağlar?** `==` ve `!=` operatörleri ile eşitlik karşılaştırması.
- **Ne yapar?** `eq()` metodunu implement eder. Struct'larda tüm alanlar eşitse eşit; enum'larda aynı varyantlar eşit sayılır.
- **Nerede gerekir?** `assert_eq!` macrosunda.

### 3. `Eq` Trait'i

- **Ne sağlar?** Hiç metodu yoktur; türün **her değerinin kendine eşit olduğunu** işaret eder.
- **Kısıt:** Yalnızca `PartialEq` implement eden türlere uygulanabilir.
- **İstisna:** `f32` ve `f64`'daki `NaN` (Not a Number) değeri kendine eşit olmadığından bu türler `Eq` implement edemez.
- **Nerede gerekir?** `HashMap<K, V>`'de anahtar olarak kullanımda.

### 4. `PartialOrd` Trait'i

- **Ne sağlar?** `<`, `>`, `<=`, `>=` operatörleri ile sıralama.
- **Ne yapar?** `partial_cmp()` metodunu implement eder; `Option<Ordering>` döner. Bazı değerler için `None` dönebilir (örn: `NaN`).
- **Struct'larda:** Alanlar tanımlandıkları sırada karşılaştırılır.
- **Enum'larda:** Tanımda önce gelen varyant, "küçük" kabul edilir.
- **Kısıt:** `PartialEq` gerektirir.

### 5. `Ord` Trait'i

- **Ne sağlar?** Herhangi iki değer arasında **mutlaka** geçerli bir sıralama olduğunu garanti eder.
- **Ne yapar?** `cmp()` metodu — `Option<Ordering>` değil, doğrudan `Ordering` döner.
- **Kısıt:** Hem `PartialEq` hem `PartialOrd` hem `Eq` gerektirir.
- **Nerede gerekir?** `BTreeSet<T>` gibi sıralı veri yapılarında.

### 6. `Clone` Trait'i

- **Ne sağlar?** Bir değerin **derin kopyasını** (deep copy) açıkça oluşturur.
- **Ne yapar?** `clone()` metodu — türün tüm parçalarında `clone` çağırır.
- **Kısıt:** Tüm alanların da `Clone` implement etmesi gerekir.
- **Nerede gerekir?** Bir slice'tan `to_vec()` çağrıldığında.

### 7. `Copy` Trait'i

- **Ne sağlar?** Yalnızca stack'teki bitleri kopyalayarak değeri çoğaltır; keyfi kod çalışmaz.
- **Önemli:** Hiç metodu yoktur — bu, kopyalamanın her zaman hızlı olacağını garanti eder.
- **Kısıt:** Tüm parçaları `Copy` olmalı ve `Copy`, `Clone`'u da gerektirir.
- **Not:** `Copy` nadiren doğrudan gerekir; amacı optimizasyondur, böylece `clone()` çağrısı gerekmez.

### 8. `Hash` Trait'i

- **Ne sağlar?** Keyfi boyuttaki bir tür örneğini, hash fonksiyonu ile sabit boyutlu bir değere eşler.
- **Ne yapar?** `hash()` metodu — her parçada `hash` çağırarak sonucu birleştirir.
- **Kısıt:** Tüm alanların `Hash` implement etmesi gerekir.
- **Nerede gerekir?** `HashMap<K, V>`'de verimli anahtarlama için.

### 9. `Default` Trait'i

- **Ne sağlar?** Tür için bir varsayılan değer oluşturur.
- **Ne yapar?** `default()` fonksiyonu — her parçada `default` çağırır.
- **Kısıt:** Tüm alanların `Default` implement etmesi gerekir.
- **Nerede gerekir?**
  - Struct update syntax ile: `MyStruct { field: value, ..Default::default() }`
  - `Option<T>::unwrap_or_default()` metodunda.

```rust
#[derive(Default)]
struct Config {
    timeout: u32,      // 0
    name: String,      // ""
    verbose: bool,     // false
}

fn main() {
    let cfg = Config { timeout: 500, ..Default::default() };
}
```

---

## 🅳 Ek D: Kullanışlı Geliştirme Araçları

Rust ekosistemi, geliştiricilerin hayatını kolaylaştıran birkaç güçlü araçla birlikte gelir.

### 1. `rustfmt` — Otomatik Kod Formatlayıcı

Topluluğun kabul ettiği kod stiline göre kodunuzu yeniden biçimlendirir. Birçok ortak proje, stil tartışmalarını önlemek için `rustfmt` kullanılmasını zorunlu kılar.

```bash
cargo fmt
```

Bu komut, mevcut crate'teki tüm Rust kodunu yeniden biçimlendirir. Yalnızca **kod stilini** değiştirir, **semantiği** değiştirmez.

### 2. `rustfix` — Otomatik Uyarı Düzeltici

Derleyici uyarılarını otomatik olarak düzeltir. Özellikle `cargo fix` komutu ile kullanılır.

**Örnek:**
```rust
fn main() {
    let mut x = 42;
    println!("{x}");
}
```

Bu kodda `x` değişkeni `mut` olarak tanımlanmış ama hiç değiştirilmemiş. Rust uyarır:

```
warning: variable does not need to be mutable
```

`cargo fix` çalıştırırsanız:

```bash
$ cargo fix
     Fixing src/main.rs (1 fix)
```

Sonuç:
```rust
fn main() {
    let x = 42;  // 'mut' otomatik kaldırıldı!
    println!("{x}");
}
```

Ayrıca `cargo fix` komutu, Rust edition'ları arasında geçiş yapmak için de kullanılabilir (Ek E'de anlatılıyor).

### 3. `Clippy` — Linter (Kod Analiz Aracı)

Yaygın hataları yakalayan ve Rust kodunuzu iyileştiren lint'ler (kod analizi kuralları) koleksiyonudur.

```bash
cargo clippy
```

**Örnek:**
```rust
fn main() {
    let x = 3.1415;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

Clippy bunu yakalar:
```
error: approximate value of `f{32, 64}::consts::PI` found
  = help: consider using the constant directly
```

Çözüm:
```rust
fn main() {
    let x = std::f64::consts::PI;  // Standart kütüphanedeki hassas PI sabiti
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

### 4. `rust-analyzer` — IDE Entegrasyonu

Rust topluluğunun önerdiği, Language Server Protocol (LSP) konuşan bir araç setidir. VS Code, Vim, Emacs gibi editörlerle entegre olur ve şu özellikleri sağlar:

- Otomatik tamamlama (autocompletion)
- Tanıma atlama (jump to definition)
- Satır içi hata gösterimi (inline errors)
- Yeniden adlandırma (rename)
- Type inference bilgileri

Kurulum için [rust-analyzer'ın resmi sayfası](https://rust-analyzer.github.io/) ziyaret edilebilir.

---

## 🅴 Ek E: Edition'lar (Sürümler)

Rust'ın dil ve derleyicisi **6 haftalık** bir yayın döngüsüne sahiptir. Bu, sürekli küçük güncellemeler anlamına gelir. Ancak zamanla bu küçük değişiklikler birikir. İşte bu noktada **Edition** kavramı devreye girer.

### Edition Nedir?

Her **3 yılda bir**, Rust ekibi yeni bir **Edition** yayınlar. Her edition, biriken özellikleri net bir paket halinde, tam güncellenmiş dokümantasyon ve araçlarla sunar.

### Edition'ların Farklı Kitleler İçin Anlamı

- **Aktif Rust kullanıcıları için:** Artımlı değişiklikleri anlaşılır bir pakette toplar.
- **Kullanmayanlar için:** Rust'a tekrar bakmalarını sağlayacak büyük gelişmelerin sinyalini verir.
- **Rust geliştiricileri için:** Proje için bir odak noktası oluşturur.

### Mevcut Edition'lar

Şu anda 4 Rust edition'ı mevcuttur:
- **Rust 2015**
- **Rust 2018**
- **Rust 2021**
- **Rust 2024** (Bu kitap 2024 edition deyimleriyle yazılmıştır)

### `Cargo.toml`'da Edition Belirtimi

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2024"   # <-- Edition burada belirtilir
```

Eğer `edition` anahtarı yoksa, geriye uyumluluk için Rust **2015**'i varsayar.

### Edition Uyumluluğu

- Her edition, yeni anahtar kelimeler gibi **uyumsuz değişiklikler** içerebilir.
- Ancak siz yeni edition'a geçmediğiniz sürece, kodunuz yeni derleyici sürümlerinde de derlenmeye devam eder.
- **Tüm Rust derleyici sürümleri**, kendi yayınlarından önce var olan tüm edition'ları destekler.
- **Farklı edition'lardaki crate'ler birbirine bağlanabilir.** Örneğin siz 2015 kullanırken bağımlılığınız 2018 olabilir — sorun çıkmaz!

### Önemli Noktalar

- Çoğu özellik tüm edition'larda mevcuttur.
- Ancak bazı özellikler (özellikle yeni anahtar kelimeler gerektirenler) yalnızca yeni edition'larda kullanılabilir.
- Detaylar için **The Rust Edition Guide** başvurulabilir; `cargo fix` ile edition'lar arasında otomatik geçiş yapılabilir.

---

## 🅵 Ek F: Kitabın Çevirisi

Rust topluluğu küreseldir ve Rust kitabı birçok dile gönüllüler tarafından çevrilmektedir. Bu ek, kitabın çevirilerine erişim bilgilerini içerir.

Rust kitabının resmi çevirileri, Rust topluluğu üyeleri tarafından sürdürülmektedir. Çeviriler, dilin öğrenilmesini dünya genelinde erişilebilir kılmayı amaçlar. Eğer kendi ana dilinizde okumak isterseniz, Rust'ın resmi web sitesindeki çeviri bağlantılarını kontrol edebilirsiniz.

---

## 🅶 Ek G: Nightly Rust (Gece Sürümü Rust)

Rust'ın kararlı (stable) sürümü 6 haftada bir yayınlanır. Ancak bazı özellikler henüz kararlı değildir — bunlar **Nightly Rust** sürümünde test edilir.

### Nightly Neden Var?

Bazı gelişmiş özellikler (örneğin yeni syntax, derleyici eklentileri, deneysel API'ler) kararsız (unstable) olarak işaretlenir ve yalnızca nightly derleyicide kullanılabilir. Bu özellikler yeterince test edildikten ve RFC sürecinden geçtikten sonra **stable**'a terfi eder.

### Nightly Nasıl Kullanılır?

`rustup` ile nightly toolchain'i yükleyebilirsiniz:

```bash
rustup toolchain install nightly
```

Belirli bir projede nightly kullanmak için proje dizinine bir `rust-toolchain.toml` dosyası ekleyebilirsiniz:

```toml
[toolchain]
channel = "nightly"
```

Ya da tek seferlik çalıştırmalar için:

```bash
cargo +nightly build
```

### Feature Flags

Nightly özelliklerini kullanmak için kodunuzun üst kısmında `#![feature(...)]` attribute'u kullanmanız gerekir:

```rust
#![feature(some_experimental_feature)]

fn main() {
    // Deneysel özelliği kullanan kod
}
```

### Ne Zaman Nightly Kullanmalı?

- **Çoğu kullanıcı için:** Stable yeterlidir, nightly gerekmez.
- **Deneysel özellikler için:** Async/await stable olmadan önce yıllarca nightly'deydi.
- **Performans kritik sistemler için:** Bazı nightly-only optimizasyonlar olabilir.
- **Derleyici eklentileri için:** Procedural macro'ların bazı gelişmiş özellikleri.

> **Uyarı:** Nightly özellikleri değişkendir ve her gece derlemesinde kırılabilir. Production kodda kullanımdan kaçınılmalıdır.

---

## 📌 Özet: Eklerden Ne Öğrendik?

| Ek | Konu | Anahtar Kavramlar |
|---|---|---|
| **A** | Anahtar Kelimeler | Strict, reserved, raw identifiers (`r#`) |
| **B** | Operatörler & Semboller | Aşırı yükleme trait'leri, turbofish, path syntax |
| **C** | Türetilebilir Trait'ler | Debug, PartialEq, Eq, Clone, Copy, Hash, Default |
| **D** | Geliştirme Araçları | rustfmt, cargo fix, Clippy, rust-analyzer |
| **E** | Edition'lar | 2015/2018/2021/2024, geriye uyumluluk, Cargo.toml |
| **F** | Çeviri | Topluluk çevirileri |
| **G** | Nightly Rust | Kararsız özellikler, feature flags, rustup |

Bu ekler, Rust'ın referans niteliğindeki teknik detaylarını içerir. İlk okumada hepsini ezberlemenize gerek yok — ancak Rust ile çalıştıkça, özellikle **operatörlerin aşırı yüklenmesi (Ek B)**, **derive trait'leri (Ek C)** ve **geliştirme araçları (Ek D)** bölümlerine sık sık döneceksiniz. Bu ekler, Rust yolculuğunuzda birer **başucu kaynağı** olarak her zaman yanınızda olmalı! 🚀

# DeepSeek Yorumu --------------------
Şimdi seninle Rust programlama dilinin resmi kitabı olan "The Rust Programming Language" (TR: Rust Programlama Dili) kitabının **Ekler (Appendix)** bölümünü, yani şu sayfayı ve alt başlıklarını, sanki bir sınıfta ders anlatıyormuşuz gibi en ince ayrıntısına kadar işleyeceğiz.

Kitabın bu kısmı, dilin temel konularını öğrendikten sonra başvurman gereken bir referans niteliğindedir. Anlatımımız tamamen Türkçe ve öğretici bir üslupla olacak. Hazırsan başlayalım.

---

## Ekler Bölümüne Genel Bakış

Kitabın sonunda yer alan bu bölüm, Rust ile yazılım geliştirirken sık sık karşına çıkacak bazı referans tablolarını, listeleri ve ek bilgileri içerir. Temel amacı, "Acaba şu operatör ne işe yarıyordu?", "Hangi özellikleri otomatik türetebilirim?" veya "Rust'ın sürümleri arasındaki farklar neler?" gibi sorulara hızlıca cevap verebilmendir. Yani bir nevi el altındaki kılavuzdur.

Şimdi sırasıyla her bir eki detaylandıralım.

---

## Ek A: Anahtar Kelimeler (Keywords)

Her programlama dilinde olduğu gibi Rust’ta da dilin sözdizimi tarafından özel anlam yüklenmiş kelimeler vardır. Bunlara **anahtar kelime** diyoruz. Bu kelimeleri değişken ismi, fonksiyon ismi gibi tanımlayıcı olarak kullanamazsın (kullanmaya çalışırsan derleyici hata verir). Ek A, Rust’taki anahtar kelimeleri üç kategoriye ayırır:

### 1. Katı (Strict) Anahtar Kelimeler
Bunlar dilin her durumda özel anlam taşıyan, kesinlikle tanımlayıcı olarak kullanılamayan kelimelerdir. Şu anki Rust sürümünde şunları görürüz:

- **Temel akış kontrolü:** `as`, `break`, `const`, `continue`, `crate`, `else`, `enum`, `extern`, `false`, `fn`, `for`, `if`, `impl`, `in`, `let`, `loop`, `match`, `mod`, `move`, `mut`, `pub`, `ref`, `return`, `self`, `Self`, `static`, `struct`, `super`, `trait`, `true`, `type`, `unsafe`, `use`, `where`, `while`
- **Zamanlama ve eşzamansızlık:** `async`, `await`
- **Ödünç alma ve yaşam süreleri ile ilgili:** `dyn` (trait nesneleri için)

Örneğin `let` bir anahtar kelimedir; `let x = 5;` yazdığımızda Rust bunun bir değişken bağlama ifadesi olduğunu anlar. `let` isminde bir fonksiyon tanımlayamayız.

### 2. Rezerve Edilmiş (Reserved) Anahtar Kelimeler
Bunlar henüz dilde aktif olarak kullanılmayan, ancak ileride kullanılabileceği için tanımlayıcı olarak kullanılması yasaklanmış kelimelerdir. Şunları içerir: `abstract`, `become`, `box`, `do`, `final`, `macro`, `override`, `priv`, `try`, `typeof`, `unsized`, `virtual`, `yield`.

Bunları kullanmaya çalışırsan derleyici "şu anda kullanılmıyor ama rezerve" diyerek hata verir. Böylece dil geliştiricileri gelecekte bu kelimeleri dilin sözdizimine eklediklerinde mevcut kodlar bozulmaz.

### 3. Zayıf (Weak) Anahtar Kelimeler
Bunlar özel durumlarda anahtar kelime anlamı kazanan, ama çoğu yerde normal tanımlayıcı olarak kullanılabilen kelimelerdir. Şu an için iki tane var: `union` ve `'static` (yaşam süresi olarak kullanılan). `union` özel bir veri yapısı tanımlamak için kullanılır; ancak bağlam uygunsa değişken ismi olabilir. `'static` ise yaşam süresi belirtecidir, fakat tek başına bir tanımlayıcı olabilir.

Pratikte en çok katı anahtar kelimelerle işimiz olur. Bir isimlendirme yaparken aklında bulunsun: Eğer seçtiğin isim derlenmiyorsa, muhtemelen anahtar kelimeler listesinde olup olmadığını kontrol et.

---

## Ek B: Operatörler ve Semboller (Operators and Symbols)

Bu ek, Rust’ta kullanılan tüm operatörleri ve özel sembolleri anlamlarıyla birlikte tablo halinde sunar. Hem yeni başlayanlar hem de başka dillerden gelenler için son derece faydalı bir referanstır. Operatörleri kategorilerine göre açıklayalım:

### 1. Aritmetik Operatörler
| Operatör | Anlamı | Örnek |
| :---: | :--- | :--- |
| `+` | Toplama | `a + b` |
| `-` | Çıkarma | `a - b` |
| `*` | Çarpma | `a * b` |
| `/` | Bölme | `a / b` (tam sayılarda tam bölme) |
| `%` | Kalan (mod) | `a % b` |

Bunlar standart matematiksel işlemlerdir. Tamsayı bölmesi ondalık kısmı atar: `5 / 2` sonucu `2`’dir. Kayan noktalı sayılarda ise `5.0 / 2.0` = `2.5`.

### 2. Bit Düzeyinde Operatörler
| Operatör | Anlamı |
| :---: | :--- |
| `&` | Bitwise AND |
| `\|` | Bitwise OR |
| `^` | Bitwise XOR |
| `!` | Bitwise NOT (tek terimli) |
| `<<` | Sola kaydırma |
| `>>` | Sağa kaydırma |

`!` operatörü aynı zamanda mantıksal değil için de kullanılmaz; bitwise NOT’tur. Mantıksal değil için `!` değil, `!`ifade şeklinde kullanılır aslında... (burada dikkat: Rust'ta mantıksal değil `!` ile yapılır. Ama bitwise NOT da `!` ile yapılır. Bağlam farklıdır: `!true` mantıksal, `!0b1010` bitwise NOT olarak çalışır. Aslında aynı operatör, tipine göre davranır.)

### 3. Karşılaştırma ve Mantıksal Operatörler
| Operatör | Anlamı |
| :---: | :--- |
| `==` | Eşittir |
| `!=` | Eşit değildir |
| `>` , `<` | Büyüktür, küçüktür |
| `>=` , `<=` | Büyük-eşit, küçük-eşit |
| `&&` | Mantıksal VE (kısa devre) |
| `\|\|` | Mantıksal VEYA (kısa devre) |
| `!` | Mantıksal DEĞİL |

Karşılaştırma operatörleri `bool` döndürür. `&&` ve `||` kısa devre yapar: `false && fonksiyon()` çağrılmaz.

### 4. Atama ve Bileşik Atama Operatörleri
`=` temel atamadır. Rust'ta atama bir ifade değil, bir deyimdir (statement), yani değer döndürmez. Bileşik atamalar: `+=`, `-=`, `*=`, `/=`, `%=`, `&=`, `|=`, `^=`, `<<=`, `>>=` şeklindedir.

### 5. Referans ve İşaret Operatörleri
| Operatör / Sembol | Anlamı |
| :---: | :--- |
| `&` | Referans alma (borrow) |
| `&mut` | Değişebilir referans alma |
| `*` | Dereference (referansın gösterdiği değere erişim) |
| `ref` | Desen eşlemede referans ile bağlama |
| `mut` | Değişebilirlik belirteci |

Örnek: `let y = &x;` → `y` artık `x`'in referansıdır. `*y` ile `x`'in değerine ulaşırız.

### 6. Yapısal ve Sentaks Sembolleri
Bu kısımda operatörden çok Rust’ın sözdizimini oluşturan semboller bulunur:

- `->` : Fonksiyon dönüş tipi belirtir. `fn topla(a: i32, b: i32) -> i32`
- `=>` : Match kollarında desen ile kod bloğunu ayırır. Kapanış ifadelerinde (closure) de kullanılır.
- `::` : Yol (path) ayırıcı. Modül, ilişkili fonksiyon, sabit erişimi: `std::io::stdin()`, `String::new()`.
- `;` : İfadeyi sonlandırır, ifadeyi deyime çevirir. Dönüş değerini bastırır.
- `,` : Liste öğelerini ayırır.
- `.` : Alan erişimi, metod çağrısı.
- `..` , `..=` : Aralık (range) ifadeleri. `0..5` (0,1,2,3,4) `0..=5` (0,1,2,3,4,5)
- `?` : Hata yayma operatörü. `Result` veya `Option` döndüren bir ifadenin sonunda kullanılır; hata varsa fonksiyondan erken döner.
- `#` : Attribute (nitelik) başlatır: `#[derive(Debug)]`, `#![allow(unused)]`
- `_` : "Herhangi bir şey" deseni, kullanılmayan değişkenleri bağlamak için. `let _ = fonksiyon();`
- `!` : Makro çağrısı belirteci: `println!()`, `vec![]`. Ayrıca `!` türü hiçbir zaman dönmeyen (never type) olarak da vardır.

Bu liste oldukça uzun; her birini deneyerek öğrenmek en iyisi. Özetle, Ek B’yi açıp bir sembol gördüğünde "bu neydi?" dediğinde hemen bakabilirsin.

---

## Ek C: Türetilebilir Özellikler (Derivable Traits)

Rust’ta **trait** (özellik), bir tipin sahip olduğu davranışları tanımlayan bir yapıdır. Birçok standart trait, `#[derive(TraitAdı)]` niteliği ile yapı (struct) veya enum üzerinde otomatik olarak uygulanabilir. Bu bölüm, hangi trait’lerin türetilebileceğini ve ne işe yaradıklarını açıklar.

### Neden Derive Kullanırız?
Bir trait’i manuel olarak implemente etmek mümkündür, ancak çoğu durumda davranış mekanik ve tekrarlıdır. Örneğin `Debug` trait’i, tipimizin `{:?}` ile yazdırılabilmesi için gerekli `fmt` metodunu uygular. Bunu elle yazmak hataya açık ve sıkıcıdır; `derive` makrosu derleme zamanında bu kodu bizim için üretir.

İşte türetilebilen başlıca trait’ler ve anlamları:

### 1. `Debug`
Amaç: Hata ayıklama amaçlı formatlama. `println!("{:?}", nesne)` ile kullanılır. Hemen hemen her tipe türetilmesi tavsiye edilir, çünkü geliştirme sırasında iç yapıyı incelemek çok işe yarar.

```rust
#[derive(Debug)]
struct Nokta { x: i32, y: i32 }
let n = Nokta { x: 3, y: 5 };
println!("Nokta: {:?}", n); // Nokta { x: 3, y: 5 }
```

### 2. `Clone` ve `Copy`
- **Clone:** Bir değerin derinlemesine bir kopyasını oluşturmak için `.clone()` metodunu ekler. Pahalı olabilir, bu yüzden açık çağrı gerektirir.
- **Copy:** Atama ve fonksiyon argümanı geçişlerinde değerin otomatik olarak kopyalanmasını sağlar (bit düzeyinde kopya). Bir tip `Copy` ise `Clone` da olmalıdır. Basit, sabit boyutlu tipler (tamsayılar, karakter, bool, sadece Copy alanları olan yapılar) için uygundur. `String` gibi heap verisi tutanlar Copy olamaz, sadece Clone olabilir.

```rust
#[derive(Clone, Copy)]
struct Renk { r: u8, g: u8, b: u8 }
let kirmizi = Renk { r:255, g:0, b:0 };
let kopya = kirmizi; // Taşınma olmaz, otomatik kopyalanır.
```

### 3. `PartialEq` ve `Eq`
- **PartialEq:** `==` ve `!=` operatörlerini kullanabilmek için gereklidir. Bütün alanları karşılaştırarak eşitlik denetimi yapar.
- **Eq:** `PartialEq`’ın yanı sıra, tipin tüm değerler için eşitliğin yansıma özelliğine sahip olduğunu belirtir (yani `x == x` her zaman doğru). Matematiksel bir kesinlik gerektirir. `f32`/`f64` gibi NaN içeren tipler `Eq` olamaz çünkü NaN kendisine eşit değildir.

Genelde ikisi birlikte türetilir: `#[derive(PartialEq, Eq)]`

### 4. `PartialOrd` ve `Ord`
- **PartialOrd:** `<`, `>`, `<=`, `>=` karşılaştırma operatörlerini mümkün kılar. Alanların sırasına göre karşılaştırma yapar (ilk alandan başlayarak). Yine kayan noktalı sayılarda NaN nedeniyle kısmi sıralama geçerlidir.
- **Ord:** Tam sıralama. Bütün değerler karşılaştırılabilir ve sıralanabilir. `Ord` türetebilmek için `PartialOrd` ve `Eq` da türetilmelidir.

Örnek: Bir liderlik tablosu sıralamak için `Ord` kullanışlıdır.

### 5. `Hash`
Bir tipin hash değerinin hesaplanabilmesini sağlar. `HashMap` veya `HashSet` içinde anahtar olarak kullanılacaksa gereklidir. Eğer `Hash` türetirsen, aynı zamanda `PartialEq` ve `Eq` da türetmelisin (hash çakışmalarında eşitlik kontrolü için).

### 6. `Default`
Bir tip için varsayılan bir değer oluşturan `default()` metodunu ekler. Her alanın kendi `Default` değerini çağırır. Örneğin `i32` için `0`, `String` için boş string. `..Default::default()` ile yapı güncelleme sözdiziminde kullanılabilir:

```rust
#[derive(Default)]
struct Ayarlar { ses: u8, tema: String }
let a = Ayarlar { ses: 50, ..Default::default() };
```

### Ek Bilgiler
Bu trait’ler dışında, ilerleyen Rust sürümlerinde veya belirli kütüphanelerde ek türetilebilir trait’ler olabilir. Ayrıca, derive makroları kendi özel trait’lerin için de yazılabilir (prosedürel makrolar ile), ancak bu ileri seviye bir konudur.

---

## Ek D: Faydalı Geliştirme Araçları (Useful Development Tools)

Rust ekosistemi, yazılım geliştirme sürecini kolaylaştıran bir dizi resmi ve topluluk aracı sunar. Ek D, bu araçların kısa tanıtımını yapar.

### 1. `rustfmt` (Biçimlendirici)
Kodunu otomatik olarak Rust’ın stil rehberine göre biçimlendirir. Takım çalışmasında tutarlılık sağlar. `cargo fmt` komutuyla çalıştırılır. Genellikle editör entegrasyonu ile her kaydetmede otomatik çalışacak şekilde ayarlanır.

### 2. `rustfix` (Otomatik Düzeltme)
Derleyici tarafından önerilen düzeltmeleri otomatik olarak uygular. Özellikle yeni bir Rust sürümüne geçerken veya `cargo check` ile gelen uyarıları düzeltirken kullanışlıdır. `cargo fix` komutu ile çalışır.

### 3. Clippy (Deyim Denetleyici - Linter)
Kodundaki yaygın hataları, performans sorunlarını, deyimsel olmayan kullanımları belirten bir statik analiz aracıdır. `cargo clippy` ile çalıştırılır. Örneğin, `if koşul == true` yerine doğrudan `if koşul` demeni önerir. Çıktısı eğiticidir; Rust’ı daha iyi yazmayı öğretir.

### 4. `rust-analyzer` (Dil Sunucusu)
Modern IDE özellikleri (kod tamamlama, tanıma git, hata vurgulama, satır içi tip ipuçları) sağlayan bir dil sunucusudur. VS Code, Emacs, Vim gibi editörlerle kullanılır. Resmi olarak Rust projesi tarafından geliştirilir ve günümüzde standart haline gelmiştir.

### 5. Diğer Araçlar
- `cargo doc`: Dokümantasyonu yerel olarak oluşturur. `rustdoc` aracını kullanarak kodundaki yorumlardan HTML belgeleri üretir.
- `cargo bench`: Benchmark testlerini çalıştırır (gecelik Rust gerektirebilir).
- `cargo build` / `run` / `test` zaten temel iş akışının parçası.

Bu araçlar Rust’ta geliştirme deneyimini son derece keyifli ve verimli kılar. Hepsini erkenden kurup kullanmayı alışkanlık haline getirmeni öneririm.

---

## Ek E: Sürümler (Editions)

Rust, dilde geriye dönük uyumlu olmayan değişiklikleri yönetmek için **edition** (sürüm) sistemini kullanır. Bu, büyük kırılımlar olmadan dilin evrilmesini sağlar. Ek E, şu ana kadarki sürümleri ve aralarındaki farkları açıklar.

### Neden Sürümler Var?
Dil geliştiricileri bazen sözdiziminde iyileştirme yapmak ister (örneğin yeni anahtar kelimeler eklemek). Bunu doğrudan yapmak mevcut kodu bozar. Bunun yerine, yeni değişiklikler bir **edition** altında toplanır. Projeler `Cargo.toml` dosyasında hangi edition’ı kullandığını belirtir (`edition = "2021"` gibi). Eski edition’daki kod, yeni derleyiciyle hâlâ derlenmeye devam eder. Aynı proje içinde farklı edition’daki bağımlılıklar bir arada çalışabilir.

### Başlıca Sürümler ve Önemli Farklılıklar
- **2015:** Orijinal Rust 1.0 ile gelen, varsayılan ilk edition.
- **2018:** Pek çok sözdizimi iyileştirmesi getirdi:
  - `dyn` ve `impl Trait` sözdizimi.
  - Modül sistemi değişiklikleri (`mod.rs` zorunluluğunun kalkması, `use` yollarının daha tutarlı hale gelmesi).
  - `async`/`await` temelleri (kararlı hale gelmesi daha sonra).
  - `?` operatörünün `main` ve `test` fonksiyonlarında kullanılabilmesi gibi.
- **2021:** Daha küçük ama önemli değişiklikler:
  - `IntoIterator` diziler için implemente edildi; `for i in [1,2,3]` artık referans almadan çalışır.
  - Kapanışların (closure) yakalama kuralları değişti (daha az `move` gerektiren, alan bazlı yakalama).
  - `panic!` makrosunun formatlama değişiklikleri.
  - Uyarıların daha katı hale gelmesi.
- **2024:** (Bu yanıtın yazıldığı zaman itibarıyla taslak aşamasındaydı, ancak kitap güncellemeleri içerebilir) Yeni özellikler arasında `gen` anahtar kelimesi, `impl Trait` iyileştirmeleri gibi konular yer alabilir.

Pratikte, yeni bir projeye başlarken daima en son kararlı edition’ı kullanırsın. Eski projeleri yeni edition’a taşımak için `cargo fix --edition` kullanarak otomatik dönüşüm yapabilirsin.

---

## Ek F: Kitabın Çevirileri (Translations of the Book)

Bu bölüm, "The Rust Programming Language" kitabının farklı dillere yapılmış topluluk çevirilerine bağlantılar verir. Türkçe çeviri de bulunabilir (topluluk tarafından yapılmakta ve güncelliği değişebilmektedir). Eğer İngilizce dışında bir dilde okumayı tercih ediyorsan bu kaynaklara bakabilirsin. Teknik içerik açısından bu ek ders niteliğinde olmaktan çok bilgilendirme amaçlıdır.

---

## Ek G: Rust Nasıl Geliştiriliyor ve “Nightly Rust” (How Rust is Made and “Nightly Rust”)

Bu ek Rust’ın açık kaynak geliştirme sürecini ve yayın kanallarını açıklar. Dilin nasıl evrildiğini anlamak açısından değerlidir.

### Yayın Kanalları (Channels)
Rust derleyicisi üç kanaldan dağıtılır:
1. **Stable (Kararlı):** Çoğu kullanıcının kullandığı, altı haftada bir yayınlanan sürümdür. Tüm özellikler kararlıdır, geriye dönük uyumluluk garantisi vardır.
2. **Beta:** Bir sonraki kararlı sürümün önizlemesidir. Test amaçlı kullanılır.
3. **Nightly (Gecelik):** Her gece otomatik olarak derlenen en güncel sürümdür. Deneysel özellikler içerir. Bu özellikleri kullanmak için `#![feature(...)]` niteliği eklenir ve yalnızca Nightly derleyicide çalışır.

### Geliştirme Süreci
Rust, **RFC (Request For Comments)** süreci ile yönetilir. Büyük değişiklikler, yeni dil özellikleri topluluk tarafından önerilir, tartışılır ve resmi ekipler tarafından onaylanır. Uygulama genellikle önce Nightly’de deneysel olarak yer alır, sonra kararlı hale getirilir.

Bu süreç sayesinde Rust, hem yenilikçi hem de son derece güvenilir bir dil olarak kalmayı başarır. Sen de bir gün RFC yazıp dile katkıda bulunabilirsin!

---

## Kapanış

İşte böylece kitabın Ekler bölümünü baştan sona detaylı bir şekilde öğrenmiş olduk. Bu kısım, Rust yolculuğunda masanın üstünde açık durması gereken bir harita gibidir. Herhangi bir sembolü unuttuğunda, bir trait’in türetilip türetilemediğini merak ettiğinde veya araçları keşfetmek istediğinde buraya danışabilirsin.

Umarım dersimiz faydalı olmuştur. Bir sonraki konuda buluşana dek bol pratik yapmayı unutma!