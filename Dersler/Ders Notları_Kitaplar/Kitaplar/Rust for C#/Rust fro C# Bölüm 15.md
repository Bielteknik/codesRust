# 🔨 Rust for .NET Developers: Compilation and Building (Derleme ve Yapı)

Bu bölüm, Rust'ın derleme sisteminin temellerinden ileri düzey optimizasyon tekniklerine kadar kapsamlı bir rehberdir. C# dünyasında **MSBuild, Roslyn, JIT compilation** ve **Visual Studio build system** ile çalışmaya alışkınız. Rust ise **Cargo** build sistemi, ** Ahead-of-Time (AOT) compilation**, **LLVM backend** ve **zero-cost abstraction** felsefesi ile çok farklı bir yaklaşım sunar.

> 🎯 **Temel Fark:** C# kodu önce IL (Intermediate Language)'e derlenir, sonra **JIT (Just-In-Time)** tarafından çalışma zamanında makine koduna çevrilir. Rust ise doğrudan **makine koduna** derlenir (AOT) - JIT overhead'i yoktur, bu da embedded sistemler (RP2354B) için kritik önem taşır!

---

# 📚 BÖLÜM 1: Temel Derleme Kavramları

## 1.1 C# vs Rust Derleme Mimarisi

```
┌─────────────────────────────────────────────────────────────────┐
│ C# Derleme Süreci                                               │
├─────────────────────────────────────────────────────────────────┤
│ Source Code (.cs)                                               │
│    ↓                                                            │
│ Roslyn Compiler                                                 │
│    ↓                                                            │
│ IL Assembly (.dll) ← Metadata + Bytecode                        │
│    ↓                                                            │
│ JIT Compiler (CLR) ← Çalışma zamanında                          │
│    ↓                                                            │
│ Native Machine Code                                             │
│                                                                 │
│ ⚠️ JIT overhead var                                             │
│ ⚠️ Runtime gerektirir (.NET Runtime)                            │
│ ⚠️ Warm-up süresi var                                           │
│ ❌ Embedded sistemlerde çalışmaz                                │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│ Rust Derleme Süreci                                             │
├─────────────────────────────────────────────────────────────────┤
│ Source Code (.rs)                                               │
│    ↓                                                            │
│ rustc Compiler (LLVM frontend)                                  │
│    ↓                                                            │
│ MIR (Mid-level IR) ← Optimizasyonlar                            │
│    ↓                                                            │
│ LLVM IR ← LLVM optimizasyonları                                 │
│    ↓                                                            │
│ LLVM Backend ← Target-specific code generation                  │
│    ↓                                                            │
│ Native Machine Code (ELF/PE/Mach-O)                             │
│                                                                 │
│ ✅ JIT overhead YOK                                             │
│ ✅ Runtime GEREKMEZ (bare metal çalışabilir)                    │
│ ✅ Warm-up süresi YOK                                           │
│ ✅ Embedded sistemlerde çalışır (RP2354B, STM32, vs.)           │
└─────────────────────────────────────────────────────────────────┘
```

## 1.2 Derleme Karşılaştırma Tablosu

