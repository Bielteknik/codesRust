# 🦀 Ders Notları: Bölüm 21 - 🎓 Rust ile Sıfırdan Web Sunucusu Geliştirme — Bölüm 21 Detaylı Ders

Hoş geldiniz! Bu dersimizde, Rust Kitabı'nın final projesi olan **"Bir Web Sunucusu Geliştirme"** bölümünü satır satır, ders anlatır gibi inceleyeceğiz. Bu bölüm, kitabın önceki bölümlerinde öğrendiğiniz birçok kavramı (thread'ler, channel'lar, trait'ler, closure'lar, smart pointer'lar vb.) tek bir projede birleştirir.

---

## 📋 Genel Bakış: Ne Yapacağız?

Bu projede tarayıcıda **"Hello!"** yazan basit ama tam işlevsel bir web sunucusu geliştireceğiz. Yol haritamız şu 5 adımdan oluşuyor:

| Adım | Konu | Ne Öğreneceğiz? |
|------|------|-----------------|
| 1 | TCP ve HTTP temelleri | Web'in altında yatan protokoller |
| 2 | TCP bağlantı dinleme | Soket programlama, `TcpListener` |
| 3 | HTTP isteklerini parse etme | Raw byte'ları okuma, `BufReader` |
| 4 | HTTP yanıtı oluşturma | Durum kodları, header'lar, body |
| 5 | Thread pool ile throughput artırma | Eşzamanlılık, `Arc<Mutex<T>>`, channel'lar |

> ⚠️ **Önemli Not:** Bu yöntem production için en iyi yol değildir. Rust topluluğunda `actix-web`, `axum`, `tokio` gibi çok daha güçlü crate'ler mevcuttur. Ancak burada amacımız **öğrenmek** — Rust'ın bir sistem programlama dili olarak ne kadar düşük seviyeye inebildiğini görmek. Ayrıca bu projede `async/await` kullanmayacağız; thread pool'u senkron olarak inşa edeceğiz.

---

## 📖 BÖLÜM 21.1 — Tek Thread'li Web Sunucusu

### 1.1 🌐 TCP ve HTTP Protokollerine Kısa Bakış

Web sunucuları iki temel protokol üzerinde çalışır:

```
┌─────────────────────────────────────────────┐
│              HTTP (Yüksek Seviye)           │
│  "İsteğin içeriği ne? Hangi sayfa isteniyor?" │
├─────────────────────────────────────────────┤
│              TCP (Düşük Seviye)             │
│  "Veri bir bilgisayardan diğerine nasıl      │
│   güvenilir şekilde ulaşır?"                 │
└─────────────────────────────────────────────┘
```

- **TCP (Transmission Control Protocol):** Verinin nasıl taşındığını belirler. İstek-cevap (request-response) modelindedir.
- **HTTP (Hypertext Transfer Protocol):** TCP üzerinde çalışır; istek ve cevapların *içeriğini* tanımlar.

Her ikisi de **raw byte** olarak akar. Biz de bu byte'ları doğrudan okuyup yazacağız.

---

### 1.2 🔌 TCP Bağlantılarını Dinleme

İlk adım: Bir port üzerinde bağlantıları dinlemek.

```rust
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Bağlantı kuruldu!");
    }
}
```

**Kodun Anatomisi:**

| Kavram | Açıklama |
|--------|----------|
| `127.0.0.1` | Localhost — bilgisayarınızın kendisi |
| `7878` | Port numarası. Telefon tuş takımında "rust" yazıyor olması 😄 ve standart HTTP portlarıyla (80, 8080) çakışmaması seçilmiş |
| `bind()` | Bir porta "bağlanmak" = o portu dinlemeye almak. `Result` döner çünkü port zaten kullanımda olabilir |
| `incoming()` | Gelen bağlantıların bir **iterator**'ü. Her `TcpStream` bir istemci-sunucu bağlantısını temsil eder |

**🧪 Test Etmek:** Programı çalıştırın ve tarayıcıda `http://127.0.0.1:7878` adresine gidin. Terminalde "Bağlantı kuruldu!" mesajını göreceksiniz.

---

### 1.3 📨 HTTP İsteğini Okuma

Şimdi bağlantıdan gelen veriyi okuyalım. `handle_connection` fonksiyonu yazarak sorumlulukları ayırıyoruz:

