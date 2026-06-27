# 🔀 Rust for .NET Developers: Conditional Compilation (Koşullu Derleme)

Bu bölüm, Rust'ta koşullu derlemenin temellerinden ileri düzey tekniklere kadar kapsamlı bir rehberdir. C# dünyasında `#if DEBUG`, `#if NET6_0_OR_GREATER` gibi preprocessor directive'leri ile koşullu kod derlemeye alışkınız. Rust ise **attribute-tabanlı, daha esnek ve güçlü** bir yaklaşım sunar.

> 🎯 **Temel Fark:** C#'ta koşullu derleme **preprocessor directive**'leri (`#if`, `#else`, `#endif`) ile yapılır. Rust'ta ise **`#[cfg(...)]` attribute** sistemi kullanılır - bu, daha güvenli, daha okunabilir ve derleyici tarafından tam olarak anlaşılır.

---

# 📚 BÖLÜM 1: Conditional Compilation Nedir ve Neden Önemlidir?

## 1.1 Tanım

**Conditional Compilation (Koşullu Derleme)**: Belirli koşullara bağlı olarak kodun bazı bölümlerinin derlenip bazı bölümlerinin derlenmemesini sağlayan tekniktir.

## 1.2 Kullanım Alanları

1. **Platform-specific kod**: Windows, Linux, macOS için farklı implementasyonlar
2. **Debug/Release builds**: Geliştirme ve production için farklı kodlar
3. **Feature flags**: Opsiyonel özellikleri açma/kapama
4. **Target architecture**: x86, ARM, WASM için optimize kod
5. **Embedded systems**: `no_std` ortamında farklı davranışlar
6. **Testing**: Test-only kod blokları
7. **Version compatibility**: Farklı Rust versiyonları için uyumluluk

## 1.3 C# vs Rust Yaklaşımı

| Özellik | C# | Rust |
|---|---|---|
| Syntax | `#if DEBUG` | `#[cfg(debug_assertions)]` |
| Scope | Preprocessor (metin bazlı) | Attribute (AST bazlı) |
| Type safety | ❌ Zayıf | ✅ Güçlü |
| IDE desteği | ✅ İyi | ✅ Mükemmel |
| Feature flags | ⚠️ MSBuild ile karmaşık | ✅ Cargo features ile basit |
| Target-specific | ⚠️ `#if WINDOWS` | ✅ `#[cfg(target_os = "windows")]` |
| Nested conditions | ✅ Karmaşık | ✅ Okunabilir |
| Custom conditions | ✅ `#define CUSTOM` | ✅ `--cfg custom` |
| Embedded desteği | ❌ Zayıf | ✅ Mükemmel |

---

# 📚 BÖLÜM 2: C# Conditional Compilation (Kısa Özet)

## 2.1 Preprocessor Directive'leri

```csharp
// Debug build'de derlenir
#if DEBUG
    Console.WriteLine("Debug modu aktif");
    Debug.Assert(deger > 0);
#endif

// Release build'de derlenir
#if RELEASE
    Console.WriteLine("Release modu");
#endif

// .NET versiyonuna göre
#if NET6_0_OR_GREATER
    // .NET 6+ özellikleri
    var yeniOzellik = YeniAPI.Kullan();
#else
    // Eski .NET için fallback
    var eskiOzellik = EskiAPI.Kullan();
#endif

// Platform-specific
#if WINDOWS
    // Windows-specific kod
    Registry.SetValue(...);
#elif LINUX
    // Linux-specific kod
    File.WriteAllText("/etc/config", ...);
#endif

// Custom symbol
#if PREMIUM_FEATURE
    // Premium özellikler
    PremiumAPI.Kullan();
#endif
```

## 2.2 C#'ın Sorunları

- ❌ **Preprocessor metin bazlı**: Derleyici öncesi çalışır, syntax kontrolü yok
- ❌ **Nested conditions karmaşık**: `#if A && (B || C)` okunması zor
- ❌ **IDE desteği sınırlı**: Gri renkle gösterilir ama tam IntelliSense yok
- ❌ **Runtime'da değiştirilemez**: Derleme zamanında sabit
- ❌ **Feature management karmaşık**: MSBuild property'leri ile uğraşmak gerekir

---

# 📚 BÖLÜM 3: Rust `#[cfg(...)]` Attribute Sistemi ⭐

## 3.1 Temel Kullanım

```rust
// Debug build'de derlenir
#[cfg(debug_assertions)]
fn debug_log(mesaj: &str) {
    println!("DEBUG: {}", mesaj);
}

// Release build'de derlenir
#[cfg(not(debug_assertions))]
fn debug_log(_mesaj: &str) {
    // No-op - release'da hiçbir şey yapma
}

fn main() {
    debug_log("Bu mesaj sadece debug'da görünür");
}
```

## 3.2 cfg Koşulları

Rust'ta birçok yerleşik cfg koşulu vardır:

### Debug/Release

