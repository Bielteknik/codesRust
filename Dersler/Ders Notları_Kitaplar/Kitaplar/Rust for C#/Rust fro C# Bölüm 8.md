# 📝 Rust for .NET Developers: Logging and Tracing (Günlükleme ve İzleme)

Bu bölüm, Rust'ta loglama ve izleme sistemlerinin temellerinden ileri düzey tekniklere kadar kapsamlı bir rehberdir. C# dünyasında `ILogger`, `Serilog`, `NLog` gibi güçlü framework'ler ve ASP.NET Core'un yerleşik logging altyapısıyla çalışmaya alışkınız. Rust ise **modüler, trait-tabanlı** ve özellikle **embedded sistemler için optimize edilmiş** bir yaklaşım sunar.

> 🎯 **Temel Fark:** C#'ta logging genellikle **framework'e gömülüdür** (ASP.NET Core ILogger). Rust'ta ise logging **crate ekosistemi** üzerinden çalışır ve özellikle `tracing` crate'i async/await ile mükemmel entegredir. Embedded tarafta ise `defmt` ile **sıfır maliyetli** loglama mümkündür.

---

# 📚 BÖLÜM 1: Logging Nedir ve Neden Önemlidir?

## 1.1 Tanım

**Logging (Günlükleme)**: Uygulamanın çalışma zamanındaki olaylarını, durumlarını ve hatalarını kaydetme süreci.

**Tracing (İzleme)**: Uygulamanın akışını, fonksiyon çağrılarını ve performans metriklerini izleme.

## 1.2 C# vs Rust Yaklaşımı

| Özellik | C# | Rust |
|---|---|---|
| Yerleşik çözüm | `ILogger<T>` (ASP.NET Core) | Yok (crate gerekir) |
| Ana framework | Serilog, NLog, log4net | `log`, `tracing`, `env_logger` |
| Structured logging | ✅ (Serilog) | ✅ (tracing) |
| Async desteği | ✅ | ✅ (tracing ile mükemmel) |
| Embedded desteği | ❌ | ✅ (defmt - sıfır maliyet) |
| DI entegrasyonu | ✅ (yerleşik) | ⚠️ (manuel veya crate ile) |
| Performance overhead | Orta (reflection) | Düşük (zero-cost abstraction) |

## 1.3 Neden Logging/Tracing Önemli?

1. **Hata ayıklama (Debugging)**: Production'da neyin yanlış gittiğini anlama
2. **Performans izleme**: Darboğazları tespit etme
3. **Audit trail**: Kim, ne zaman, ne yaptı?
4. **Metrics**: Uygulama sağlığını izleme
5. **Distributed tracing**: Mikroservislerde istek takibi
6. **Embedded debugging**: RP2354B gibi donanımlarda UART üzerinden hata ayıklama

---

# 📚 BÖLÜM 2: C# Logging Ekosistemi (Kısa Özet)

## 2.1 ASP.NET Core ILogger

```csharp
public class MotorKontrolController : ControllerBase
{
    private readonly ILogger<MotorKontrolController> _logger;
    
    public MotorKontrolController(ILogger<MotorKontrolController> logger)
    {
        _logger = logger;
    }
    
    [HttpPost("hareket")]
    public IActionResult HareketEt([FromBody] HareketKomutu komut)
    {
        _logger.LogInformation("Motor hareket ettiriliyor: {Hedef} mm", komut.Hedef);
        
        try
        {
            _motorServis.HareketEt(komut.Hedef);
            _logger.LogInformation("Hareket tamamlandı");
            return Ok();
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Motor hareketi başarısız oldu");
            return StatusCode(500);
        }
    }
}
```

## 2.2 Serilog (Structured Logging)

```csharp
Log.Logger = new LoggerConfiguration()
    .MinimumLevel.Debug()
    .WriteTo.Console()
    .WriteTo.File("logs/motor-.txt", rollingInterval: RollingInterval.Day)
    .WriteTo.Seq("http://localhost:5341")
    .CreateLogger();

// Kullanım
Log.Information("Motor hızı {Hiz} RPM, sıcaklık {Sicaklik}°C", 1500, 45.2);
// Çıktı: {"Timestamp":"...","Level":"Information","MessageTemplate":"...","Properties":{"Hiz":1500,"Sicaklik":45.2}}
```

## 2.3 C# Logging'in Sorunları

- ❌ Reflection overhead (özellikle Serilog)
- ❌ GC baskısı (string allocation)
- ❌ Embedded sistemlerde kullanılamaz
- ❌ Async/await ile bazen sorunlar (logical call context)

---

# 📚 BÖLÜM 3: Rust Logging - `log` Crate (Temel Yaklaşım)

## 3.1 `log` Crate - Logging Facade

`log` crate, C#'taki `Microsoft.Extensions.Logging.Abstractions` gibi bir **soyutlama katmanıdır**. Sadece log makrolarını sağlar, gerçek loglama işini başka bir crate yapar.

**Cargo.toml**:
```toml
[dependencies]
log = "0.4"
env_logger = "0.10"  # Basit console logger
```

