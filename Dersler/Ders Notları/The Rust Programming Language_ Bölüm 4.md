# 🦀   Ders Notları: Bölüm 4 -   Ownership (Mülkiyet) Sistemi - Kapsamlı Ders

Merhaba! Bu derste   programlama dilinin en önemli ve en ayırt edici özelliği olan **Ownership (Mülkiyet)** sistemini öğreneceğiz. Bu konu  'ın bel kemiğidir ve dilin geri kalanını anlamak için sağlam bir temel oluşturur.

---

## 📚 Bölüm 1: Ownership Nedir? (What is Ownership?)

### 🎯 Neden Ownership'a İhtiyacımız Var?

Tüm programlar çalışırken bilgisayarın belleğini yönetmek zorundadır. Farklı diller bunu farklı şekillerde yapar:

1. **Çöp Toplayıcılı Diller (Garbage Collection - GC)**: Java, Python, JavaScript gibi dillerde arka planda bir çöp toplayıcı çalışır ve kullanılmayan belleği otomatik temizler.
2. **Manuel Yönetim**: C ve C++ gibi dillerde programcı belleği kendisi ayırır (allocate) ve kendisi serbest bırakır (free).
3. ** 'ın Yaklaşımı**:   üçüncü bir yol izler - **Ownership sistemi** ile derleyici (compiler) bellek yönetimini kontrol eder. Çalışma zamanında (runtime) hiçbir ek maliyet yoktur!

### 📦 Stack ve Heap: Belleğin İki Yüzü

Ownership'ı anlamak için önce **Stack** ve **Heap** kavramlarını anlamamız gerekiyor:

#### Stack (Yığın)
- **LIFO** (Last In, First Out - Son Giren İlk Çıkar) prensibiyle çalışır
- Bir tabak yığını düşünün: Yeni tabak eklediğinizde üste koyarsınız, ihtiyacınız olduğunda üsttekini alırsınız
- **Bilinen, sabit boyutlu** veriler burada saklanır
- Çok hızlıdır çünkü allocator (bellek ayırıcı) yeni veri için yer aramak zorunda değildir
- Veri her zaman üstte eklenir ve üstten çıkarılır

#### Heap (Öbek)
- Daha düzensiz bir yapıdır
- Bellek ayırıcı, yeterli büyüklükte boş bir alan bulur, onu kullanımda olarak işaretler ve o konumun adresini gösteren bir **pointer (işaretçi)** döndürür
- Boyutu derleme zamanında bilinmeyen veya değişebilen veriler burada saklanır
- Stack'e göre daha yavaştır çünkü allocator önce yeterli alan bulmalı, sonra bookkeeping yapmalıdır
- Veriye erişmek için pointer'ı takip etmek gerekir

**Benzetme**: Bir restorana gittiğinizi düşünün. Garson size bir masa bulur ve oraya oturursunuz. Geç gelen bir arkadaşınız nerede olduğunuzu sorduğunda masa numaranızı (pointer) söylersiniz ve sizi bulur.

### 📏 Ownership Kuralları

 'ta ownership sistemi üç temel kurala dayanır:

1. ** 'taki her değerin bir sahibi (owner) vardır.**
2. **Aynı anda sadece bir sahip olabilir.**
3. **Sahip scope dışına çıktığında, değer düş edilir (dropped).**

### 🔍 Scope (Kapsam) Kavramı

Scope, bir öğenin geçerli olduğu aralıktır:

``` 
fn main() {
    {                      // s henüz geçerli değil, çünkü henüz tanımlanmadı
        let s = "hello";   // s bu noktadan itibaren geçerli
        
        // s ile işlemler yap
    }                      // scope bitti, s artık geçerli değil
}
```

İki önemli zaman noktası:
- `s` **scope'a girdiğinde** geçerli olur
- **Scope'tan çıkana kadar** geçerli kalır

### 📝 String Tipi ve Ownership

Basit tipler (integer, boolean vb.) stack'te saklanır ve scope sonlandığında otomatik temizlenir. Ancak **String** tipi heap'te saklanır ve ownership kurallarını anlamak için mükemmel bir örnektir:

``` 
let s = String::from("hello");
```

Bu string literal'dan farklıdır çünkü:
- **Değiştirilebilir (mutable)** olabilir
- İçeriği derleme zamanında bilinmeyebilir (örn: kullanıcı girdisi)
- Heap'te bellek ayırır

``` 
let mut s = String::from("hello");
s.push_str(", world!"); // String'e metin ekler
println!("{s}"); // "hello, world!" yazdırır
```

### 🗑️ Drop Fonksiyonu

 , bir değişken scope dışına çıktığında otomatik olarak **`drop`** fonksiyonunu çağırır ve belleği geri verir:

``` 
fn main() {
    {
        let s = String::from("hello"); // s geçerli
        // s ile işlemler
    }  // scope bitti,   otomatik olarak drop çağırır ve belleği temizler
}
```

Bu pattern C++'daki **RAII (Resource Acquisition Is Initialization)** ile benzerdir.

### 🔄 Değişkenler ve Veri Etkileşimi

#### Integer Ataması

``` 
let x = 5;
let y = x;
```

Burada `x`'in değeri kopyalanır ve `y`'ye atanır. Stack'te iki ayrı `5` değeri vardır.

#### String Ataması - Move Kavramı

``` 
let s1 = String::from("hello");
let s2 = s1;
```

Bu ilk bakışta aynı gibi görünse de **çok farklı** bir şey olur!

**String'in bellek yapısı:**
- **Stack'te**: Pointer (heap'teki veriye işaretçi), length (uzunluk), capacity (kapasite)
- **Heap'te**: Gerçek string içeriği

`s1 = s2` ataması yapıldığında:
- Stack'teki pointer, length ve capacity **kopyalanır**
- Heap'teki veri **kopyalanmaz**

Bu durumda iki pointer aynı heap verisini gösteriyor olurdu. Her iki değişken de scope'tan çıktığında aynı belleği iki kez serbest bırakmaya çalışırlardı - bu **double free** hatasıdır ve ciddi bir güvenlik açığıdır!

**  bunu önlemek için**: `let s2 = s1;` satırından sonra **`s1` artık geçerli değildir**. Buna **Move** denir.

``` 
let s1 = String::from("hello");
let s2 = s1;

println!("{s1}, world!"); // ❌ HATA! s1 artık geçerli değil
```

**Hata mesajı:**
```
error[E0382]: borrow of moved value: `s1`
```

#### Clone - Derin Kopyalama

Eğer heap verisini de gerçekten kopyalamak istiyorsak **`clone()`** kullanırız:

``` 
let s1 = String::from("hello");
let s2 = s1.clone(); // Gerçekten derin kopyalama yapar

println!("s1 = {s1}, s2 = {s2}"); // ✅ İkisi de geçerli
```

`clone()` gördüğünüzde, pahalı bir işlem yapıldığını bilin!

#### Copy Trait - Stack Tipleri

Integer gibi basit tipler için move olmaz, kopyalama olur:

``` 
let x = 5;
let y = x;

println!("x = {x}, y = {y}"); // ✅ İkisi de geçerli
```

**Copy trait**'i uygulayan tipler:
- Tüm integer tipleri (`u32`, `i64`, vb.)
- `bool` (`true` ve `false`)
- Tüm floating-point tipleri (`f64`, `f32`)
- `char`
- Sadece Copy trait'li tipler içeren tuple'lar: `(i32, i32)` ✅ ama `(i32, String)` ❌

### 📤 Fonksiyonlar ve Ownership

Fonksiyona değer geçirmek, değişkene atama yapmakla aynıdır:

``` 
fn main() {
    let s = String::from("hello");  // s scope'a girer
    
    takes_ownership(s);             // s'in değeri fonksiyona taşınır (move)
                                    // Bu noktadan sonra s geçerli değil
    
    let x = 5;                      // x scope'a girer
    
    makes_copy(x);                  // i32 Copy trait'li olduğu için kopyalanır
                                    // x hala geçerli
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
} // some_string scope'tan çıkar, drop çağrılır, bellek serbest bırakılır

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
} // some_integer scope'tan çıkar, özel bir şey olmaz
```

