# ⚡ Rust for .NET Developers: Asynchronous Programming (Eşzamanlı Programlama)

Bu bölüm, Rust'ın **async/await** modelinin temellerinden ileri düzey eşzamanlılık pattern'lerine kadar kapsamlı bir rehberdir. C# dünyasında `Task`, `async/await`, `Task.WhenAll` ve `IAsyncEnumerable` ile zengin bir async ekosistemine alışkınız. Rust ise **farklı bir felsefe** sunar: Async kod **çalışmaz**, sadece bir "future" üretir - çalıştırmak için bir **runtime** gerekir!

> 🎯 **Temel Fark:** C#'ta async kod **otomatik olarak** .NET ThreadPool üzerinde çalışır. Rust'ta async kod sadece bir **state machine** üretir - onu çalıştırmak için `tokio`, `async-std` veya `embassy-rs` gibi bir **runtime** gerekir. Bu, embedded sistemlerde (RP2354B) **sıfır overhead** ile async kullanmanızı sağlar!

---

# 📚 BÖLÜM 1: Async Programlama Nedir ve Neden Farklı?

## 1.1 C# Async Modeli

```csharp
// C# - Her şey otomatik
public async Task<string> VeriIndirAsync(string url)
{
    using var client = new HttpClient();
    return await client.GetStringAsync(url);
}

// Çalıştırma
var sonuc = await VeriIndirAsync("https://example.com");
```

**Arka planda:**
- .NET runtime otomatik olarak **Task Scheduler** kullanır
- ThreadPool thread'leri üzerinde çalışır
- `SynchronizationContext` ile context yönetimi
- GC ile bellek yönetimi

## 1.2 Rust Async Modeli

```rust
// Rust - Runtime gerekli
async fn veri_indir(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    response.text().await
}

// Çalıştırma - Runtime GEREKLİ!
#[tokio::main]
async fn main() {
    let sonuc = veri_indir("https://example.com").await.unwrap();
}
```

**Arka planda:**
- Async fonksiyon sadece bir **Future** üretir (state machine)
- Çalıştırmak için **executor** (runtime) gerekir
- Bellek yönetimi manuel (ownership/borrowing)
- Thread-safe garantisi derleme zamanında

## 1.3 Kritik Farklar Tablosu

| Özellik | C# | Rust |
|---|---|---|
| Async runtime | .NET ThreadPool (gömülü) | Harici crate (tokio, async-std) |
| Task türü | `Task<T>` | `Future<Output = T>` |
| Başlatma | Otomatik | Manuel (spawn veya await) |
| Thread modeli | 1:1 (Task → Thread) | M:N (Future → Thread pool) |
| Cancellation | `CancellationToken` | `select!` veya drop |
| Context | `SynchronizationContext` | Yok (thread-agnostic) |
| Memory allocation | Heap (Task object) | Stack (state machine) |
| Embedded desteği | ❌ | ✅ (embassy-rs) |
| Zero-cost | ❌ (Task overhead) | ✅ (compile-time) |
| Hot path optimization | Zor | Kolay |

---

# 📚 BÖLÜM 2: Future Trait - Async'in Kalbi ⭐

## 2.1 Future Trait Tanımı

```rust
// Rust standart kütüphanesinden (basitleştirilmiş)
pub trait Future {
    type Output;
    
    // Tek zorunlu metod: poll
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Ready(T),      // Sonuç hazır
    Pending,       // Henüz hazır değil, tekrar dene
}
```

> 💡 **Kritik Nokta:** `Future` **tembeldir (lazy)**. `await` veya `spawn` çağrılmadıkça **hiçbir iş yapmaz**. Bu, C#'taki `Task`'tan çok farklıdır!

## 2.2 C# Task vs Rust Future

**C#**:
```csharp
// Task hemen başlar
var task = VeriIndirAsync("url");  // ✅ İndirme başlar
// ...
await task;  // Sonucu bekle
```

**Rust**:
```rust
// Future hiçbir şey yapmaz!
let future = veri_indir("url");  // ❌ Henüz başlamadı!
// ...
future.await;  // Şimdi başlar ve tamamlanır
```

## 2.3 Future'ı Çalıştırma Yöntemleri

```rust
// Yöntem 1: await ile (mevcut task içinde)
let sonuc = veri_indir("url").await?;

// Yöntem 2: spawn ile (yeni task olarak)
let handle = tokio::spawn(veri_indir("url"));
let sonuc = handle.await??;

// Yöntem 3: join ile (birden fazla future)
let (r1, r2) = tokio::join!(
    veri_indir("url1"),
    veri_indir("url2")
);
```

---

# 📚 BÖLÜM 3: async/await Syntax ⭐

## 3.1 Temel async Fonksiyon

**C#**:
```csharp
public async Task<int> ToplaAsync(int a, int b)
{
    await Task.Delay(100);  // Simüle async iş
    return a + b;
}
```

**Rust**:
```rust
async fn topla(a: i32, b: i32) -> i32 {
    tokio::time::sleep(Duration::from_millis(100)).await;
    a + b
}
```

## 3.2 async Fonksiyonların Gerçek Anlamı

```rust
// Yazdığınız kod:
async fn topla(a: i32, b: i32) -> i32 {
    tokio::time::sleep(Duration::from_millis(100)).await;
    a + b
}

// Derleyicinin ürettiği (basitleştirilmiş):
fn topla(a: i32, b: i32) -> impl Future<Output = i32> {
    async move {
        tokio::time::sleep(Duration::from_millis(100)).await;
        a + b
    }
}
```

> 💡 **Önemli:** `async fn` aslında `impl Future` döndüren bir fonksiyondur!

## 3.3 .await Zincirleme

**C#**:
```csharp
var sonuc = await (await GetirAsync()).IsleAsync();
```

**Rust**:
```rust
let sonuc = getir().await.isle().await;
```