```rust
#[cfg(debug_assertions)]
// Debug build (cargo build)

#[cfg(not(debug_assertions))]
// Release build (cargo build --release)
```

### Target OS

```rust
#[cfg(target_os = "windows")]
fn platform_kodu() {
    println!("Windows'ta çalışıyor");
}

#[cfg(target_os = "linux")]
fn platform_kodu() {
    println!("Linux'ta çalışıyor");
}

#[cfg(target_os = "macos")]
fn platform_kodu() {
    println!("macOS'te çalışıyor");
}
```

### Target Architecture

```rust
#[cfg(target_arch = "x86_64")]
fn optimize_for_x86() {
    // x86_64-specific optimizasyonlar
}

#[cfg(target_arch = "aarch64")]
fn optimize_for_arm() {
    // ARM64-specific optimizasyonlar
}

#[cfg(target_arch = "wasm32")]
fn wasm_kodu() {
    // WebAssembly için kod
}
```

### Target Family

```rust
#[cfg(target_family = "unix")]
fn unix_kodu() {
    // Unix-like sistemler (Linux, macOS, BSD)
}

#[cfg(target_family = "windows")]
fn windows_kodu() {
    // Windows
}
```

### Test

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_something() {
        assert_eq!(2 + 2, 4);
    }
}
```

## 3.3 C# vs Rust Karşılaştırması

**C#**:
```csharp
#if DEBUG
    var logLevel = LogLevel.Debug;
#else
    var logLevel = LogLevel.Info;
#endif
```

**Rust**:
```rust
#[cfg(debug_assertions)]
const LOG_LEVEL: LogLevel = LogLevel::Debug;

#[cfg(not(debug_assertions))]
const LOG_LEVEL: LogLevel = LogLevel::Info;
```

> 💡 **Kritik Fark:** Rust'ta `#[cfg]` attribute'ları derleyici tarafından tam olarak anlaşılır. IDE, hangi kodun derleneceğini bilir ve sadece o kod için IntelliSense sunar. C#'ta ise preprocessor metin bazlı çalıştığı için IDE bazen yanılabilir.

---

# 📚 BÖLÜM 4: cfg Koşullarını Birleştirme

## 4.1 `all()` - VE (AND)

Birden fazla koşulun **tümünün** sağlanması gerekir:

```rust
// Debug VE Windows
#[cfg(all(debug_assertions, target_os = "windows"))]
fn debug_windows_kodu() {
    println!("Debug modunda Windows'ta çalışıyor");
}

// Release VE x86_64
#[cfg(all(not(debug_assertions), target_arch = "x86_64"))]
fn optimized_x86_kodu() {
    // Release build'de x86_64 için optimize edilmiş kod
}
```

## 4.2 `any()` - VEYA (OR)

Koşullardan **en az birinin** sağlanması gerekir:

```rust
// Unix-like sistemler (Linux VEYA macOS)
#[cfg(any(target_os = "linux", target_os = "macos"))]
fn unix_kodu() {
    println!("Unix-like sistemde çalışıyor");
}

// Debug VEYA test
#[cfg(any(debug_assertions, test))]
fn gelistirme_kodu() {
    println!("Geliştirme modu");
}
```

## 4.3 `not()` - DEĞİL (NOT)

Koşulun **sağlanmaması** gerekir:

```rust
// Windows DEĞİL
#[cfg(not(target_os = "windows"))]
fn non_windows_kodu() {
    println!("Windows dışında çalışıyor");
}

// Debug DEĞİL (yani release)
#[cfg(not(debug_assertions))]
fn release_kodu() {
    println!("Release modu");
}
```

## 4.4 Karmaşık Koşullar

```rust
// (Debug VE Windows) VEYA (Release VE Linux)
#[cfg(any(
    all(debug_assertions, target_os = "windows"),
    all(not(debug_assertions), target_os = "linux")
))]
fn karmasik_kosul() {
    println!("Karmaşık koşul sağlandı");
}

// ARM VE (Linux VEYA macOS)
#[cfg(all(
    target_arch = "aarch64",
    any(target_os = "linux", target_os = "macos")
))]
fn arm_unix_kodu() {
    println!("ARM mimarisinde Unix-like sistem");
}
```

## 4.5 C# vs Rust Nested Conditions

**C#**:
```csharp
#if (DEBUG && WINDOWS) || (RELEASE && LINUX)
    // Karmaşık koşul
#endif
```

**Rust**:
```rust
#[cfg(any(
    all(debug_assertions, target_os = "windows"),
    all(not(debug_assertions), target_os = "linux")
))]
// Karmaşık koşul
```

> 💡 **Okunabilirlik:** Rust'ın syntax'ı daha okunabilir ve derleyici tarafından tam olarak doğrulanır. C#'ta parantezler karışabilir, Rust'ta ise `all()` ve `any()` fonksiyonları açıkça niyeti belirtir.

---

# 📚 BÖLÜM 5: `cfg_if` Crate - Daha Temiz Syntax

