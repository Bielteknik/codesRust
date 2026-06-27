# Rust Standart Kütüphane: Çeşitli Konular (Std Misc) — Kapsamlı Ders

Merhaba! Bu derste, Rust'ın standart kütüphanesinin sunduğu ve ilkel türlerin (primitives) ötesine geçen önemli yapıları inceleyeceğiz. Bu konular; **Thread'ler (İş Parçacıkları)**, **Kanallar (Channels)**, **Dosya G/Ç (File I/O)**, **Dosya Yolları (Path)**, **Süreçler (Process)**, **Komut Satırı Argümanları** ve **FFI (Yabancı Fonksiyon Arayüzü)** gibi başlıkları kapsar.

Hazırsanız başlayalım! 🚀

---

## 1. 🧵 Thread'ler (İş Parçacıkları)

### 1.1 Temel Kavram: `spawn` Fonksiyonu

Rust, işletim sisteminin kendi thread'lerini (native OS threads) oluşturmanıza olanak tanır. Bunu `std::thread::spawn` fonksiyonu ile yaparız. Bu fonksiyon, **taşıyıcı bir kapanış (moving closure)** kabul eder.

> **Dikkat:** `spawn` içine verdiğiniz closure, veriyi "taşır" (move). Yani dışarıdaki değişkenlerin mülkiyetini (ownership) thread'e devredersiniz.

```rust
use std::thread;
use std::time::Duration;

fn main() {
    // Yeni bir thread oluşturuyoruz
    let handle = thread::spawn(|| {
        for i in 0..5 {
            println!("Merhaba, thread içinden! Adım: {}", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    // Ana thread de aynı anda çalışıyor
    for i in 0..3 {
        println!("Ana thread çalışıyor: {}", i);
        thread::sleep(Duration::from_millis(300));
    }

    // Ana thread'in bitmesini bekle
    handle.join().unwrap();
    println!("Her iki thread de tamamlandı!");
}
```

### 1.2 `join` Handle (Katılma Tutamacı)

`spawn` fonksiyonu bir `JoinHandle` döndürür. Bu tutamacın `.join()` metodunu çağırdığınızda, **çağıran thread, oluşturulan thread bitene kadar bekler.**

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        // Uzun süren bir işlem
        42 // Thread'den değer döndürebiliriz!
    });

    // join() çağrısı, thread bitene kadar bloklar
    // ve thread'in döndürdüğü değeri verir
    let sonuc = handle.join().unwrap();
    println!("Thread'den gelen değer: {}", sonuc);
}
```

### 1.3 Thread'lere Veri Taşımak: `move` Anahtar Kelimesi

Bir thread'in dışarıdaki değişkenleri kullanmasını istiyorsanız `move` anahtar kelimesini kullanmalısınız:

```rust
use std::thread;

fn main() {
    let mesaj = String::from("Merhaba!");
    
    // `move` ile `mesaj` değişkeninin mülkiyeti thread'e taşınır
    let handle = thread::spawn(move || {
        println!("Thread diyor ki: {}", mesaj);
    });
    
    handle.join().unwrap();
    
    // println!("{}", mesaj); // ❌ HATA! `mesaj` artık burada kullanılamaz.
}
```

### 1.4 Paylaşılan Durum: `Arc<Mutex<T>>`

Birden fazla thread'in aynı veriye güvenli şekilde erişmesi için Rust, **`Arc<Mutex<T>>`** desenini önerir:
- **`Arc` (Atomic Reference Counted):** Verinin birden fazla thread tarafından sahiplenilmesini sağlar.
- **`Mutex` (Mutual Exclusion):** Aynı anda sadece bir thread'in veriye erişmesini garanti eder.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Arc ile sarmalanmış, başlangıç değeri 0 olan bir Mutex
    let sayac = Arc::new(Mutex::new(0));
    let mut tutamaclar = vec![];

    for _ in 0..10 {
        let sayac_klonu = Arc::clone(&sayac);
        
        let handle = thread::spawn(move || {
            let mut sayi = sayac_klonu.lock().unwrap();
            *sayi += 1;
        });
        
        tutamaclar.push(handle);
    }

    for handle in tutamaclar {
        handle.join().unwrap();
    }

    println!("Sonuç: {}", *sayac.lock().unwrap()); // 10
}
```