## 3.4 async Bloklar

```rust
// İsimsiz async fonksiyon
let future = async {
    tokio::time::sleep(Duration::from_secs(1)).await;
    42
};

let sonuc = future.await;
println!("Sonuç: {}", sonuc);
```

---

# 📚 BÖLÜM 4: Runtime'lar - Executor Seçimi ⭐⭐

Rust'ta async kodu çalıştırmak için bir **runtime** gerekir. En popüler seçenekler:

## 4.1 Tokio (En Popüler) ⭐

```toml
[dependencies]
tokio = { version = "1.35", features = ["full"] }
```

```rust
use tokio;

#[tokio::main]
async fn main() {
    println!("Merhaba Tokio!");
    
    // Paralel görevler
    let t1 = tokio::spawn(async {
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("Task 1 bitti");
        1
    });
    
    let t2 = tokio::spawn(async {
        tokio::time::sleep(Duration::from_secs(2)).await;
        println!("Task 2 bitti");
        2
    });
    
    let (r1, r2) = tokio::join!(t1, t2);
    println!("Sonuçlar: {:?}, {:?}", r1, r2);
}
```

**Özellikler:**
- ✅ Multi-threaded work-stealing scheduler
- ✅ Zengin ekosistem (tokio-tungstenite, tokio-stream, vs.)
- ✅ Async I/O (TCP, UDP, Unix sockets)
- ✅ Timer ve interval desteği
- ✅ Sync-async köprüsü (`spawn_blocking`)
- ❌ Binary boyutu büyük (~2-3 MB)
- ❌ Öğrenme eğrisi dik

## 4.2 async-std

```toml
[dependencies]
async-std = { version = "1.12", features = ["attributes"] }
```

```rust
use async_std::task;

#[async_std::main]
async fn main() {
    println!("Merhaba async-std!");
    
    let t1 = task::spawn(async {
        task::sleep(Duration::from_secs(1)).await;
        1
    });
    
    let sonuc = t1.await;
    println!("Sonuç: {}", sonuc);
}
```

**Özellikler:**
- ✅ C# async modeline benzer
- ✅ Standart kütüphane benzeri API
- ❌ Tokio kadar popüler değil
- ❌ Daha az üçüncü parti desteği

## 4.3 smol (Minimalist)

```toml
[dependencies]
smol = "2.0"
```

```rust
async fn main() {
    smol::block_on(async {
        println!("Merhaba smol!");
    });
}
```

**Özellikler:**
- ✅ Çok küçük binary boyutu
- ✅ Basit API
- ✅ Minimalist
- ❌ Az özellik
- ❌ Küçük ekosistem

## 4.4 embassy-rs (Embedded için) ⭐⭐⭐

**RP2354B projeniz için EN ÖNEMLİ runtime!**

```toml
[dependencies]
embassy-executor = { version = "0.5", features = ["arch-cortex-m", "executor-thread"] }
embassy-time = "0.3"
embassy-rp = { version = "0.1", features = ["rp235xa"] }
```

```rust
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use defmt::info;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Embassy başlatıldı!");
    
    // Task'ları spawn et
    spawner.spawn(motor_kontrol()).unwrap();
    spawner.spawn(sensor_oku()).unwrap();
    
    // Ana task
    loop {
        info!("Ana döngü");
        Timer::after(Duration::from_secs(1)).await;
    }
}

#[embassy_executor::task]
async fn motor_kontrol() {
    loop {
        info!("Motor kontrol ediliyor");
        Timer::after(Duration::from_millis(10)).await;
    }
}

#[embassy_executor::task]
async fn sensor_oku() {
    loop {
        info!("Sensör okunuyor");
        Timer::after(Duration::from_millis(100)).await;
    }
}
```

**Özellikler:**
- ✅ **no_std** uyumlu
- ✅ **Sıfır heap allocation**
- ✅ **Interrupt-driven** async
- ✅ **Real-time** garantileri
- ✅ RP2040/RP235x için optimize
- ✅ Çok düşük güç tüketimi

## 4.5 Runtime Karşılaştırması

| Runtime | Thread Modeli | Binary Boyutu | Embedded | Kullanım |
|---|---|---|---|---|
| Tokio | Multi-thread | Büyük (~3 MB) | ❌ | Web, backend |
| async-std | Multi-thread | Orta (~1.5 MB) | ❌ | Genel amaçlı |
| smol | Single/Multi | Küçük (~500 KB) | ❌ | Minimalist |
| embassy-rs | Single-thread | Çok küçük (~50 KB) | ✅ | Embedded |

---

# 📚 BÖLÜM 5: Spawn ve JoinHandle ⭐

## 5.1 tokio::spawn

```rust
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() {
    // Yeni task oluştur
    let handle: JoinHandle<i32> = tokio::spawn(async {
        tokio::time::sleep(Duration::from_secs(1)).await;
        42
    });
    
    // Task'ın bitmesini bekle
    match handle.await {
        Ok(sonuc) => println!("Sonuç: {}", sonuc),
        Err(e) => println!("Task hata yaptı: {:?}", e),
    }
}
```

## 5.2 Birden Fazla Task

**C#**:
```csharp
var tasks = new List<Task<int>>();
for (int i = 0; i < 10; i++)
{
    tasks.Add(Task.Run(() => i * i));
}
var sonuclar = await Task.WhenAll(tasks);
```

**Rust**:
```rust
let mut handles = vec![];

for i in 0..10 {
    let handle = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(100)).await;
        i * i
    });
    handles.push(handle);
}

// Tüm task'ları bekle
let sonuclar: Vec<i32> = futures::future::join_all(handles)
    .await
    .into_iter()
    .map(|r| r.unwrap())
    .collect();

println!("Sonuçlar: {:?}", sonuclar);
```

## 5.3 Spawn Blocking (CPU-Bound İşler)

