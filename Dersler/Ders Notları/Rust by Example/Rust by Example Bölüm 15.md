Harika! Rust By Example'ın "Scope Rules" (Kapsam Kuralları) bölümünü ve alt başlıklarını inceledim. Şimdi size bu konuyu ders anlatır gibi detaylı, örneklerle ve kapsamlı bir şekilde açıklayacağım.

---

# 📚 Rust'ta Kapsam Kuralları (Scope Rules) - Detaylı Ders

## 🎯 Giriş: Kapsam (Scope) Nedir?

Rust'ta **kapsam (scope)**, değişkenlerin, fonksiyonların ve diğer tanımlamaların nerede geçerli olduğunu belirleyen temel bir kavramdır [[5]]. Kapsamlar; ownership (sahiplik), borrowing (ödünç alma) ve lifetime (yaşam ömrü) kavramlarının anlaşılmasında kritik rol oynar [[5]].

Basitçe söylemek gerekirse:
- **Kapsam**, bir değişkenin "nerede var olduğunu" ve "nerede kullanılabileceğini" belirler
- Derleyiciye borrmaların ne zaman geçerli olduğunu, kaynakların ne zaman serbest bırakılabileceğini ve değişkenlerin ne zaman oluşturulup yok edileceğini gösterir [[5]]

---

## 1️⃣ Blok Kapsamı (Block Scope)

Rust'ta değişken bağlamaları (variable bindings) bir **blok** içinde yaşamak zorundadır [[19]]. Blok, süslü parantezler `{}` ile çevrili ifade koleksiyonudur [[19]].

### 📖 Temel Örnek:

```rust
fn main() {
    // Bu bağlama main fonksiyonunda yaşar
    let long_lived_binding = 1;

    // Bu bir bloktur ve main fonksiyonundan daha küçük bir kapsama sahiptir
    {
        // Bu bağlama sadece bu blokta var olur
        let short_lived_binding = 2;
        
        println!("inner short: {}", short_lived_binding);
    } // Bloğun sonu

    // HATA! `short_lived_binding` bu kapsamda mevcut değil
    println!("outer short: {}", short_lived_binding);
    // FIXME ^ Bu satırı yorum yapın

    println!("outer long: {}", long_lived_binding);
}
```

### 🔍 Analiz:

1. **`long_lived_binding`**: `main` fonksiyonunun başında tanımlandığı için, fonksiyonun tamamında geçerlidir
2. **`short_lived_binding`**: İç blokta `{}` tanımlandığı için, sadece o blok içinde geçerlidir
3. Blok sona erdiğinde (`}`), blok içindeki tüm değişkenler **kapsam dışı çıkar** ve erişilemez hale gelir

### 💡 Önemli Nokta:

Rust'ta her `{ ... }` bloğu yeni bir kapsam oluşturur. Bu, değişkenlerin yaşam ömrünü otomatik olarak yönetir ve bellek güvenliğini sağlar.

---

## 2️⃣ Değişken Gölgeleme (Variable Shadowing)

Rust'ta **variable shadowing** (değişken gölgeleme) izin verilen bir özelliktir [[19]]. Bu, aynı isimle yeni bir değişken tanımladığınızda, yeni değişkenin eski değişkeni "gölgelemesi" anlamına gelir [[21]].

### 📖 Gölgeleme Örneği:

```rust
fn main() {
    let shadowed_binding = 1;

    {
        println!("before being shadowed: {}", shadowed_binding);

        // Bu bağlama dıştakini *gölgeler*
        let shadowed_binding = "abc";
        
        println!("shadowed in inner block: {}", shadowed_binding);
    }
    println!("outside inner block: {}", shadowed_binding);

    // Bu bağlama önceki bağlamayı *gölgeler*
    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);
}
```

### 🔍 Analiz:

1. **İlk tanım**: `shadowed_binding = 1` (integer)
2. **İç blokta**: Aynı isimle yeni bir değişken tanımlanır: `shadowed_binding = "abc"` (string)
   - Bu yeni değişken, dış bloktaki değişkeni gölgeler
   - İç blokta artık `"abc"` değeri kullanılır
