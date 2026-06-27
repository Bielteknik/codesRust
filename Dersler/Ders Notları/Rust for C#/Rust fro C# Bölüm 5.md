# 🧵 Rust for .NET Developers: Threading (İş Parçacığı Yönetimi)

Bu bölüm, C# ve Rust arasındaki **en keskin farklardan birini** inceler: eşzamanlılık (concurrency) ve paralellik (parallelism). C# `Task` ve `async/await` ile soyutlanmış bir model sunarken, Rust **derleme zamanında veri yarışlarını (data race) engelleyen** benzersiz bir yaklaşım sunar.

> 🎯 **Temel Fark:** C# threading hatalarını **runtime'da** (deadlock, race condition) yaşarsınız. Rust ise bu hataların **%99'unu derleme zamanında** yakalar ve kodun çalışmasına izin vermez.

---

# 📚 BÖLÜM 1: Thread Oluşturma (Spawning Threads)

## 1.1 C# Yaklaşımı

**C#**:
```csharp
using System.Threading;

// Thread oluşturma
var thread = new Thread(() => {
    Console.WriteLine($"Thread ID: {Thread.CurrentThread.ManagedThreadId}");
    Thread.Sleep(1000);
    Console.WriteLine("Thread bitti");
});

thread.Start();
thread.Join();  // Thread bitene kadar bekle

// Task ile (modern yaklaşım)
var task = Task.Run(() => {
    Console.WriteLine("Task çalışıyor");
});
task.Wait();
```

## 1.2 Rust Yaklaşımı

**Rust**:
```rust
use std::thread;
use std::time::Duration;

let handle = thread::spawn(|| {
    println!("Thread ID: {:?}", thread::current().id());
    thread::sleep(Duration::from_millis(1000));
    println!("Thread bitti");
});

handle.join().unwrap();  // Thread bitene kadar bekle
```

> 💡 **Benzerlik:** Her iki dilde de thread oluşturmak için bir closure (lambda) kullanılır. `join()` metodu her iki dilde de thread'in bitmesini bekler.

## 1.3 Kritik Fark: Closure Capture

**C#**:
```csharp
var mesaj = "Merhaba";
var thread = new Thread(() => {
    Console.WriteLine(mesaj);  // ✅ Çalışır - GC korur
});
thread.Start();
thread.Join();
// mesaj burada hala geçerli
```

**Rust**:
```rust
let mesaj = String::from("Merhaba");
let handle = thread::spawn(|| {
    // ❌ HATA: `mesaj`'ın ownership'ı thread'e taşınmamış!
    // println!("{}", mesaj);
});

// ✅ Çözüm: move anahtar kelimesi
let handle = thread::spawn(move || {
    println!("{}", mesaj);  // mesaj'ın sahipliği thread'e taşındı
});
handle.join().unwrap();
// println!("{}", mesaj);  // ❌ HATA: mesaj artık bu kapsamda geçersiz
```

> ⚠️ **Kritik Kural:** Rust, thread'ler arasında veri paylaşımını **güvenli** hale getirmek için `move` closure kullanmanızı zorunlu kılar. Bu, thread bittiğinde verinin serbest bırakılmasını garanti eder.

---

# 📚 BÖLÜM 2: Thread-Shared State (Paylaşılan Durum) ⭐

Bu bölüm, Rust'ın **en güçlü** ve **en zorlayıcı** konusudur.

## 2.1 C# Yaklaşımı: lock

**C#**:
```csharp
class Sayac {
    private int _deger = 0;
    private readonly object _lock = new object();
    
    public void Artir() {
        lock (_lock) {
            _deger++;
        }
    }
    
    public int Deger => _deger;
}

var sayac = new Sayac();
var tasks = new List<Task>();
for (int i = 0; i < 10; i++) {
    tasks.Add(Task.Run(() => {
        for (int j = 0; j < 1000; j++) {
            sayac.Artir();
        }
    }));
}
Task.WaitAll(tasks.ToArray());
Console.WriteLine($"Sonuç: {sayac.Deger}");  // 10000
```