---

## 2. 📨 Kanallar (Channels)

### 2.1 Temel Kavram

Rust, thread'ler arasında **asenkron iletişim** için kanallar sunar. Kanalın iki ucu vardır:
- **`Sender` (Gönderici):** Veri gönderir.
- **`Receiver` (Alıcı):** Veri alır.

Kanal **tek yönlüdür** (unidirectional) — sadece Sender'dan Receiver'a veri akar.

```rust
use std::sync::mpsc; // mpsc = Multi-Producer, Single Consumer
use std::thread;

fn main() {
    // Kanal oluştur
    let (gonderici, alici) = mpsc::channel();

    // Yeni thread oluştur ve göndericiyi taşı
    thread::spawn(move || {
        let mesaj = String::from("Merhaba ana thread!");
        gonderici.send(mesaj).unwrap();
    });

    // Ana thread'de mesajı al
    let alinan = alici.recv().unwrap();
    println!("Ana thread aldı: {}", alinan);
}
```

### 2.2 `recv()` vs `try_recv()`

- **`recv()`**: Mesaj gelene kadar thread'i **bloklar** (bekletir).
- **`try_recv()`**: Mesaj yoksa hemen döner, bloklamaz.

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        tx.send("Gecikmeli mesaj").unwrap();
    });

    // try_recv ile beklemeden dene
    match rx.try_recv() {
        Ok(msg) => println!("Mesaj var: {}", msg),
        Err(_) => println!("Henüz mesaj yok, bekliyorum..."),
    }

    // recv ile mesaj gelene kadar bekle
    println!("Gelen mesaj: {}", rx.recv().unwrap());
}
```

### 2.3 Çoklu Üretici (Multi-Producer)

`mpsc`'nin "mp" kısmı **Multi-Producer** anlamına gelir. Sender'ı klonlayarak birden fazla thread'den aynı kanala veri gönderebilirsiniz:

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 0..5 {
        let tx_klonu = tx.clone();
        thread::spawn(move || {
            tx_klonu.send(format!("Thread {} mesajı", i)).unwrap();
        });
    }

    // Orijinal sender'ı bırakmalıyız ki kanal kapansın
    drop(tx);

    // Tüm mesajları al (kanal kapanana kadar)
    for mesaj in rx {
        println!("Alındı: {}", mesaj);
    }
}
```

---

## 3. 📁 Dosya G/Ç (File I/O)

### 3.1 `File` Yapısı

`File` struct'ı açılmış bir dosyayı temsil eder ve dosya tanımlayıcısını (file descriptor) sarar. Dosya işlemlerinde birçok şey ters gidebileceğinden, tüm `File` metodları **`io::Result<T>`** döndürür. Bu, hataların açıkça ele alınmasını zorunlu kılar.

```rust
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // Dosya yazma
    let mut dosya = File::create("merhaba.txt")?;
    dosya.write_all(b"Merhaba Rust Dünyası!")?;

    // Dosya okuma
    let mut dosya = File::open("merhaba.txt")?;
    let mut icerik = String::new();
    dosya.read_to_string(&mut icerik)?;
    
    println!("Dosya içeriği: {}", icerik);
    Ok(())
}
```

### 3.2 `?` Operatörü ile Hata Yönetimi

Rust'ın `?` operatörü, `Result` döndüren fonksiyonlarda hata yönetimini çok kolaylaştırır. Hata oluşursa fonksiyondan erken döner, başarı varsa değeri açar.

### 3.3 `BufReader` ve `BufWriter`

Büyük dosyalarla çalışırken tamponlu okuma/yazma performansı büyük ölçüde artırır:

```rust
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() -> std::io::Result<()> {
    let dosya = File::open("buyuk_dosya.txt")?;
    let okuyucu = BufReader::new(dosya);

    // Satır satır oku
    for satir in okuyucu.lines() {
        println!("{}", satir?);
    }
    Ok(())
}
```