Karmaşık koşullar için `cfg_if` crate'i daha okunabilir bir syntax sunar.

## 5.1 Kurulum

**Cargo.toml**:
```toml
[dependencies]
cfg-if = "1.0"
```

## 5.2 Temel Kullanım

```rust
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_os = "windows")] {
        fn platform_ozel() {
            println!("Windows");
        }
    } else if #[cfg(target_os = "linux")] {
        fn platform_ozel() {
            println!("Linux");
        }
    } else if #[cfg(target_os = "macos")] {
        fn platform_ozel() {
            println!("macOS");
        }
    } else {
        fn platform_ozel() {
            println!("Bilinmeyen platform");
        }
    }
}
```

## 5.3 C# vs cfg_if

**C#**:
```csharp
#if WINDOWS
    void PlatformOzel() => Console.WriteLine("Windows");
#elif LINUX
    void PlatformOzel() => Console.WriteLine("Linux");
#elif MACOS
    void PlatformOzel() => Console.WriteLine("macOS");
#else
    void PlatformOzel() => Console.WriteLine("Bilinmeyen");
#endif
```

**Rust (cfg_if)**:
```rust
cfg_if! {
    if #[cfg(target_os = "windows")] {
        fn platform_ozel() { println!("Windows"); }
    } else if #[cfg(target_os = "linux")] {
        fn platform_ozel() { println!("Linux"); }
    } else if #[cfg(target_os = "macos")] {
        fn platform_ozel() { println!("macOS"); }
    } else {
        fn platform_ozel() { println!("Bilinmeyen"); }
    }
}
```

> 💡 **Avantaj:** `cfg_if` ile if-else-if zinciri gibi yazabilirsiniz. Bu, özellikle çok fazla platform desteği varsa çok okunaklıdır.

---

# 📚 BÖLÜM 6: Feature Flags (Özellik Bayrakları) ⭐⭐

Feature flags, Rust'ın **en güçlü** koşullu derleme mekanizmasıdır. Cargo.toml'da tanımlanır ve kullanıcılar hangi özellikleri etkinleştirmek istediklerini seçer.

## 6.1 Cargo.toml'da Feature Tanımlama

```toml
[package]
name = "motor_kontrol"
version = "0.1.0"
edition = "2021"

[features]
# Varsayılan özellikler
default = ["logging", "basic_motor"]

# Temel özellikler
logging = ["dep:log", "dep:env_logger"]
basic_motor = []

# Gelişmiş özellikler
advanced_motor = ["basic_motor"]  # basic_motor'a bağımlı
profiling = []
simulation = []

# Premium özellikler
premium = ["advanced_motor", "profiling"]

# Tüm özellikler
full = ["premium", "simulation"]
```

## 6.2 Kodda Feature Kullanımı

```rust
// Sadece "logging" feature'ı aktifse derlenir
#[cfg(feature = "logging")]
fn log_motor_durumu(durum: &MotorDurumu) {
    log::info!("Motor durumu: {:?}", durum);
}

#[cfg(not(feature = "logging"))]
fn log_motor_durumu(_durum: &MotorDurumu) {
    // No-op
}

// "advanced_motor" feature'ı aktifse derlenir
#[cfg(feature = "advanced_motor")]
mod advanced {
    pub fn hassas_kontrol() {
        // Gelişmiş motor kontrol algoritmaları
    }
}

// "simulation" feature'ı aktifse derlenir
#[cfg(feature = "simulation")]
fn simulate_motor() {
    println!("Simülasyon modu");
}

#[cfg(not(feature = "simulation"))]
fn simulate_motor() {
    panic!("Simülasyon feature'ı aktif değil!");
}
```

## 6.3 Feature ile Dependency Yönetimi

```toml
[dependencies]
# Her zaman gerekli
serde = "1.0"

# Opsiyonel dependency'ler
log = { version = "0.4", optional = true }
env_logger = { version = "0.10", optional = true }
tokio = { version = "1.0", optional = true, features = ["full"] }

[features]
logging = ["dep:log", "dep:env_logger"]
async_support = ["dep:tokio"]
```

```rust
// Sadece "logging" feature'ı aktifse log crate'i derlenir
#[cfg(feature = "logging")]
use log::{info, debug};

#[cfg(feature = "logging")]
fn log_info(mesaj: &str) {
    info!("{}", mesaj);
}

#[cfg(not(feature = "logging"))]
fn log_info(_mesaj: &str) {
    // No-op
}
```

## 6.4 Feature'ları Etkinleştirme

```bash
# Varsayılan feature'larla derle
cargo build

# Belirli feature'ları etkinleştir
cargo build --features "logging,advanced_motor"

# Varsayılan feature'ları devre dışı bırak
cargo build --no-default-features

# Tüm feature'ları etkinleştir
cargo build --all-features

# Belirli feature'larla test et
cargo test --features "simulation"
```

## 6.5 C# vs Rust Feature Management