```rust
use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("İstek: {http_request:#?}");
}
```

**Burada Neler Oluyor?**

1. **`BufReader`**: Veriyi tamponlayarak okur. Performans için önemlidir — her byte için ayrı sistem çağrısı yapmaz.
2. **`.lines()`**: Stream'i satır satır okur (HTTP, satırları `\r\n` ile ayırır).
3. **`.take_while(|line| !line.is_empty())`**: HTTP'de boş bir satır, header'ların bittiğini işaret eder. Bu yüzden burada duruyoruz.

**Tarayıcıdan Gelen Tipik İstek:**
```
[
    "GET / HTTP/1.1",
    "Host: 127.0.0.1:7878",
    "User-Agent: Mozilla/5.0 ...",
    "Accept: text/html",
    ...
]
```

---

### 1.4 ✍️ HTTP Yanıtı Yazma

Bir HTTP yanıtının formatı şöyledir:

```
HTTP-Versiyon Durum-Kodu Açıklama\r\n
Header1: Değer1\r\n
Header2: Değer2\r\n
\r\n
Mesaj-Gövdesi
```

Örneğin basit bir 200 OK yanıtı:

```rust
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
```

**`hello.html` dosyası:**
```html
<!DOCTYPE html>
<html lang="tr">
<head>
    <meta charset="utf-8">
    <title>Merhaba!</title>
</head>
<body>
    <h1>Merhaba!</h1>
    <p>Rust ile yapılan web sunucusundan selamlar.</p>
</body>
</html>
```

**Yanıtın Parçaları:**
- `HTTP/1.1 200 OK` → Versiyon ve durum kodu
- `Content-Length: ...` → Tarayıcıya body'nin kaç byte olduğunu söyler
- `\r\n\r\n` → Header'ların bittiğini, body'nin başladığını işaret eder
- `{contents}` → HTML içeriğinin kendisi

---

### 1.5 🔀 İsteklere Göre Farklı Yanıtlar (Routing)

Şu an sunucumuz her isteğe aynı HTML'i döndürüyor. Bir web sunucusu olarak **doğru yolu (/)** istenmişse 200, başka bir şey istenmişse **404** döndürmeli.

```rust
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = contents.len();
        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();
        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes()).unwrap();
    }
}
```

**`404.html` örneği:**
```html
<!DOCTYPE html>
<html lang="tr">
<head>
    <meta charset="utf-8">
    <title>Sayfa Bulunamadı</title>
</head>
<body>
    <h1>Oops!</h1>
    <p>Aradığınız sayfa bulunamadı.</p>
</body>
</html>
```

---

### 1.6 🎨 Kodu İyileştirme (Refactoring)

Dikkat ederseniz `if` ve `else` bloklarında aynı kodu tekrarlamak gereksiz. Sadece **durum satırı** ve **dosya adı** farklı! Bunları değişkene çekelim:

```rust
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
```

✨ **Daha temiz, daha okunabilir, tekrarsız!** Rust'ta tuple destructuring'in gücünü burada görüyoruz.

---

## 📖 BÖLÜM 21.2 — Thread Pool ile Çok Thread'li Sunucu

### 2.1 ⚠️ Tek Thread'in Sorunu

Mevcut sunucumuz **senkron** çalışıyor. Yani her isteği sırayla işliyor. Eğer bir istek uzun sürerse (örneğin `thread::sleep` ile 5 saniye bekletirsek), diğer tüm istekler **o bitene kadar beklemek zorunda**.

```
Zaman ──────────────────────────────►
İstek 1: [████████████ 5 sn]
İstek 2:                [████████████ 5 sn]
İstek 3:                               [████████████ 5 sn]
```

Çözüm: Her istek için yeni bir thread oluşturmak? ❌ **Hayır!** Bu da kötü çünkü 10 milyon istek gelirse 10 milyon thread oluşur ve sistem çöker (**DoS saldırısı** gibi).

---

### 2.2 💡 Thread Pool Nedir?

**Thread Pool (İş Parçacığı Havuzu):** Önceden oluşturulmuş, sabit sayıda thread'in bir havuzda beklediği ve gelen işleri sırayla üstlendiği yapıdır.

