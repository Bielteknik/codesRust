#   Koleksiyonlar (Collections) Detaylı Ders Anlatımı

 'ta **Koleksiyonlar (Collections)**, birden fazla değeri tek bir yapı altında toplayan, boyutları **çalışma zamanında (runtime) büyüyüp küçülebilen** ve verileri **Heap (Öbek)** bellekte saklayan veri yapılarıdır.

Daha önce öğrendiğimiz **Array (Dizi)** ve **Tuple (Demet)** yapılarının boyutları derleme zamanında sabittir ve Stack'te tutulur. Koleksiyonlar ise bu kısıtlamayı ortadan kaldırır.

 'ın standart kütüphanesinde en çok kullanılan **3 temel koleksiyon tipi** vardır:
1. **`Vec<T>`** (Vektör)
2. **`String`** (Metin Dizisi)
3. **`HashMap<K, V>`** (Hash Haritası)

---

## 1. Vektörler (`Vec<T>`)

Vektörler, bellekte yan yana (contiguous) duran, **aynı tipte** verileri tutan dinamik dizilerdir. C++'daki `std::vector`, Python'daki `list` veya Java'daki `ArrayList` yapısına karşılık gelir.

### 1.1. Vektör Oluşturma
``` 
fn main() {
    // 1. Boş bir vektör oluşturma (Tip belirtmek zorunludur çünkü içi boş)
    let mut v: Vec<i32> = Vec::new();

    // 2. `vec!` makrosu ile başlangıç değerleri verme (En yaygın yöntem)
    let mut sayilar = vec![1, 2, 3];
    
    // 3. Aynı değeri tekrarlayarak vektör oluşturma
    let sifirlar = vec![0; 5]; // [0, 0, 0, 0, 0]
}
```

### 1.2. Vektöre Eleman Ekleme ve Çıkarma
``` 
fn main() {
    let mut v = Vec::new();

    v.push(5); // Sonuna eleman ekle
    v.push(6);
    v.push(7);
    v.push(8);

    let son_eleman = v.pop(); // Son elemanı çıkar ve döndür (Option<T>)
    println!("Çıkarılan: {:?}", son_eleman); // Some(8)
    println!("Vektör: {:?}", v); // [5, 6, 7]
}
```

### 1.3. Vektörden Okuma (Erişim)
``` 
fn main() {
    let v = vec![10, 20, 30, 40, 50];

    // 1. Köşeli parantez ile doğrudan erişim (Sınır aşılırsa PANIC verir!)
    let ucuncu = &v[2]; 
    println!("Üçüncü eleman: {}", ucuncu); // 30

    // 2. `get` metodu ile güvenli erişim (Option<T> döndürür, PANIC vermez)
    match v.get(10) {
        Some(deger) => println!("Onuncu eleman: {}", deger),
        None => println!("Onuncu eleman yok!"), // Bu çalışacak
    }
}
```

### 1.4. Vektör Üzerinde Döngü (Iteration)
``` 
fn main() {
    let mut v = vec![100, 32, 57];

    // Sadece Okuma (Borrowing)
    for i in &v {
        print!("{} ", i);
    }
    println!();

    // Değiştirme (Mutable Borrowing)
    for i in &mut v {
        *i += 50; // Referansın gösterdiği değeri değiştir
    }
    println!("Güncellenmiş: {:?}", v); // [150, 82, 107]
}
```

---

## 2. Metin Dizileri (`String`)

 'ta metin işlemleri için iki temel tip vardır: `&str` (String Slice - sabit, salt okunur) ve **`String`** (Koleksiyon - dinamik, değiştirilebilir, Heap'te tutulur). `String` aslında UTF-8 kodlu byte'ların tutulduğu bir `Vec<u8>` sarmalayıcısıdır (wrapper).

### 2.1. String Oluşturma
``` 
fn main() {
    // 1. Boş String
    let mut s1 = String::new();

    // 2. Literal'dan dönüştürme
    let s2 = "Merhaba".to_string();
    let s3 = String::from("Dünya");
    
    println!("{} {}", s2, s3);
}
```