**C#** (MSBuild):
```xml
<Project>
  <PropertyGroup>
    <DefineConstants>$(DefineConstants);LOGGING</DefineConstants>
  </PropertyGroup>
  
  <ItemGroup Condition="'$(LOGGING)' == 'true'">
    <PackageReference Include="Serilog" Version="3.0.0" />
  </ItemGroup>
</Project>
```

```csharp
#if LOGGING
    // Serilog kullan
#endif
```

**Rust**:
```toml
[features]
logging = ["dep:log"]

[dependencies]
log = { version = "0.4", optional = true }
```

```rust
#[cfg(feature = "logging")]
use log::info;
```

> 💡 **Kritik Fark:** Rust'ta feature'lar **Cargo.toml'da merkezi** olarak tanımlanır ve kullanıcılar `--features` flag'i ile kolayca etkinleştirir. C#'ta ise MSBuild property'leri ile uğraşmak gerekir, bu daha karmaşıktır.

---

# 📚 BÖLÜM 7: Custom cfg Değerleri

Kendi cfg değerlerinizi tanımlayabilirsiniz.

## 7.1 Derleme Zamanında Custom cfg

```bash
# Custom cfg değeri ile derle
cargo build --cfg my_custom_flag

# Değer ile
cargo build --cfg 'my_custom_flag="value1"'
```

```rust
#[cfg(my_custom_flag)]
fn custom_kod() {
    println!("Custom flag aktif");
}

#[cfg(my_custom_flag = "value1")]
fn value1_kodu() {
    println!("Value1 aktif");
}

#[cfg(my_custom_flag = "value2")]
fn value2_kodu() {
    println!("Value2 aktif");
}
```

## 7.2 .cargo/config.toml ile Varsayılan cfg

```toml
# .cargo/config.toml
[build]
rustflags = ["--cfg", "my_custom_flag"]

[target.'cfg(target_arch = "arm")']
rustflags = ["--cfg", "embedded_mode"]
```

## 7.3 Build Script ile Dinamik cfg

```rust
// build.rs
fn main() {
    // Ortam değişkenine göre cfg ayarla
    if std::env::var("ENABLE_EXPERIMENTAL").is_ok() {
        println!("cargo:rustc-cfg=experimental");
    }
    
    // Git commit hash'ine göre cfg
    let output = std::process::Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .expect("Git komutu başarısız");
    
    let commit_hash = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-cfg=commit_hash=\"{}\"", commit_hash.trim());
}
```

```rust
#[cfg(experimental)]
fn deneysel_ozellik() {
    println!("Deneysel özellik aktif");
}

#[cfg(commit_hash = "abc1234")]
fn specific_commit_kodu() {
    println!("Belirli commit için özel kod");
}
```

## 7.4 Pratik Örnek: Development Stages

```rust
// .cargo/config.toml
[build]
rustflags = ["--cfg", "dev_stage=\"development\""]
// veya "staging", "production"

#[cfg(dev_stage = "development")]
const API_URL: &str = "http://localhost:8080";

#[cfg(dev_stage = "staging")]
const API_URL: &str = "https://staging-api.example.com";

#[cfg(dev_stage = "production")]
const API_URL: &str = "https://api.example.com";

#[cfg(not(any(
    dev_stage = "development",
    dev_stage = "staging",
    dev_stage = "production"
)))]
compile_error!("dev_stage belirtilmeli: development, staging, veya production");
```

---

# 📚 BÖLÜM 8: `cfg_attr` - Koşullu Attributes

`cfg_attr`, bir attribute'u yalnızca belirli koşullar sağlandığında uygular.

## 8.1 Temel Kullanım

```rust
// Sadece debug_assertions varsa derive(Debug) uygula
#[cfg_attr(debug_assertions, derive(Debug))]
struct MotorDurumu {
    pozisyon: i32,
    hiz: u16,
}

// Sadece test modunda derive(Default) uygula
#[cfg_attr(test, derive(Default))]
struct TestYapisi {
    deger: i32,
}
```

## 8.2 Birden Fazla Attribute

```rust
// Birden fazla attribute koşullu olarak uygula
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(feature = "logging", derive(Debug))]
struct Yapilandirma {
    port: u16,
    host: String,
}
```

## 8.3 Pratik Örnekler

### Conditional Derive

```rust
#[cfg_attr(debug_assertions, derive(Debug, Clone))]
#[cfg_attr(not(debug_assertions), derive(Clone))]
struct PerformansKritikYapi {
    veri: [u8; 1024],
}
```

### Conditional Lint

```rust
#![cfg_attr(debug_assertions, allow(dead_code))]
#![cfg_attr(not(debug_assertions), deny(warnings))]

// Debug'da kullanılmayan kod uyarısı vermez
// Release'da tüm uyarılar hata olur
```

### Conditional Inline

```rust
#[cfg_attr(not(debug_assertions), inline(always))]
fn kritik_fonksiyon() {
    // Release'da her zaman inline et
    // Debug'da normal derle (debugging kolaylığı için)
}
```