### 3.4 Dosya Kipi (Open Options)

Dosyayı ekleme (append), yazma, oluşturma gibi farklı kiplerde açmak için `OpenOptions` kullanılır:

```rust
use std::fs::OpenOptions;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut dosya = OpenOptions::new()
        .append(true)        // Ekleme modu
        .create(true)        // Yoksa oluştur
        .open("log.txt")?;
    
    writeln!(dosya, "Yeni bir log satırı")?;
    Ok(())
}
```

---

## 4. 🛤️ Dosya Yolları (Path)

### 4.1 `Path` ve `PathBuf`

`Path`, işletim sisteminin dosya yollarını temsil eder. Platformlar arası farklılıkları soyutlar.

- **`Path`**: Değişmez (immutable), `str` gibidir.
- **`PathBuf`**: Değiştirilebilir (mutable), `String` gibidir.

```rust
use std::path::Path;

fn main() {
    let yol = Path::new("/home/kullanici/belge.txt");
    
    println!("Üst dizin: {:?}", yol.parent());
    println!("Dosya adı: {:?}", yol.file_name());
    println!("Uzantı: {:?}", yol.extension());
    println!("Mevcut mu? {}", yol.exists());
}
```

### 4.2 Önemli Not: UTF-8 Garantisi Yok

`Path` dahili olarak **`OsString`** ile saklanır, UTF-8 string değildir. Bu yüzden `&str`'ye dönüşüm **başarısız olabilir**:

```rust
use std::path::Path;

fn main() {
    let yol = Path::new("/some/path");
    
    // Güvenli dönüşüm (Option döner)
    if let Some(s) = yol.to_str() {
        println!("String olarak: {}", s);
    }
    
    // OsString'e dönüşüm her zaman başarılı
    let os_string = yol.to_os_string();
}
```

---

## 5. ⚙️ Süreçler (Process)

### 5.1 `Command` ile Harici Program Çalıştırma

`std::process::Command`, harici komutları çalıştırmak için bir "process builder"dır.

```rust
use std::process::Command;

fn main() {
    let cikti = Command::new("ls")
        .arg("-l")
        .arg("/home")
        .output()
        .expect("Komut çalıştırılamadı");

    if cikti.status.success() {
        println!("Çıktı:\n{}", String::from_utf8_lossy(&cikti.stdout));
    } else {
        eprintln!("Hata:\n{}", String::from_utf8_lossy(&cikti.stderr));
    }
}
```

### 5.2 Pipe (Boru) ile G/Ç Yönlendirme

Bir sürecin standart girdi/çıktısını yönlendirebilirsiniz:

```rust
use std::process::{Command, Stdio};
use std::io::Write;

fn main() {
    // "grep Rust" komutunu başlat
    let mut cocuk = Command::new("grep")
        .arg("Rust")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("grep başlatılamadı");

    // Standart girdiye yaz
    {
        let stdin = cocuk.stdin.as_mut().expect("stdin alınamadı");
        stdin.write_all(b"Rust harika!\nPython da iyi.\nRust sistem programlama içindir.\n")
            .expect("Yazma başarısız");
    }

    let cikti = cocuk.wait_with_output().expect("Bekleme başarısız");
    println!("Eşleşenler:\n{}", String::from_utf8_lossy(&cikti.stdout));
}
```

---

## 6. 📋 Komut Satırı Argümanları (Args)

Programınıza komut satırından gelen argümanlara `std::env::args()` ile erişebilirsiniz:

```rust
use std::env;

fn main() {
    let argumanlar: Vec<String> = env::args().collect();
    
    println!("Program adı: {}", argumanlar[0]);
    
    for (i, arg) in argumanlar.iter().enumerate().skip(1) {
        println!("Argüman {}: {}", i, arg);
    }
}
```

Çalıştırma:
```bash
$ cargo run -- foo bar baz
```

---

## 7. 🔗 FFI (Foreign Function Interface)

### 7.1 C Kütüphanelerini Çağırma

Rust, C ile yazılmış kütüphaneleri çağırmanıza izin verir. `extern "C"` bloğu kullanılır:

```rust
// C'nin standart kütüphanesinden `abs` fonksiyonunu çağır
extern "C" {
    fn abs(input: i32) -> i32;
}

// Rust'ın C tarafından çağrılabilir olması için
#[no_mangle]
pub extern "C" fn rust_topla(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    unsafe {
        println!("-5'in mutlak değeri: {}", abs(-5));
    }
}
```

### 7.2 `unsafe` Bloğu

FFI çağrıları **`unsafe`** blok içinde yapılmalıdır çünkü Rust derleyicisi C kodunun güvenliğini garanti edemez.

---

## 🎓 Özet ve En İyi Pratikler

| Konu | Anahtar Modül | Önemli Nokta |
|------|---------------|--------------|
| **Threads** | `std::thread` | `move` closure ile veri taşı, `Arc<Mutex<T>>` ile paylaş |
| **Channels** | `std::sync::mpsc` | Tek yönlü iletişim, `Sender` klonlanabilir |
| **File I/O** | `std::fs`, `std::io` | Her zaman `io::Result` ile hata yönetimi |
| **Path** | `std::path` | `Path` değişmez, `PathBuf` değiştirilebilir |
| **Process** | `std::process` | `Command` ile harici programlar çalıştır |
| **Args** | `std::env` | `args()` ile komut satırı parametreleri |
| **FFI** | `extern` blokları | C ile birlikte çalışabilirlik, `unsafe` gerekli |

### 💡 Altın Kurallar

1. **Hata Yönetimi:** Dosya ve ağ işlemlerinde `unwrap()` yerine `?` operatörünü veya `match` kullanın.
2. **Thread Güvenliği:** Paylaşılan veri için her zaman `Arc<Mutex<T>>` tercih edin.
3. **Mülkiyet:** Thread'lere veri taşırken `move` anahtar kelimesini unutmayın.
4. **Kaynak Yönetimi:** Dosyalar ve süreçler otomatik kapanır (RAII), ama `join()` ile thread'leri beklemeyi unutmayın.

---

## 🚀 Sıradaki Adım

Bu konuları pekiştirmek için:
- Küçük bir çok thread'li dosya işleme uygulaması yazın
- Bir producer-consumer pattern'i ile kanal kullanın
- Harici bir komutu (örn. `git status`) çalıştırıp çıktısını işleyin

Sorularınız olursa sormaktan çekinmeyin! Başarılar! 🎉

# DeepSeek Yorumu -------------------

## Rust Standart Kütüphanesi: std_misc (Çeşitli Özellikler)

Rust'ın standart kütüphanesi, temel ilkel türlerin (`primitives`) ötesine geçen birçok kullanışlı özellik sunar. Bu bölümde, özellikle **Thread'ler (İş Parçacıkları)**, **Channel'lar (Kanallar)** ve **Dosya G/Ç (File I/O)** işlemleri gibi günlük programlamada sıkça ihtiyaç duyacağınız konuları ele alacağız.

---

## 1. Thread'ler (İş Parçacıkları)

Rust, işletim sistemi seviyesinde yerel (native) thread'ler oluşturmak için `std::thread` modülü aracılığıyla güçlü bir mekanizma sunar. Thread'ler, programınızın aynı anda birden fazla işi yapmasını sağlayarak performansı artırır.

### Temel Kullanım: `thread::spawn`

Yeni bir thread başlatmak için `thread::spawn` fonksiyonu kullanılır. Bu fonksiyona argüman olarak bir **kapanış (closure)** verilir ve bu kapanışın sahipliği yeni thread'e **taşınır (move)**.

Aşağıda 10 adet thread oluşturan ve her birine bir numara yazdıran basit bir örnek verilmiştir:

```rust
use std::thread;

const NTHREADS: u32 = 10;

fn main() {
    // Tüm thread'lerin tutulacağı bir vektör oluşturuyoruz
    let mut children = vec![];

    for i in 0..NTHREADS {
        // Yeni bir thread başlatılıyor
        children.push(thread::spawn(move || {
            println!("bu thread numarası {}", i);
        }));
    }

    // Tüm thread'lerin tamamlanmasını bekliyoruz
    for child in children {
        let _ = child.join();
    }
}
```

