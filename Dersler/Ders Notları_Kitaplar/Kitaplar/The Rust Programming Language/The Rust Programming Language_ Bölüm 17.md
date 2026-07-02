# 🦀 Ders Notları: Bölüm 17 - Rust'ta Async/Await: Kapsamlı Ders Notları 📚

Rust'ın resmi kitabının 17. bölümü olan "Async, Await, Futures ve Streams" konusunu ders anlatır gibi, adım adım ve detaylı olarak inceleyelim.

---

## 📖 Bölüm 17.0: Giriş - Asenkron Programlamanın Temelleri

### Neden Asenkron Programlamaya İhtiyacımız Var?

Bilgisayarlarda uzun süren işlemler sırasında başka işler yapabilmek için iki temel teknik vardır: **paralellik** ve **eşzamanlılık**.

**Örnek Senaryo:**
- **Video dışa aktarma** (CPU-bağımlı): İşlemci ve ekran kartı gücünü sonuna kadar kullanır
- **Video indirme** (I/O-bağımlı): Ağdan veri gelmesini bekler

İşletim sistemleri programları görünmez şekilde kesintiye uğratarak eşzamanlılık sağlar, ancak bu sadece **program düzeyinde** gerçekleşir. Programın içindeki daha küçük fırsatları işletim sistemi göremez.

### Paralellik vs Eşzamanlılık Farkı

| Özellik | Eşzamanlılık (Concurrency) | Paralellik (Parallelism) |
|---------|---------------------------|--------------------------|
| **Tanım** | Tek kişi birden fazla görevi sırayla yapar | Birden fazla kişi aynı anda farklı görevleri yapar |
| **Benzerlik** | Bir proje sıkıldığında diğerine geçmek | İki ayrı kişi iki farklı proje üzerinde çalışır |
| **Donanım** | Tek CPU çekirdeğinde bile olabilir | Birden fazla CPU çekirdeği gerekir |
| **Amaç** | Yapısal düzeni iyi yönetmek | Aynı anda birden fazla iş yapmak |

**Önemli Not:** Rust'ta async kod genellikle eşzamanlı çalışır, ancak kullanılan donanım ve runtime'a bağlı olarak arka planda paralellik de kullanabilir.

---

## 📖 Bölüm 17.1: Futures ve Async Sözdizimi

### Future Nedir?

**Future** (gelecek), şu anda hazır olmayıp gelecekte bir noktada hazır olacak bir değeri temsil eder. Diğer dillerde "task" veya "promise" olarak da bilinir.

```rust
// Future trait'i Rust'ta temel yapı taşıdır
trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

### async ve await Anahtar Kelimeleri

- **`async`**: Blokları ve fonksiyonları kesilip devam edilebilir olarak işaretler
- **`await`**: Bir future'ın hazır olmasını bekler
- **Polling**: Future'ın değerinin hazır olup olmadığını kontrol etme süreci

### İlk Async Programımız

```rust
extern crate trpl;
use trpl::Html;

async fn page_title(url: &str) -> Option<String> {
    let response_text = trpl::get(url).await.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html())
}
```

**Önemli Noktalar:**
1. **Future'lar temmeldir (lazy)**: `await` ile istemediğiniz sürece hiçbir şey yapmazlar
2. **`await` postfix'tir**: Rust'ta `await` kelimesi ifadeden sonra gelir (diğer dillerin aksine)
3. **`main` async olamaz**: Çünkü async kod bir **runtime** gerektirir

### Runtime Nedir?

Runtime, asenkron kodun yürütülmesini yöneten bir Rust crate'idir. Rust, diğer dillerin aksine, gömülü bir runtime ile gelmez - bu size esneklik sağlar.

```rust
fn main() {
    trpl::block_on(async {
        let url = "https://www.rust-lang.org";
        match page_title(url).await {
            Some(title) => println!("Başlık: {title}"),
            None => println!("Başlık yok"),
        }
    })
}
```

### İki URL'yi Yarıştırmak

```rust
use trpl::{Either, Html};