```rust
#[tokio::main]
async fn main() {
    // CPU-bound işler için spawn_blocking kullan
    let handle = tokio::task::spawn_blocking(|| {
        // Ağır hesaplama
        fibonacci(40)
    });
    
    let sonuc = handle.await.unwrap();
    println!("Fibonacci: {}", sonuc);
}

fn fibonacci(n: u64) -> u64 {
    if n <= 1 { return n; }
    fibonacci(n - 1) + fibonacci(n - 2)
}
```

> ⚠️ **Kritik Kural:** Async runtime'da **CPU-bound** işler yapmayın! `spawn_blocking` kullanın. Aksi halde tüm runtime'ı bloklarsınız.

---

# 📚 BÖLÜM 6: async move Closures ⭐⭐

## 6.1 Ownership ve async

```rust
#[tokio::main]
async fn main() {
    let mesaj = String::from("Merhaba");
    
    // ❌ HATA: mesaj'ın ownership'ı closure'a taşınmamış
    // let handle = tokio::spawn(async {
    //     println!("{}", mesaj);
    // });
    
    // ✅ Çözüm: move closure
    let handle = tokio::spawn(async move {
        println!("{}", mesaj);
    });
    
    handle.await.unwrap();
    
    // println!("{}", mesaj);  // ❌ mesaj artık kullanılamaz
}
```

## 6.2 Referans ile Çalışma

```rust
#[tokio::main]
async fn main() {
    let mesaj = String::from("Merhaba");
    
    // Referans kopyala
    let mesaj_ref = &mesaj;
    
    let handle = tokio::spawn(async move {
        println!("{}", mesaj_ref);
    });
    
    handle.await.unwrap();
    
    println!("{}", mesaj);  // ✅ mesaj hala kullanılabilir
}
```

## 6.3 Arc ile Paylaşılan Veri

```rust
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let veri = Arc::new(vec![1, 2, 3, 4, 5]);
    
    let mut handles = vec![];
    
    for i in 0..5 {
        let veri_clone = Arc::clone(&veri);
        let handle = tokio::spawn(async move {
            println!("Task {}: {:?}", i, veri_clone);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.await.unwrap();
    }
}
```

---

# 📚 BÖLÜM 7: Concurrent Pattern'ler ⭐⭐

## 7.1 join! - Tümünü Bekle

**C#**:
```csharp
var t1 = VeriIndirAsync("url1");
var t2 = VeriIndirAsync("url2");
var t3 = VeriIndirAsync("url3");

await Task.WhenAll(t1, t2, t3);
```

**Rust**:
```rust
let (r1, r2, r3) = tokio::join!(
    veri_indir("url1"),
    veri_indir("url2"),
    veri_indir("url3")
);

println!("{:?}, {:?}, {:?}", r1, r2, r3);
```

## 7.2 try_join! - Hata Yönetimi ile

```rust
let sonuc = tokio::try_join!(
    veri_indir("url1"),
    veri_indir("url2"),
    veri_indir("url3")
);

match sonuc {
    Ok((r1, r2, r3)) => println!("Tümü başarılı"),
    Err(e) => println!("Bir hata oluştu: {}", e),
}
```

## 7.3 select! - İlk Biteni Al (Race)

**C#**:
```csharp
var t1 = VeriIndirAsync("url1");
var t2 = VeriIndirAsync("url2");

var tamamlanan = await Task.WhenAny(t1, t2);
var sonuc = await tamamlanan;
```

**Rust**:
```rust
use tokio::select;

let t1 = veri_indir("url1");
let t2 = veri_indir("url2");

tokio::pin!(t1);
tokio::pin!(t2);

let sonuc = select! {
    r1 = &mut t1 => {
        println!("t1 önce bitti");
        r1
    }
    r2 = &mut t2 => {
        println!("t2 önce bitti");
        r2
    }
};
```

## 7.4 Timeout ile select

```rust
use tokio::select;
use tokio::time::{timeout, Duration};

async fn islem() -> Result<String, String> {
    tokio::time::sleep(Duration::from_secs(5)).await;
    Ok("Tamamlandı".to_string())
}

#[tokio::main]
async fn main() {
    let sonuc = select! {
        sonuc = islem() => sonuc,
        _ = tokio::time::sleep(Duration::from_secs(2)) => {
            Err("Timeout!".to_string())
        }
    };
    
    match sonuc {
        Ok(s) => println!("Başarılı: {}", s),
        Err(e) => println!("Hata: {}", e),
    }
}
```

## 7.5 loop + select Pattern

```rust
use tokio::select;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<String>(100);
    
    // Mesaj gönderici
    tokio::spawn(async move {
        for i in 0..5 {
            tx.send(format!("Mesaj {}", i)).await.unwrap();
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    });
    
    // Event loop
    loop {
        select! {
            Some(mesaj) = rx.recv() => {
                println!("Alındı: {}", mesaj);
            }
            _ = tokio::time::sleep(Duration::from_secs(2)) => {
                println!("Timeout - bekleniyor...");
            }
        }
    }
}
```

---

# 📚 BÖLÜM 8: Channels (Kanal) - Message Passing ⭐⭐

## 8.1 mpsc (Multi-Producer, Single-Consumer)

**C#**:
```csharp
var channel = Channel.CreateUnbounded<string>();

// Üretici
_ = Task.Run(async () => {
    for (int i = 0; i < 5; i++) {
        await channel.Writer.WriteAsync($"Mesaj {i}");
        await Task.Delay(100);
    }
    channel.Writer.Complete();
});

// Tüketici
await foreach (var mesaj in channel.Reader.ReadAllAsync()) {
    Console.WriteLine($"Alındı: {mesaj}");
}
```

