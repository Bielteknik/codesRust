# ⚙️ Rust for .NET Developers: Environment and Configuration (Ortam ve Yapılandırma)

Bu bölüm, Rust'ta ortam değişkenleri, konfigürasyon dosyaları, komut satırı argümanları ve secrets yönetiminin temellerinden ileri düzey tekniklere kadar kapsamlı bir rehberdir. C# dünyasında `IConfiguration`, `appsettings.json`, User Secrets ve Dependency Injection tabanlı konfigürasyon sistemlerine alışkınız. Rust ise **modüler, crate-tabanlı** ve özellikle **embedded sistemler için optimize edilmiş** bir yaklaşım sunar.

> 🎯 **Temel Fark:** C#'ta konfigürasyon **framework'e gömülüdür** (ASP.NET Core IConfiguration). Rust'ta ise konfigürasyon **crate ekosistemi** üzerinden çalışır - `std::env` (temel), `config` (dosya), `clap` (CLI), `figment` (birleştirici) gibi araçlar kullanılır. Embedded tarafta ise **compile-time constants** ve **flash storage** ile çalışılır.

---

# 📚 BÖLÜM 1: Configuration Nedir ve Neden Önemlidir?

## 1.1 Tanım

**Configuration (Yapılandırma)**: Uygulamanın davranışını belirleyen, çalışma zamanında değiştirilebilen ayarlar bütünü.

## 1.2 Kullanım Alanları

1. **Ortam değişkenleri**: Database connection strings, API keys
2. **Konfigürasyon dosyaları**: appsettings.json benzeri yapılandırma
3. **Komut satırı argümanları**: CLI uygulamaları için
4. **Secrets yönetimi**: Hassas bilgilerin güvenle saklanması
5. **Feature flags**: Özellik açma/kapama
6. **Embedded settings**: Donanım parametreleri, kalibrasyon değerleri

## 1.3 C# vs Rust Yaklaşımı

| Özellik | C# | Rust |
|---|---|---|
| Yerleşik çözüm | `IConfiguration` (ASP.NET Core) | Yok (crate gerekir) |
| Konfigürasyon dosyası | `appsettings.json` | TOML, JSON, YAML (crate ile) |
| Environment variables | `Environment.GetEnvironmentVariable` | `std::env::var` |
| CLI parsing | `System.CommandLine` | `clap` (en popüler) |
| Strongly-typed config | `IOptions<T>` | `serde` + `config` crate |
| Secrets | User Secrets, Azure Key Vault | `secrecy` crate, OS keychain |
| DI entegrasyonu | ✅ (yerleşik) | ⚠️ (manuel veya crate ile) |
| Embedded desteği | ❌ | ✅ (const/static, flash storage) |
| Hot reload | ✅ (IOptionsSnapshot) | ⚠️ (file watch ile) |
| Validation | DataAnnotations | `validator` crate |

---

# 📚 BÖLÜM 2: Environment Variables (Ortam Değişkenleri)

## 2.1 C# Yaklaşımı

```csharp
// Okuma
var connectionString = Environment.GetEnvironmentVariable("DATABASE_URL");
var port = Environment.GetEnvironmentVariable("PORT") ?? "8080";

// Yazma
Environment.SetEnvironmentVariable("MY_VAR", "value");

// Tüm değişkenleri listele
foreach (DictionaryEntry entry in Environment.GetEnvironmentVariables())
{
    Console.WriteLine($"{entry.Key} = {entry.Value}");
}
```

## 2.2 Rust Yaklaşımı - `std::env`

```rust
use std::env;

fn main() {
    // Okuma
    let db_url = env::var("DATABASE_URL");
    match db_url {
        Ok(url) => println!("Database: {}", url),
        Err(env::VarError::NotPresent) => println!("DATABASE_URL ayarlanmamış"),
        Err(env::VarError::NotUnicode(s)) => println!("Geçersiz Unicode: {:?}", s),
    }
    
    // Varsayılan değer ile
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT geçerli bir sayı olmalı");
    
    // Yazma (sadece process içinde geçerli)
    env::set_var("MY_VAR", "value");
    
    // Tüm değişkenleri listele
    for (key, value) in env::vars() {
        println!("{} = {}", key, value);
    }
}
```

## 2.3 Güvenli Environment Variable Okuma

```rust
use std::env;

struct AppConfig {
    database_url: String,
    port: u16,
    debug: bool,
}

impl AppConfig {
    fn from_env() -> Result<Self, String> {
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| "DATABASE_URL ortam değişkeni eksik")?;
        
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .map_err(|_| "PORT geçerli bir sayı olmalı")?;
        
        let debug = env::var("DEBUG")
            .map(|v| v == "1" || v.to_lowercase() == "true")
            .unwrap_or(false);
        
        Ok(AppConfig {
            database_url,
            port,
            debug,
        })
    }
}

fn main() {
    match AppConfig::from_env() {
        Ok(config) => {
            println!("Database: {}", config.database_url);
            println!("Port: {}", config.port);
            println!("Debug: {}", config.debug);
        }
        Err(e) => {
            eprintln!("Konfigürasyon hatası: {}", e);
            std::process::exit(1);
        }
    }
}
```

## 2.4 dotenv Crate (.env Dosyası Desteği)

**Cargo.toml**:
```toml
[dependencies]
dotenv = "0.15"
```

**.env dosyası**:
```
DATABASE_URL=postgres://localhost/motor_db
PORT=8080
DEBUG=true
LOG_LEVEL=info
```

**Kod**:
```rust
use dotenv::dotenv;
use std::env;

fn main() {
    // .env dosyasını yükle (sadece development için)
    dotenv().ok();
    
    let db_url = env::var("DATABASE_URL").unwrap();
    println!("Database: {}", db_url);
}
```

> 💡 **Kritik Fark:** C#'ta `appsettings.json` dosyası otomatik yüklenir. Rust'ta `.env` dosyası için `dotenv` crate gerekir ve **production'da kullanılmamalıdır** (güvenlik riski).

---

# 📚 BÖLÜM 3: Configuration Files (Konfigürasyon Dosyaları)

## 3.1 C# Yaklaşımı - appsettings.json

```json
{
  "MotorKontrol": {
    "MaxHiz": 1500,
    "Ivme": 100,
    "AdimBasinaMesafe": 0.01
  },
  "Logging": {
    "LogLevel": {
      "Default": "Information"
    }
  }
}
```

```csharp
// Program.cs
builder.Configuration.AddJsonFile("appsettings.json", optional: false, reloadOnChange: true);

// Kullanım
var maxHiz = builder.Configuration.GetValue<int>("MotorKontrol:MaxHiz");
```

## 3.2 Rust Yaklaşımı - TOML (Önerilen)