### 2.2. String Güncelleme (Birleştirme)
``` 
fn main() {
    let mut s1 = String::from("Merhaba");
    let s2 = String::from(" Dünya");
    let s3 = "!";

    // 1. `push_str` ve `push` (Sahipliği almaz, referans kullanır)
    s1.push_str(&s2); 
    s1.push('!');

    // 2. `+` operatörü (Sol tarafın sahipliğini alır - MOVE eder!)
    let s4 = s1 + &s3; // s1 artık kullanılamaz!
    
    // 3. `format!` makrosu (Sahipliği ALMAZ, en güvenli yöntem)
    let s5 = format!("{}-{}-{}", s2, s3, " "); 
    
    println!("{}", s4); // Merhaba Dünya!
    println!("{}", s5); //  Dünya-!- 
}
```

### 2.3. String İndeksleme (Çok Önemli Kural!)
** 'ta bir `String`'e köşeli parantez ile (`s[0]`) erişemezsiniz.** 
*Neden?* Çünkü `String` UTF-8'dir. Bir karakter 1, 2, 3 veya 4 byte olabilir. `s[0]` dediğinizde ilk *byte*'ı mı yoksa ilk *karakteri* mi istiyorsunuz?   bu belirsizliği sevmez ve derleme hatası verir.

``` 
fn main() {
    let s = String::from("Merhaba");

    // ❌ let ilk_harf = s[0]; // HATA!
    
    // ✅ String Slice (Dilimleme) Kullanın:
    let dilim = &s[0..4]; // İlk 4 byte'ı al ("Merh")
    println!("{}", dilim);

    // ✅ Karakterler üzerinde gezinmek için `.chars()` kullanın:
    for harf in s.chars() {
        print!("{} ", harf);
    }
    println!();

    // ✅ Byte'lar üzerinde gezinmek için `.bytes()` kullanın:
    for byte in s.bytes() {
        print!("{} ", byte);
    }
}
```

---

## 3. Hash Haritalar (`HashMap<K, V>`)

Anahtar (Key) ve Değer (Value) çiftlerini saklayan koleksiyondur. Python'daki `dict`, JavaScript'teki `Object/Map` yapısına benzer. Veriler rastgele sırayla tutulur.

### 3.1. HashMap Oluşturma ve Ekleme
``` 
use std::collections::HashMap;

fn main() {
    let mut puanlar = HashMap::new();

    // Ekleme (insert)
    puanlar.insert(String::from("Ali"), 90);
    puanlar.insert(String::from("Ayşe"), 85);
    puanlar.insert(String::from("Mehmet"), 70);

    println!("Puanlar: {:?}", puanlar);
}
```

### 3.2. Okuma ve Güncelleme
``` 
use std::collections::HashMap;

fn main() {
    let mut puanlar = HashMap::new();
    puanlar.insert(String::from("Ali"), 90);

    // 1. Okuma (`get` metodu Option<&V> döndürür)
    let ali_puan = puanlar.get(&String::from("Ali"));
    match ali_puan {
        Some(p) => println!("Ali'nin puanı: {}", p),
        None => println!("Ali bulunamadı"),
    }

    // 2. Güncelleme (Aynı anahtara tekrar insert yaparsanız üzerine yazar)
    puanlar.insert(String::from("Ali"), 95);

    // 3. Sadece anahtar yoksa ekleme (Entry API - Çok güçlüdür!)
    puanlar.entry(String::from("Veli")).or_insert(60); // Veli yoksa 60 ekle
    puanlar.entry(String::from("Ali")).or_insert(100); // Ali var, dokunma (95 kalır)
    
    println!("{:?}", puanlar);
}
```

### 3.3. HashMap Üzerinde Döngü
``` 
use std::collections::HashMap;

fn main() {
    let mut harita = HashMap::new();
    harita.insert("Mavi", 10);
    harita.insert("Sarı", 50);
    harita.insert("Kırmızı", 100);

    // Anahtar ve Değerleri Gezme
    for (anahtar, deger) in &harita {
        println!("{}: {}", anahtar, deger);
    }
}
```