## 8.4 C# vs cfg_attr

C#'ta attribute'ları koşullu olarak uygulamak zordur:

```csharp
// C# - Zor
#if DEBUG
[Serializable]
#endif
public class MyClass { }
```

```rust
// Rust - Kolay
#[cfg_attr(debug_assertions, derive(Serialize))]
struct Yapim {}
```

---

# 📚 BÖLÜM 9: Target-Specific Dependencies

Farklı platformlar için farklı dependency'ler kullanabilirsiniz.

## 9.1 Target-Specific Dependencies

```toml
[dependencies]
serde = "1.0"

[target.'cfg(target_os = "windows")'.dependencies]
winapi = "0.3"
registry = "1.2"

[target.'cfg(target_os = "linux")'.dependencies]
nix = "0.26"
procfs = "0.15"

[target.'cfg(target_os = "macos")'.dependencies]
core-foundation = "0.9"

[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "0.2"
web-sys = "0.3"
```

## 9.2 Kodda Target-Specific Kullanım

```rust
#[cfg(target_os = "windows")]
fn platform_bilgisi() -> String {
    format!("Windows {}", winapi::get_version())
}

#[cfg(target_os = "linux")]
fn platform_bilgisi() -> String {
    let kernel = procfs::kernel_version().unwrap();
    format!("Linux {}", kernel)
}

#[cfg(target_os = "macos")]
fn platform_bilgisi() -> String {
    format!("macOS {}", core_foundation::get_version())
}
```

## 9.3 Embedded Target Dependencies

```toml
# RP2354B (ARM Cortex-M33) için
[target.'cfg(target_arch = "arm")'.dependencies]
cortex-m = "0.7"
cortex-m-rt = "0.7"
rp235x-hal = { version = "0.1", features = ["rt"] }

# Host (x86_64) için test
[target.'cfg(target_arch = "x86_64")'.dependencies]
tokio = { version = "1.0", features = ["full"] }
```

---

# 📚 BÖLÜM 10: Embedded Sistemler için Conditional Compilation 🎯

**Bu bölüm, RP2354B step motor projeniz için KRİTİK ÖNEM taşır!**

## 10.1 no_std ve std Ayrımı

```rust
// lib.rs
#![cfg_attr(not(test), no_std)]

#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Test modunda std kullan
#[cfg(test)]
use std::println;

#[cfg(test)]
fn test_log(mesaj: &str) {
    println!("{}", mesaj);
}

#[cfg(not(test))]
fn test_log(_mesaj: &str) {
    // Embedded'de log yok
}
```

## 10.2 Target Architecture Specific Kod

```rust
// RP2354B (ARM Cortex-M33) için optimize
#[cfg(target_arch = "arm")]
mod arm_specific {
    use cortex_m::asm;
    
    pub fn delay_cycles(cycles: u32) {
        asm::delay(cycles);
    }
    
    pub fn nop() {
        asm::nop();
    }
}

// Host (x86_64) için simülasyon
#[cfg(target_arch = "x86_64")]
mod x86_specific {
    use std::thread;
    use std::time::Duration;
    
    pub fn delay_cycles(cycles: u32) {
        // Simülasyon: cycles'ı microseconds'a çevir
        thread::sleep(Duration::from_micros(cycles as u64 / 150));
    }
    
    pub fn nop() {
        // No-op
    }
}

// Ortak API
pub fn adim_at() {
    #[cfg(target_arch = "arm")]
    arm_specific::delay_cycles(1000);
    
    #[cfg(target_arch = "x86_64")]
    x86_specific::delay_cycles(1000);
}
```

## 10.3 Feature Flags ile Embedded Optimizasyon

```toml
[features]
default = ["debug_uart"]

# Debug özellikleri
debug_uart = ["dep:defmt", "dep:defmt-rtt"]
simulation = []

# Performance özellikleri
fast_math = []
dma_transfer = []

# Tüm debug özellikleri
debug = ["debug_uart", "simulation"]
```

```rust
// UART debug logging
#[cfg(feature = "debug_uart")]
use defmt::{info, debug, error};

#[cfg(not(feature = "debug_uart"))]
macro_rules! info { ($($arg:tt)*) => {}; }
#[cfg(not(feature = "debug_uart"))]
macro_rules! debug { ($($arg:tt)*) => {}; }
#[cfg(not(feature = "debug_uart"))]
macro_rules! error { ($($arg:tt)*) => {}; }

// Simülasyon modu
#[cfg(feature = "simulation")]
fn gercek_donanim_kodu() {
    // Simülasyon - gerçek donanım yok
    info!("Simülasyon modu aktif");
}

#[cfg(not(feature = "simulation"))]
fn gercek_donanim_kodu() {
    // Gerçek donanım kodu
    unsafe {
        // GPIO register yazma
    }
}

// Fast math optimizasyonu
#[cfg(feature = "fast_math")]
fn hesapla_adim_sayisi(mesafe: f64) -> u32 {
    // SIMD veya hardware float kullan
    (mesafe * 200.0 / 5.0) as u32
}

#[cfg(not(feature = "fast_math"))]
fn hesapla_adim_sayisi(mesafe: f64) -> u32 {
    // Standart float hesaplama
    (mesafe * 200.0 / 5.0) as u32
}
```