3. **İç blok bittiğinde**: Gölgeleme sona erer, dış bloktaki orijinal `1` değeri geri döner
4. **Dış blokta tekrar tanım**: `shadowed_binding = 2`
   - Bu, önceki `1` değerini gölgeler

### 💡 Gölgelemenin Özellikleri:

- ✅ **Farklı tipler**: Gölgeleyen değişken, farklı bir tipe sahip olabilir (örn: `i32` → `&str`)
- ✅ **Aynı isim**: Aynı değişken ismi kullanılabilir
- ✅ **Geçici**: Gölgeleme sadece mevcut kapsamda geçerlidir
- ✅ **Değiştirilemezlik**: Rust'ta değişkenler varsayılan olarak değiştirilemez (immutable) olduğundan, gölgeleme değer değiştirmenin bir yoludur

### 🎯 Gölgeleme vs Değiştirilebilirlik (Mutability):

```rust
// Gölgeleme ile
let x = 5;
let x = x + 1;  // Yeni bir değişken oluşturulur

// Mutability ile
let mut y = 5;
y = y + 1;  // Aynı değişken değiştirilir
```

---

## 3️⃣ Önce Bildir, Sonra Başlat (Declare First)

Rust'ta değişken bağlamalarını önce bildirip daha sonra başlatmak mümkündür, ancak **tüm değişken bağlamaları kullanılmadan önce başlatılmalıdır** [[28]]. Derleyici, başlatılmamış değişkenlerin kullanımını yasaklar çünkü bu tanımsız davranışa yol açabilir [[28]].

### 📖 Önce Bildirim Örneği:

```rust
fn main() {
    // Bir değişken bağlaması bildir
    let a_binding;

    {
        let x = 2;

        // Bağlamayı başlat
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // HATA! Başlatılmamış bağlama kullanımı
    println!("another binding: {}", another_binding);
    // FIXME ^ Bu satırı yorum yapın

    another_binding = 1;

    println!("another binding: {}", another_binding);
}
```

### 🔍 Analiz:

1. **`a_binding`**: Önce bildirilir, sonra iç blokta başlatılır ve başarıyla kullanılır
2. **`another_binding`**: Bildirilir ama başlatılmadan kullanılmaya çalışılır → **DERLEME HATASI**
3. Derleyici, başlatılmamış değişken kullanımını **compile-time**'da tespit eder

### 💡 Best Practice (En İyi Uygulama):

Rust topluluğunda, değişken bildirimini ve başlatmayı ayırmak **yaygın değildir** [[28]]. Bunun nedenleri:

- ❌ Okuyucunun başlatmayı bulması zorlaşır
- ❌ Kodun okunabilirliği azalır
- ✅ Değişkeni kullanmadan hemen önce tanımlamak ve başlatmak daha iyidir

### ✅ Önerilen Yaklaşım:

```rust
// ❌ Yaygın olmayan yaklaşım
let x;
// ... birçok satır ...
x = 5;

// ✅ Önerilen yaklaşım
let x = 5;  // Tanımla ve hemen başlat
```

---

## 4️⃣ Kapsam ve Ownership (Sahiplik) İlişkisi

Kapsam kuralları, Rust'ın **ownership system** (sahiplik sistemi) ile yakından ilişkilidir. Bir değişken kapsam dışına çıktığında:

1. **Değişken düşer (drops)**: Bellek otomatik olarak serbest bırakılır
2. **Sahiplik sona erer**: Kaynak üzerindeki sahiplik hakkı biter
3. **Referanslar geçersiz olur**: Değişkene yapılan tüm borrmalar geçersiz hale gelir

### 📖 Ownership ve Kapsam Örneği:

```rust
fn main() {
    let s1 = String::from("merhaba");
    
    {
        let s2 = String::from("dünya");
        println!("s2: {}", s2);
    } // s2 burada düşer (drop), bellek serbest bırakılır
    
    // s1 hala geçerli
    println!("s1: {}", s1);
    
    // println!("s2: {}", s2); // HATA! s2 artık yok
}
```

---

## 5️⃣ İç İçe Bloklar ve Kapsam Hiyerarşisi

Rust'ta bloklar iç içe geçebilir ve her blok kendi kapsamını oluşturur:

### 📖 İç İçe Bloklar:

```rust
fn main() {
    let level1 = 1;
    
    {
        let level2 = 2;
        println!("level1: {}, level2: {}", level1, level2);
        
        {
            let level3 = 3;
            // İç bloklar dış bloklardaki değişkenlere erişebilir
            println!("level1: {}, level2: {}, level3: {}", 
                     level1, level2, level3);
        } // level3 düşer
        
        // println!("level3: {}", level3); // HATA! level3 artık yok
    } // level2 düşer
    
    // println!("level2: {}", level2); // HATA! level2 artık yok
    println!("level1: {}", level1);
} // level1 düşer
```

### 💡 Kapsam Hiyerarşisi Kuralları:

1. ✅ **İç bloklar**, dış bloklardaki değişkenlere erişebilir
2. ❌ **Dış bloklar**, iç bloklardaki değişkenlere erişemez
3. ✅ Gölgeleme, iç bloklarda dış bloklardaki değişkenleri geçersiz kılabilir
4. ✅ En içteki değişken, aynı isimdeki dış değişkenleri gölgeler

---

## 6️⃣ Fonksiyonlar ve Kapsam

Her fonksiyon kendi kapsamını oluşturur:

### 📖 Fonksiyon Kapsamı:

```rust
fn fonksiyon_a() {
    let x = 10;
    println!("fonksiyon_a'da x: {}", x);
}

fn fonksiyon_b() {
    let x = 20;  // fonksiyon_a'daki x ile aynı isim ama farklı değişken
    println!("fonksiyon_b'de x: {}", x);
}

fn main() {
    fonksiyon_a();
    fonksiyon_b();
    // println!("x: {}", x); // HATA! x main'de tanımlı değil
}
```

---

## 7️⃣ Döngüler ve Kapsam

Döngüler de blok oluşturur ve her iterasyonda kapsam yönetimi önemlidir:

### 📖 Döngü Kapsamı:

```rust
fn main() {
    for i in 0..3 {
        let loop_var = i * 2;
        println!("döngü içinde: {}", loop_var);
    }
    // println!("döngü dışında: {}", loop_var); // HATA!
    
    let mut counter = 0;
    while counter < 3 {
        let temp = counter;
        println!("while içinde: {}", temp);
        counter += 1;
    }
}
```

---

## 8️⃣ Pratik Örnekler ve Senaryolar

### 🎯 Örnek 1: Shadowing ile Tip Dönüşümü

```rust
fn main() {
    let spaces = "   ";  // &str tipi
    let spaces = spaces.len();  // usize tipi
    
    println!("Boşluk sayısı: {}", spaces);
    
    // Bu, mut ile yapılamazdı çünkü tip değişiyor
    // let mut spaces = "   ";
    // spaces = spaces.len(); // HATA! Tip uyuşmazlığı
}
```

### 🎯 Örnek 2: Kapsam ile Bellek Yönetimi

```rust
fn buyuk_veri_isle() {
    {
        let buyuk_veri = vec![0; 1_000_000];  // 1MB bellek
        println!("Veri işleniyor...");
        // Veri ile işlemler
    } // buyuk_veri burada düşer, bellek anında serbest bırakılır
    
    println!("İşlem tamamlandı, bellek serbest bırakıldı");
    // buyuk_veri'ye artık erişilemez
}
```

### 🎯 Örnek 3: Koşullu Kapsam

