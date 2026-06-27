# 🏗️ Rust for .NET Developers: Project Structure (Proje Yapısı)

Bu bölüm, Rust'ın proje yapısı, modül sistemi, workspace yönetimi ve bağımlılık ekosistemini kapsamlı bir şekilde inceler. C# dünyasında **Solution → Project → Namespace → Class** hiyerarşisine ve **NuGet** paket yönetimine alışkınız. Rust ise **Workspace → Crate → Module** yapısı ve **crates.io** ekosistemi ile benzer ama daha esnek bir yaklaşım sunar.

> 🎯 **Temel Fark:** C#'ta bir Solution birden fazla Project içerir, her Project bir Assembly üretir. Rust'ta bir Workspace birden fazla Crate içerir, her Crate bir library veya binary üretir. Rust'ın modül sistemi ise dosya sistemi ile sıkı bağlıdır ve **visibility (görünürlük)** kuralları C#'tan çok daha katıdır.

---

# 📚 BÖLÜM 1: Temel Kavramlar ve Terminoloji

## 1.1 C# vs Rust Terminoloji Haritası

| C# Kavramı | Rust Karşılığı | Açıklama |
|---|---|---|
| Solution (`.sln`) | **Workspace** | Birden fazla projeyi gruplayan yapı |
| Project (`.csproj`) | **Crate** | Derlenebilen birim (library veya binary) |
| Namespace | **Module** (`mod`) | Kod organizasyon birimi |
| Assembly (`.dll`/`.exe`) | **Crate** (compiled) | Derlenmiş çıktı |
| NuGet Package | **Crate** (published) | Yayınlanmış paket |
| `using` directive | `use` statement | İsim getirme |
| `public`/`internal`/`private` | `pub`/`pub(crate)`/default | Görünürlük belirteçleri |
| `Program.cs` | `main.rs` veya `lib.rs` | Entry point |
| `appsettings.json` | `Cargo.toml` | Konfigürasyon dosyası |
| `.cs` dosyası | `.rs` dosyası | Kaynak kod dosyası |
| MSBuild | **Cargo** | Build sistemi |
| `dotnet build` | `cargo build` | Derleme komutu |
| `dotnet run` | `cargo run` | Çalıştırma komutu |
| `dotnet test` | `cargo test` | Test komutu |
| `dotnet publish` | `cargo build --release` | Yayın derlemesi |

## 1.2 Crate Türleri

Rust'ta iki temel crate türü vardır:

```
┌─────────────────────────────────────────────────────────┐
│ Crate Türleri                                           │
├─────────────────────────────────────────────────────────┤
│ 1. Binary Crate (Uygulama)                              │
│    ├─ Entry point: main() fonksiyonu                    │
│    ├─ Kaynak: src/main.rs                               │
│    ├─ Çıktı: Çalıştırılabilir binary                    │
│    └─ Örnek: Web sunucusu, CLI aracı                    │
├─────────────────────────────────────────────────────────┤
│ 2. Library Crate (Kütüphane)                            │
│    ├─ Entry point: Yok                                  │
│    ├─ Kaynak: src/lib.rs                                │
│    ├─ Çıktı: .rlib (Rust library)                       │
│    └─ Örnek: Yardımcı fonksiyonlar, trait'ler           │
└─────────────────────────────────────────────────────────┘
```

---

# 📚 BÖLÜM 2: Tek Crate Projesi

## 2.1 Binary Crate Oluşturma

```bash
# Yeni binary crate oluştur
cargo new motor_kontrol
cd motor_kontrol
```

**Oluşan yapı**:
```
motor_kontrol/
├── Cargo.toml          # .csproj karşılığı
├── src/
│   └── main.rs         # Entry point (Program.cs karşılığı)
├── .gitignore
└── target/             # Derleme çıktıları (bin/obj karşılığı)
    ├── debug/
    └── release/
```

**Cargo.toml**:
```toml
[package]
name = "motor_kontrol"
version = "0.1.0"
edition = "2021"
authors = ["Ali Yılmaz <ali@example.com>"]
description = "Step motor kontrol uygulaması"

[dependencies]
# NuGet PackageReference karşılığı
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.35", features = ["full"] }
```

**src/main.rs**:
```rust
// C#'taki Program.cs karşılığı
fn main() {
    println!("Motor kontrol sistemi başlatılıyor!");
    
    // Kodunuz burada
}
```

## 2.2 Library Crate Oluşturma

```bash
# Yeni library crate oluştur
cargo new motor_lib --lib
```

**Oluşan yapı**:
```
motor_lib/
├── Cargo.toml
├── src/
│   └── lib.rs          # Library entry point
└── tests/              # Integration testler
    └── integration_test.rs
```

**src/lib.rs**:
```rust
// Library crate - main() fonksiyonu YOK
pub fn topla(a: i32, b: i32) -> i32 {
    a + b
}

pub mod motor;  // Alt modül
pub mod sensor;  // Alt modül

// Testler aynı dosyada olabilir
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_topla() {
        assert_eq!(topla(2, 3), 5);
    }
}
```

## 2.3 C# vs Rust Proje Oluşturma