```
        ┌────────────┐
İstek → │   Kuyruk   │
        └─────┬──────┘
              │
    ┌─────────┼─────────┬─────────┐
    ▼         ▼         ▼         ▼
[Thread 1] [Thread 2] [Thread 3] [Thread 4]
```

- Havuzda **sabit sayıda** thread var (örneğin 4)
- Gelen istekler **kuyruğa** alınır
- Boş olan thread kuyruktan iş çeker ve çalıştırır
- İş bitince thread tekrar havuza döner

Bu sayede **aynı anda en fazla N istek** işlenebilir, sistem aşırı yüklenmez.

---

### 2.3 🎯 Hedef API Tasarımı

Kullanmak istediğimiz API şu olsun:

```rust
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);  // 4 thread'lik havuz

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {          // thread::spawn yerine pool.execute
            handle_connection(stream);
        });
    }
}
```

`thread::spawn` yerine `pool.execute` kullanacağız. Closure alacak ve havuzdaki bir thread'e verecek.

---

### 2.4 🛠️ Compiler-Driven Development (Derleyici Destekli Geliştirme)

Rust'ın en güzel özelliklerinden biri: **Derleyici size ne yapmanız gerektiğini söyler.** Adım adım ilerleyelim:

**Adım 1 — Boş struct tanımla:**
```rust
// src/lib.rs
pub struct ThreadPool;
```

❌ Hata: `ThreadPool::new` bulunamadı.

**Adım 2 — `new` fonksiyonu ekle:**
```rust
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool
    }
}
```

❌ Hata: `execute` metodu bulunamadı.

**Adım 3 — `execute` metodu ekle:**
```rust
impl ThreadPool {
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
```

✅ Artık derleniyor! Şimdi içini doldurabiliriz.

> 📌 **Trait Bound Açıklaması:**
> - `FnOnce()`: Closure bir kez çağrılabilir
> - `Send`: Thread'ler arası taşınabilir
> - `'static`: Closure'daki referanslar program boyunca geçerli olmalı

---

### 2.5 👷 Worker Yapısı

Her thread'i doğrudan tutmak yerine, `Worker` adında bir sarmalayıcı struct kullanacağız. Böylece her thread'e bir `id` verebilir, loglama ve debug yapabiliriz.

```rust
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});
        Worker { id, thread }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);  // 0 thread'lik havuz anlamsız!
        
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id));
        }
        ThreadPool { workers }
    }
}
```

---

### 2.6 📬 Channel ile İş Kuyruğu

Worker'lar işleri nereden alacak? **Channel** kullanarak! `ThreadPool` işleri kanala gönderecek, `Worker`'lar kanaldan çekecek.

```rust
use std::sync::mpsc;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
```

---

### 2.7 🔐 Neden `Arc<Mutex<T>>`?

Burada kritik bir sorun var: Rust'ın standart `mpsc::Receiver`'ı **tek tüketimli** (single consumer). Yani birden fazla worker'a paylaştırılamaz.

Çözüm: `Arc<Mutex<Receiver<Job>>>`

| Yapı | Görevi |
|------|--------|
| `Arc` | **A**tomic **R**eference **C**ount — Birden fazla thread'in aynı veriye sahip olmasını sağlar |
| `Mutex` | **Mut**ual **Ex**clusion — Aynı anda sadece bir thread receiver'a erişebilir (race condition'ı önler) |

```rust
let receiver = Arc::new(Mutex::new(receiver));
// Her worker'a Arc'ın bir klonu verilir
workers.push(Worker::new(id, Arc::clone(&receiver)));
```

---

### 2.8 🔄 Worker'ın Sonsuz Döngüsü

Her worker, sonsuz bir döngüde kanaldan iş bekleyecek ve gelen işi çalıştıracak:

```rust
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {id} iş aldı; çalıştırıyor.");
                job();
            }
        });
        Worker { id, thread }
    }
}
```

