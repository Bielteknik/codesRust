 'ta **`HashSet`**, sadece **benzersiz (unique)** elemanları saklayan, sırasız bir koleksiyondur. 

Matematikteki "Küme" (Set) kavramının tam karşılığıdır. Bir kümede aynı elemandan sadece bir tane bulunabilir. Arka planda aslında bir `HashMap` yapısı kullanır, ancak değer (value) kısmı boş bırakılmıştır (sadece anahtarlar tutulur).

İşte   `HashSet` yapısının detaylı anlatımı:

---

### 1. HashSet Nedir? (Çalışma Mantığı)

`HashSet`, elemanları bir **Hash Fonksiyonu** kullanarak bellekteki rastgele bir adrese yerleştirir. Bu sayede bir elemanın küme içinde olup olmadığını kontrol etmek **ortalama $O(1)$** (sabit zaman) sürer. 

⚠️ **Önemli Kural:** Tıpkı `HashMap` gibi, `HashSet` de standart kütüphaneye (prelude) dahil değildir. Kullanmak için dosyanın başına şu satırı eklemeniz şarttır:
``` 
use std::collections::HashSet;
```

---

### 2. Neden HashSet Kullanılır? (Avantajları)

1. **Otomatik Tekillik (Uniqueness):** Bir elemanı `HashSet`'e eklemeye çalıştığınızda, eğer o eleman zaten varsa, ekleme işlemi başarısız olur. Tekrar eden verileri filtrelemek için mükemmeldir.
2. **Çok Hızlı Arama ($O(1)$):** Bir elemanın listede olup olmadığını kontrol etmek (`contains`), `Vec`'teki gibi $O(n)$ (tüm listeyi tarama) değil, $O(1)$ (anlık) sürer.
3. **Gelişmiş Küme İşlemleri:** İki farklı `HashSet` arasında birleşim (union), kesişim (intersection) ve fark (difference) gibi matematiksel işlemleri tek satırda yapabilirsiniz.

---

### 3. Temel İşlemler ve Sözdizimi

#### A. Oluşturma ve Eleman Ekleme
``` 
use std::collections::HashSet;

fn main() {
    let mut meyveler = HashSet::new();

    // Eleman ekleme (insert)
    // insert metodu `bool` döndürür: 
    // Eğer eleman YENİ ise `true`, zaten VARSA `false` döner.
    let eklendi_mi = meyveler.insert("Elma"); 
    println!("Elma eklendi mi? {}", eklendi_mi); // true

    let eklendi_mi2 = meyveler.insert("Elma");
    println!("Elma tekrar eklendi mi? {}", eklendi_mi2); // false (Çünkü zaten var!)

    meyveler.insert("Armut");
    meyveler.insert("Muz");

    println!("Meyveler: {:?}", meyveler); // Sıra garantili değildir!
}
```

#### B. Eleman Kontrolü ve Silme
``` 
// contains: Eleman var mı? (Çok hızlıdır - O(1))
if meyveler.contains(&"Armut") {
    println!("Sepette armut var.");
}

// remove: Elemanı siler. Varsa `true`, yoksa `false` döner.
let silindi = meyveler.remove(&"Muz");
```

#### C. Döngü (Iteration)
``` 
// HashSet sırasızdır. Eklediğiniz sırada dönmeyi garanti etmez.
for meyve in &meyveler {
    println!("{}", meyve);
}
```

---

### 4. HashSet'in Süper Gücü: Küme İşlemleri (Set Operations)

İki farklı `HashSet` üzerinde matematiksel küme işlemleri yapabilirsiniz. Bu metodlar **Iterator** döndürür, bu yüzden sonuçları görmek için `.collect()` kullanmanız gerekir.

``` 
use std::collections::HashSet;

fn main() {
    let set_a: HashSet<i32> = [1, 2, 3, 4].iter().cloned().collect();
    let set_b: HashSet<i32> = [3, 4, 5, 6].iter().cloned().collect();

    // 1. Kesişim (Intersection): Her iki kümede de ortak olan elemanlar
    let kesisim: HashSet<_> = set_a.intersection(&set_b).cloned().collect();
    println!("Kesişim: {:?}", kesisim); // {3, 4}

    // 2. Birleşim (Union): İki kümedeki tüm elemanlar (tekrarsız)
    let birlesim: HashSet<_> = set_a.union(&set_b).cloned().collect();
    println!("Birleşim: {:?}", birlesim); // {1, 2, 3, 4, 5, 6}

    // 3. Fark (Difference): A'da olup B'de olmayanlar
    let fark: HashSet<_> = set_a.difference(&set_b).cloned().collect();
    println!("A - B Farkı: {:?}", fark); // {1, 2}

    // 4. Simetrik Fark (Symmetric Difference): Ya A'da ya B'de olanlar (ortak olmayanlar)
    let simetrik_fark: HashSet<_> = set_a.symmetric_difference(&set_b).cloned().collect();
    println!("Simetrik Fark: {:?}", simetrik_fark); // {1, 2, 5, 6}
}
```

---

### 5. HashSet Nerede Kullanılmalı? (Use Cases & Karşılaştırmalar)

#### A. HashSet vs Vector (`Vec<T>`)
*   **Ne zaman Vector kullanmalı?** Elemanların **sırası önemliyse**, tekrar eden elemanlara izin veriliyorsa veya eleman sayısı çok küçükse (örn. 10-20 eleman).
*   **Ne zaman HashSet kullanmalı?** 
    *   Bir listedeki **tekrar eden elemanları silmek** istiyorsanız (Deduplication).
    *   Büyük bir listede (örn. 100.000 eleman) **"bu eleman listede var mı?"** sorusunu sıkça soracaksanız. `Vec`'te bu tarama yavaştır, `HashSet`'te anlıktır.