| Özellik | C# | Rust |
|---|---|---|
| Derleme tipi | AOT + JIT | Sadece AOT |
| Compiler | Roslyn (C#) | rustc (LLVM tabanlı) |
| Intermediate code | IL (Common Intermediate Language) | MIR → LLVM IR |
| Backend | JIT (CoreCLR) | LLVM |
| Output | .dll/.exe (IL + metadata) | Binary (ELF/PE/Mach-O) |
| Runtime | .NET Runtime (CLR) | Yok (veya minimal std) |
| Start-up time | Yavaş (JIT warm-up) | Hızlı (direkt çalışır) |
| Memory footprint | Büyük (GC + JIT) | Küçük (sadece kod) |
| Cross-platform | ✅ (IL portable) | ✅ (LLVM target) |
| Embedded | ❌ | ✅ (no_std) |
| Link-time optimization | ❌ | ✅ (LTO) |
| Whole-program optimization | ❌ | ✅ (LTO ile) |

---

# 📚 BÖLÜM 2: Cargo Build Sistemi ⭐

## 2.1 Cargo Nedir?

Cargo, Rust'ın **paket yöneticisi** ve **build sistemidir**. C#'taki NuGet + MSBuild + dotnet CLI kombinasyonunun tek bir araçta birleşmiş halidir.

```bash
# C# eşdeğerleri
cargo build       # dotnet build
cargo run         # dotnet run
cargo test        # dotnet test
cargo check       # dotnet build (sadece syntax check)
cargo clean       # dotnet clean
cargo publish     # dotnet pack + nuget push
cargo install     # dotnet tool install
```

## 2.2 Temel Cargo Komutları

```bash
# Debug build (varsayılan)
cargo build
# Çıktı: target/debug/motor_kontrol

# Release build (optimize edilmiş)
cargo build --release
# Çıktı: target/release/motor_kontrol

# Çalıştır
cargo run
cargo run --release

# Sadece kontrol et (derleme yapma, hızlı)
cargo check
cargo check --release

# Temizle
cargo clean

# Bağımlılıkları güncelle
cargo update

# Bağımlılık ekle
cargo add serde --features derive
cargo add tokio

# Test et
cargo test
cargo test --release

# Benchmark
cargo bench

# Format kontrolü
cargo fmt --check
cargo fmt  # Formatla

# Lint
cargo clippy
cargo clippy --fix  # Otomatik düzelt
```

## 2.3 Cargo.toml Yapısı

```toml
[package]
name = "motor_kontrol"
version = "0.1.0"
edition = "2021"  # Rust versiyonu (C#'taki LangVersion)
authors = ["Ali Yılmaz <ali@example.com>"]
description = "Step motor kontrol sistemi"
license = "MIT"
repository = "https://github.com/user/motor_kontrol"

# Binary hedefi (C#'taki OutputType: Exe)
[[bin]]
name = "motor_kontrol"
path = "src/main.rs"

# Library hedefi (C#'taki OutputType: Library)
[lib]
name = "motor_lib"
path = "src/lib.rs"
crate-type = ["cdylib", "rlib"]

# Bağımlılıklar (C#'taki PackageReference)
[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.35", features = ["full"] }

# Development bağımlılıkları (sadece test/bench)
[dev-dependencies]
criterion = "0.5"
mockall = "0.12"

# Build bağımlılıkları (build.rs için)
[build-dependencies]
cc = "1.0"
bindgen = "0.69"

# Feature'lar (opsiyonel özellikler)
[features]
default = ["std", "logging"]
std = []
logging = ["dep:log"]
embedded = ["no_std"]

# Profil ayarları
[profile.dev]
opt-level = 0
debug = true

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

---

# 📚 BÖLÜM 3: Derleme Profilleri ⭐⭐

## 3.1 Varsayılan Profiller

Rust'ta 4 varsayılan profil vardır:

```toml
# 1. dev - Geliştirme derlemesi (cargo build)
[profile.dev]
opt-level = 0              # Optimizasyon yok (hızlı derleme)
debug = true               # Debug bilgileri dahil
split-debuginfo = "..."    # Platform'a göre değişir
strip = "none"             # Semboller korunur
debug-assertions = true    # Debug kontrolleri aktif
overflow-checks = true     # Taşma kontrolleri aktif
incremental = true         # Incremental compilation
codegen-units = 256        # Paralel derleme (hızlı ama optimize değil)
panic = "unwind"           # Stack unwind
lto = false                # LTO kapalı

# 2. release - Yayın derlemesi (cargo build --release)
[profile.release]
opt-level = 3              # Maksimum optimizasyon
debug = false              # Debug bilgileri yok (veya "line-tables-only")
strip = "symbols"          # Semboller kaldırılır (küçük binary)
debug-assertions = false   # Debug kontrolleri kapalı
overflow-checks = false    # Taşma kontrolleri kapalı
incremental = false        # Incremental kapalı
codegen-units = 16         # Daha iyi optimizasyon
panic = "unwind"           # Stack unwind
lto = "thin"               # Thin LTO (hızlı + iyi optimizasyon)

# 3. test - Test derlemesi (cargo test)
[profile.test]
opt-level = 0              # Optimizasyon yok
debug = true               # Debug bilgileri
inherits = "dev"           # dev profilinden miras alır

# 4. bench - Benchmark derlemesi (cargo bench)
[profile.bench]
opt-level = 3              # Maksimum optimizasyon
debug = false
strip = "none"
inherits = "release"       # release profilinden miras alır
```

## 3.2 Custom Profiller

Kendi profillerinizi oluşturabilirsiniz:

```toml
# Hızlı geliştirme derlemesi
[profile.dev-fast]
inherits = "dev"
opt-level = 1              # Biraz optimizasyon
codegen-units = 16         # Daha hızlı derleme

# Production derlemesi (maksimum optimizasyon)
[profile.production]
inherits = "release"
lto = "fat"                # Fat LTO (daha yavaş ama daha iyi)
codegen-units = 1          # Tek codegen unit (maksimum optimizasyon)
panic = "abort"            # Panic'te abort (daha küçük binary)
strip = "symbols"          # Tüm semboller kaldır

# Embedded derlemesi (RP2354B için)
[profile.embedded]
inherits = "release"
opt-level = "s"            # Boyut optimizasyonu (z veya s)
lto = true                 # Full LTO
codegen-units = 1
panic = "abort"
strip = true               # Tüm debug bilgileri kaldır
debug = false
incremental = false

# Boyut odaklı derleme
[profile.size-optimized]
inherits = "release"
opt-level = "z"            # Agresif boyut optimizasyonu
lto = true
codegen-units = 1
panic = "abort"
strip = "symbols"
```

**Kullanım**:
```bash
cargo build --profile production
cargo build --profile embedded
cargo build --profile size-optimized
```

## 3.3 Profil Karşılaştırma (RP2354B Firmware)

```bash
# Farklı profillerle derleme ve boyut karşılaştırması
cargo build --release --target thumbv8m.main-none-eabihf
ls -lh target/thumbv8m.main-none-eabihf/release/firmware
# Çıktı: 450 KB

cargo build --profile embedded --target thumbv8m.main-none-eabihf
ls -lh target/thumbv8m.main-none-eabihf/embedded/firmware
# Çıktı: 180 KB (%60 küçülme!)

cargo build --profile size-optimized --target thumbv8m.main-none-eabihf
ls -lh target/thumbv8m.main-none-eabihf/size-optimized/firmware
# Çıktı: 150 KB (%67 küçülme!)
```

---

# 📚 BÖLÜM 4: Cross-Compilation ⭐⭐⭐ (RP2354B için Kritik!)

## 4.1 Target Triple Nedir?

Rust, LLVM'in cross-compilation yeteneklerini kullanarak **tek bir makinede** farklı platformlar için derleme yapabilir.

```
Target Triple Formatı: <arch><sub>-<vendor>-<os>-<env>

Örnekler:
- x86_64-unknown-linux-gnu      # Linux x86_64
- x86_64-pc-windows-msvc        # Windows x86_64
- aarch64-apple-darwin          # macOS ARM64 (M1/M2)
- thumbv8m.main-none-eabihf     # ARM Cortex-M33 (RP2354B!)
- wasm32-unknown-unknown        # WebAssembly
```

## 4.2 Target Listeleme ve Yükleme

```bash
# Yüklü target'ları listele
rustup target list --installed

# Tüm target'ları listele
rustup target list

# RP2354B için target yükle
rustup target add thumbv8m.main-none-eabihf

# Windows için cross-compile
rustup target add x86_64-pc-windows-gnu

# WebAssembly için
rustup target add wasm32-unknown-unknown
```

## 4.3 RP2354B için Cross-Compilation ⭐

### 4.3.1 Target Kurulumu

```bash
# 1. ARM target yükle
rustup target add thumbv8m.main-none-eabihf

# 2. LLVM tools kur (objcopy, size için)
# Ubuntu/Debian:
sudo apt install binutils-arm-none-eabi

# macOS:
brew install arm-none-eabi-binutils

# Windows:
# ARM GCC toolchain kur: https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain
```

### 4.3.2 .cargo/config.toml

```toml
# .cargo/config.toml
[target.thumbv8m.main-none-eabihf]
# Flash tool olarak probe-rs kullan
runner = "probe-rs run --chip RP235x"

# Linker script
rustflags = ["-C", "link-arg=-Tlink.x"]

[build]
# Varsayılan target (opsiyonel)
target = "thumbv8m.main-none-eabihf"

# Tüm derlemeler için ek bayraklar
rustflags = ["-C", "link-arg=--nmagic"]
```

### 4.3.3 memory.x (RP2354B Memory Layout)

```
/* RP2354B Memory Layout */
MEMORY
{
    /* Boot2 - 256 byte (flash'tan RAM'e kopyalanır) */
    BOOT2 : ORIGIN = 0x10000000, LENGTH = 0x100
    
    /* Flash - 4MB (kod ve sabit veriler) */
    FLASH : ORIGIN = 0x10000100, LENGTH = 4096K - 0x100
    
    /* RAM - 520KB (toplam) */
    /* SRAM0 - 256KB */
    RAM : ORIGIN = 0x20000000, LENGTH = 256K
    
    /* SRAM1 - 256KB (DMA için) */
    SRAM1 : ORIGIN = 0x20040000, LENGTH = 256K
    
    /* SRAM2 - 8KB */
    SRAM2 : ORIGIN = 0x20080000, LENGTH = 8K
    
    /* SRAM3 - 4KB (PIO için) */
    SRAM3 : ORIGIN = 0x20082000, LENGTH = 4K
    
    /* SRAM4 - 4KB (PIO için) */
    SRAM4 : ORIGIN = 0x20083000, LENGTH = 4K
    
    /* SRAM5 - 4KB */
    SRAM5 : ORIGIN = 0x20084000, LENGTH = 4K
}
```

### 4.3.4 link.x (Linker Script)

```
/* RP2354B Linker Script */
INCLUDE memory.x

SECTIONS
{
    /* Boot2 section - flash'ın başında olmalı */
    .boot2 ORIGIN(BOOT2) :
    {
        KEEP(*(.boot2));
        KEEP(*(.boot2.*));
    } > BOOT2
    
    /* Text section - kod */
    .text :
    {
        *(.vector_table);      /* Interrupt vector table */
        *(.text .text.*);
        *(.rodata .rodata.*);
    } > FLASH
    
    /* Data section - başlatılmış veriler */
    .data : AT(ADDR(.text) + SIZEOF(.text))
    {
        *(.data .data.*);
    } > RAM
    
    /* BSS section - sıfırlanmış veriler */
    .bss :
    {
        *(.bss .bss.*);
        *(COMMON);
    } > RAM
    
    /* Stack ve heap için yer ayır */
    .stack :
    {
        . = ALIGN(8);
        . = . + 0x2000;  /* 8KB stack */
        __stack_top = .;
    } > RAM
}

/* Entry point */
ENTRY(Reset)

/* Stack başlangıcı */
__stack_start = ORIGIN(RAM) + LENGTH(RAM);
```

### 4.3.5 RP2354B Firmware Derleme

**Cargo.toml**:
```toml
[package]
name = "motor_firmware"
version = "0.1.0"
edition = "2021"

[dependencies]
cortex-m = "0.7"
cortex-m-rt = "0.7"
rp235x-hal = { version = "0.1", features = ["rt"] }
embassy-executor = { version = "0.5", features = ["arch-cortex-m", "executor-thread"] }
embassy-time = "0.3"
defmt = "0.3"
defmt-rtt = "0.4"
panic-probe = { version = "0.3", features = ["print-defmt"] }

[profile.embedded]
inherits = "release"
opt-level = "s"
lto = true
codegen-units = 1
panic = "abort"
strip = true
debug = false
```

**Derleme komutları**:
```bash
# Debug build
cargo build --target thumbv8m.main-none-eabihf

# Release build
cargo build --release --target thumbv8m.main-none-eabihf

# Embedded profile ile (en optimize)
cargo build --profile embedded --target thumbv8m.main-none-eabihf

# Binary boyutunu kontrol et
cargo size --target thumbv8m.main-none-eabihf --release

# ELF'den binary'e çevir (flash için)
arm-none-eabi-objcopy -O binary \
  target/thumbv8m.main-none-eabihf/release/motor_firmware \
  target/motor_firmware.bin

# Flash et
probe-rs flash download \
  --chip RP235x \
  target/motor_firmware.bin

# Veya tek komutla (runner kullan)
cargo run --release
```

## 4.4 Windows için Cross-Compilation (Linux'tan)

```bash
# 1. Target yükle
rustup target add x86_64-pc-windows-gnu

# 2. MinGW-w64 toolchain kur
sudo apt install gcc-mingw-w64

# 3. .cargo/config.toml
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"

# 4. Derle
cargo build --release --target x86_64-pc-windows-gnu

# Çıktı: target/x86_64-pc-windows-gnu/release/motor_kontrol.exe
```

## 4.5 WebAssembly için Derleme

```bash
# Target yükle
rustup target add wasm32-unknown-unknown

# wasm-pack kur
cargo install wasm-pack

# Derle ve paketle
wasm-pack build --target web

# Çıktı: pkg/motor_kontrol_bg.wasm
```

---

# 📚 BÖLÜM 5: Derleme Zamanı Optimizasyonları ⭐⭐

## 5.1 Link-Time Optimization (LTO)

LTO, tüm crate'leri tek bir birim olarak optimize eder.

```toml
[profile.release]
# Thin LTO - Hızlı, iyi optimizasyon
lto = "thin"

# Fat LTO - Yavaş, en iyi optimizasyon
lto = "fat"

# veya
lto = true  # Fat LTO ile aynı
```

**Etkisi**:
```bash
# LTO olmadan
cargo build --release
ls -lh target/release/motor_kontrol
# 2.5 MB

# Thin LTO ile
cargo build --release  # lto = "thin"
ls -lh target/release/motor_kontrol
# 1.8 MB (%28 küçülme)

# Fat LTO ile
cargo build --release  # lto = "fat"
ls -lh target/release/motor_kontrol
# 1.5 MB (%40 küçülme)
```

## 5.2 Codegen Units

```toml
[profile.release]
# Tek codegen unit - Maksimum optimizasyon (yavaş derleme)
codegen-units = 1

# 16 codegen unit - Denge
codegen-units = 16

# 256 codegen unit - Hızlı derleme (az optimizasyon)
codegen-units = 256
```

## 5.3 Panic Strategy

```toml
[profile.release]
# Stack unwind - Default, daha büyük binary
panic = "unwind"

# Abort - Daha küçük binary, stack unwind yok
panic = "abort"
```

**Embedded için**:
```toml
[profile.embedded]
panic = "abort"  # RP2354B'de unwind desteği yok
```

## 5.4 Strip Symbols

```toml
[profile.release]
# Sembolleri kaldır
strip = "symbols"

# Debug bilgilerini kaldır
strip = "debuginfo"

# Sadece line tables tut
strip = "line-tables-only"

# Hiçbir şey kaldırma
strip = "none"
```

## 5.5 Optimizasyon Seviyeleri

```toml
[profile.release]
# Optimizasyon yok
opt-level = "0"

# Temel optimizasyon
opt-level = "1"

# Orta optimizasyon
opt-level = "2"

# Agresif optimizasyon
opt-level = "3"

# Boyut optimizasyonu
opt-level = "s"

# Agresif boyut optimizasyonu
opt-level = "z"
```

## 5.6 Optimizasyon Karşılaştırması

```bash
# Farklı optimizasyon seviyeleriyle derle ve karşılaştır
for level in 0 1 2 3 s z; do
    cargo build --release --config "profile.release.opt-level='$level'"
    size=$(stat -c%s target/release/motor_kontrol)
    echo "opt-level=$level: $size bytes"
done

# Tipik sonuçlar:
# opt-level=0: 2.8 MB
# opt-level=1: 2.2 MB
# opt-level=2: 1.9 MB
# opt-level=3: 1.8 MB
# opt-level=s: 1.5 MB
# opt-level=z: 1.4 MB
```

---

# 📚 BÖLÜM 6: Build Scripts (build.rs) ⭐⭐

## 6.1 build.rs Nedir?

`build.rs`, derleme **öncesinde** çalışan bir Rust script'idir. C#'taki MSBuild targets veya pre-build events'e benzer.

```toml
# Cargo.toml
[package]
name = "motor_kontrol"
build = "build.rs"  # Varsayılan zaten bu

[build-dependencies]
cc = "1.0"           # C/C++ derleme
bindgen = "0.69"     # C header'dan Rust binding
```

## 6.2 Temel build.rs Örneği

```rust
// build.rs
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Ortam değişkenlerine erişim
    let target = env::var("TARGET").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    let profile = env::var("PROFILE").unwrap();
    
    println!("cargo:warning=Building for target: {}", target);
    println!("cargo:warning=Build profile: {}", profile);
    
    // Kod üretimi
    let dest_path = Path::new(&out_dir).join("build_info.rs");
    let build_date = chrono::Utc::now().to_rfc3339();
    let git_hash = get_git_hash();
    
    let code = format!(
        r#"
pub const BUILD_DATE: &str = "{}";
pub const GIT_HASH: &str = "{}";
pub const TARGET: &str = "{}";
"#,
        build_date, git_hash, target
    );
    
    fs::write(&dest_path, code).unwrap();
    
    // Değişiklik izleme
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-env-changed=MOTOR_CONFIG");
}