**Akış:**
1. `receiver.lock()` → Mutex'i kilitle (sadece ben çekeceğim)
2. `.recv()` → Kanaldan bir iş gelene kadar bekle
3. `job()` → İşi (closure'ı) çalıştır
4. Döngü başa döner → bir sonraki işi bekle

---

### 2.9 🎉 Sonuç: Çalışan Thread Pool!

Artık sunucumuz 4 thread ile eşzamanlı istekleri işleyebiliyor. Test etmek için `/sleep` gibi yavaş bir rota ekleyin:

```rust
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET /sleep HTTP/1.1" {
        thread::sleep(Duration::from_secs(5));  // 5 saniye bekle
        // ... 200 OK yanıtı
    } else if request_line == "GET / HTTP/1.1" {
        // ... hello.html
    } else {
        // ... 404.html
    }
}
```

**Sonuç:** `/sleep` isteği bir thread'i 5 saniye meşgul ederken, diğer 3 thread yeni istekleri karşılayabilir! 🚀

---

## 🎯 Özet ve Kazanımlar

Bu projede öğrendiklerimiz:

| Konu | Kazanım |
|------|---------|
| `TcpListener`, `TcpStream` | Düşük seviye ağ programlama |
| HTTP protokolü | İstek/yanıt formatı, durum kodları |
| `BufReader` | Verimli I/O okuma |
| `thread::spawn` | Thread oluşturma |
| `mpsc::channel` | Thread'ler arası iletişim |
| `Arc<Mutex<T>>` | Thread-safe paylaşım |
| `FnOnce + Send + 'static` | Closure trait bound'ları |
| Compiler-driven development | Derleyici hatalarıyla öğrenme |

---

## 🚀 Bir Sonraki Adım

Bu proje eğitici olsa da production için yeterli değil. Gerçek dünyada şunları kullanmalısınız:

- **`tokio`** veya **`async-std`** — Asenkron runtime
- **`axum`**, **`actix-web`**, **`warp`** — Yüksek performanslı web framework'leri
- **`hyper`** — Düşük seviye HTTP kütüphanesi

Ancak bu framework'lerin arka planda tam olarak **ne yaptığını** artık biliyorsunuz! 🎓

---

Sorularınız varsa sormaktan çekinmeyin. Belirli bir kısmı (örneğin `Arc<Mutex<T>>` mantığı veya channel kullanımı) daha detaylı açıklamamı ister misiniz?

# DeepSeek Yorumu -------------------

# Rust ile Çok Kanallı Bir Web Sunucusu Geliştirme: Kapsamlı Bir Ders

Merhaba! Bugün Rust programlama dilinin resmi kitabının 21. bölümünde yer alan final projesini adım adım işleyeceğiz. Bu proje, bir web sunucusunun temel taşlarını anlamak için harika bir fırsat sunuyor. Konuyu üç ana başlık altında, bir ders anlatır gibi detaylandıracağım:

1. **Tek Kanallı (Single-Threaded) Web Sunucusu**
2. **Çok Kanallı (Multithreaded) Sunucuya Dönüşüm**
3. **Zarif Kapanış ve Temizlik (Graceful Shutdown & Cleanup)**

Hazırsanız, bir web sunucusunun kalbine doğru yolculuğa çıkalım!

---

## Projeye Genel Bakış

Amacımız, `127.0.0.1:7878` adresini dinleyen, HTTP isteklerini kabul eden ve yanıt veren bir program yazmak. Önce tek bir iş parçacığında çalışan basit bir yapı kuracağız. Ardından performansı artırmak için bir **iş parçacığı havuzu (thread pool)** ekleyip aynı anda birden çok isteği işleyeceğiz. Son olarak sunucuyu temiz bir şekilde durdurmayı öğreneceğiz.

Bu proje boyunca şu Rust kavramlarını pekiştireceğiz:
- `TcpListener` ve `TcpStream` ile ağ programlaması
- HTTP protokolünün temelleri
- `std::thread`, `Mutex`, `Arc` ve mesaj kanalları (`mpsc`)
- `Drop` trait’i ile kaynak temizliği
- Ownership, borrowing ve yaşam süreleri

---

## Bölüm 21.1: Tek Kanallı Web Sunucusu Oluşturma

### Adım 1: Bir TCP Bağlantısını Dinlemek

İlk olarak bir TCP bağlantısı dinleyelim. Rust standart kütüphanesindeki `TcpListener` bu iş için biçilmiş kaftan.

```rust
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Bağlantı kuruldu!");
    }
}
```

Bu kodda:
- `TcpListener::bind` ile adrese bağlanıyoruz. `unwrap` ile olası hataları yok sayıyoruz (gerçek projede düzgün hata yönetimi şart!).
- `incoming()` metodu bir yineleyici döndürür; her eleman bir `TcpStream` denemesidir (bağlantı kurulduğunda `Ok`, hata olduğunda `Err`).
- Şimdilik her bağlantıyı konsola yazdırıp akışı bırakıyoruz.

### Adım 2: İsteği Okumak

Artık gelen ham HTTP isteğini okuyabiliriz. Basit bir tampon oluşturup `stream.read()` ile verileri alalım.

```rust
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    println!("İstek: {}", String::from_utf8_lossy(&buffer[..]));
}
```

`main` içinde her bağlantı için bu fonksiyonu çağırıyoruz. `String::from_utf8_lossy` geçersiz UTF-8 baytlarını değiştirerek okunabilir bir çıktı verir. Tarayıcıdan `http://127.0.0.1:7878` adresine girdiğinizde şuna benzer bir çıktı görürsünüz:

```
GET / HTTP/1.1
Host: 127.0.0.1:7878
User-Agent: Mozilla/5.0 ...
Accept: text/html,application/xhtml+xml,...
...
```

Bu HTTP istek formatıdır: birinci satırda **istek satırı** (method, yol, protokol), devamında başlıklar (headers) ve opsiyonel gövde.

### Adım 3: HTTP Yanıtı Oluşturmak

Sunucumuzun bir yanıt döndürmesi gerekiyor. En basit haliyle bir durum satırı, başlıklar ve isteğe bağlı gövdeden oluşur.

```rust
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let response = "HTTP/1.1 200 OK\r\n\r\n<html><body><h1>Merhaba Dünya!</h1></body></html>";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

Burada:
- `200 OK` durum kodunu belirtiyoruz.
- Başlıklar ile gövde arasında boş bir satır (`\r\n\r\n`) bulunur.
- Tarayıcı HTML gövdesini yorumlayarak başlığı gösterir.

### Adım 4: İstek Yoluna Göre Yanıt Döndürmek

Şimdi sunucuyu biraz akıllandıralım: `/` istendiğinde hoş geldiniz sayfası, aksi takdirde 404 dönsün.

Tampondan isteğin ilk satırını alıp kontrol edelim:

```rust
let get = b"GET / HTTP/1.1\r\n";

if buffer.starts_with(get) {
    let contents = fs::read_to_string("hello.html").unwrap();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
} else {
    let status_line = "HTTP/1.1 404 NOT FOUND";
    let contents = fs::read_to_string("404.html").unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
}
stream.flush().unwrap();
```

- `hello.html` ve `404.html` dosyalarını oluşturmalısınız.
- `Content-Length` başlığı, tarayıcının gövdenin nerede bittiğini anlaması için önemlidir.

### Adım 5: Yavaş İstekler ve Performans Sorunu

Sunucumuz şu anda her seferinde yalnızca bir bağlantıyı işliyor. Bir isteğin cevaplanması uzun sürerse (örneğin `/sleep` yolu), diğer tüm istekler beklemek zorunda kalır. Bunu test etmek için `handle_connection` içinde isteğe bağlı olarak bir süre uyutabiliriz:

```rust
let sleep = b"GET /sleep HTTP/1.1\r\n";
if buffer.starts_with(sleep) {
    thread::sleep(Duration::from_secs(5));
    // ... 200 yanıtı
}
```

Artık iki tarayıcı sekmesi aynı anda `/sleep` ve `/` açarsa, `/` isteği 5 saniye beklemek zorundadır. Bu kabul edilemez. Çözüm: eşzamanlılık.

---

## Bölüm 21.2: Sunucuyu Çok Kanallı Hale Getirme

Performans sorununu aşmak için bir **iş parçacığı havuzu (thread pool)** uygulayacağız. Havuz sınırlı sayıda iş parçacığı barındıracak ve gelen işleri bunlara dağıtacak. Bu sayede aynı anda birçok istek işlenebilecek.

### ThreadPool Tasarımı

Yapmak istediğimiz:
- Belirli bir sayıda (örneğin 4) iş parçacığı oluştur.
- Ana iş parçacığı (`main`) yeni bir bağlantı aldığında, onu havuzdaki bir iş parçacığına gönder.
- Havuz, iş parçacıklarına gönderilen işleri (closure) çalıştırsın.
- Havuz `Drop` edildiğinde bütün iş parçacıkları temizce dursun.

### Adım 1: ThreadPool Yapısını Tanımlamak

```rust
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}
```

- `workers`: işçi yapılarını tutar.
- `sender`: kanalın gönderici ucu; ana iş parçacığı buraya yeni `Job` gönderir.
- `Job`: çalıştırılacak kapanış (closure). `Box` ile heap’e koyuyoruz çünkü boyutu bilinmez. `FnOnce` + `Send` + `'static` sınırları, iş parçacıkları arasında güvenle aktarılabilmesi için.
- `Worker`: her bir işçi bir ID ve ana iş parçacığına bağlanmayı bekleyen `JoinHandle` içerir.

