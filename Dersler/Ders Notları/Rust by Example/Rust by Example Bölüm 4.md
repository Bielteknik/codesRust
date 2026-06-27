# Rust'ta Variable Bindings (Değişken Bağlamaları) - Kapsamlı Ders

Merhaba! Bu derste Rust'ın temel taşlarından biri olan **Variable Bindings** (değişken bağlamaları) konusunu tüm alt başlıklarıyla birlikte ders anlatır gibi işleyeceğiz. Hazırsanız başlayalım! 🚀

---

## 📚 Bölüm 1: Variable Bindings Nedir?

Rust'ta değişkenlere "değişken" demek yerine **"bağlama" (binding)** denir. Bu isimlendirme tesadüf değil! Çünkü Rust'ta bir değişken, aslında bir değere **bağlanan** bir isimdir.

### Temel Söz Dizimi

```rust
let değişken_ismi = değer;
```

### Tür Belirtimi (Type Annotation)

Rust **statik tipleme** (static typing) kullanan bir dildir. Bu, değişkenlerin türlerinin derleme zamanında bilinmesi gerektiği anlamına gelir. İki şekilde tür belirtebilirsiniz:

**1. Açık Tür Belirtimi (Explicit Type Annotation):**
```rust
let yaş: i32 = 25;
let isim: &str = "Ahmet";
let pi: f64 = 3.14159;
```

**2. Tür Çıkarımı (Type Inference):**
Çoğu durumda derleyici, değeri görerek türü otomatik olarak çıkarabilir:
```rust
let sayı = 42;        // Derleyici bunu i32 olarak anlar
let ondalık = 3.14;   // Derleyici bunu f64 olarak anlar
let doğru_mu = true;  // Derleyici bunu bool olarak anlar
```

> 💡 **İpucu:** Rust'ın tür çıkarımı oldukça güçlüdür, bu yüzden çoğu zaman tür belirtmeye gerek kalmaz. Bu, kodu daha temiz ve okunabilir kılar!

---

## 📚 Bölüm 2: Mutability (Değişkenlik)

Rust'ın en önemli özelliklerinden biri: **Değişkenler varsayılan olarak değişmezdir (immutable).**

### Varsayılan: Değişmezlik

```rust
let x = 5;
println!("x'in değeri: {}", x);

x = 6; // ❌ HATA! Değişmez bir değeri değiştiremezsiniz
println!("x'in yeni değeri: {}", x);
```

Bu kodu derlemeye çalışırsanız şöyle bir hata alırsınız:
```
error[E0384]: cannot assign twice to immutable variable `x`
```

### `mut` Anahtar Kelimesi

Bir değişkeni değiştirilebilir yapmak için `mut` anahtar kelimesini kullanırız:

```rust
let mut y = 5;
println!("y'nin ilk değeri: {}", y);

y = 6; // ✅ Artık sorun yok!
println!("y'nin yeni değeri: {}", y);
```

### Neden Varsayılan Olarak Değişmez?

Bu tasarım tercihinin birkaç önemli nedeni var:

1. **Güvenlik:** Değişmez değerler, beklenmedik değişiklikleri önler
2. **Eşzamanlılık (Concurrency):** Değişmez veriler thread-safe'tir
3. **Okunabilirlik:** Bir değerin değişmeyeceğini bilmek kodu anlamayı kolaylaştırır
4. **Derleyici Optimizasyonu:** Derleyici değişmez değerler için daha iyi optimizasyon yapabilir

### Örnek Uygulama

```rust
fn main() {
    let değişmez_sayı = 10;
    let mut değişebilir_sayı = 10;
    
    println!("Başlangıç değerleri:");
    println!("  Değişmez: {}", değişmez_sayı);
    println!("  Değişebilir: {}", değişebilir_sayı);
    
    // değişmez_sayı = 20;  // ❌ Bu satır hata verir
    
    değişebilir_sayı = 20;  // ✅ Bu çalışır
    println!("\nDeğişiklikten sonra:");
    println!("  Değişmez: {}", değişmez_sayı);
    println!("  Değişebilir: {}", değişebilir_sayı);
}
```

