# 🦀 Rust'ta Akış Kontrolü (Flow Control) — Kapsamlı Ders

Rust By Example'ın "Flow Control" bölümü, Rust'ta programın akışını yönlendiren tüm yapıları kapsar. Bu dersimizde her bir konuyu, **neden** ve **nasıl** kullanıldığını bol örneklerle, adım adım inceleyeceğiz. Hazırsan başlayalım! 🚀

---

## 📚 Bölüm İçeriği Haritası

1. **if / else** — Koşullu dallanma
2. **if let** — Desen eşleştirme odaklı koşul
3. **while** — Koşullu döngü
4. **loop** — Sonsuz döngü ve `break` / `continue`
5. **loop'tan değer döndürme**
6. **İç içe döngüler ve etiketler (nested loops & labels)**
7. **for** — Aralık ve iterator ile döngü
8. **iter / into_iter / iter_mut** — Koleksiyon görünümleri
9. **match** — Desen eşleştirmenin kralı
10. **Destructuring** — Tuple, enum, struct parçalama
11. **Guard** — Ek koşullar
12. **Binding (@)** — Eşleşen değeri bağlama

---

## 1️⃣ `if / else` — Koşullu Dallanma

Rust'ta `if-else` diğer dillere benzer ama **iki önemli farkı** vardır:

- ✅ Boole koşulun etrafında **parantez gerekmez**.
- ✅ `if-else` bir **ifadedir (expression)**, yani bir değer döndürebilir.
- ⚠️ Tüm dallar **aynı tipte** değer döndürmek zorundadır.

### Temel Kullanım

```rust
fn main() {
    let sayi = 42;

    if sayi < 0 {
        println!("{} negatiftir", sayi);
    } else if sayi > 0 {
        println!("{} pozitiftir", sayi);
    } else {
        println!("{} sıfırdır", sayi);
    }
}
```

> 📌 **Dikkat:** `if` bloğu bir **ifade** olduğu için, sonuçta bir değişkene atanabilir. Ama `else` olmazsa Rust bunu `()` (unit) olarak görür ve tip uyumsuzluğu olur.

### `if-else` İfadesi Olarak Kullanım

```rust
fn main() {
    let kosul = true;

    // if-else bir ifade olduğu için doğrudan atama yapabiliriz
    let sonuc = if kosul {
        "koşul doğru"   // ← noktalı virgül YOK, bu bir değer
    } else {
        "koşul yanlış"
    };

    println!("Sonuç: {}", sonuc);
}
```

### ❌ Hatalı Kullanım (Tip Uyumsuzluğu)

```rust
let deger = if true {
    42       // i32
} else {
    "metin"  // &str  → HATA! Tüm dallar aynı tipte olmalı
};
```

> 🎓 **Derste Akılda Kalacak Kural:** Rust'ta `if-else` dalları **her zaman aynı tipi** döndürmelidir. C/C++'taki gibi "son satır değerdir" mantığı yoktur; tüm yollar aynı tipe gelmelidir.

---

## 2️⃣ `if let` — Desen Eşleştirmeli Koşul

`match` kullanmak bazen çok uzun olur. Özellikle sadece **tek bir desene** bakıp diğerlerini görmezden geliyorsak, `if let` çok daha temizdir.

### Problem: `match` ile Option Kullanımı

```rust
fn main() {
    let opsiyonel = Some(7);

    match opsiyonel {
        Some(i) => println!("Uzun bir string ve `{:?}`", i),
        _ => {},   // ← Boş bir dal! Sanki gereksiz yer kaplıyor
    };
}
```

### Çözüm: `if let` ile Temiz Kod

```rust
fn main() {
    let opsiyonel = Some(7);

    if let Some(i) = opsiyonel {
        println!("Değer: {}", i);
    }
    // Some değilse hiçbir şey yapma, sessizce geç
}
```

### `else` Dalı ile Birlikte

```rust
fn main() {
    let harf_veya_sayi = "a";

    if let Some(sayi) = harf_veya_sayi.parse::<i32>().ok() {
        println!("Sayıya eşleşti: {}", sayi);
    } else if harf_veya_sayi.chars().next().unwrap().is_alphabetic() {
        println!("Harfe eşleşti!");
    } else {
        println!("Ne sayı ne de harf!");
    }
}
```

### Enum ile `if let`

```rust
enum Hayvan {
    Kedi,
    Kopek,
    Kus,
}

fn main() {
    let hayvan = Hayvan::Kedi;

    if let Hayvan::Kedi = hayvan {
        println!("Miyav!");
    }
}
```

> 🎯 **Ne Zaman Kullanmalı?** Eğer `match`'te sadece **tek bir durumu** önemsiyor, diğerlerini boş bırakıyorsan → `if let` kullan. Kodun daha okunur olur.

---

## 3️⃣ `while` Döngüsü

`while`, bir koşul doğru olduğu sürece çalışır. Koşul `false` olduğunda döngü biter.