**Kod**:
```rust
use log::{debug, info, warn, error, trace};

fn motor_hareketi(hedef_mm: f64) {
    trace!("motor_hareketi çağrıldı: hedef={}", hedef_mm);
    debug!("Hedef pozisyon hesaplanıyor");
    
    if hedef_mm < 0.0 {
        warn!("Negatif hedef değer, mutlak değer kullanılacak");
    }
    
    info!("Motor {} mm hareket ettiriliyor", hedef_mm);
    
    // Hareket mantığı...
    
    if false { // hata durumu simülasyonu
        error!("Motor sürücü ile iletişim kopuk!");
    }
}

fn main() {
    // Logger'ı başlat (bir kez, uygulamanın başında)
    env_logger::init();
    
    motor_hareketi(100.0);
}
```

## 3.2 Log Seviyeleri

| Seviye | C# Karşılığı | Kullanım |
|---|---|---|
| `trace!` | `Trace` | Çok detaylı, geliştirme sırasında |
| `debug!` | `Debug` | Hata ayıklama bilgileri |
| `info!` | `Information` | Önemli olaylar (başlangıç, bitiş) |
| `warn!` | `Warning` | Beklenmedik ama tolere edilebilir durumlar |
| `error!` | `Error` | Hatalar, başarısız işlemler |

## 3.3 Çalıştırma ve Filtreleme

```bash
# Tüm logları göster
RUST_LOG=debug cargo run

# Sadece kendi crate'inizin loglarını göster
RUST_LOG=motor_projesi=debug cargo run

# Farklı modüller için farklı seviyeler
RUST_LOG=motor_projesi=debug,hyper=info,tokio=warn cargo run

# Trace seviyesine kadar
RUST_LOG=trace cargo run
```

> 💡 **Kritik Fark:** C#'ta log seviyeleri `appsettings.json` ile yapılandırılır. Rust'ta ise **environment variable** (`RUST_LOG`) ile runtime'da değiştirilebilir - yeniden derleme gerekmez!

## 3.4 Structured Logging (log crate ile)

```rust
use log::info;

// Basit formatlı log
info!("Motor hızı {} RPM, sıcaklık {}°C", 1500, 45.2);

// log crate doğrudan structured logging desteklemez
// Bunun için tracing crate kullanılır
```

---

# 📚 BÖLÜM 4: `tracing` Crate (Modern Yaklaşım) ⭐

`tracing`, Rust'ın **modern, async-dostu** logging ve tracing framework'üdür. C#'taki Serilog + OpenTelemetry kombinasyonuna benzer ama çok daha entegredir.

## 4.1 Neden `tracing`?

- ✅ **Structured logging** yerleşik
- ✅ **Async/await** ile mükemmel çalışır
- ✅ **Span-based tracing** (fonksiyon çağrılarını otomatik izle)
- ✅ **Zero-cost abstraction** (kullanılmayan loglar derleme zamanında elenir)
- ✅ **OpenTelemetry** entegrasyonu
- ✅ **Distributed tracing** desteği

## 4.2 Kurulum

**Cargo.toml**:
```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
```

## 4.3 İlk tracing Örneği

```rust
use tracing::{info, debug, warn, error, instrument};
use tracing_subscriber::{fmt, EnvFilter};

#[instrument]  // Bu fonksiyonu otomatik olarak izle!
fn motor_hareketi(hedef_mm: f64) -> Result<(), String> {
    info!(hedef = hedef_mm, "Motor hareketi başlatılıyor");
    
    if hedef_mm < -500.0 || hedef_mm > 500.0 {
        warn!(hedef = hedef_mm, limit = 500.0, "Hedef sınır dışında");
        return Err("Hedef sınır dışında".to_string());
    }
    
    debug!("Hız profili hesaplanıyor");
    let hiz_profili = hesapla_hiz_profili(hedef_mm);
    
    info!(adim_sayisi = hiz_profili.len(), "Hareket tamamlandı");
    Ok(())
}

fn hesapla_hiz_profili(mesafe: f64) -> Vec<u32> {
    // Basit hız profili
    vec![100, 200, 300, 400, 500]
}

fn main() {
    // Subscriber'ı başlat
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    
    info!(versiyon = "1.0.0", "Motor kontrol sistemi başlatılıyor");
    
    match motor_hareketi(100.0) {
        Ok(_) => info!("İşlem başarılı"),
        Err(e) => error!(hata = %e, "İşlem başarısız"),
    }
}
```

## 4.4 Çalıştırma

```bash
RUST_LOG=info cargo run
```

**Çıktı**:
```
2024-01-15T10:30:45.123Z  INFO motor_kontrol: versiyon="1.0.0" Motor kontrol sistemi başlatılıyor
2024-01-15T10:30:45.124Z  INFO motor_hareketi: motor_kontrol: hedef=100.0 Motor hareketi başlatılıyor
2024-01-15T10:30:45.125Z  INFO motor_hareketi: motor_kontrol: adim_sayisi=5 Hareket tamamlandı
2024-01-15T10:30:45.126Z  INFO motor_kontrol: İşlem başarılı
```

---

# 📚 BÖLÜM 5: Structured Logging (Yapılandırılmış Günlükleme)

## 5.1 Temel Structured Logging

```rust
use tracing::info;

// C# (Serilog)
// Log.Information("Motor {MotorId} hızı {Hiz} RPM", motorId, hiz);

// Rust (tracing)
info!(
    motor_id = 1,
    hiz = 1500,
    sicaklik = 45.2,
    durum = "calisiyor",
    "Motor durumu güncellendi"
);
```