**C#**:
```bash
# Binary projesi
dotnet new console -n MotorKontrol

# Library projesi
dotnet new classlib -n MotorLib

# Solution oluştur
dotnet new sln -n MotorSolution
dotnet sln add MotorKontrol
dotnet sln add MotorLib
```

**Rust**:
```bash
# Binary crate
cargo new motor_kontrol

# Library crate
cargo new motor_lib --lib

# Workspace oluştur (aşağıda anlatılacak)
```

---

# 📚 BÖLÜM 3: Modül Sistemi (Module System) ⭐⭐

Rust'ın modül sistemi, C#'taki namespace sisteminden **temelde farklıdır**. Dosya yapısı ile sıkı bağlıdır ve visibility kuralları çok daha katıdır.

## 3.1 Temel Modül Yapısı

**C#**:
```csharp
// Her dosyada birden fazla namespace olabilir
namespace Sirket.Motor.Kontrol
{
    public class MotorController { }
    
    public class SensorReader { }
}

// Kullanım
using Sirket.Motor.Kontrol;
var controller = new MotorController();
```

**Rust**:
```rust
// src/main.rs veya src/lib.rs
mod motor;      // motor.rs veya motor/mod.rs dosyasını dahil et
mod sensor;     // sensor.rs veya sensor/mod.rs dosyasını dahil et

fn main() {
    let controller = motor::MotorController::new();
    let reader = sensor::SensorReader::new();
}
```

## 3.2 Modül Hiyerarşisi

```
src/
├── main.rs (veya lib.rs)     # Root modül
├── motor.rs                   # motor modülü
├── motor/                     # motor alt modülleri
│   ├── kontrol.rs            # motor::kontrol
│   ├── hiz_profili.rs        # motor::hiz_profili
│   └── mod.rs                # motor modülünün root'u (opsiyonel)
├── sensor/
│   ├── adc.rs                # sensor::adc
│   ├── encoder.rs            # sensor::encoder
│   └── mod.rs                # sensor modülünün root'u
└── utils.rs                   # utils modülü
```

**src/main.rs**:
```rust
// Modülleri bildir
mod motor;
mod sensor;
mod utils;

// Modülleri kullan
use motor::MotorController;
use sensor::SensorReader;
use utils::helpers::format_hiz;

fn main() {
    let motor = MotorController::new();
    let sensor = SensorReader::new();
    
    println!("Hız: {}", format_hiz(1500));
}
```

**src/motor.rs** (veya src/motor/mod.rs):
```rust
// Alt modülleri bildir
pub mod kontrol;
pub mod hiz_profili;

// Bu modülün kendi öğeleri
pub struct MotorController {
    pub hiz: u16,
    pozisyon: i32,  // private (varsayılan)
}

impl MotorController {
    pub fn new() -> Self {
        Self {
            hiz: 0,
            pozisyon: 0,
        }
    }
    
    pub fn hareket_et(&mut self, hedef: i32) {
        self.pozisyon = hedef;
    }
    
    // Private metod
    fn hiz_ayarla(&mut self, hiz: u16) {
        self.hiz = hiz;
    }
}

// Alt modül içeriği
pub use kontrol::PidKontrol;
pub use hiz_profili::HizProfili;
```

**src/motor/kontrol.rs**:
```rust
pub struct PidKontrol {
    pub kp: f64,
    pub ki: f64,
    pub kd: f64,
}

impl PidKontrol {
    pub fn new(kp: f64, ki: f64, kd: f64) -> Self {
        Self { kp, ki, kd }
    }
    
    pub fn hesapla(&self, hata: f64) -> f64 {
        // PID hesaplama
        self.kp * hata
    }
}
```

## 3.3 Visibility (Görünürlük) Kuralları ⭐

Rust'ta visibility, C#'tan **çok daha katıdır**:

```
┌─────────────────────────────────────────────────────────┐
│ Rust Visibility Kuralları                               │
├─────────────────────────────────────────────────────────┤
│ Varsayılan: private (sadece tanımlandığı modülde)       │
│                                                         │
│ pub: Her yerden erişilebilir                            │
│ pub(crate): Sadece aynı crate içinde                    │
│ pub(super): Sadece üst modülde                          │
│ pub(in path): Belirli bir yolda                         │
│                                                         │
│ ÖNEMLİ: Bir öğeye erişmek için TÜM üst modüllerin       │
│ public olması gerekir!                                  │
└─────────────────────────────────────────────────────────┘
```

**Örnek**:
```rust
// src/lib.rs
pub mod motor {
    // motor modülü public
    
    pub struct MotorController {
        pub hiz: u16,           // public alan
        pozisyon: i32,          // private alan (varsayılan)
        pub(crate) id: u32,     // crate içinde public
    }
    
    impl MotorController {
        pub fn new() -> Self { /* ... */ }      // public metod
        fn hiz_ayarla(&mut self, h: u16) { }   // private metod
        pub(crate) fn reset(&mut self) { }     // crate içinde public
    }
    
    mod internal {
        // private modül - dışarıdan erişilemez
        
        pub fn helper() { }  // internal içinde public
        // Ama external'dan erişilemez çünkü parent private!
    }
}

// Kullanım
use crate::motor::MotorController;  // ✅

fn main() {
    let mut m = MotorController::new();
    m.hiz = 100;          // ✅ public alan
    // m.pozisyon = 0;    // ❌ private alan
    m.id = 1;             // ✅ crate içinde public
    m.hiz_ayarla(200);    // ❌ private metod
    m.reset();            // ✅ crate içinde public
}
```