### Klasik FizzBuzz Örneği

```rust
fn main() {
    let mut n = 1;

    while n <= 100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        n += 1;
    }
}
```

> 💡 **İpucu:** Rust'ta `while` döngüsü genellikle `for` tercih edildiği için az kullanılır. Ancak koşul belli değilse (örn. kullanıcı girdisi beklerken) idealdir.

---

## 4️⃣ `loop` — Sonsuz Döngü

Rust'ta `loop` **kesinlikle sonsuz** bir döngü oluşturur. İçinden çıkmak için `break`, bir adımı atlamak için `continue` kullanılır.

### Temel Kullanım

```rust
fn main() {
    let mut sayac = 0;

    loop {
        sayac += 1;
        println!("sayac: {}", sayac);

        if sayac == 5 {
            println!("5'e ulaştık, çıkıyoruz!");
            break;
        }

        if sayac % 2 == 0 {
            continue;   // ← Çift sayılarda aşağıyı atla
        }

        println!("tek sayı: {}", sayac);
    }
}
```

**Çıktı:**
```
sayac: 1
tek sayı: 1
sayac: 2
sayac: 3
tek sayı: 3
sayac: 4
sayac: 5
5'e ulaştık, çıkıyoruz!
```

---

## 5️⃣ `loop`'tan Değer Döndürme

Rust'ta `loop` bir **ifadedir** ve `break` ile birlikte bir değer döndürebilir! Bu, Rust'ın çok güçlü özelliklerinden biridir.

```rust
fn main() {
    let mut sayac = 0;

    let sonuc = loop {
        sayac += 1;

        if sayac == 10 {
            break sayac * 2;   // ← break ile değer döndür!
        }
    };

    println!("Döngüden dönen değer: {}", sonuc);  // 20
}
```

> 🎓 **Derste Akılda Kalacak Kural:** `break ifade;` şeklinde kullanırsan, döngü o ifadeyi **sonuç olarak** döndürür. Bu, `while` döngülerinde yapamayacağın bir şey!

---

## 6️⃣ İç İçe Döngüler ve Etiketler (Labels)

Birden fazla iç içe döngün varsa, `break` ve `continue` hangi döngüyü etkileyecek? Rust buna **etiketler (labels)** ile cevap verir.

```rust
fn main() {
    let mut sayac = 0;

    'dis_dongu: loop {
        println!("Dış döngü başladı");

        let mut ic_sayac = 0;

        loop {
            println!("İç döngü: {}", ic_sayac);

            if ic_sayac == 3 {
                println!("İç döngü bitti");
                break;   // ← Sadece iç döngüyü kırar
            }

            if sayac == 2 {
                println!("Dış döngüyü de kırıyoruz!");
                break 'dis_dongu;   // ← Etiketli break, dış döngüyü kırar
            }

            ic_sayac += 1;
        }

        sayac += 1;
    }

    println!("Program sonlandı. sayac = {}", sayac);
}
```

> 🏷️ **Etiket Kuralları:**
> - Etiketler `'` ile başlar (örn. `'dis_dongu`)
> - Etiketler `loop`, `while`, `for` önüne konur
> - `break 'etiket;` → o etiketli döngüden çıkar
> - `continue 'etiket;` → o etiketli döngünün başına döner

---

## 7️⃣ `for` Döngüsü

`for`, Rust'taki **en yaygın** döngüdür. İki temel kullanım şekli vardır:

### Aralık (Range) ile

```rust
fn main() {
    // 1'den 9'a kadar (10 dahil DEĞİL)
    for i in 1..10 {
        println!("{}", i);
    }

    println!("---");

    // 1'den 10'a kadar (10 dahil)
    for i in 1..=10 {
        println!("{}", i);
    }
}
```

> 📌 `a..b` → `a` dahil, `b` hariç  
> 📌 `a..=b` → `a` ve `b` dahil (inclusive)

### `for` ile FizzBuzz

```rust
fn main() {
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}
```

> 💡 `while` versiyonuna göre çok daha temiz, değil mi? Sayaç değişkeni yok, artış yok, sadece saf mantık.

---

## 8️⃣ `iter` / `into_iter` / `iter_mut` — Üç Farklı Görünüm

Bir koleksiyonu `for` ile dönerken Rust arka planda **iterator** kullanır. Ama **nasıl** döndüğümüz, koleksiyonun akıbetini değiştirir!

### 📖 `iter` — Ödünç Al (Immutable Borrow)

```rust
fn main() {
    let meyveler = vec!["elma", "armut", "muz"];

    for meyve in meyveler.iter() {
        println!("Meyve: {}", meyve);   // &str
    }

    // ✅ meyveler hâlâ kullanılabilir, çünkü sadece ödünç aldık
    println!("İlk meyve: {}", meyveler[0]);
}
```

### 🍽️ `into_iter` — Tüket (Consume / Move)