**JSON Çıktısı** (tracing-subscriber JSON formatı ile):
```json
{
  "timestamp": "2024-01-15T10:30:45.123Z",
  "level": "INFO",
  "fields": {
    "message": "Motor durumu güncellendi",
    "motor_id": 1,
    "hiz": 1500,
    "sicaklik": 45.2,
    "durum": "calisiyor"
  },
  "target": "motor_kontrol"
}
```

## 5.2 JSON Formatında Loglama

```rust
use tracing_subscriber::{fmt, EnvFilter};

fn main() {
    tracing_subscriber::fmt()
        .json()  // JSON formatında çıktı
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(true)  // Modül adını ekle
        .with_thread_ids(true)  // Thread ID ekle
        .with_file(true)  // Dosya adını ekle
        .with_line_number(true)  // Satır numarasını ekle
        .init();
    
    info!(kullanici = "admin", islem = "login", "Kullanıcı giriş yaptı");
}
```

**Çıktı**:
```json
{"timestamp":"2024-01-15T10:30:45.123Z","level":"INFO","fields":{"message":"Kullanıcı giriş yaptı","kullanici":"admin","islem":"login"},"target":"motor_kontrol","filename":"src/main.rs","line_number":15}
```

## 5.3 C# vs Rust Structured Logging

| Özellik | C# (Serilog) | Rust (tracing) |
|---|---|---|
| Syntax | `Log.Info("Mesaj {Param}", deger)` | `info!(param = deger, "Mesaj")` |
| Type safety | ⚠️ Runtime kontrol | ✅ Compile-time kontrol |
| Performance | Reflection overhead | Zero-cost |
| Async context | AsyncLocal | Span-based (daha iyi) |
| Output formats | Sink'ler ile | Subscriber'lar ile |

---

# 📚 BÖLÜM 6: Spans ve Instrumentation ⭐

`tracing`'in en güçlü özelliği: **Span-based instrumentation**. Fonksiyon çağrılarını otomatik olarak izler.

## 6.1 `#[instrument]` Attribute

```rust
use tracing::{info, instrument};

#[instrument]  // Bu fonksiyon için otomatik span oluştur
fn veritabani_sorgula(kullanici_id: i32) -> Vec<String> {
    info!("Sorgu başlatılıyor");
    std::thread::sleep(std::time::Duration::from_millis(100));
    vec!["veri1".to_string(), "veri2".to_string()]
}

#[instrument]
fn is_mantigi() {
    info!("İş mantığı çalışıyor");
    let veriler = veritabani_sorgula(42);
    info!("{} veri alındı", veriler.len());
}

#[instrument]
fn api_cagrisi() {
    info!("API çağrısı başlatılıyor");
    is_mantigi();
    info!("API çağrısı tamamlandı");
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("trace")
        .init();
    
    api_cagrisi();
}
```

**Çıktı**:
```
2024-01-15T10:30:45.123Z  INFO api_cagrisi: motor_kontrol: API çağrısı başlatılıyor
2024-01-15T10:30:45.124Z  INFO is_mantigi: motor_kontrol: İş mantığı çalışıyor
2024-01-15T10:30:45.125Z  INFO veritabani_sorgula: motor_kontrol: Sorgu başlatılıyor
2024-01-15T10:30:45.225Z  INFO is_mantigi: motor_kontrol: 2 veri alındı
2024-01-15T10:30:45.226Z  INFO api_cagrisi: motor_kontrol: API çağrısı tamamlandı
```

> 💡 **Güçlü Özellik:** Her span, parent-child ilişkisini otomatik olarak takip eder. Bu, **call tree** oluşturmak için mükemmeldir.

## 6.2 Span Parametreleri

```rust
use tracing::{info, instrument, Level};

#[instrument(level = "debug")]  // Span seviyesini belirle
fn hassas_islem(
    #[instrument(fields(kullanici_id))]  // Bu parametreyi span'a ekle
    kullanici: &Kullanici,
    
    #[instrument(skip)]  // Bu parametreyi loglama (gizli veri)
    sifre: &str,
) {
    info!("İşlem yapılıyor");
}

#[instrument(ret)]  // Dönüş değerini de logla
fn hesapla(a: i32, b: i32) -> i32 {
    a + b
}

#[instrument(err)]  // Hata durumunda otomatik error log
fn riskli_islem() -> Result<(), String> {
    Err("Bir hata oluştu".to_string())
}
```

## 6.3 Manuel Span Oluşturma

```rust
use tracing::{info_span, debug_span};

fn karmasik_islem() {
    let span = info_span!("karmasik_islem", phase = "hesaplama");
    let _guard = span.enter();  // Span'ı aktif et
    
    info!("Hesaplama başladı");
    
    {
        let alt_span = debug_span!("alt_islem", adim = 1);
        let _alt_guard = alt_span.enter();
        info!("Alt işlem 1 çalışıyor");
    } // alt_span burada otomatik kapanır
    
    info!("Hesaplama tamamlandı");
} // span burada otomatik kapanır
```

## 6.4 Async Fonksiyonlarda Instrumentation