fn get_git_hash() -> String {
    std::process::Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_else(|| "unknown".to_string())
}
```

**Kullanım**:
```rust
// src/main.rs
include!(concat!(env!("OUT_DIR"), "/build_info.rs"));

fn main() {
    println!("Build date: {}", BUILD_DATE);
    println!("Git hash: {}", GIT_HASH);
    println!("Target: {}", TARGET);
}
```

## 6.3 C/C++ Kütüphaneleri ile Bağlantı

```rust
// build.rs
use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    
    // C kodunu derle
    cc::Build::new()
        .file("src/motor_driver.c")
        .file("src/sensor_driver.c")
        .include("include")
        .opt_level(2)
        .debug(false)
        .compile("motor_driver");
    
    // Linker bayrakları
    println!("cargo:rustc-link-lib=static=motor_driver");
    
    // Platform-specific
    if target.contains("windows") {
        println!("cargo:rustc-link-lib=winmm");
    } else if target.contains("linux") {
        println!("cargo:rustc-link-lib=rt");
        println!("cargo:rustc-link-lib=pthread");
    }
}
```

## 6.4 Bindgen ile C Binding

```rust
// build.rs
use bindgen;

fn main() {
    // C header'dan Rust binding üret
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .allowlist_function("motor_.*")
        .allowlist_type("Motor.*")
        .generate()
        .expect("Unable to generate bindings");
    
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
```

```rust
// src/main.rs
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    unsafe {
        motor_init();
        motor_set_speed(1500);
    }
}
```

## 6.5 Koşullu Derleme Bayrakları

```rust
// build.rs
fn main() {
    let target = env::var("TARGET").unwrap();
    
    // ARM target için cfg bayrağı ayarla
    if target.contains("arm") || target.contains("thumb") {
        println!("cargo:rustc-cfg=embedded");
    }
    
    // Debug build için
    if env::var("PROFILE").unwrap() == "debug" {
        println!("cargo:rustc-cfg=debug_build");
    }
    
    // Feature detection
    if cfg!(feature = "high_precision") {
        println!("cargo:rustc-cfg=high_precision_mode");
    }
}
```

```rust
// src/main.rs
#[cfg(embedded)]
use embedded_hal as hal;