fn main() {
    trpl::block_on(async {
        let title_fut_1 = page_title("https://example.com");
        let title_fut_2 = page_title("https://rust-lang.org");

        let (url, maybe_title) = match trpl::select(title_fut_1, title_fut_2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{url} önce bitti");
    })
}
```

**`Either` Tipi:** `Left` ve `Right` varyantlarına sahip, hangisinin önce bittiğini gösteren bir tip.

---

## 📖 Bölüm 17.2: Async ile Eşzamanlılık

### Thread vs Async Task

| Özellik | Thread | Async Task |
|---------|--------|------------|
| **Yönetim** | İşletim sistemi | Runtime (kütüphane) |
| **Bellek** | Her thread için fazla bellek | Hafif, runtime tarafından yönetilir |
| **Eşzamanlılık** | Thread'ler arasında | Task'lar arasında VE içinde |
| **Taşınabilirlik** | OS bağımlı | Her yerde çalışır (gömülü sistemler dahil) |

### spawn_task Kullanımı

```rust
use std::time::Duration;

fn main() {
    trpl::block_on(async {
        trpl::spawn_task(async {
            for i in 1..10 {
                println!("Birinci task'tan {i}");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("İkinci task'tan {i}");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    });
}
```

### Join Handle ile Bekleme

Thread'lerdeki `join` metodunun async karşılığı, handle'ı `await` etmektir:

```rust
let handle = trpl::spawn_task(async { /* ... */ });
// Diğer işler...
handle.await.unwrap(); // Task'in bitmesini bekle
```

### trpl::join ile Adil Birleştirme

```rust
let fut1 = async { /* ... */ };
let fut2 = async { /* ... */ };

trpl::join(fut1, fut2).await; // İkisi de bitene kadar bekle
```

**`trpl::join` adil (fair) çalışır:** Her future'a eşit sıklıkta bakar, birinin diğerinden öne geçmesine izin vermez.

### Async Kanallar ile Mesajlaşma

```rust
let (tx, mut rx) = trpl::channel();

let tx_fut = async move {
    for val in vec!["hi", "from", "the", "future"] {
        tx.send(val.to_string()).unwrap();
        trpl::sleep(Duration::from_millis(500)).await;
    }
};

let rx_fut = async {
    while let Some(value) = rx.recv().await {
        println!("Alındı: '{value}'");
    }
};

trpl::join(tx_fut, rx_fut).await;
```

**`while let` Döngüsü:** Async olarak üretilen öğe serileri için kullanılır.

**`move` Anahtar Kelimesi:** Async bloklara ownership taşımak için kullanılır, tıpkı closure'lardaki gibi.

---

## 📖 Bölüm 17.3: Daha Fazla Future - Starvation ve Timeout

### Starvation (Aç Bırakma) Problemi

**Kritik Kural:** Rust, async blokları **sadece await noktalarında** duraklatır. Await noktaları arasındaki her şey senkrondur!

```rust
// ❌ KÖTÜ: Diğer future'ları aç bırakır
let a = async {
    slow_operation_1(); // thread::sleep kullanıyor
    slow_operation_2();
    trpl::sleep(Duration::from_millis(50)).await; // Tek await noktası
};

// ✅ İYİ: Düzenli olarak kontrolü bırakır
let a = async {
    slow_operation_1();
    trpl::yield_now().await; // Kontrolü runtime'a bırak
    slow_operation_2();
    trpl::yield_now().await;
};
```

### yield_now ile Kontrolü Bırakma

```rust
trpl::yield_now().await; // "Benim işim şimdilik bitti, başkasına geç"
```

Bu, `sleep` kullanmaktan daha iyidir çünkü:
- Gerçekten uyumaz, sadece kontrolü bırakır
- Timer'ların minimum granüleritesi yoktur (sleep en az 1ms bekler)
- Daha hızlıdır

### Timeout Fonksiyonu Yazma

Future'ları compose ederek güçlü soyutlamalar oluşturabiliriz:

```rust
async fn timeout<F: Future>(
    future_to_try: F,
    max_time: Duration,
) -> Result<F::Output, Duration> {
    match trpl::select(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),   // Future önce bitti
        Either::Right(_) => Err(max_time),    // Timeout oldu
    }
}
```

**Kullanımı:**
```rust
match timeout(slow_future, Duration::from_secs(2)).await {
    Ok(message) => println!("Başarılı: '{message}'"),
    Err(duration) => println!("{} saniye sonra başarısız", duration.as_secs()),
}
```

### İşbirlikçi Çoklu Görev (Cooperative Multitasking)

Async sistemde her future, ne zaman kontrolü bırakacağına **kendisi karar verir**. Bu nedenle:
- Her future, çok uzun süre bloklamama **sorumluluğuna** sahiptir
- Bazı gömülü işletim sistemlerinde bu **tek** çoklu görev türüdür
- Gerçek kodda her satırda `yield_now` çağırmazsınız - ölçüm yaparak dengeyi bulun

---

## 📖 Bölüm 17.4: Streams - Zaman İçinde Future'lar

### Stream Nedir?

Stream, zaman içinde bir öğe dizisi üreten bir desendir. Iterator'ın asenkron versiyonu gibi düşünülebilir.

**Stream vs Iterator:**
| Özellik | Iterator | Stream |
|---------|----------|--------|
| **Zamanlama** | Senkron | Asenkron |
| **API** | `next()` | `next().await` |
| **Kullanım** | Hemen hazır veriler | Zamanla gelen veriler |

### Stream Oluşturma ve Kullanma

```rust
use trpl::StreamExt;

fn main() {
    trpl::block_on(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("Değer: {value}");
        }
    });
}
```

### Stream ve StreamExt Trait'leri

- **`Stream` trait:** Düşük seviyeli arayüz, `Iterator` ve `Future`'ı birleştirir
- **`StreamExt` trait:** `next()` metodu ve diğer yardımcı metodları sağlar

**Önemli:** `StreamExt`'i scope'a import etmezseniz `next()` metodunu kullanamazsınız!

```rust
use trpl::StreamExt; // Bu gerekli!
```

### Stream Kullanım Alanları

- Kuyruktaki öğeler zamanla hazır olduğunda
- Dosya sisteminden parça parça veri çekme (bellek yetersiz olduğunda)
- Ağdan zamanla gelen veriler
- Olayları gruplama (batching)
- Uzun süren işlemlere timeout koyma
- UI olaylarını sınırlama (throttling)

---

## 📖 Bölüm 17.5: Async İçin Trait'ler - Pin ve Unpin

### Future Trait'inin Derinlikleri

```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Ready(T),    // Future tamamlandı, değer hazır
    Pending,     // Henüz hazır değil, sonra tekrar dene
}
```

**`await` kelimesi arka planda şuna dönüşür:**
```rust
loop {
    match future.poll() {
        Poll::Ready(value) => return value,
        Poll::Pending => {
            // Runtime'a kontrolü bırak, başka işler yap
            // Hazır olduğunda tekrar uyan
        }
    }
}
```

### Pin Neden Gerekli?

Async bloklar için oluşturulan state machine'ler **kendi kendine referans** verebilir:

```rust
// Basitleştirilmiş örnek
enum MyFuture {
    Initial { data: String },
    AfterAwait { data_ref: *const String }, // Kendi içindeki veriye referans!
}
```

Eğer bu struct'ı bellekte taşırsanız, referans artık geçersiz bir adresi gösterir! **Pin**, bir değerin bellekte taşınmayacağını garanti eder.

### Unpin Marker Trait

```rust
// Unpin, bir tipin güvenli bir şekilde taşınabileceğini belirtir
// Çoğu tip otomatik olarak Unpin'dir (i32, String, vs.)

