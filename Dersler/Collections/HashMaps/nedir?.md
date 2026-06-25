 'ta **HashMap**, verileri **Anahtar-Değer (Key-Value)** çiftleri halinde saklayan, standart kütüphanenin en güçlü ve en sık kullanılan koleksiyonlarından biridir. 

Gerçek hayattaki bir sözlük (kelime -> tanım) veya telefon rehberi (isim -> numara) mantığıyla çalışır. Python'daki `dict`, JavaScript'teki `Map/Object`, Java'daki `HashMap` yapısının karşılığıdır.

İşte   HashMap yapısının "nedir, neden kullanılır, nerelerde kullanılır" sorularını yanıtlayan detaylı anlatımı:

---

### 1. HashMap Nedir? (Çalışma Mantığı)

HashMap, arka planda bir **Hash Tablosu (Hash Table)** veri yapısı kullanır. 
Bir anahtar (key) verdiğinizde, HashMap bu anahtarı bir **Hash Fonksiyonundan** geçirir. Bu fonksiyon, anahtarı bellekteki belirli bir adrese (bucket/kova) dönüştürür. 

*   **Ekleme, Silme ve Arama** işlemleri ortalama olarak **$O(1)$** (sabit zaman) karmaşıklığına sahiptir. Yani içinde 10 öğe de olsa 10 milyon öğe de olsa, bir veriyi bulma süresi neredeyse aynıdır.

⚠️ **Önemli   Detayı:** `Vec` ve `String` standart kütüphaneye (prelude) dahil edilirken, `HashMap` **dahil edilmez**. Kullanmak için dosyanın başına şu satırı eklemeniz şarttır:
``` 
use std::collections::HashMap;
```

---

### 2. Neden HashMap Kullanılır? (Avantajları)

1.  **İnanılmaz Hız:** $O(1)$ arama süresi sayesinde, büyük veri setlerinde bir veriyi bulmak için tüm listeyi taramanıza gerek kalmaz.
2.  **Anlamlı Erişim:** Array veya Vector'lerde `liste[0]`, `liste[1]` gibi anlamsız indeksler kullanırken, HashMap'te `puanlar["Ali"]` gibi anlamlı anahtarlarla erişirsiniz.
3.  **Tekillik (Uniqueness):** Bir HashMap'te aynı anahtardan (key) sadece **bir tane** bulunabilir. Bu, verilerin tekrar etmesini otomatik olarak engeller.

---

### 3. Temel İşlemler ve Sözdizimi

#### A. Oluşturma ve Ekleme
``` 
use std::collections::HashMap;

fn main() {
    // Boş bir HashMap oluşturma
    let mut puanlar = HashMap::new();

    // Anahtar-Değer ekleme (insert)
    // Anahtarlar String, Değerler i32 tipinde
    puanlar.insert(String::from("Ali"), 90);
    puanlar.insert(String::from("Ayşe"), 85);
    puanlar.insert(String::from("Mehmet"), 70);

    // Eğer aynı anahtarı tekrar eklerseniz, eski değerin üzerine yazar.
    // insert metodu, eğer eski bir değer varsa onu Option<V> olarak döndürür.
    let eski_deger = puanlar.insert(String::from("Ali"), 95); 
    println!("Eski değer: {:?}", eski_deger); // Some(90)
}
```

#### B. Okuma (Güvenli Erişim)
**Altın Kural:** HashMap'ten okuma yaparken asla köşeli parantez `map["anahtar"]` kullanmayın (çünkü anahtar yoksa program çöker). Bunun yerine `.get()` metodunu kullanın.

``` 
// .get() metodu Option<&V> döndürür. (Referans döndürür, sahipliği almaz)
match puanlar.get(&String::from("Ali")) {
    Some(puan) => println!("Ali'nin puanı: {}", puan),
    None => println!("Ali sistemde kayıtlı değil."),
}
```

#### C. Üzerinde Döngü (Iteration)
HashMap'ler **sırasızdır (unordered)**. Eklediğiniz sırayla dönmeyi garanti etmez.

