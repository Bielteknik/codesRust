# 🎭 Rust for .NET Developers: Meta-programming (Üst Düzey Programlama)

Bu bölüm, Rust'ın güçlü **makro sistemi** ve **meta-programming** yeteneklerini inceler. C# dünyasında T4 Templates, Source Generators, Roslyn API ve reflection tabanlı çözümlerle meta-programming yapmaya alışkınız. Rust ise **compile-time**'da çalışan, **type-safe** ve **zero-cost** iki farklı makro sistemi sunar.

> 🎯 **Temel Fark:** C#'ta meta-programming genellikle **runtime reflection** veya **Source Generators** (Roslyn) ile yapılır. Rust'ta ise **compile-time macro expansion** ile yapılır - sıfır runtime maliyeti! Rust'ta iki tür makro vardır: **Declarative** (`macro_rules!`) ve **Procedural** (derive, attribute, function-like).

---

# 📚 BÖLÜM 1: Meta-programming Nedir ve Neden Önemlidir?

## 1.1 Tanım

**Meta-programming**: Kodun, başka kod ürettiği veya kodun yapısını derleme zamanında değiştirdiği programlama tekniğidir.

## 1.2 Kullanım Alanları

1. **Code generation**: Tekrarlayan kodları otomatik üretme
2. **DSL (Domain-Specific Language)**: Kendi mini dilinizi oluşturma
3. **Derive macros**: Trait implementasyonlarını otomatik yapma
4. **Attribute macros**: Fonksiyon/struct davranışını değiştirme
5. **Compile-time validation**: Derleme zamanında doğrulama
6. **Serialization/Deserialization**: `serde` gibi kütüphaneler
7. **Async runtime**: `#[tokio::main]` gibi attribute'lar

## 1.3 C# vs Rust Yaklaşımı

| Özellik | C# | Rust |
|---|---|---|
| Temel yaklaşım | Reflection + Source Generators | Compile-time macro expansion |
| Declarative macros | ❌ Yok | ✅ `macro_rules!` |
| Procedural macros | Source Generators (Roslyn) | Proc-macro crate'leri |
| Derive macros | ❌ Yok (manuel veya generator) | ✅ `#[derive(...)]` |
| Attribute macros | ❌ Yok | ✅ `#[attribute]` |
| Function-like macros | ❌ Yok | ✅ `makro_adi!()` |
| Runtime overhead | ⚠️ Reflection var | ✅ Sıfır (compile-time) |
| Type safety | ⚠️ Runtime kontrol | ✅ Compile-time kontrol |
| IDE desteği | ✅ Mükemmel (Roslyn) | ✅ İyi (rust-analyzer) |
| Hata mesajları | ✅ İyi | ⚠️ Bazen karmaşık |
| Embedded desteği | ❌ | ✅ (no_std uyumlu) |

---

# 📚 BÖLÜM 2: Rust Makro Sistemi - Genel Bakış

Rust'ta **iki ana makro türü** vardır:

## 2.1 Makro Türleri

```
┌─────────────────────────────────────────────────────────┐
│ Rust Makro Sistemi                                      │
├─────────────────────────────────────────────────────────┤
│ 1. Declarative Macros (macro_rules!)                    │
│    ├─ Pattern matching tabanlı                          │
│    ├─ Basit kod üretimi                                 │
│    ├─ Ayrı crate gerekmez                               │
│    └─ "Macro by Example"                                │
├─────────────────────────────────────────────────────────┤
│ 2. Procedural Macros                                    │
│    ├─ Token stream alır, Token stream döndürür          │
│    ├─ Ayrı crate gerekir (proc-macro = true)            │
│    ├─ Üç alt tür:                                       │
│    │   ├─ Derive Macros: #[derive(Trait)]                │
│    │   ├─ Attribute Macros: #[attribute]                 │
│    │   └─ Function-like Macros: makro!()                 │
│    └─ Tam programatik kontrol                           │
└─────────────────────────────────────────────────────────┘
```

## 2.2 C# Karşılıkları

| Rust Makro Türü | C# Karşılığı |
|---|---|
| `macro_rules!` | T4 Templates (benzer ama daha güçlü) |
| `#[derive(Trait)]` | Source Generators (manuel implementasyon yerine) |
| `#[attribute]` | Source Generators + Attributes |
| `makro!()` | T4 Templates veya Source Generators |

---

# 📚 BÖLÜM 3: Declarative Macros (`macro_rules!`) ⭐

Declarative makrolar, **pattern matching** tabanlı kod üretimidir. C#'taki T4 Templates'e benzer ama daha güçlüdür.

## 3.1 Temel Syntax

```rust
macro_rules! makro_adi {
    // Kural 1
    (pattern1) => {
        // Üretilecek kod
    };
    
    // Kural 2
    (pattern2) => {
        // Üretilecek kod
    };
}
```

## 3.2 İlk Declarative Macro

```rust
// Basit bir "merhaba" makrosu
macro_rules! selam_ver {
    () => {
        println!("Merhaba Dünya!");
    };
    ($isim:expr) => {
        println!("Merhaba, {}!", $isim);
    };
    ($isim:expr, $yas:expr) => {
        println!("Merhaba, {}! Yaşın: {}", $isim, $yas);
    };
}

fn main() {
    selam_ver!();                    // Merhaba Dünya!
    selam_ver!("Ali");               // Merhaba, Ali!
    selam_ver!("Veli", 25);          // Merhaba, Veli! Yaşın: 25
}
```

## 3.3 Metavariables (Meta Değişkenler)