```rust
fn main() {
    let meyveler = vec![String::from("elma"), String::from("armut")];

    for meyve in meyveler.into_iter() {
        // meyve burada String olarak taşındı (moved)
        println!("Meyve: {}", meyve);
    }

    // ❌ HATA! meyveler artık kullanılamaz, tüketildi
    // println!("{}", meyveler[0]);
}
```

### ✏️ `iter_mut` — Değiştirilebilir Ödünç Al (Mutable Borrow)

```rust
fn main() {
    let mut sayilar = vec![1, 2, 3, 4];

    for sayi in sayilar.iter_mut() {
        *sayi *= 2;   // ← Her elemanı 2 ile çarp
    }

    println!("{:?}", sayilar);   // [2, 4, 6, 8]
}
```

### 📊 Karşılaştırma Tablosu

| Metod       | Verdiği Tip | Koleksiyon Durumu | Kullanım Amacı            |
|-------------|-------------|-------------------|---------------------------|
| `iter()`    | `&T`        | Dokunulmaz        | Sadece okuma              |
| `into_iter()` | `T`       | **Tüketilir**     | Sahiplenip dönüştürme     |
| `iter_mut()`| `&mut T`    | Yerinde değişir   | Elemanları değiştirme     |

> 🎯 **Altın Kural:** Döngüden sonra koleksiyonu kullanmaya devam edecek misin?
> - **Evet** → `iter()` veya `iter_mut()`
> - **Hayır, tüketebilirim** → `into_iter()`

---

## 9️⃣ `match` — Desen Eşleştirmenin Kralı 👑

`match`, Rust'ın **en güçlü** kontrol yapılarından biridir. Bir değeri bir dizi desene karşı karşılaştırır ve eşleşen dalı çalıştırır.

### Temel Kullanım

```rust
fn main() {
    let sayi = 13;

    match sayi {
        1 => println!("Bir!"),
        2 | 3 | 5 | 7 | 11 => println!("Bu bir asal sayı deseni"),
        13..=19 => println!("Teenager!"),   // 13-19 arası
        _ => println!("Normal bir sayı"),   // ← "catch-all" (yakala-hepsini)
    }
}
```

### ⚠️ Kritik Kural: **Exhaustive (Kapsayıcı) Olmak Zorunda**

Rust, `match`'in **tüm olası durumları** kapsamasını zorunlu kılar. Unutursan derleyici hata verir.

```rust
let x = 5;
match x {
    1 => println!("bir"),
    2 => println!("iki"),
    // ❌ HATA: 3, 4, 5... için dal yok!
}
```

Çözüm: `_` ile "geriye kalan her şey" dalı ekle.

---

## 🔟 `match` ile Destructuring (Parçalama)

`match`'in asıl gücü, **karmaşık verileri parçalayarak** eşleştirmesidir.

### 📦 Tuple Eşleştirme

```rust
fn main() {
    let koordinat = (3, 5);

    match koordinat {
        (0, 0) => println!("Başlangıç noktası"),
        (x, 0) => println!("X ekseni üzerinde, x = {}", x),
        (0, y) => println!("Y ekseni üzerinde, y = {}", y),
        (x, y) => println!("({}, {}) noktasında", x, y),
    }
}
```

### 🏷️ Enum Eşleştirme

```rust
enum Mesaj {
    Cikis,
    Metin(String),
    Renk(u8, u8, u8),
    Koordinat { x: i32, y: i32 },
}

fn main() {
    let msg = Mesaj::Renk(255, 128, 0);

    match msg {
        Mesaj::Cikis => println!("Çıkış yapılıyor"),
        Mesaj::Metin(metin) => println!("Metin: {}", metin),
        Mesaj::Renk(r, g, b) => println!("RGB: ({}, {}, {})", r, g, b),
        Mesaj::Koordinat { x, y } => println!("Konum: ({}, {})", x, y),
    }
}
```

### 📚 Struct Eşleştirme

```rust
struct Nokta {
    x: i32,
    y: i32,
}

fn main() {
    let p = Nokta { x: 3, y: 7 };

    match p {
        Nokta { x: 0, y: 0 } => println!("Orijinde"),
        Nokta { x, y: 0 } => println!("X ekseni üzerinde, x = {}", x),
        Nokta { x: 0, y } => println!("Y ekseni üzerinde, y = {}", y),
        Nokta { x, y } => println!("({}, {})", x, y),
    }
}
```

### 🔗 Referans ve Pointer Eşleştirme

```rust
fn main() {
    let sayi = Some(Box::new(42));

    match sayi {
        Some(Box::new(n)) if n > 10 => println!("Büyük sayı: {}", n),
        Some(ref mut n) => {
            *n += 1;
            println!("Değiştirildi: {}", n);
        }
        None => println!("Hiçbir şey yok"),
    }
}
```

> 🎓 **İpucu:** `ref` ve `ref mut` desenleri, değeri **taşımadan** ödünç almanı sağlar.

---

## 1️⃣1️⃣ `match` Guard — Ek Koşullar