```rust
fn main() {
    let deger = 10;
    
    if deger > 5 {
        let sonuc = deger * 2;
        println!("Sonuç: {}", sonuc);
    }
    // println!("Sonuç: {}", sonuc); // HATA! sonuc sadece if bloğunda geçerli
    
    // Doğru yaklaşım: Değişkeni dışarıda tanımla
    let sonuc;
    if deger > 5 {
        sonuc = deger * 2;
    } else {
        sonuc = deger;
    }
    println!("Final sonuç: {}", sonuc);
}
```

---

## 9️⃣ Yaygın Hatalar ve Çözümleri

### ❌ Hata 1: Kapsam Dışında Değişken Kullanma

```rust
fn main() {
    {
        let x = 5;
    }
    println!("{}", x); // HATA: cannot find value `x` in this scope
}
```

**✅ Çözüm**: Değişkeni dış kapsamda tanımlayın veya kullanımı blok içinde yapın.

### ❌ Hata 2: Başlatılmamış Değişken Kullanma

```rust
fn main() {
    let x;
    println!("{}", x); // HATA: use of possibly-uninitialized variable
    x = 5;
}
```

**✅ Çözüm**: Değişkeni kullanmadan önce mutlaka başlatın.

### ❌ Hata 3: Gölgeleme Karışıklığı

```rust
fn main() {
    let x = 5;
    {
        let x = 10;
        println!("İç: {}", x); // 10 yazdırır
    }
    println!("Dış: {}", x); // 5 yazdırır (gölgeleme sona erdi)
}
```

**✅ Çözüm**: Farklı isimler kullanarak karışıklığı önleyin veya gölgelemenin farkında olun.

---

## 🔟 Özet ve En İyi Uygulamalar

### 📌 Temel Kurallar:

1. **Blok Kapsamı**: Değişkenler `{}` bloklarıyla sınırlıdır
2. **Gölgeleme**: Aynı isimle yeni değişken tanımlanabilir
3. **Başlatma Zorunluluğu**: Değişkenler kullanılmadan önce başlatılmalıdır
4. **Hiyerarşi**: İç bloklar dış blokları görür, tersi geçerli değildir

### ✅ En İyi Uygulamalar:

1. **Yerel Tanımlama**: Değişkenleri kullanılmadan hemen önce tanımlayın
2. **Küçük Bloklar**: Blokları küçük ve odaklı tutun
3. **Anlamlı İsimler**: Gölgeleme yerine farklı isimler kullanmayı tercih edin
4. **Kapsam Farkındalığı**: Değişkenlerin nerede geçerli olduğunu bilin
5. **RAII Prensibi**: Kaynakları kapsam ile yönetin (Resource Acquisition Is Initialization)

### 🎓 Kapsam Kurallarının Önemi:

- **Bellek Güvenliği**: Otomatik bellek yönetimi sağlar
- **Hata Önleme**: Geçersiz değişken kullanımını derleme zamanında yakalar
- **Kod Okunabilirliği**: Değişkenlerin yaşam ömrünü netleştirir
- **Ownership Sistemi**: Rust'ın güçlü sahiplik sisteminin temelini oluşturur

---

## 🚀 Sonraki Adımlar

Kapsam kurallarını anladıktan sonra, şu konulara geçebilirsiniz:

1. **RAII (Resource Acquisition Is Initialization)**: Kaynak yönetimi
2. **Ownership and Moves**: Sahiplik ve taşıma semantiği
3. **Borrowing**: Ödünç alma ve referanslar
4. **Lifetimes**: Yaşam ömürleri ve açık notasyon

Bu konular, Rust'ın bellek güvenliği garantilerinin temelini oluşturur ve kapsam kurallarıyla yakından ilişkilidir.

---

Bu ders, Rust By Example'ın "Scope Rules" bölümünü ve alt başlıklarını kapsamlı bir şekilde ele aldı. Umarım faydalı olmuştur! Başka sorularınız varsa sormaktan çekinmeyin. 🦀

# DeepSeek Yorumu -------------------