**Rust**:
```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<String>(100);
    
    // Üretici
    tokio::spawn(async move {
        for i in 0..5 {
            let mesaj = format!("Mesaj {}", i);
            tx.send(mesaj).await.unwrap();
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        // tx drop edildiğinde kanal kapanır
    });
    
    // Tüketici
    while let Some(mesaj) = rx.recv().await {
        println!("Alındı: {}", mesaj);
    }
    
    println!("Kanal kapandı");
}
```

## 8.2 Broadcast Channel

```rust
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (tx, mut rx1) = broadcast::channel::<String>(16);
    let mut rx2 = tx.subscribe();
    
    // Yayınla
    tokio::spawn(async move {
        tx.send("Herkese merhaba!".to_string()).unwrap();
    });
    
    // Her iki alıcı da mesajı alır
    let m1 = rx1.recv().await.unwrap();
    let m2 = rx2.recv().await.unwrap();
    
    println!("rx1: {}", m1);
    println!("rx2: {}", m2);
}
```

## 8.3 oneshot Channel

```rust
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel::<i32>();
    
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(1)).await;
        tx.send(42).unwrap();
    });
    
    let sonuc = rx.await.unwrap();
    println!("Sonuç: {}", sonuc);
}
```

---

# 📚 BÖLÜM 9: Shared State - Arc<Mutex<T>> ⭐⭐

## 9.1 Paylaşılan Durum

**C#**:
```csharp
class Sayac
{
    private int _deger = 0;
    private readonly object _lock = new();
    
    public void Artir()
    {
        lock (_lock)
        {
            _deger++;
        }
    }
    
    public int Deger => _deger;
}

var sayac = new Sayac();
var tasks = Enumerable.Range(0, 10)
    .Select(_ => Task.Run(() => {
        for (int i = 0; i < 1000; i++)
            sayac.Artir();
    }));
await Task.WhenAll(tasks);
Console.WriteLine($"Sonuç: {sayac.Deger}");  // 10000
```

**Rust**:
```rust
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let sayac = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let sayac_clone = Arc::clone(&sayac);
        let handle = tokio::spawn(async move {
            for _ in 0..1000 {
                let mut deger = sayac_clone.lock().unwrap();
                *deger += 1;
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.await.unwrap();
    }
    
    let sonuc = *sayac.lock().unwrap();
    println!("Sonuç: {}", sonuc);  // 10000
}
```

## 9.2 MutexGuard ve RAII

```rust
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let veri = Arc::new(Mutex::new(Vec::new()));
    
    let veri_clone = Arc::clone(&veri);
    tokio::spawn(async move {
        // Lock al
        let mut v = veri_clone.lock().unwrap();
        v.push(1);
        v.push(2);
        v.push(3);
        // v burada drop edilir, lock otomatik serbest bırakılır!
    }).await.unwrap();
    
    let sonuc = veri.lock().unwrap().clone();
    println!("Veri: {:?}", sonuc);
}
```

## 9.3 RwLock (Okuma-Yazma Kilidi)

```rust
use std::sync::{Arc, RwLock};

#[tokio::main]
async fn main() {
    let veri = Arc::new(RwLock::new(String::from("Merhaba")));
    
    // Çoklu okuyucu
    let r1 = Arc::clone(&veri);
    let r2 = Arc::clone(&veri);
    
    tokio::spawn(async move {
        let okuma = r1.read().unwrap();
        println!("Okuyucu 1: {}", *okuma);
    });
    
    tokio::spawn(async move {
        let okuma = r2.read().unwrap();
        println!("Okuyucu 2: {}", *okuma);
    });
    
    // Tek yazıcı
    let w = Arc::clone(&veri);
    tokio::spawn(async move {
        let mut yazma = w.write().unwrap();
        *yazma = "Dünya".to_string();
    }).await.unwrap();
}
```

---

# 📚 BÖLÜM 10: Async Trait'ler ⭐⭐

## 10.1 Problem

```rust
// ❌ Bu çalışmaz (Rust 1.75 öncesi)
trait Veritabani {
    async fn kayit_getir(&self, id: i32) -> Option<Kayit>;
}
```

## 10.2 async_trait Crate (Eski Yöntem)

```toml
[dependencies]
async-trait = "0.1"
```

```rust
use async_trait::async_trait;

#[async_trait]
trait Veritabani {
    async fn kayit_getir(&self, id: i32) -> Option<Kayit>;
    async fn kayit_olustur(&self, kayit: Kayit) -> Result<(), Hata>;
}

struct PostgresDB;

#[async_trait]
impl Veritabani for PostgresDB {
    async fn kayit_getir(&self, id: i32) -> Option<Kayit> {
        // Async sorgu
        Some(Kayit { id, ad: "Test".to_string() })
    }
    
    async fn kayit_olustur(&self, kayit: Kayit) -> Result<(), Hata> {
        Ok(())
    }
}
```

## 10.3 Native Async Trait (Rust 1.75+)

```rust
// Rust 1.75+ ile native destek
trait Veritabani {
    async fn kayit_getir(&self, id: i32) -> Option<Kayit>;
}

struct PostgresDB;

impl Veritabani for PostgresDB {
    async fn kayit_getir(&self, id: i32) -> Option<Kayit> {
        Some(Kayit { id, ad: "Test".to_string() })
    }
}
```

## 10.4 Async Trait Object

```rust
use std::sync::Arc;

trait Servis {
    async fn calistir(&self) -> Result<(), Hata>;
}

// Async trait object kullanımı
async fn islem_yap(servis: &(dyn Servis + Send + Sync)) {
    servis.calistir().await.unwrap();
}

// Veya Arc ile
async fn islem_yap_arc(servis: Arc<dyn Servis + Send + Sync>) {
    servis.calistir().await.unwrap();
}
```

---

# 📚 BÖLÜM 11: Stream Trait (Async Iterator) ⭐⭐

## 11.1 Stream Nedir?

`Stream`, async iterator'dür - zamanla değer üreten async bir sekans.