Makrolarda `$` ile başlayan meta değişkenler kullanılır:

```rust
macro_rules! degisken_ornek {
    ($x:expr) => {
        // $x bir ifade (expression)
        let sonuc = $x;
        println!("Sonuç: {}", sonuc);
    };
}
```

### Meta Değişken Türleri

| Fragment Specifier | Açıklama | Örnek |
|---|---|---|
| `expr` | İfade | `x + 1`, `foo()` |
| `ty` | Tür | `i32`, `String` |
| `ident` | Tanımlayıcı | `degisken_adi` |
| `pat` | Pattern | `Some(x)`, `_` |
| `stmt` | İfade (statement) | `let x = 5;` |
| `block` | Blok | `{ ... }` |
| `path` | Yol | `std::collections::HashMap` |
| `tt` | Token tree (herhangi bir token) | Her şey |
| `item` | Öğe (fonksiyon, struct, vs.) | `fn foo() {}` |
| `meta` | Meta (attribute içeriği) | `derive(Debug)` |
| `lifetime` | Lifetime | `'a`, `'static` |
| `vis` | Visibility | `pub`, `pub(crate)` |
| `literal` | Değişmez | `42`, `"merhaba"` |

## 3.4 Tekrar (Repetition)

Makrolarda tekrar için `*`, `+`, `?` kullanılır:

```rust
// Sıfır veya daha fazla tekrar (*)
macro_rules! vektor {
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

fn main() {
    let v = vektor![1, 2, 3, 4, 5];
    println!("{:?}", v); // [1, 2, 3, 4, 5]
    
    let bos = vektor![];
    println!("{:?}", bos); // []
}
```

### Tekrar Operatörleri

| Operatör | Açıklama |
|---|---|
| `*` | Sıfır veya daha fazla |
| `+` | Bir veya daha fazla |
| `?` | Sıfır veya bir |

### Ayırıcılar (Separators)

```rust
macro_rules! ornekler {
    // Virgülle ayrılmış
    ( $( $x:expr ),* ) => { /* ... */ };
    
    // Noktalı virgülle ayrılmış
    ( $( $x:expr );* ) => { /* ... */ };
    
    // Boşlukla ayrılmış (ayırıcı yok)
    ( $( $x:expr )* ) => { /* ... */ };
}
```

## 3.5 Pratik Declarative Macro Örnekleri

### 3.5.1 `vec!` Benzeri Makro

```rust
macro_rules! liste {
    // Boş liste
    () => {
        Vec::new()
    };
    
    // Tekrarlayan elemanlar
    ( $elem:expr ; $count:expr ) => {
        std::vec::from_elem($elem, $count)
    };
    
    // Eleman listesi
    ( $( $x:expr ),+ $(,)? ) => {
        {
            let mut temp = Vec::new();
            $(
                temp.push($x);
            )+
            temp
        }
    };
}

fn main() {
    let v1 = liste![1, 2, 3];
    let v2 = liste![0; 5];  // [0, 0, 0, 0, 0]
    let v3 = liste![];
    
    println!("{:?}", v1); // [1, 2, 3]
    println!("{:?}", v2); // [0, 0, 0, 0, 0]
    println!("{:?}", v3); // []
}
```

### 3.5.2 Assertion Makrosu

```rust
macro_rules! assert_esit {
    ($sol:expr, $sag:expr) => {
        if $sol != $sag {
            panic!(
                "Assertion başarısız: {} != {} ({}:{}:{})",
                stringify!($sol),
                stringify!($sag),
                file!(),
                line!(),
                column!()
            );
        }
    };
    ($sol:expr, $sag:expr, $mesaj:expr) => {
        if $sol != $sag {
            panic!(
                "{}: {} != {} ({}:{}:{})",
                $mesaj,
                stringify!($sol),
                stringify!($sag),
                file!(),
                line!(),
                column!()
            );
        }
    };
}

fn main() {
    assert_esit!(2 + 2, 4);
    assert_esit!(5, 5, "Beşler eşit olmalı");
    // assert_esit!(1, 2); // Panic!
}
```

### 3.5.3 Newtype Pattern Makrosu

```rust
macro_rules! new_type {
    ($isim:ident, $temel_ty:ty) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $isim(pub $temel_ty);
        
        impl $isim {
            pub fn new(deger: $temel_ty) -> Self {
                $isim(deger)
            }
            
            pub fn deger(&self) -> &$temel_ty {
                &self.0
            }
        }
        
        impl From<$temel_ty> for $isim {
            fn from(deger: $temel_ty) -> Self {
                $isim(deger)
            }
        }
        
        impl std::ops::Deref for $isim {
            type Target = $temel_ty;
            
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

// Kullanım
new_type!(Metre, f64);
new_type!(Saniye, f64);
new_type!(RPM, u16);

fn main() {
    let mesafe = Metre::new(100.0);
    let sure = Saniye::new(10.0);
    let hiz = RPM::new(1500);
    
    println!("Mesafe: {:?}", mesafe);
    println!("Süre: {:?}", sure);
    println!("Hız: {:?}", hiz);
}
```

### 3.5.4 HashMap Oluşturma Makrosu

```rust
macro_rules! hashmap {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}

fn main() {
    let config = hashmap! {
        "host" => "localhost",
        "port" => "8080",
        "debug" => "true"
    };
    
    println!("{:?}", config);
}
```

## 3.6 C# vs macro_rules! Karşılaştırması