Guard, bir desen eşleştikten **sonra** ek bir koşul kontrol etmeni sağlar. `if` ile yazılır.

```rust
fn main() {
    let sayi = 4;

    match sayi {
        x if x < 0 => println!("Negatif sayı"),
        x if x % 2 == 0 => println!("{} çifttir", x),
        x => println!("{} tektir", x),
    }
}
```

### Enum ile Guard

```rust
enum Sicaklik {
    Celsius(i32),
    Fahrenheit(i32),
}

fn main() {
    let sicaklik = Sicaklik::Celsius(35);

    match sicaklik {
        Sicaklik::Celsius(d) if d > 30 => println!("Çok sıcak: {}°C", d),
        Sicaklik::Celsius(d) if d < 0 => println!("Dondurucu: {}°C", d),
        Sicaklik::Celsius(d) => println!("Normal: {}°C", d),
        Sicaklik::Fahrenheit(f) => println!("Fahrenheit: {}°F", f),
    }
}
```

> ⚡ **Guard'ın Gücü:** Desen eşleştirmenin ötesinde, **dinamik koşullar** eklemene izin verir.

---

## 1️⃣2️⃣ `@` (Binding) — Eşleşen Değeri Yakala

`@` operatörü, bir desene bağlanırken **aynı zamanda tüm değeri de bir değişkene bağlamanı** sağlar.

```rust
fn main() {
    let msg = Mesaj::Merhaba { id: 5, icerik: String::from("selam") };

    match msg {
        // Hem tüm msg'yi yakala, hem de id'yi parçala
        m @ Mesaj::Merhaba { id: 3..=7, .. } => {
            println!("Orta aralıkta bir Merhaba mesajı: {:?}", m);
        }
        Mesaj::Merhaba { id, .. } => println!("Farklı id: {}", id),
        _ => println!("Başka bir mesaj"),
    }
}
```

### Basit Örnek

```rust
fn main() {
    let sayi = Some(42);

    match sayi {
        Some(n @ 1..=100) => println!("1-100 arası bir sayı: {}", n),
        Some(n) => println!("Başka sayı: {}", n),
        None => println!("Yok"),
    }
}
```

> 🎯 **Ne Zaman Kullanılır?** Hem **desen eşleştirme** hem de **tüm değere erişim** gerektiğinde. Özellikle hata mesajlarında veya loglamada çok işe yarar.

---

## 🎁 Bonus: `let else` (Rust 1.65+)

`if let`'in tersi gibi düşünülebilir. Desen eşleşmezse `else` bloğu çalışır ve **zorunlu olarak** `return`, `break` veya `panic!` içermelidir.

```rust
fn islem_degerini_al(json: &str) -> Option<i32> {
    let deger: serde_json::Value = serde_json::from_str(json).ok()?;
    
    let sayi = match deger.get("sayi") {
        Some(v) => v,
        None => return None,
    };
    
    // let else ile daha temiz:
    let Some(sayi) = deger.get("sayi") else {
        return None;
    };
    
    Some(sayi.as_i64()? as i32)
}
```

---

## 📝 Özet Tablosu

| Yapı       | Ne Zaman Kullanılır?                                | Örnek                        |
|------------|-----------------------------------------------------|------------------------------|
| `if/else`  | Basit boole koşulları                               | `if x > 5 { ... }`           |
| `if let`   | Tek bir desen önemli, diğerleri önemsiz             | `if let Some(x) = opsiyonel` |
| `while`    | Koşul değişken, kaç iterasyon belli değil           | `while !bitir { ... }`       |
| `loop`     | Sonsuz döngü, `break` ile çıkış                     | `loop { if kosul { break; } }`|
| `for`      | Koleksiyon/aralık üzerinde yineleme                 | `for i in 1..=10 { ... }`    |
| `match`    | Birden fazla desen, kapsayıcı eşleştirme            | `match x { 1 => ..., _ => ... }`|
| Guard      | Desen + ek koşul                                    | `x if x > 5 => ...`          |
| `@`        | Deseni yakala + tüm değeri de tut                   | `n @ 1..=100`                |

---

## 🧠 Dersten Çıkarılacak 5 Altın Kural

1. ✅ **Rust'ta `if-else` bir ifadedir** — değer döndürebilir, tüm dallar aynı tipte olmalı.
2. ✅ **`match` kapsayıcı olmak zorundadır** — `_` ile "geriye kalan her şey" dalı ekle.
3. ✅ **`loop` değer döndürebilir** — `break deger;` ile döngüden sonuç al.
4. ✅ **`iter` / `into_iter` / `iter_mut`** farkını bilmek, ownership hatalarını önler.
5. ✅ **Etiketler (`'etiket`)** iç içe döngülerde hangi döngüyü kıracağını netleştirir.

---

## 🚀 Sıradaki Adım