```toml
[dependencies]
futures = "0.3"
tokio-stream = "0.1"
```

## 11.2 Stream Oluşturma

```rust
use tokio_stream::StreamExt;
use tokio::time::{interval, Duration};

#[tokio::main]
async fn main() {
    // Her 1 saniyede bir değer üreten stream
    let mut stream = interval(Duration::from_secs(1));
    
    for _ in 0..5 {
        let _ = stream.next().await;
        println!("Tick!");
    }
}
```

## 11.3 C# IAsyncEnumerable vs Rust Stream

**C#**:
```csharp
public async IAsyncEnumerable<int> SayilarAsync()
{
    for (int i = 0; i < 5; i++)
    {
        await Task.Delay(100);
        yield return i;
    }
}

await foreach (var sayi in SayilarAsync())
{
    Console.WriteLine(sayi);
}
```

**Rust**:
```rust
use futures::stream::{self, StreamExt};
use tokio::time::{Duration, sleep};

async fn sayilar() -> impl futures::Stream<Item = i32> {
    stream::iter(0..5).then(|i| async move {
        sleep(Duration::from_millis(100)).await;
        i
    })
}

#[tokio::main]
async fn main() {
    let mut stream = Box::pin(sayilar());
    
    while let Some(sayi) = stream.next().await {
        println!("{}", sayi);
    }
}
```

## 11.4 Stream Adapter'ları

```rust
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let stream = tokio_stream::iter(vec![1, 2, 3, 4, 5]);
    
    // map
    let kareler = stream.map(|x| x * x);
    
    // filter
    let ciftler = kareler.filter(|x| x % 2 == 0);
    
    // collect
    let sonuc: Vec<i32> = ciftler.collect().await;
    println!("{:?}", sonuc);  // [4, 16]
}
```

## 11.5 Custom Stream

```rust
use futures::stream::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};

struct SayacStream {
    mevcut: i32,
    maksimum: i32,
}

impl Stream for SayacStream {
    type Item = i32;
    
    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.mevcut < self.maksimum {
            let deger = self.mevcut;
            self.mevcut += 1;
            Poll::Ready(Some(deger))
        } else {
            Poll::Ready(None)
        }
    }
}

#[tokio::main]
async fn main() {
    use futures::StreamExt;
    let mut stream = SayacStream { mevcut: 0, maksimum: 5 };
    
    while let Some(deger) = stream.next().await {
        println!("{}", deger);
    }
}
```

---

# 📚 BÖLÜM 12: Async I/O ⭐

## 12.1 Async Dosya İşlemleri

```rust
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Yazma
    let mut dosya = File::create("test.txt").await?;
    dosya.write_all(b"Merhaba Dünya!").await?;
    
    // Okuma
    let mut dosya = File::open("test.txt").await?;
    let mut icerik = String::new();
    dosya.read_to_string(&mut icerik).await?;
    
    println!("İçerik: {}", icerik);
    Ok(())
}
```

## 12.2 Async HTTP

```rust
use reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    
    let response = client
        .get("https://api.example.com/data")
        .send()
        .await?;
    
    let body = response.text().await?;
    println!("Body: {}", body);
    
    Ok(())
}
```

## 12.3 Async TCP Server

```rust
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server 8080'de dinliyor");
    
    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("Yeni bağlantı: {}", addr);
        
        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            
            loop {
                let n = match socket.read(&mut buffer).await {
                    Ok(n) if n == 0 => return,  // Bağlantı kapandı
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("Hata: {}", e);
                        return;
                    }
                };
                
                // Echo back
                if let Err(e) = socket.write_all(&buffer[0..n]).await {
                    eprintln!("Yazma hatası: {}", e);
                    return;
                }
            }
        });
    }
}
```

---

# 📚 BÖLÜM 13: Cancellation (İptal) ⭐⭐

## 13.1 Drop ile İptal

Rust'ta future'lar **drop edildiğinde otomatik olarak iptal edilir**.

```rust
#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        loop {
            println!("Çalışıyor...");
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });
    
    // 3 saniye bekle
    tokio::time::sleep(Duration::from_secs(3)).await;
    
    // Task'ı iptal et (drop et)
    handle.abort();
    
    println!("Task iptal edildi");
}
```

## 13.2 CancellationToken Benzeri Pattern

```rust
use tokio::sync::watch;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = watch::channel(false);
    
    // Uzun süren iş
    let handle = tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = async { 
                    tokio::time::sleep(Duration::from_millis(100)).await;
                } => {
                    println!("Çalışıyor...");
                }
                _ = async {
                    while !*rx.borrow_and_update() {
                        rx.changed().await.unwrap();
                    }
                } => {
                    println!("İptal sinyali alındı");
                    break;
                }
            }
        }
    });
    
    // 2 saniye sonra iptal et
    tokio::time::sleep(Duration::from_secs(2)).await;
    tx.send(true).unwrap();
    
    handle.await.unwrap();
}
```

---

# 📚 BÖLÜM 14: Sync-Async Köprüsü ⭐

## 14.1 Blocking Kodu Async İçinde Çalıştırma

```rust
#[tokio::main]
async fn main() {
    // Senkron kodu async runtime'da çalıştırma
    let sonuc = tokio::task::spawn_blocking(|| {
        // CPU-bound veya blocking I/O
        std::thread::sleep(Duration::from_secs(2));
        42
    })
    .await
    .unwrap();
    
    println!("Sonuç: {}", sonuc);
}
```

## 14.2 Async Kodu Sync İçinde Çalıştırma

```rust
fn sync_fonksiyon() {
    // Async runtime başlat
    let sonuc = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async {
            async_islem().await
        });
    
    println!("Sonuç: {}", sonuc);
}

async fn async_islem() -> i32 {
    42
}
```

## 14.3 block_in_place