**C# (T4 Template)**:
```csharp
// T4 Template (.t4 dosyası)
<# for (int i = 1; i <= 5; i++) { #>
public class Entity<#= i #> {
    public int Id { get; set; }
    public string Name<#= i #> { get; set; }
}
<# } #>
```

**Rust (macro_rules!)**:
```rust
macro_rules! entity_olustur {
    ( $( $id:literal ),+ ) => {
        $(
            paste::paste! {
                pub struct [<Entity $id>] {
                    pub id: i32,
                    pub name: String,
                }
            }
        )+
    };
}

entity_olustur!(1, 2, 3, 4, 5);
```

---

# 📚 BÖLÜM 4: Procedural Macros ⭐⭐

Procedural makrolar, **TokenStream** alıp **TokenStream** döndüren fonksiyonlardır. C#'taki Source Generators'a benzer ama daha esnektir.

## 4.1 Üç Tür Procedural Macro

```rust
// 1. Derive Macro
#[derive(MyTrait)]
struct Yapim { }

// 2. Attribute Macro
#[my_attribute]
fn fonksiyon() { }

// 3. Function-like Macro
makro_adi!(arg1, arg2);
```

## 4.2 Proc-Macro Crate Oluşturma

Procedural makrolar **ayrı bir crate** gerektirir:

```bash
# Ana proje
cargo new my_app

# Proc-macro crate
cargo new my_macros --lib
```

**my_macros/Cargo.toml**:
```toml
[package]
name = "my_macros"
version = "0.1.0"
edition = "2021"

[lib]
proc-macro = true  # ÖNEMLİ: Bu crate bir proc-macro

[dependencies]
syn = { version = "2.0", features = ["full"] }
quote = "1.0"
proc-macro2 = "1.0"
```

**Ana proje Cargo.toml**:
```toml
[dependencies]
my_macros = { path = "../my_macros" }
```

## 4.3 İlk Derive Macro

**my_macros/src/lib.rs**:
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Describe)]
pub fn describe_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    
    // Struct/Enum ismi
    let name = &input.ident;
    
    // String olarak isim
    let name_str = name.to_string();
    
    // Alan sayısını bul (sadece struct için)
    let field_count = match &input.data {
        syn::Data::Struct(data) => data.fields.len(),
        syn::Data::Enum(_) => 0,
        syn::Data::Union(_) => 0,
    };
    
    // Kod üret
    let expanded = quote! {
        impl #name {
            pub fn describe(&self) -> String {
                format!(
                    "Struct adı: {}, Alan sayısı: {}",
                    #name_str,
                    #field_count
                )
            }
        }
    };
    
    // Token stream olarak döndür
    TokenStream::from(expanded)
}
```

**Ana projede kullanım**:
```rust
use my_macros::Describe;

#[derive(Describe)]
struct Motor {
    hiz: u16,
    ivme: u16,
    pozisyon: i32,
}

fn main() {
    let motor = Motor {
        hiz: 1500,
        ivme: 100,
        pozisyon: 0,
    };
    
    println!("{}", motor.describe());
    // Çıktı: Struct adı: Motor, Alan sayısı: 3
}
```

## 4.4 syn, quote ve proc-macro2

Procedural makrolar için üç temel crate:

### 4.4.1 `syn` - Rust Kodunu Parse Etme

```rust
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(MyDerive)]
pub fn my_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    // Struct adı
    let name = &input.ident;
    
    // Alanları incele
    match &input.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields) => {
                    for field in &fields.named {
                        let field_name = field.ident.as_ref().unwrap();
                        let field_type = &field.ty;
                        println!("Alan: {} : {:?}", field_name, field_type);
                    }
                }
                Fields::Unnamed(fields) => {
                    println!("Tuple struct: {} alan", fields.unnamed.len());
                }
                Fields::Unit => {
                    println!("Unit struct");
                }
            }
        }
        Data::Enum(data_enum) => {
            for variant in &data_enum.variants {
                println!("Variant: {}", variant.ident);
            }
        }
        Data::Union(_) => {
            println!("Union (desteklenmiyor)");
        }
    }
    
    TokenStream::new()
}
```

### 4.4.2 `quote` - Kod Üretme

```rust
use quote::quote;

let name = syn::Ident::new("Motor", proc_macro2::Span::call_site());
let field_count = 3;

// Kod üret
let expanded = quote! {
    impl #name {
        pub fn field_count(&self) -> usize {
            #field_count
        }
        
        pub fn name(&self) -> &'static str {
            stringify!(#name)
        }
    }
};

// TokenStream'e çevir
let tokens: proc_macro2::TokenStream = expanded.into();
```

### 4.4.3 `proc-macro2` - Token Manipülasyonu

```rust
use proc_macro2::{TokenStream, Span, Ident};

// Identifier oluştur
let ident = Ident::new("yeni_degisken", Span::call_site());

// Literal oluştur
let literal = proc_macro2::Literal::u32_unsuffixed(42);