## 2.2 Rust Yaklaşımı: Mutex + Arc

**Rust**:
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let sayac = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let sayac_clone = Arc::clone(&sayac);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                let mut deger = sayac_clone.lock().unwrap();
                *deger += 1;
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Sonuç: {}", *sayac.lock().unwrap());  // 10000
}
```

> 🎯 **Kritik Fark:** Rust'ta `Mutex<T>` sadece bir lock değil, **veriyi de içeren** bir yapıdır. Lock'u açmadan veriye erişemezsiniz!

## 2.3 Neden Arc Gerekli?

**Tek başına Mutex yeterli değil:**
```rust
let sayac = Mutex::new(0);

for _ in 0..10 {
    thread::spawn(move || {
        // ❌ HATA: sayac'ın sahipliği ilk thread'e taşındı!
        // Diğer thread'ler sayac'a erişemez
    });
}
```

**Arc (Atomic Reference Counted) ile:**
```rust
let sayac = Arc::new(Mutex::new(0));

for _ in 0..10 {
    let sayac_clone = Arc::clone(&sayac);  // Referans sayısı artar
    thread::spawn(move || {
        // ✅ Tüm thread'ler aynı Mutex'e erişebilir
        let mut deger = sayac_clone.lock().unwrap();
        *deger += 1;
    });
}
// Tüm thread'ler bittiğinde Arc drop edilir, Mutex de drop edilir
```

> 💡 **Arc vs Rc:** `Rc<T>` thread-safe **değildir**. Multi-thread ortamda **mutlaka** `Arc<T>` kullanın.

---

# 📚 BÖLÜM 3: Mutex vs RwLock

## 3.1 Mutex<T> - Karşılıklı Dışlama

- **Tek bir yazıcı** veya **çoklu okuyucu** (ama aynı anda değil)
- Basit ve hızlı
- Çoğu kullanım için yeterli

```rust
use std::sync::Mutex;

let m = Mutex::new(5);
{
    let mut num = m.lock().unwrap();
    *num = 6;
}  // Lock burada otomatik serbest bırakılır (guard drop)
```

## 3.2 RwLock<T> - Okuma-Yazma Kilidi

- **Çoklu okuyucu** aynı anda erişebilir
- **Tek yazıcı** özel erişim alır
- Okuma yoğun işloads için ideal

```rust
use std::sync::RwLock;

let rw = RwLock::new(5);

// Çoklu okuyucu
{
    let r1 = rw.read().unwrap();
    let r2 = rw.read().unwrap();
    println!("r1: {}, r2: {}", *r1, *r2);  // ✅ İkisi de okuyabilir
}

// Tek yazıcı
{
    let mut w = rw.write().unwrap();
    *w = 6;
    // let r3 = rw.read().unwrap();  // ❌ HATA: Yazma kilidi varken okuma yapılamaz
}
```

## 3.3 Karşılaştırma Tablosu

| Özellik | `Mutex<T>` | `RwLock<T>` |
|---|---|---|
| Eşzamanlı okuma | ❌ Hayır | ✅ Evet |
| Eşzamanlı yazma | ❌ Hayır | ❌ Hayır |
| Performans | Hızlı | Okuma yoğun işloads için daha hızlı |
| Karmaşıklık | Basit | Daha karmaşık |
| Kullanım | Genel amaçlı | Okuma > Yazma |

---

# 📚 BÖLÜM 4: Kanallar (Channels) - Message Passing

Rust'ın felsefesi: **"Veriyi paylaşarak iletişim kurma, iletişim kurarak veri paylaş."** (Do not communicate by sharing memory; instead, share memory by communicating.)

## 4.1 C# Yaklaşımı: BlockingCollection

**C#**:
```csharp
var kanal = new BlockingCollection<string>();

// Üretici thread
Task.Run(() => {
    for (int i = 0; i < 5; i++) {
        kanal.Add($"Mesaj {i}");
        Thread.Sleep(100);
    }
    kanal.CompleteAdding();
});