```rust
#[tokio::main]
async fn main() {
    // Mevcut thread'de blocking iş yap
    tokio::task::block_in_place(|| {
        // Blocking I/O
        std::fs::read_to_string("dosya.txt").unwrap();
    });
}
```

---

# 📚 BÖLÜM 15: Embedded Async - embassy-rs (RP2354B) ⭐⭐⭐

**Step motor projeniz için EN ÖNEMLİ bölüm!**

## 15.1 Neden Embedded Async?

```
Geleneksel Embedded (RTIC):
├─ Interrupt-driven
├─ Priority-based preemptive
├─ Karmaşık state management
└─ Kod okunabilirliği düşük

Embassy Async:
├─ Cooperative multitasking
├─ Zero heap allocation
├─ Basit async/await syntax
├─ Kolay state management
└─ Deterministik davranış
```

## 15.2 Temel embassy-rs Yapısı

```rust
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::{Duration, Timer};
use defmt::info;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // RP2354B peripherals'ı başlat
    let p = embassy_rp::init(Default::default());
    
    info!("Embassy başlatıldı!");
    
    // LED task'ı spawn et
    let led = Output::new(p.PIN_25, Level::Low);
    spawner.spawn(led_task(led)).unwrap();
    
    // Motor task'ı spawn et
    spawner.spawn(motor_task()).unwrap();
    
    // Ana döngü
    loop {
        info!("Ana döngü çalışıyor");
        Timer::after(Duration::from_secs(5)).await;
    }
}

#[embassy_executor::task]
async fn led_task(mut led: Output<'static>) {
    loop {
        led.set_high();
        Timer::after(Duration::from_millis(500)).await;
        led.set_low();
        Timer::after(Duration::from_millis(500)).await;
    }
}

#[embassy_executor::task]
async fn motor_task() {
    loop {
        info!("Motor kontrol ediliyor");
        Timer::after(Duration::from_millis(10)).await;
    }
}
```

## 15.3 Step Motor Kontrolü

```rust
use embassy_rp::gpio::{Level, Output};
use embassy_time::{Duration, Timer, Instant};

struct StepMotor {
    step_pin: Output<'static>,
    dir_pin: Output<'static>,
    pozisyon: i32,
}

impl StepMotor {
    fn new(step: Output<'static>, dir: Output<'static>) -> Self {
        Self {
            step_pin: step,
            dir_pin: dir,
            pozisyon: 0,
        }
    }
    
    async fn adim_at(&mut self) {
        self.step_pin.set_high();
        Timer::after_micros(5).await;  // Minimum pulse width
        self.step_pin.set_low();
        Timer::after_micros(5).await;
    }
    
    async fn hareket_et(&mut self, hedef: i32, hiz_rpm: u16) {
        let adim_sayisi = (hedef - self.pozisyon).abs();
        let yon = if hedef > self.pozisyon { 1 } else { -1 };
        
        // Yön ayarla
        if yon > 0 {
            self.dir_pin.set_high();
        } else {
            self.dir_pin.set_low();
        }
        
        // Adım başına süre (mikrosaniye)
        let adim_suresi_us = 60_000_000 / (hiz_rpm as u64 * 200);
        
        for _ in 0..adim_sayisi {
            self.adim_at().await;
            self.pozisyon += yon;
            Timer::after_micros(adim_suresi_us).await;
        }
    }
}

#[embassy_executor::task]
async fn motor_kontrol_task() {
    let p = embassy_rp::init(Default::default());
    
    let step_pin = Output::new(p.PIN_20, Level::Low);
    let dir_pin = Output::new(p.PIN_21, Level::Low);
    
    let mut motor = StepMotor::new(step_pin, dir_pin);
    
    loop {
        // 1000 adım ileri
        info!("İleri hareket");
        motor.hareket_et(1000, 500).await;
        
        Timer::after(Duration::from_secs(1)).await;
        
        // 500 adım geri
        info!("Geri hareket");
        motor.hareket_et(500, 300).await;
        
        Timer::after(Duration::from_secs(1)).await;
    }
}
```

## 15.4 Concurrent Task Pattern'ları

```rust
use embassy_sync::channel::Channel;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;

// Task'lar arası iletişim
static KOMUT_KANALI: Channel<ThreadModeRawMutex, MotorKomutu, 10> = Channel::new();

enum MotorKomutu {
    Hareket { hedef: i32, hiz: u16 },
    Durdur,
    AcilDurdur,
}

#[embassy_executor::task]
async fn kullanici_arayuzu_task() {
    // UART veya USB'den komutları al
    loop {
        // Kullanıcı girdisi simülasyonu
        Timer::after(Duration::from_secs(5)).await;
        
        let komut = MotorKomutu::Hareket {
            hedef: 1000,
            hiz: 500,
        };
        
        KOMUT_KANALI.send(komut).await;
    }
}

#[embassy_executor::task]
async fn motor_executor_task() {
    let mut motor = motor_init();
    
    loop {
        let komut = KOMUT_KANALI.receive().await;
        
        match komut {
            MotorKomutu::Hareket { hedef, hiz } => {
                info!("Hareket: hedef={}, hiz={}", hedef, hiz);
                motor.hareket_et(hedef, hiz).await;
            }
            MotorKomutu::Durdur => {
                info!("Motor durduruldu");
            }
            MotorKomutu::AcilDurdur => {
                info!("ACİL DURDURMA!");
                motor.acil_durdur();
            }
        }
    }
}

#[embassy_executor::task]
async fn sensor_monitor_task() {
    loop {
        let sicaklik = sensor_oku();
        
        if sicaklik > 80.0 {
            KOMUT_KANALI.send(MotorKomutu::AcilDurdur).await;
        }
        
        Timer::after(Duration::from_millis(100)).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    spawner.spawn(kullanici_arayuzu_task()).unwrap();
    spawner.spawn(motor_executor_task()).unwrap();
    spawner.spawn(sensor_monitor_task()).unwrap();
}
```