// Token stream birleştir
let mut tokens = TokenStream::new();
tokens.extend(quote! {
    let #ident = #literal;
});
```

## 4.5 Gelişmiş Derive Macro Örneği

### 4.5.1 Builder Pattern Generator

```rust
// my_macros/src/lib.rs
use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(Builder)]
pub fn builder_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let builder_name = format_ident!("{}Builder", name);
    
    // Sadece named fields destekle
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("Builder sadece named fields destekler"),
        },
        _ => panic!("Builder sadece struct'lar için"),
    };
    
    // Builder struct alanları
    let builder_fields = fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_type = &f.ty;
        quote! {
            #field_name: Option<#field_type>
        }
    });
    
    // Builder setter metodları
    let builder_setters = fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_type = &f.ty;
        quote! {
            pub fn #field_name(mut self, value: #field_type) -> Self {
                self.#field_name = Some(value);
                self
            }
        }
    });
    
    // Build metodundaki alan kontrolleri
    let build_checks = fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_name_str = field_name.as_ref().unwrap().to_string();
        quote! {
            let #field_name = self.#field_name
                .ok_or(format!("{} alanı eksik", #field_name_str))?;
        }
    });
    
    // Build metodundaki struct oluşturma
    let build_fields = fields.iter().map(|f| {
        let field_name = &f.ident;
        quote! {
            #field_name
        }
    });
    
    let expanded = quote! {
        pub struct #builder_name {
            #(#builder_fields),*
        }
        
        impl #builder_name {
            pub fn new() -> Self {
                Self {
                    #(#fields: None),*
                }
            }
            
            #(#builder_setters)*
            
            pub fn build(self) -> Result<#name, String> {
                #(#build_checks)*
                
                Ok(#name {
                    #(#build_fields),*
                })
            }
        }
        
        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name::new()
            }
        }
    };
    
    TokenStream::from(expanded)
}
```

**Kullanım**:
```rust
use my_macros::Builder;

#[derive(Builder, Debug)]
struct Motor {
    hiz: u16,
    ivme: u16,
    pozisyon: i32,
}

fn main() {
    let motor = Motor::builder()
        .hiz(1500)
        .ivme(100)
        .pozisyon(500)
        .build()
        .unwrap();
    
    println!("{:?}", motor);
    // Motor { hiz: 1500, ivme: 100, pozisyon: 500 }
}
```

## 4.6 Attribute Macros

Attribute makrolar, fonksiyonları, struct'ları veya enum'ları değiştirmek için kullanılır.

### 4.6.1 Timer Attribute

```rust
// my_macros/src/lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn timed(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    
    let name = &input.sig.ident;
    let block = &input.block;
    let inputs = &input.sig.inputs;
    let output = &input.sig.output;
    let vis = &input.vis;
    let attrs = &input.attrs;
    
    let expanded = quote! {
        #(#attrs)*
        #vis fn #name(#inputs) #output {
            let start = std::time::Instant::now();
            let result = (|| #block)();
            let elapsed = start.elapsed();
            println!("{}: {:?}", stringify!(#name), elapsed);
            result
        }
    };
    
    TokenStream::from(expanded)
}
```

**Kullanım**:
```rust
use my_macros::timed;

#[timed]
fn yavas_fonksiyon() -> i32 {
    std::thread::sleep(std::time::Duration::from_millis(100));
    42
}

fn main() {
    let sonuc = yavas_fonksiyon();
    // Çıktı: yavas_fonksiyon: 100.123ms
    println!("Sonuç: {}", sonuc);
}
```

### 4.6.2 Retry Attribute

```rust
#[proc_macro_attribute]
pub fn retry(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let max_retries: usize = attr.to_string().parse().unwrap_or(3);
    
    let name = &input.sig.ident;
    let block = &input.block;
    let inputs = &input.sig.inputs;
    let output = &input.sig.output;
    let vis = &input.vis;
    
    let expanded = quote! {
        #vis fn #name(#inputs) #output {
            let mut deneme = 0;
            loop {
                match (|| #block)() {
                    Ok(val) => return Ok(val),
                    Err(e) => {
                        deneme += 1;
                        if deneme >= #max_retries {
                            return Err(e);
                        }
                        eprintln!("Deneme {} başarısız: {:?}", deneme, e);
                        std::thread::sleep(std::time::Duration::from_millis(100 * deneme as u64));
                    }
                }
            }
        }
    };
    
    TokenStream::from(expanded)
}
```

**Kullanım**:
```rust
#[retry(5)]
fn kararsiz_islem() -> Result<String, String> {
    static COUNTER: std::sync::atomic::AtomicUsize = 
        std::sync::atomic::AtomicUsize::new(0);
    
    let count = COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    
    if count < 3 {
        Err("Geçici hata".to_string())
    } else {
        Ok("Başarılı!".to_string())
    }
}
```

## 4.7 Function-like Macros

```rust
// my_macros/src/lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    let sql_string = parse_macro_input!(input as LitStr);
    let sql_value = sql_string.value();
    
    let expanded = quote! {
        {
            println!("SQL çalıştırılıyor: {}", #sql_value);
            // Burada gerçek SQL çalıştırma kodu olabilir
            Ok::<_, String>(())
        }
    };
    
    TokenStream::from(expanded)
}
```

**Kullanım**:
```rust
use my_macros::sql;

fn main() {
    sql!("SELECT * FROM users WHERE id = 1").unwrap();
    // Çıktı: SQL çalıştırılıyor: SELECT * FROM users WHERE id = 1
}
```

---

# 📚 BÖLÜM 5: Popüler Procedural Macro Crate'leri ⭐

## 5.1 `serde` - Serialization/Deserialization

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct MotorDurumu {
    hiz: u16,
    pozisyon: i32,
    aktif: bool,
}

fn main() {
    let durum = MotorDurumu {
        hiz: 1500,
        pozisyon: 500,
        aktif: true,
    };
    
    // JSON'a çevir
    let json = serde_json::to_string(&durum).unwrap();
    println!("JSON: {}", json);
    
    // JSON'dan geri çevir
    let parsed: MotorDurumu = serde_json::from_str(&json).unwrap();
    println!("Parsed: {:?}", parsed);
}
```