## 3.4 `use` Statement ve Path'ler

```rust
// Absolute path (crate root'tan başla)
use crate::motor::MotorController;

// Relative path (mevcut modülden başla)
use self::motor::MotorController;

// Parent modülden
use super::utils::helpers;

// Glob import (tüm public öğeleri getir)
use crate::motor::*;

// Yeniden adlandırma
use crate::motor::MotorController as MC;

// Gruplama
use std::collections::{HashMap, HashSet, BTreeMap};

// Re-export (dışarıya yeniden export et)
pub use crate::motor::MotorController;
pub use crate::sensor::SensorReader;
```

## 3.5 C# vs Rust Modül Karşılaştırması

| Özellik | C# | Rust |
|---|---|---|
| Namespace tanımı | `namespace X { }` | `mod X { }` veya dosya |
| Dosya ile ilişki | Zayıf | Güçlü (dosya = modül) |
| Birden fazla namespace | ✅ Aynı dosyada | ❌ Bir dosyada bir modül |
| Partial class | ✅ | ❌ Yok |
| Visibility | `public`/`internal`/`private` | `pub`/`pub(crate)`/private |
| Import | `using X;` | `use X;` |
| Static import | `using static X;` | `use X::*;` |
| Alias | `using X = Y;` | `use Y as X;` |
| Nested namespace | `namespace X.Y.Z` | İç içe `mod` veya dosya yapısı |

---

# 📚 BÖLÜM 4: Workspace (Çoklu Crate Yönetimi) ⭐⭐⭐

C#'taki **Solution** yapısının Rust'taki karşılığı **Workspace**'tir. Birden fazla crate'i tek bir yerde yönetmenizi sağlar.

## 4.1 Workspace Oluşturma

```bash
# Ana klasör oluştur
mkdir motor_projesi
cd motor_projesi

# Workspace Cargo.toml oluştur
cat > Cargo.toml << 'EOF'
[workspace]
members = [
    "motor_kontrol",
    "motor_lib",
    "sensor_lib",
    "cli_araci",
]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["Ali Yılmaz <ali@example.com>"]

[workspace.dependencies]
# Ortak bağımlılıklar
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.35", features = ["full"] }
thiserror = "1.0"
EOF
```

**Oluşan yapı**:
```
motor_projesi/
├── Cargo.toml              # Workspace tanımı (Solution.sln)
├── Cargo.lock              # Tüm crate'ler için ortak lock dosyası
├── motor_kontrol/          # Binary crate (Console App)
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── motor_lib/              # Library crate (Class Library)
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
├── sensor_lib/             # Library crate
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
├── cli_araci/              # Binary crate (CLI Tool)
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── target/                 # Ortak derleme çıktıları
```

## 4.2 Crate'ler Arası Bağımlılık

**motor_kontrol/Cargo.toml**:
```toml
[package]
name = "motor_kontrol"
version.workspace = true      # Workspace'den al
edition.workspace = true
authors.workspace = true

[dependencies]
# Workspace içindeki diğer crate'ler
motor_lib = { path = "../motor_lib" }
sensor_lib = { path = "../sensor_lib" }

# Workspace bağımlılıklarını kullan
serde.workspace = true
tokio.workspace = true
thiserror.workspace = true

# Sadece bu crate'e özgü bağımlılıklar
clap = { version = "4.4", features = ["derive"] }
```

**motor_lib/Cargo.toml**:
```toml
[package]
name = "motor_lib"
version.workspace = true
edition.workspace = true

[dependencies]
serde.workspace = true
thiserror.workspace = true

# sensor_lib'e bağımlı
sensor_lib = { path = "../sensor_lib" }

[dev-dependencies]
# Sadece testler için
criterion = "0.5"

[[bench]]
name = "benchmarklar"
harness = false
```

## 4.3 Workspace Komutları

```bash
# Tüm workspace'i derle
cargo build

# Belirli bir crate'i derle
cargo build -p motor_lib

# Tüm workspace'i test et
cargo test

# Belirli bir crate'i test et
cargo test -p motor_lib

# Tüm workspace'i çalıştır (binary crate'ler)
cargo run -p motor_kontrol

# Tüm workspace için check
cargo check --workspace

# Release build
cargo build --release --workspace

# Format kontrolü
cargo fmt --all

# Lint kontrolü
cargo clippy --workspace
```

## 4.4 C# Solution vs Rust Workspace

| Özellik | C# Solution | Rust Workspace |
|---|---|---|
| Tanım dosyası | `Solution.sln` | `Cargo.toml` (workspace section) |
| Proje ekleme | `dotnet sln add` | `members` listesine ekle |
| Ortak bağımlılık | Directory.Packages.props | `[workspace.dependencies]` |
| Ortak versiyon | Directory.Build.props | `[workspace.package]` |
| Build çıktıları | Her projede ayrı | Tek `target/` klasörü |
| Lock dosyası | Her projede ayrı | Tek `Cargo.lock` |
| Cross-compile | ❌ Zor | ✅ Kolay |
| Bağımlılık çözümü | NuGet | crates.io + path |