Bu konuları pekiştirmek için:
- Kendi `enum` tipini tanımla ve `match` ile işle
- `loop` ile bir "retry" mekanizması kur (3 denemeden sonra hata ver)
- Bir vektör üzerinde `iter_mut` ile tüm elemanları güncelle

Rust'ın akış kontrolü, dilin **güvenlik** ve **ifade gücünü** bir arada sunduğu en güzel alanlardan biridir. Bol kodlamalı günler! 🦀✨

# DeepSeek Yorumu ------------------

# Rust Akış Kontrolü (Flow Control) Dersi

Merhaba! Bu derste Rust programlama dilindeki akış kontrol mekanizmalarını en ince ayrıntısına kadar öğreneceğiz. Rust-By-Example'daki Flow Control sayfasını adım adım işleyeceğiz. Hazırsanız başlayalım.

---

## 1. if/else - Koşullu Dallanma

`if/else` yapısı, programın akışını belirli bir koşula göre yönlendirmemizi sağlar. Rust'ta `if/else`'in diğer dillerden en büyük farkı, bir **ifade (expression)** olmasıdır. Yani bir değer döndürebilir.

### Temel Sözdizimi

```rust
if koşul {
    // koşul true ise çalışır
} else if başka_koşul {
    // diğer koşul true ise çalışır
} else {
    // hiçbir koşul true değilse çalışır
}
```

**Önemli not:** Rust'ta koşul parantez içine alınmak zorunda değildir (isteğe bağlı), ama süslü parantezler `{}` zorunludur. Ayrıca koşul mutlaka `bool` tipinde olmalıdır. JavaScript veya C'deki gibi sayıları veya boş olmayan değerleri `true` kabul etmez.

```rust
let sayi = 5;

if sayi > 0 {
    println!("Sayı pozitiftir");
} else if sayi < 0 {
    println!("Sayı negatiftir");
} else {
    println!("Sayı sıfırdır");
}
```

### if/else Bir İfadedir (Expression)

Bu özellik Rust'ı çok güçlü kılar. `if/else` bloklarının son satırına noktalı virgül koymazsak, o blok bir değer döndürür. Bu değeri bir değişkene atayabiliriz.

```rust
let sayi = 5;
let mesaj = if sayi > 0 {
    "pozitif"
} else if sayi < 0 {
    "negatif"
} else {
    "sıfır"
}; // Burada noktalı virgül let ifadesini sonlandırır

println!("Sayı {}'dır", mesaj); // Sayı pozitif'dir
```

**Dikkat edilmesi gereken kritik nokta:** Tüm bloklar aynı tipte değer döndürmelidir. Aşağıdaki kod derlenmez:

```rust
let sayi = 5;
let sonuc = if sayi > 0 {
    42          // i32
} else {
    "merhaba"   // &str - HATA! Türler uyuşmuyor
};
```

Derleyici şöyle bir hata verir: `if and else have incompatible types`

### if let ile Desen Eşleştirme (Ön Bakış)

`if let`, belirli bir deseni eşleştirmenin kısa yoludur. Şimdilik kısaca değinelim, detaylıca `match` ile birlikte işleyeceğiz.

```rust
let belki_sayi = Some(5);

if let Some(deger) = belki_sayi {
    println!("Değer: {}", deger);
} else {
    println!("Değer yok");
}
```

---

## 2. loop - Sonsuz Döngü

`loop` anahtar kelimesi, açıkça `break` ile durdurulana kadar sonsuza kadar çalışan bir döngü oluşturur. Rust'ta `loop` da bir ifadedir ve `break` ile değer döndürebilir!

### Temel Kullanım

```rust
let mut sayac = 0;

loop {
    sayac += 1;
    println!("Sayaç: {}", sayac);
    
    if sayac >= 5 {
        break; // Döngüden çık
    }
}
```

### loop ile Değer Döndürme

`break` anahtar kelimesinden sonra bir değer belirterek, döngünün sonucunu döndürebiliriz. Bu, bir hesaplamayı döngü içinde yapıp sonucu dışarı aktarmak için harika bir yoldur.

```rust
let mut sayac = 0;

let sonuc = loop {
    sayac += 1;
    
    if sayac == 10 {
        break sayac * 2; // Döngüden çık ve 20 değerini döndür
    }
};

println!("Sonuç: {}", sonuc); // Sonuç: 20
```

`break` ile değer döndürürken noktalı virgül kullanmadığımıza dikkat edin; bu bir ifade olduğu için.

### İç İçe Döngüler ve Etiketler (Labels)

İç içe `loop`'larda, hangi döngüden çıkmak istediğimizi belirtmek için **etiket** kullanabiliriz. Etiket adının başına tek tırnak `'` konur.

```rust
let mut i = 0;
'distaki: loop {
    println!("Dış döngü: {}", i);
    i += 1;
    
    let mut j = 0;
    'icteki: loop {
        println!("  İç döngü: {}", j);
        j += 1;
        
        if i + j >= 5 {
            break 'distaki; // Direkt dış döngüden çık!
        }
        if j >= 2 {
            break 'icteki; // Sadece iç döngüden çık
        }
    }
}
```

Bu örnekte:
- `break 'distaki` hem iç hem dış döngüyü sonlandırır.
- `break 'icteki` sadece iç döngüden çıkar, dış döngü devam eder.
- Etiketsiz `break` sadece en içteki döngüden çıkar.