## 5.2 `thiserror` - Error Handling

```toml
[dependencies]
thiserror = "1.0"
```

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MotorHatasi {
    #[error("Motor hızı çok yüksek: {0} RPM (max: {max})")]
    HizHatasi { hiz: u16, max: u16 },
    
    #[error("Pozisyon sınır dışında: {pozisyon} (min: {min}, max: {max})")]
    PozisyonHatasi {
        pozisyon: i32,
        min: i32,
        max: i32,
    },
    
    #[error("İletişim hatası: {0}")]
    IletisimHatasi(#[from] std::io::Error),
    
    #[error("Bilinmeyen hata")]
    Bilinmeyen,
}

fn motor_kontrol(hiz: u16) -> Result<(), MotorHatasi> {
    if hiz > 3000 {
        return Err(MotorHatasi::HizHatasi { hiz, max: 3000 });
    }
    Ok(())
}

fn main() {
    match motor_kontrol(4000) {
        Ok(_) => println!("Başarılı"),
        Err(e) => println!("Hata: {}", e),
    }
    // Çıktı: Hata: Motor hızı çok yüksek: 4000 RPM (max: 3000)
}
```

## 5.3 `clap` - Command Line Parsing

```toml
[dependencies]
clap = { version = "4.4", features = ["derive"] }
```

```rust
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "motor-kontrol")]
#[command(about = "Step motor kontrol aracı")]
struct Args {
    /// Motor hızı (RPM)
    #[arg(short, long, default_value_t = 1000)]
    hiz: u16,
    
    /// Hedef pozisyon (mm)
    #[arg(short, long)]
    hedef: Option<f64>,
    
    /// Debug modu
    #[arg(short, long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
```

## 5.4 `strum` - Enum Utilities

```toml
[dependencies]
strum = { version = "0.25", features = ["derive"] }
strum_macros = "0.25"
```

```rust
use strum_macros::{Display, EnumString, EnumIter};

#[derive(Debug, Display, EnumString, EnumIter)]
enum MotorDurumu {
    #[strum(serialize = "BOSTA")]
    Bosta,
    
    #[strum(serialize = "ÇALIŞIYOR")]
    Calisiyor,
    
    #[strum(serialize = "HATA")]
    Hata,
    
    #[strum(serialize = "BAKIMDA")]
    Bakimda,
}

fn main() {
    // String'den enum'a
    let durum: MotorDurumu = "ÇALIŞIYOR".parse().unwrap();
    println!("{:?}", durum);
    
    // Enum'dan string'e
    println!("{}", MotorDurumu::Bosta);
    
    // Tüm variant'ları yinele
    for durum in MotorDurumu::iter() {
        println!("{}", durum);
    }
}
```

## 5.5 `derive_more` - Ek Trait'ler

```toml
[dependencies]
derive_more = "0.99"
```

```rust
use derive_more::{Display, From, Into, Add, Mul};

#[derive(Debug, Display, From, Into, Add, Mul)]
struct Metre(f64);

#[derive(Debug, Display, From, Into)]
struct Saniye(f64);

fn main() {
    let m1 = Metre::from(10.0);
    let m2 = Metre::from(20.0);
    
    // Add trait
    let m3 = m1 + m2;
    println!("Toplam: {}", m3);  // 30
    
    // Mul trait
    let m4 = m1 * 2.0;
    println!("Çarpım: {}", m4);  // 20
    
    // Into trait
    let deger: f64 = m1.into();
    println!("f64: {}", deger);  // 10
}
```

---

# 📚 BÖLÜM 6: Compile-Time Programming ⭐⭐

Rust'ın makro sistemi, **compile-time programming** için çok güçlü araçlar sunar.

## 6.1 `include_str!` ve `include_bytes!`

```rust
// Dosya içeriğini compile-time'da dahil et
const CONFIG: &str = include_str!("config.toml");
const LOGO: &[u8] = include_bytes!("logo.png");

fn main() {
    println!("Config: {}", CONFIG);
    println!("Logo boyutu: {} bytes", LOGO.len());
}
```

## 6.2 `env!` Macro

```rust
// Compile-time environment variable okuma
const VERSION: &str = env!("CARGO_PKG_VERSION");
const BUILD_DATE: &str = env!("BUILD_DATE");  // build.rs'de ayarlanır

fn main() {
    println!("Versiyon: {}", VERSION);
    println!("Build tarihi: {}", BUILD_DATE);
}
```

## 6.3 `concat!` ve `stringify!`

```rust
fn main() {
    // String birleştirme (compile-time)
    let mesaj = concat!("Merhaba", " ", "Dünya", "!");
    println!("{}", mesaj);
    
    // İfadeyi string'e çevirme (compile-time)
    let ifade = stringify!(2 + 2);
    println!("İfade: {}", ifade);  // "2 + 2"
    
    // Line, file, column bilgileri
    println!("{}:{}:{}", file!(), line!(), column!());
}
```

## 6.4 `cfg!` Macro

```rust
fn main() {
    // Compile-time koşul kontrolü
    if cfg!(target_os = "windows") {
        println!("Windows'ta çalışıyor");
    } else if cfg!(target_os = "linux") {
        println!("Linux'ta çalışıyor");
    }
    
    if cfg!(debug_assertions) {
        println!("Debug modu");
    } else {
        println!("Release modu");
    }
}
```

## 6.5 Build Scripts (build.rs) ile Compile-Time Code Generation

**build.rs**:
```rust
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Build timestamp oluştur
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("build_info.rs");
    