Rust ekosisteminde **TOML** formatı tercih edilir (Cargo.toml da TOML'dir).

**Cargo.toml**:
```toml
[dependencies]
toml = "0.8"
serde = { version = "1.0", features = ["derive"] }
```

**config.toml**:
```toml
[motor_kontrol]
max_hiz = 1500
ivme = 100
adim_basina_mesafe = 0.01

[logging]
level = "info"
dosya = "motor.log"
```

**Kod**:
```rust
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Config {
    motor_kontrol: MotorKontrolConfig,
    logging: LoggingConfig,
}

#[derive(Debug, Deserialize)]
struct MotorKontrolConfig {
    max_hiz: u16,
    ivme: u16,
    adim_basina_mesafe: f64,
}

#[derive(Debug, Deserialize)]
struct LoggingConfig {
    level: String,
    dosya: String,
}

fn config_yukle() -> Result<Config, Box<dyn std::error::Error>> {
    let icerik = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&icerik)?;
    Ok(config)
}

fn main() {
    match config_yukle() {
        Ok(config) => {
            println!("Max hız: {}", config.motor_kontrol.max_hiz);
            println!("Log seviyesi: {}", config.logging.level);
        }
        Err(e) => {
            eprintln!("Konfigürasyon yüklenemedi: {}", e);
            std::process::exit(1);
        }
    }
}
```

## 3.3 JSON Formatı

**Cargo.toml**:
```toml
[dependencies]
serde_json = "1.0"
serde = { version = "1.0", features = ["derive"] }
```

**config.json**:
```json
{
  "motor_kontrol": {
    "max_hiz": 1500,
    "ivme": 100,
    "adim_basina_mesafe": 0.01
  }
}
```

**Kod**:
```rust
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Config {
    motor_kontrol: MotorKontrolConfig,
}

#[derive(Debug, Deserialize)]
struct MotorKontrolConfig {
    max_hiz: u16,
    ivme: u16,
    adim_basina_mesafe: f64,
}

fn config_yukle() -> Result<Config, Box<dyn std::error::Error>> {
    let icerik = fs::read_to_string("config.json")?;
    let config: Config = serde_json::from_str(&icerik)?;
    Ok(config)
}
```

## 3.4 YAML Formatı

**Cargo.toml**:
```toml
[dependencies]
serde_yaml = "0.9"
serde = { version = "1.0", features = ["derive"] }
```

**config.yaml**:
```yaml
motor_kontrol:
  max_hiz: 1500
  ivme: 100
  adim_basina_mesafe: 0.01
```

**Kod**:
```rust
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Config {
    motor_kontrol: MotorKontrolConfig,
}

#[derive(Debug, Deserialize)]
struct MotorKontrolConfig {
    max_hiz: u16,
    ivme: u16,
    adim_basina_mesafe: f64,
}

fn config_yukle() -> Result<Config, Box<dyn std::error::Error>> {
    let icerik = fs::read_to_string("config.yaml")?;
    let config: Config = serde_yaml::from_str(&icerik)?;
    Ok(config)
}
```

## 3.5 Format Karşılaştırması

| Format | Rust Desteği | Okunabilirlik | Hız | Kullanım |
|---|---|---|---|---|
| TOML | ✅ Mükemmel | ✅ Yüksek | Hızlı | Rust ekosistemi (önerilen) |
| JSON | ✅ Mükemmel | ⚠️ Orta | Hızlı | Web API'leri |
| YAML | ✅ İyi | ✅ Yüksek | Orta | Kubernetes, Docker |
| INI | ⚠️ Sınırlı | ✅ Yüksek | Hızlı | Eski sistemler |

---

# 📚 BÖLÜM 4: `config` Crate - Unified Configuration ⭐

`config` crate, birden fazla kaynaktan (dosya, env, CLI) konfigürasyonu birleştirmek için kullanılır.

## 4.1 Kurulum

**Cargo.toml**:
```toml
[dependencies]
config = "0.14"
serde = { version = "1.0", features = ["derive"] }
```

## 4.2 Temel Kullanım

```rust
use config::{Config, File, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AppConfig {
    database_url: String,
    port: u16,
    debug: bool,
    motor: MotorConfig,
}

#[derive(Debug, Deserialize)]
struct MotorConfig {
    max_hiz: u16,
    ivme: u16,
}

fn config_yukle() -> Result<AppConfig, config::ConfigError> {
    let config = Config::builder()
        // 1. Varsayılan değerler
        .set_default("port", 8080)?
        .set_default("debug", false)?
        
        // 2. config.toml dosyası
        .add_source(File::with_name("config.toml").required(false))
        
        // 3. Environment değişkenleri (MOTOR_MAX_HIZ şeklinde)
        .add_source(
            Environment::with_prefix("APP")
                .separator("__")
                .try_parsing(true)
        )
        
        // 4. .env dosyası (development için)
        .add_source(File::with_name(".env").required(false))
        
        .build()?;
    
    config.try_deserialize()
}

fn main() {
    match config_yukle() {
        Ok(config) => {
            println!("Port: {}", config.port);
            println!("Motor max hız: {}", config.motor.max_hiz);
        }
        Err(e) => {
            eprintln!("Konfigürasyon hatası: {}", e);
            std::process::exit(1);
        }
    }
}
```

## 4.3 Configuration Priority (Öncelik Sırası)

`config` crate, kaynakları **sondan başa** doğru birleştirir:

```
1. Varsayılan değerler (en düşük öncelik)
2. config.toml dosyası
3. .env dosyası
4. Environment variables (en yüksek öncelik)
```

**Örnek**:
```bash
# Environment variable, config.toml'daki değeri override eder
MOTOR__MAX_HIZ=2000 cargo run
```

## 4.4 C# vs config Crate

**C#** (ASP.NET Core):
```csharp
builder.Configuration
    .AddJsonFile("appsettings.json")
    .AddEnvironmentVariables()
    .AddCommandLine(args);
```

**Rust** (config crate):
```rust
Config::builder()
    .add_source(File::with_name("config.toml"))
    .add_source(Environment::with_prefix("APP"))
    .add_source(config::CommandLine::with_args(std::env::args()))
    .build()?;
```

---

# 📚 BÖLÜM 5: Command-Line Arguments (Komut Satırı Argümanları)

## 5.1 C# Yaklaşımı - System.CommandLine

```csharp
using System.CommandLine;

var rootCommand = new RootCommand("Motor kontrol uygulaması");

var hizOption = new Option<int>(
    "--hiz",
    getDefaultValue: () => 1000,
    description: "Motor hızı (RPM)"
);

var ivmeOption = new Option<int>(
    "--ivme",
    getDefaultValue: () => 50,
    description: "İvme değeri"
);

var debugOption = new Option<bool>(
    "--debug",
    getDefaultValue: () => false,
    description: "Debug modu"
);

rootCommand.AddOption(hizOption);
rootCommand.AddOption(ivmeOption);
rootCommand.AddOption(debugOption);

rootCommand.SetHandler((hiz, ivme, debug) => {
    Console.WriteLine($"Hız: {hiz}, İvme: {ivme}, Debug: {debug}");
}, hizOption, ivmeOption, debugOption);

return await rootCommand.InvokeAsync(args);
```

## 5.2 Rust Yaklaşımı - `clap` Crate ⭐

`clap`, Rust'ın **en popüler** CLI parsing crate'idir.

**Cargo.toml**:
```toml
[dependencies]
clap = { version = "4.4", features = ["derive"] }
```

### 5.2.1 Derive API (Önerilen)

```rust
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "motor-kontrol")]
#[command(author = "Ali Yılmaz")]
#[command(version = "1.0")]
#[command(about = "Step motor kontrol uygulaması", long_about = None)]
struct Args {
    /// Motor hızı (RPM)
    #[arg(short, long, default_value_t = 1000)]
    hiz: u16,
    
    /// İvme değeri
    #[arg(short, long, default_value_t = 50)]
    ivme: u16,
    
    /// Debug modu
    #[arg(short, long, default_value_t = false)]
    debug: bool,
    
    /// Hedef pozisyon (mm)
    #[arg(short, long)]
    hedef: Option<f64>,
    
    /// Konfigürasyon dosyası
    #[arg(short, long, default_value = "config.toml")]
    config: String,
    
    /// Verbose loglama
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

fn main() {
    let args = Args::parse();
    
    println!("Motor hızı: {} RPM", args.hiz);
    println!("İvme: {}", args.ivme);
    println!("Debug: {}", args.debug);
    
    if let Some(hedef) = args.hedef {
        println!("Hedef pozisyon: {} mm", hedef);
    }
    
    println!("Verbose seviyesi: {}", args.verbose);
}
```

**Kullanım**:
```bash
# Basit kullanım
cargo run -- --hiz 1500 --ivme 100

# Kısa flag'ler
cargo run -- -h 1500 -i 100 -d

# Verbose seviyeleri
cargo run -- -vvv  # Verbose seviyesi 3

# Help
cargo run -- --help
```

**Otomatik Help Çıktısı**:
```
Step motor kontrol uygulaması

Usage: motor-kontrol [OPTIONS]

Options:
  -h, --hiz <HIZ>            Motor hızı (RPM) [default: 1000]
  -i, --ivme <IVME>          İvme değeri [default: 50]
  -d, --debug                Debug modu
  -H, --hedef <HEDEF>        Hedef pozisyon (mm)
  -c, --config <CONFIG>      Konfigürasyon dosyası [default: config.toml]
  -v, --verbose...           Verbose loglama
      --help                 Print help
      --version              Print version
```

### 5.2.2 Builder API

```rust
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("motor-kontrol")
        .version("1.0")
        .author("Ali Yılmaz")
        .about("Step motor kontrol uygulaması")
        .arg(
            Arg::new("hiz")
                .short('h')
                .long("hiz")
                .value_name("RPM")
                .help("Motor hızı")
                .default_value("1000")
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();
    
    let hiz: u16 = matches.get_one::<String>("hiz")
        .unwrap()
        .parse()
        .unwrap();
    
    let debug = matches.get_flag("debug");
    
    println!("Hız: {}, Debug: {}", hiz, debug);
}
```

### 5.2.3 Subcommands (Alt Komutlar)

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "motor-kontrol")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Motoru hareket ettir
    Hareket {
        /// Hedef pozisyon (mm)
        #[arg(short, long)]
        hedef: f64,
        
        /// Hız (RPM)
        #[arg(short, long, default_value_t = 1000)]
        hiz: u16,
    },
    
    /// Motor durumunu göster
    Durum,
    
    /// Motoru kalibre et
    Kalibre {
        /// Referans noktası
        #[arg(short, long)]
        referans: f64,
    },
    
    /// Acil durdur
    Durdur,
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Hareket { hedef, hiz } => {
            println!("Motor {} mm hedefe {} RPM ile hareket ediyor", hedef, hiz);
        }
        Commands::Durum => {
            println!("Motor durumu: Çalışıyor");
        }
        Commands::Kalibre { referans } => {
            println!("Motor {} mm referans noktasına göre kalibre ediliyor", referans);
        }
        Commands::Durdur => {
            println!("Motor ACİL DURDURULUYOR!");
        }
    }
}
```

**Kullanım**:
```bash
cargo run -- hareket --hedef 100.5 --hiz 1500
cargo run -- durum
cargo run -- kalibre --referans 0.0
cargo run -- durdur
```

## 5.3 clap vs System.CommandLine

| Özellik | C# (System.CommandLine) | Rust (clap) |
|---|---|---|
| API stili | Fluent API | Derive macro (önerilen) / Builder |
| Auto help | ✅ | ✅ (daha güzel) |
| Auto completion | ✅ | ✅ (bash, zsh, fish, powershell) |
| Subcommands | ✅ | ✅ |
| Validation | ✅ | ✅ |
| Type safety | ⚠️ Runtime | ✅ Compile-time |
| Performance | Orta | Hızlı |

---

# 📚 BÖLÜM 6: Figment Crate - Layered Configuration ⭐⭐

`figment`, birden fazla konfigürasyon kaynağını **zarif bir şekilde** birleştirmek için kullanılır.

## 6.1 Kurulum

**Cargo.toml**:
```toml
[dependencies]
figment = { version = "0.10", features = ["toml", "env", "json"] }
serde = { version = "1.0", features = ["derive"] }
```

## 6.2 Temel Kullanım

```rust
use figment::{Figment, providers::{Toml, Env, Format, Json}};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    port: u16,
    debug: bool,
    motor: MotorConfig,
}