### Adım 2: İşçileri Oluşturmak (Kanal ve Paylaşımlı Durum)

Her işçi bir döngü içinde çalışacak: kanalın alıcı ucundan (`receiver`) bir iş gelmesini bekleyecek, geldiğinde onu çağıracak. Alıcının birden çok iş parçacığı tarafından paylaşılması gerektiğinden `Arc<Mutex<Receiver>>` kullanacağız.

`Worker::new` metodunda:

```rust
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("İşçi {} bir işi çalıştırıyor.", id);
                job();
            }
        });

        Worker { id, thread }
    }
}
```

- `receiver.lock()` ile mutex’i kilitliyoruz, böylece yalnızca bir iş parçacığı alıcıya erişebilir.
- `recv()` bloklayıcıdır; iş gelene kadar bekler.
- İş geldiğinde `job()` çağrılarak closure çalıştırılır.

**Not:** Mutex’in kilit süresi kısa tutulmalıdır. `recv()` kilidi bırakır mı? Hayır, `lock()` dönen `MutexGuard` kapsam boyunca kilitli kalır. Ancak `recv()` bloklarken diğer iş parçacıklarının beklemesine yol açar. Kitapta daha sonra bu tasarım değişecek, merak etmeyin.

### Adım 3: ThreadPool::new