**Açıklamalar:**

- `thread::spawn(move || { ... })` ile yeni bir thread oluşturulur. `move` anahtar kelimesi, `i` değişkeninin sahipliğinin thread'e geçmesini sağlar.
- `children.push(...)` ile oluşturulan thread'ler bir vektörde saklanır. Bu, daha sonra hepsinin tamamlanmasını beklemek için gereklidir.
- `child.join()` metodu, ilgili thread'in işini bitirmesini bekler. Bu metod bir `Result` döndürür - thread panik yaparsa hata bilgisi alınabilir.

> ⚠️ **Önemli**: Thread'ler işletim sistemi tarafından planlanır (scheduled). Hangi thread'in ne zaman çalışacağı kesin değildir ve her çalıştırmada çıktı sıralaması farklı olabilir.

---

## 2. Channel'lar (Kanallar)

Thread'ler arasında güvenli ve etkili bir şekilde veri paylaşmak için Rust, **asynchronous channel** (eşzamansız kanal) mekanizması sunar. Kanallar, iki uç arasında **tek yönlü** bir bilgi akışı sağlar:

- **Sender (Gönderici)**: Veriyi kanala gönderir.
- **Receiver (Alıcı)**: Kanaldan veriyi alır.

### Temel Kullanım: `mpsc::channel`

Rust standart kütüphanesi `std::sync::mpsc` modülünde **multi-producer, single-consumer** (çoklu gönderici, tek alıcı) kanallar sunar. Yani birden fazla thread aynı kanala veri gönderebilir, ancak veriler tek bir alıcı tarafından okunur.

```rust
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

static NTHREADS: i32 = 3;

fn main() {
    // Kanal oluşturuluyor: (tx) gönderici, (rx) alıcı
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let mut children = Vec::new();

    for id in 0..NTHREADS {
        // Gönderici kopyalanabilir (clone) - her thread kendi göndericisine sahip olur
        let thread_tx = tx.clone();
        
        let child = thread::spawn(move || {
            // Her thread kendi kimliğini kanala gönderiyor
            thread_tx.send(id).unwrap();
            println!("thread {} bitti", id);
        });
        children.push(child);
    }

    // Tüm mesajlar toplanıyor
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        // `recv()` kanaldan mesaj alır - mesaj yoksa thread bloklanır
        ids.push(rx.recv());
    }

    // Thread'lerin tamamlanmasını bekle
    for child in children {
        child.join().expect("thread panik yaptı!");
    }

    println!("{:?}", ids);
}
```

**Açıklamalar:**

- `mpsc::channel()` ile bir kanal oluşturulur. Bu fonksiyon `(Sender<T>, Receiver<T>)` tuple'ı döndürür.
- `tx.clone()` ile gönderici kopyalanır. Her thread kendi göndericisine sahip olur.
- `thread_tx.send(id)` ile veri kanala gönderilir. Bu **bloklamayan (non-blocking)** bir işlemdir - thread mesajı gönderdikten hemen sonra devam eder.
- `rx.recv()` ile alıcı kanaldan mesaj alır. Eğer kanalda mesaj yoksa, alıcı thread **bloklanır** (beklemeye geçer).
- Mesajların geliş sırası kesin değildir - thread'lerin çalışma sırasına bağlıdır.

---

## 3. Dosya G/Ç (File I/O)

Rust'da dosya işlemleri `std::fs::File` tipi üzerinden yapılır. `File`, açılmış bir dosyayı temsil eder ve dosya tanımlayıcısını (file descriptor) sarmalar. Dosya kapatma işlemi otomatiktir - `File` değişkeni kapsam dışına çıktığında (`drop` olduğunda) dosya kapanır.

> ⚠️ **Hata Yönetimi**: Tüm dosya G/Ç işlemleri `io::Result` döndürür (`Result<T, io::Error>` için bir takma addır). Bu sayede her olası başarısızlık durumu açıkça görünür ve geliştirici proaktif bir şekilde hata yönetimi yapmaya teşvik edilir.