```rust
use tracing::{info, instrument};

#[instrument]
async fn veri_indir(url: &str) -> Result<String, reqwest::Error> {
    info!(url = url, "İndirme başlatılıyor");
    let response = reqwest::get(url).await?;
    let icerik = response.text().await?;
    info!(boyut = icerik.len(), "İndirme tamamlandı");
    Ok(icerik)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    let sonuc = veri_indir("https://example.com").await;
    info!("Sonuç: {:?}", sonuc.is_ok());
}
```

> 🎯 **Kritik Fark:** C#'ta async logging bazen sorunludur (AsyncLocal context kaybı). Rust'ın `tracing` crate'i, async/await ile **mükemmel** çalışır çünkü span'lar future'lar ile birlikte taşınır.

---

# 📚 BÖLÜM 7: Log Seviyeleri ve Filtreleme

## 7.1 Environment Filter

```rust
use tracing_subscriber::EnvFilter;

fn main() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .init();
}
```

```bash
# Örnek filtreler
RUST_LOG=info cargo run                              # Tüm modüller info+
RUST_LOG=motor=debug,db=trace cargo run              # Modül bazlı
RUST_LOG=info,motor=debug,hyper=off cargo run        # hyper'ı kapat
RUST_LOG=info,[db]=trace cargo run                   # db modülü trace
```

## 7.2 Programatik Filtreleme

```rust
use tracing_subscriber::{fmt, EnvFilter, filter::LevelFilter};

fn main() {
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy()
        .add_directive("motor=debug".parse().unwrap())
        .add_directive("tokio=warn".parse().unwrap());
    
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .init();
}
```

## 7.3 Dinamik Filtre Değiştirme

```rust
use tracing_subscriber::reload;
use tracing::Level;

fn main() {
    let (filter, reload_handle) = reload::Layer::new(
        tracing_subscriber::EnvFilter::new("info")
    );
    
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .init();
    
    // Runtime'da filtre değiştir
    info!("Başlangıç seviyesi: INFO");
    
    // Debug seviyesine geçir
    reload_handle.modify(|filter| {
        *filter = tracing_subscriber::EnvFilter::new("debug");
    }).unwrap();
    
    debug!("Bu mesaj artık görünür!");
}
```

> 💡 **Avantaj:** C#'ta log seviyesini değiştirmek için genellikle uygulamayı yeniden başlatmak gerekir. Rust'ta `reload` layer ile **runtime'da** değiştirilebilir!

---

# 📚 BÖLÜM 8: Subscribers (Log Çıktıları)

## 8.1 Multiple Subscribers

```rust
use tracing_subscriber::{fmt, EnvFilter, prelude::*};
use std::fs::File;

fn main() {
    // Console subscriber
    let console_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(true);
    
    // File subscriber
    let dosya = File::create("motor.log").unwrap();
    let file_layer = fmt::layer()
        .json()
        .with_writer(dosya);
    
    tracing_subscriber::registry()
        .with(EnvFilter::new("info"))
        .with(console_layer)
        .with(file_layer)
        .init();
    
    tracing::info!("Bu mesaj hem console'a hem dosyaya yazılır");
}
```

## 8.2 Rolling File Appender

```rust
use tracing_appender::rolling;
use tracing_subscriber::{fmt, EnvFilter};

fn main() {
    // Günlük dosya oluştur
    let dosya_appender = rolling::daily("logs", "motor.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(dosya_appender);
    
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_env_filter(EnvFilter::new("info"))
        .with_ansi(false)  // Dosyada renk kodları olmasın
        .init();
    
    tracing::info!("Uygulama başlatıldı");
}
```

## 8.3 Subscriber Katmanları (Layers)

```rust
use tracing_subscriber::{fmt, EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::new("info"))
        .with(fmt::layer().with_target(true))  // Layer 1: Console
        .with(
            fmt::layer()
                .json()
                .with_writer(|| File::create("audit.json").unwrap())
        )  // Layer 2: JSON dosya
        .init();
}
```

---

# 📚 BÖLÜM 9: Özel Log Formatları

## 9.1 Custom Formatter

```rust
use tracing_subscriber::fmt::format::Format;
use tracing_subscriber::fmt::time::UtcTime;

fn main() {
    tracing_subscriber::fmt()
        .with_timer(UtcTime::rfc_3339())  // RFC 3339 zaman formatı
        .with_target(false)  // Target'ı gizle
        .with_thread_names(true)  // Thread isimlerini göster
        .with_level(true)
        .compact()  // Kompakt format
        .init();
}
```

## 9.2 Tamamen Özel Format

```rust
use tracing::{Event, Subscriber};
use tracing_subscriber::fmt::FormatEvent;
use tracing_subscriber::fmt::FormatFields;
use tracing_subscriber::registry::LookupSpan;
use std::fmt;

struct OzelFormat;

impl<S, N> FormatEvent<S, N> for OzelFormat
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &tracing_subscriber::fmt::FmtContext<'_, S, N>,
        mut writer: tracing_subscriber::fmt::format::Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        let metadata = event.metadata();
        write!(
            writer,
            "[{}] [{}] [{}] ",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"),
            metadata.level(),
            metadata.target()
        )?;
        ctx.field_format().format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}

fn main() {
    tracing_subscriber::fmt()
        .event_format(OzelFormat)
        .init();
    
    tracing::info!(motor_id = 1, "Özel format testi");
}
```

---

# 📚 BÖLÜM 10: Distributed Tracing (OpenTelemetry)

## 10.1 OpenTelemetry Entegrasyonu