#[derive(Debug, Deserialize)]
struct MotorConfig {
    max_hiz: u16,
    ivme: u16,
}

fn main() {
    let config: Config = Figment::new()
        // 1. Varsayılan değerler
        .merge(("port", 8080))
        .merge(("debug", false))
        
        // 2. config.toml dosyası
        .merge(Toml::file("config.toml").nested())
        
        // 3. config.json (override)
        .merge(Json::file("config.json").nested())
        
        // 4. Environment variables (MOTOR__MAX_HIZ şeklinde)
        .merge(Env::prefixed("APP_").split("__"))
        
        .extract()
        .unwrap();
    
    println!("Port: {}", config.port);
    println!("Motor max hız: {}", config.motor.max_hiz);
}
```

## 6.3 Profile Support (Ortam Bazlı Konfigürasyon)

```rust
use figment::{Figment, providers::{Toml, Env, Format}};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    port: u16,
    database_url: String,
}

fn main() {
    // APP_ENV=production cargo run
    let env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
    
    let config: Config = Figment::new()
        // Varsayılan config
        .merge(Toml::file("config.toml").nested())
        
        // Environment-specific config
        .merge(Toml::file(format!("config.{}.toml", env)).nested())
        
        // Environment variables (en yüksek öncelik)
        .merge(Env::prefixed("APP_").split("__"))
        
        .extract()
        .unwrap();
    
    println!("Port: {}", config.port);
    println!("Database: {}", config.database_url);
}
```

**Dosya yapısı**:
```
config.toml              # Varsayılan değerler
config.development.toml  # Development ortamı
config.production.toml   # Production ortamı
config.staging.toml      # Staging ortamı
```

## 6.4 Validation

```rust
use figment::{Figment, providers::Toml};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    port: u16,
    motor: MotorConfig,
}