// Tüketici thread
foreach (var mesaj in kanal.GetConsumingEnumerable()) {
    Console.WriteLine($"Alındı: {mesaj}");
}
```

## 4.2 Rust Yaklaşımı: mpsc (Multi-Producer, Single-Consumer)

**Rust**:
```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    // Üretici thread
    thread::spawn(move || {
        for i in 0..5 {
            let mesaj = format!("Mesaj {}", i);
            tx.send(mesaj).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
        // tx burada drop edilir, kanal kapanır
    });
    
    // Tüketici (ana thread)
    for mesaj in rx {
        println!("Alındı: {}", mesaj);
    }
    // rx iteratörü, tx drop edildiğinde otomatik biter
}
```

> 💡 **Kritik Fark:** Rust'ın `mpsc` kanalı **tür güvenliği** sağlar. Kanal üzerinden sadece belirli bir tür gönderilebilir. C#'taki `BlockingCollection<object>` gibi boxing yoktur.

## 4.3 Multi-Producer (Çoklu Üretici)

**Rust**:
```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();

for i in 0..5 {
    let tx_clone = tx.clone();  // Üretici klonla
    thread::spawn(move || {
        tx_clone.send(format!("Thread {} mesajı", i)).unwrap();
    });
}

drop(tx);  // Orijinal tx'i drop et (yoksa rx asla bitmez)

for mesaj in rx {
    println!("Alındı: {}", mesaj);
}
```

## 4.4 Kanallar vs Mutex

| Durum | Tercih |
|---|---|
| Basit veri paylaşımı | `Mutex<T>` |
| Thread'ler arası mesajlaşma | `mpsc::channel()` |
| Çoklu üretici, tek tüketici | `mpsc::channel()` |
| Yüksek performanslı streaming | `crossbeam-channel` crate |
| Async ortam | `tokio::sync::mpsc` |

---

# 📚 BÖLÜM 5: Send ve Sync Trait'leri ⭐⭐

Bu bölüm, Rust'ın **thread safety'yi derleme zamanında** nasıl garanti ettiğini açıklar.

## 5.1 Send Trait

**`Send`**: Bir türün **sahipliğinin thread'ler arasında taşınabileceğini** belirtir.

```rust
// Tüm primitive tipler Send'dir
let x = 42;
thread::spawn(move || println!("{}", x));  // ✅ i32 is Send

// Rc<T> Send DEĞİLDİR
use std::rc::Rc;
let rc = Rc::new(42);
// thread::spawn(move || println!("{}", rc));  // ❌ HATA: Rc<T> is not Send
```

> 💡 **Neden?** `Rc<T>` thread-safe değildir (referans sayımı atomik değildir). Rust bunu derleme zamanında engeller.

## 5.2 Sync Trait

**`Sync`**: Bir türün **referansının (&T) thread'ler arasında güvenle paylaşılabileceğini** belirtir.

```rust
// Tüm primitive tipler Sync'dir
let x = 42;
let r = &x;
thread::spawn(move || println!("{}", r));  // ✅ &i32 is Send (çünkü i32 is Sync)

// RefCell<T> Sync DEĞİLDİR
use std::cell::RefCell;
let rc = RefCell::new(42);
// let r = &rc;
// thread::spawn(move || println!("{}", r.borrow()));  // ❌ HATA
```

## 5.3 Send ve Sync Matrisi

| Tür | Send | Sync | Açıklama |
|---|---|---|---|
| `i32`, `String`, `Vec<T>` | ✅ | ✅ | Temel tipler |
| `&T` (immutable ref) | ✅ | ✅ | Eğer T: Sync ise |
| `&mut T` | ✅ | ✅ | Eğer T: Send ise |
| `Rc<T>` | ❌ | ❌ | Thread-safe değil |
| `Arc<T>` | ✅ | ✅ | Thread-safe |
| `RefCell<T>` | ✅ | ❌ | Runtime borrow check |
| `Mutex<T>` | ✅ | ✅ | Thread-safe |
| `Cell<T>` | ✅ | ❌ | Interior mutability |

## 5.4 Unsafe ile Send/Sync Implementasyonu

Kendi türünüzü thread-safe yapmak isterseniz:

```rust
struct MyType {
    ptr: *mut u8,  // Raw pointer - otomatik olarak !Send ve !Sync
}