Ayrıca `continue` da etiketle kullanılabilir, o zaman belirtilen etiketli döngünün bir sonraki iterasyonuna atlar.

---

## 3. while - Koşullu Döngü

`while` döngüsü, belirtilen koşul `true` olduğu sürece çalışır. `loop`'un aksine, bir koşula bağlıdır ve koşul `false` olduğunda otomatik sonlanır.

### Temel Kullanım

```rust
let mut sayi = 3;

while sayi > 0 {
    println!("{}!", sayi);
    sayi -= 1;
}
println!("Fırlat!");

// Çıktı:
// 3!
// 2!
// 1!
// Fırlat!
```

### FizzBuzz Örneği

Klasik FizzBuzz problemini `while` ile çözelim:

```rust
let mut sayi = 1;

while sayi <= 20 {
    if sayi % 15 == 0 {
        println!("FizzBuzz");
    } else if sayi % 3 == 0 {
        println!("Fizz");
    } else if sayi % 5 == 0 {
        println!("Buzz");
    } else {
        println!("{}", sayi);
    }
    sayi += 1;
}
```

**Not:** `while` döngüsü `loop` gibi `break` ile değer döndüremez. Eğer döngüden değer döndürmeniz gerekiyorsa `loop` kullanmalısınız.

### while let ile Desen Eşleştirme

`if let`'in döngü versiyonu olan `while let`, bir desen eşleştiği sürece döngüyü çalıştırır. Özellikle iterator'ler ve `Option` tipleriyle çok kullanışlıdır.

```rust
let mut yigin = vec![1, 2, 3];

while let Some(ust) = yigin.pop() {
    println!("Üst eleman: {}", ust);
}

// Çıktı:
// Üst eleman: 3
// Üst eleman: 2
// Üst eleman: 1
```

Bu kod, `yigin.pop()` her çağrıldığında `Some(deger)` döndürdüğü sürece çalışır. Vektör boşalıp `None` döndüğünde döngü sonlanır.

---

## 4. for - Yineleyici (Iterator) Döngüsü

Rust'ta `for` döngüsü, bir **iterator**'ün elemanları üzerinde gezinmek için kullanılır. C/C++'daki gibi üç parçalı `for (;;)` sözdizimi yoktur; onun yerine modern, güvenli bir yaklaşım sunulur.

### Temel Kullanım: Aralıklar (Ranges)

En yaygın kullanım, sayı aralıklarıyla çalışmaktır:

```rust
// 0'dan 4'e kadar (4 dahil değil)
for i in 0..5 {
    println!("i = {}", i);
}
// Çıktı: i = 0, i = 1, i = 2, i = 3, i = 4

// 0'dan 5'e kadar (5 dahil)
for i in 0..=5 {
    println!("i = {}", i);
}
// Çıktı: i = 0, i = 1, ..., i = 5
```

Aralık sözdizimleri:
- `a..b`: a'dan b'ye (b hariç) - yarı açık aralık
- `a..=b`: a'dan b'ye (b dahil) - kapalı aralık

### Koleksiyonlarla Kullanım

`for` döngüsü iterator'lerle çalıştığı için vektörler, diziler ve diğer koleksiyonlarla doğal olarak kullanılabilir:

```rust
let meyveler = ["elma", "armut", "muz"];

// Değerleri ödünç alma (referans)
for meyve in &meyveler {
    println!("{}", meyve);
}

// İndis ve değer birlikte
for (indis, meyve) in meyveler.iter().enumerate() {
    println!("{}. meyve: {}", indis + 1, meyve);
}

// Değiştirilebilir referans
let mut sayilar = vec![1, 2, 3];
for sayi in &mut sayilar {
    *sayi *= 2; // Her elemanı iki katına çıkar
}
println!("{:?}", sayilar); // [2, 4, 6]
```

**Önemli:** Bir koleksiyonu `for` döngüsünde doğrudan kullanırsanız (örn. `for x in sayilar`), **sahiplik** (ownership) döngüye geçer ve koleksiyon tüketilir (consumed). Sonrasında kullanamazsınız. Bu yüzden genellikle `&` ile ödünç alırız.

### İterator Metodlarıyla Güçlü Kombinasyonlar

Rust'ın iterator'leri çok zengindir. `for` döngüsünü iterator adaptörleriyle birleştirerek temiz ve etkili kod yazabilirsiniz:

```rust
// FizzBuzz'ın modern yazımı
for sayi in 1..=20 {
    match (sayi % 3, sayi % 5) {
        (0, 0) => println!("FizzBuzz"),
        (0, _) => println!("Fizz"),
        (_, 0) => println!("Buzz"),
        _ => println!("{}", sayi),
    }
}
```