#[derive(Debug, Deserialize)]
struct MotorConfig {
    max_hiz: u16,
    ivme: u16,
}

impl Config {
    fn validate(&self) -> Result<(), String> {
        if self.port == 0 {
            return Err("Port 0 olamaz".to_string());
        }
        
        if self.motor.max_hiz > 3000 {
            return Err("Motor max hız 3000 RPM'yi aşamaz".to_string());
        }
        
        if self.motor.ivme == 0 {
            return Err("İvme 0 olamaz".to_string());
        }
        
        Ok(())
    }
}

fn main() {
    let config: Config = Figment::new()
        .merge(Toml::file("config.toml"))
        .extract()
        .expect("Konfigürasyon yüklenemedi");
    
    if let Err(e) = config.validate() {
        eprintln!("Konfigürasyon validasyon hatası: {}", e);
        std::process::exit(1);
    }
    
    println!("Konfigürasyon geçerli: {:?}", config);
}
```

---

# 📚 BÖLÜM 7: Secrets Management (Gizli Bilgi Yönetimi)

## 7.1 C# Yaklaşımı - User Secrets

```bash
# .NET CLI ile
dotnet user-secrets init
dotnet user-secrets set "Database:Password" "super_secret"
dotnet user-secrets set "ApiKey" "abc123"
```

```csharp
// appsettings.Development.json
{
  "Logging": {
    "LogLevel": {
      "Default": "Debug"
    }
  }
}

// Kullanım
var password = builder.Configuration["Database:Password"];
```

## 7.2 Rust Yaklaşımı - `.env` + `.gitignore`

**.env dosyası**:
```
DATABASE_PASSWORD=super_secret
API_KEY=abc123
JWT_SECRET=xyz789
```

**.gitignore**:
```
.env
.env.*
!.env.example
```

**.env.example** (repository'ye commit edilir):
```
DATABASE_PASSWORD=
API_KEY=
JWT_SECRET=
```

**Kod**:
```rust
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    
    let db_password = env::var("DATABASE_PASSWORD")
        .expect("DATABASE_PASSWORD环境变量必须设置");
    
    let api_key = env::var("API_KEY")
        .expect("API_KEY环境变量必须设置");
    
    println!("Database password: {}", db_password);
}
```

## 7.3 `secrecy` Crate - Güvenli Secret İşleme

**Cargo.toml**:
```toml
[dependencies]
secrecy = { version = "0.8", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] }
```

**Kod**:
```rust
use secrecy::{Secret, ExposeSecret, SecretString};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    database_url: SecretString,
    api_key: Secret<String>,
    port: u16,
}

fn main() {
    let config = Config {
        database_url: SecretString::from("postgres://user:password@localhost/db".to_string()),
        api_key: Secret::new("abc123".to_string()),
        port: 8080,
    };
    
    // Normal Debug çıktısı secret'ları gizler
    println!("Config: {:?}", config);
    // Çıktı: Config { database_url: [REDACTED], api_key: [REDACTED], port: 8080 }
    
    // Secret'ı açıkça ortaya çıkar
    println!("API Key: {}", config.api_key.expose_secret());
}
```

> 💡 **Avantaj:** `secrecy`, secret'ların yanlışlıkla loglanmasını önler. `Debug` trait'i otomatik olarak `[REDACTED]` gösterir.

## 7.4 OS Keychain Entegrasyonu

**Cargo.toml**:
```toml
[dependencies]
keyring = "2.3"
```

**Kod**:
```rust
use keyring::Entry;

fn main() -> Result<(), keyring::Error> {
    let entry = Entry::new("motor-kontrol", "database_password")?;
    
    // Secret kaydet
    entry.set_password("super_secret")?;
    
    // Secret oku
    let password = entry.get_password()?;
    println!("Password: {}", password);
    
    // Secret sil
    entry.delete_credential()?;
    
    Ok(())
}
```

## 7.5 Vault/Cloud Secrets (AWS Secrets Manager, Azure Key Vault)

**Cargo.toml**:
```toml
[dependencies]
aws-config = "1.1"
aws-sdk-secretsmanager = "1.15"
tokio = { version = "1.0", features = ["full"] }
```

**Kod**:
```rust
use aws_config::BehaviorVersion;
use aws_sdk_secretsmanager::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&config);
    
    let secret_name = "motor-kontrol/production/database";
    
    let resp = client
        .get_secret_value()
        .secret_id(secret_name)
        .send()
        .await?;
    
    let secret_string = resp.secret_string().unwrap();
    println!("Secret: {}", secret_string);
    
    Ok(())
}
```

---

# 📚 BÖLÜM 8: Configuration Validation (Doğrulama)

## 8.1 Manuel Validation

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    port: u16,
    motor: MotorConfig,
}

#[derive(Debug, Deserialize)]
struct MotorConfig {
    max_hiz: u16,
    ivme: u16,
}

impl Config {
    fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        if self.port == 0 {
            errors.push("Port 0 olamaz".to_string());
        }
        
        if self.port > 65535 {
            errors.push("Port 65535'ten büyük olamaz".to_string());
        }
        
        if self.motor.max_hiz > 3000 {
            errors.push("Motor max hız 3000 RPM'yi aşamaz".to_string());
        }
        
        if self.motor.ivme == 0 {
            errors.push("İvme 0 olamaz".to_string());
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

fn main() {
    let config = load_config().unwrap();
    
    match config.validate() {
        Ok(_) => println!("Konfigürasyon geçerli"),
        Err(errors) => {
            eprintln!("Konfigürasyon hataları:");
            for error in errors {
                eprintln!("  - {}", error);
            }
            std::process::exit(1);
        }
    }
}
```

## 8.2 `validator` Crate

**Cargo.toml**:
```toml
[dependencies]
validator = { version = "0.18", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
```