**Cargo.toml**:
```toml
[dependencies]
tracing = "0.1"
tracing-opentelemetry = "0.22"
opentelemetry = { version = "0.21", features = ["trace"] }
opentelemetry-jaeger = "0.20"  # Jaeger exporter
```

**Kod**:
```rust
use opentelemetry::global;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use tracing_subscriber::{layer::SubscriberExt, Registry};

fn main() {
    // Jaeger exporter başlat
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("motor-kontrol-servisi")
        .install_simple()
        .expect("Tracer kurulamadı");
    
    // OpenTelemetry layer
    let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);
    
    let subscriber = Registry::default()
        .with(tracing_subscriber::EnvFilter::new("info"))
        .with(tracing_subscriber::fmt::layer())
        .with(otel_layer);
    
    tracing::subscriber::set_global_default(subscriber).unwrap();
    
    {
        let root_span = tracing::info_span!("api_cagrisi");
        let _guard = root_span.enter();
        
        tracing::info!("İstek alındı");
        is_mantigi();
    }
    
    // Program bitince tracer'ı kapat
    global::shutdown_tracer_provider();
}

#[tracing::instrument]
fn is_mantigi() {
    tracing::info!("İş mantığı çalışıyor");
    std::thread::sleep(std::time::Duration::from_millis(50));
}
```

## 10.2 Jaeger UI'da Görüntüleme

```bash
# Jaeger'ı Docker ile başlat
docker run -d --name jaeger \
  -e COLLECTOR_OTLP_ENABLED=true \
  -p 16686:16686 \
  jaegertracing/all-in-one:1.52

# Uygulamayı çalıştır
cargo run

# Tarayıcıda aç
# http://localhost:16686
```

> 🎯 **Avantaj:** Mikroservis mimarisinde, bir isteğin tüm servislerde nasıl ilerlediğini görsel olarak görebilirsiniz.

---

# 📚 BÖLÜM 11: Embedded Logging - `defmt` (RP2354B için) ⭐⭐⭐

**Bu bölüm, RP2354B step motor projeniz için KRİTİK ÖNEM taşır!**

## 11.1 Embedded Logging Zorlukları

- ❌ UART bandwidth sınırlı (genellikle 115200 baud)
- ❌ String formatting pahalı (CPU ve bellek)
- ❌ Heap allocation yok (`no_std`)
- ❌ Real-time gereksinimler (loglama zamanlamayı bozamaz)

## 11.2 `defmt` Nedir?

**defmt** (deferred formatting), **sıfır maliyetli** embedded logging framework'üdür:

- ✅ **Compile-time formatting**: Format string'leri binary'e gömülür
- ✅ **Sıfır heap allocation**: Stack'te bile allocation yok
- ✅ **Çok düşük bandwidth**: Sadece parametreler gönderilir
- ✅ **Real-time safe**: Deterministik zamanlama
- ✅ **Probe-rs entegrasyonu**: SWD/JTAG üzerinden loglama

## 11.3 defmt Kurulumu

**Cargo.toml**:
```toml
[dependencies]
defmt = "0.3"
defmt-rtt = "0.4"  # RTT (Real-Time Transfer) üzerinden loglama
# veya
# defmt-semihosting = "0.1"  # Semihosting üzerinden

[dependencies.cortex-m]
version = "0.7"
features = ["critical-section-single-core"]

[dependencies.embedded-hal]
version = "1.0"
```

**Memory.x** (RP2354B için):
```
MEMORY
{
  BOOT2 : ORIGIN = 0x10000000, LENGTH = 0x100
  FLASH : ORIGIN = 0x10000100, LENGTH = 2048K - 0x100
  RAM   : ORIGIN = 0x20000000, LENGTH = 512K
}
```

## 11.4 İlk defmt Örneği

```rust
#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;  // RTT transport
use panic_probe as _;  // Panic handler

use rp235x_hal as hal;
use hal::pac;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    
    // GPIO başlat
    let sio = hal::Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    
    let mut led = pins.gpio25.into_push_pull_output();
    
    defmt::info!("Motor kontrol sistemi başlatılıyor!");
    defmt::info!("MCU: RP2354B, Clock: {} MHz", 150);
    
    loop {
        led.toggle().unwrap();
        defmt::debug!("LED toggle");
        cortex_m::asm::delay(125_000_000);  // 1 saniye
    }
}
```

## 11.5 Log Seviyeleri

```rust
use defmt::{trace, debug, info, warn, error};

fn motor_kontrol() {
    trace!("Çok detaylı bilgi");  // Sadece geliştirme
    debug!("Debug bilgisi");       // Hata ayıklama
    info!("Motor başlatıldı");     // Önemli olaylar
    warn!("Sıcaklık yüksek: {}°C", 85);  // Uyarılar
    error!("Motor sürücü hatası!");       // Hatalar
}
```

## 11.6 Structured Logging (defmt)