---

## 5. match - Desen Eşleştirme (Pattern Matching)

`match`, Rust'ın en güçlü özelliklerinden biridir. Bir değeri bir dizi desenle karşılaştırır ve eşleşen ilk desenin kod bloğunu çalıştırır. `match` de bir ifadedir, yani değer döndürür.

### Temel Sözdizimi

```rust
match deger {
    desen1 => {
        // kod
        sonuc_ifadesi
    },
    desen2 => ifade, // Tek satırsa süslü parantez opsiyonel
    _ => varsayilan_sonuc, // "catch-all" deseni
}
```

### Basit Bir Örnek

```rust
let sayi = 3;

match sayi {
    1 => println!("Bir"),
    2 => println!("İki"),
    3 => println!("Üç"),
    4 | 5 => println!("Dört veya Beş"), // Birden fazla desen | ile birleştirilir
    _ => println!("Diğer"), // Tüm diğer durumlar
}
```

### match İfadesiyle Değer Döndürme

```rust
let sayi = 4;
let yazi = match sayi {
    1 => "bir",
    2 => "iki",
    3 => "üç",
    _ => "bilinmiyor",
};
println!("{} sayısının yazılışı: {}", sayi, yazi);
```

### Desenlerle Değişken Bağlama (Destructuring)

`match`'in asıl gücü, karmaşık veri yapılarını parçalayabilmesidir.

#### Enum'ları Parçalama

```rust
enum ParaBirimi {
    TL,
    Dolar,
    Euro,
}

enum Fiyat {
    Miktar(f64, ParaBirimi), // Tuple varyant
    Bilinmiyor,              // Birim varyant
}

let fiyat = Fiyat::Miktar(19.99, ParaBirimi::TL);

match fiyat {
    Fiyat::Miktar(miktar, ParaBirimi::TL) => {
        println!("{} TL", miktar);
    },
    Fiyat::Miktar(miktar, birim) => {
        println!("{} yabancı para birimi", miktar);
    },
    Fiyat::Bilinmiyor => {
        println!("Fiyat bilinmiyor");
    },
}
```

#### Tuple'ları Parçalama

```rust
let uclu = (0, 0.5, "merhaba");

match uclu {
    (0, y, z) => println!("İlk 0, y = {}, z = {}", y, z),
    (x, 0.5, _) => println!("Ortanca 0.5, x = {}", x),
    _ => println!("Diğer durum"),
}
```

#### Yapıları (Struct) Parçalama

```rust
struct Nokta {
    x: i32,
    y: i32,
}

let nokta = Nokta { x: 10, y: 20 };

match nokta {
    Nokta { x, y: 0 } => println!("x ekseninde, x = {}", x),
    Nokta { x: 0, y } => println!("y ekseninde, y = {}", y),
    Nokta { x, y } => println!("Düzlemde bir nokta: ({}, {})", x, y),
}
```

#### Referansları Eşleştirme

```rust
let sayi = 42;
let referans = &sayi;

match referans {
    &deger => println!("Referansın gösterdiği değer: {}", deger),
}

// Veya otomatik dereference ile:
match *referans {
    deger => println!("Değer: {}", deger),
}
```

### Koruyucu Koşullar (Match Guards)

Desenlere ek koşullar eklemek için `if` kullanabiliriz:

```rust
let cift = Some(4);

match cift {
    Some(x) if x % 2 == 0 => println!("Çift sayı: {}", x),
    Some(x) => println!("Tek sayı: {}", x),
    None => println!("Sayı yok"),
}
```

### match'in Tüm Durumları Kapsama Zorunluluğu (Exhaustiveness)

Rust derleyicisi, `match` ifadelerinin tüm olası durumları kapsamasını zorunlu kılar. Bu, unutulan durumlardan kaynaklanan hataları önler.

```rust
let sayi = 5;
match sayi { // Derlenmez!
    1 => println!("bir"),
    2 => println!("iki"),
    // _ => eklenmeli
}
```

Bu kod derlenmez, çünkü `i32` tipinin tüm değerlerini kapsamaz. `_` (alt çizgi) deseni "diğer tüm durumlar" anlamına gelir ve kapsamlılığı sağlar.

---

## 6. if let - Kısa Desen Eşleştirme

Bazen `match` kullanmak gereksiz uzun olabilir, özellikle sadece tek bir desenle ilgileniyorsak. `if let` imdadımıza yetişir!

### Sözdizimi

```rust
if let desen = ifade {
    // eşleşirse çalışır
} else {
    // eşleşmezse çalışır
}
```

### match ile Karşılaştırma

```rust
// match kullanarak:
let belki_sayi = Some(7);
match belki_sayi {
    Some(x) => println!("Sayı: {}", x),
    _ => (), // Hiçbir şey yapma
}

// if let kullanarak (daha temiz):
if let Some(x) = belki_sayi {
    println!("Sayı: {}", x);
}
```

