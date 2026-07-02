# 🦀 Ders Notları: Bölüm 16 - Rust'ta Eşzamanlılık (Concurrency) - Kapsamlı Ders

## 📚 Giriş: Neden Eşzamanlılık Önemli?

Merhaba arkadaşlar! Bugün Rust programlamanın en güçlü özelliklerinden biri olan **eşzamanlılık** (concurrency) konusunu öğreneceğiz.

**Eşzamanlı programlama**, bir programın farklı bölümlerinin bağımsız olarak çalışması demektir. **Paralel programlama** ise farklı bölümlerin aynı anda çalışmasıdır. Günümüzde çoğu bilgisayar birden fazla işlemci çekirdeğine sahip, bu yüzden bu kavramlar giderek daha önemli hale geliyor.

### Rust'ın Farkı: "Cesur Eşzamanlılık" (Fearless Concurrency)

Tarihsel olarak eşzamanlı programlama zor ve hata yapmaya açıktı. Rust ise bu durumu değiştiriyor! Rust'ın **sahiplik sistemi** (ownership) ve **tip sistemi** (type system) hem bellek güvenliğini hem de eşzamanlılık sorunlarını yönetmek için güçlü araçlar sağlıyor.

Pek çok eşzamanlılık hatası Rust'ta **çalışma zamanı hataları yerine derleme zamanı hatalarıdır**. Yani kodunuz çalışmadan önce hatalar yakalanır! Bu özelliğe Rustçılar **"cesur eşzamanlılık"** diyor.

---

## 🧵 Bölüm 16.1: Thread'ler (İş Parçacıkları)

### Thread Nedir?

İşletim sistemlerinde bir program **process** (süreç) içinde çalışır. Thread'ler ise bir program içindeki bağımsız bölümlerdir ve aynı anda çalışabilirler.

**Örnek:** Bir web sunucusu, birden fazla isteğe aynı anda cevap verebilmek için birden fazla thread kullanabilir.

### Thread Oluşturma

Yeni bir thread oluşturmak için `thread::spawn` fonksiyonunu kullanırız:

```rust
use std::thread;
use std::time::Duration;

fn main() {
    // Yeni bir thread oluşturuyoruz
    thread::spawn(|| {
        for i in 1..10 {
            println!("Spawned thread'den selam {i}!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Ana thread
    for i in 1..5 {
        println!("Ana thread'den selam {i}!");
        thread::sleep(Duration::from_millis(1));
    }
}
```

**Önemli Nokta:** Ana thread bittiğinde, tüm spawned thread'ler de biter! Bu yüzden çıktı şöyle olabilir:

```
Ana thread'den selam 1!
Spawned thread'den selam 1!
Ana thread'den selam 2!
Spawned thread'den selam 2!
...
```

### JoinHandle ile Thread'leri Beklemek

Thread'in tamamlanmasını beklemek için `JoinHandle` kullanırız:

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Spawned thread: {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Ana thread: {i}");
        thread::sleep(Duration::from_millis(1));
    }

    // Spawned thread bitene kadar bekle
    handle.join().unwrap();
}
```

`join()` çağrısı, thread bitene kadar mevcut thread'i bloklar (bekletir).

### Move Keyword ile Veri Taşıma

Thread'ler arasında veri taşırken `move` keyword'ü çok önemlidir:

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // move keyword'ü olmadan hata alırız!
    let handle = thread::spawn(move || {
        println!("İşte vektör: {v:?}");
    });

    handle.join().unwrap();
}
```

**Neden move gerekli?** Rust, spawned thread'in ne kadar çalışacağını bilmez. Eğer referans kullansaydık, ana thread sonlanabilir ve referans geçersiz hale gelebilirdi. `move` ile sahipliği yeni thread'e devrediyoruz.

---

## 📨 Bölüm 16.2: Message Passing (Mesajlaşma)

### Kanal (Channel) Kavramı

Message passing, thread'lerin birbirleriyle veri göndererek iletişim kurmasıdır. Go dilinin felsefesi şöyledir:

> "Hafızayı paylaşarak iletişim kurma; bunun yerine iletişim kurarak hafızayı paylaş."

Rust'ta kanallar `std::sync::mpsc` modülünde bulunur. **mpsc** = **multiple producer, single consumer** (çoklu üretici, tekil tüketici).