---

# 📚 BÖLÜM 5: Dependencies (Bağımlılık) Yönetimi ⭐⭐

## 5.1 Bağımlılık Ekleme

```bash
# crates.io'dan bağımlılık ekle
cargo add serde --features derive
cargo add tokio --features full
cargo add reqwest --features json

# Belirli versiyon
cargo add serde@1.0.190

# Git repository'den
cargo add my_lib --git https://github.com/user/repo

# Path (lokal)
cargo add motor_lib --path ../motor_lib

# Kaldırma
cargo remove serde
```

**Cargo.toml'a otomatik eklenir**:
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.35", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
```

## 5.2 Bağımlılık Türleri

```toml
[dependencies]
# Normal bağımlılıklar - her zaman derlenir
serde = "1.0"

[dev-dependencies]
# Sadece test ve benchmark'larda kullanılır
criterion = "0.5"
mockall = "0.12"

[build-dependencies]
# build.rs çalıştırılırken kullanılır
cc = "1.0"
bindgen = "0.69"

[target.'cfg(windows)'.dependencies]
# Sadece Windows'ta
winapi = "0.3"

[target.'cfg(unix)'.dependencies]
# Sadece Unix'te
nix = "0.27"

[target.'cfg(target_arch = "arm")'.dependencies]
# Sadece ARM mimarisinde
cortex-m = "0.7"
```

## 5.3 Features (Özellikler)

```toml
[dependencies]
# Varsayılan özelliklerle
serde = "1.0"

# Varsayılan özellikler kapalı, sadece belirli özellikler
tokio = { version = "1.35", default-features = false, features = ["rt", "macros"] }

# Tüm özellikler
serde_json = { version = "1.0", features = ["preserve_order", "float_roundtrip"] }
```

**Kendi crate'inizde feature tanımlama**:
```toml
[features]
default = ["std", "logging"]
std = []
logging = ["dep:log", "dep:env_logger"]
embedded = ["no_std"]
full = ["std", "logging", "async_support"]

[dependencies]
log = { version = "0.4", optional = true }
env_logger = { version = "0.10", optional = true }
```

## 5.4 Cargo.lock Dosyası

```toml
# Cargo.lock - Bağımlılıkların tam versiyonları
[[package]]
name = "serde"
version = "1.0.193"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "25dd9975e68d0cb5aa11..."
dependencies = [
 "serde_derive",
]

[[package]]
name = "serde_derive"
version = "1.0.193"
source = "registry+https://github.com/rust-lang/crates.io-index"
```

**Önemli**: `Cargo.lock` dosyası **binary projeler için** Git'e commit edilmeli, **library projeler için** edilmemelidir.

## 5.5 C# vs Rust Bağımlılık Yönetimi

| Özellik | C# (NuGet) | Rust (Cargo) |
|---|---|---|
| Paket kaynağı | nuget.org | crates.io |
| Versiyon belirtimi | `<PackageReference>` | `serde = "1.0"` |
| Lock dosyası | `packages.lock.json` (opsiyonel) | `Cargo.lock` (otomatik) |
| Feature/flag | ❌ Yok | ✅ `features = [...]` |
| Conditional | `Condition` attribute | `[target.'cfg(...)'.dependencies]` |
| Dev dependencies | ❌ Yok | ✅ `[dev-dependencies]` |
| Build dependencies | ❌ Yok | ✅ `[build-dependencies]` |
| Path dependencies | ✅ Project reference | ✅ `path = "..."` |
| Git dependencies | ✅ PackageReference | ✅ `git = "..."` |
| Offline build | ✅ NuGet cache | ✅ `cargo vendor` |

---

# 📚 BÖLÜM 6: Build Profiles ve Konfigürasyon ⭐

## 6.1 Varsayılan Profiller

```toml
# Cargo.toml
[profile.dev]
# Geliştirme derlemesi (cargo build)
opt-level = 0
debug = true
split-debuginfo = "..."
strip = "none"
debug-assertions = true
overflow-checks = true
lto = false
panic = "unwind"
incremental = true
codegen-units = 256

[profile.release]
# Yayın derlemesi (cargo build --release)
opt-level = 3
debug = false  # veya "line-tables-only"
strip = "symbols"  # Binary boyutunu küçült
debug-assertions = false
overflow-checks = false
lto = "thin"  # veya "fat"
panic = "unwind"
incremental = false
codegen-units = 16

[profile.test]
# Test derlemesi (cargo test)
opt-level = 0
debug = true

[profile.bench]
# Benchmark derlemesi (cargo bench)
opt-level = 3
debug = false
```

## 6.2 Custom Profiller

```toml
[profile.dev-fast]
inherits = "dev"
opt-level = 1  # Daha hızlı derleme için optimizasyon

[profile.production]
inherits = "release"
lto = "fat"
codegen-units = 1
strip = "symbols"
panic = "abort"  # Binary boyutunu daha da küçült