```rust
use defmt::info;

struct MotorDurumu {
    id: u8,
    hiz: u16,
    sicaklik: f32,
}

fn durum_guncelle(durum: &MotorDurumu) {
    // defmt format string'leri compile-time'da işlenir!
    info!(
        "Motor {} - Hız: {} RPM, Sıcaklık: {=f32}°C",
        durum.id,
        durum.hiz,
        durum.sicaklik
    );
}

// defmt::Format trait implementasyonu
#[derive(defmt::Format)]
struct AdimKomutu {
    hedef_pozisyon: i32,
    hiz: u16,
    ivme: u16,
}

fn komut_gonder(komut: &AdimKomutu) {
    info!("Komut gönderiliyor: {:#?}", komut);
    // Çıktı: AdimKomutu { hedef_pozisyon: 1000, hiz: 500, ivme: 100 }
}
```

## 11.7 defmt vs println! Karşılaştırması

```rust
// ❌ Kötü: println! (no_std'de yok ama örnek olsun)
// println!("Motor hızı {} RPM", hiz);  // String allocation!

// ❌ Kötü: write! ile UART
use core::fmt::Write;
writeln!(uart, "Motor hızı {} RPM", hiz).unwrap();  // Runtime formatting

// ✅ İyi: defmt
defmt::info!("Motor hızı {} RPM", hiz);  // Compile-time formatting!
```

**Binary Boyut Karşılaştırması**:
```
println! benzeri yaklaşım:  +50 KB flash, +2 KB RAM
defmt:                       +2 KB flash, +0.5 KB RAM
```

## 11.8 Log Görüntüleme (probe-rs)

```bash
# probe-rs kur
cargo install probe-rs-tools

# Firmware'i flash et ve logları görüntüle
cargo run --release

# Veya ayrı ayrı
cargo flash --chip RP2354 --release
cargo embed --chip RP2354
```

**Çıktı**:
```
0.000000 INFO Motor kontrol sistemi başlatılıyor!
0.000123 INFO MCU: RP2354B, Clock: 150 MHz
0.500000 DEBUG LED toggle
1.000000 DEBUG LED toggle
1.500000 DEBUG LED toggle
```

## 11.9 Filtreleme (defmt)

```rust
// .cargo/config.toml
[env]
DEFMT_LOG = "info"  # Veya: trace, debug, info, warn, error
```

```bash
# Runtime'da değiştir
DEFMT_LOG=debug cargo run
DEFMT_LOG=motor_kontrol=trace cargo run
```

## 11.10 Timestamp ve Assertion

```rust
use defmt::{info, assert, unwrap};

fn kritik_islem(deger: i32) {
    // Assertion (panic ile aynı ama defmt formatında)
    assert!(deger > 0, "Değer pozitif olmalı: {=i32}", deger);
    
    // Unwrap (Option/Result için)
    let sonuc = Some(42);
    let deger = unwrap!(sonuc, "None olmamalıydı");
    
    info!("İşlem tamamlandı: {=i32}", deger);
}

// Panic handler
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("PANIC: {}", info);
    loop {
        cortex_m::asm::bkpt();
    }
}
```

## 11.11 Step Motor Projesi İçin defmt Stratejisi

```rust
#![no_std]
#![no_main]

use defmt::{info, debug, warn, error};
use rp235x_hal as hal;

// Motor durumu
struct MotorState {
    pozisyon: i32,
    hiz: u16,
    adim_sayaci: u32,
}

impl defmt::Format for MotorState {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MotorState {{ pozisyon: {=i32}, hiz: {=u16}, adim: {=u32} }}",
            self.pozisyon,
            self.hiz,
            self.adim_sayaci
        );
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    // ... initialization ...
    
    info!("Step motor kontrolcüsü başlatıldı");
    
    let mut state = MotorState {
        pozisyon: 0,
        hiz: 0,
        adim_sayaci: 0,
    };
    
    loop {
        // Interrupt'ta loglama
        debug!(state = &state, "Motor durumu güncellendi");
        
        if state.hiz > 1000 {
            warn!(hiz = state.hiz, "Motor çok hızlı!");
        }
        
        // Adım üretimi (kritik zamanlama)
        adim_at();
        state.adim_sayaci += 1;
        
        // Her 1000 adımda bir info log
        if state.adim_sayaci % 1000 == 0 {
            info!(adim = state.adim_sayaci, "1000 adım tamamlandı");
        }
    }
}

#[interrupt]
fn TIMER_IRQ() {
    // Interrupt handler'da da defmt kullanılabilir
    // Ama dikkat: çok sık loglama real-time'ı bozabilir
    debug!("Timer interrupt");
}
```

## 11.12 defmt Best Practices

1. **Production'da Sadece info+ Kullanın:**
```toml
# .cargo/config.toml
[env]
DEFMT_LOG = "info"  # Debug/trace release'ta kapalı
```

2. **Kritik Bölümlerde Loglamayı Azaltın:**
```rust
// ❌ Kötü: Her interrupt'ta log
#[interrupt]
fn TIMER_IRQ() {
    info!("Interrupt");  // 10kHz interrupt = saniyede 10000 log!
}

// ✅ İyi: Sayaç ile azalt
static SAYAC: AtomicU32 = AtomicU32::new(0);

#[interrupt]
fn TIMER_IRQ() {
    if SAYAC.fetch_add(1, Ordering::Relaxed) % 1000 == 0 {
        info!("1000 interrupt işlendi");
    }
}
```

3. **Format String'lerini Basit Tutun:**
```rust
// ✅ İyi
info!("Motor {} RPM", hiz);

// ❌ Kötü: Karmaşık format
info!("Motor {} RPM, {}°C, {}V, {}A, durum: {:?}", 
    hiz, sicaklik, voltaj, akim, durum);
```