### Kanal Oluşturma

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    // Kanal oluştur - (transmitter, receiver) döner
    let (tx, rx) = mpsc::channel();

    // Yeni thread oluştur ve transmitter'ı taşı
    thread::spawn(move || {
        let val = String::from("merhaba");
        tx.send(val).unwrap();
    });

    // Ana thread'de mesajı al
    let received = rx.recv().unwrap();
    println!("Alındı: {received}");
}
```

### recv() vs try_recv()

- **`recv()`**: Mesaj gelene kadar bloklar (bekler)
- **`try_recv()`**: Bloklamaz, hemen döner. Mesaj yoksa `Err` döner

### Birden Fazla Mesaj Gönderme

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("merhaba"),
            String::from("nasılsın"),
            String::from("iyi"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Receiver'ı iterator olarak kullan
    for received in rx {
        println!("Alındı: {received}");
    }
}
```

### Çoklu Üretici (Multiple Producers)

Transmitter'ı klonlayarak birden fazla thread'den mesaj gönderebiliriz:

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    // İlk producer
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec!["thread 1 - mesaj 1", "thread 1 - mesaj 2"];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // İkinci producer
    thread::spawn(move || {
        let vals = vec!["thread 2 - mesaj 1", "thread 2 - mesaj 2"];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Alındı: {received}");
    }
}
```

### Sahiplik ve Kanallar

Kanallar sahiplik kurallarına uyar! Mesaj gönderdikten sonra o değeri kullanamazsınız:

```rust
thread::spawn(move || {
    let val = String::from("merhaba");
    tx.send(val).unwrap();
    // println!("{val}"); // HATA! val zaten taşındı
});
```

Bu, veri yarışlarını (race conditions) önler!

---

## 🔒 Bölüm 16.3: Shared State (Paylaşımlı Durum)

### Mutex<T> Nedir?

**Mutex** = **Mutual Exclusion** (Karşılıklı Dışlama). Bir mutex, aynı anda sadece bir thread'in veriye erişmesine izin verir.

**Gerçek dünya analojisi:** Bir panel tartışmasında tek bir mikrofon var. Konuşmak isteyen mikrofonu almalı, işi bitince başkasına vermeli.

### Mutex Kullanımı (Tek Thread)

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        // Kilidi al
        let mut num = m.lock().unwrap();
        *num = 6;
    } // num scope'tan çıkınca kilit otomatik serbest bırakılır

    println!("m = {m:?}");
}
```

**Önemli:** `lock()` metodu `MutexGuard` döner. Bu guard scope'tan çıkınca kilit otomatik serbest bırakılır (Drop trait'i sayesinde).

### Mutex'i Thread'ler Arasında Paylaşma

10 thread ile bir sayacı 10'a çıkaralım:

```rust
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Sonuç: {}", *counter.lock().unwrap());
}
```

**HATA!** Bu kod derlenmez çünkü `counter` ilk loop iterasyonunda taşındı.

### Rc<T> ile Deneme

Çoklu sahiplik için `Rc<T>` kullanmayı deneyelim:

```rust
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    // ...
}
```

**Yine HATA!** `Rc<T>` thread-safe değil çünkü referans sayısını atomik olarak güncellemiyor.

### Çözüm: Arc<T> (Atomic Reference Counted)