---

## 📚 Bölüm 3: Scope ve Shadowing (Kapsam ve Gölgeleme)

### Scope (Kapsam) Nedir?

Her değişkenin bir **kapsamı** vardır ve değişkenler **blok** adı verilen süslü parantezler `{}` içinde yaşarlar.

```rust
fn main() {
    // Bu bağlama main fonksiyonunda yaşar
    let uzun_ömürlü = 1;
    
    {
        // Bu bir blok ve main'den daha dar bir kapsama sahip
        let kısa_ömürlü = 2;
        
        println!("iç kısa: {}", kısa_ömürlü);  // ✅ Çalışır
        println!("iç uzun: {}", uzun_ömürlü);  // ✅ Çalışır
    }
    // Blok sonu
    
    // println!("dış kısa: {}", kısa_ömürlü);  // ❌ HATA! kısa_ömürlü artık yok
    println!("dış uzun: {}", uzun_ömürlü);     // ✅ Çalışır
}
```

**Kural:** İç bloklar, dış bloklardaki değişkenlere erişebilir ama dış bloklar iç bloklardaki değişkenlere erişemez.

### Shadowing (Gölgeleme) Nedir?

Aynı isimle yeni bir değişken tanımlayabilirsiniz. Bu yeni değişken, eskisini **gölgeler** (shadow):

```rust
fn main() {
    let gölgelenen = 1;
    
    {
        println!("gölgelenmeden önce: {}", gölgelenen);  // 1
        
        // Bu bağlama dıştakini *gölgeler*
        let gölgelenen = "abc";
        
        println!("iç blokta gölgelenen: {}", gölgelenen);  // "abc"
    }
    
    println!("iç blok dışında: {}", gölgelenen);  // 1 (eski değer geri döndü)
    
    // Bu bağlama öncekini *gölgeler*
    let gölgelenen = 2;
    println!("dış blokta gölgelenen: {}", gölgelenen);  // 2
}
```

### Shadowing vs `mut` Farkı

Bu çok önemli bir ayrım:

**Shadowing:**
```rust
let spaces = "   ";        // &str türünde
let spaces = spaces.len(); // usize türünde - YENİ bir değişken!
```
- Tür değiştirebilirsiniz
- `let` kullanırsınız
- Aslında yeni bir değişken yaratırsınız

**Mut:**
```rust
let mut spaces = "   ";
// spaces = spaces.len();  // ❌ HATA! Tür değiştiremezsiniz
spaces = "abc";            // ✅ Sadece aynı türde değer atayabilirsiniz
```
- Tür değişmez
- `mut` kullanırsınız
- Aynı değişkeni değiştirirsiniz

---

## 📚 Bölüm 4: Declare First (Önce Tanımlama)

Rust'ta bir değişkeni önce tanımlayıp, sonra başlatabilirsiniz. **ANCAK** kullanmadan önce mutlaka başlatılmalıdır!

### Geçerli Kullanım

```rust
fn main() {
    let a;      // Önce tanımla
    
    // ... bazı işlemler ...
    
    a = 42;     // Sonra başlat
    
    println!("a'nın değeri: {}", a);  // ✅ Kullan, başlatıldı
}
```

### Geçersiz Kullanım

```rust
fn main() {
    let b;
    // println!("b: {}", b);  // ❌ HATA! Başlatılmadan kullanılamaz
    b = 10;
    println!("b: {}", b);     // ✅ Şimdi çalışır
}
```

### Koşullu Başlatma

```rust
fn main() {
    let değer;
    let koşul = true;
    
    if koşul {
        değer = 100;
    } else {
        değer = 200;
    }
    
    println!("değer: {}", değer);  // ✅ Her iki yolda da başlatıldı
}
```

### İyi Uygulama Önerisi