#### B. HashSet vs BTreeSet
*   **Ne zaman HashSet kullanmalı?** Sıralama önemli değilse, sadece hızlı ekleme/çıkarma/arama gerekiyorsa.
*   **Ne zaman BTreeSet kullanmalı?** Elemanların **alfabetik veya sayısal olarak sıralı** tutulması gerekiyorsa veya "10 ile 20 arasındaki tüm elemanları getir" (range query) gibi işlemler yapacaksanız. `BTreeSet` $O(\log n)$ hızındadır ama sıralıdır.

#### C. HashSet vs HashMap
*   **Ne zaman HashMap kullanmalı?** Bir anahtara karşılık gelen bir **değer (value)** saklamanız gerekiyorsa (Örn: `KullanıcıID -> KullanıcıAdı`).
*   **Ne zaman HashSet kullanmalı?** Sadece elemanın **var olup olmadığını** kontrol ediyorsanız ve bir değere ihtiyacınız yoksa (Örn: `KullanıcıID`'nin sisteme kayıtlı olup olmadığını kontrol etmek).

---

### 6. Dikkat Edilmesi Gereken Tuzaklar (Gotchas)

1. **Sırasızlık (Non-deterministic Order):**
   `HashSet`'e `A, B, C` ekleseniz bile, döngüye soktuğunuzda `C, A, B` şeklinde dönebilir. Eğer sıralı bir çıktı istiyorsanız, `HashSet`'i bir `Vec`'e aktarıp sıralamanız (`sort`) veya `BTreeSet` kullanmanız gerekir.

2. **Trait Gereksinimleri (`Eq` ve `Hash`):**
   `HashSet`'e ekleyeceğiniz her tip, `Eq` ve `Hash` trait'lerini implemente etmelidir. 
   *   `i32`, `String`, `bool` gibi tipler bunu otomatik yapar.
   *   Kendi `struct`'larınızı eklemek isterseniz, `#[derive(Eq, Hash, PartialEq)]` eklemeniz gerekir.

3. **`f32` ve `f64` Eklenemez:**
   Ondalıklı sayılar (`f32`, `f64`) `Eq` trait'ini implemente etmez (çünkü `NaN == NaN` false döner). Bu yüzden ondalıklı sayıları `HashSet`'e eleman olarak ekleyemezsiniz.

---

### 7. Gerçek Dünya Örnekleri

#### Örnek 1: Bir Listedeki Tekrarları Silmek (Deduplication)
``` 
use std::collections::HashSet;

fn main() {
    let kelimeler = vec!["elma", "armut", "elma", "muz", "armut", "kiraz"];
    
    // Vector'ü HashSet'e çevirerek tekrarları otomatik sileriz
    let benzersiz_kelimeler: HashSet<&str> = kelimeler.into_iter().collect();
    
    // Eğer tekrar Vector istiyorsak:
    let sonuc_vec: Vec<&str> = benzersiz_kelimeler.into_iter().collect();
    
    println!("{:?}", sonuc_vec); // Sıra karışık olabilir: ["muz", "elma", "armut", "kiraz"]
}
```

#### Örnek 2: İki Kullanıcının Ortak Arkadaşlarını Bulmak
``` 
use std::collections::HashSet;

fn main() {
    let ali_arkadaslari: HashSet<&str> = ["Ayşe", "Mehmet", "Veli", "Fatma"].into_iter().collect();
    let ayse_arkadaslari: HashSet<&str> = ["Mehmet", "Veli", "Zeynep", "Can"].into_iter().collect();

    // Kesişim (Intersection) ile ortak arkadaşları buluyoruz
    let ortak_arkadaslar: HashSet<_> = ali_arkadaslari.intersection(&ayse_arkadaslari).collect();

    println!("Ali ve Ayşe'nin ortak arkadaşları: {:?}", ortak_arkadaslar); 
    // Çıktı: {"Mehmet", "Veli"}
}
```

#### Örnek 3: Ziyaret Edilen Yerleri Takip Etmek
``` 
use std::collections::HashSet;

fn main() {
    let mut ziyaret_edilenler = HashSet::new();
    let sehirler = vec!["İstanbul", "Ankara", "İstanbul", "İzmir", "Ankara"];

    for sehir in sehirler {
        // insert `false` döndürdüyse, bu şehir daha önce ziyaret edilmiş demektir.
        if !ziyaret_edilenler.insert(sehir) {
            println!("{} zaten ziyaret edilmişti, tekrar eklenmedi.", sehir);
        } else {
            println!("{} ilk kez ziyaret edildi!", sehir);
        }
    }
}
```

---

### Özet ve Altın Kurallar

*   **`HashSet` = Benzersizlik + Hız:** Bir veri kümesinde tekrarları önlemek ve hızlı arama yapmak istiyorsanız ilk tercihiniz olmalıdır.
*   **`insert` Metodunu Akıllıca Kullanın:** `insert` metodunun `bool` döndürdüğünü unutmayın. Bu, elemanın daha önce eklenip eklenmediğini kontrol etmek için harika bir kısayoldur.
*   **Küme İşlemlerini Ezberleyin:** `intersection`, `union`, `difference` metodları, iki liste arasındaki ilişkileri bulmak için yazacağınız iç içe `for` döngülerini tamamen ortadan kaldırır.
*   **Sıra Beklemeyin:** `HashSet`'in elemanları sıralı tuttuğunu asla varsaymayın. Sıra önemliyse `BTreeSet` kullanın veya sonucu `Vec`'e alıp sıralayın.
*   **Performans:** İçinde 10 eleman da olsa 10 milyon eleman da olsa, `contains` metodu neredeyse aynı sürede (anlık) çalışır. Büyük veri setlerinde `Vec::contains` yerine her zaman `HashSet` tercih edin.