## 15.5 Interrupt-Driven Async

```rust
use embassy_rp::bind_interrupts;
use embassy_rp::uart::{Uart, InterruptHandler};
use embassy_rp::peripherals::UART0;

bind_interrupts!(struct Irqs {
    UART0_IRQ => InterruptHandler<UART0>;
});

#[embassy_executor::task]
async fn uart_task() {
    let p = embassy_rp::init(Default::default());
    
    let config = embassy_rp::uart::Config::default();
    let mut uart = Uart::new(p.UART0, p.PIN_0, p.PIN_1, Irqs, p.DMA_CH0, p.DMA_CH1, config);
    
    let mut buffer = [0u8; 64];
    
    loop {
        // Async UART okuma - interrupt-driven
        let n = uart.read(&mut buffer).await.unwrap();
        
        let komut = core::str::from_utf8(&buffer[..n]).unwrap();
        info!("UART alındı: {}", komut);
        
        // Komutu işle
        match komut.trim() {
            "MOVE 1000" => { /* ... */ }
            "STOP" => { /* ... */ }
            _ => { /* ... */ }
        }
    }
}
```

---

# 📚 BÖLÜM 16: Async Best Practices ⭐⭐

## 16.1 ✅ İyi Pratikler

### 1. I/O-bound vs CPU-bound Ayrımı

```rust
// ✅ İYİ: I/O-bound işler için async
async fn veri_indir(url: &str) -> Result<String, Error> {
    reqwest::get(url).await?.text().await
}

// ✅ İYİ: CPU-bound işler için spawn_blocking
async fn agir_hesaplama(veri: Vec<u8>) -> Vec<u8> {
    tokio::task::spawn_blocking(move || {
        // CPU-bound hesaplama
        fibonacci(40);
        veri
    })
    .await
    .unwrap()
}
```

### 2. Mutex'i Kısa Süre Tut

```rust
// ❌ KÖTÜ: Uzun süreli lock
async fn kotu() {
    let mut data = mutex.lock().await;
    tokio::time::sleep(Duration::from_secs(1)).await;  // Lock altında await!
    data.push(1);
}

// ✅ İYİ: Kısa süreli lock
async fn iyi() {
    {
        let mut data = mutex.lock().await;
        data.push(1);
    }  // Lock burada serbest bırakılır
    tokio::time::sleep(Duration::from_secs(1)).await;
}
```

### 3. Clone Yerine Arc Kullan

```rust
// ❌ KÖTÜ: Büyük veriyi clone'la
async fn islem(veri: Vec<u8>) {
    // ...
}

tokio::spawn(islem(buyuk_veri.clone()));  // Pahalı!

// ✅ İYİ: Arc ile paylaş
let veri = Arc::new(buyuk_veri);
tokio::spawn(islem(Arc::clone(&veri)));  // Ucuz!
```

### 4. Timeout Kullan

```rust
// ✅ İYİ: Her async işlemde timeout
async fn guvenli_islem() -> Result<(), Error> {
    tokio::time::timeout(Duration::from_secs(5), async {
        // Uzun sürebilecek işlem
        veri_indir("url").await
    })
    .await
    .map_err(|_| Error::Timeout)?
}
```

## 16.2 ❌ Anti-Patterns

### 1. Async Fonksiyonda Blocking Çağrı

```rust
// ❌ KÖTÜ: Async içinde blocking
async fn kotu() {
    std::thread::sleep(Duration::from_secs(1));  // Tüm runtime'ı bloklar!
    std::fs::read_to_string("dosya.txt").unwrap();  // Blocking I/O!
}

// ✅ İYİ: Async versiyonları kullan
async fn iyi() {
    tokio::time::sleep(Duration::from_secs(1)).await;
    tokio::fs::read_to_string("dosya.txt").await.unwrap();
}
```

### 2. Çok Fazla Spawn

```rust
// ❌ KÖTÜ: Her istek için yeni task
async fn handle_request(req: Request) {
    tokio::spawn(async move {
        // Küçük bir işlem için spawn gereksiz
        process(req).await
    });
}

// ✅ İYİ: Mevcut task'ta çalıştır
async fn handle_request(req: Request) {
    process(req).await;
}
```

### 3. Unwrap Kullanımı

```rust
// ❌ KÖTÜ
async fn kotu() {
    let data = mutex.lock().await.unwrap();  // Panic!
    let sonuc = veri_indir("url").await.unwrap();  // Panic!
}

// ✅ İYİ
async fn iyi() -> Result<(), Error> {
    let data = mutex.lock().await.map_err(|_| Error::Lock)?;
    let sonuc = veri_indir("url").await?;
    Ok(())
}
```

---

# 📚 BÖLÜM 17: Performans ve Ölçüm ⭐

## 17.1 Async Overhead Ölçümü

```rust
use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

fn bench_sync(c: &mut Criterion) {
    c.bench_function("sync", |b| {
        b.iter(|| {
            let mut toplam = 0;
            for i in 0..1000 {
                toplam += i;
            }
            toplam
        })
    });
}

fn bench_async(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    c.bench_function("async", |b| {
        b.iter(|| {
            rt.block_on(async {
                let mut toplam = 0;
                for i in 0..1000 {
                    toplam += i;
                }
                toplam
            })
        })
    });
}

criterion_group!(benches, bench_sync, bench_async);
criterion_main!(benches);
```

## 17.2 Task Switching Overhead

```rust
use tokio::task;

#[tokio::main]
async fn main() {
    let start = std::time::Instant::now();
    
    let mut handles = vec![];
    for _ in 0..10_000 {
        handles.push(task::spawn(async {
            tokio::task::yield_now().await;
        }));
    }
    
    for handle in handles {
        handle.await.unwrap();
    }
    
    println!("10000 task: {:?}", start.elapsed());
    // Tipik: ~10-50ms
}
```