---

## 4. Diğer Koleksiyonlar (`std::collections`)

Standart kütüphanenin `collections` modülünde, özel durumlar için kullanılan ek koleksiyonlar bulunur:

| Koleksiyon | Açıklama | Ne Zaman Kullanılır? |
| :--- | :--- | :--- |
| **`VecDeque<T>`** | Çift uçlu kuyruk (Double-ended queue). | Hem baştan hem sondan hızlı ekleme/çıkarma (`push_front`, `pop_front`) gerekiyorsa (Örn: Kuyruk yapısı, BFS algoritması). |
| **`LinkedList<T>`** | Çift yönlü bağlı liste. | Çok nadir kullanılır. `Vec` veya `VecDeque` neredeyse her zaman daha hızlıdır. |
| **`HashSet<T>`** | Sadece anahtar tutan Hash Map. | Bir listedeki tekrar eden elemanları filtrelemek veya "bu eleman listede var mı?" sorusunu O(1)'de sormak için. |
| **`BTreeMap<K, V>`** | Sıralı Hash Map. | Anahtarların **alfabetik/sayısal sırayla** tutulması gerekiyorsa veya aralık sorguları (range queries) yapılacaksa kullanılır. |
| **`BTreeSet<T>`** | Sıralı Hash Set. | Sıralı ve tekrarsız veri tutmak için. |

---

## 5. Koleksiyonlarda Sahiplik (Ownership) Kuralı

Koleksiyonlar, içlerine koyduğunuz verilerin **sahibi (owner)** olurlar. Bir değeri koleksiyona eklediğinizde, o değerin sahipliği koleksiyona geçer (Move edilir).

``` 
fn main() {
    let s1 = String::from("Metin 1");
    let s2 = String::from("Metin 2");
    let s3 = String::from("Metin 3");

    let mut v = Vec::new();
    v.push(s1); // s1'in sahipliği v'ye geçti.
    v.push(s2); // s2'nin sahipliği v'ye geçti.
    // v.push(s3.clone()); // Eğer s3'ü sonra kullanacaksanız klonlamalısınız.
    v.push(s3); 

    // println!("{}", s1); // ❌ HATA! s1 artık v'nin içinde, sahipliği bizde değil.
    
    println!("Vektör: {:?}", v);
}
// `v` kapsam (scope) dışına çıktığında, içindeki String'ler de otomatik olarak silinir (Drop).
```

---

## 6. Özet ve En İyi Pratikler

1. **Hangi Koleksiyonu Seçmeliyim?**
   * Sıralı, aynı tipte, sona ekleme/okuma yapacaksanız ➡️ **`Vec<T>`**
   * Metin üzerinde değişiklik yapacaksanız ️ **`String`**
   * Anahtar-Değer eşleştirmesi yapacaksanız ️ **`HashMap<K, V>`**
   * Hem baştan hem sondan ekleme/çıkarma yapacaksanız ➡️ **`VecDeque<T>`**
   * Sıralı anahtarlar gerekiyorsa ️ **`BTreeMap<K, V>`**

2. **Performans İpucu:** Vektörün (`Vec`) boyutunu önceden biliyorsanız, `Vec::with_capacity(100)` kullanarak bellek tahsisini (allocation) baştan yapın. Bu, vektör büyürken gerçekleşen yeniden tahsis (reallocation) maliyetini ortadan kaldırır.

3. **String İndeksleme:** Asla `s[0]` yapmaya çalışmayın. Bunun yerine `.chars()` kullanın veya dilimleme (`&s[0..4]`) yapın.

4. **Entry API:** `HashMap` kullanırken "varsa güncelle, yoksa ekle" mantığını `if/else` yerine her zaman `.entry().or_insert()` veya `.or_insert_with()` ile kurun. Kodunuz hem daha kısa hem de daha hızlı (tek hash hesaplama) olur.