**Kod**:
```rust
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
struct Config {
    #[validate(range(min = 1, max = 65535))]
    port: u16,
    
    #[validate]
    motor: MotorConfig,
    
    #[validate(email)]
    admin_email: String,
    
    #[validate(length(min = 10, max = 100))]
    api_key: String,
}

#[derive(Debug, Deserialize, Validate)]
struct MotorConfig {
    #[validate(range(min = 1, max = 3000))]
    max_hiz: u16,
    
    #[validate(range(min = 1, max = 500))]
    ivme: u16,
    
    #[validate(range(min = 0.001, max = 10.0))]
    adim_basina_mesafe: f64,
}

fn main() {
    let config = load_config().unwrap();
    
    match config.validate() {
        Ok(_) => println!("Konfigürasyon geçerli"),
        Err(e) => {
            eprintln!("Validasyon hatası: {}", e);
            std::process::exit(1);
        }
    }
}
```

## 8.3 Custom Validation

```rust
use validator::{Validate, ValidationErrors};
use serde::Deserialize;

#[derive(Debug, Deserialize, Validate)]
struct Config {
    #[validate(range(min = 1, max = 65535))]
    port: u16,
    
    #[validate(custom(function = "validate_motor_config"))]
    motor: MotorConfig,
}

#[derive(Debug, Deserialize)]
struct MotorConfig {
    max_hiz: u16,
    ivme: u16,
}

fn validate_motor_config(motor: &MotorConfig) -> Result<(), validator::ValidationError> {
    // İvme, max hızın %20'sinden fazla olamaz
    if motor.ivme > motor.max_hiz / 5 {
        return Err(validator::ValidationError::new("ivme_too_high")
            .with_message("İvme, max hızın %20'sinden fazla olamaz".into()));
    }
    
    Ok(())
}
```

---

# 📚 BÖLÜM 9: Hot Reload (Canlı Yeniden Yükleme)

## 9.1 File Watch ile Hot Reload

**Cargo.toml**:
```toml
[dependencies]
notify = "6.1"
toml = "0.8"
serde = { version = "1.0", features = ["derive"] }
```

**Kod**:
```rust
use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc;
use std::time::Duration;
use std::fs;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
struct Config {
    port: u16,
    debug: bool,
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = mpsc::channel();
    
    let mut watcher = watcher(tx, Duration::from_secs(2))?;
    watcher.watch("config.toml", RecursiveMode::NonRecursive)?;
    
    let mut current_config = load_config()?;
    println!("Initial config: {:?}", current_config);
    
    loop {
        match rx.recv() {
            Ok(_) => {
                match load_config() {
                    Ok(new_config) => {
                        println!("Config reloaded: {:?}", new_config);
                        current_config = new_config;
                        // Burada config'i kullanan component'leri güncelle
                    }
                    Err(e) => {
                        eprintln!("Config reload failed: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Watch error: {}", e);
                break;
            }
        }
    }
    
    Ok(())
}
```

## 9.2 Arc<Mutex<T>> ile Thread-Safe Config

```rust
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
struct Config {
    port: u16,
    debug: bool,
}

fn main() {
    let config = Arc::new(Mutex::new(Config {
        port: 8080,
        debug: false,
    }));
    
    // Config'i kullanan thread
    let config_clone = Arc::clone(&config);
    thread::spawn(move || {
        loop {
            let cfg = config_clone.lock().unwrap();
            println!("Thread: Port = {}, Debug = {}", cfg.port, cfg.debug);
            thread::sleep(Duration::from_secs(2));
        }
    });
    
    // Config'i güncelleyen thread
    let config_clone = Arc::clone(&config);
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(5));
        let mut cfg = config_clone.lock().unwrap();
        cfg.port = 9090;
        cfg.debug = true;
        println!("Config updated!");
    });
    
    thread::sleep(Duration::from_secs(10));
}
```

---

# 📚 BÖLÜM 10: Embedded Systems Configuration (RP2354B için) 🎯

**Bu bölüm, RP2354B step motor projeniz için KRİTİK ÖNEM taşır!**

## 10.1 Embedded Konfigürasyon Zorlukları

- ❌ Dosya sistemi yok (veya sınırlı)
- ❌ Environment variables yok
- ❌ Heap allocation kısıtlı
- ❌ Real-time gereksinimler
- ❌ Power-off durumunda kalıcılık gerekli

## 10.2 Compile-Time Constants

```rust
// src/config.rs
pub const MOTOR_MAX_HIZ: u16 = 1500;
pub const MOTOR_IVME: u16 = 100;
pub const ADIM_BASINA_MESAFE: f64 = 0.01;  // mm/step
pub const MAX_POZISYON: i32 = 100_000;  // steps

pub const UART_BAUD_RATE: u32 = 115200;
pub const WATCHDOG_TIMEOUT_MS: u32 = 5000;

// Feature-based config
#[cfg(feature = "high_precision")]
pub const ADIM_BASINA_MESAFE: f64 = 0.005;  // Daha hassas

#[cfg(feature = "fast_mode")]
pub const MOTOR_MAX_HIZ: u16 = 3000;
```

```rust
// src/main.rs
mod config;

fn main() {
    println!("Max hız: {} RPM", config::MOTOR_MAX_HIZ);
    println!("İvme: {}", config::MOTOR_IVME);
}
```

## 10.3 Flash Storage ile Runtime Configuration

RP2354B'de **flash memory**'nin bir bölümünü konfigürasyon için kullanabilirsiniz.

**Cargo.toml**:
```toml
[dependencies]
rp235x-hal = "0.1"
embedded-storage = "0.3"
serde = { version = "1.0", default-features = false, features = ["derive"] }
postcard = { version = "1.0", features = ["alloc"] }  # no_std serde
```

**Kod**:
```rust
#![no_std]
#![no_main]

use rp235x_hal as hal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct MotorConfig {
    max_hiz: u16,
    ivme: u16,
    adim_basina_mesafe_mikrometre: u32,  // µm/step (float yerine integer)
    max_pozisyon: i32,
}

impl Default for MotorConfig {
    fn default() -> Self {
        Self {
            max_hiz: 1500,
            ivme: 100,
            adim_basina_mesafe_mikrometre: 10,  // 10 µm = 0.01 mm
            max_pozisyon: 100_000,
        }
    }
}

// Flash storage için config alanı
const CONFIG_FLASH_OFFSET: u32 = 0x100000;  // 1 MB offset
const CONFIG_MAX_SIZE: usize = 4096;  // 4 KB

fn config_oku() -> MotorConfig {
    // Flash'tan config oku
    // Eğer geçersizse default döndür
    unsafe {
        let flash_ptr = (0x10000000 + CONFIG_FLASH_OFFSET) as *const u8;
        let slice = core::slice::from_raw_parts(flash_ptr, CONFIG_MAX_SIZE);
        
        match postcard::from_bytes::<MotorConfig>(slice) {
            Ok(config) => config,
            Err(_) => MotorConfig::default(),
        }
    }
}

fn config_yaz(config: &MotorConfig) -> Result<(), ()> {
    let mut buffer = [0u8; CONFIG_MAX_SIZE];
    
    match postcard::to_slice(config, &mut buffer) {
        Ok(serialized) => {
            // Flash'a yaz
            // rp235x_hal flash write fonksiyonu kullan
            Ok(())
        }
        Err(_) => Err(()),
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let config = config_oku();
    
    // Config'i kullan
    defmt::info!("Motor config: {:?}", config);
    
    loop {
        cortex_m::asm::wfi();
    }
}
```

