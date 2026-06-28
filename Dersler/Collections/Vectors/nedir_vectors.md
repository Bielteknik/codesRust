 'ta **Vector (`Vec<T>`)**, standart kütüphanenin en temel, en sık kullanılan ve en güçlü koleksiyon türüdür. Eğer bir programlama dilinde "dizi" (array) denildiğinde akla gelen dinamik yapı varsa,  'taki karşılığı Vector'dür.

İşte   Vector'ünün "nedir, neden kullanılır, nerelerde kullanılır" sorularını yanıtlayan detaylı ve kapsamlı anlatımı:

---

### 1. Vector Nedir? (Bellek Modeli)

Vector, **Heap (Öbek)** bellekte tutulan, **çalışma zamanında (runtime) boyutu büyüyüp küçülebilen** ve **aynı tipte** verileri **bellekte yan yana (contiguous)** tutan bir veri yapısıdır.

 'ta bir `Vec<T>` arka planda aslında 3 şeyi barındırır:
1. **Pointer (İşaretçi):** Heap'teki verinin başladığı adresi gösterir.
2. **Length (Uzunluk - `len`):** Vector'ün şu an kaç eleman içerdiğini söyler.
3. **Capacity (Kapasite - `cap`):** Vector'ün yeniden bellek tahsis etmeden (reallocation) kaç eleman tutabileceğini söyler.

> **Kural:** `len` her zaman `cap`'a eşit veya ondan küçüktür (`len <= cap`). `len == cap` olduğunda ve siz yeni bir eleman eklediğinizde, Vector arka planda daha büyük bir bellek alanı tahsis eder, eski verileri oraya kopyalar ve eski alanı siler.

---

### 2. Neden Vector Kullanılır? (Avantajları)

1. **Dinamik Boyut:** Array'lerin aksine, derleme zamanında boyutunu bilmek zorunda değilsiniz. Kullanıcıdan veri okurken veya bir dosyayı parse ederken boyut sonradan belli olur.
2. **Önbellek Dostu (Cache-Friendly):** Veriler bellekte yan yana (contiguous) durduğu için CPU önbelleği (Cache) tarafından çok hızlı okunur. Bu, `LinkedList` gibi dağınık bellek yapılarına göre **kat kat daha hızlı** olmasını sağlar.
3. **Zengin API:**  'ın standart kütüphanesi Vector'ler için yüzlerce optimize edilmiş metod sunar (`sort`, `binary_search`, `retain`, `dedup` vb.).
4. **Sıfır Maliyetli Soyutlama:** Generic yapısı sayesinde, Vector'ü kullanmanın manuel bellek yönetimi yapmaya kıyasla hiçbir performans cezası yoktur.

---

### 3. Temel İşlemler ve Sözdizimi

#### A. Oluşturma
``` 
fn main() {
    // 1. Boş bir vector (Tip belirtmek zorunludur)
    let mut v1: Vec<i32> = Vec::new();

    // 2. `vec!` makrosu ile (En yaygın ve pratik yöntem)
    let mut v2 = vec![1, 2, 3];

    // 3. Belirli bir değeri tekrarlayarak oluşturma
    let v3 = vec![0; 5]; // [0, 0, 0, 0, 0]
}
```

#### B. Eleman Ekleme ve Çıkarma
``` 
fn main() {
    let mut v = Vec::new();

    v.push(10); // Sonuna ekle
    v.push(20);
    v.insert(0, 5); // 0. indekse ekle (Diğer elemanları sağa kaydırır - O(n) maliyetli!)

    let son = v.pop(); // Son elemanı çıkar (Option<T> döndürür) -> Some(20)
    let ilk = v.remove(0); // Belirli indeksten çıkar -> 5
}
```

#### C. Okuma (Güvenli vs Güvensiz)
``` 
fn main() {
    let v = vec![10, 20, 30];

    // ❌ GÜVENSİZ: Sınır aşılırsa program PANIC verip çöker!
    // let deger = v[10]; 

    // ✅ GÜVENLİ: Option<T> döndürür, program çökmez.
    match v.get(10) {
        Some(x) => println!("Değer: {}", x),
        None => println!("İndeks sınır dışı!"),
    }
}
```