#[cfg(not(embedded))]
use std::time;

fn main() {
    #[cfg(embedded)]
    {
        // Embedded kod
    }
    
    #[cfg(not(embedded))]
    {
        // Desktop kod
    }
}
```

---

# 📚 BÖLÜM 7: Incremental Compilation ⭐

## 7.1 Incremental Compilation Nedir?

Incremental compilation, sadece değişen dosyaları yeniden derler. C#'taki "Fast Up-to-Date Check" benzeri ama daha gelişmiş.

```toml
[profile.dev]
incremental = true  # Varsayılan

[profile.release]
incremental = false  # Release'de kapalı (tam optimizasyon için)
```

## 7.2 Incremental Compilation Cache

```bash
# Cache konumu
# Linux/macOS: target/debug/incremental/
# Windows: target\debug\incremental\

# Cache temizle
cargo clean
cargo clean -p motor_lib  # Sadece belirli crate

# Incremental derleme zamanı karşılaştırması
time cargo build  # İlk: 30s
time cargo build  # İkinci (değişiklik yok): 0.5s
# Bir dosyayı değiştir
time cargo build  # Üçüncü: 2s (sadece değişen modül)
```

## 7.3 Incremental Compilation Limitasyonları

```
✅ Desteklenen:
- Kaynak kodu değişiklikleri
- Cargo.toml bağımlılık değişiklikleri
- Feature flag değişiklikleri