[profile.embedded]
inherits = "release"
opt-level = "s"  # Boyut optimizasyonu
lto = true
panic = "abort"
strip = true
```

**Kullanım**:
```bash
cargo build --profile production
cargo build --profile embedded
```

## 6.3 Embedded Sistemler için Profil (RP2354B)

```toml
# RP2354B için optimize profil
[profile.embedded]
inherits = "release"
opt-level = "s"           # Boyut optimizasyonu
lto = true                # Link-time optimization
codegen-units = 1         # Daha iyi optimizasyon
panic = "abort"           # Panic handler boyutunu küçült
strip = true              # Debug bilgilerini kaldır
debug = false

[profile.dev-fast]
inherits = "dev"
opt-level = 1
```

## 6.4 Build Scripts (build.rs)

```rust
// build.rs - Derleme öncesi çalıştırılır
use std::env;
use std::path::Path;

fn main() {
    // Ortam değişkenlerine erişim
    let target = env::var("TARGET").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    
    // Koşullu derleme bayrakları ayarla
    if target.contains("arm") {
        println!("cargo:rustc-cfg=embedded");
    }
    
    // C/C++ kütüphaneleri ile bağlantı
    println!("cargo:rustc-link-lib=static=motor_driver");
    println!("cargo:rustc-link-search=native=/usr/local/lib");
    
    // Dosya değişikliklerini izle
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");
    
    // Kod üretimi
    let dest_path = Path::new(&out_dir).join("config.rs");
    std::fs::write(&dest_path, "pub const VERSION: &str = \"1.0.0\";").unwrap();
}
```

## 6.5 .cargo/config.toml

```toml
# .cargo/config.toml - Proje bazlı Cargo konfigürasyonu

[build]
# Varsayılan hedef
target = "thumbv8m.main-none-eabihf"  # RP2354B için

# Varsayılan derleme bayrakları
rustflags = ["-C", "link-arg=-Tlink.x"]

[target.thumbv8m.main-none-eabihf]
# RP2354B için linker
runner = "probe-rs run --chip RP235x"
rustflags = ["-C", "link-arg=-Tlink.x"]

[target.x86_64-unknown-linux-gnu]
# Linux x86_64 için
runner = "./target/x86_64-unknown-linux-gnu/debug/motor_kontrol"

[alias]
# Kısayollar
build-embedded = "build --release --target thumbv8m.main-none-eabihf"
flash = "run --release --target thumbv8m.main-none-eabihf"
test-all = "test --workspace --all-features"

[env]
# Ortam değişkenleri
DEFMT_LOG = "info"
```

---

# 📚 BÖLÜM 7: Test Organizasyonu ⭐

## 7.1 Test Türleri ve Konumları

```
motor_lib/
├── src/
│   ├── lib.rs
│   ├── motor.rs
│   └── sensor.rs
├── tests/                    # Integration testler
│   ├── motor_test.rs
│   └── sensor_test.rs
├── benches/                  # Benchmark'lar
│   └── benchmarklar.rs
└── examples/                 # Örnekler
    └── basit_kullanim.rs
```

## 7.2 Unit Testler (Aynı Dosyada)

```rust
// src/motor.rs
pub struct Motor {
    pub hiz: u16,
}

impl Motor {
    pub fn yeni() -> Self {
        Self { hiz: 0 }
    }
    
    pub fn hiz_ayarla(&mut self, hiz: u16) {
        self.hiz = hiz;
    }
}

// Unit testler aynı dosyada
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_yeni_motor() {
        let motor = Motor::yeni();
        assert_eq!(motor.hiz, 0);
    }
    
    #[test]
    fn test_hiz_ayarla() {
        let mut motor = Motor::yeni();
        motor.hiz_ayarla(1500);
        assert_eq!(motor.hiz, 1500);
    }
}
```

## 7.3 Integration Testler (Ayrı Dosyada)

```rust
// tests/motor_test.rs
use motor_lib::motor::Motor;

#[test]
fn test_motor_entegrasyon() {
    let mut motor = Motor::yeni();
    motor.hiz_ayarla(1000);
    
    // Gerçek dünya senaryosu
    assert_eq!(motor.hiz, 1000);
}

#[test]
fn test_motor_sinir_degerleri() {
    let mut motor = Motor::yeni();
    motor.hiz_ayarla(u16::MAX);
    assert_eq!(motor.hiz, u16::MAX);
}
```

## 7.4 Örnekler (Examples)

```rust
// examples/basit_kullanim.rs
use motor_lib::motor::Motor;