## 10.4 EEPROM/FRAM ile Kalıcı Konfigürasyon

Daha sık güncellenen config'ler için **EEPROM** veya **FRAM** kullanabilirsiniz.

```rust
use embedded_hal::i2c::I2c;

const EEPROM_ADDRESS: u8 = 0x50;
const CONFIG_EEPROM_OFFSET: u16 = 0;

fn config_oku_eeprom<I: I2c>(i2c: &mut I) -> MotorConfig {
    let mut buffer = [0u8; 64];
    
    // EEPROM'dan oku
    let mut tx = [
        (CONFIG_EEPROM_OFFSET >> 8) as u8,
        (CONFIG_EEPROM_OFFSET & 0xFF) as u8,
    ];
    
    if i2c.write_read(EEPROM_ADDRESS, &tx, &mut buffer).is_ok() {
        postcard::from_bytes(&buffer).unwrap_or_default()
    } else {
        MotorConfig::default()
    }
}

fn config_yaz_eeprom<I: I2c>(i2c: &mut I, config: &MotorConfig) -> Result<(), ()> {
    let mut buffer = [0u8; 64];
    let serialized = postcard::to_slice(config, &mut buffer).map_err(|_| ())?;
    
    // EEPROM'a yaz
    let mut tx_data = [0u8; 66];
    tx_data[0] = (CONFIG_EEPROM_OFFSET >> 8) as u8;
    tx_data[1] = (CONFIG_EEPROM_OFFSET & 0xFF) as u8;
    tx_data[2..2 + serialized.len()].copy_from_slice(serialized);
    
    i2c.write(EEPROM_ADDRESS, &tx_data).map_err(|_| ())?;
    
    Ok(())
}
```

## 10.5 UART/Serial ile Konfigürasyon Değiştirme

```rust
use defmt::{info, error};

enum Komut {
    SetMaxHiz(u16),
    SetIvme(u16),
    GetConfig,
    SaveConfig,
    ResetConfig,
}

fn parse_komut(input: &str) -> Option<Komut> {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    
    match parts.as_slice() {
        ["SET_MAX_HIZ", deger] => deger.parse().ok().map(Komut::SetMaxHiz),
        ["SET_IVME", deger] => deger.parse().ok().map(Komut::SetIvme),
        ["GET_CONFIG"] => Some(Komut::GetConfig),
        ["SAVE_CONFIG"] => Some(Komut::SaveConfig),
        ["RESET_CONFIG"] => Some(Komut::ResetConfig),
        _ => None,
    }
}

fn komut_isle(komut: Komut, config: &mut MotorConfig) {
    match komut {
        Komut::SetMaxHiz(hiz) => {
            if hiz <= 3000 {
                config.max_hiz = hiz;
                info!("Max hız {} olarak ayarlandı", hiz);
            } else {
                error!("Max hız 3000'ü aşamaz");
            }
        }
        Komut::SetIvme(ivme) => {
            if ivme <= 500 {
                config.ivme = ivme;
                info!("İvme {} olarak ayarlandı", ivme);
            } else {
                error!("İvme 500'ü aşamaz");
            }
        }
        Komut::GetConfig => {
            info!("Config: max_hiz={}, ivme={}", config.max_hiz, config.ivme);
        }
        Komut::SaveConfig => {
            // Flash'a kaydet
            info!("Config kaydedildi");
        }
        Komut::ResetConfig => {
            *config = MotorConfig::default();
            info!("Config sıfırlandı");
        }
    }
}
```

## 10.6 Step Motor Projesi İçin Konfigürasyon Stratejisi

```rust
// src/config.rs
#![no_std]

use serde::{Deserialize, Serialize};

/// Motor konfigürasyonu - flash'ta saklanır
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotorConfig {
    /// Maksimum hız (RPM)
    pub max_hiz_rpm: u16,
    
    /// İvme (step/s²)
    pub ivme: u16,
    
    /// Bir adımın mesafesi (µm cinsinden)
    pub adim_basina_mesafe_um: u32,
    
    /// Maksimum pozisyon (step cinsinden)
    pub max_pozisyon_step: i32,
    
    /// Minimum pozisyon (step cinsinden)
    pub min_pozisyon_step: i32,
    
    /// Home pozisyonu (step cinsinden)
    pub home_pozisyon: i32,
    
    /// Motor akım limiti (mA)
    pub akim_limiti_ma: u16,
    
    /// Microstep modu (1, 2, 4, 8, 16, 32)
    pub microstep: u8,
}

impl Default for MotorConfig {
    fn default() -> Self {
        Self {
            max_hiz_rpm: 1500,
            ivme: 100,
            adim_basina_mesafe_um: 10,  // 10 µm = 0.01 mm
            max_pozisyon_step: 100_000,
            min_pozisyon_step: -100_000,
            home_pozisyon: 0,
            akim_limiti_ma: 2000,
            microstep: 16,
        }
    }
}

impl MotorConfig {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.max_hiz_rpm > 3000 {
            return Err("Max hız 3000 RPM'yi aşamaz");
        }
        
        if self.ivme == 0 || self.ivme > 500 {
            return Err("İvme 1-500 arasında olmalı");
        }
        
        if self.adim_basina_mesafe_um == 0 {
            return Err("Adım başına mesafe 0 olamaz");
        }
        
        if self.microstep == 0 || (self.microstep & (self.microstep - 1)) != 0 {
            return Err("Microstep 2'nin kuvveti olmalı");
        }
        
        Ok(())
    }
}

/// Sistem konfigürasyonu - compile-time constants
pub struct SistemConfig;

impl SistemConfig {
    pub const UART_BAUD_RATE: u32 = 115200;
    pub const WATCHDOG_TIMEOUT_MS: u32 = 5000;
    pub const STEP_PULSE_WIDTH_US: u32 = 5;  // Minimum step pulse width
    
    #[cfg(feature = "debug_uart")]
    pub const LOG_SEVIYESI: &'static str = "debug";
    
    #[cfg(not(feature = "debug_uart"))]
    pub const LOG_SEVIYESI: &'static str = "info";
}
```