### 📥 Return Değerleri ve Ownership

Fonksiyonlar ownership'i geri de verebilir:

``` 
fn main() {
    let s1 = gives_ownership();        // s1'e ownership taşınır
    
    let s2 = String::from("hello");    // s2 scope'a girer
    
    let s3 = takes_and_gives_back(s2); // s2, fonksiyona taşınır
                                       // ve return değeri s3'e taşınır
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // Return edilir ve ownership çağrıya taşınır
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // Return edilir ve ownership çağrıya taşınır
}
```

### 😓 Sorun: Ownership'i Geri Vermek

Her seferinde ownership'i alıp geri vermek çok zahmetli! Tuple ile birden fazla değer döndürebiliriz:

``` 
fn main() {
    let s1 = String::from("hello");
    
    let (s2, len) = calculate_length(s1);
    
    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length) // Hem String'i hem de uzunluğu döndürür
}
```

Ama bu çok fazla kod tekrarı! **Çözüm: References (Referanslar)** 🎉

---

## 📚 Bölüm 2: References ve Borrowing (Referanslar ve Ödünç Alma)

### 🔗 Referans Nedir?

Referans, bir değere işaret eden bir pointer'dır ama o değerin sahibi değildir. Referansın işaret ettiği veri başka bir değişken tarafından sahiplenilir.

**Referans vs Pointer**: Referans, pointer'ın aksine, yaşam süresi boyunca geçerli bir değere işaret etmesi garanti edilir.

### 📖 Borrowing (Ödünç Alma)

Referans oluşturmak **borrowing** olarak adlandırılır. Gerçek hayatta olduğu gibi: birisi bir şeye sahipse, ondan ödünç alabilirsiniz. İşiniz bittiğinde geri verirsiniz, çünkü sizin değil.

``` 
fn main() {
    let s1 = String::from("hello");
    
    let len = calculate_length(&s1); // &s1 referans oluşturur
    
    println!("The length of '{s1}' is {len}."); // s1 hala geçerli!
}

fn calculate_length(s: &String) -> usize { // &String: String'e referans
    s.len()
} // s scope'tan çıkar ama işaret ettiği String drop edilmez
  // çünkü s onun sahibi değil
```

**Açıklama:**
- `&s1`: `s1`'in değerine referans oluşturur ama sahibi olmaz
- `s: &String`: Parametre bir String referansı
- `s` scope'tan çıktığında String drop edilmez çünkü `s` onun sahibi değil

### ✏️ Mutable References (Değiştirilebilir Referanslar)

Referanslar varsayılan olarak **immutable**'dır (değiştirilemez). Değiştirmek için **mutable reference** gerekir:

``` 
fn main() {
    let mut s = String::from("hello"); // s mut olmalı
    
    change(&mut s); // &mut ile mutable referans oluştur
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

### ⚠️ Mutable Reference Kısıtlaması

**Çok önemli kural**: Bir değere mutable reference'ınız varsa, o değere başka hiçbir reference'ınız olamaz.

``` 
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s; // ❌ HATA! Aynı anda iki mutable reference olamaz

println!("{r1}, {r2}");
```

**Hata:**
```
error[E0499]: cannot borrow `s` as mutable more than once at a time
```

**Neden bu kural var?** Bu kural **data race**'leri (veri yarışlarını) derleme zamanında önler!

**Data race** şu üç durum bir araya geldiğinde oluşur:
1. İki veya daha fazla pointer aynı anda aynı veriye erişir
2. En az bir pointer veriye yazıyor
3. Veriye erişimi senkronize edecek bir mekanizma yok

  bu kodu derlemeyi reddederek data race'i önler!

#### Scope ile Çözüm

``` 
let mut s = String::from("hello");

{
    let r1 = &mut s;
} // r1 scope'tan çıkar, artık yeni mutable reference oluşturabiliriz