// async blokların future'ları Unpin DEĞİLDİR
// Çünkü kendi kendilerine referans verebilirler
```

**Kullanım:**
```rust
// ❌ Derlenmez
let mut futures: Vec<Box<dyn Future<Output = ()>>> = vec![];
futures.push(Box::new(async { /* ... */ }));
trpl::join_all(futures).await; // Hata: Unpin değil!

// ✅ Doğru: Pin kullan
let mut futures: Vec<Pin<Box<dyn Future<Output = ()>>>> = vec![];
futures.push(Box::pin(async { /* ... */ }));
trpl::join_all(futures).await; // Çalışır!
```

### Stream Trait Tanımı

```rust
trait Stream {
    type Item;
    
    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<Option<Self::Item>>;
}

trait StreamExt: Stream {
    async fn next(&mut self) -> Option<Self::Item>
    where
        Self: Unpin;
}
```

**`poll_next` dönüş tipi:** `Poll<Option<Self::Item>>`
- **`Poll`**: Hazır mı değil mi?
- **`Option`**: Başka öğe var mı?

---

## 📖 Bölüm 17.6: Future'lar, Task'lar ve Thread'ler

### Ne Zaman Hangisini Kullanmalı?

| İş Türü | Önerilen Yöntem | Neden? |
|---------|-----------------|---------|
| **CPU-bağımlı** (çok paralelleştirilebilir) | Thread | Her çekirdek tam kapasite çalışır |
| **I/O-bağımlı** (çok eşzamanlı) | Async | Binlerce bağlantıyı az kaynakla yönetir |
| **Her ikisi de** | Thread + Async | İkisini birlikte kullanın! |

### Thread ve Async'i Birlikte Kullanma

```rust
use std::{thread, time::Duration};