Genellikle değişkeni **tanımladığınız yerde başlatmanız** önerilir:

```rust
// ❌ Yaygın değil, okuması zor
let x;
// ... 20 satır kod ...
x = 42;

// ✅ Yaygın ve okunabilir
let x = 42;
```

**Neden?** Çünkü okuyucu için değişkenin nerede başlatıldığını bulmak zorlaşır. Değişkeni kullanacağınız yere yakın tanımlamak en iyi pratiktir.

---

## 📚 Bölüm 5: Freezing (Dondurma)

Bu konu Rust'ın ilginç ve güçlü özelliklerinden biridir. Bir veriye **değişmez** olarak aynı isimle bağlandığında, o veri **dondurulur** (freeze). Dondurulmuş veri, değişmez bağlam kapsamı dışına çıkana kadar değiştirilemez.

### Temel Konsept

```rust
fn main() {
    let mut mut_data = 1;
    
    {
        // Aynı isimle değişmez bağlam oluştur
        let freeze = mut_data;  // mut_data artık donduruldu!
        
        println!("dondurulmuş içinde: {}", freeze);
        
        // mut_data = 99;  // ❌ HATA! freeze kapsamındayken değiştirilemez
    }
    
    // freeze kapsamı bitti, artık mut_data'yı değiştirebiliriz
    mut_data = 99;
    println!("dondurma bitti: {}", mut_data);  // ✅ Çalışır
}
```

### Neden Önemli?

Dondurma konsepti, Rust'ın **ödünç alma** (borrowing) sisteminin temelini oluşturur. Bir veriyi değişmez olarak ödünç aldığınızda, o veri dondurulur ve değiştirilemez hale gelir. Bu, veri yarışlarını (data races) önler.

### Pratik Örnek

```rust
fn main() {
    let mut sayı = 5;
    println!("1. sayı: {}", sayı);
    
    {
        let _dondur = &sayı;  // Değişmez referans - sayı dondu!
        println!("2. dondurulmuş: {}", _dondur);
        
        // sayı = 10;  // ❌ HATA! Dondurulmuş durumda
    }
    
    sayı = 10;  // ✅ Artık dondurma bitti, değiştirebiliriz
    println!("3. dondurma bitti: {}", sayı);
}
```

---

## 🎯 Özet Tablosu

| Konu | Anahtar Kelime | Açıklama |
|------|----------------|----------|
| **Bağlama** | `let` | Değere isim bağlama |
| **Değişmezlik** | (varsayılan) | Değer değiştirilemez |
| **Değişebilirlik** | `let mut` | Değer değiştirilebilir |
| **Kapsam** | `{}` | Değişkenin yaşadığı alan |
| **Gölgeleme** | `let` (aynı isim) | Eski bağlamı gizleme |
| **Dondurma** | Değişmez bağlam | Değiştirmeyi geçici engelleme |

---

## 💻 Tam Örnek Kod

Tüm konuları birleştiren kapsamlı bir örnek:

```rust
fn main() {
    println!("=== Rust Variable Bindings Demo ===\n");
    
    // 1. Temel bağlamalar
    let isim = "Rust";
    let versiyon: &str = "2024";
    println!("1. {} - Versiyon: {}", isim, versiyon);
    
    // 2. Değişebilirlik
    let mut sayaç = 0;
    sayaç += 1;
    sayaç += 1;
    println!("2. Sayaç: {}", sayaç);
    
    // 3. Kapsam
    let dış = "dış";
    {
        let iç = "iç";
        println!("3. Blok içi: {} ve {}", dış, iç);
    }
    // println!("iç: {}", iç);  // ❌ iç artık erişilemez
    
    // 4. Gölgeleme
    let x = 5;
    let x = x * 2;
    let x = "artık string";
    println!("4. Gölgeleme: {}", x);
    
    // 5. Önce tanımlama
    let sonradan;
    sonradan = 42;
    println!("5. Sonradan başlatılan: {}", sonradan);
    
    // 6. Dondurma
    let mut değişken = 10;
    {
        let _dondurucu = &değişken;
        // değişken = 20;  // ❌ Dondurulmuş!
    }
    değişken = 20;  // ✅ Artık değiştirilebilir
    println!("6. Dondurma sonrası: {}", değişken);
    
    println!("\n=== Demo Tamamlandı ===");
}
```