Rust By Example'daki "Scoping rules" (Kapsam Kuralları) sayfası, Rust'taki sahiplik (ownership), ödünç alma (borrowing) ve yaşam süreleri (lifetimes) kavramlarının merkezinde yer alan **kapsam (scope)** kavramına bir giriş niteliğindedir.

Sayfa, kapsamların derleyiciye üç temel konuda rehberlik ettiğini vurgular:
1.  **Ödünç almaların (borrows) ne zaman geçerli olduğu**
2.  **Kaynakların ne zaman serbest bırakılabileceği (free)**
3.  **Değişkenlerin ne zaman oluşturulup yok edildiği**

Bu sayfa, Rust'ın bellek güvenliğinin temelini oluşturan bu üç ana mekanizmayı anlamak için bir ön bilgi niteliğindedir.

### 📚 Konunun Derinlemesine İncelenmesi

Sayfanın kendisi kısa bir özet sunsa da, "Scoping rules" başlığı altında ele alınan ana kavramlar ve alt başlıklar şunlardır:

#### 1. RAII (Kaynak Edinimi Başlatmadır)
Rust'ta değişkenler, kapsam dışına çıktıklarında kaynaklarını otomatik olarak serbest bırakırlar. Bu yaklaşıma **RAII** (Resource Acquisition Is Initialization) denir. Bir değişken, kapsam içinde oluşturulduğunda kaynağa (örneğin, heap üzerinde ayrılmış bir bellek) sahip olur ve kapsamdan çıktığında `drop` fonksiyonu çağrılarak kaynak temizlenir. Bu sayede bellek sızıntıları ve kaynak yönetimiyle ilgili hatalar büyük ölçüde önlenir.

#### 2. Değişken Kapsamı ve Gölgeleme (Shadowing)
Bir değişken, içinde bulunduğu blok (`{}`) boyunca geçerlidir. Rust'ta aynı isimle yeni bir değişken tanımlamak, öncekini **gölgeleyebilir (shadowing)**. Bu, önceki değişkenin değerini değiştirmekten farklıdır; aslında yeni bir değişken oluşturulur ve önceki değişken kapsam dışı kalana kadar erişilemez hale gelir. Gölgeleme, özellikle bir değerin türünü dönüştürmek veya geçici olarak değiştirmek için kullanışlıdır.

#### 3. Ödünç Alma (Borrowing) ve Referansların Kapsamı
Rust'ta bir değeri taşımak (move) yerine, referans (`&`) kullanarak **ödünç alabilirsiniz (borrow)**. Ödünç almanın en önemli kuralı, bir referansın kapsamının (yaşam süresinin), referans gösterilen verinin kapsamını **aşamamasıdır**. Derleyici, bu kuralı denetleyerek **asılı referanslara (dangling references)** karşı koruma sağlar. Yani, bir veri kapsam dışına çıktıktan sonra ona işaret eden bir referans kullanılamaz.

#### 4. Yaşam Süreleri (Lifetimes)
Yaşam süreleri, referansların ne kadar süreyle geçerli olduğunu belirtmek için kullanılan etiketlerdir (`'a`). Çoğu durumda derleyici bu süreleri kendisi çıkarım yapabilse de (lifetime elision), bazen özellikle fonksiyon imzalarında referanslar arasındaki ilişkiyi açıkça belirtmek gerekir. Yaşam süreleri, referansların kapsamlar arasında güvenli bir şekilde kullanılmasını sağlayan temel mekanizmadır.

### 🎯 Sonuç Olarak

"Scoping rules" sayfası, Rust'taki bellek yönetiminin kalbinde yatan kapsam kavramına kısa bir giriş niteliğindedir. Buradaki temel prensipleri anlamak, ilerleyen bölümlerde ayrıntılı olarak ele alınan **Sahiplik**, **Ödünç Alma** ve **Yaşam Süreleri** konularını kavramak için bir ön koşuldur.

---

Bu sayfa ve alt başlıklarıyla ilgili aklınıza takılan başka bir nokta olursa lütfen sormaktan çekinmeyin.