```rust
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }
}
```

- Kanal oluşturulur.
- Alıcı bir `Arc` içine alınıp her işçiye klonlanarak verilir.
- İşçi vektörü doldurulur.

### Adım 4: execute Metodu

Havuzun `execute` metodu, aldığı closure’u kanala gönderecek:

```rust
impl ThreadPool {
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
```

Tip sınırları ile işin güvenle gönderilebileceğini garanti ediyoruz.

### Adım 5: main İçinde Kullanım

```rust
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
```

Her bağlantı havuzdaki bir işçiye yönlendirilir. Artık `/sleep` gibi yavaş istekler diğerlerini engellemez.

### Tasarımı İyileştirmek: Alıcı Kilidini Azaltmak

Mevcut kodda bir işçi `recv()` ile beklerken mutex’i kilitli tutar, diğer işçiler iş alamaz. Kitap bunu fark eder ve `while let` yerine döngüyü değiştirir:

```rust
// Kötü: while let Ok(job) = receiver.lock().unwrap().recv() { ... }
// Çünkü lock()'tan dönen MutexGuard tüm while let bloğu boyunca yaşar.

// İyi:
loop {
    let job = receiver.lock().unwrap().recv().unwrap();
    // MutexGuard burada düşer, kilit bırakılır.
    job();
}
```

Bu değişiklikle yalnızca `recv()` anında kilit tutulur, iş çalıştırılırken diğer işçiler serbest kalır.

---

## Bölüm 21.3: Zarif Kapanış ve Temizlik

Sunucuyu Ctrl+C ile kapattığımızda iş parçacıkları aniden sonlanır. Oysa biz, havuz `Drop` edildiğinde işçilerin mevcut işlerini bitirip düzgünce kapanmasını isteriz.

### Drop Trait’ini Uygulamak

`ThreadPool` için `Drop` implemente ederek kapanış sinyali göndereceğiz.

**Plan:**
- İş parçacıklarına "artık yeni iş yok, mevcut işi bitir ve çık" demek için kanala özel bir mesaj göndermek yerine, göndericiyi (`sender`) düşürerek kanalı kapatabiliriz. `recv()` hata verdiğinde (`Err`) döngüden çıkarlar.
- Ancak gönderici `ThreadPool` içinde olduğu için `Drop` sırasında düşecektir. Fakat `drop`’tan sonra işçiler hâlâ bekliyorsa kanal kapanır ve döngüden çıkarlar.
- Ama asıl mesele: `JoinHandle`’ları beklemek (`join`). Tüm işçilerin bitmesini beklemeliyiz, aksi takdirde ana iş parçacığı erken çıkarsa program sonlanır.