``` 
// Hem anahtarı hem değeri okuma
for (anahtar, deger) in &puanlar {
    println!("{}: {}", anahtar, deger);
}

// Sadece anahtarları okuma
for anahtar in puanlar.keys() {
    println!("{}", anahtar);
}

// Sadece değerleri okuma
for deger in puanlar.values() {
    println!("{}", deger);
}
```

#### D. Silme
``` 
// Belirli bir anahtarı silme. Silinen değeri Option<V> olarak döndürür.
let silinen = puanlar.remove(&String::from("Mehmet"));
```

---

### 4.  'ın Süper Gücü: Entry API (Çok Önemli!)

HashMap kullanırken en sık yapılan işlem şudur: *"Eğer bu anahtar varsa değerini güncelle, yoksa yeni bir değer ekle."* 

Bunu `if/else` ve `.get()` ile yapmak hem uzun hem de yavaştır (anahtarı iki kez hash'ler).   bunun için **`entry`** API'sini sunar.

#### Klasik (Kötü) Yöntem:
``` 
let mut sayac = HashMap::new();
let kelime = String::from("merhaba");

if !sayac.contains_key(&kelime) {
    sayac.insert(kelime.clone(), 0);
}
*sayac.get_mut(&kelime).unwrap() += 1;
```

#### İdiomatik (Entry API) Yöntem:
``` 
let mut sayac = HashMap::new();
let kelime = String::from("merhaba");

// "merhaba" anahtarı yoksa 0 ekle, varsa mevcut değeri referans olarak al.
// Sonra o referansın gösterdiği değeri 1 artır.
*sayac.entry(kelime).or_insert(0) += 1;
```

**Gerçek Dünya Örneği: Kelime Frekansı Sayacı**
``` 
use std::collections::HashMap;

fn main() {
    let metin = "merhaba dünya merhaba   merhaba dünya";
    let mut frekanslar = HashMap::new();

    for kelime in metin.split_whitespace() {
        // Entry API'nin en güzel kullanımı:
        let sayac = frekanslar.entry(kelime).or_insert(0);
        *sayac += 1;
    }

    println!("{:?}", frekanslar); 
    // Çıktı (sıra değişebilir): {"merhaba": 3, "dünya": 2, " ": 1}
}
```

---

### 5. HashMap Nerede Kullanılmalı? (Use Cases & Karşılaştırmalar)

#### A. HashMap vs Vector (`Vec<T>`)
*   **Ne zaman Vector kullanmalı?** Verilerinizin bir sırası varsa, indeks ile erişmeniz gerekiyorsa veya verileri eklediğiniz sırayla tekrar tekrar gezecekseniz.
*   **Ne zaman HashMap kullanmalı?** "Bu eleman listede var mı?" (contains) sorusunu sıkça soracaksanız veya bir veriyi ismi/ID'si ile hızlıca bulmanız gerekiyorsa. 
    *   *Örnek:* 10.000 kullanıcının ID'sine göre profilini çekmek. Vector'de bu $O(n)$ (tarama) iken, HashMap'te $O(1)$ (anlık) sürer.

#### B. HashMap vs BTreeMap
*   **Ne zaman HashMap kullanmalı?** Sıralama önemli değilse, sadece hızlı ekleme/okuma gerekiyorsa.
*   **Ne zaman BTreeMap kullanmalı?** Anahtarların **alfabetik veya sayısal olarak sıralı** tutulması gerekiyorsa veya "10 ile 20 arasındaki tüm anahtarları getir" (range query) gibi işlemler yapacaksanız. `BTreeMap` $O(\log n)$ hızındadır ama sıralıdır.

#### C. HashMap vs Struct (Yapı)
*   **Ne zaman Struct kullanmalı?** Anahtarlarınız (alan adlarınız) **derleme zamanında (compile-time)** kesin olarak belliyse. (Örn: `Kullanici { isim, yas }`). Struct çok daha hızlıdır ve tip güvenliği sağlar.
*   **Ne zaman HashMap kullanmalı?** Anahtarlarınız **çalışma zamanında (runtime)** belliyse veya dinamik olarak değişiyorsa. (Örn: Bir JSON dosyasından okunan ve alanları önceden bilinmeyen veriler).

---

### 6. Dikkat Edilmesi Gereken Tuzaklar (Gotchas)

1.  **Sırasızlık (Non-deterministic Order):**
    HashMap'e `A, B, C` sırasıyla ekleseniz bile, döngüye soktuğunuzda `C, A, B` sırasıyla dönebilir. Eğer sıralı bir çıktı istiyorsanız, HashMap'i bir Vector'e aktarıp sıralamanız (`sort`) veya `BTreeMap` kullanmanız gerekir.

2.  **Trait Gereksinimleri (`Eq` ve `Hash`):**
    HashMap'e anahtar (key) olarak koyacağınız her tip, `Eq` (eşitlik) ve `Hash` (hashlenebilirlik) trait'lerini implemente etmelidir. 
    *   `i32`, `String`, `bool` gibi tipler bunu otomatik yapar.
    *   Kendi yazdığınız `struct`'ları anahtar olarak kullanmak isterseniz, `#[derive(Eq, Hash, PartialEq)]` eklemeniz gerekir.
    *   *Not:* Değer (value) olarak her türlü tipi kullanabilirsiniz.

3.  **Sahiplik (Ownership) ve Borrowing:**
    ``` 
    let mut map = HashMap::new();
    let anahtar = String::from("test");
    
    map.insert(anahtar, 10); 
    // println!("{}", anahtar); // ❌ HATA! 'anahtar'ın sahipliği HashMap'e geçti.
    
    // Eğer anahtarı sonra da kullanacaksanız klonlamalısınız:
    map.insert(anahtar.clone(), 10);
    ```

4.  **`f32` ve `f64` Anahtar Olamaz:**
    Ondalıklı sayılar (`f32`, `f64`) `Eq` trait'ini implemente etmez (çünkü `NaN == NaN` false döner ve hash fonksiyonları tutarsız çalışır). Bu yüzden ondalıklı sayıları HashMap anahtarı yapamazsınız.

---

### 7. Gerçek Dünya Örneği: Basit Bir Önbellek (Cache)

HashMap'lerin en yaygın kullanım alanlarından biri, pahalı işlemlerin sonuçlarını saklamaktır (Memoization/Cache).

``` 
use std::collections::HashMap;

// Pahalı bir işlem (Örn: Veritabanı sorgusu veya ağır matematik)
fn pahalı_hesaplama(x: u64) -> u64 {
    println!("Hesaplanıyor... {}", x);
    x * x 
}

struct Ozbellek {
    degerler: HashMap<u64, u64>,
}

impl Ozbellek {
    fn yeni() -> Self {
        Ozbellek { degerler: HashMap::new() }
    }

    fn deger_al(&mut self, x: u64) -> u64 {
        // Entry API ile: Varsa al, yoksa hesapla ve ekle.
        self.degerler.entry(x).or_insert_with(|| pahalı_hesaplama(x))
    }
}

fn main() {
    let mut cache = Ozbellek::yeni();

    println!("İstek 1: {}", cache.deger_al(5)); // Hesaplar (Çıktı: Hesaplanıyor... 5)
    println!("İstek 2: {}", cache.deger_al(5)); // Cache'den okur (Hesaplamaz!)
    println!("İstek 3: {}", cache.deger_al(10)); // Hesaplar
}
```

### Özet
**HashMap**, anahtar-değer ilişkisi kurmanız ve verilere **anında (O(1))** erişmeniz gerektiğinde kullanacağınız birincil araçtır.  'ın `Entry API`'si, bu koleksiyonu manipüle etmeyi diğer dillere göre çok daha güvenli, okunaklı ve performanslı hale getirir. Sıralama önemli değilse ve anahtarlar derleme zamanında belli değilse, her zaman HashMap'i tercih edin.