4. **Binary Boyutu İzleyin:**
```bash
cargo size --release
# defmt string'lerinin flash kullanımını kontrol edin
```

---

# 📚 BÖLÜM 12: Pratik Örnekler

## 12.1 Web Servisi Logging (Actix-web + tracing)

```rust
use actix_web::{web, App, HttpServer, HttpResponse};
use tracing::{info, instrument};
use tracing_subscriber::{fmt, EnvFilter};

#[instrument]
async fn motor_kontrol(body: web::Json<HareketKomutu>) -> HttpResponse {
    info!(hedef = body.hedef, hiz = body.hiz, "Motor komutu alındı");
    
    // İş mantığı
    match motor_servis.hareket_et(body.hedef, body.hiz).await {
        Ok(_) => {
            info!("Hareket başarılı");
            HttpResponse::Ok().json({"status": "ok"})
        }
        Err(e) => {
            tracing::error!(hata = %e, "Hareket başarısız");
            HttpResponse::InternalServerError().json({"error": e.to_string()})
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .json()
        .init();
    
    info!("Motor kontrol servisi başlatılıyor");
    
    HttpServer::new(|| {
        App::new()
            .wrap(TracingLogger::default())  // HTTP request/response loglama
            .route("/api/motor/hareket", web::post().to(motor_kontrol))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

## 12.2 Middleware ile Otomatik Logging

```rust
use tracing::{info, Span};
use std::time::Instant;

struct LogMiddleware;

impl<S, B> Transform<S, ServiceRequest> for LogMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
{
    // ... implementation ...
    
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start = Instant::now();
        let path = req.path().to_string();
        let method = req.method().to_string();
        
        info!(method = %method, path = %path, "Request başladı");
        
        let fut = self.service.call(req);
        
        Box::pin(async move {
            let res = fut.await?;
            let elapsed = start.elapsed();
            
            info!(
                method = %method,
                path = %path,
                status = res.status().as_u16(),
                elapsed_ms = elapsed.as_millis(),
                "Request tamamlandı"
            );
            
            Ok(res)
        })
    }
}
```

## 12.3 Error Context ile Zenginleştirme

```rust
use tracing::error;
use anyhow::{Context, Result};

fn veritabani_baglantisi() -> Result<Connection> {
    let baglanti = Connection::new("postgres://localhost/db")
        .context("Veritabanına bağlanılamadı")?;
    
    baglanti.ping()
        .context("Veritabanı ping başarısız")?;
    
    Ok(baglanti)
}

fn ana_fonksiyon() -> Result<()> {
    let baglanti = veritabani_baglantisi()
        .context("Ana fonksiyon: DB bağlantı hatası")?;
    
    // ...
    Ok(())
}

fn main() {
    tracing_subscriber::fmt::init();
    
    if let Err(e) = ana_fonksiyon() {
        error!(hata = ?e, "Uygulama hatası");
        // Çıktı: 
        // Uygulama hatası: Ana fonksiyon: DB bağlantı hatası: 
        // Veritabanına bağlanılamadı: Connection refused
    }
}
```

---

# 📚 BÖLÜM 13: Logging Best Practices

## 13.1 ✅ İyi Pratikler

1. **Structured Logging Kullanın:**
```rust
// ✅ İyi
info!(motor_id = 1, hiz = 1500, sicaklik = 45.2, "Motor durumu");

// ❌ Kötü
info!("Motor 1 hızı 1500 RPM, sıcaklık 45.2°C");
```

2. **Hassas Verileri Loglamayın:**
```rust
// ✅ İyi
#[instrument(skip(sifre))]  // sifre parametresini loglama
fn login(kullanici: &str, sifre: &str) { }

// ❌ Kötü
info!("Login: {} / {}", kullanici, sifre);  // Şifre loglandı!
```

3. **Error'ları Context ile Zenginleştirin:**
```rust
// ✅ İyi
dosya_oku("config.json")
    .context("Konfigürasyon dosyası okunamadı")?;

// ❌ Kötü
let icerik = dosya_oku("config.json").unwrap();  // Panic!
```

4. **Uygun Log Seviyesi Kullanın:**
```rust
// ✅ Doğru seviyeler
trace!("Değişken değeri: x = {}", x);  // Çok detaylı
debug!("Fonksiyon çağrıldı");           // Geliştirme
info!("İşlem başlatıldı");              // Önemli olaylar
warn!("Kaynak %90 dolu");               // Uyarı
error!("Bağlantı koptu");               // Hata
```

5. **Correlation ID Kullanın:**
```rust
use uuid::Uuid;

#[instrument(fields(request_id = %Uuid::new_v4()))]
async fn api_handler() {
    // Tüm loglar otomatik olarak request_id içerir
    info!("İstek işleniyor");
}
```

## 13.2 ❌ Anti-Patterns

```rust
// ❌ Çok fazla log
fn hesapla(a: i32, b: i32) -> i32 {
    debug!("a değeri: {}", a);
    debug!("b değeri: {}", b);
    let sonuc = a + b;
    debug!("sonuç: {}", sonuc);
    sonuc
}

// ❌ String concatenation
info!("Kullanıcı ".to_string() + &kullanici + " giriş yaptı");

// ✅ Doğru
info!(kullanici = %kullanici, "Giriş yapıldı");