let r2 = &mut s; // ✅ Sorun yok
```

### 🔄 Mutable ve Immutable Referansların Karışımı

Aynı anda hem mutable hem immutable reference **OLAMAZ**:

``` 
let mut s = String::from("hello");

let r1 = &s;  // ✅ immutable reference
let r2 = &s;  // ✅ immutable reference
let r3 = &mut s; // ❌ BÜYÜK SORUN!

println!("{r1}, {r2}, and {r3}");
```

**Neden?** Immutable reference kullananlar, değerin aniden değişmesini beklemezler!

Ama birden fazla immutable reference olabilir çünkü kimse veriyi değiştirmiyor.

#### Reference Scope Anlayışı

Reference'ın scope'u, oluşturulduğu yerden başlar ve **son kullanıldığı yere** kadar devam eder:

``` 
let mut s = String::from("hello");

let r1 = &s; // sorun yok
let r2 = &s; // sorun yok
println!("{r1} and {r2}");
// r1 ve r2 bu noktadan sonra kullanılmayacak

let r3 = &mut s; // sorun yok! r1 ve r2 artık kullanılmıyor
println!("{r3}");
```

### 🚫 Dangling References (Sarkık Referanslar)

Pointer'lı dillerde **dangling pointer** oluşturmak kolaydır - belleği serbest bırakıp pointer'ı korumaya devam edersiniz.

**  derleyici bunu garanti eder**: Bir veriye reference'ınız varsa, derleyici reference scope'tan çıkmadan verinin scope'tan çıkmayacağını garanti eder.

``` 
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s // s'in referansını döndürmeye çalışıyoruz
} // s scope'tan çıkar ve drop edilir - TEHLİKE!
```

**Hata:**
```
error[E0106]: missing lifetime specifier
this function's return type contains a borrowed value, 
but there is no value for it to be borrowed from
```

**Çözüm**: String'i doğrudan döndür:

``` 
fn no_dangle() -> String {
    let s = String::from("hello");
    s // Ownership taşınır, hiçbir şey drop edilmez
}
```

### 📋 Reference Kuralları Özeti

1. **Herhangi bir anda, ya bir mutable reference ya da herhangi bir sayıda immutable reference olabilir** (ama ikisi birden değil)
2. **Reference'lar her zaman geçerli olmalıdır**

---

## 📚 Bölüm 3: The Slice Type (Dilim Tipi)

### 🎯 Problem: String'in Bir Kısmına Erişim

Bir cümledeki ilk kelimeyi bulan bir fonksiyon yazmak istediğimizi düşünelim:

``` 
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i; // Kelimenin bittiği index'i döndür
        }
    }
    
    s.len()
}
```

**Sorun**: Sadece bir `usize` döndürüyoruz. Bu sayı `String` ile bağlantılı değil:

``` 
let mut s = String::from("hello world");
let word = first_word(&s); // word = 5

s.clear(); // String'i boşaltır

// word hala 5 ama s artık boş! word artık geçersiz!
```

Bu bir bug ama derleyici bunu yakalamaz!

### ✂️ String Slice Çözümü

**String slice**, bir `String`'in ardışık bir bölümüne referanstır:

``` 
let s = String::from("hello world");

let hello = &s[0..5];   // "hello"
let world = &s[6..11];  // "world"
```

**Sözdizimi**: `[başlangıç_index..bitiş_index]`
- `başlangıç_index`: ilk pozisyon
- `bitiş_index`: son pozisyondan bir fazlası

**Dahili yapı**: Slice, başlangıç pointer'ı ve uzunluk saklar.

#### Range Sözdizimi Kısayolları

``` 
let s = String::from("hello");

// Başlangıçtan başlıyorsa:
let slice = &s[0..2];
let slice = &s[..2]; // Aynı şey

// Sona kadar gidiyorsa:
let slice = &s[3..len];
let slice = &s[3..]; // Aynı şey

// Tüm string:
let slice = &s[0..len];
let slice = &s[..]; // Aynı şey
```

### 🎯 first_word'ü Slice ile Yeniden Yazma

``` 
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // String slice döndürür
        }
    }
    
    &s[..]
}
```

**Tip**: `&str` (string slice tipi)

Şimdi bug imkansız:

``` 
let mut s = String::from("hello world");
let word = first_word(&s);