---

## 🎓 Öğrenme Kontrol Listesi

Bu dersi tamamladıktan sonra şunları yapabilmelisiniz:

- [x] `let` ile değişken bağlamaları oluşturmak
- [x] Tür belirtimi ve tür çıkarımı arasındaki farkı anlamak
- [x] `mut` ile değişebilir değişkenler oluşturmak
- [x] Kapsam (scope) kavramını anlamak
- [x] Gölgeleme (shadowing) yapabilmek
- [x] Shadowing ile `mut` arasındaki farkı bilmek
- [x] Değişkenleri önce tanımlayıp sonra başlatabilmek
- [x] Dondurma (freezing) konseptini anlamak

---

## 🚀 Sıradaki Adım

Variable Bindings konusunu öğrendiğinize göre, artık Rust'ın **temel veri tipleri** (Types), **fonksiyonlar** (Functions) ve **kontrol akışı** (Control Flow) konularına geçebilirsiniz!

Sorularınız varsa sormaktan çekinmeyin. Başarılar! 🎉

# DeepSeek Yorumu --------------------

Merhaba! Rust By Example’ın **Variable Bindings** (Değişken Bağlamaları) sayfasını baştan sona, bir ders anlatır gibi ele alalım. Tüm alt başlıkları (değişken tanımlama, değişebilirlik, kapsam ve gölgeleme, önce tanımlama, dondurma) olabildiğince detaylı ve öğretici bir şekilde açıklayacağım. Hazırsan başlayalım.

---

## 1. Değişken Bağlamalarına Giriş

Rust’ta bir değere isim vermek için `let` anahtar kelimesini kullanırız. Buna **değişken bağlama (variable binding)** denir çünkü aslında bir ismi bir değere *bağlarız*. Çoğu dilde buna “değişken tanımlama” denir, ancak Rust’ta durum biraz farklıdır; çünkü varsayılan olarak bağlamalar değişmezdir (immutable).

```rust
fn main() {
    let x = 5;
    println!("x'in değeri: {}", x);
}
```

Burada `x` bağlaması `5` tamsayısına bağlanmıştır. `x`’in türünü biz yazmadık; Rust tür çıkarımı (type inference) sayesinde `x`’in `i32` olduğunu anlar. İstersek türü açıkça belirtebiliriz:

```rust
let x: i32 = 5;
```

Tür belirtmek zorunlu değildir, ancak bazen derleyiciye yardımcı olmak veya kodu daha okunaklı kılmak için kullanılır.

---

## 2. Değişebilirlik (Mutability)

Rust’ın en ayırt edici özelliklerinden biri: **bağlamalar varsayılan olarak değişmezdir**. Yani bir kez değer atadıktan sonra onu değiştiremezsiniz.

```rust
let x = 5;
x = 6; // HATA! `x` değişmez bir bağlamadır.
```

Bu kod derlenmez. Hata mesajı şuna benzer: *"cannot assign twice to immutable variable `x`"*. Rust bizi, istemeden değer değiştirme hatalarından korur.

Eğer bir bağlamanın değerini sonradan değiştirmek istiyorsak `mut` anahtar kelimesini ekleriz:

```rust
let mut y = 5;
println!("y: {}", y); // 5
y = 10;
println!("y: {}", y); // 10
```

`mut` ile işaretlenen bağlamalar artık **değişebilir (mutable)** hale gelir. Değişebilirlik, Rust’ta sadece değer ataması için değil, aynı zamanda referanslar (`&mut`) ile veri ödünç alırken de büyük rol oynar. İleride göreceğimiz “dondurma” konusu bununla ilgilidir.