### else if let ile Zincirleme

```rust
enum Mesaj {
    Metin(String),
    Sayi(i32),
    Bos,
}

let mesaj = Mesaj::Metin(String::from("Merhaba"));

if let Mesaj::Metin(icerik) = mesaj {
    println!("Metin mesajı: {}", icerik);
} else if let Mesaj::Sayi(n) = mesaj {
    println!("Sayı mesajı: {}", n);
} else {
    println!("Boş mesaj");
}
```

### if let ile Karmaşık Desenler

```rust
let nokta = Some((3, 4));

if let Some((x, y)) = nokta {
    println!("Koordinatlar: ({}, {})", x, y);
}

// Match guard ile:
if let Some((x, y)) = nokta if x == y {
    println!("x ve y eşit: {}", x);
}
```

---

## 7. while let - Koşullu Döngü Desen Eşleştirme

`while let`, bir desen eşleştiği sürece döngüyü çalıştıran bir yapıdır.

### Temel Kullanım

```rust
let mut istekler = vec![1, 2, 3, 4, 5];

while let Some(istek) = istekler.pop() {
    println!("İstek işleniyor: {}", istek);
    // istekler boşalana kadar çalışır
}
println!("Tüm istekler işlendi.");
```

### Kanal (Channel) ile Kullanım

Gerçek hayatta sık kullanılan bir örnek:

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

let (gonderici, alici) = mpsc::channel();

thread::spawn(move || {
    let mesajlar = vec!["merhaba", "dunya", "rust"];
    for mesaj in mesajlar {
        gonderici.send(mesaj).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

// Mesajlar geldiği sürece işle
while let Ok(mesaj) = alici.recv() {
    println!("Alınan mesaj: {}", mesaj);
}
```

### while let ile İç İçe Desen Eşleştirme

```rust
enum Islem {
    Ekle(i32, i32),
    Cikar(i32, i32),
}

let islemler = vec![
    Islem::Ekle(5, 3),
    Islem::Cikar(10, 4),
    Islem::Ekle(1, 1),
];

let mut iter = islemler.into_iter();

while let Some(islem) = iter.next() {
    match islem {
        Islem::Ekle(a, b) => println!("{} + {} = {}", a, b, a + b),
        Islem::Cikar(a, b) => println!("{} - {} = {}", a, b, a - b),
    }
}
```

---

## Önemli Noktaların Özeti

| Yapı | Ne Zaman Kullanılır? | İfade mi? | Değer Döndürür mü? |
|------|---------------------|-----------|-------------------|
| `if/else` | Koşullu dallanma | Evet | Evet (tüm bloklar aynı tip) |
| `loop` | Sonsuz döngü (break ile çıkılır) | Evet | Evet (break ile) |
| `while` | Koşullu döngü | Evet | Hayır |
| `for` | Iterator üzerinde gezinme | Evet | Hayır (ama collect ile dönüşebilir) |
| `match` | Kapsamlı desen eşleştirme | Evet | Evet |
| `if let` | Tek desen eşleştirme | Evet | Evet |
| `while let` | Desen eşleştikçe döngü | Evet | Hayır |

## Rust'ın Akış Kontrol Felsefesi

1. **Her şey bir ifadedir:** `if`, `loop`, `match` blokları değer döndürebilir. Bu, Rust'ın işlevsel (functional) programlamadan etkilendiğini gösterir.

2. **Güvenlik ön planda:** `match`'in tüm durumları kapsama zorunluluğu (exhaustiveness checking) ve koşulların `bool` olma zorunluluğu, çalışma zamanı hatalarını derleme zamanında yakalar.

3. **Sahiplik ve ödünç alma ile entegrasyon:** Döngülerde koleksiyonların sahipliğine dikkat edilmesi, bellek güvenliğinin akış kontrolüne de yansımasıdır.

4. **Desen eşleştirmenin gücü:** `match`, `if let`, `while let` ile karmaşık veri yapıları zarifçe parçalanabilir. Bu, Rust kodunun okunabilirliğini ve güvenliğini artırır.

5. **Iterator merkezli döngüler:** C tarzı `for` döngüleri yerine iterator tabanlı `for`, daha güvenli ve soyut bir yaklaşım sunar. Hatalara açık indis yönetimi ortadan kalkar.

---

Bu dersimizde Rust'ın akış kontrol mekanizmalarını kapsamlı bir şekilde inceledik. Artık `if/else`, `loop`, `while`, `for`, `match`, `if let` ve `while let` yapılarını etkili bir şekilde kullanabilirsiniz. Bir sonraki adımda bu yapıları kendi projelerinizde bol bol pratik yaparak pekiştirmenizi öneririm. Özellikle `match` ve iterator'leri sık kullanmak, Rust'ın gücünü daha iyi hissetmenizi sağlayacaktır.

Sorularınız varsa çekinmeden sorabilirsiniz!