### Adım 1: İşçileri Temizlemek İçin Drop

```rust
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Göndericiyi düşür; bu kanalı kapatır ve recv() hata döndürür.
        drop(self.sender);

        for worker in &mut self.workers {
            println!("İşçi {} kapatılıyor.", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
```

- `drop(self.sender)` ile gönderici yok edilir, kanal kapanır. İşçiler `recv()`’de `Err` alıp döngüden çıkarlar.
- `worker.thread.take()` ile `Option<JoinHandle>`’dan `JoinHandle`’ı alırız (sahiplik kuralları gereği) ve `join` ile işçinin bitmesini bekleriz.

### Adım 2: İşçi Yapısını Güncellemek

İşçinin `thread` alanını `Option<JoinHandle<()>>` yapmalıyız ki `take()` ile değeri alıp `join` edebilelim.

```rust
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
```

`Worker::new` içinde `Some(thread)` olarak başlatılır.

### Adım 3: Kapanma Sinyaline Duyarlı Döngü

Şimdi işçi döngüsü, `recv()` hata döndüğünde döngüden çıkmalı:

```rust
loop {
    let message = receiver.lock().unwrap().recv();

    match message {
        Ok(job) => {
            println!("İşçi {} bir iş aldı.", id);
            job();
        }
        Err(_) => {
            println!("İşçi {} kapatılıyor.", id);
            break;
        }
    }
}
```

Bu sayede `sender` düşürüldüğünde işçiler döngüden çıkıp iş parçacığını sonlandırır.

### Adım 4: main’de Ctrl+C Yönetimi (Opsiyonel)

Kitap, sinyal işleme (signal handling) konusuna girmese de sunucuyu Ctrl+C ile durdurduğumuzda `main` içindeki `incoming` döngüsü sonlanmaz. Aslında program aniden sonlanır. Zarif kapanış için pool’un `Drop`’u devreye girer mi? `main` sonuna kadar çalışırsa pool scope dışına çıkar ve `Drop` çağrılır. Ama `incoming` sonsuz döngüsünden çıkmak için ayrı bir mekanizma gerekir; kitap buna değinmez, çünkü amaç thread pool’un `Drop` mantığını göstermektir. Siz isterseniz bir `AtomicBool` ile döngüyü kırabilirsiniz.

### Tam Çalışan Örnek

Projenin son halinde:
- `ThreadPool` oluşturulur.
- Gelen bağlantılar `execute` ile havuzdaki işçilere dağıtılır.
- Program sonlandığında (örneğin `main` döngüsünü bir şekilde kırarsak) pool düşer, işçiler mevcut işleri tamamlar ve bekler, sonra `join` ile ana iş parçacığı onları bekler.
- Temiz ve kayıpsız bir kapanış gerçekleşir.

---

## Öğrendiklerimizin Özeti

Bu final projesiyle Rust’ın güçlü yönlerini uygulamalı olarak gördük:

- **Sahiplik ve ödünç alma:** `TcpStream`, `ThreadPool`, `Worker` arasında veri paylaşımı.
- **Eşzamanlılık:** `Mutex<Receiver>` ve `Arc` ile güvenli paylaşım, iş parçacıkları ile paralellik.
- **Mesaj geçişi:** `mpsc::channel` ile iş dağıtımı.
- **Trait’ler ve tip sistemi:** `FnOnce` box’lama, `Drop` ile temizlik.
- **Ağ programlama:** TCP dinleyici ve HTTP temelleri.

Artık kendi web sunucunuzun temelini atacak bilgiye sahipsiniz. Bu yapıyı genişleterek statik dosya sunumu, routing, hata yönetimi gibi özellikler ekleyebilirsiniz. Unutmayın, pratik yapmak öğrenmenin en iyi yoludur. Kendi sunucunuzu yazmaya başlayın!

Herhangi bir sorunuz olursa, Rust topluluğu ve dokümantasyonu size yardımcı olacaktır. Başarılar!