---

# 🎯 ÖZET: Async Programlama Kontrol Listesi

| Özellik | C# | Rust |
|---|---|---|
| Async fonksiyon | `async Task<T>` | `async fn -> T` |
| Await | `await` | `.await` |
| Runtime | .NET ThreadPool (gömülü) | Harici (tokio, embassy-rs) |
| Task türü | `Task<T>` | `Future<Output = T>` |
| Spawn | `Task.Run()` | `tokio::spawn()` |
| Concurrent | `Task.WhenAll()` | `tokio::join!()` |
| Race | `Task.WhenAny()` | `tokio::select!` |
| Channels | `Channel<T>` | `tokio::sync::mpsc` |
| Shared state | `lock` | `Arc<Mutex<T>>` |
| Cancellation | `CancellationToken` | Drop veya `watch` channel |
| Async iterator | `IAsyncEnumerable<T>` | `Stream<Item = T>` |
| Async trait | Native | Native (1.75+) veya `async_trait` |
| Embedded | ❌ | ✅ (embassy-rs) |
| Zero-cost | ❌ | ✅ |

---

# 🚀 Son Tavsiyeler

## Web/Backend Uygulamaları İçin

1. **Tokio Kullanın:** En olgun ve geniş ekosisteme sahip
2. **spawn_blocking Kullanın:** CPU-bound işler için
3. **Arc<Mutex<T>> ile Paylaşın:** Thread-safe shared state için
4. **Timeout Ekleyin:** Her async işlemde timeout kullanın
5. **CancellationToken Yerine select!:** İptal için
6. **Channels Kullanın:** Task'lar arası iletişim için

## Embedded Sistemler (RP2354B) İçin

1. **embassy-rs Kullanın:** Sıfır overhead, real-time safe
2. **Task'ları Küçük Tutun:** Her task tek bir sorumluluk
3. **Channel ile İletişim:** Task'lar arası message passing
4. **Timer ile Bekleme:** `Timer::after()` kullanın
5. **Interrupt-Driven I/O:** UART, SPI, I2C için async kullanın
6. **Critical Section Kullanın:** Paylaşılan veriler için

## Step Motor Projeniz İçin Async Stratejisi

```rust
// Task yapısı
├─ main_task
│  ├─ Motor init
│  └─ Diğer task'ları spawn et
├─ motor_executor_task
│  ├─ KOMUT_KANALI'ndan komut al
│  ├─ Hareket planlaması yap
│  └─ Step pulse üret
├─ sensor_monitor_task
│  ├─ Sensörleri oku (ADC, encoder)
│  ├─ Sicaklık kontrolü
│  └─ Hata durumunda ACİL DURDUR
├─ communication_task
│  ├─ UART/USB'den komut al
│  ├─ Durum bilgisi gönder
│  └─ Protokol işleme
└─ watchdog_task
   ├─ Sistem sağlığını kontrol et
   └─ Reset gerekirse reset at
```

## Async Kod Şablonu

```rust
// Temiz async yapı
async fn temiz_async() -> Result<(), Error> {
    // 1. Kaynakları hazırla
    let veri = Arc::new(veri_yukle().await?);
    
    // 2. Task'ları spawn et
    let mut handles = vec![];
    for i in 0..10 {
        let veri_clone = Arc::clone(&veri);
        handles.push(tokio::spawn(async move {
            islem_yap(veri_clone, i).await
        }));
    }
    
    // 3. Sonuçları topla
    let sonuclar = futures::future::join_all(handles).await;
    
    // 4. Hataları kontrol et
    for sonuc in sonuclar {
        sonuc??;  // JoinError ve iç hata
    }
    
    Ok(())
}
```

## Performans İpuçları

1. **Future'ları Küçük Tutun:** Büyük future'lar stack'te çok yer kaplar
2. **`.await` Noktalarını Azaltın:** Her await bir context switch
3. **Box::pin Kullanın:** Büyük future'lar için
4. **spawn_local ile Single-Thread:** Tek thread executor için
5. **tokio::select! ile Multiplexing:** Birden fazla future'ı aynı anda dinle

## Debugging İpuçları

```rust
// 1. tokio-console kullan
// Cargo.toml
[dependencies]
console-subscriber = "0.2"

// Kod
#[tokio::main]
async fn main() {
    console_subscriber::init();
    // ...
}

// Çalıştır
RUSTFLAGS="--cfg tokio_unstable" cargo run

// 2. tracing kullan
use tracing::{info, instrument};

#[instrument]
async fn islem() {
    info!("İşlem başladı");
    // ...
}
```

> 🦀 **Unutmayın:**
> - **Rust async**, C# async'den **temelde farklıdır** - runtime seçimi kritiktir
> - **Future'lar tembeldir** - await veya spawn olmadan çalışmazlar
> - **Tokio** web/backend için, **embassy-rs** embedded için en iyi seçimdir
> - **spawn_blocking** kullanarak CPU-bound işleri async runtime'dan ayırın
> - **Arc<Mutex<T>>** ile thread-safe shared state yönetin
> - **select!** ile concurrent pattern'leri zarif bir şekilde uygulayın
> - **RP2354B projenizde** embassy-rs ile interrupt-driven async kullanın
> - **Zero-cost abstraction** sayesinde async kod, senkron kod kadar hızlıdır
> 
> Rust'ın async modeli, başta karmaşık gelse de, bir kez anladığınızda C#'ta asla ulaşamayacağınız **performans ve kontrol** sağlar. Özellikle embedded sistemlerde, async/await syntax'ının sağladığı okunabilirliği, sıfır maliyetle kullanabilirsiniz! Step motor projenizde embassy-rs ile hem gerçek zamanlı davranış hem de temiz kod yazabilirsiniz. 🚀
```