// ❌ Sensitive data
info!("Kredi kartı: {}", kart_no);  // ASLA!

// ❌ Loop içinde log
for i in 0..10000 {
    info!("İterasyon: {}", i);  // 10000 log!
}

// ✅ Doğru
info!("{} iterasyon tamamlandı", 10000);
```

---

# 🎯 ÖZET: Logging/Tracing Kontrol Listesi

| Özellik | C# (Serilog/ILogger) | Rust (tracing/defmt) |
|---|---|---|
| Temel crate | Microsoft.Extensions.Logging | `log` veya `tracing` |
| Structured logging | ✅ (Serilog) | ✅ (tracing) |
| Async desteği | ⚠️ (AsyncLocal) | ✅ (Span-based) |
| Instrumentation | ⚠️ (manuel) | ✅ (`#[instrument]`) |
| Distributed tracing | OpenTelemetry | OpenTelemetry |
| Embedded logging | ❌ | ✅ (defmt - sıfır maliyet) |
| Runtime filtreleme | ⚠️ (kısıtlı) | ✅ (env-filter) |
| Multiple outputs | Sinks | Subscribers/Layers |
| Performance overhead | Orta | Düşük (zero-cost) |
| Binary boyut etkisi | N/A | defmt: minimal |
| Real-time safe | ❌ | ✅ (defmt) |

---

# 🚀 Son Tavsiyeler

## Web/Backend Uygulamaları İçin

1. **`tracing` Crate Kullanın:** Modern, async-dostu, structured logging
2. **`#[instrument]` Attribute:** Tüm public fonksiyonları otomatik izleyin
3. **JSON Formatı:** Log aggregation (ELK, Loki) için ideal
4. **OpenTelemetry:** Distributed tracing için entegre edin
5. **Environment Filter:** `RUST_LOG` ile runtime'da filtreleyin
6. **Correlation ID:** Her request'e benzersiz ID ekleyin

## Embedded Sistemler (RP2354B) İçin

1. **`defmt` Kullanın:** Sıfır maliyetli, real-time safe
2. **RTT Transport:** SWD/JTAG üzerinden yüksek hızlı loglama
3. **Production'da info+:** Debug/trace release'ta kapalı
4. **Kritik Bölgelerde Az Log:** Interrupt handler'larda dikkatli
5. **Format Trait:** Özel tipler için `defmt::Format` implemente edin
6. **Binary Boyutu İzleyin:** defmt string'leri flash kullanır

## Step Motor Projeniz İçin Önerilen Yapı

```
┌─────────────────────────────────────────────────────────┐
│ Host (PC) Tarafı                                        │
│ ├─ tracing + tracing-subscriber                         │
│ ├─ JSON output → ELK/Loki                               │
│ ├─ OpenTelemetry → Jaeger                               │
│ └─ Actix-web middleware ile HTTP logging                │
└─────────────────────────────────────────────────────────┘
                          ↕ UART/USB
┌─────────────────────────────────────────────────────────┐
│ RP2354B (Embedded) Tarafı                               │
│ ├─ defmt + defmt-rtt                                    │
│ ├─ SWD üzerinden probe-rs ile log görüntüleme           │
│ ├─ Motor durumu: info (her 1000 adımda)                 │
│ ├─ Hata durumları: error (anında)                       │
│ ├─ Debug bilgileri: debug (geliştirme sırasında)        │
│ └─ Interrupt handler: trace (sadece debug build)        │
└─────────────────────────────────────────────────────────┘
```

## Log Stratejisi

```rust
// Production build
#[cfg(not(debug_assertions))]
const LOG_SEVIYESI: &str = "info";

// Development build
#[cfg(debug_assertions)]
const LOG_SEVIYESI: &str = "debug";
```

## CI/CD Entegrasyonu

```yaml
# .github/workflows/test.yml
- name: Run tests with logging
  run: cargo test
  env:
    RUST_LOG: debug
    
- name: Check binary size (embedded)
  run: |
    cargo build --release
    cargo size --release
    # defmt string boyutunu kontrol et
```

## Performans Ölçümü

```bash
# Logging overhead'ini ölç
cargo bench --bench logging_benchmark

# defmt vs diğer yaklaşımlar
# defmt: ~100 ns per log
# println!: ~10 µs per log
# UART write!: ~1 µs per log
```

> 🦀 **Unutmayın:** 
> - **Web uygulamalarında** `tracing` crate'i ile structured logging ve distributed tracing yapın
> - **Embedded sistemlerde** `defmt` ile sıfır maliyetli, real-time safe loglama yapın
> - **Her zaman** uygun log seviyesini kullanın (trace < debug < info < warn < error)
> - **Hassas verileri** ASLA loglamayın (şifre, kredi kartı, kişisel bilgiler)
> - **Production'da** sadece info+ seviyesini kullanın, debug/trace'ı kapatın
> - **RP2354B projenizde** defmt + RTT ile SWD üzerinden loglama yapın - UART'tan 100x daha hızlı!
> 
> İyi loglama, iyi hata ayıklama demektir. Rust'ın zero-cost abstraction'ları sayesinde, loglamanın performansa etkisi minimumdur. Ama yine de **gereksiz loglardan kaçının** - özellikle embedded sistemlerde her byte ve her cycle değerlidir!