**Neden varsayılan değişmezlik?** Rust’ın güvenlik ve eşzamanlılık garantileri büyük ölçüde bu tasarıma dayanır. Çoğu değişkenin değişmez olması, programın mantığını akıl yürütmeyi kolaylaştırır.

---

## 3. Kapsam ve Gölgeleme (Scope and Shadowing)

Bağlamalar, tanımlandıkları blok içinde geçerlidir. Blok `{}` ile sınırlıdır.

```rust
fn main() {
    let a = 1;
    {
        let b = 2;
        println!("a: {}, b: {}", a, b); // a ve b görünür
    }
    println!("a: {}", a); // a görünür
    // println!("b: {}", b); // HATA! b burada tanımlı değil
}
```

Kapsam kuralları çoğu dilden tanıdıktır. Ancak Rust’ın **gölgeleme (shadowing)** özelliği biraz farklıdır.

### Gölgeleme (Shadowing)

Aynı isimde yeni bir bağlama tanımlayarak önceki bağlamayı **gölgeleyebilirsiniz**. Bu, değişebilirlikten farklıdır; çünkü yepyeni bir bağlama oluşur, eskisi hâlâ bellekte durur ancak ismi görünmez olur.

```rust
fn main() {
    let x = 5;
    let x = x + 1;   // yeni bir x bağlaması, eski x'i gölgeler
    let x = x * 2;   // bir kez daha gölgeleme
    println!("x: {}", x); // 12 (6 * 2)
}
```

Burada her `let x = ...` ifadesi tamamen yeni bir bağlama oluşturur. İlk `x` hâlâ vardır, ancak blok sonuna kadar erişilemez. Gölgeleme sayesinde bir değişkeni dönüştürürken yeni bir isim bulmak zorunda kalmayız. Ayrıca türü bile değiştirebiliriz:

```rust
let spaces = "   ";         // &str (string dilimi)
let spaces = spaces.len();  // usize, önceki "spaces"i gölgeler
```

`mut` ile bu mümkün olmazdı; çünkü `mut` sadece değeri değiştirir, tür aynı kalmalıdır. Gölgeleme ise yepyeni bir bağlama olduğu için tür tamamen farklı olabilir. Bu, Rust’ta çok kullanışlı bir desendir.

**Gölgeleme ve kapsam ilişkisi:**

```rust
let x = 10;
{
    let x = "merhaba";   // iç blokta x'i string olarak gölgeledik
    println!("iç blok x: {}", x); // "merhaba"
}
println!("dış blok x: {}", x);    // 10
```

İç bloktaki `x`, dış bloktaki `x`’i gölgeler; iç bloktan çıkınca gölge kalkar, orijinal `x` tekrar görünür olur.

---

## 4. Önce Tanımlama (Declare First)

Rust’ta bir bağlamayı değer vermeden önce tanımlayabilir, daha sonra ilk değerini atayabiliriz. Ancak **kullanılmadan önce mutlaka ilk değer atanmış olmalıdır**. Derleyici, başlatılmamış değişken kullanımına izin vermez.

```rust
fn main() {
    let x;          // tanımlandı ama başlatılmadı
    // println!("{}", x); // HATA! x henüz başlatılmadı
    x = 42;
    println!("{}", x); // sorunsuz, çünkü artık x'in değeri var
}
```

Bu özellik, özellikle bir değişkenin değerinin bir koşula bağlı olarak farklı yerlerde atanması gerektiğinde kullanışlıdır:

```rust
let y;
if condition {
    y = 1;
} else {
    y = 2;
}
// burada y kesinlikle başlatılmıştır, derleyici bunu kontrol eder.
println!("y: {}", y);
```

Derleyicinin akıllı kontrolü sayesinde, programın her dalında değişkenin bir değer aldığından emin oluruz. Bir dalda atlanırsa derleme hatası alırız. Bu, Rust’ın güvenlik odaklı tasarımının güzel bir örneğidir.