fn main() {
    let mut motor = Motor::yeni();
    motor.hiz_ayarla(1500);
    
    println!("Motor hızı: {} RPM", motor.hiz);
}
```

**Çalıştırma**:
```bash
cargo run --example basit_kullanim
```

---

# 📚 BÖLÜM 8: Publishing (Yayınlama) ⭐

## 8.1 crates.io'ya Yayınlama

```toml
# Cargo.toml - Gerekli alanlar
[package]
name = "motor_lib"
version = "0.1.0"
edition = "2021"
authors = ["Ali Yılmaz <ali@example.com>"]
description = "Step motor kontrol kütüphanesi"
license = "MIT OR Apache-2.0"
repository = "https://github.com/kullanici/motor_lib"
documentation = "https://docs.rs/motor_lib"
homepage = "https://example.com"
keywords = ["motor", "stepper", "embedded", "rp2354"]
categories = ["embedded", "hardware-support"]
readme = "README.md"
exclude = [
    "tests/",
    "examples/",
    ".github/",
]
```

**Yayınlama**:
```bash
# Giriş yap
cargo login

# Paket doğrulama
cargo package

# Yayınlama (kuru çalıştırma)
cargo publish --dry-run

# Gerçek yayınlama
cargo publish
```

## 8.2 Private Registry

```toml
# .cargo/config.toml
[registries]
sirket-registry = { index = "https://git.sirket.com/packages/index" }

[dependencies]
sirket_motor_lib = { version = "1.0", registry = "sirket-registry" }
```

## 8.3 Workspace Publishing

```bash
# Tüm workspace'i yayınla
cargo workspaces publish

# Belirli crate'i yayınla
cargo publish -p motor_lib

# Versiyon güncelle
cargo set-version 0.2.0 -p motor_lib
```

---

# 📚 BÖLÜM 9: Proje Şablonları ve Araçlar ⭐

## 9.1 cargo-generate ile Şablonlar

```bash
# cargo-generate kur
cargo install cargo-generate

# Şablon kullanarak proje oluştur
cargo generate --git https://github.com/user/template
cargo generate --git https://github.com/rp-rs/rp2350-template
```

## 9.2 Popüler Şablonlar

```bash
# Web uygulaması
cargo generate --git https://github.com/actix/actix-web

# CLI aracı
cargo generate --git https://github.com/rust-cli/cli-template

# Embedded (RP2354B)
cargo generate --git https://github.com/rp-rs/rp2350-template

# Library
cargo generate --git https://github.com/rust-lang/library-template
```

## 9.3 Proje Yapısı Şablonu (RP2354B için)

```
motor_projesi/
├── Cargo.toml                    # Workspace tanımı
├── Cargo.lock
├── .cargo/
│   └── config.toml               # Hedef ve runner ayarları
├── memory.x                      # RP2354B memory layout
├── link.x                        # Linker script
├── .vscode/
│   └── settings.json             # IDE ayarları
├── motor_lib/                    # Core library
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── motor/
│       │   ├── mod.rs
│       │   ├── kontrol.rs
│       │   └── hiz_profili.rs
│       ├── sensor/
│       │   ├── mod.rs
│       │   ├── adc.rs
│       │   └── encoder.rs
│       └── utils/
│           ├── mod.rs
│           └── math.rs
├── firmware/                     # Embedded firmware
│   ├── Cargo.toml
│   ├── build.rs
│   └── src/
│       ├── main.rs
│       ├── tasks/
│       │   ├── mod.rs
│       │   ├── motor_task.rs
│       │   └── sensor_task.rs
│       └── hal/
│           ├── mod.rs
│           └── gpio.rs
├── cli/                          # Desktop CLI tool
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── web_api/                      # Web API (test için)
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── docs/                         # Dokümantasyon
    ├── architecture.md
    └── api.md
```

---

# 📚 BÖLÜM 10: Pratik Örnekler ⭐⭐

## 10.1 Basit Workspace Örneği

**Cargo.toml** (root):
```toml
[workspace]
members = ["motor_lib", "motor_cli"]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"

[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.35", features = ["full"] }
thiserror = "1.0"
```

**motor_lib/Cargo.toml**:
```toml
[package]
name = "motor_lib"
version.workspace = true
edition.workspace = true

[dependencies]
serde.workspace = true
thiserror.workspace = true
```

**motor_cli/Cargo.toml**:
```toml
[package]
name = "motor_cli"
version.workspace = true
edition.workspace = true

[dependencies]
motor_lib = { path = "../motor_lib" }
tokio.workspace = true
clap = { version = "4.4", features = ["derive"] }
```

## 10.2 Conditional Compilation ile Multi-Platform

```toml
# Cargo.toml
[dependencies]
# Her platform için
serde = "1.0"

[target.'cfg(target_os = "windows")'.dependencies]
winapi = "0.3"

[target.'cfg(target_os = "linux")'.dependencies]
nix = "0.27"

[target.'cfg(target_arch = "arm")'.dependencies]
cortex-m = "0.7"
embedded-hal = "1.0"
```

## 10.3 Feature Flags ile Opsiyonel Özellikler

```toml
[features]
default = ["std", "logging"]
std = []
logging = ["dep:log", "dep:env_logger"]
async_support = ["dep:tokio"]
embedded = ["no_std", "dep:embedded-hal"]
full = ["std", "logging", "async_support"]

[dependencies]
log = { version = "0.4", optional = true }
env_logger = { version = "0.10", optional = true }
tokio = { version = "1.35", optional = true, features = ["full"] }
embedded-hal = { version = "1.0", optional = true }
```

**Kodda kullanım**:
```rust
#[cfg(feature = "logging")]
use log::{info, debug};

#[cfg(feature = "async_support")]
use tokio;

#[cfg(feature = "embedded")]
use embedded_hal;

pub fn islem() {
    #[cfg(feature = "logging")]
    info!("İşlem başlatıldı");
    
    // ...
}
```

## 10.4 Build Script ile Kod Üretimi

**build.rs**:
```rust
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("version.rs");
    
    let version = env::var("CARGO_PKG_VERSION").unwrap();
    let build_date = chrono::Utc::now().to_rfc3339();
    
    let code = format!(
        r#"
pub const VERSION: &str = "{}";
pub const BUILD_DATE: &str = "{}";
"#,
        version, build_date
    );
    
    fs::write(&dest_path, code).unwrap();
    
    println!("cargo:rerun-if-changed=build.rs");
}
```

**src/lib.rs**:
```rust
include!(concat!(env!("OUT_DIR"), "/version.rs"));