❌ Desteklenmeyen:
- rustc versiyon değişikliği
- Target değişikliği
- Bazı compiler flag değişiklikleri
```

---

# 📚 BÖLÜM 8: Dependency Resolution ⭐⭐

## 8.1 Semver (Semantic Versioning)

```toml
[dependencies]
# Tam versiyon
serde = "=1.0.193"

# Uyumluluk (varsayılan)
serde = "1.0"      # >=1.0.0, <2.0.0
serde = "1.0.193"  # >=1.0.193, <1.1.0

# wildcard
serde = "1.*"      # >=1.0.0, <2.0.0
serde = "*"        # Herhangi bir versiyon

# Karşılaştırma operatörleri
serde = ">=1.0, <1.2"
serde = "~1.0"     # >=1.0.0, <1.1.0
serde = "^1.0"     # >=1.0.0, <2.0.0 (varsayılan)

# Git
serde = { git = "https://github.com/serde-rs/serde" }
serde = { git = "https://github.com/serde-rs/serde", branch = "master" }
serde = { git = "https://github.com/serde-rs/serde", tag = "v1.0.193" }
serde = { git = "https://github.com/serde-rs/serde", rev = "abc123" }

# Path (lokal)
motor_lib = { path = "../motor_lib" }
```

## 8.2 Cargo.lock Dosyası

```toml
# Cargo.lock - Bağımlılıkların tam versiyonları
# Bu dosya OTOMATIK olarak oluşturulur, ELLE eklenmez!

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

**Önemli**:
- **Binary projeler**: `Cargo.lock` Git'e commit edilmeli
- **Library projeler**: `Cargo.lock` Git'e commit edilmemeli (`.gitignore`'a ekle)

## 8.3 Dependency Güncelleme

```bash
# Tüm bağımlılıkları güncelle
cargo update

# Belirli bir bağımlılığı güncelle
cargo update -p serde

# Belirli bir versiyona güncelle
cargo update -p serde --precise 1.0.193

# Bağımlılık ağacını göster
cargo tree
cargo tree -p serde  # Sadece serde ve bağımlılıkları
cargo tree -i serde  # serde'yi kim kullanıyor?

# Güncel olmayan bağımlılıkları bul
cargo outdated

# Güvenlik açıklarını kontrol et
cargo audit
```

## 8.4 Dependency Çakışmaları

```bash
# Farklı versiyonlar çakışırsa
cargo tree --duplicates

# Çözüm: Cargo.toml'da [patch] kullan
[patch.crates-io]
serde = { git = "https://github.com/serde-rs/serde", branch = "master" }
```

---

# 📚 BÖLÜM 9: Binary Boyutu Optimizasyonu ⭐⭐⭐ (RP2354B için Kritik!)

## 9.1 Binary Boyutunu Analiz Etme

```bash
# Boyut bilgisi
cargo size --release --target thumbv8m.main-none-eabihf
# Çıktı:
#    text    data     bss     dec     hex filename
#  180456    2048   16384  198888   308e8 motor_firmware

# Detaylı analiz
cargo bloat --release --target thumbv8m.main-none-eabihf

# Crate bazında boyut
cargo bloat --release --crates

# Fonksiyon bazında boyut
cargo bloat --release --functions

# NM ile sembol analizi
arm-none-eabi-nm --size-sort --print-size target/thumbv8m.main-none-eabihf/release/motor_firmware
```