**Not:** `let x;` ifadesinde tür belirtmek zorunda kalabilirsiniz çünkü derleyici türü çıkaramaz (eğer sonraki atamalardan tür net değilse):

```rust
let x: i32; // türü açıkça belirttik
x = 10;
```

---

## 5. Dondurma (Freezing)

Rust’ta bir veri, değişmez (immutable) bir referans (`&T`) ile ödünç alındığında, orijinal bağlama **dondurulur** (frozen). Yani, ödünç alınan veri üzerinde ödünç devam ettiği sürece değişiklik yapılamaz. Bu kural, veri yarışlarını (data races) engeller.

```rust
fn main() {
    let mut x = 5;
    {
        let y = &x;        // x'i değişmez olarak ödünç aldık
        // x = 10;         // HATA! x şu anda donduruldu, değiştirilemez
        println!("y: {}", y);
    }                     // y'nin ödüncü burada sona erer
    x = 10;               // artık x tekrar değiştirilebilir
    println!("x: {}", x);
}
```

Yukarıda `let y = &x;` ile `x`’in değerine değişmez bir referans oluşturduk. Bu ödünç alma (borrow) boyunca `x` dondurulur; yani `mut` olmasına rağmen yeni bir değer atayamayız. `y`’nin kapsamı bitince dondurma kalkar, `x` yeniden değişebilir hale gelir.

Dondurma, Rust’ın mülkiyet (ownership) ve ödünç alma (borrowing) sisteminin doğal bir sonucudur. Derleyici, aynı anda hem değişmez referanslar hem de değişebilir bir referans veya doğrudan değişiklik olmasına izin vermez. Bu sayede programın belirli bir bölümünde verinin değişmeyeceğini garanti ederiz.

**Pratik önemi:** Dondurma sayesinde, bir koleksiyon üzerinde döngüyle gezinirken koleksiyonu değiştirmeye çalıştığınızda derleme hatası alırsınız. Bu, çoğu dilde yaygın bir hata kaynağı olan “iteration invalidation” problemlerini tamamen ortadan kaldırır.

---

## Genel Bakış ve Özet

Rust’ta değişken bağlamaları:

- **`let`** ile oluşturulur, varsayılan olarak **değişmezdir**.
- **`mut`** anahtar kelimesi ile değişebilir hale getirilir.
- **Gölgeleme** sayesinde aynı isimle yeni bir bağlama oluşturulabilir, hatta tür değiştirilebilir.
- **Kapsam**, bloklarla sınırlıdır; iç blokta yapılan gölgeleme dış blokta etkili olmaz.
- **Önce tanımlama** ile değişkeni ilk değer vermeden bırakabiliriz, ancak derleyici kullanımdan önce mutlaka başlatıldığını kontrol eder.
- **Dondurma** mekanizması, değişmez ödünç alma sırasında veriyi değişikliğe karşı kilitler; ödünç bitene kadar mutasyon engellenir.

Bu temel kurallar, Rust’ın hem yüksek performanslı hem de güvenli kod yazmayı nasıl başardığını anlamanın ilk adımlarıdır. Değişmezlik ve mülkiyet sistemi, programcının niyetini derleyiciye açıkça anlatmasını sağlar ve sınıfının en iyisi hata mesajlarıyla yanlışları erkenden yakalar.

Sıradaki konulara (örneğin, türler, dönüşümler, kontrol akışı) geçmeden önce bu kavramları iyice sindirmeni öneririm. Elini kirletmek için küçük denemeler yapabilirsin; örneğin gölgeleme ile dönüşümler yap, `mut` ile gölgelemenin farkını gözlemle, dondurma hatasını bilinçli olarak üretip hata mesajını oku. Rust öğrenirken derleyici en büyük öğretmenindir.

Umarım anlaşılır ve kapsamlı bir ders olmuştur. Başka soruların olursa çekinmeden sorabilirsin!