pub fn print_version() {
    println!("Versiyon: {}", VERSION);
    println!("Build tarihi: {}", BUILD_DATE);
}
```

---

# 📚 BÖLÜM 11: Best Practices ⭐⭐

## 11.1 ✅ İyi Pratikler

### 1. Modüler Yapı

```rust
// ✅ İYİ: Küçük, odaklı modüller
src/
├── lib.rs          # Sadece modül bildirimleri ve re-exports
├── motor/
│   ├── mod.rs      # Motor modülünün public API'si
│   ├── kontrol.rs  # Kontrol mantığı
│   └── hiz.rs      # Hız profili
└── sensor/
    ├── mod.rs
    ├── adc.rs
    └── encoder.rs

// ❌ KÖTÜ: Büyük, tek dosya
src/
└── lib.rs  // 5000+ satır kod!
```

### 2. Public API Tasarımı

```rust
// ✅ İYİ: Minimal public API
pub mod motor {
    // Sadece gerekli öğeleri public yap
    pub struct MotorController { /* ... */ }
    
    impl MotorController {
        pub fn new() -> Self { /* ... */ }
        pub fn hareket_et(&mut self, hedef: i32) { /* ... */ }
        
        // İç detayları private tut
        fn internal_hesaplama(&self) -> f64 { /* ... */ }
    }
    
    // Re-export ile temiz API
    pub use self::kontrol::PidKontrol;
}