#### D. Döngü (Iteration)
``` 
fn main() {
    let mut v = vec![1, 2, 3];

    // Sadece Okuma (Referans ile)
    for i in &v {
        println!("{}", i);
    }

    // Değiştirme (Mutable Referans ile)
    for i in &mut v {
        *i *= 2; // Değeri 2 ile çarp
    }

    // Tüketme (Vector'ü yok ederek elemanları alma - Ownership)
    for i in v {
        println!("{}", i);
    }
    // println!("{:?}", v); // ❌ HATA! v artık yok, döngüde tüketildi.
}
```

---

### 4. Vector'ün En Kritik Özelliği: `len` vs `capacity` ve Performans

Vector'ü verimli kullanmanın sırrı **Yeniden Tahsis (Reallocation)** maliyetini anlamaktır.

``` 
fn main() {
    let mut v = Vec::new();
    println!("len: {}, cap: {}", v.len(), v.capacity()); // len: 0, cap: 0

    v.push(1); 
    // Arka planda: cap 4'e (veya 1'e) çıkar.
    println!("len: {}, cap: {}", v.len(), v.capacity()); // len: 1, cap: 4

    v.push(2); v.push(3); v.push(4); v.push(5);
    // 5. eleman eklendiğinde cap doldu! 
    // Arka planda: Yeni, daha büyük bir alan (genelde 2 katı, cap: 8) ayrılır, 
    // eski veriler kopyalanır, eski alan silinir.
    println!("len: {}, cap: {}", v.len(), v.capacity()); // len: 5, cap: 8
}
```

#### 🚀 Performans İpucu: `with_capacity`
Eğer bir Vector'e yaklaşık kaç eleman ekleyeceğinizi **biliyorsanız**, baştan kapasiteyi belirtin. Bu, gereksiz bellek kopyalama (reallocation) işlemlerini önler.

``` 
// Kötü: Dosyadan 10.000 satır okunacaksa, Vector defalarca yeniden boyutlanır.
let mut satirlar = Vec::new(); 

// İyi: Başlangıçta 10.000 elemanlık yer ayır, reallocation yapma.
let mut satirlar = Vec::with_capacity(10_000); 
```

---

### 5. Vector Nerede Kullanılmalı? (Use Cases & Karşılaştırmalar)

Vector'ü her yerde kullanmak doğru değildir. Duruma göre en iyi seçeneği belirlemek için şu karşılaştırmaları yapmalısınız:

#### A. Vector vs Array (`[T; N]`)
* **Ne zaman Array kullanmalı?** Boyutu derleme zamanında kesin olarak belliyse ve küçükse. (Örn: RGB renkleri `[u8; 3]`, 3D koordinatlar `[f32; 3]`, haftanın günleri). Array'ler Stack'te tutulur ve çok daha hızlıdır.
* **Ne zaman Vector kullanmalı?** Boyut çalışma anında belli olacaksa veya 1000+ eleman gibi büyük veriler söz konusuysa (Stack Overflow riskini önlemek için Heap'e ihtiyaç vardır).

#### B. Vector vs LinkedList
* **Kural:**  'ta **neredeyse asla `LinkedList` kullanmayın.** 
* **Neden?** Vector'ün bellekte yan yana durması (Cache Locality), LinkedList'in pointer'ları takip etmesinden çok daha hızlıdır. LinkedList'in tek avantajı olan "ortadan hızlı silme/ekleme" işlemi bile modern CPU mimarisinde Vector'ün `remove()` işleminden daha yavaş kalabilir.

#### C. Vector vs VecDeque (`std::collections::VecDeque`)
* **Ne zaman Vector kullanmalı?** Elemanları sadece **sondan** ekleyip çıkaracaksanız (`push`, `pop`). Vector'ün sonu optimize edilmiştir.
* **Ne zaman VecDeque kullanmalı?** Elemanları **baştan** da ekleyip çıkarmanız gerekiyorsa (`push_front`, `pop_front`). Örn: Bir iş kuyruğu (Job Queue) veya BFS algoritması. Vector'de `insert(0, x)` yapmak tüm elemanları sağa kaydırmayı gerektirir (O(n) maliyet), VecDeque'de bu O(1)'dir.

#### D. Vector vs HashSet / HashMap
* **Ne zaman Vector kullanmalı?** Sıralama önemliyse, indeks ile erişim gerekiyorsa veya veri kümesi küçükse (örn. 10-20 eleman).
* **Ne zaman Hash yapısı kullanmalı?** "Bu eleman listede var mı?" (contains) sorusunu milyonlarca eleman arasında hızlıca (O(1)) sormanız gerekiyorsa. Vector'de bu arama O(n)'dir.

---

### 6. İleri Seviye ve İdiomatik (Idiomatic) Vector İşlemleri

 çılar ( aceans) Vector'leri manipüle ederken `for` döngüleri yerine standart kütüphane metodlarını tercih ederler:

#### A. `retain` (Filtreleme)
Vector içindeki belirli bir koşulu sağlamayan elemanları silmek için en verimli yoldur.
``` 
let mut sayilar = vec![1, 2, 3, 4, 5, 6];
// Sadece çift sayıları tut
sayilar.retain(|&x| x % 2 == 0); 
// sayilar artık: [2, 4, 6]
```

#### B. `drain` (Aralık Silme ve Tüketme)
Belirli bir aralıktaki elemanları silip, onlar üzerinde işlem yapmak veya başka bir yere taşımak için kullanılır.
``` 
let mut v = vec![1, 2, 3, 4, 5];
let drained: Vec<i32> = v.drain(1..4).collect(); // 1, 2, 3 indekslerini (2,3,4) çıkar
// v artık: [1, 5]
// drained: [2, 3, 4]
```

#### C. `extend` (Başka bir koleksiyonu ekleme)
``` 
let mut v1 = vec![1, 2];
let v2 = vec![3, 4, 5];
v1.extend(v2); // v1 artık: [1, 2, 3, 4, 5]
```

#### D. `sort` ve `dedup` (Sıralama ve Tekrarları Silme)
``` 
let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
v.sort();      // Sırala: [1, 1, 2, 3, 4, 5, 5, 6, 9]
v.dedup();     // Yan yana tekrarları sil: [1, 2, 3, 4, 5, 6, 9]
```

---

### 7. Sık Yapılan Hatalar ve Tuzaklar

1. **Döngü İçinde Vector'ü Değiştirmeye Çalışmak:**
   ``` 
   let mut v = vec![1, 2, 3];
   // ❌ HATA: v'yi hem okuyup (for) hem de değiştiriyorsunuz (push). 
   // Borrow checker buna izin vermez.
   for i in &v {
       v.push(i * 2); 
   }
   ```
   *Çözüm:* Önce veriyi toplayın, sonra Vector'e ekleyin veya `retain`/`drain` gibi metodları kullanın.

2. **Büyük Struct'ları Vector'de Tutmak:**
   Eğer Vector'ün içindeki elemanlar çok büyükse (örn. 1000 byte'lık bir struct), Vector'ü sıralamak (`sort`) veya eleman taşımak çok yavaş olur.
   *Çözüm:* Vector'ün içine struct'ın kendisini değil, **referansını** veya **ID/Index'ini** koyun.

3. **`&str` yerine `String` Tercihi:**
   Vector'ün içine metin koyacaksanız ve bu metinler dışarıdan bir yerden (örn. dosya okuma) geliyorsa, `&str` (lifetime yönetimi zorlaşır) yerine `String` kullanmak genellikle daha kolaydır. Ancak metinler sabitse `Vec<&str>` daha performanslıdır.

### Özet
**Vector (`Vec<T>`)**,  'ın iş atıdır. Dinamik boyut gerektiren, sıralı ve aynı tipte verileri tutmanız gereken her durumda (API yanıtları, dosya satırları, oyun nesneleri, kullanıcı girdileri) ilk başvurmanız gereken yapı Vector'dür. Bellekte contiguous (bitişik) durması, onu modern işlemciler için en optimize edilmiş koleksiyon yapar.