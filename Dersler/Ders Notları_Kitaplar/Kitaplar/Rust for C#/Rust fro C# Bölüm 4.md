# 🔒 Rust for .NET Developers: Kaynak Yönetimi (Resource Management)

Bu bölüm, Rust'ın **kaynak yönetimi felsefesini** ve C#'taki `IDisposable`/`using` deseninin Rust'ta nasıl karşılık bulduğunu inceler. Önceki "Bellek Yönetimi" bölümünde öğrendiğimiz **Ownership** ve **Drop** kavramlarının pratikte nasıl uygulandığını göreceğiz.

> 🎯 **Temel Fark:** C# kaynakları **runtime'da** GC veya manuel `Dispose()` ile serbest bırakır. Rust ise **derleme zamanında** kapsam (scope) tabanlı, deterministik ve **garantili** bir temizlik sağlar.

---

# 📚 BÖLÜM 1: RAII Prensibi (Resource Acquisition Is Initialization)

Rust'ın kaynak yönetimi, C++'tan miras alınan **RAII** prensibine dayanır. Bu prensibin temel fikri şudur:

> **"Kaynak edinimi = Nesne oluşturma. Kaynak serbest bırakma = Nesne yok etme."**

Yani bir kaynağı (dosya, ağ bağlantısı, veritabanı bağlantısı, mutex, vs.) edindiğinizde, bu kaynak bir nesne ile temsil edilir. Nesne kapsam dışına çıktığında kaynak **otomatik** olarak serbest bırakılır.

## 1.1 C# Yaklaşımı: IDisposable + using

**C#**:
```csharp
public class DatabaseConnection : IDisposable
{
    private readonly string _connectionString;
    private SqlConnection _connection;

    public DatabaseConnection(string connectionString) {
        _connectionString = connectionString;
        _connection = new SqlConnection(connectionString);
        _connection.Open();
    }

    public void Dispose()
    {
        _connection.Dispose();  // Manuel temizlik
        Console.WriteLine($"Closing connection: {_connectionString}");
    }
}

// Kullanım
using (var db1 = new DatabaseConnection("Server=A;Database=DB1"))
using (var db2 = new DatabaseConnection("Server=A;Database=DB2"))
{
    // db1 ve db2 ile çalış
} // <- Burada Dispose() OTOMATİK çağrılır
```

## 1.2 Rust Yaklaşımı: Drop Trait

**Rust**:
```rust
struct DatabaseConnection {
    connection_string: &'static str,
    // connection: SqlConnection,
}

impl DatabaseConnection {
    fn new(conn_str: &'static str) -> Self {
        // Bağlantıyı aç
        DatabaseConnection { connection_string: conn_str }
    }
    
    // ... veritabanı işlemleri için metotlar ...
}

impl Drop for DatabaseConnection {
    fn drop(&mut self) {
        // Bağlantıyı kapat
        // self.close_connection();
        println!("Closing connection: {}", self.connection_string);
    }
}

fn main() {
    let _db1 = DatabaseConnection::new("Server=A;Database=DB1");
    let _db2 = DatabaseConnection::new("Server=A;Database=DB2");
    // db1 ve db2 ile çalış
} // <- Burada drop() OTOMATİK çağrılır, LIFO sırasıyla!
```

> 💡 **Kritik Fark:** Rust'ta `using` bloğuna **ihtiyaç yoktur**. Kapsam sonu = otomatik temizlik. Bu, C#'taki `using` unutulma riskini tamamen ortadan kaldırır.

---

# 📚 BÖLÜM 2: Drop Trait - IDisposable'in Rust Karşılığı

## 2.1 Drop Trait Tanımı

```rust
trait Drop {
    fn drop(&mut self);
}
```

- Herhangi bir `struct` veya `enum` için `Drop` trait'ini implemente edebilirsiniz.
- `drop()` metotu **asla manuel çağrılmaz** - derleyici otomatik çağırır.
- `drop()` metotu `&mut self` alır, yani nesneyi değiştirme hakkınız vardır ama **sahipliğini alamazsınız**.