```rust
// src/main.rs
#![no_std]
#![no_main]

mod config;

use config::{MotorConfig, SistemConfig};
use rp235x_hal as hal;

#[cfg(feature = "debug_uart")]
use defmt::{info, error};

#[cortex_m_rt::entry]
fn main() -> ! {
    // Flash'tan config oku
    let mut motor_config = config_oku_flash();
    
    // Config validasyonu
    if let Err(e) = motor_config.validate() {
        #[cfg(feature = "debug_uart")]
        error!("Config validasyon hatası: {}, default kullanılıyor", e);
        
        motor_config = MotorConfig::default();
    }
    
    #[cfg(feature = "debug_uart")]
    info!("Motor config yüklendi: {:?}", motor_config);
    
    #[cfg(feature = "debug_uart")]
    info!("UART baud rate: {}", SistemConfig::UART_BAUD_RATE);
    
    // Motor kontrol döngüsü
    loop {
        // UART komutlarını dinle
        // Motor kontrolü yap
        cortex_m::asm::wfi();
    }
}
```

---

# 📚 BÖLÜM 11: Pratik Örnekler

## 11.1 Web Servisi Konfigürasyonu

```rust
use figment::{Figment, providers::{Toml, Env, Format}};
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
struct AppConfig {
    #[validate(range(min = 1, max = 65535))]
    port: u16,
    
    #[validate(length(min = 1))]
    host: String,
    
    #[validate]
    database: DatabaseConfig,
    
    #[validate]
    motor: MotorConfig,
    
    debug: bool,
}

#[derive(Debug, Deserialize, Validate)]
struct DatabaseConfig {
    #[validate(length(min = 1))]
    url: String,
    
    #[validate(range(min = 1, max = 100))]
    max_connections: u32,
    
    #[validate(range(min = 1))]
    connection_timeout_ms: u64,
}

#[derive(Debug, Deserialize, Validate)]
struct MotorConfig {
    #[validate(range(min = 1, max = 3000))]
    max_hiz_rpm: u16,
    
    #[validate(range(min = 1, max = 500))]
    ivme: u16,
    
    #[validate(range(min = 0.001, max = 10.0))]
    adim_basina_mesafe_mm: f64,
}

fn load_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    let config: AppConfig = Figment::new()
        .merge(Toml::file("config.toml").nested())
        .merge(Toml::file("config.local.toml").nested())
        .merge(Env::prefixed("APP_").split("__"))
        .extract()?;
    
    config.validate()?;
    
    Ok(config)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    
    println!("Server {}:{} üzerinde başlatılıyor", config.host, config.port);
    println!("Database: {}", config.database.url);
    println!("Motor max hız: {} RPM", config.motor.max_hiz_rpm);
    
    // Actix-web veya Axum ile server başlat
    Ok(())
}
```

## 11.2 CLI Tool Konfigürasyonu

```rust
use clap::Parser;
use figment::{Figment, providers::{Toml, Env}};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "motor-cli")]
#[command(about = "Motor kontrol CLI aracı")]
struct Cli {
    /// Konfigürasyon dosyası
    #[arg(short, long, default_value = "config.toml")]
    config: PathBuf,
    
    /// Debug modu
    #[arg(short, long)]
    debug: bool,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Motoru hareket ettir
    Hareket {
        /// Hedef pozisyon (mm)
        #[arg(short, long)]
        hedef: f64,
        
        /// Hız (RPM)
        #[arg(short, long)]
        hiz: Option<u16>,
    },
    
    /// Motor durumunu göster
    Durum,
    
    /// Motoru kalibre et
    Kalibre,
}

#[derive(Debug, Deserialize)]
struct Config {
    motor_ip: String,
    motor_port: u16,
    default_hiz: u16,
}

fn main() {
    let cli = Cli::parse();
    
    // Config yükle
    let config: Config = Figment::new()
        .merge(Toml::file(&cli.config))
        .merge(Env::prefixed("MOTOR_"))
        .extract()
        .expect("Config yüklenemedi");
    
    match cli.command {
        Commands::Hareket { hedef, hiz } => {
            let hiz = hiz.unwrap_or(config.default_hiz);
            println!("Motor {} mm hedefe {} RPM ile hareket ediyor", hedef, hiz);
        }
        Commands::Durum => {
            println!("Motor durumu sorgulanıyor: {}:{}", config.motor_ip, config.motor_port);
        }
        Commands::Kalibre => {
            println!("Motor kalibre ediliyor...");
        }
    }
}
```

## 11.3 Multi-Environment Configuration

```rust
use figment::{Figment, providers::{Toml, Env}};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    port: u16,
    database_url: String,
    log_level: String,
}

fn load_config() -> Config {
    let env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
    
    Figment::new()
        // Base config
        .merge(Toml::file("config.toml").nested())
        // Environment-specific override
        .merge(Toml::file(format!("config.{}.toml", env)).nested())
        // Environment variables (highest priority)
        .merge(Env::prefixed("APP_").split("__"))
        .extract()
        .expect("Config yüklenemedi")
}

fn main() {
    let config = load_config();
    println!("Config: {:?}", config);
}
```

**config.toml**:
```toml
port = 8080
log_level = "info"
```

**config.development.toml**:
```toml
database_url = "postgres://localhost/dev_db"
log_level = "debug"
```

**config.production.toml**:
```toml
database_url = "postgres://prod-server/prod_db"
log_level = "warn"
```

---

# 📚 BÖLÜM 12: Best Practices

## 12.1 ✅ İyi Pratikler

1. **Strongly-Typed Config Kullanın:**
```rust
// ✅ İyi: Strongly-typed
#[derive(Debug, Deserialize)]
struct Config {
    port: u16,
    motor: MotorConfig,
}

// ❌ Kötü: String-based
let port = env::var("PORT").unwrap();
```

2. **Validation Ekleyin:**
```rust
// ✅ İyi: Validation
impl Config {
    fn validate(&self) -> Result<(), String> {
        if self.port == 0 {
            return Err("Port 0 olamaz".to_string());
        }
        Ok(())
    }
}

// ❌ Kötü: Validation yok
let config = load_config();
// Geçersiz config ile devam
```

3. **Secret'ları Ayrı Tutun:**
```rust
// ✅ İyi: Secret'lar ayrı
#[derive(Debug, Deserialize)]
struct Config {
    port: u16,
}

#[derive(Debug, Deserialize)]
struct Secrets {
    database_password: SecretString,
    api_key: SecretString,
}

// ❌ Kötü: Her şey aynı yerde
#[derive(Debug, Deserialize)]
struct Config {
    port: u16,
    database_password: String,  // Log'da görünebilir!
}
```

4. **Default Değerler Belirleyin:**
```rust
// ✅ İyi: Default değerler
impl Default for Config {
    fn default() -> Self {
        Self {
            port: 8080,
            debug: false,
        }
    }
}

// ❌ Kötü: Default yok
let port = env::var("PORT").unwrap();  // Panic!
```

5. **Environment Hierarchy Kullanın:**
```rust
// ✅ İyi: Öncelik sırası
Config::builder()
    .set_default("port", 8080)?           // En düşük
    .add_source(File::with_name("config.toml"))
    .add_source(Environment::with_prefix("APP"))  // En yüksek
    .build()?;
```