    let build_date = chrono::Utc::now().to_rfc3339();
    let git_hash = std::process::Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .map(|o| String::from_utf8(o.stdout).unwrap().trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string());
    
    let code = format!(
        r#"
pub const BUILD_DATE: &str = "{}";
pub const GIT_HASH: &str = "{}";
pub const VERSION: &str = "{}";
"#,
        build_date,
        git_hash,
        env::var("CARGO_PKG_VERSION").unwrap()
    );
    
    fs::write(&dest_path, code).unwrap();
    
    println!("cargo:rustc-env=BUILD_DATE={}", build_date);
    println!("cargo:rerun-if-changed=build.rs");
}
```

**main.rs**:
```rust
// build.rs'de oluşturulan modülü dahil et
include!(concat!(env!("OUT_DIR"), "/build_info.rs"));

fn main() {
    println!("Build tarihi: {}", BUILD_DATE);
    println!("Git hash: {}", GIT_HASH);
    println!("Versiyon: {}", VERSION);
}
```

---

# 📚 BÖLÜM 7: Advanced Macro Patterns ⭐⭐

## 7.1 Recursive Macros

```rust
// İç içe yapılar için recursive makro
macro_rules! sum {
    // Tek eleman
    ($x:expr) => { $x };
    
    // Birden fazla eleman
    ($x:expr, $($rest:expr),+) => {
        $x + sum!($($rest),+)
    };
}

fn main() {
    let sonuc = sum!(1, 2, 3, 4, 5);
    println!("Toplam: {}", sonuc);  // 15
}
```

## 7.2 Operator Overloading via Macros

```rust
macro_rules! implement_ops {
    ($ty:ty) => {
        impl std::ops::Add for $ty {
            type Output = Self;
            
            fn add(self, other: Self) -> Self {
                Self(self.0 + other.0)
            }
        }
        
        impl std::ops::Sub for $ty {
            type Output = Self;
            
            fn sub(self, other: Self) -> Self {
                Self(self.0 - other.0)
            }
        }
        
        impl std::ops::Mul<$ty> for $ty {
            type Output = Self;
            
            fn mul(self, other: Self) -> Self {
                Self(self.0 * other.0)
            }
        }
    };
}

struct Metre(f64);
implement_ops!(Metre);

struct Saniye(f64);
implement_ops!(Saniye);

fn main() {
    let m1 = Metre(10.0);
    let m2 = Metre(20.0);
    let m3 = m1 + m2;
    println!("Toplam: {} m", m3.0);
}
```

## 7.3 DSL (Domain-Specific Language) Oluşturma

```rust
// Basit bir HTML DSL
macro_rules! html {
    // Boş etiket
    (< $tag:ident />) => {
        format!("<{} />", stringify!($tag))
    };
    
    // İçerikli etiket
    (< $tag:ident > $($content:tt)*) => {
        format!(
            "<{}>{}</{}>",
            stringify!($tag),
            html!($($content)*),
            stringify!($tag)
        )
    };
    
    // Metin içeriği
    ($text:expr) => {
        $text.to_string()
    };
    
    // Birden fazla çocuk
    ($($child:tt)*) => {
        {
            let mut result = String::new();
            $(
                result.push_str(&html!($child));
            )*
            result
        }
    };
}

fn main() {
    let sayfa = html!(
        <html>
            <head>
                <title>Test</title>
            </head>
            <body>
                <h1>Merhaba</h1>
                <p>Dünya</p>
            </body>
        </html>
    );
    
    println!("{}", sayfa);
}
```

## 7.4 Conditional Compilation with Macros

```rust
macro_rules! log {
    // Release modda hiçbir şey yapma
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            println!($($arg)*);
        }
    };
}

fn main() {
    log!("Bu sadece debug modda görünür");
    
    // Release build'de bu satır tamamen kaldırılır
    log!("Performans kritik kodda kullanılabilir");
}
```

---

# 📚 BÖLÜM 8: Embedded Sistemlerde Macro Kullanımı (RP2354B) 🎯

## 8.1 no_std Uyumlu Makrolar

```rust
#![no_std]

// no_std uyumlu log makrosu
macro_rules! debug_log {
    ($($arg:tt)*) => {
        #[cfg(feature = "debug_uart")]
        {
            // defmt veya UART üzerinden log
            defmt::debug!($($arg)*);
        }
    };
}

macro_rules! error_log {
    ($($arg:tt)*) => {
        // Hata logları her zaman aktif
        defmt::error!($($arg)*);
    };
}

fn motor_kontrol() {
    debug_log!("Motor kontrol başlatıldı");
    
    if hata_var() {
        error_log!("Motor hatası tespit edildi!");
    }
}
```

## 8.2 Register Access Macros

```rust
// RP2354B register erişimi için makro
macro_rules! register {
    ($name:ident, $offset:expr) => {
        #[allow(non_snake_case)]
        pub mod $name {
            pub const OFFSET: u32 = $offset;
            
            #[inline(always)]
            pub unsafe fn read() -> u32 {
                core::ptr::read_volatile((0x4000_0000 + OFFSET) as *const u32)
            }
            
            #[inline(always)]
            pub unsafe fn write(value: u32) {
                core::ptr::write_volatile((0x4000_0000 + OFFSET) as *mut u32, value);
            }
            
            #[inline(always)]
            pub unsafe fn modify<F>(f: F)
            where
                F: FnOnce(&mut u32),
            {
                let mut value = read();
                f(&mut value);
                write(value);
            }
        }
    };
}

