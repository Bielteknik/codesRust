# 🦀   Ders Notları: Bölüm 3 - Ortak Programlama Kavramları

Bu bölümde değişkenlerin nasıl çalıştığını, temel veri tiplerini, fonksiyonları, yorum satırlarını ve kodun akışını nasıl kontrol edeceğimizi öğreneceğiz.  'ın bu konulara yaklaşımı, dilin **güvenlik (safety)** ve **hız (performance)** odaklı felsefesini yansıtır.

---

## 📌 3.1 Variables and Mutability (Değişkenler ve Değişkenlik)

 'ta değişkenlerle ilgili en önemli ve ilk öğrenilmesi gereken kural şudur: **Değişkenler varsayılan olarak değişmezdir (immutable).**

### 1. Varsayılan Değişmezlik (Immutability)
Bir değişken tanımladığınızda, onun değerini sonradan değiştiremezsiniz. Bu,  'ın size sunduğu bir güvenlik önlemidir. Büyük kod tabanlarında veya çoklu iş parçacığı (concurrency) kullanılan durumlarda, bir değerin beklenmedik bir şekilde değiştirilmesi zor bulunan hatalara (bug) yol açar.

``` 
fn main() {
    let x = 5;
    println!("x'in değeri: {}", x);
    x = 6; // HATA! (cannot assign twice to immutable variable)
    println!("x'in değeri: {}", x);
}
```

### 2. Değişkenlik Eklemek: `mut`
Eğer bir değişkenin değerini gerçekten değiştirmeniz gerekiyorsa, `let` kelimesinden sonra **`mut`** anahtar kelimesini kullanırsınız. Bu, kodu okuyan diğer geliştiricilere "bu değerin ileride değişeceğini" açıkça beyan eder.

``` 
fn main() {
    let mut x = 5;
    println!("x'in değeri: {}", x);
    x = 6; // GEÇERLİ!
    println!("x'in yeni değeri: {}", x);
}
```

### 3. Sabitler (Constants)
Değişmez değişkenler (immutable variables) ile sabitler (`const`) aynı şey değildir.
*   **Sabitler** *her zaman* değişmezdir ve `mut` kullanılamaz.
*   `let` yerine `const` anahtar kelimesi kullanılır.
*   Veri tipini belirtmek **zorunludur**.
*   Sadece derleme zamanında (compile-time) hesaplanabilen ifadelerle tanımlanabilirler (fonksiyon çağrısı sonucu vb. sabit olarak atanamaz).
*     geleneklerine göre sabit isimleri BÜYÜK HARF ve alt çizgi ile yazılır: `MAX_POINTS`.