## 12.2 ❌ Anti-Patterns

```rust
// ❌ Hard-coded config
const PORT: u16 = 8080;
const DATABASE_URL: &str = "postgres://localhost/db";

// ❌ Global mutable config
static mut CONFIG: Option<Config> = None;

fn set_config(config: Config) {
    unsafe {
        CONFIG = Some(config);
    }
}

// ❌ String-based config
let config = HashMap<String, String>::new();
let port = config.get("port").unwrap().parse().unwrap();

// ❌ Config'i her seferinde yükle
fn handle_request() {
    let config = load_config();  // Her request'te dosya oku!
    // ...
}
```

---

# 🎯 ÖZET: Environment and Configuration Kontrol Listesi

| Özellik | C# | Rust |
|---|---|---|
| Environment variables | `Environment.GetEnvironmentVariable` | `std::env::var` |
| Config files | `appsettings.json` | TOML/JSON/YAML (crate ile) |
| CLI parsing | `System.CommandLine` | `clap` (önerilen) |
| Strongly-typed | `IOptions<T>` | `serde` + struct |
| Validation | DataAnnotations | `validator` crate |
| Secrets | User Secrets, Key Vault | `secrecy`, OS keychain |
| Hot reload | `IOptionsSnapshot` | File watch (manuel) |
| Multi-environment | `appsettings.{env}.json` | Figment profiles |
| Embedded config | ❌ | ✅ Compile-time constants, flash |
| DI integration | ✅ (yerleşik) | ⚠️ (manuel) |

---

# 🚀 Son Tavsiyeler

## Web/Backend Uygulamaları İçin

1. **Figment Kullanın:** Birden fazla kaynağı birleştirmek için en iyi çözüm
2. **Serde ile Strongly-Typed Config:** Type safety sağlar
3. **Validator ile Doğrulama:** Geçersiz config'i erken yakala
4. **Secrets Ayrı Yönet:** `secrecy` crate veya OS keychain kullan
5. **Environment Hierarchy:** Default → File → Env → CLI
6. **Multi-Environment:** `config.{env}.toml` pattern kullan

## CLI Uygulamaları İçin

1. **Clap Derive API:** En okunabilir ve güçlü
2. **Subcommands:** Karmaşık CLI'lar için kullan
3. **Auto Completion:** Bash, zsh, fish desteği ekle
4. **Help Messages:** Detaylı ve Türkçe yardım metinleri yaz

## Embedded Sistemler (RP2354B) İçin

1. **Compile-Time Constants:** Mümkün olduğunda kullan (sıfır maliyet)
2. **Flash Storage:** Runtime config için flash'ın bir bölümünü ayır
3. **Postcard + Serde:** no_std için efficient serialization
4. **EEPROM/FRAM:** Sık güncellenen config'ler için
5. **UART Komutları:** Runtime config değişikliği için serial interface
6. **Validation:** Flash'tan okunan config'i her zaman doğrula
7. **Default Fallback:** Geçersiz config durumunda default kullan

## Step Motor Projeniz İçin Önerilen Yapı

```
┌─────────────────────────────────────────────────────────┐
│ Compile-Time Config (const)                             │
│ ├─ UART baud rate                                       │
│ ├─ Watchdog timeout                                     │
│ ├─ Step pulse width                                     │
│ └─ Feature flags                                        │
└─────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│ Flash Storage Config (MotorConfig struct)               │
│ ├─ Max hız (RPM)                                        │
│ ├─ İvme (step/s²)                                       │
│ ├─ Adım başına mesafe (µm)                              │
│ ├─ Min/Max pozisyon                                     │
│ ├─ Home pozisyonu                                       │
│ ├─ Akım limiti                                          │
│ └─ Microstep modu                                       │
└─────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│ Runtime Config (UART komutları ile)                     │
│ ├─ SET_MAX_HIZ <deger>                                  │
│ ├─ SET_IVME <deger>                                     │
│ ├─ GET_CONFIG                                           │
│ ├─ SAVE_CONFIG                                          │
│ └─ RESET_CONFIG                                         │
└─────────────────────────────────────────────────────────┘
```

## Config Stratejisi

```rust
// 1. Compile-time: Mümkün olduğunda kullan
pub const UART_BAUD_RATE: u32 = 115200;

// 2. Flash storage: Motor parametreleri için
#[derive(Serialize, Deserialize)]
struct MotorConfig {
    max_hiz_rpm: u16,
    ivme: u16,
    // ...
}

// 3. EEPROM/FRAM: Sık güncellenen değerler için
// (Eğer flash wear-leveling sorunu varsa)

// 4. UART komutları: Runtime değişiklik için
// SET_MAX_HIZ 1500
// SAVE_CONFIG
```

## CI/CD Entegrasyonu

```yaml
# .github/workflows/test.yml
- name: Test with default config
  run: cargo test

- name: Test with custom config
  run: cargo test --features "high_precision"

- name: Build for RP2354B
  run: |
    cargo build --release --target thumbv8m.main-none-eabihf
    cargo size --release

- name: Validate config schema
  run: |
    cargo run --bin validate-config -- --config config.toml
```

## Config Dosyası Örneği (config.toml)

```toml
# Motor Kontrol Konfigürasyonu

[server]
port = 8080
host = "0.0.0.0"

[database]
url = "postgres://localhost/motor_db"
max_connections = 10
connection_timeout_ms = 5000

[motor]
max_hiz_rpm = 1500
ivme = 100
adim_basina_mesafe_mm = 0.01
max_pozisyon_mm = 1000.0
min_pozisyon_mm = -1000.0
akim_limiti_ma = 2000
microstep = 16

[logging]
level = "info"
dosya = "motor.log"
max boyut_mb = 100

[features]
high_precision = false
fast_mode = false
simulation = false
```

> 🦀 **Unutmayın:**
> - **Web uygulamalarında** `figment` + `serde` + `validator` üçlüsü en güçlü kombinasyondur
> - **CLI uygulamalarında** `clap` derive API en okunabilir ve güçlüdür
> - **Embedded sistemlerde** compile-time constants + flash storage + UART komutları üçlüsü en esnek çözümdür
> - **Secret'ları** asla config dosyalarında tutmayın - environment variables veya OS keychain kullanın
> - **Validation** ekleyerek geçersiz config'i erken yakalayın
> - **Default değerler** belirleyerek eksik config durumunda graceful degradation sağlayın
> - **RP2354B projenizde** flash storage + postcard + serde kombinasyonu ile kalıcı, verimli ve type-safe konfigürasyon yapabilirsiniz
> 
> İyi konfigürasyon yönetimi, uygulamanızın farklı ortamlarda (development, staging, production) sorunsuz çalışmasını sağlar. Rust'ın type safety ve zero-cost abstraction'ları sayesinde, konfigürasyon hatalarını **derleme zamanında** yakalayabilirsiniz!