// Kullanım
register!(GPIO_OUT, 0x0004);
register!(GPIO_IN, 0x0008);
register!(GPIO_OE, 0x000C);

fn main() {
    unsafe {
        // GPIO pin 25'i yüksek yap
        GPIO_OUT::modify(|val| *val |= 1 << 25);
    }
}
```

## 8.3 Step Motor Projesi İçin Makrolar

```rust
// Motor komutları için DSL
macro_rules! motor_komut {
    (hiz $hiz:expr) => {
        MotorKomutu::HizAyarla($hiz)
    };
    (pozisyon $poz:expr) => {
        MotorKomutu::PozisyonAyarla($poz)
    };
    (hareket $hedef:expr, $hiz:expr) => {
        MotorKomutu::Hareket {
            hedef: $hedef,
            hiz: $hiz,
        }
    };
    (durdur) => {
        MotorKomutu::Durdur
    };
    (acil_durdur) => {
        MotorKomutu::AcilDurdur
    };
}

enum MotorKomutu {
    HizAyarla(u16),
    PozisyonAyarla(i32),
    Hareket { hedef: i32, hiz: u16 },
    Durdur,
    AcilDurdur,
}

fn main() {
    let komut1 = motor_komut!(hiz 1500);
    let komut2 = motor_komut!(hareket 1000, 500);
    let komut3 = motor_komut!(durdur);
}
```

## 8.4 Compile-Time Validation

```rust
// Compile-time motor parametre validasyonu
macro_rules! validate_motor_params {
    ($hiz:expr, $ivme:expr) => {
        const _: () = {
            assert!($hiz > 0, "Hız 0'dan büyük olmalı");
            assert!($hiz <= 3000, "Hız 3000 RPM'yi aşamaz");
            assert!($ivme > 0, "İvme 0'dan büyük olmalı");
            assert!($ivme <= 500, "İvme 500'ü aşamaz");
        };
    };
}

fn main() {
    // Bunlar compile-time'da doğrulanır!
    validate_motor_params!(1500, 100);  // ✅ Geçerli
    // validate_motor_params!(4000, 100);  // ❌ Compile error!
}
```

## 8.5 Interrupt Handler Macros

```rust
// Interrupt handler oluşturmak için makro
macro_rules! interrupt_handler {
    ($name:ident, $priority:expr, $body:block) => {
        #[interrupt]
        fn $name() {
            // Öncelik bazlı kritik bölüm
            critical_section::with(|_| {
                $body
            });
        }
    };
}

// Kullanım
interrupt_handler!(TIMER_IRQ, 3, {
    // Timer interrupt kodu
    led.toggle();
    sayac += 1;
});

interrupt_handler!(UART_IRQ, 2, {
    // UART interrupt kodu
    let data = uart.read();
    buffer.push(data);
});
```

---

# 📚 BÖLÜM 9: Macro Best Practices ⭐

## 9.1 ✅ İyi Pratikler

### 9.1.1 Makro İsimlendirmesi

```rust
// ✅ İyi: snake_case, açıklayıcı isim
macro_rules! create_motor_controller { }
macro_rules! validate_config! { }

// ❌ Kötü: Belirsiz isimler
macro_rules! m1 { }
macro_rules! process { }
```

### 9.1.2 Hygiene (Hijyen)

```rust
// ✅ İyi: Benzersiz değişken isimleri kullan
macro_rules! good_macro {
    ($x:expr) => {
        let __temp_result = $x;  // __ ile başlayan benzersiz isim
        println!("{}", __temp_result);
    };
}

// ❌ Kötü: Çakışabilecek isimler
macro_rules! bad_macro {
    ($x:expr) => {
        let result = $x;  // Kullanıcının 'result' değişkeniyle çakışabilir!
        println!("{}", result);
    };
}
```

### 9.1.3 Trailing Comma Desteği

```rust
// ✅ İyi: Trailing comma kabul et
macro_rules! my_vec {
    ( $( $x:expr ),* $(,)? ) => {
        vec![ $( $x ),* ]
    };
}

// Kullanım
let v1 = my_vec![1, 2, 3];      // ✅
let v2 = my_vec![1, 2, 3,];     // ✅ (trailing comma)
let v3 = my_vec![];              // ✅
```

### 9.1.4 Error Messages

```rust
// ✅ İyi: Açıklayıcı hata mesajları
macro_rules! validate {
    ($x:expr) => {
        if $x <= 0 {
            compile_error!("Değer pozitif olmalı");
        }
    };
}

// ❌ Kötü: Belirsiz hata mesajları
macro_rules! bad_validate {
    ($x:expr) => {
        if $x <= 0 {
            compile_error!("Hata");
        }
    };
}
```

## 9.2 ❌ Anti-Patterns

### 9.2.1 Çok Karmaşık Makrolar

```rust
// ❌ Kötü: Çok karmaşık, okunması zor
macro_rules! ultra_karmaşık {
    ($($t:tt)*) => {
        // 100+ satırlık kod
    };
}

// ✅ İyi: Basit ve odaklı
macro_rules! simple_macro {
    ($x:expr) => {
        // 5-10 satırlık kod
    };
}
```

### 9.2.2 Side Effects

```rust
// ❌ Kötü: Beklenmedik yan etkiler
macro_rules! gizli_degisken {
    () => {
        let x = 42;  // Kullanıcının kapsamını kirletir!
    };
}