`Arc<T>`, `Rc<T>`'nin thread-safe versiyonudur:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Sonuç: {}", *counter.lock().unwrap());
}
```

**Çıktı:** `Sonuç: 10` ✅

### Mutex<T> ve Interior Mutability

`Mutex<T>`, `RefCell<T>` gibi **interior mutability** sağlar. Dışarıdan immutable olsa bile, içindeki veriyi değiştirebiliriz.

### Deadlock Riski

Rust sizi deadlock'lardan koruyamaz! İki thread birbirinin kilidini beklerse sonsuz döngü oluşur. Bu tür mantıksal hataları siz önlemelisiniz.

---

## 🛡️ Bölüm 16.4: Send ve Sync Trait'leri

### Send Trait

**`Send`**: Bir tipin sahipliğinin thread'ler arasında taşınabileceğini belirtir.

- Neredeyse tüm Rust tipleri `Send` implement eder
- **İstisna:** `Rc<T>` - thread-safe değil çünkü referans sayısını atomik güncellemiyor
- Tüm primitive tipler `Send`'dir (raw pointer'lar hariç)

### Sync Trait

**`Sync`**: Bir tipin birden fazla thread'den güvenle referans edilebileceğini belirtir.

- `&T` (T'nin referansı) `Send` ise, `T` `Sync`'dir
- `Rc<T>` ve `RefCell<T>` `Sync` değildir
- `Mutex<T>` `Sync`'dir ve thread'ler arasında paylaşılabilir

### Otomatik Implementasyon

Eğer bir tip sadece `Send` ve `Sync` olan tiplerden oluşuyorsa, otomatik olarak `Send` ve `Sync` implement eder. Bu yüzden çoğu zaman bu trait'leri manuel implement etmenize gerek yoktur.

### Manuel Implementasyon

Bu trait'leri manuel implement etmek **unsafe Rust** gerektirir ve dikkatli yapılmalıdır.

---

## 🎯 Özet ve En İyi Pratikler

### Ne Zaman Ne Kullanmalı?

| Durum | Çözüm |
|-------|-------|
| Thread'ler arası mesajlaşma | **Kanallar** (`mpsc::channel`) |
| Paylaşımlı veri erişimi | **`Mutex<T>` + `Arc<T>`** |
| Basit sayısal işlemler | **`AtomicUsize`, `AtomicI32`** vb. |
| Tek thread'de çoklu sahiplik | **`Rc<T>`** |

### Rust'ın Garantileri

✅ **Derleme zamanı güvenliği:** Pek çok eşzamanlılık hatası derleme zamanında yakalanır  
✅ **Sahiplik sistemi:** Veri yarışlarını önler  
✅ **Type system:** Thread-safe olmayan tipleri thread'ler arası taşımanızı engeller  

### Unutmayın!

- `thread::spawn` ile yeni thread oluşturun
- `join()` ile thread'lerin bitmesini bekleyin
- `move` keyword'ü ile closure'lara sahipliği devredin
- Kanallar ile mesajlaşma yapın (`mpsc::channel`)
- Paylaşımlı durum için `Mutex<T>` + `Arc<T>` kullanın
- `Send` ve `Sync` trait'leri thread güvenliğini garanti eder

---

## 🚀 Sonraki Adımlar

Bu bölümde eşzamanlılığın temellerini öğrendiniz. Bir sonraki bölümde **async/await** ile asenkron programlamayı öğreneceğiz. Ayrıca 21. bölümdeki projede bu kavramları gerçek dünya senaryolarında kullanacaksınız.

**Cesur olun ve eşzamanlı programlamanın keyfini çıkarın!** 🦀✨

# DeepSeek Yorumu ----------------------

Merhaba arkadaşlar, bugün Rust'ın en güçlü yanlarından biri olan eşzamanlılık (concurrency) konusuna derinlemesine dalıyoruz. Rust'ın “Korkusuz Eşzamanlılık” (Fearless Concurrency) sloganını duymuşsunuzdur. Peki bu ne demek? Rust’ın sahiplik sistemi ve tip sistemi sayesinde, veri yarışı (data race) gibi hataları daha kod yazarken, derleme aşamasında yakalayabiliyoruz. Bu bölüm, `std::thread`, kanallar (message passing), `Mutex`, `Arc` ve `Sync`/`Send` özelliklerini (trait) kapsayarak size güvenli eşzamanlı programlamanın kapılarını açıyor. Hazırsanız başlayalım.

---

## 16.1. İş Parçacıkları ile Eş Zamanlı Kod Çalıştırma (Using Threads to Run Code Simultaneously)

İş parçacıkları (thread), programınızın farklı bölümlerini aynı anda çalıştırmanızı sağlar. Rust standart kütüphanesi, işletim sisteminin iş parçacıklarını **1:1 model** ile kullanır, yani her Rust iş parçacığı bir işletim sistemi iş parçacığıdır. Yeni bir iş parçacığı oluşturmak için `std::thread::spawn` fonksiyonuna bir closure veririz.

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Yan iş parçacığında sayı: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Ana iş parçacığında sayı: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); // Yan iş parçacığının bitmesini bekle
}
```