fn main() {
    let (tx, mut rx) = trpl::channel();

    // Thread: CPU-yoğun iş için
    thread::spawn(move || {
        for i in 1..11 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Async: I/O ve mesajlaşma için
    trpl::block_on(async {
        while let Some(message) = rx.recv().await {
            println!("{message}");
        }
    });
}
```

### Work Stealing (İş Çalma)

Modern runtime'lar (Tokio gibi) **multithreaded**'dır ve **work stealing** kullanır:
- Task'lar thread'ler arasında şeffaf bir şekilde taşınır
- Thread'lerin kullanım durumuna göre otomatik dengeleme yapılır
- Hem thread'leri hem task'ları gerektirir

### Gerçek Dünya Örneği

Video kodlama senaryosu:
- **Video kodlama**: Thread kullan (CPU-bağımlı)
- **UI'ya bildirim**: Async kanal kullan (I/O-bağımlı)

```rust
// Thread: Video kodlama (CPU-yoğun)
thread::spawn(|| {
    encode_video(); // Uzun süren CPU işlemi
    tx.send("Bitti!").unwrap();
});

// Async: UI güncellemeleri
trpl::block_on(async {
    while let Some(msg) = rx.recv().await {
        update_ui(msg);
    }
});
```

---

## 🎓 Özet ve En İyi Pratikler

### Temel Kavramlar

1. **Future**: Gelecekte hazır olacak bir değeri temsil eder
2. **async/await**: Async kodu senkron gibi yazmayı sağlayan sözdizimi
3. **Runtime**: Async kodu yöneten ve yürüten kütüphane
4. **Task**: Runtime tarafından yönetilen hafif eşzamanlı birim
5. **Stream**: Zaman içinde gelen öğe dizisi

### Altın Kurallar

✅ **DOĞRU:**
- I/O işlemleri için async kullanın
- Uzun süren CPU işlemlerinde `yield_now` ile kontrolü bırakın
- Thread ve async'i birlikte kullanmaktan çekinmeyin
- Future'ları `await` etmeyi unutmayın (tembel olduklarını unutmayın!)

❌ **YANLIŞ:**
- Async bloklarda uzun süre await olmadan çalışmayın
- CPU-yoğun işler için sadece async kullanmayın
- `main` fonksiyonunu async yapmaya çalışmayın
- `StreamExt`'i import etmeden stream kullanmaya çalışmayın

### Performans İpuçları

- Her satırda `yield_now` çağırmayın - ölçüm yapın
- Async state machine'in overhead'i vardır - gereksiz kullanmayın
- Work-stealing runtime'lar otomatik optimizasyon sağlar
- Gömülü sistemlerde async tek seçenek olabilir

---

Bu kapsamlı ders notları, Rust'ın async/await sisteminin tüm temel kavramlarını, pratik kullanımlarını ve derinliklerini kapsamaktadır. Artık Rust'ta güvenli ve etkili asenkron programlar yazmaya hazırsınız! 🚀

# DeepSeek Yorumu ---------------------------

Rust programlama dilinin 17. bölümü **Async ve Await** (Asenkron Programlama), dilin modern ve verimli eşzamanlılık modelini enine boyuna anlatır. Bu bölüm “Futures”, “async/await” sözdizimi ve asenkron çalışma zamanının (runtime) temellerini, kitabın en kapsamlı derslerinden biri olacak şekilde işler. Şimdi bu bölümü bir ders anlatır gibi sıfırdan, adım adım ve bol örnekle açıklayalım.

---

## 17.0 – Giriş: Asenkron Programlama Nedir?

Bilgisayar programlarında çoğu zaman işlemciyi beklemek zorunda kaldığımız anlar vardır: ağdan veri gelmesi, dosya okunması, zamanlayıcının dolması gibi. Geleneksel yöntemde işletim sistemi bir iş parçacığını (thread) durdurup başka bir iş parçacığına geçer. Ancak her iş parçacığının kendi yığını (stack) vardır ve binlerce iş parçacığı oluşturmak bellek açısından pahalıdır.

**Asenkron programlama** (asenkron I/O) ise tek bir iş parçacığı üzerinde birden fazla görevi, “bekleme” anlarında başka bir göreve geçerek yürütme fikrine dayanır. Rust’ta bu model *future* (gelecek) kavramı ve `async`/`await` anahtar kelimeleri ile gerçekleştirilir.

Bu bölüm boyunca şunları öğreneceğiz:
- `Future` trait’i ve `async` blokları / fonksiyonları
- `await` ile başka bir future’ın tamamlanmasını bekleme
- Aynı anda birden fazla işi yürütme (concurrency)
- Stream (akış) kavramı
- Derinlemesine: `Pin`, `Waker`, `Context` ve yürütücüler (executor)
- Async/await ile iş parçacıkları arasındaki ilişki

Şimdi sırayla alt başlıklara dalalım.

---

## 17.1 – Futures ve Async Sözdizimi

### Future Nedir?

`Future` trait’i, ileride bir değer üretecek bir hesaplamayı temsil eder. Standart kütüphanede (`std::future::Future`) şöyle tanımlanmıştır:

```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

- `Output`: Future tamamlandığında döneceği değerin tipi.
- `poll`: “Biraz daha ilerle, mümkünse sonucu ver” metodudur. `Poll::Ready(T)` dönerse future bitmiştir, `Poll::Pending` dönerse daha sonra tekrar yoklanmak üzere askıda kalır.

Ama endişelenmeyin; çoğu zaman `poll`’u siz çağırmazsınız. Yürütücü (executor) sizin adınıza yoklar.

### `async` Fonksiyonlar ve Bloklar

Rust’ta bir fonksiyonu asenkron yapmak için başına `async` eklenir:

```rust
async fn fetch_data() -> String {
    // bir ağ isteği yap, bekle, sonra döndür
    "veri".to_string()
}
```

Bu fonksiyonu çağırdığınızda hemen çalışmaz; size bir **future** döner. Bu future, çalıştırılmak üzere bir yürütücüye (executor) verilir. En yaygın yürütücülerden biri `tokio`’dur, ancak kitap basit bir özel yürütücü kullanarak kavramları öğretir. Biz de önce mantığını anlayalım, sonra üçüncü parti bir runtime ile devam ederiz.

`async` bloklar da vardır:

```rust
let future = async {
    let result = fetch_data().await;
    println!("{}", result);
};
```

Bu bir closure gibi düşünülebilir, ancak içindeki `.await` noktalarında durabilir.

### `.await` ile Bekleme

Bir future’ı çalıştırıp sonucunu almak için `.await` kullanılır. `.await` yalnızca `async` bir fonksiyon ya da blok içinde kullanılabilir.

```rust
let data = fetch_data().await;
```

`.await`, kontrolü yürütücüye geri verir: “Bu iş bitene kadar beni askıya al, başka bir future varsa onu çalıştır.” Bekleme bittiğinde kod kaldığı yerden devam eder.

### Yürütücü (Executor) Olmadan Çalışmaz

Saf Rust’ta bir future’ı çalıştırmak için bir “çalıştırıcı” gerekir. Kitapta, basit bir yürütücüyü kendimiz yazıyoruz (veya `futures` crate’indeki `block_on` gibi basit fonksiyonları kullanıyoruz). Özet fikir şudur: Yürütücü, tamamlanana kadar future’ı sürekli `poll` eder.

```rust
use futures::executor::block_on;

async fn hello() {
    println!("Merhaba, async dünya!");
}

fn main() {
    let future = hello();
    block_on(future); // future'ı çalıştır ve sonucu bekle
}
```

`block_on` mevcut iş parçacığını bloke ederek verilen future tamamlanana kadar çalıştırır. Gerçek asenkron uygulamalarda ise bir olay döngüsü (event loop) birden fazla future’ı aynı anda yürütür.

### Pin ve Neden Gerekli?

`.await` noktasında future’ın durumu bellekte taşınmamalıdır çünkü içindeki referanslar geçersiz hale gelebilir. Rust bunu `Pin` ile garanti altına alır. Şimdilik “`Pin`, bir değerin bellekteki yerinin değişmeyeceğini garanti eder” demek yeterli; detayları 17.5’te irdeleyeceğiz.

---

## 17.2 – Async ile Eşzamanlılık (Concurrency)

Asenkron programlamanın esas gücü, birden çok işi aynı anda yürütebilmektir. Bu bölümde `futures::join!` ve `tokio::join!` gibi araçlarla birden fazla future’ı eşzamanlı çalıştırmayı öğreniyoruz.

### `join!` Makrosu

İki future’ı aynı anda başlatıp ikisi de tamamlanana kadar beklemek için `join!` kullanılır. Örnek (`futures` crate’i ile):

```rust
use futures::join;

async fn get_weather() -> String { "Güneşli".into() }
async fn get_news() -> String { "Son dakika...".into() }

async fn dashboard() {
    let (weather, news) = join!(get_weather(), get_news());
    println!("Hava: {weather}, Haber: {news}");
}
```

Burada `get_weather()` ve `get_news()` aynı anda çalışır. Eğer birisi ağ gecikmesi yaşıyorsa diğeri ilerleyebilir. İkisi de `Ready` olduğunda `join!` makrosu her ikisinin sonucunu bir tuple olarak döndürür. Bu, sıralı `.await` kullanmaktan çok daha hızlıdır:

```rust
// kötü yaklaşım - sıralı
let weather = get_weather().await;
let news = get_news().await;
// önce hava durumu biter, sonra haber başlar
```

### `try_join!` ile Hata Yönetimi

Eğer future’lar `Result` döndürüyorsa, ilk hatada kısa devre yapmak için `try_join!` kullanılabilir. Bu, tüm future’lar tamamlanana kadar bekler ancak herhangi biri `Err` dönerse hemen o hatayı döndürür.

### `race!` ya da `select!` ile Seçim Yapma

Bazen “hangisi önce biterse onun sonucunu al” istersiniz. `futures` crate’indeki `select!` makrosu (ya da `race!`) tam bunu yapar. Örneğin bir zaman aşımı ile yarıştırmak:

```rust
use futures::{select, future::Either};
use std::time::Duration;

async fn slow_task() -> String {
    // 5 saniye süren bir iş
    "yavaş".into()
}

async fn timeout() {
    // 2 saniye bekle
    tokio::time::sleep(Duration::from_secs(2)).await;
}

async fn task_with_timeout() {
    select! {
        result = slow_task().fuse() => println!("Görev tamam: {result}"),
        _ = timeout().fuse() => println!("Zaman aşımı!"),
    }
}
```

`select!`, dallardan birisi bitene kadar tümünü yoklar; ilk biten dal kazanır, diğerleri iptal edilir (dropped). `fuse()` çağrısı, future’ın tamamlandıktan sonra tekrar yoklanmaması için `FusedFuture` garantisi sağlar (detayı 17.5’te).

---

## 17.3 – Herhangi Bir Sayıda Future ile Çalışma

Gerçek dünyada, “dinamik” sayıda future ile uğraşmanız gerekir. Örneğin bir vektör dolusu dosya indirme future’ınız olabilir. Bölüm 17.3 bu senaryoyu işler.

### `join_all`

`futures::future::join_all` fonksiyonu, bir future koleksiyonunu alır, hepsini eşzamanlı sürdürür ve tamamlandıklarında sonuçları bir vektör olarak döndürür.

```rust
use futures::future::join_all;

async fn download(url: &str) -> String {
    // indirme işlemi
    format!("{url} içeriği")
}

async fn download_all(urls: Vec<&str>) -> Vec<String> {
    let futures: Vec<_> = urls.into_iter().map(|url| download(url)).collect();
    join_all(futures).await
}
```

Bu, tüm indirme işlemlerinin aynı anda başlamasını sağlar. Ancak dikkat: `join_all` verilen tüm future’ları aynı anda yürütmeye çalışır. Çok fazla sayıda future (örneğin on binlerce) bir anda poll edildiğinde yürütücüyü şişirebilir. Gerçek uygulamalarda genellikle bir **eşzamanlılık sınırı** koymak istenir. Bunun için bir semafor veya tamponlu akış (buffered stream) kullanılabilir.

### Görev Spawn Etmek (Task Yaklaşımı)

`join_all` aynı görev (task) içinde çalışır; yani tek bir `async` fonksiyonun içinde tüm future’lar sırayla poll edilir. Eğer her bir future’ı bağımsız görevlere dönüştürmek isterseniz, bir async çalışma zamanının `spawn` fonksiyonu kullanılır. Örneğin `tokio::spawn`:

```rust
let handles: Vec<_> = urls.into_iter().map(|url| {
    tokio::spawn(async move {
        download(url).await
    })
}).collect();

for handle in handles {
    let result = handle.await.unwrap();
    println!("{}", result);
}
```

Bu durumda her indirme kendi görevinde çalışır ve çalışma zamanı bunları iş parçacıkları arasında dağıtabilir (çok çekirdekli kullanım). Kitap, spawn’ın artılarını ve eksilerini işler: Paylaşımlı durumlara dikkat edilmesi gerekir (örneğin `Arc<Mutex<>>`).

---

## 17.4 – Streams (Akışlar)

`Future` tek bir değer üretirken, `Stream` birden fazla değeri zaman içinde üretir. Rust’ta `Stream` trait’i (`futures::stream::Stream`) şöyledir (standart kütüphaneye henüz eklenmedi, `futures` crate’indedir):

```rust
pub trait Stream {
    type Item;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;
}
```

Bitiş sinyali `Poll::Ready(None)` ile verilir. Tıpkı bir `Iterator`’ın asenkron versiyonu gibi düşünebilirsiniz.

### Stream Oluşturma

- `stream::iter` ile bir iterator’dan stream yapılabilir.
- `stream::unfold` ile durum taşıyan stream üretilebilir.
- Kanal (channel) alıcısı bir stream’dir (örn. `tokio::sync::mpsc::Receiver`).

Örnek:

```rust
use futures::stream::{self, StreamExt};

let s = stream::iter(1..=5);
s.for_each(|n| async move {
    println!("Sayı: {n}");
}).await;
```

### Stream Tüketme

`StreamExt` çeşitli adaptörler ve sonlandırıcılar sunar:
- `.next().await` – sonraki öğeyi al
- `.for_each()` – her öğe için bir async closure çalıştır
- `.map()`, `.filter()`, `.fold()` vs.

Kitapta bir TCP sunucudan gelen bağlantıları bir stream olarak ele alma örneği vardır (tokio kullanarak):

```rust
let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
loop {
    let (stream, addr) = listener.accept().await.unwrap();
    tokio::spawn(async move {
        // bağlantıyı işle
    });
}
```

`listener.accept()` aslında bir stream’dir, ancak burada `next()` kullanılabilir. Kitap bu tür bir sonsuz döngüyü stream’e dönüştürmeyi anlatır.

### Stream ve Eşzamanlılık

Stream’lerle çalışırken `buffer_unordered` veya `for_each_concurrent` gibi yöntemlerle sınırlı eşzamanlılık sağlayabilirsiniz. Örneğin, bir stream’den gelen her işi en fazla 5 eşzamanlı future ile işlemek için:

```rust
stream::iter(urls)
    .map(|url| download(url))
    .buffer_unordered(5)  // en fazla 5 tanesi aynı anda
    .for_each(|data| async move { ... })
    .await;
```

Bu, bölüm 17.3’teki “çok fazla future” sorununa zarif bir çözümdür.

---

## 17.5 – Async’nin Derinliklerindeki Trait’ler

Bu bölüm, perde arkasını merak edenler için `Future`, `Pin`, `Waker` ve `Context`’i detaylandırır. Bir async fonksiyonunun derleyici tarafından nasıl bir state makinesine dönüştürüldüğünü anlarız.

### Future’ın Durum Makinesi

Bir `async fn` veya `async` blok, derleyici tarafından anonim bir veri yapısına dönüştürülür. Bu yapı, fonksiyonun içindeki her `.await` noktası için bir durum (state) saklar. Örneğin:

```rust
async fn example() -> i32 {
    let a = step_one().await;
    let b = step_two(a).await;
    b
}
```

Derleyici şöyle bir enum üretir:

```rust
enum ExampleFuture {
    State0 { ... },
    State1 { a: i32, ... },
    Done,
}
```

`.await` noktalarında `poll` çağrıldığında, ilgili durum çalıştırılır ve eğer içteki future `Pending` dönerse, dıştaki future da `Pending` döner ve yürütücüye kontrol verilir. Durum makinesi, bir sonraki `poll` çağrısında kaldığı yerden devam edecek şekilde bellekte tutulur.

### `Pin` ve Neden Bu Kadar Önemli

Durum makinesi kendi kendine referanslar (self-referential struct) içerebilir. Örneğin:

```rust
async {
    let x = read_to_string().await; // x bir String
    let y = &x;
    process(y).await;   // y, x'e referans
}
```

Burada `y`, aynı future yapısının içindeki `x` alanına bir referanstır. Eğer future bellekte taşınırsa (örneğin bir vektör yeniden boyutlandırıldığında), `y` geçersiz bir adres gösterir. Rust’ın güvenlik garantilerini bozmamak için bu tür future’ların **taşınması** engellenmelidir.

İşte `Pin` burada devreye girer. `Pin<P>` işaretçi tipi, işaret ettiği değerin bellekteki yerinin değişmeyeceğini garanti eder. Future’lar `poll` edilirken `Pin<&mut Self>` olarak alınır, böylece yürütücü onları hareket ettiremez. `Unpin` trait’i ise “ben taşınabilirim” diyen tipler içindir; çoğu standart tip `Unpin`’dir, ancak `async` blokları genelde değildir. Bu yüzden onları `Box::pin` ile heap’e koyarak veya `pin!` makrosuyla sabitleyerek çalışırız.

### `Waker` ve `Context`

`Future::poll` aldığı `Context` içinden bir `Waker`’a ulaşır. `Waker`, “hazır olduğumda beni uyandır” çağrısı yapmak için kullanılır. Çoğu future bir kaynağın (örneğin I/O) tamamlanmasını beklerken `Pending` döner ama bir yere `waker`’ı kaydeder. Kaynak hazır olduğunda `waker.wake()` çağrılır ve yürütücü ilgili görevi tekrar sıraya koyar.

Bu mekanizma sayesinde sürekli meşgul bekleme (busy polling) yapılmaz, işlemci boşa harcanmaz.

Kitap, basit bir zamanlayıcı future’ı (`TimerFuture`) örneği vererek `Waker` kullanımını somutlaştırır. İşte özet kod:

```rust
struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let mut shared = self.shared_state.lock().unwrap();
        if shared.completed {
            Poll::Ready(())
        } else {
            shared.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}
```

Başka bir iş parçacığı süre dolduğunda `shared.completed = true` yapıp `waker.wake()` çağırır. Bu, yürütücünün ilgili future’ı tekrar `poll` etmesini sağlar.

---

## 17.6 – Futures, Görevler (Tasks) ve İş Parçacıkları (Threads)

Bu son bölümde büyük resme bakılır: Asenkron programlama ile iş parçacıkları birbirinin rakibi değil, tamamlayıcısıdır.

### Çalışma Zamanı (Runtime) ve Görevler

Bir async çalışma zamanı (tokio, async-std, smol vb.) genellikle bir **iş parçacığı havuzu** ve bir **görev zamanlayıcısı** içerir. `spawn` ile yarattığınız her future bir **görev** (task) haline gelir. Zamanlayıcı, görevleri iş parçacıklarına dağıtır. Bir görev `.await` ile beklerken, iş parçacığı başka bir görevi çalıştırabilir.

Bu, M:N iş parçacığı modeline benzer: Çok sayıda hafif görev, az sayıda işletim sistemi iş parçacığı üzerinde çalışır. Bağlam değiştirme maliyeti çok düşüktür.

### Async vs. Threads: Ne Zaman Hangisi?

- **I/O ağırlıklı işler:** Async idealdir. Binlerce bağlantıyı birkaç iş parçacığıyla yönetebilirsiniz.
- **CPU ağırlıklı işler:** Uzun süreli hesaplamalar asenkron zamanlamayı bloke eder. Bu durumda `spawn_blocking` veya özel iş parçacıkları kullanılmalıdır.
- **Basitlik:** Eğer sadece birkaç eşzamanlı işiniz varsa, iş parçacıkları (`std::thread`) ve kanallar daha basit olabilir. Async, runtime bağımlılığı ve `Pin` gibi karmaşıklıklar getirir.

### Yürütücüyü Beslemek

Async kodun ilerlemesi için yürütücünün `poll` yapması gerekir. `#[tokio::main]` veya `#[async_std::main]` gibi makrolar bir main fonksiyonunu sarmalayarak gerekli altyapıyı kurar. İçeride bir olay döngüsü çalışır.

Kitap son olarak, bir yürütücüyü elle yazmanın ne kadar eğitici olduğunu ancak üretimde hazır bir runtime kullanmanın daha mantıklı olduğunu vurgular.

---

## Özet

Rust’ta asenkron programlama:
1. **`Future` trait** ve `async`/`await` sözdizimi ile yapılır.
2. `.await` noktalarında kontrol yürütücüye bırakılır; başka future’lar çalışabilir.
3. `join!`, `select!`, `try_join!` gibi makrolar eşzamanlılığı kolaylaştırır.
4. Dinamik sayıda future için `join_all` ya da görev `spawn` etme yöntemleri kullanılır.
5. **Stream**’ler, zaman içinde birden çok değer üreten asenkron yapılardır.
6. Derleyici `async` fonksiyonları state makinesine çevirir; `Pin` bellek güvenliğini sağlar, `Waker` ise bildirim tabanlı uyanmayı mümkün kılar.
7. Async, iş parçacıklarının yerini almaz; onları tamamlayarak yüksek verimli I/O işlemleri için hafif bir eşzamanlılık modeli sunar.

Bu temel bilgilerle artık Rust’ın asenkron dünyasına sıkı bir giriş yapmış oldunuz. Kitabın bu bölümünü dikkatle takip eden biri, hem `tokio` gibi kütüphaneleri etkin kullanabilir hem de kendi asenkron bileşenlerini yazabilecek donanıma sahip olur.