## 10.4 Memory Layout Conditional

```rust
// RP2354B memory layout
#[cfg(target_arch = "arm")]
mod memory {
    pub const FLASH_START: u32 = 0x10000000;
    pub const FLASH_SIZE: u32 = 2 * 1024 * 1024;  // 2MB
    pub const RAM_START: u32 = 0x20000000;
    pub const RAM_SIZE: u32 = 512 * 1024;  // 512KB
}

// Host simulation
#[cfg(target_arch = "x86_64")]
mod memory {
    pub const FLASH_START: u32 = 0;
    pub const FLASH_SIZE: u32 = 2 * 1024 * 1024;
    pub const RAM_START: u32 = 0;
    pub const RAM_SIZE: u32 = 512 * 1024;
}
```

## 10.5 Pratik Örnek: Step Motor Controller

```rust
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(feature = "debug_uart")]
use defmt_rtt as _;

#[cfg(not(test))]
use panic_probe as _;

use rp235x_hal as hal;

// Feature-based motor control
#[cfg(feature = "advanced_motor")]
mod motor_control {
    pub fn hassas_hareket(hedef: i32) {
        // PID kontrol ile hassas hareket
        #[cfg(feature = "debug_uart")]
        defmt::info!("Hassas hareket: hedef={}", hedef);
    }
}

#[cfg(not(feature = "advanced_motor"))]
mod motor_control {
    pub fn basit_hareket(hedef: i32) {
        // Basit open-loop kontrol
        #[cfg(feature = "debug_uart")]
        defmt::debug!("Basit hareket: hedef={}", hedef);
    }
}

#[cfg(test)]
mod motor_control {
    pub fn test_hareket(hedef: i32) -> bool {
        // Test modunda simülasyon
        println!("Test hareket: hedef={}", hedef);
        true
    }
}

#[cfg(not(test))]
#[cortex_m_rt::entry]
fn main() -> ! {
    let mut pac = hal::pac::Peripherals::take().unwrap();
    
    #[cfg(feature = "debug_uart")]
    defmt::info!("Motor kontrolcüsü başlatılıyor");
    
    #[cfg(feature = "advanced_motor")]
    motor_control::hassas_hareket(1000);
    
    #[cfg(not(feature = "advanced_motor"))]
    motor_control::basit_hareket(1000);
    
    loop {
        cortex_m::asm::wfi();
    }
}

#[cfg(test)]
fn main() {
    println!("Test modu çalışıyor");
    assert!(motor_control::test_hareket(1000));
}
```

## 10.6 Build Script ile Hardware Detection

```rust
// build.rs
fn main() {
    // Ortam değişkeninden hardware versiyonunu al
    if let Ok(hw_version) = std::env::var("HARDWARE_VERSION") {
        println!("cargo:rustc-cfg=hw_version=\"{}\"", hw_version);
    }
    
    // Default hardware version
    #[cfg(not(hw_version))]
    println!("cargo:rustc-cfg=hw_version=\"v1\"");
}
```

```rust
// Hardware version-specific kod
#[cfg(hw_version = "v1")]
const MOTOR_CURRENT_LIMIT: u16 = 1000;  // 1A

#[cfg(hw_version = "v2")]
const MOTOR_CURRENT_LIMIT: u16 = 2000;  // 2A

#[cfg(hw_version = "v3")]
const MOTOR_CURRENT_LIMIT: u16 = 3000;  // 3A
```

---

# 📚 BÖLÜM 11: Pratik Örnekler

## 11.1 Cross-Platform File Paths

```rust
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_os = "windows")] {
        const CONFIG_PATH: &str = "C:\\ProgramData\\MotorKontrol\\config.json";
        const LOG_PATH: &str = "C:\\ProgramData\\MotorKontrol\\logs";
    } else if #[cfg(target_os = "linux")] {
        const CONFIG_PATH: &str = "/etc/motor-kontrol/config.json";
        const LOG_PATH: &str = "/var/log/motor-kontrol";
    } else if #[cfg(target_os = "macos")] {
        const CONFIG_PATH: &str = "/Library/Application Support/MotorKontrol/config.json";
        const LOG_PATH: &str = "/Library/Logs/MotorKontrol";
    } else {
        const CONFIG_PATH: &str = "./config.json";
        const LOG_PATH: &str = "./logs";
    }
}

fn config_yukle() -> Result<Config, Error> {
    let icerik = std::fs::read_to_string(CONFIG_PATH)?;
    serde_json::from_str(&icerik).map_err(Into::into)
}
```