``` 
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

### 4. Gölgeleme (Shadowing)
 'ta aynı isimde yeni bir değişken tanımlayarak eski değişkeni "gölgeleyebilirsiniz". İlk `let` bittiğinde, yeni `let` onun yerini alır.

**`mut` kullanmaktan farkı nedir?**
*   `mut` ile sadece değeri değiştirirsiniz, veri tipi aynı kalmak zorundadır.
*   **Shadowing (Gölgeleme)** ile değişkenin **veri tipini** değiştirebilirsiniz, çünkü aslında *yeni* bir değişken yaratıyorsunuzdur.

``` 
let spaces = "   ";         // Veri tipi: &str (Metin)
let spaces = spaces.len();  // Veri tipi: usize (Sayı). Bu geçerlidir!
```

---

## 📌 3.2 Data Types (Veri Tipleri)

  **statik tipli (statically typed)** bir dildir. Bu, derleyicinin tüm değişkenlerin tiplerini derleme zamanında bilmek istediği anlamına gelir.  'ta iki ana veri tipi grubu vardır: **Skaler** ve **Bileşik**.

### 1. Skaler Tipler (Scalar Types)
Tek bir değeri temsil ederler.

*   **Integer (Tamsayılar):** Kesirli olmayan sayılardır. İşaretli (`i` ile başlar, negatif alabilir) ve işaretsiz (`u` ile başlar, sadece pozitif) olmak üzere çeşitleri vardır.
    *   *Örn:* `i8`, `i16`, `i32` (varsayılan), `i64`, `i128`.
    *   *Örn:* `u8`, `u16`, `u32`, `u64`, `u128`.
    *   *Mimariye bağlı:* `isize` ve `usize` (Dizilerin indekslenmesinde kullanılır).
*   **Floating-Point (Ondalıklı Sayılar):** `f32` ve `f64`. Varsayılan olarak `f64` kullanılır çünkü modern CPU'larda hızları eşittir ama `f64` daha hassastır.
*   **Boolean (`bool`):** Sadece `true` veya `false` değerlerini alır. Boyutu 1 byte'tır.
*   **Character (`char`):** Tek tırnak (`' '`) ile tanımlanır.  'taki `char` 4 byte'tır ve **Unicode**'u destekler. Bu yüzden emojiler, Çince karakterler vb. birer `char` olabilir.
    *   *Örn:* `let c = 'z'; let kalp = '❤️';`

### 2. Bileşik Tipler (Compound Types)
Birden fazla değeri tek bir tipte gruplar.

*   **Tuple (Demet):** Farklı veri tiplerini bir araya getirebilir. Sabit uzunluktadır.
    ``` 
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // Erişim 1: Desen Eşleme (Destructuring)
    let (x, y, z) = tup;
    // Erişim 2: Nokta ve indeks
    let ilk_eleman = tup.0; 
    ```
*   **Array (Dizi):** Tuple'ın aksine, dizideki **tüm elemanlar aynı veri tipinde** olmalıdır. Diziler bellekte yığın (stack) üzerinde tutulur.
    ``` 
    let a = [1, 2, 3, 4, 5];
    let aylar = ["Ocak", "Şubat", "Mart"];
    // Aynı değeri tekrar tekrar yazmak yerine:
    let sifirler = [0; 10]; // 10 tane 0'dan oluşan dizi
    ```
    > ⚠️ **Önemli (Sınır Denetimi - Bounds Checking):**  'ta bir dizinin olmayan bir indeksine (örn: `a[100]`) erişmeye çalışırsanız, derleyici kodu derler veya çalışırken programı anında **çökertir (panic!)**. C/C++ gibi dillerde bu "geçersiz bellek okumaya" (buffer overflow) izin verirken,   güvenliği elden bırakmaz.

---

## 📌 3.3 Functions (Fonksiyonlar)

  kodunda her yerdedirler. `main` fonksiyonu, bir programın başlangıç noktasıdır.
Anahtar kelime **`fn`**'dir. İsimlendirmede *snake_case* (küçük harf ve alt çizgi) tercih edilir.

``` 
fn main() {
    println!("Merhaba, dünya!");
    baska_fonksiyon(5, 'R');
}

fn baska_fonksiyon(x: i32, y: char) { // Parametrelerin TİPİ mutlaka yazılmalıdır.
    println!("x'in değeri: {}", x);
}
```

### Statements (Deyimler) vs. Expressions (İfadeler)
 , *ifadeler (expressions)* temelli bir dildir. Bu ayrımı anlamak çok önemlidir:
*   **Statements (Deyimler):** Bir eylem gerçekleştirir ancak bir **değer döndürmezler**. Sonlarında noktalı virgül (`;`) bulunur.
    *   *Örn:* `let y = 6;` (Bir deyimdir. `let y = 6`'nın kendisi bir değer üretmez, sadece atama yapar).
*   **Expressions (İfadeler):** Bir sonuç hesaplar ve bir **değer döndürürler**.
    *   *Örn:* `5 + 6` (Bir ifadedir ve `11` değerini üretir).
    *   Süslü parantez blokları `{}` da birer ifadedir.

### Değer Döndüren Fonksiyonlar
Fonksiyonların ne döndüreceği `->` oku ile belirtilir.  'ta `return` kelimesini kullanmanıza gerek yoktur; fonksiyonun **en sonundaki ifade** (sonunda noktalı virgül yoksa) otomatik olarak dönüş değeri olur.

``` 
fn bes() -> i32 {
    5 // Noktalı virgül YOK! Bu bir ifadedir ve fonksiyon 5 döndürür.
}