- `thread::spawn` bir `JoinHandle` döndürür. `join` metodu çağrıldığında, ana iş parçacığı o iş parçacığının bitmesini bekler.
- Eğer `handle.join()` yazmazsanız, ana iş parçacığı bittiğinde program sonlanır ve yan iş parçacığı tamamlanamaz.  
- Çıktı her çalıştırmada farklı sıralanabilir; işletim sistemi hangi iş parçacığını ne zaman çalıştıracağına kendisi karar verir.

**`move` Closure Kullanımı**

İş parçacığına dışarıdan veri taşımak istediğimizde closure'ın başına `move` anahtar kelimesini eklememiz gerekir. Böylece closure, yakaladığı değişkenlerin sahipliğini alır ve veriyi iş parçacığına taşır.

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("İşte vektör: {:?}", v);
    });

    handle.join().unwrap();
}
```

Burada `v`'nin sahipliği closure'a geçtiği için ana iş parçacığında artık kullanılamaz. Rust sayesinde, iş parçacıkları arasında yanlışlıkla aynı veriye aynı anda erişme derdi kalmaz.

---

## 16.2. Mesaj Geçişi ile İş Parçacıkları Arasında Veri Aktarımı (Using Message Passing to Transfer Data Between Threads)

Go dilinden tanıdığımız bir motto: **"Belleği paylaşarak iletişim kurmayın; iletişim kurarak belleği paylaşın."** Rust’ta mesaj geçişi için standart kütüphanede **kanal (channel)** mekanizması bulunur. `std::sync::mpsc` modülü **çok üreticili, tek tüketicili** (multiple producer, single consumer) bir kanal sağlar.

- `mpsc::channel()` bir **gönderici** (transmitter, `tx`) ve bir **alıcı** (receiver, `rx`) demet döndürür.
- Gönderici `.send()` ile mesaj yollar, alıcı `.recv()` ile mesaj alır (bekler) veya `.try_recv()` ile anında kontrol eder.

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("merhaba");
        tx.send(val).unwrap(); // val'in sahipliği kanala geçer
        // artık val burada kullanılamaz!
    });

    let received = rx.recv().unwrap();
    println!("Alınan: {}", received);
}
```

`send` ile gönderilen değer taşınır (move). Bu sayede gönderen iş parçacığı o veriyi bir daha kullanamaz; veri yarışı imkânsız hale gelir.

**Birden Fazla Üretici**

`tx.clone()` yaparak birden çok gönderici oluşturabiliriz. Hepsi aynı alıcıya mesaj yollayabilir.

```rust
let (tx, rx) = mpsc::channel();
let tx1 = tx.clone();
thread::spawn(move || {
    let vals = vec![String::from("selam"), String::from("dünya")];
    for val in vals {
        tx1.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

thread::spawn(move || {
    let vals = vec![String::from("başka"), String::from("mesaj")];
    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

for received in rx {
    println!("Alınan: {}", received);
}
```

Alıcıyı bir döngü içinde kullanırsak, göndericiler düştüğünde (drop) kanal kapanır ve döngü sonlanır.

Mesaj geçişi, veriyi paylaşma ihtiyacını ortadan kaldırdığı için eşzamanlı programlamada çok tercih edilen bir modeldir. Rust’ın sahiplik kuralları da bunu son derece güvenli hale getirir.

---

## 16.3. Paylaşımlı Durum Eşzamanlılığı (Shared-State Concurrency)

Mesaj geçişi her zaman en uygun çözüm olmayabilir. Bazen birden çok iş parçacığının aynı veriye erişip onu değiştirmesi gerekir. İşte burada **Mutex** (mutual exclusion) devreye girer.

**`Mutex<T>`**

`std::sync::Mutex<T>`, içindeki veriye yalnızca bir iş parçacığının aynı anda erişmesine izin verir. İç veriyi kullanmak için `lock()` metodunu çağırırız; bu işlem, kilidi almaya çalışır ve başarılı olursa `MutexGuard` döner. `MutexGuard`, `Deref` ve `Drop` uygular; böylece kilit, koruma alanından çıktığımızda otomatik serbest bırakılır.

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    } // mutex guard düşer, kilit açılır

    println!("m = {:?}", m); // Mutex { data: 6 }
}
```

Ama `Mutex<T>` tek başına yeterli değildir; onu birden fazla iş parçacığıyla paylaşmaya çalıştığımızda sahiplik sorunu çıkar. İşte bu noktada **`Arc<T>`** imdada yetişir.

**`Arc<T>` – Atomik Referans Sayıcı**

`Rc<T>`’nin iş parçacığı güvenli versiyonudur. `Arc<T>`, referans sayımını **atomik** olarak yapar, böylece farklı iş parçacıklarında güvenle paylaşılabilir.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Sonuç: {}", *counter.lock().unwrap());
}
```