## 9.2 Boyut Optimizasyon Teknikleri

```toml
# Cargo.toml
[profile.embedded]
inherits = "release"

# 1. Boyut optimizasyonu
opt-level = "z"      # Agresif boyut optimizasyonu

# 2. LTO
lto = true           # Tüm crate'leri birlikte optimize et

# 3. Codegen units
codegen-units = 1    # Tek unit, daha iyi optimizasyon

# 4. Panic strategy
panic = "abort"      # Unwind kodunu kaldır

# 5. Strip
strip = true         # Tüm sembolleri kaldır

# 6. Debug bilgileri
debug = false        # Debug bilgilerini kaldır

# 7. Overflow checks
overflow-checks = false

# 8. Debug assertions
debug-assertions = false
```

## 9.3 Cargo.toml Optimizasyonları

```toml
[dependencies]
# Gereksiz feature'ları kaldır
tokio = { version = "1.35", default-features = false, features = ["rt", "macros"] }

# Lightweight alternatifler kullan
# serde_json yerine simd-json (daha hızlı, daha küçük)
# reqwest yerine ureq (daha küçük)
```

## 9.4 Kod Optimizasyonları

```rust
// ❌ Kötü: Büyük string literal'ler
const MESAJ: &str = "Bu çok uzun bir mesaj, binary'ye gömülecek ve boyutu artıracak...";

// ✅ İyi: Kısa mesajlar veya PROGMEM (embedded)
const MESAJ: &str = "Hata!";

// ❌ Kötü: Gereksiz format! kullanımı
log!("{}", uzun_ifade);

// ✅ İyi: defmt (embedded için optimize)
defmt::info!("{}", uzun_ifade);

// ❌ Kötü: Büyük static veriler
static BUYUK_DIZI: [u8; 10000] = [0; 10000];

// ✅ İyi: Sadece gereken kadar
static KUCUK_DIZI: [u8; 100] = [0; 100];
```

## 9.5 Boyut Karşılaştırması (RP2354B Firmware)

```bash
# Varsayılan release
cargo build --release --target thumbv8m.main-none-eabihf
ls -lh target/thumbv8m.main-none-eabihf/release/motor_firmware
# 450 KB

# LTO ile
# [profile.release]
# lto = true
cargo build --release --target thumbv8m.main-none-eabihf
ls -lh target/thumbv8m.main-none-eabihf/release/motor_firmware
# 320 KB (%29 küçülme)

# Opt-level = "s"
# [profile.release]
# opt-level = "s"
cargo build --release --target thumbv8m.main-none-eabihf
ls -lh target/thumbv8m.main-none-eabihf/release/motor_firmware
# 280 KB (%38 küçülme)

# Tam optimize (embedded profile)
# [profile.embedded]
# opt-level = "z"
# lto = true
# codegen-units = 1
# panic = "abort"
# strip = true
cargo build --profile embedded --target thumbv8m.main-none-eabihf
ls -lh target/thumbv8m.main-none-eabihf/embedded/motor_firmware
# 150 KB (%67 küçülme!)
```

---

# 📚 BÖLÜM 10: CI/CD Entegrasyonu ⭐⭐

## 10.1 GitHub Actions

```yaml
# .github/workflows/ci.yml
name: CI/CD

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always

jobs:
  # 1. Code quality checks
  check:
    name: Code Quality
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      
      - name: Cache cargo
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Check formatting
        run: cargo fmt --all -- --check
      
      - name: Run clippy
        run: cargo clippy --workspace --all-targets --all-features -- -D warnings
      
      - name: Check documentation
        run: cargo doc --workspace --no-deps --document-private-items
  
  # 2. Test
  test:
    name: Test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
      
      - name: Cache cargo
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Run tests
        run: cargo test --workspace --all-features
      
      - name: Run tests (release)
        run: cargo test --workspace --release
  
  # 3. Build (Desktop)
  build-desktop:
    name: Build Desktop
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
      
      - name: Cache cargo
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Build
        run: cargo build --release --workspace
      
      - name: Upload artifact
        uses: actions/upload-artifact@v3
        with:
          name: motor_kontrol-${{ matrix.os }}
          path: target/release/motor_kontrol*
  
  # 4. Build (RP2354B Firmware)
  build-firmware:
    name: Build Firmware (RP2354B)
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: thumbv8m.main-none-eabihf
      
      - name: Install ARM tools
        run: sudo apt-get install -y binutils-arm-none-eabi
      
      - name: Cache cargo
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-firmware-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Build firmware
        run: cargo build --profile embedded --target thumbv8m.main-none-eabihf
      
      - name: Check binary size
        run: |
          cargo size --profile embedded --target thumbv8m.main-none-eabihf
          SIZE=$(stat -c%s target/thumbv8m.main-none-eabihf/embedded/motor_firmware)
          echo "Binary size: $SIZE bytes"
          if [ $SIZE -gt 500000 ]; then
            echo "Binary too large!"
            exit 1
          fi
      
      - name: Create binary file
        run: |
          arm-none-eabi-objcopy -O binary \
            target/thumbv8m.main-none-eabihf/embedded/motor_firmware \
            motor_firmware.bin
      
      - name: Upload firmware
        uses: actions/upload-artifact@v3
        with:
          name: motor_firmware
          path: motor_firmware.bin
  
  # 5. Security audit
  audit:
    name: Security Audit
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install cargo-audit
        run: cargo install cargo-audit
      
      - name: Run audit
        run: cargo audit
  
  # 6. Coverage
  coverage:
    name: Code Coverage
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
      
      - name: Install cargo-tarpaulin
        run: cargo install cargo-tarpaulin
      
      - name: Run coverage
        run: cargo tarpaulin --out Xml
      
      - name: Upload to codecov
        uses: codecov/codecov-action@v3
        with:
          file: cobertura.xml
```