fn main() {
    let x = bes(); // x'in değeri 5 olur.
}
```

---

## 📌 3.4 Comments (Yorum Satırları)

Kodunuzu açıklamak için kullanılır. Derleyici bunları görmezden gelir.
*   Çift eğik çizgi `//` ile başlar ve satırın sonuna kadar devam eder.

``` 
// Bu bir yorum satırıdır.
let x = 5; // Bu da satır sonu yorumudur.
```

---

## 📌 3.5 Control Flow (Kontrol Akışı)

Koşullara ve döngülere göre kodun hangi sırayla çalışacağını belirler.

### 1. `if` İfadeleri
Bir koşula göre kod bloklarını çalıştırmanızı sağlar.
> 🚨 **Çok Kritik Kural:**  'ta `if` koşulu **MUTLAKA `bool` (true/false) tipinde** olmalıdır. C, JavaScript veya Python gibi dillerdeki gibi `0`'ı, `null`'ı veya boş string'i "false" olarak değerlendirmez. Bunu yaparsanız derleyici **hata verir**.

``` 
fn main() {
    let number = 3;

    if number != 0 {
        println!("Sıfırdan farklı bir sayı");
    } else {
        println!("Sayı sıfırdı");
    }
}
```

#### `let` İçinde `if` Kullanımı
`if` bir *ifade (expression)* olduğu için, bir değişkene değer atamak üzere kullanılabilir. Ancak `if` ve `else` bloklarının döndürdüğü veri tipleri **birebir aynı olmak zorundadır**.

``` 
let condition = true;
// Hem 5 hem de 6 i32 (tamsayı) olduğu için bu kod çalışır.
let number = if condition { 5 } else { 6 }; 
```

### 2. Döngüler (Loops)

 'ta 3 temel döngü türü vardır: `loop`, `while` ve `for`.

#### A. `loop` (Sonsuz Döngü)
Siz ona "dur" diyene kadar (`break`) aynı kodu tekrar tekrar çalıştırır.
 'ın harika bir özelliği, `loop` döngüsünü kırarken (`break`) bir **değer döndürebilmenizdir**:

``` 
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // Döngüyü kır ve 20 değerini result'a ata.
        }
    };

    println!("Sonuç: {}", result); // 20 yazar.
}
```

#### B. `while` Döngüsü
Koşul `true` olduğu sürece çalışır.
``` 
let mut number = 3;
while number != 0 {
    println!("{}!", number);
    number -= 1;
}
```

#### C. `for` Döngüsü (En Güvenli Olan)
Bir koleksiyonun (örneğin bir dizinin) elemanları üzerinde döngü kurmanın en yaygın ve **en güvenli** yoludur. Manuel indeks takibi yapmadığınız için "sınır dışı (out of bounds)" hatalarını ortadan kaldırır.

``` 
let a = [10, 20, 30, 40, 50];

// .iter() dizinin elemanlarını tek tek döndürür.
for element in a.iter() {
    println!("Değer: {}", element);
}

// Belirli bir aralık (Range) üzerinde dönmek için:
for number in (1..4).rev() { // 1'den 4'e kadar (4 hariç) ve tersten (rev)
    println!("{}!", number);
}
```

---

### 🎓 Özet / Akılda Kalıcılar (Cheat Sheet)
1.  **Güvenlik önce gelir:** Değişkenler `mut` yazmadıkça değiştirilemez.
2.  **Tipler önemlidir:** Derleyici her şeyin tipini bilmek ister, belirsizlik yoktur.
3.  **Statements vs Expressions:** `let y = 6;` bir deyimdir (değer döndürmez), `{ 6 }` bir ifadedir (değer döndürür).
4.  **Doğruluk (Truthiness) yoktur:** `if` bloklarında sadece `true` veya `false` kullanılır.
5.  **Koleksiyonlar için `for`:** Dizi/vektör döngülerinde hata riskini sıfıra indirmek için her zaman `for` tercih edilmelidir.