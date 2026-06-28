 'ta **Tuple** (Türkçesiyle *Demet*), farklı veri tiplerindeki değerleri tek bir değişken altında toplayan, **sabit uzunluğa** sahip bir veri yapısıdır. 

Tuple'lar, birden fazla farklı tipte veriyi bir arada tutmak ve özellikle **fonksiyonlardan birden fazla değer döndürmek** için  'ta çok sık kullanılır.

İşte   Tuple'larının detaylı anlatımı:

---

### 1. Tuple Oluşturma ve Tanımlama
Bir tuple oluşturmak için değerleri parantez `()` içine alıp virgülle ayırmanız yeterlidir.  , tipleri otomatik olarak algılayabilir (type inference) ancak isterseniz açıkça da belirtebilirsiniz.

``` 
fn main() {
    // Tip belirtilmeden (  otomatik algılar)
    let tup1 = (500, 6.4, true, "Merhaba");

    // Tipler açıkça belirtilerek
    // (i32, f64, bool, &str) formatındadır.
    let tup2: (i32, f64, u8) = (500, 6.4, 1); 
}
```

---

### 2. Tuple İçindeki Verilere Erişim
Tuple içindeki verilere erişmenin **iki temel yolu** vardır:

#### A. Nokta (Dot) Notasyonu ile İndeksleme
Array'lerin (dizilerin) aksine, tuple'larda köşeli parantez `[]` değil, **nokta `.`** kullanılır. İndeksler `0`'dan başlar.

``` 
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let x = tup.0; // 500 (i32)
    let y = tup.1; // 6.4 (f64)
    let z = tup.2; // 1   (u8)

    println!("x: {}, y: {}, z: {}", x, y, z);
}
```
*⚠️ Önemli Not: `tup[0]` yazarsanız   derleme hatası (compile error) verir.*

#### B. Yapı Bozma (Destructuring)
Tuple'ı tek seferde parçalarına ayırarak yeni değişkenlere atamak için "pattern matching" (desen eşleştirme) yapısını kullanabilirsiniz. Buna **Destructuring** denir.

``` 
fn main() {
    let tup = (500, 6.4, 1);

    // Tuple'ı parçalara ayırıyoruz
    let (x, y, z) = tup;

    println!("x değeri: {}", x); // 500
    println!("y değeri: {}", y); // 6.4
    println!("z değeri: {}", z); // 1
}
```

Eğer tuple'ın sadece belirli bir elemanına ihtiyacınız varsa, diğerlerini `_` (underscore) ile yok sayabilirsiniz:
``` 
let (x, _, z) = (10, 20, 30); // y (20) kullanılmadı.
```

---

### 3. Boş Tuple ve Unit Tipi `()`
Hiçbir değer içermeyen, uzunluğu sıfır olan tuple'a **Unit Tuple** denir ve `()` şeklinde gösterilir. 
 'ta bir fonksiyon açıkça bir değer döndürmüyorsa, arka planda `()` (Unit tipi) döndürür. Bu, diğer dillerdeki `void` kavramının  'taki karşılığıdır.

``` 
fn bir_sey_yap() -> () {
    println!("Bu fonksiyon bir şey döndürmez.");
}

// Aslında yukarıdaki fonksiyon şununla tamamen aynıdır:
fn bir_sey_yap2() {
    println!("Tip belirtilmediğinde   otomatik olarak () döndürür.");
}

fn main() {
    let unit_deger: () = ();
}
```

---

### 4. Neden Tuple Kullanırız? (Kullanım Alanları)

#### A. Fonksiyonlardan Birden Fazla Değer Döndürmek
 'ta (Python'daki gibi) bir fonksiyondan birden fazla değer döndürmenin en zarif yolu tuple kullanmaktır.

``` 
fn kullanici_bilgisi_al() -> (String, i32) {
    let isim = String::from("Ahmet");
    let yas = 28;
    (isim, yas) // Tuple döndürüyoruz
}

fn main() {
    let (isim, yas) = kullanici_bilgisi_al();
    println!("İsim: {}, Yaş: {}", isim, yas);
}
```

#### B. Pattern Matching (Desen Eşleştirme)
`match` ifadelerinde birden fazla koşulu aynı anda kontrol etmek için tuple'lar harikadır.

``` 
fn main() {
    let koordinat = (0, 5);

    match koordinat {
        (0, 0) => println!("Başlangıç noktası"),
        (x, 0) => println!("X ekseni üzerinde, x: {}", x),
        (0, y) => println!("Y ekseni üzerinde, y: {}", y),
        (x, y) => println!("Kordinat: ({}, {})", x, y),
    }
}
```

---

### 5. Tuple Struct (Tuple Yapıları)
 'ta isimleri olmayan ama bir isme sahip olan tuple yapıları tanımlayabilirsiniz. Bu, struct (yapı) ile tuple'ın birleşimidir. Genellikle "Newtype" pattern'i oluşturmak için kullanılır.

``` 
// Renk isminde bir Tuple Struct tanımladık
struct Renk(i32, i32, i32);
struct Nokta(i32, i32, i32);

fn main() {
    let siyah = Renk(0, 0, 0);
    let baslangic = Nokta(0, 0, 0);

    // Renk ve Nokta aynı tuple yapısına sahip olsa da 
    // farklı tiplerdir. Birbirlerinin yerine kullanılamazlar.
    // let test: Renk = baslangic; // HATA!
    
    println!("Siyah RGB: {}, {}, {}", siyah.0, siyah.1, siyah.2);
}
```

---

### 6. Tuple vs Array (Dizi) Karşılaştırması

| Özellik | Tuple | Array (Dizi) |
| :--- | :--- | :--- |
| **Veri Tipleri** | **Heterojen** (Farklı tipler olabilir: `i32`, `f64`, `bool`) | **Homojen** (Sadece aynı tip: `[i32; 3]`) |
| **Uzunluk** | Sabit (Compile-time'da bellidir) | Sabit (Compile-time'da bellidir) |
| **Erişim** | Nokta ile: `tup.0`, `tup.1` | Köşeli parantez ile: `arr[0]`, `arr[1]` |
| **Iterasyon (Döngü)** | **Yapılamaz** (Tipler farklı olduğu için `for` döngüsüne sokulamaz) | **Yapılabilir** (`for x in arr` kullanılabilir) |
| **Kullanım Amacı** | Farklı tiplerdeki ilişkili verileri gruplamak | Aynı tipteki verilerin listesi |

---

### Özet ve İpuçları
* Tuple'lar **stack** üzerinde tutulur ve çok hızlıdırlar (çünkü boyutları bellidir).
* Eğer bir tuple içindeki eleman sayısı çok artıyorsa (örneğin 7-8'den fazla), okunabilirlik açısından **`struct`** kullanmaya geçmeniz tavsiye edilir.
* Tuple içindeki verilere erişirken **nokta (.)** notasyonunu kullanmayı unutmayın (`[]` array'ler içindir).
* Fonksiyonlardan 2 veya 3 farklı tipte değer döndürmeniz gerektiğinde ilk aklınıza gelen yapı **Tuple** olmalıdır.