## 10.2 Release Workflow

```yaml
# .github/workflows/release.yml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    name: Release
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: thumbv8m.main-none-eabihf
      
      - name: Build all targets
        run: |
          cargo build --release
          cargo build --profile embedded --target thumbv8m.main-none-eabihf
      
      - name: Create release artifacts
        run: |
          mkdir -p release
          cp target/release/motor_kontrol release/motor_kontrol-linux
          cp target/thumbv8m.main-none-eabihf/embedded/motor_firmware release/
          arm-none-eabi-objcopy -O binary release/motor_firmware release/motor_firmware.bin
      
      - name: Create GitHub Release
        uses: softprops/action-gh-release@v1
        with:
          files: |
            release/*
          body: |
            ## Changes
            See [CHANGELOG.md](CHANGELOG.md)
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
      
      - name: Publish to crates.io
        run: cargo publish --token ${{ secrets.CARGO_REGISTRY_TOKEN }}
```

---

# 📚 BÖLÜM 11: Troubleshooting ve Debugging ⭐⭐

## 11.1 Yaygın Derleme Hataları

### Hata 1: Linker Hatası

```
error: linking with `cc` failed: exit status: 1
```

**Çözüm**:
```bash
# Linux
sudo apt install build-essential

# macOS
xcode-select --install

# Windows
# Visual Studio Build Tools kur
```

### Hata 2: Target Bulunamadı

```
error[E0463]: can't find crate for `std`
```

**Çözüm**:
```bash
# Target yükle
rustup target add thumbv8m.main-none-eabihf

# Veya no_std kullan
#![no_std]
```

### Hata 3: Out of Memory

```
error: linking with `cc` failed: signal: 9 killed
```

**Çözüm**:
```toml
# codegen-units azalt
[profile.release]
codegen-units = 1

# LTO kapat
lto = false

# Swap ekle (Linux)
sudo fallocate -l 4G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile
```

### Hata 4: Dependency Çakışması

```
error: failed to select a version for `serde`
```

**Çözüm**:
```bash
# Dependency tree'i kontrol et
cargo tree --duplicates

# [patch] ile düzelt
[patch.crates-io]
serde = { git = "https://github.com/serde-rs/serde" }
```

## 11.2 Verbose Derleme

```bash
# Detaylı derleme çıktısı
cargo build -v

# Çok detaylı
cargo build -vv

# Zamanlama bilgisi
cargo build --timings
# Çıktı: target/cargo-timings/index.html (görsel rapor)
```

## 11.3 Derleme Hatası Debugging

```bash
# Hata mesajlarını renkli ve detaylı göster
cargo build --color=always --message-format=json

# Backtrace
RUST_BACKTRACE=1 cargo run

# LLVM IR gör
cargo rustc -- --emit=llvm-ir

# Assembly gör
cargo rustc -- --emit=asm
```

## 11.4 RP2354B Firmware Debugging

```bash
# probe-rs ile debug
probe-rs gdb --chip RP235x

# defmt loglarını görüntüle
probe-rs run --chip RP235x target/thumbv8m.main-none-eabihf/release/motor_firmware

# RTT logları
cargo run --release
# defmt logları otomatik görüntülenir
```

---

# 📚 BÖLÜM 12: İleri Düzey Konular ⭐⭐

## 12.1 Custom Target Specification

Kendi target tanımınızı oluşturabilirsiniz:

```bash
# Mevcut target'ı dışa aktar
rustc +nightly -Z unstable-options --print target-spec-json --target thumbv8m.main-none-eabihf > custom-target.json

# Düzenle
# custom-target.json'da değişiklikler yap

# Custom target ile derle
cargo build --target custom-target.json
```

## 12.2 Compiler Plugins ve Custom Lints

```toml
# Cargo.toml
[dependencies]
clippy = { version = "*", optional = true }
```

```rust
// src/lib.rs
#![feature(plugin)]
#![plugin(clippy)]

#![warn(clippy::all)]
#![allow(clippy::too_many_arguments)]
```

## 12.3 Sanitizers

```bash
# Address sanitizer
RUSTFLAGS="-Z sanitizer=address" cargo build --target x86_64-unknown-linux-gnu

# Memory sanitizer
RUSTFLAGS="-Z sanitizer=memory" cargo build --target x86_64-unknown-linux-gnu

# Thread sanitizer
RUSTFLAGS="-Z sanitizer=thread" cargo build --target x86_64-unknown-linux-gnu
```

## 12.4 Profile-Guided Optimization (PGO)

```bash
# 1. Instrumented build
cargo build --release
RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data" cargo build --release

# 2. Run with representative workload
./target/release/motor_kontrol --benchmark

# 3. Merge profile data
llvm-profdata merge -o /tmp/pgo-data/merged.profdata /tmp/pgo-data

# 4. Optimized build
RUSTFLAGS="-Cprofile-use=/tmp/pgo-data/merged.profdata" cargo build --release
```

---

# 🎯 ÖZET: Compilation and Building Kontrol Listesi

| Özellik | C# | Rust |
|---|---|---|
| Build system | MSBuild | Cargo |
| Compiler | Roslyn | rustc (LLVM) |
| Compilation type | AOT + JIT | Sadece AOT |
| Output | IL (.dll/.exe) | Native binary |
| Runtime | .NET Runtime | Yok |
| Cross-compile | ❌ Zor | ✅ Kolay |
| Embedded | ❌ | ✅ (no_std) |
| LTO | ❌ | ✅ |
| Incremental | ✅ | ✅ |
| Build scripts | MSBuild targets | build.rs |
| Package manager | NuGet | crates.io |
| Dependency lock | packages.lock.json | Cargo.lock |
| Profiles | Debug/Release | dev/release/test/bench + custom |
| Binary size optimization | ⚠️ Sınırlı | ✅ Çok iyi |
| CI/CD | Azure DevOps, GitHub Actions | GitHub Actions, GitLab CI |