## 11.2 Conditional Compilation ile Testing

```rust
pub struct VeritabaniBaglantisi {
    #[cfg(test)]
    mock_data: Option<Vec<Kayit>>,
    
    #[cfg(not(test))]
    baglanti: PgConnection,
}

impl VeritabaniBaglantisi {
    #[cfg(test)]
    pub fn new_mock(data: Vec<Kayit>) -> Self {
        Self {
            mock_data: Some(data),
        }
    }
    
    #[cfg(not(test))]
    pub fn new(baglanti_string: &str) -> Result<Self, Error> {
        let baglanti = PgConnection::establish(baglanti_string)?;
        Ok(Self { baglanti })
    }
    
    pub fn kayitlari_getir(&self) -> Vec<Kayit> {
        #[cfg(test)]
        {
            self.mock_data.clone().unwrap_or_default()
        }
        
        #[cfg(not(test))]
        {
            use diesel::prelude::*;
            kayitlar::table.load::<Kayit>(&self.baglanti).unwrap()
        }
    }
}
```

## 11.3 Performance Profiling

```rust
#[cfg(feature = "profiling")]
use std::time::Instant;

#[cfg(feature = "profiling")]
macro_rules! profile {
    ($name:expr, $block:block) => {{
        let start = Instant::now();
        let result = $block;
        let elapsed = start.elapsed();
        println!("{}: {:?}", $name, elapsed);
        result
    }};
}

#[cfg(not(feature = "profiling"))]
macro_rules! profile {
    ($name:expr, $block:block) => {
        $block
    };
}

fn karmasik_hesaplama() -> i32 {
    profile!("karmasik_hesaplama", {
        let mut toplam = 0;
        for i in 0..1_000_000 {
            toplam += i;
        }
        toplam
    })
}
```

## 11.4 API Version Compatibility

```rust
// Farklı API versiyonları için conditional compilation
#[cfg(feature = "api_v1")]
mod api {
    pub fn veri_getir() -> String {
        // Eski API
        "v1 data".to_string()
    }
}

#[cfg(feature = "api_v2")]
mod api {
    pub fn veri_getir() -> String {
        // Yeni API
        "v2 data".to_string()
    }
}

#[cfg(not(any(feature = "api_v1", feature = "api_v2")))]
compile_error!("API versiyonu belirtilmeli: api_v1 veya api_v2");
```

---

# 📚 BÖLÜM 12: Best Practices

## 12.1 ✅ İyi Pratikler

1. **Feature Flags Kullanın:**
```rust
// ✅ İyi: Feature-based
#[cfg(feature = "logging")]
fn log_info(mesaj: &str) {
    log::info!("{}", mesaj);
}

// ❌ Kötü: Hard-coded debug
#[cfg(debug_assertions)]
fn log_info(mesaj: &str) {
    println!("{}", mesaj);
}
```

2. **cfg_if ile Okunabilirlik:**
```rust
// ✅ İyi: cfg_if ile
cfg_if! {
    if #[cfg(target_os = "windows")] {
        // Windows kodu
    } else if #[cfg(target_os = "linux")] {
        // Linux kodu
    } else {
        // Fallback
    }
}

// ❌ Kötü: İç içe cfg
#[cfg(target_os = "windows")]
fn platform_kodu() { }

#[cfg(not(target_os = "windows"))]
#[cfg(target_os = "linux")]
fn platform_kodu() { }

#[cfg(not(target_os = "windows"))]
#[cfg(not(target_os = "linux"))]
fn platform_kodu() { }
```

3. **Compile Error ile Zorunluluk:**
```rust
// ✅ İyi: Kullanıcıyı uyar
#[cfg(not(any(
    target_arch = "x86_64",
    target_arch = "aarch64"
)))]
compile_error!("Bu crate sadece x86_64 ve aarch64 mimarilerini destekler");
```

4. **Test-Only Kod:**
```rust
// ✅ İyi: Test-only helper
#[cfg(test)]
mod test_helpers {
    pub fn mock_veritabani() -> MockDB {
        MockDB::new()
    }
}

#[cfg(test)]
mod tests {
    use super::test_helpers::*;
    
    #[test]
    fn test_something() {
        let db = mock_veritabani();
        // ...
    }
}
```

## 12.2 ❌ Anti-Patterns

```rust
// ❌ Çok fazla cfg ile karmaşık kod
#[cfg(feature = "a")]
#[cfg(feature = "b")]
#[cfg(target_os = "linux")]
fn karmasik() {
    // ...
}

// ✅ cfg_if ile basitleştir
cfg_if! {
    if #[cfg(all(feature = "a", feature = "b", target_os = "linux"))] {
        fn karmasik() { }
    }
}

// ❌ Runtime'da cfg kontrolü
fn runtime_kontrol() {
    #[cfg(feature = "a")]
    println!("Feature A aktif");
    
    #[cfg(not(feature = "a"))]
    println!("Feature A aktif değil");
}

// ✅ Compile-time'da hallet
#[cfg(feature = "a")]
fn runtime_kontrol() {
    println!("Feature A aktif");
}

#[cfg(not(feature = "a"))]
fn runtime_kontrol() {
    println!("Feature A aktif değil");
}

// ❌ Feature flag olmadan conditional
#[cfg(debug_assertions)]
const MAX_CONNECTIONS: usize = 10;

#[cfg(not(debug_assertions))]
const MAX_CONNECTIONS: usize = 1000;

// ✅ Feature flag ile
const MAX_CONNECTIONS: usize = if cfg!(feature = "high_connections") {
    1000
} else {
    10
};
```