// ✅ İyi: Açık ve izole
macro_rules! temiz_makro {
    () => {
        {
            let x = 42;  // Kendi kapsamı içinde
            x
        }
    };
}
```

---

# 📚 BÖLÜM 10: Macro Debugging

## 10.1 `cargo expand` Kullanımı

Makroların genişletilmiş halini görmek için:

```bash
# cargo-expand kur
cargo install cargo-expand

# Makroları genişlet
cargo expand

# Belirli bir modülü genişlet
cargo expand motor_kontrol
```

**Örnek çıktı**:
```rust
// Orijinal kod
#[derive(Debug)]
struct Motor {
    hiz: u16,
}

// Genişletilmiş kod
struct Motor {
    hiz: u16,
}
impl ::core::fmt::Debug for Motor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            Motor { hiz: __self_0_0 } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "Motor");
                ::core::fmt::DebugStruct::field(debug_trait_builder, "hiz", &&(*__self_0_0));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
```

## 10.2 `macro_rules!` Debug

```rust
macro_rules! trace {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            eprintln!("[TRACE] {}:{}: {}", file!(), line!(), format!($($arg)*));
        }
    };
}

fn main() {
    trace!("Program başladı");
    trace!("Değer: {}", 42);
}
```

## 10.3 Compile Error ile Debug

```rust
macro_rules! type_of {
    ($x:expr) => {
        compile_error!(concat!("Tür: ", stringify!($x)));
    };
}

fn main() {
    let x = 42;
    type_of!(x);  // Compile error: "Tür: x"
}
```

---

# 🎯 ÖZET: Meta-programming Kontrol Listesi

| Özellik | C# | Rust |
|---|---|---|
| Declarative macros | T4 Templates | `macro_rules!` |
| Procedural macros | Source Generators | Proc-macro crate'leri |
| Derive macros | ❌ | ✅ `#[derive(...)]` |
| Attribute macros | ❌ | ✅ `#[attribute]` |
| Function-like macros | ❌ | ✅ `makro!()` |
| Runtime overhead | ⚠️ Var | ✅ Sıfır |
| Type safety | ⚠️ Runtime | ✅ Compile-time |
| IDE desteği | ✅ Mükemmel | ✅ İyi |
| Debugging | ✅ İyi | ⚠️ `cargo expand` |
| Embedded desteği | ❌ | ✅ `no_std` uyumlu |
| DSL oluşturma | ⚠️ Zor | ✅ Kolay |
| Code generation | Source Generators | Proc-macro + build.rs |

---

# 🚀 Son Tavsiyeler

## 1. Declarative Makrolar için

- Basit kod üretimi için `macro_rules!` kullanın
- Pattern matching'i iyi öğrenin
- Trailing comma desteği ekleyin
- Hygiene kurallarına uyun

## 2. Procedural Makrolar için

- `syn`, `quote`, `proc-macro2` üçlüsünü öğrenin
- Ayrı bir proc-macro crate oluşturun
- Hata mesajlarını açıklayıcı yapın
- `cargo expand` ile debug yapın

## 3. Popüler Crate'leri Kullanın

- `serde`: Serialization için
- `thiserror`: Error handling için
- `clap`: CLI parsing için
- `strum`: Enum utilities için
- `derive_more`: Ek trait'ler için

## 4. Embedded Sistemler (RP2354B) için

```rust
// Compile-time validation
macro_rules! validate_motor {
    ($hiz:expr) => {
        const _: () = assert!($hiz <= 3000);
    };
}

// Register access
macro_rules! register {
    ($name:ident, $offset:expr) => {
        // Register modülü oluştur
    };
}

// Conditional logging
macro_rules! debug_log {
    ($($arg:tt)*) => {
        #[cfg(feature = "debug_uart")]
        defmt::debug!($($arg)*);
    };
}

// DSL for motor commands
macro_rules! motor_komut {
    (hiz $h:expr) => { MotorKomutu::Hiz($h) };
    // ...
}
```

## 5. Step Motor Projeniz İçin Önerilen Makrolar

```rust
// 1. Compile-time parametre validasyonu
validate_motor_params!(1500, 100);

// 2. Register erişimi
register!(GPIO_OUT, 0x0004);

// 3. Conditional logging
debug_log!("Motor başlatıldı");

// 4. Motor komut DSL
let cmd = motor_komut!(hareket 1000, 500);

// 5. Interrupt handler
interrupt_handler!(TIMER_IRQ, 3, {
    // Kod
});

// 6. Newtype pattern
new_type!(RPM, u16);
new_type!(Metre, f64);
```

## 6. Best Practices

- Makroları basit tutun
- Açıklayıcı isimler kullanın
- Hata mesajlarını iyileştirin
- `cargo expand` ile debug yapın
- Proc-macro'ları ayrı crate'te tutun
- Embedded'de `no_std` uyumluluğuna dikkat edin
- Compile-time validation kullanın
- DSL'ler için makrolardan yararlanın

> 🦀 **Unutmayın:** Rust'ın makro sistemi, C#'taki Source Generators'dan **çok daha güçlü ve esnektir**. Zero-cost abstraction sayesinde, makroların runtime maliyeti sıfırdır. RP2354B projenizde makrolar, compile-time validation, register erişimi, conditional logging ve DSL oluşturma için vazgeçilmezdir. Makro sistemini iyi anlamak, Rust'ın gücünü tam olarak kullanmanın anahtarıdır! Declarative makrolarla başlayın, ihtiyaç oldukça procedural makrolara geçin. Popüler crate'leri (`serde`, `thiserror`, `clap`) kullanarak zaman kazanın ve kendi makrolarınızı yazarken best practices'e uyun.