// Güvenlik sorumluluğu size ait!
unsafe impl Send for MyType {}
unsafe impl Sync for MyType {}
```

> ⚠️ **Uyarı:** `Send` ve `Sync` implementasyonu **unsafe**'tır. Yanlış implementasyon data race'e yol açar.

---

# 📚 BÖLÜM 6: Scoped Threads (Kapsamlı Thread'ler)

Rust 1.63+ ile gelen güçlü bir özellik.

## 6.1 Problem: Stack Referansları

**Rust (eski yöntem)**:
```rust
let veri = vec![1, 2, 3];
let handle = thread::spawn(|| {
    // ❌ HATA: `veri`'nin lifetime'ı thread'den daha kısa olabilir
    // println!("{:?}", veri);
});
```

**Çözüm (move)**:
```rust
let veri = vec![1, 2, 3];
let handle = thread::spawn(move || {
    println!("{:?}", veri);  // ✅ veri thread'e taşındı
});
handle.join().unwrap();
// veri artık bu kapsamda geçersiz
```

## 6.2 Scoped Threads ile Çözüm

```rust
let veri = vec![1, 2, 3];

thread::scope(|s| {
    s.spawn(|| {
        println!("Thread 1: {:?}", veri);  // ✅ veri ödünç alındı
    });
    s.spawn(|| {
        println!("Thread 2: {:?}", veri);  // ✅ aynı veri paylaşılabilir
    });
});  // Tüm thread'ler burada join edilir

println!("Ana thread: {:?}", veri);  // ✅ veri hala geçerli!
```

> 🎯 **Güçlü Özellik:** `thread::scope`, tüm thread'lerin kapsam sonundan önce biteceğini **garanti eder**. Bu, stack referanslarının güvenli şekilde paylaşılmasını sağlar.

## 6.3 C# Karşılaştırması

C#'ta bu özellik yoktur çünkü GC tüm lifetime sorunlarını çözer. Ama Rust'ta bu, **sıfır maliyetli** bir güvenlik sağlar.

---

# 📚 BÖLÜM 7: Thread Local Storage

## 7.1 C# Yaklaşımı

**C#**:
```csharp
var threadLocal = new ThreadLocal<int>(() => 42);

Task.Run(() => {
    Console.WriteLine($"Thread value: {threadLocal.Value}");
    threadLocal.Value = 100;
});

Console.WriteLine($"Main value: {threadLocal.Value}");  // 42
```

## 7.2 Rust Yaklaşımı

**Rust**:
```rust
use std::cell::RefCell;
use std::thread;

thread_local! {
    static DEGER: RefCell<i32> = RefCell::new(42);
}

thread::spawn(|| {
    DEGER.with(|d| {
        println!("Thread value: {}", d.borrow());
        *d.borrow_mut() = 100;
    });
});

DEGER.with(|d| {
    println!("Main value: {}", d.borrow());  // 42
});
```

> 💡 **Kullanım Alanları:** Request context, logging context, transaction context gibi thread-specific veriler için idealdir.

---

# 📚 BÖLÜM 8: Atomics (Atomik İşlemler)

## 8.1 Neden Atomics?

`Mutex<T>` bazen **aşırı yavaş** olabilir. Basit sayaçlar, flag'ler için atomik işlemler daha uygundur.

## 8.2 C# Yaklaşımı

**C#**:
```csharp
private int _sayac = 0;

// Atomik artırma
Interlocked.Increment(ref _sayac);

// Atomik okuma
var deger = Volatile.Read(ref _sayac);