### 3.1. Dosya Açma: `File::open`

`File::open` fonksiyonu, bir dosyayı **salt okunur (read-only)** modda açar.

```rust
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("hello.txt");
    let display = path.display();

    // Dosyayı okunur modda aç
    let mut file = match File::open(&path) {
        Err(why) => panic!("{} açılamadı: {}", display, why),
        Ok(file) => file,
    };

    // Dosya içeriğini bir String'e oku
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("{} okunamadı: {}", display, why),
        Ok(_) => println!("{} içeriği:\n{}", display, s),
    }
    // `file` kapsam dışına çıkar ve dosya otomatik kapanır
}
```

**Örnek çalıştırma**:
```bash
$ echo "Hello World!" > hello.txt
$ rustc open.rs && ./open
hello.txt içeriği:
Hello World!
```

> 💡 **Deneyin**: `hello.txt` dosyasını silerek veya okuma izinlerini değiştirerek farklı hata durumlarını test edin.

### 3.2. Dosya Oluşturma: `File::create`

`File::create` fonksiyonu, bir dosyayı **sadece yazma (write-only)** modunda açar. Eğer dosya zaten varsa, eski içeriği **tamamen silinir** (üzerine yazılır). Yoksa yeni bir dosya oluşturulur.

```rust
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

static LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet...";

fn main() {
    let path = Path::new("lorem_ipsum.txt");
    let display = path.display();

    // Dosyayı yazma modunda aç (varsa üzerine yazar)
    let mut file = match File::create(&path) {
        Err(why) => panic!("{} oluşturulamadı: {}", display, why),
        Ok(file) => file,
    };

    // Dosyaya yaz
    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => panic!("{} yazılamadı: {}", display, why),
        Ok(_) => println!("{} başarıyla yazıldı", display),
    }
}
```

> 💡 **Esnek Ayarlar**: Daha karmaşık açma seçenekleri (örneğin ekleme modu, okuma-yazma) için [`OpenOptions`](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html) yapısı kullanılabilir.

### 3.3. Dosyadan Satır Satır Okuma: `read_lines`

Bir dosyayı satır satır okumak için çeşitli yaklaşımlar vardır. İşte en verimli olanı:

```rust
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./hosts.txt") {
        for line in lines.map_while(Result::ok) {
            println!("{}", line);
        }
    }
}

// Dosya satırları üzerinde bir yineleyici (iterator) döndürür
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
```

**Neden bu yaklaşım?** 

- `BufReader` dahili bir tampon (buffer) kullanarak ara bellek tahsislerini azaltır.
- Tüm dosyayı tek bir `String`'e okumak yerine, satırları tembel (lazy) bir şekilde işler.
- Özellikle **büyük dosyalarda** bellek kullanımı açısından çok daha verimlidir.

**Karşılaştırma - Naif Yaklaşım**:
```rust
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }
    result
}
```
Bu yaklaşım tüm dosyayı belleğe okur ve her satır için yeni bir `String` tahsis eder - küçük dosyalar için uygun olsa da büyük dosyalarda sorun yaratır.

---

## Özet

| Konu | Ana Tip/Modül | Temel İşlemler |
|------|---------------|----------------|
| **Thread'ler** | `std::thread` | `spawn()`, `join()` |
| **Channel'lar** | `std::sync::mpsc` | `channel()`, `send()`, `recv()` |
| **Dosya G/Ç** | `std::fs::File` | `open()`, `create()`, `read_to_string()`, `write_all()` |
| **Verimli Okuma** | `std::io::BufReader` | `lines()` ile satır yineleyici |

Bu üç temel alan, Rust'da eşzamanlı programlama ve dosya işlemlerinin temelini oluşturur. Thread'ler ve channel'lar birlikte kullanıldığında güçlü ve güvenli eşzamanlı sistemler inşa etmenizi sağlar. Dosya G/Ç işlemlerinde ise Rust'ın hata yönetimi felsefesi (`Result` ile açık hata taşıma) sayesinde sağlam ve güvenilir uygulamalar geliştirebilirsiniz.