---

# 🚀 Son Tavsiyeler

## 1. Geliştirme Süreci

```bash
# Hızlı kontrol
cargo check

# Test
cargo test

# Format
cargo fmt

# Lint
cargo clippy

# Run
cargo run
```

## 2. Release Süreci

```bash
# Release build
cargo build --release

# Binary boyutunu kontrol et
cargo size --release

# Test
cargo test --release

# Benchmark
cargo bench
```

## 3. Embedded (RP2354B) Süreci

```bash
# Firmware derle
cargo build --profile embedded --target thumbv8m.main-none-eabihf

# Boyut kontrolü
cargo size --profile embedded --target thumbv8m.main-none-eabihf

# Binary oluştur
arm-none-eabi-objcopy -O binary \
  target/thumbv8m.main-none-eabihf/embedded/motor_firmware \
  motor_firmware.bin

# Flash et
probe-rs flash download --chip RP235x motor_firmware.bin

# Veya tek komutla
cargo run --profile embedded
```

## 4. Optimizasyon Stratejisi

1. **Önce doğru kodu yaz** (dev profile)
2. **Test et** (cargo test)
3. **Benchmark yap** (cargo bench)
4. **Profile et** (cargo build --timings, cargo bloat)
5. **Optimize et** (release profile)
6. **Tekrar test et** (cargo test --release)
7. **Tekrar benchmark** (cargo bench)

## 5. RP2354B Projeniz İçin Önerilen Yapı

```toml
# Cargo.toml
[profile.dev]
opt-level = 0
debug = true
incremental = true

[profile.release]
opt-level = 3
lto = "thin"
codegen-units = 16

[profile.embedded]
inherits = "release"
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
strip = true
debug = false

[profile.size-optimized]
inherits = "embedded"
opt-level = "z"
```

## 6. CI/CD Pipeline

```yaml
# .github/workflows/ci.yml
jobs:
  test:
    - cargo fmt --check
    - cargo clippy
    - cargo test
  
  build-desktop:
    - cargo build --release
  
  build-firmware:
    - cargo build --profile embedded --target thumbv8m.main-none-eabihf
    - cargo size --profile embedded
    - arm-none-eabi-objcopy -O binary ...
  
  audit:
    - cargo audit
  
  release:
    - cargo publish
    - GitHub release oluştur
```

## 7. Performans İpuçları

1. **Incremental compilation kullan**: `cargo check` ile hızlı geri bildirim
2. **Parallel compilation**: `codegen-units` yüksek tut (dev)
3. **LTO kullan**: Release ve embedded için
4. **Strip symbols**: Binary boyutunu küçült
5. **panic = "abort"**: Embedded için daha küçük binary
6. **opt-level = "z"**: Boyut kritikse

## 8. Debugging İpuçları

```bash
# Verbose build
cargo build -vv

# Timing information
cargo build --timings

# Dependency tree
cargo tree

# Binary analysis
cargo bloat --release

# Symbol analysis
arm-none-eabi-nm --size-sort target/release/motor_firmware
```

## 9. Embedded Best Practices

1. **no_std kullan**: `#![no_std]`
2. **Panic handler tanımla**: `panic-probe` veya custom
3. **Memory layout doğru tanımla**: `memory.x`
4. **Linker script kullan**: `link.x`
5. **defmt kullan**: Sıfır maliyetli loglama
6. **probe-rs kullan**: Flash ve debug için
7. **Boyutu sürekli izle**: CI'da boyut kontrolü

## 10. Yaygın Hatalar ve Çözümleri

| Hata | Sebep | Çözüm |
|---|---|---|
| `linker not found` | Toolchain eksik | `build-essential` veya `gcc` kur |
| `can't find crate` | Target eksik | `rustup target add ...` |
| `out of memory` | LTO + çok codegen unit | `codegen-units = 1` veya swap ekle |
| `dependency conflict` | Versiyon çakışması | `[patch]` kullan |
| `binary too large` | Optimizasyon yok | `opt-level = "z"`, `lto = true`, `strip = true` |
| `firmware won't boot` | Yanlış memory layout | `memory.x` kontrol et |
| `hard fault` | Stack overflow | Stack boyutunu artır |

---

> 🦀 **Unutmayın:**
> - **Rust'ın derleme sistemi**, C#'tan **çok daha güçlü ve esnektir**
> - **AOT compilation** sayesinde JIT overhead yoktur, embedded sistemlerde çalışabilir
> - **LLVM backend** sayesinde mükemmel optimizasyonlar yapılır
> - **LTO** ile tüm crate'ler birlikte optimize edilir
> - **Cross-compilation** çok kolaydır - tek komutla farklı platformlar için derleme
> - **Cargo** hem build system hem package manager'dır
> - **RP2354B projenizde** custom profiller kullanarak hem hızlı geliştirme hem de optimize firmware elde edebilirsiniz
> - **Binary boyutu** embedded sistemlerde kritik önem taşır - `opt-level = "z"`, `lto = true`, `strip = true` kullanın
> - **CI/CD** ile her commit'te otomatik test, build ve deployment yapın
> 
> Rust'ın derleme sistemi, başta karmaşık gelse de, bir kez anladığınızda C#'ta asla ulaşamayacağınız **kontrol ve optimizasyon** sağlar. Özellikle embedded sistemlerde, bu kontrol **hayati önem** taşır. RP2354B projenizde custom profiller, cross-compilation ve binary boyutu optimizasyonları ile mükemmel bir firmware geliştirebilirsiniz! 🚀
```