// Compare-and-swap
Interlocked.CompareExchange(ref _sayac, 10, 5);
```

## 8.3 Rust Yaklaşımı

**Rust**:
```rust
use std::sync::atomic::{AtomicUsize, Ordering};

let sayac = AtomicUsize::new(0);

// Atomik artırma
sayac.fetch_add(1, Ordering::SeqCst);

// Atomik okuma
let deger = sayac.load(Ordering::SeqCst);

// Compare-and-swap
sayac.compare_exchange(5, 10, Ordering::SeqCst, Ordering::SeqCst);
```

## 8.4 Ordering (Sıralama) Kuralları

Rust'ta atomik işlemler **memory ordering** belirtmek zorundadır:

| Ordering | Açıklama | Kullanım |
|---|---|---|
| `SeqCst` | Sıralı tutarlılık (en güçlü) | Varsayılan, en güvenli |
| `Acquire` | Sonraki okumalar/yazmalar bundan önce sıralanır | Lock alma |
| `Release` | Önceki okumalar/yazmalar bundan önce sıralanır | Lock bırakma |
| `Relaxed` | Sıralama garantisi yok | Sayaçlar |

> ⚠️ **Uyarı:** Yanlış ordering kullanımı data race'e yol açabilir. Emin değilseniz `SeqCst` kullanın.

## 8.5 Atomic Türler

| Tür | C# Karşılığı |
|---|---|
| `AtomicBool` | `volatile bool` |
| `AtomicI32`, `AtomicU32` | `volatile int` |
| `AtomicI64`, `AtomicU64` | `volatile long` |
| `AtomicPtr<T>` | `IntPtr` |

---

# 📚 BÖLÜM 9: Rayon - Paralel Iterator'lar

## 9.1 C# Yaklaşımı: PLINQ

**C#**:
```csharp
var sonuc = liste
    .AsParallel()
    .Where(x => x > 5)
    .Select(x => x * 2)
    .Sum();
```

## 9.2 Rust Yaklaşımı: Rayon

**Rust**:
```rust
use rayon::prelude::*;

let sonuc: i32 = vektor
    .par_iter()  // Paralel iterator!
    .filter(|&x| *x > 5)
    .map(|&x| x * 2)
    .sum();
```

> 🎯 **Muhteşem Özellik:** Sadece `.iter()` yerine `.par_iter()` yazarak kodunuzu paralel hale getirirsiniz! Rayon, iş yükünü otomatik olarak thread'lere dağıtır.

## 9.3 Rayon'un Gücü

```rust
use rayon::prelude::*;

// Büyük dosyaları paralel işleme
let dosyalar: Vec<&str> = vec!["a.txt", "b.txt", "c.txt"];

dosyalar
    .par_iter()
    .for_each(|dosya| {
        let icerik = std::fs::read_to_string(dosya).unwrap();
        println!("{}: {} satır", dosya, icerik.lines().count());
    });
```

> 💡 **Data Race Garantisi:** Rayon, Rust'ın `Send` ve `Sync` trait'lerini kullanarak **derleme zamanında** data race olmadığını garanti eder.

---

# 📚 BÖLÜM 10: Async/Await ve Threading İlişkisi

## 10.1 Temel Fark

**C#**: `async/await` otomatik olarak thread pool üzerinde çalışır.
**Rust**: `async/await` sadece bir **Future** üretir. Çalıştırmak için bir **runtime** gerekir.

## 10.2 Tokio ile Async Threading

**Rust**:
```rust
use tokio;