// ❌ KÖTÜ: Her şeyi public yap
pub mod motor {
    pub struct MotorController { /* ... */ }
    pub struct InternalState { /* ... */ }  // Dışarıya sızmalı mı?
    pub fn helper_function() { /* ... */ }  // Gerçekten public olmalı mı?
}
```

### 3. Error Handling

```rust
// ✅ İYİ: thiserror ile tip güvenli hatalar
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MotorHatasi {
    #[error("Hız sınır dışı: {0} RPM (max: {max})", hiz)]
    HizHatasi { hiz: u16, max: u16 },
    
    #[error("İletişim hatası: {0}")]
    IletisimHatasi(#[from] std::io::Error),
}

// ❌ KÖTÜ: String hatalar
fn islem() -> Result<(), String> {
    Err("Bir hata oluştu".to_string())
}
```

### 4. Documentation

```rust
// ✅ İYİ: Dokümantasyon örnekleri ile
/// Motor kontrolcüsü
///
/// # Examples
///
/// ```
/// use motor_lib::MotorController;
///
/// let mut motor = MotorController::new();
/// motor.hareket_et(1000);
/// ```
pub struct MotorController {
    // ...
}

// ❌ KÖTÜ: Dokümantasyon yok
pub struct MotorController {
    // ...
}
```

## 11.2 ❌ Anti-Patterns

### 1. God Module

```rust
// ❌ KÖTÜ: Her şeyi içeren dev modül
mod her_sey {
    pub struct Motor { }
    pub struct Sensor { }
    pub struct Controller { }
    pub struct Database { }
    pub struct Network { }
    // 2000+ satır...
}

// ✅ İYİ: Küçük, odaklı modüller
mod motor { }
mod sensor { }
mod controller { }
mod database { }
mod network { }
```

### 2. Circular Dependencies

```rust
// ❌ KÖTÜ: motor → sensor → motor
// motor.rs
use crate::sensor::Sensor;

// sensor.rs
use crate::motor::Motor;

// ✅ İYİ: Trait ile soyutla
// common.rs
pub trait SensorReader {
    fn oku(&self) -> f64;
}

// motor.rs
use crate::common::SensorReader;
```

### 3. Excessive Re-exports

```rust
// ❌ KÖTÜ: Her şeyi root'ta re-export
pub use motor::*;
pub use sensor::*;
pub use utils::*;
// Kullanıcı hangi modülden geldiğini bilmiyor!

// ✅ İYİ: Anlamlı re-exports
pub use motor::MotorController;
pub use sensor::SensorReader;
// Veya modül yapısını koru
pub mod motor;
pub mod sensor;
```

---

# 🎯 ÖZET: Project Structure Kontrol Listesi

| Özellik | C# | Rust |
|---|---|---|
| Solution | `.sln` dosyası | `[workspace]` in Cargo.toml |
| Project | `.csproj` | `Cargo.toml` |
| Namespace | `namespace X` | `mod X` |
| Assembly | `.dll`/`.exe` | Crate (binary/library) |
| Package | NuGet | crates.io |
| Entry point | `Program.cs` | `main.rs` veya `lib.rs` |
| Visibility | `public`/`internal`/`private` | `pub`/`pub(crate)`/private |
| Build system | MSBuild | Cargo |
| Test framework | xUnit, NUnit, MSTest | Built-in `#[test]` |
| Documentation | XML comments (`///`) | Doc comments (`///`) |
| Code generation | Source Generators | Macros, build.rs |
| Multi-target | Target frameworks | Target triples |
| Conditional | `#if`, MSBuild conditions | `cfg`, target-specific deps |

---

# 🚀 Son Tavsiyeler

## 1. Workspace Kullanın

Birden fazla crate içeren projeler için **mutlaka workspace** kullanın:
- Ortak bağımlılık yönetimi
- Tek `Cargo.lock` dosyası
- Kolay cross-crate bağımlılıklar
- Tutarlı versiyon yönetimi

## 2. Modüler Tasarım

- Küçük, odaklı modüller oluşturun
- Her modül tek bir sorumluluğa sahip olsun
- Public API'yi minimal tutun
- Internal detayları gizleyin

## 3. Feature Flags

Opsiyonel özellikler için **feature flags** kullanın:
- Binary boyutunu küçültün
- Embedded sistemlerde `no_std` desteği
- Kullanıcıya seçim hakkı verin

## 4. Build Profiles

Farklı kullanım senaryoları için **custom profiller** oluşturun:
- `dev`: Hızlı derleme
- `release`: Optimize edilmiş
- `embedded`: Boyut optimize, `no_std`
- `production`: LTO, strip, panic=abort

## 5. Documentation

- Her public öğe için dokümantasyon yazın
- Örnekler ekleyin (doc tests)
- README.md oluşturun
- CHANGELOG.md tutun

## 6. Testing

- Unit testleri kodla birlikte yazın
- Integration testleri `tests/` klasöründe tutun
- Örnekleri `examples/` klasöründe tutun
- Benchmark'ları `benches/` klasöründe tutun

## 7. CI/CD

```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo fmt --all -- --check
      - run: cargo clippy --workspace -- -D warnings
      - run: cargo test --workspace
      - run: cargo build --release --workspace
```

## 8. RP2354B Projeniz İçin Önerilen Yapı

```
motor_projesi/
├── Cargo.toml                    # Workspace
├── .cargo/
│   └── config.toml               # ARM target, probe-rs runner
├── motor_lib/                    # Core library (host'ta test edilebilir)
├── firmware/                     # RP2354B firmware (embassy-rs)
│   ├── Cargo.toml
│   ├── memory.x
│   ├── build.rs
│   └── src/
│       ├── main.rs
│       └── tasks/
├── cli/                          # Desktop CLI tool
├── web_api/                      # Test için web API
└── docs/                         # Dokümantasyon
```

## 9. Araçlar

- **cargo-edit**: `cargo add`, `cargo remove`
- **cargo-watch**: Otomatik yeniden derleme
- **cargo-expand**: Makro genişletmelerini gör
- **cargo-outdated**: Güncel olmayan bağımlılıkları bul
- **cargo-audit**: Güvenlik açıklarını kontrol et
- **cargo-udeps**: Kullanılmayan bağımlılıkları bul
- **cargo-geiger**: Unsafe kod kullanımını analiz et

## 10. IDE Entegrasyonu

**VS Code**:
- rust-analyzer extension
- CodeLLDB (debugging)
- Even Better TOML

**CLion/IntelliJ**:
- Rust plugin
- Native debugging

**Hızlı Başlangıç**:
```bash
# rust-analyzer için
rustup component add rust-src rust-analyzer

# Proje oluştur
cargo new motor_projesi
cd motor_projesi

# İlk derleme
cargo build

# Çalıştır
cargo run

# Test
cargo test

# Format
cargo fmt

# Lint
cargo clippy
```

> 🦀 **Unutmayın:**
> - **Workspace**, birden fazla crate içeren projeler için **zorunludur**
> - **Modül sistemi**, C#'taki namespace'ten **çok daha katıdır** - visibility kurallarına dikkat edin
> - **Feature flags**, embedded sistemlerde **kritik önem** taşır - `no_std` desteği için kullanın
> - **Build profiles**, farklı kullanım senaryoları için optimize edilmiş derlemeler sağlar
> - **Cargo**, C#'taki NuGet + MSBuild + Solution yapısını **tek bir araçta** birleştirir
> - **RP2354B projenizde** workspace kullanarak firmware, library ve CLI araçlarını aynı yerde yönetin
> 
> Rust'ın proje yapısı, başta karmaşık gelse de, bir kez anladığınızda C#'ta asla ulaşamayacağınız **esneklik ve kontrol** sağlar. Özellikle embedded sistemlerde, workspace ve feature flags kombinasyonu ile hem host'ta test edilebilir hem de hedef donanımda çalıştırılabilen projeler geliştirebilirsiniz! 🚀