## 2.2 Drop Sırası (LIFO - Son Giren İlk Çıkar)

C# ve Rust'ta `using`/kapsam sonundaki temizlik sırası **aynıdır**:

**C#**:
```csharp
using (var a = new Kaynak("A"))      // 1. oluştur
using (var b = new Kaynak("B"))      // 2. oluştur
using (var c = new Kaynak("C"))      // 3. oluştur
{
    // çalış
} 
// Sıra: C.Dispose() → B.Dispose() → A.Dispose()
```

**Rust**:
```rust
{
    let a = Kaynak::new("A");  // 1. oluştur
    let b = Kaynak::new("B");  // 2. oluştur
    let c = Kaynak::new("C");  // 3. oluştur
    // çalış
}
// Sıra: c.drop() → b.drop() → a.drop()
```

> ⚠️ **Önemli:** Drop sırası **tanımlanma sırasının tersidir** (LIFO). Bu, özellikle bağımlı kaynaklar için kritiktir (örn: transaction, connection'dan önce kapanmalı).

## 2.3 Drop ve Üyelerin Sırası

Bir struct'ın birden fazla üyesi varsa, önce üyeler drop edilir, sonra struct'ın kendisi:

```rust
struct Inner;
impl Drop for Inner {
    fn drop(&mut self) { println!("Inner dropped"); }
}

struct Outer {
    inner1: Inner,
    inner2: Inner,
}
impl Drop for Outer {
    fn drop(&mut self) { println!("Outer dropped"); }
}

fn main() {
    let _o = Outer { inner1: Inner, inner2: Inner };
}
// Çıktı:
// Inner dropped (inner2)
// Inner dropped (inner1)
// Outer dropped
```

---

# 📚 BÖLÜM 3: Erken Drop (Early Drop)

Bazen bir kaynağı kapsam sonundan **önce** serbest bırakmak isteyebilirsiniz.

## 3.1 C#: Manuel Dispose

```csharp
var baglanti = new SqlConnection("...");
baglanti.Open();
// işlemler
baglanti.Dispose();  // Manuel çağrı
// baglanti.Open();  // ❌ ObjectDisposedException (runtime'da)
```

## 3.2 Rust: std::mem::drop

```rust
let baglanti = Baglanti::new("...");
baglanti.ac();
// işlemler
drop(baglanti);  // Manuel drop çağrısı
// baglanti.ac(); // ❌ COMPILE-TIME HATA: "value used here after move"
```

> 🎯 **Kritik Fark:**
> - **C#**: `Dispose()` sonrası kullanım → **Runtime** `ObjectDisposedException`
> - **Rust**: `drop()` sonrası kullanım → **Compile-time** hata!

Rust, `drop()` fonksiyonunu çağırarak değerin sahipliğini alır ve hemen düşürür. Sonrasında o değişkene erişmeye çalışmak **derleyici tarafından engellenir**.

## 3.3 Kapsam ile Erken Drop

Daha temiz bir yöntem, değeri kendi kapsamına almak:

```rust
{
    let baglanti = Baglanti::new("...");
    // işlemler
} // <- Burada otomatik drop edilir

// Buradan sonra baglanti zaten erişilemez
println!("İşlem tamamlandı");
```

---

# 📚 BÖLÜM 4: Compile-Time vs Runtime Garanti ⭐

Bu bölüm, Rust'ın C# karşısındaki **en büyük avantajını** gösterir.

## 4.1 C# - Runtime Hatası

```csharp
public void VeriIsle()
{
    var dosya = new FileStream("data.txt", FileMode.Open);
    dosya.Dispose();
    
    // 100 satır kod sonra...
    
    dosya.ReadByte();  // ❌ ObjectDisposedException - Runtime'da patlar!
}
```

## 4.2 Rust - Compile-Time Hatası

```rust
fn veri_isle() {
    let dosya = File::open("data.txt").unwrap();
    drop(dosya);
    
    // 100 satır kod sonra...
    
    dosya.read(&mut buffer);  // ❌ COMPILE-TIME HATA!
    // error[E0382]: borrow of moved value: `dosya`
}
```

> 💡 **Felsefe:** Rust'ta "mümkün olan her hata, derleme zamanında yakalanmalıdır." Bu, production'da beklenmedik çökmeleri önler.

## 4.3 Karşılaştırma Tablosu

| Durum | C# | Rust |
|---|---|---|
| Dispose sonrası kullanım | ❌ Runtime exception | ❌ Compile-time error |
| Dispose'u unutmak | ⚠️ Mümkün (GC sonradan temizler) | ⚠️ Mümkün değil (otomatik drop) |
| Çift Dispose | ✅ Genelde güvenli (no-op) | ❌ Mümkün değil (sahiplik taşındı) |
| Exception sırasında sızıntı | ⚠️ try-finally gerekli | ✅ Otomatik drop |
| Thread-safe disposal | ⚠️ Manuel kontrol | ✅ Derleyici garantisi |

---

# 📚 BÖLÜM 5: Panic Safety (Drop Sırasında Panic)

## 5.1 Drop Sırasında Panic Tehlikesi

Rust'ta `drop()` metodu içinde `panic!` çağırmak **tehlikelidir**:

```rust
struct Tehlikeli;

impl Drop for Tehlikeli {
    fn drop(&mut self) {
        println!("Dropping...");
        // panic!("Drop sırasında panic!");  // ❌ KÖTÜ FİKİR
    }
}
```

> ⚠️ **Kural:** Eğer bir `drop()` çağrısı sırasında panic olursa ve bu panic zaten başka bir panic'in unwind sürecindeyse, **program anında abort olur** (iki aynı anda panic olamaz).

## 5.2 Güvenli Drop Pratiği

```rust
impl Drop for GuvenliKaynak {
    fn drop(&mut self) {
        // Hata yakalama ile sarmala
        if let Err(e) = self.kapat() {
            eprintln!("Kaynak kapatılamadı: {}", e);
            // panic! YOK - sadece log'la
        }
    }
}
```

## 5.3 C# Finalizer Karşılaştırması

**C#**:
```csharp
~DatabaseConnection()  // Finalizer
{
    // GC thread'inde çalışır
    // Exception fırlatırsanız process çöker!
    Dispose(false);
}
```

**Rust**:
```rust
impl Drop for DatabaseConnection {
    fn drop(&mut self) {
        // Sahip olan thread'de çalışır
        // Deterministik zamanlama
        self.close();
    }
}
```

> 💡 **Kritik Fark:** C# finalizer'ları **belirsiz zamanda**, GC thread'inde çalışır. Rust drop'ları **kesin zamanda**, ilgili thread'de çalışır.

---

# 📚 BÖLÜM 6: Pratik Kaynak Yönetimi Desenleri

## 6.1 Dosya İşlemleri

**C#**:
```csharp
using (var stream = File.OpenRead("data.txt"))
using (var reader = new StreamReader(stream))
{
    var content = reader.ReadToEnd();
}
```

**Rust**:
```rust
use std::fs::File;
use std::io::Read;

fn dosya_oku() -> std::io::Result<String> {
    let mut file = File::open("data.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
} // file otomatik drop edilir
```

## 6.2 Lock/Mutex Yönetimi

**C#**:
```csharp
lock (obje) {
    // kritik bölüm
}
```

**Rust**:
```rust
use std::sync::Mutex;

let mutex = Mutex::new(0);
{
    let mut veri = mutex.lock().unwrap();  // Lock alındı
    *veri += 1;
} // <- Guard burada drop edilir, lock OTOMATİK serbest bırakılır!
```

> 🎯 **RAII Guard Pattern:** Rust'ta `MutexGuard` gibi "guard" tipleri, RAII prensibinin mükemmel bir örneğidir. Guard drop edildiğinde lock otomatik serbest bırakılır.

## 6.3 Ağ Bağlantıları

**C#**:
```csharp
using var client = new TcpClient();
await client.ConnectAsync("host", 8080);
using var stream = client.GetStream();
// ...
```

**Rust**:
```rust
use std::net::TcpStream;

fn baglanti_kur() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("host:8080")?;
    // stream ile çalış
    Ok(())
} // stream otomatik drop edilir, bağlantı kapanır
```

## 6.4 Transaction Yönetimi

**C#**:
```csharp
using var transaction = connection.BeginTransaction();
try {
    // işlemler
    transaction.Commit();
} catch {
    transaction.Rollback();
    throw;
}
```

**Rust**:
```rust
struct Transaction<'a> {
    conn: &'a mut Connection,
    committed: bool,
}

impl<'a> Drop for Transaction<'a> {
    fn drop(&mut self) {
        if !self.committed {
            // Otomatik rollback!
            let _ = self.conn.rollback();
        }
    }
}

impl<'a> Transaction<'a> {
    fn commit(mut self) -> Result<()> {
        self.conn.commit()?;
        self.committed = true;
        Ok(())
    }
}

// Kullanım
let tx = conn.begin_transaction()?;
// işlemler
tx.commit()?;  // Başarılı ise commit
// Başarısız ise tx drop edilir ve otomatik rollback yapılır!
```

> 💡 **Güçlü Desen:** Bu "commit-or-rollback" deseni, Rust'ın Drop trait'i ile çok zarif bir şekilde uygulanabilir. C#'ta bu deseni uygulamak çok daha zordur.

---

# 📚 BÖLÜM 7: Smart Pointer'lar ve Kaynak Yönetimi

## 7.1 Box<T> ile Kaynak Yönetimi

```rust
struct BuyukVeri {
    buffer: Vec<u8>,
}

impl Drop for BuyukVeri {
    fn drop(&mut self) {
        println!("Büyük veri serbest bırakılıyor");
    }
}

fn islem() {
    let veri = Box::new(BuyukVeri { buffer: vec![0; 1_000_000] });
    // işlemler
} // Box drop edilir, içindeki BuyukVeri de drop edilir
```

## 7.2 Rc<T> ve Referans Sayımı

```rust
use std::rc::Rc;

let kaynak = Rc::new(Kaynak::new());
let kopya1 = Rc::clone(&kaynak);  // Ref count: 2
let kopya2 = Rc::clone(&kaynak);  // Ref count: 3

drop(kopya1);  // Ref count: 2
drop(kopya2);  // Ref count: 1
drop(kaynak);  // Ref count: 0 → Kaynak drop edilir!
```

## 7.3 Rc vs Arc Kararı

| Durum | Tercih |
|---|---|
| Tek thread, paylaşılan kaynak | `Rc<T>` |
| Multi-thread, paylaşılan kaynak | `Arc<T>` |
| Değiştirilebilir paylaşılan kaynak | `Rc<RefCell<T>>` veya `Arc<Mutex<T>>` |

---

# 📚 BÖLÜM 8: Embedded Sistemlerde Kaynak Yönetimi 🎯

RP2354B ve step motor projeniz için bu bölüm kritik önem taşır.

## 8.1 Peripheral Yönetimi

Embedded Rust'ta her donanım birimi (GPIO, SPI, I2C, Timer) bir **kaynak** olarak ele alınır:

```rust
// RTIC veya embassy-rs ile
#[embassy_executor::task]
async fn motor_kontrol(mut motor: StepMotor<'static>) {
    loop {
        motor.adim_at().await;
    }
} // motor drop edildiğinde pin'ler otomatik serbest bırakılır
```

## 8.2 no_std Ortamında Drop

```rust
#![no_std]

use core::cell::UnsafeCell;

struct GpioPin {
    pin_no: u8,
}

impl Drop for GpioPin {
    fn drop(&mut self) {
        // Pin'i input moda al (güvenli durum)
        unsafe {
            // Register yazma işlemi
        }
    }
}
```

## 8.3 Interrupt Safe Kaynak Yönetimi

```rust
use critical_section::Mutex;
use core::cell::RefCell;

static MOTOR_STATE: Mutex<RefCell<Option<MotorState>>> = 
    Mutex::new(RefCell::new(None));

#[interrupt]
fn TIMER_IRQ() {
    critical_section::with(|cs| {
        if let Some(mut state) = MOTOR_STATE.borrow_ref_mut(cs).take() {
            state.adim_sayaci += 1;
            // state otomatik drop edilir, kritik bölüm biter
        }
    });
}
```

---

# 📚 BÖLÜM 9: Drop ve Ownership Etkileşimi

## 9.1 Move ve Drop

```rust
struct Kaynak(&'static str);

impl Drop for Kaynak {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

fn main() {
    let k1 = Kaynak("A");
    let k2 = k1;  // Sahiplik taşındı
    // println!("{:?}", k1);  // ❌ HATA
}
// Sadece k2 drop edilir - k1 zaten taşındı!
// Çıktı: Dropping A (sadece 1 kez!)
```

> 💡 **Güzel Özellik:** Rust, sahiplik taşınan değerlerin drop edilmesini **engeller**. Bu, "double free" hatasını önler.

## 9.2 Partial Move

```rust
struct CiftKaynak {
    a: Kaynak,
    b: Kaynak,
}

fn main() {
    let c = CiftKaynak {
        a: Kaynak("A"),
        b: Kaynak("B"),
    };
    
    let a = c.a;  // Partial move - sadece 'a' taşındı
    // c artık tamamen kullanılamaz, ama 'b' hala taşınabilir
    drop(c.b);  // ✅ OK - b hala geçerli
}
```

---

# 📚 BÖLÜM 10: ManuallyDrop ve İleri Düzey Konular

## 10.1 ManuallyDrop<T>

Bazen bir değerin **otomatik drop edilmesini istemezsiniz**:

```rust
use std::mem::ManuallyDrop;

struct OzelKaynak {
    veri: ManuallyDrop<Vec<u8>>,
}

impl Drop for OzelKaynak {
    fn drop(&mut self) {
        // Özel temizlik mantığı
        unsafe {
            ManuallyDrop::drop(&mut self.veri);
        }
    }
}
```

> ⚠️ **Uyarı:** `ManuallyDrop` kullanmak **unsafe** kod gerektirebilir. Sadece gerçekten gerektiğinde kullanın.

## 10.2 Pin ve Self-Referential Structs

```rust
use std::pin::Pin;

struct SelfRef {
    veri: String,
    referans: *const String,  // Kendine referans
}
```

Kendi kendine referans veren struct'lar için `Pin<T>` kullanılır. Bu, değerin bellekte taşınmasını engeller.

---

# 📚 BÖLÜM 11: Pratik Örnek - Veritabanı Bağlantı Havuzu

Gerçek dünya örneği:

```rust
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

struct ConnectionPool {
    connections: Arc<Mutex<VecDeque<Connection>>>,
    max_size: usize,
}

struct PooledConnection {
    conn: Option<Connection>,
    pool: Arc<Mutex<VecDeque<Connection>>>,
}

impl Drop for PooledConnection {
    fn drop(&mut self) {
        // Bağlantıyı kapatma! Havuza geri koy
        if let Some(conn) = self.conn.take() {
            if let Ok(mut pool) = self.pool.lock() {
                if pool.len() < 10 {
                    pool.push_back(conn);
                }
                // Havuz doluysa conn drop edilir ve kapanır
            }
        }
    }
}

impl ConnectionPool {
    fn acquire(&self) -> Option<PooledConnection> {
        let mut pool = self.pool.lock().ok()?;
        let conn = pool.pop_front()?;
        Some(PooledConnection {
            conn: Some(conn),
            pool: Arc::clone(&self.pool),
        })
    }
}

// Kullanım
fn islem(pool: &ConnectionPool) {
    let conn = pool.acquire().unwrap();
    // conn ile çalış
} // conn drop edildiğinde havuza OTOMATİK geri döner!
```

> 🎯 **Güçlü Desen:** Bu "connection pooling" deseni, Drop trait'i sayesinde C#'a göre çok daha güvenlidir. `conn` değişkeni kapsam dışına çıktığında **mutlaka** havuza geri döner.

---

# 📚 BÖLÜM 12: Anti-Patterns ve En İyi Pratikler

## 12.1 ❌ Kötü: Drop'ta Hata Yutma

```rust
impl Drop for KötüKaynak {
    fn drop(&mut self) {
        let _ = self.kapat();  // Hata tamamen yutuldu
    }
}
```

## 12.2 ✅ İyi: Drop'ta Hata Loglama

```rust
impl Drop for IyiKaynak {
    fn drop(&mut self) {
        if let Err(e) = self.kapat() {
            eprintln!("Kaynak kapatılamadı: {}", e);
        }
    }
}
```

## 12.3 ❌ Kötü: Drop'ta Panic

```rust
impl Drop for Tehlikeli {
    fn drop(&mut self) {
        if !self.gecerli {
            panic!("Geçersiz durum!");  // ❌ Program abort olabilir
        }
    }
}
```

## 12.4 ✅ İyi: Result Döndüren Metot

```rust
impl Guvenli {
    fn kapat(self) -> Result<(), Hata> {
        // Açıkça çağrılan metot - hata döndürebilir
        // Drop sadece son çare olarak kullanılır
    }
}

impl Drop for Guvenli {
    fn drop(&mut self) {
        let _ = self.kapat_internal();  // Panic yok
    }
}
```

---

# 🎯 ÖZET: Kaynak Yönetimi Kontrol Listesi

| Durum | C# Yaklaşımı | Rust Yaklaşımı |
|---|---|---|
| Dosya açma | `using var f = File.Open()` | `let f = File::open()?` (otomatik drop) |
| DB bağlantısı | `using var c = new Conn()` | `let c = Conn::new()` (otomatik drop) |
| Lock alma | `lock(obj) { }` | `let g = m.lock().unwrap()` (guard drop = unlock) |
| Transaction | `using var tx = ...` | `let tx = conn.begin()` + Drop ile rollback |
| Manuel erken temizlik | `obj.Dispose()` | `drop(obj)` veya yeni kapsam |
| Dispose sonrası kullanım | ❌ Runtime exception | ❌ Compile-time error |
| Exception safety | `try-finally` gerekli | ✅ Otomatik (drop unwind'da çağrılır) |
| Çift dispose | ✅ No-op | ❌ Mümkün değil (sahiplik) |
| Finalizer | `~Class()` - belirsiz zaman | `Drop::drop()` - deterministik |
| Havuz yönetimi | Manuel iade | Drop ile otomatik iade |

---

# 🚀 Son Tavsiyeler

1. **Drop'u Manuel Çağırmayın:** `drop()` sadece erken serbest bırakma için kullanılır, normal akışta derleyici halleder.
2. **Drop İçinde Panic'ten Kaçının:** Hata loglayın ama panic yapmayın.
3. **RAII Guard Pattern Kullanın:** Lock'lar, transaction'lar ve geçici durumlar için mükemmeldir.
4. **Embedded'de Peripheral Drop'una Dikkat:** Pin'lerin güvenli duruma dönmesi kritiktir.
5. **Smart Pointer'ları Doğru Seçin:** `Rc`/`Arc`/`RefCell`/`Mutex` ihtiyaçlarınıza göre seçin.
6. **`ManuallyDrop`'tan Kaçının:** Sadece gerçekten gerektiğinde, unsafe kod yazmaya hazırsanız kullanın.
7. **Step Motor Projeniz İçin:** RTIC veya embassy-rs kullanarak donanım kaynaklarının otomatik yönetimini sağlayın. Her peripheral'ın drop edilmesi, pin'lerin güvenli duruma dönmesini garanti eder.

> 🦀 **Unutmayın:** Rust'ın kaynak yönetimi, C#'taki `using` bloklarının **unutulma riskini** tamamen ortadan kaldırır. Bir kez `let` ile bağladığınız her kaynak, kapsam sonu geldiğinde **garantili** olarak serbest bırakılır. Bu, özellikle uzun süre çalışan gömülü sistemlerde (RP2354B projeniz gibi) **hayati önem** taşır.