#[tokio::main]
async fn main() {
    // Paralel görevler
    let t1 = tokio::spawn(async {
        println!("Task 1");
    });
    
    let t2 = tokio::spawn(async {
        println!("Task 2");
    });
    
    t1.await.unwrap();
    t2.await.unwrap();
}
```

## 10.3 Async vs Thread Karşılaştırması

| Özellik | `std::thread` | `tokio::spawn` |
|---|---|---|
| Maliyet | Yüksek (MB stack) | Düşük (KB stack) |
| Başlatma süresi | Yavaş | Hızlı |
| Context switch | Yavaş | Hızlı |
| Kullanım | CPU-bound | I/O-bound |
| Ölçeklenebilirlik | Sınırlı (binlerce) | Yüksek (yüz binlerce) |

## 10.4 Spawn Blocking (CPU-Bound İşler)

```rust
#[tokio::main]
async fn main() {
    // CPU-bound işler için
    let sonuc = tokio::task::spawn_blocking(|| {
        // Ağır hesaplama
        fibonacci(40)
    })
    .await
    .unwrap();
    
    println!("Sonuç: {}", sonuc);
}
```

> 💡 **Kural:** Async runtime'da **CPU-bound** işler yapmayın! `spawn_blocking` kullanın veya Rayon'a devredin.

---

# 📚 BÖLÜM 11: Pratik Örnekler

## 11.1 Producer-Consumer Pattern

**Rust**:
```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    // 3 üretici
    for i in 0..3 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            for j in 0..5 {
                tx_clone.send(format!("Üretici {} - Mesaj {}", i, j)).unwrap();
                thread::sleep(Duration::from_millis(100));
            }
        });
    }
    
    drop(tx);  // Orijinal göndericiyi kapat
    
    // 1 tüketici
    for mesaj in rx {
        println!("Alındı: {}", mesaj);
    }
}
```

## 11.2 Thread Pool Pattern

**Rust**:
```rust
use std::sync::{Arc, Mutex};
use std::thread;

struct ThreadPool {
    workers: Vec<Worker>,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        let mut workers = Vec::with_capacity(size);
        
        for id in 0..size {
            workers.push(Worker::new(id));
        }
        
        ThreadPool { workers }
    }
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(move || {
            println!("Worker {} başladı", id);
        });
        
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
```

## 11.3 Step Motor Projesi İçin Threading Stratejisi

RP2354B projeniz için önerilen yapı:

```rust
// RTIC (Real-Time Interrupt-driven Concurrency) framework
#[rtic::app(device = rp235x_hal::pac, dispatchers = [UART0_IRQ])]
mod app {
    use rp235x_hal as hal;
    
    #[shared]
    struct Shared {
        motor_pozisyonu: i32,
    }
    
    #[local]
    struct Local {
        step_pin: hal::gpio::Pin<...>,
        dir_pin: hal::gpio::Pin<...>,
    }
    
    #[task(priority = 3, shared = [motor_pozisyonu], local = [step_pin])]
    fn step_interrupt(cx: step_interrupt::Context) {
        // En yüksek öncelik - step pulse üretimi
        let step_pin = cx.local.step_pin;
        step_pin.set_high().unwrap();
        // Kısa gecikme
        step_pin.set_low().unwrap();
    }
    
    #[task(priority = 2, shared = [motor_pozisyonu])]
    fn hareket_kontrol(cx: hareket_kontrol::Context, hedef: i32) {
        // Orta öncelik - hareket planlaması
        let motor_pozisyonu = cx.shared.motor_pozisyonu;
        (*motor_pozisyonu.lock(|p| *p)) = hedef;
    }
    
    #[task(priority = 1)]
    fn kullanici_arayuzu(cx: kullanici_arayuzu::Context) {
        // Düşük öncelik - UI/komut işleme
    }
}
```

> 🎯 **Önemli:** Embedded sistemlerde **preemptive threading** yerine **priority-based interrupt-driven** yapı tercih edilir. RTIC veya embassy-rs bu konuda mükemmel çözümler sunar.

---

# 📚 BÖLÜM 12: Deadlock Önleme

## 12.1 C# Yaklaşımı

**C#**:
```csharp
// Lock ordering ile deadlock önleme
private readonly object _lock1 = new object();
private readonly object _lock2 = new object();

public void Metod1() {
    lock (_lock1) {
        lock (_lock2) {
            // işlemler
        }
    }
}

public void Metod2() {
    lock (_lock1) {  // Aynı sırayı koru!
        lock (_lock2) {
            // işlemler
        }
    }
}
```

## 12.2 Rust Yaklaşımı: try_lock

**Rust**:
```rust
use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