Burada `Arc`, `Mutex`’i çoklu sahipli yapar; `Mutex` ise veriye güvenli eşzamanlı erişim sağlar. Bu ikili, Rust'ta paylaşımlı durumun temel taşıdır. Ancak unutmayın: `Mutex` kullanırken kilitlenme (deadlock) riskine karşı dikkatli olmalısınız. Rust bunu derleme zamanında engelleyemez, ama en azından veri yarışlarını tamamen yok eder.

---

## 16.4. `Sync` ve `Send` Özellikleri ile Genişletilebilir Eşzamanlılık (Extensible Concurrency with the Sync and Send Traits)

Rust dilinin eşzamanlılık yetenekleri derleyiciye gömülü değil; büyük ölçüde standart kütüphanedeki iki özel özellik (trait) ile tanımlanmıştır: `Send` ve `Sync`. Bu özellikler, tiplerin iş parçacıkları arasında nasıl taşınabileceğini veya paylaşılabileceğini belirleyen **işaretleyici özelliklerdir** (marker trait, metodları yoktur).

**`Send` Özelliği**

Bir tip `Send` ise, onun sahipliği iş parçacıkları arasında transfer edilebilir. Rust’taki neredeyse bütün tipler `Send`’dir, ancak bazı istisnalar vardır (örneğin `Rc<T>`). `Rc<T>`’nin referans sayacı atomik olmadığı için aynı anda farklı iş parçacıklarında kullanılamaz; bu yüzden `Send` değildir. Eğer `Send` olmayan bir tipi başka iş parçacığına göndermeye çalışırsanız derleyici hata verir.

**`Sync` Özelliği**

Bir tip `Sync` ise, bu tipin referansı (`&T`) birden fazla iş parçacığında aynı anda güvenle kullanılabilir. Başka bir deyişle, `T` tipi `Sync` ancak ve ancak `&T` (yani referansı) `Send` ise. Örneğin `Mutex<T>` `Sync`’tir çünkü iç veriye aynı anda yalnızca bir referans erişebilir. `Rc<T>` ise ne `Send` ne de `Sync`’tir.

Bu iki özellik otomatik olarak türetilir; siz manuel olarak implement etmeniz gerekmez (zaten unsafe olmadan edemezsiniz). Fakat kendi tipinizin `Send` veya `Sync` olmasını engellemek isterseniz, içinde `Send`/`Sync` olmayan bir alan bulundurmanız yeterlidir. Örneğin `std::marker::PhantomData` ile işaretleme yapabilirsiniz.

Bu mekanizma sayesinde Rust, sizin yazdığınız kütüphanelerin de iş parçacığı güvenliğini derleme zamanında garanti altına almasını sağlar. Korkusuz eşzamanlılığın sırrı buradadır: dil, “tehlikeli” kombinasyonları daha siz test etmeden reddeder.

---

## Özet

Rust, eşzamanlı programlamayı üç temel yaklaşımla destekler:

1. **İş parçacıkları** – `thread::spawn` ve `move` closure ile basit paralellik.
2. **Mesaj geçişi** – Kanallar (`mpsc`) ile "paylaşarak iletişim değil, iletişerek paylaşma".
3. **Paylaşımlı durum** – `Mutex` ve `Arc` ile güvenli ortak erişim.

Tüm bunların arkasında, `Send` ve `Sync` özelliklerinin sessizce çalışan tip denetimi sayesinde **veri yarışı** ihtimali sıfıra indirgenir. Böylece siz, mantık hatalarına ve deadlock gibi problemlere odaklanabilirsiniz; çünkü en korkutucu hatalar daha derleme anında yakalanır.

Bu bölümle beraber Rust’ın güvenli eşzamanlılık felsefesini özümsemiş oldunuz. Bol pratik yaparak konuyu pekiştirmenizi öneririm. Bir sonraki konumuzda görüşmek üzere!