---

# 🎯 ÖZET: Conditional Compilation Kontrol Listesi

| Özellik | C# | Rust |
|---|---|---|
| Temel syntax | `#if`, `#else`, `#endif` | `#[cfg(...)]` |
| VE koşulu | `&&` | `all()` |
| VEYA koşulu | `\|\|` | `any()` |
| DEĞİL koşulu | `!` | `not()` |
| Feature flags | MSBuild properties | Cargo features |
| Custom conditions | `#define` | `--cfg` flag |
| Conditional attributes | Zor | `#[cfg_attr(...)]` |
| Target-specific | `#if WINDOWS` | `#[cfg(target_os = "...")]` |
| Readability | Orta | Yüksek (cfg_if) |
| IDE desteği | ✅ İyi | ✅ Mükemmel |
| Compile-time check | ❌ Zayıf | ✅ Güçlü |
| Embedded desteği | ❌ Zayıf | ✅ Mükemmel |

---

# 🚀 Son Tavsiyeler

1. **Feature Flags Kullanın:** Opsiyonel özellikler için Cargo features tanımlayın. Bu, kullanıcıların ihtiyacı olmayan kodu derlememesini sağlar.

2. **cfg_if ile Okunabilirlik:** Karmaşık koşullar için `cfg_if` crate'ini kullanın.

3. **Platform-Specific Kodu Ayrıştırın:** Her platform için ayrı modüller oluşturun, ortak API'yi üstte tutun.

4. **Test-Only Kodu İzole Edin:** `#[cfg(test)]` ile test kodunu production binary'sinden ayırın.

5. **Compile Error ile Zorunluluk:** Geçersiz konfigürasyonları `compile_error!` ile erken yakalayın.

6. **Embedded Sistemler için:**
   - `no_std` ve `std` ayrımını iyi yapın
   - Target architecture-specific optimizasyonlar kullanın
   - Feature flags ile debug/release davranışını kontrol edin
   - Memory layout'u conditional olarak tanımlayın

7. **RP2354B Projeniz İçin:**
   ```toml
   [features]
   default = ["debug_uart"]
   debug_uart = ["dep:defmt"]
   advanced_motor = []
   simulation = []
   fast_math = []
   ```
   
   ```rust
   #[cfg(feature = "debug_uart")]
   use defmt::info;
   
   #[cfg(feature = "advanced_motor")]
   mod hassas_kontrol { }
   
   #[cfg(target_arch = "arm")]
   mod arm_optimize { }
   ```

8. **Build Script ile Dinamik cfg:** Ortam değişkenleri veya Git bilgisi ile dinamik cfg değerleri oluşturun.

9. **cfg_attr ile Conditional Attributes:** Debug'da `derive(Debug)` ekleyin, release'da kaldırın.

10. **Documentation:** Feature flag'leri README'de belgeleyin:
    ```markdown
    ## Features
    
    - `logging`: Loglama desteği (varsayılan: açık)
    - `advanced_motor`: Gelişmiş motor kontrol algoritmaları
    - `simulation`: Simülasyon modu (test için)
    ```

11. **CI/CD Entegrasyonu:**
    ```yaml
    - name: Test with all features
      run: cargo test --all-features
    
    - name: Build for ARM
      run: cargo build --target arm-none-eabi --no-default-features
    
    - name: Build for x86_64
      run: cargo build --target x86_64-unknown-linux-gnu
    ```

12. **Binary Boyutu Optimizasyonu:**
    ```bash
    # Feature'ları devre dışı bırakarak binary boyutunu küçült
    cargo build --release --no-default-features --features "basic_motor"
    
    # Binary boyutunu kontrol et
    cargo size --release
    ```

> 🦀 **Unutmayın:** Rust'ın koşullu derleme sistemi, C#'taki preprocessor directive'lerinden **çok daha güçlü ve güvenlidir**. Derleyici tarafından tam olarak anlaşılır, IDE desteği mükemmeldir ve embedded sistemler için optimize edilmiştir. RP2354B projenizde `no_std`, target-specific kod ve feature flags kullanarak hem geliştirme hem de production için optimize binary'ler üretebilirsiniz. Koşullu derleme, Rust'ın "zero-cost abstraction" felsefesinin mükemmel bir örneğidir - kullanmadığınız kod **gerçekten** derlenmez!