let m1 = Arc::new(Mutex::new(0));
let m2 = Arc::new(Mutex::new(0));

let m1_clone = Arc::clone(&m1);
let m2_clone = Arc::clone(&m2);

thread::spawn(move || {
    loop {
        if let (Ok(mut v1), Ok(mut v2)) = (m1_clone.try_lock(), m2_clone.try_lock()) {
            *v1 += 1;
            *v2 += 1;
            break;  // Başarılı, döngüden çık
        }
        thread::sleep(Duration::from_millis(10));  // Bekle ve tekrar dene
    }
});
```

> 💡 **Tavsiye:** Mümkünse **tek Mutex** kullanın. Birden fazla lock gerekiyorsa, her zaman **aynı sırada** alın.

---

# 🎯 ÖZET: Threading Kontrol Listesi

| Durum | C# Çözümü | Rust Çözümü |
|---|---|---|
| Thread oluşturma | `new Thread()` / `Task.Run()` | `thread::spawn()` |
| Paylaşılan veri | `lock(obj)` | `Arc<Mutex<T>>` |
| Okuma-yazma kilidi | `ReaderWriterLockSlim` | `Arc<RwLock<T>>` |
| Thread-safe koleksiyon | `ConcurrentDictionary` | `Mutex<HashMap>` veya `DashMap` |
| Mesajlaşma | `BlockingCollection` | `mpsc::channel()` |
| Paralel iterator | `AsParallel()` (PLINQ) | `par_iter()` (Rayon) |
| Async I/O | `async/await` + Task | `async/await` + Tokio |
| Atomik işlemler | `Interlocked` | `AtomicUsize` vb. |
| Thread-local | `ThreadLocal<T>` | `thread_local!` makrosu |
| Deadlock önleme | Lock ordering | `try_lock()` veya lock ordering |
| Embedded threading | Yok (RTOS) | RTIC / embassy-rs |

---

# 🚀 Son Tavsiyeler

1. **Mümkün Olduğunca Message Passing Kullanın:** `mpsc::channel()` ile thread'ler arası iletişim kurun. Paylaşılan durumdan kaçının.

2. **Arc<Mutex<T>> Varsayılanınız Olsun:** Thread-safe veri paylaşımı için en güvenli ve basit yol budur.

3. **Send ve Sync'i Anlayın:** Rust'ın thread safety garantileri bu trait'lere dayanır. Derleyici hata veriyorsa, sorun veri yapınızdadır.

4. **Async vs Thread Kararını Doğru Verin:**
   - I/O-bound (ağ, dosya) → `tokio::spawn`
   - CPU-bound (hesaplama) → `thread::spawn` veya Rayon
   - Embedded → RTIC / embassy-rs

5. **Scoped Threads Kullanın:** Stack referanslarını thread'ler arasında güvenli şekilde paylaşmak için `thread::scope` mükemmeldir.

6. **Atomikleri Doğru Kullanın:** Basit sayaçlar ve flag'ler için `AtomicUsize` kullanın. Karmaşık veri yapıları için `Mutex` tercih edin.

7. **Step Motor Projeniz İçin:** 
   - **RTIC** veya **embassy-rs** kullanın
   - Priority-based interrupt-driven yapı kurun
   - Step pulse üretimi için en yüksek öncelikli interrupt
   - Hareket planlaması için orta öncelik
   - UI/komut işleme için düşük öncelik

8. **`cargo clippy` Kullanın:** Threading hatalarını erken yakalar.

> 🦀 **Unutmayın:** Rust, threading hatalarının **%99'unu derleme zamanında** yakalar. C#'ta runtime'da yaşadığınız deadlock, data race, use-after-free gibi sorunlar Rust'ta **kod derlenmez**. Bu, özellikle real-time sistemlerde (RP2354B projeniz gibi) **hayati önem** taşır. Milisaniyelik hassasiyet ve deterministik davranış, Rust'ın threading modeli ile mümkündür.