s.clear(); // ❌ HATA!

println!("the first word is: {word}");
```

**Hata:**
```
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
```

`clear()` mutable reference gerektirir ama `word` hala immutable reference tutuyor.   bunu derlemez!

### 📝 String Literalleri ve &str

``` 
let s = "Hello, world!";
```

Burada `s`'in tipi **`&str`**'dir! String literali aslında binary'deki belirli bir noktaya işaret eden bir slice'dır. Bu yüzden string literalleri immutable'dır - `&str` immutable bir referanstır.

### 🎯 Daha Genel API

Daha tecrübeli  acean'lar fonksiyonu şöyle yazar:

``` 
fn first_word(s: &str) -> &str {
    // &String yerine &str kullanır
    // ...
}
```

Bu hem `&String` hem de `&str` kabul eder:

``` 
let my_string = String::from("hello world");

// String üzerinde çalışır
first_word(&my_string[..]);
first_word(&my_string);

// String literali üzerinde çalışır
let my_string_literal = "hello world";
first_word(&my_string_literal[0..6]);
first_word(my_string_literal); // &str zaten slice
```

### 📊 Genel Slice Tipi

String slice sadece string'ler için değil, array'ler için de slice var:

``` 
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3]; // Tip: &[i32]

assert_eq!(slice, &[2, 3]);
```

Bu slice da aynı şekilde çalışır: ilk elemana pointer ve uzunluk saklar.

---

## 🎓 Özet ve Sonuç

### Ownership Sistemi

**Üç temel kural:**
1. Her değerin bir sahibi var
2. Aynı anda sadece bir sahip olabilir
3. Sahip scope'tan çıkınca değer drop edilir

### Move vs Copy

- **Move**: Heap verisi olan tiplerde (String, Vec, vb.) atama yapıldığında ownership taşınır
- **Copy**: Stack'te saklanan basit tiplerde (i32, bool, char, vb.) kopyalama olur

### References

- `&` ile immutable reference oluşturulur
- `&mut` ile mutable reference oluşturulur
- **Kural**: Ya bir mutable reference ya da birden fazla immutable reference (ama ikisi birden değil)
- Reference'lar her zaman geçerli olmalı

### Slices

- Bir koleksiyonun ardışık bir bölümüne referanstır
- `&str` string slice tipidir
- `&[T]` genel slice tipidir
- Ownership'i yoktur, sadece referanstır

### 🎯 Neden Bu Kadar Önemli?

Ownership sistemi:
- ✅ **Derleme zamanında** bellek güvenliğini garanti eder
- ✅ **Runtime maliyeti yoktur** - çöp toplayıcı yok
- ✅ **Data race'leri** önler
- ✅ **Dangling pointer'ları** önler
- ✅ **Double free** hatalarını önler

 'ta ownership, borrowing ve slice kavramları bellek güvenliğini derleme zamanında sağlar. Diğer sistem programlama dilleri gibi bellek üzerinde tam kontrolünüz vardır ama  'ın otomatik temizleme mekanizması sayesinde ekstra kod yazmak ve debug etmek zorunda değilsiniz.

---

## 📝 Pratik İpuçları

1. **Ownership hataları aldığınızda**: Değeri clone'lamayı veya reference kullanmayı düşünün
2. **Mutable reference gerektiğinde**: Değişkenin `mut` olduğundan emin olun
3. **Fonksiyon yazarken**: Mümkünse `&str` kullanın, `&String` değil
4. **Slice kullanın**: Index döndürmek yerine slice döndürmek daha güvenli
5. **Compiler mesajlarını okuyun**:  'ın hata mesajları çok açıklayıcı ve yardımcı

Bu konuyu iyice anlamak için bol bol pratik yapın! Ownership  'ın en önemli kavramıdır ve onu anladığınızda dilin geri kalanı çok daha kolay gelecektir.

Başarılar! 🚀