 'ta **`match`**, bir değeri birden fazla **pattern (desen)** ile karşılaştırarak eşleşen ilk desene göre kod çalıştıran güçlü bir **kontrol akışı (control flow)** yapısıdır. 

Diğer dillerdeki `switch/case` ifadelerine benzer ancak **çok daha güçlüdür**.  'ın en sevilen özelliklerinden biridir çünkü **exhaustiveness checking** (tüm durumları kapsama kontrolü) yapar ve derleyici, unuttuğunuz bir durum varsa hata verir.

İşte   `match` ifadesinin detaylı anlatımı:

---

## 1. Temel Match Sözdizimi

``` 
fn main() {
    let sayi = 3;

    match sayi {
        1 => println!("Bir"),
        2 => println!("İki"),
        3 => println!("Üç"),
        _ => println!("Başka bir sayı"), // _ : wildcard, geriye kalan her şey
    }
}
```

**Yapı:**
``` 
match DEGER {
    DESEN1 => KOD_BLOKU1,
    DESEN2 => KOD_BLOKU2,
    ...
    _ => SON_CARE, // Varsa (zorunlu değil ama genelde olur)
}
```

---

## 2. Exhaustiveness (Tüm Durumları Kapsama) -  'ın Süper Gücü

 'ta `match` ifadesi **tüm olası durumları kapsamak zorundadır**. Eğer bir durumu unutursanız, derleyici hata verir. Bu, birçok hatayı daha kod yazarken yakalamanızı sağlar.

``` 
fn main() {
    let sayi = 5;

    // ❌ HATA! Tüm durumlar kapsanmadı.
    // Derleyici: "match may not be exhaustive"
    match sayi {
        1 => println!("Bir"),
        2 => println!("İki"),
        // 3, 4, 5 ve diğerleri unutuldu!
    }

    // ✅ Doğru: _ ile tüm geriye kalan durumlar kapsandı
    match sayi {
        1 => println!("Bir"),
        2 => println!("İki"),
        _ => println!("Diğer"),
    }
}
```

---

## 3. Pattern Türleri

### A. Literal (Sabit Değer) Matching

``` 
fn main() {
    let harf = 'a';

    match harf {
        'a' | 'e' | 'i' | 'o' | 'u' => println!("Ünlü harf"),
        'b'..='z' => println!("Ünsüz harf"), // Range pattern
        _ => println!("Harf değil"),
    }
}
```

### B. Değişken Bağlama (Variable Binding)

Eşleşen değeri bir değişkene atayabilirsiniz.

``` 
fn main() {
    let x = Some(5);

    match x {
        Some(50) => println!("Tam 50"),
        Some(sayi) => println!("Bir sayı var: {}", sayi), // sayi değişkenine bağlandı
        None => println!("Değer yok"),
    }
}
```

### C. Guard (Koşul Ekleme)

Desene ek koşul eklemek için `if` kullanılır.

``` 
fn main() {
    let sayi = 4;

    match sayi {
        x if x < 0 => println!("{} negatiftir", x),
        x if x % 2 == 0 => println!("{} çifttir", x),
        x => println!("{} tektir", x),
    }
}
```

### D. Çoklu Pattern (| Operatörü)

Birden fazla deseni aynı kod bloğuna bağlayabilirsiniz.

``` 
fn main() {
    let x = 1;

    match x {
        1 | 2 => println!("Bir veya iki"),
        3 => println!("Üç"),
        _ => println!("Başka"),
    }
}
```

### E. Range Pattern (Aralık)

`..=` operatörü ile aralık belirtilebilir.

``` 
fn main() {
    let x = 5;

    match x {
        1..=5 => println!("1 ile 5 arası (dahil)"),
        6..=10 => println!("6 ile 10 arası"),
        _ => println!("10'dan büyük"),
    }
}
```

### F. Wildcard (_)

Tüm geriye kalan durumları yakalar. Kullanmayacağınız değerler için de kullanılır.

``` 
fn main() {
    let noktalar = (0, 7);

    match noktalar {
        (0, 0) => println!("Orijin"),
        (_, 0) => println!("X ekseni üzerinde"),
        (0, _) => println!("Y ekseni üzerinde"),
        (x, y) => println!("Diğer nokta: ({}, {})", x, y),
    }
}
```

---

## 4. Destructuring (Yapı Bozma)

`match`, tuple, struct ve enum'ları parçalamak için mükemmeldir.

### A. Tuple Destructuring

``` 
fn main() {
    let koordinat = (3, 5);

    match koordinat {
        (0, y) => println!("Y ekseni üzerinde, y = {}", y),
        (x, 0) => println!("X ekseni üzerinde, x = {}", x),
        (x, y) => println!("Diğer: ({}, {})", x, y),
    }
}
```

### B. Struct Destructuring

``` 
#[derive(Debug)]
struct Kullanici {
    isim: String,
    yas: u8,
}

fn main() {
    let kullanici = Kullanici {
        isim: String::from("Ali"),
        yas: 25,
    };

    match kullanici {
        Kullanici { isim, yas: 18 } => println!("{} yeni yetişkin", isim),
        Kullanici { isim, yas } if yas < 18 => println!("{} çocuk", isim),
        Kullanici { isim, yas } => println!("{} {} yaşında", isim, yas),
    }
}
```

### C. Enum Destructuring (En Önemli Kullanım!)

 'ta `match`'in en güçlü olduğu yer **Enum**'lardır.

``` 
enum Mesaj {
    Cikis,
    Hareket { x: i32, y: i32 },
    Yaz(String),
    Renk(u8, u8, u8),
}

fn isle(mesaj: Mesaj) {
    match mesaj {
        Mesaj::Cikis => println!("Çıkış yapıldı"),
        Mesaj::Hareket { x, y } => println!("Hareket: ({}, {})", x, y),
        Mesaj::Yaz(metin) => println!("Mesaj: {}", metin),
        Mesaj::Renk(r, g, b) => println!("Renk: RGB({}, {}, {})", r, g, b),
    }
}

fn main() {
    isle(Mesaj::Cikis);
    isle(Mesaj::Hareket { x: 10, y: 20 });
    isle(Mesaj::Yaz(String::from("Merhaba")));
    isle(Mesaj::Renk(255, 128, 0));
}
```

---

## 5. Option ve Result ile Kullanım

 'ta `Option` ve `Result` tipleri `match` ile çok sık kullanılır.

### Option ile

``` 
fn bolme(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    let sonuc = bolme(10.0, 2.0);

    match sonuc {
        Some(deger) => println!("Sonuç: {}", deger),
        None => println!("Sıfıra bölme hatası!"),
    }

    // Vec'ten güvenli erişim
    let sayilar = vec![1, 2, 3];
    match sayilar.get(5) {
        Some(sayi) => println!("Sayı: {}", sayi),
        None => println!("İndeks sınır dışı!"),
    }
}
```

### Result ile (Hata Yönetimi)

``` 
use std::fs::File;

fn main() {
    let dosya_sonucu = File::open("merhaba.txt");

    let dosya = match dosya_sonucu {
        Ok(dosya) => dosya,
        Err(hata) => {
            panic!("Dosya açılamadı: {:?}", hata)
        }
    };
}
```

---

## 6. @ (At) Operatörü - Değeri Yakala ve Test Et

`@` operatörü, bir değerin hem desene uyduğunu test eder hem de o değeri bir değişkene bağlar.

``` 
fn main() {
    let mesaj = String::from("Merhaba");

    match mesaj.len() {
        n @ 1..=5 => println!("Kısa mesaj ({} karakter): {}", n, mesaj),
        n @ 6..=10 => println!("Orta mesaj ({} karakter): {}", n, mesaj),
        n => println!("Uzun mesaj ({} karakter)", n),
    }

    // Enum ile @ kullanımı
    enum Robot {
        Sensor(i32),
    }

    let robot = Robot::Sensor(15);

    match robot {
        Robot::Sensor(deger @ 0..=20) => println!("Düşük sensör: {}", deger),
        Robot::Sensor(deger) => println!("Yüksek sensör: {}", deger),
    }
}
```

---

## 7. Match İfadesi Dönüş Değeri Döndürebilir

`match` bir ifade (expression) olduğu için değer döndürebilir.

``` 
fn main() {
    let sayi = 5;

    let sonuc = match sayi {
        1 => "Bir",
        2 | 3 | 5 | 7 => "Asal",
        _ => "Diğer",
    };

    println!("Sonuç: {}", sonuc); // Çıktı: Asal
}
```

---

## 8. if let ve while let (Match'in Kısaltmaları)

Eğer sadece **tek bir durumu** kontrol ediyorsanız, `match` yerine daha kısa olan `if let` kullanabilirsiniz.

### if let

``` 
fn main() {
    let sayi = Some(5);

    // Match ile (uzun)
    match sayi {
        Some(x) => println!("Değer: {}", x),
        _ => (), // Hiçbir şey yapma
    }

    // if let ile (kısa)
    if let Some(x) = sayi {
        println!("Değer: {}", x);
    }

    // else de eklenebilir
    if let Some(x) = sayi {
        println!("Değer: {}", x);
    } else {
        println!("Değer yok");
    }
}
```

### while let

``` 
fn main() {
    let mut stack = vec![1, 2, 3];

    // Stack boşalana kadar pop yap
    while let Some(ust) = stack.pop() {
        println!("{}", ust);
    }
    // Çıktı: 3, 2, 1
}
```

---

## 9. Match vs if/else Karşılaştırması

| Özellik | `match` | `if/else` |
|:---|:---|:---|
| **Pattern Matching** | ✅ Çok güçlü | ❌ Sadece boolean koşullar |
| **Exhaustiveness** | ✅ Derleyici kontrol eder | ❌ Kontrol yok |
| **Enum ile kullanım** | ✅ Mükemmel | ❌ Zor ve çirkin |
| **Okunabilirlik** | Çoklu durumlar için iyi | Basit koşullar için iyi |
| **Performans** | Aynı | Aynı |

**Kural:** Eğer 2-3 basit koşul varsa `if/else`, enum veya çoklu pattern varsa `match` kullanın.

---

## 10. Gerçek Dünya Örnekleri

### A. Basit Hesap Makinesi

``` 
fn hesapla(islem: char, a: f64, b: f64) -> Option<f64> {
    match islem {
        '+' => Some(a + b),
        '-' => Some(a - b),
        '*' => Some(a * b),
        '/' => {
            if b == 0.0 {
                None
            } else {
                Some(a / b)
            }
        }
        _ => None,
    }
}

fn main() {
    match hesapla('+', 10.0, 5.0) {
        Some(sonuc) => println!("Sonuç: {}", sonuc),
        None => println!("Geçersiz işlem"),
    }
}
```

### B. HTTP Status Code İşleme

``` 
enum HttpStatus {
    Ok,
    NotFound,
    InternalServerError,
    Other(u16),
}

fn durum_mesaji(durum: HttpStatus) {
    match durum {
        HttpStatus::Ok => println!("200: Başarılı"),
        HttpStatus::NotFound => println!("404: Bulunamadı"),
        HttpStatus::InternalServerError => println!("500: Sunucu hatası"),
        HttpStatus::Other(kod) => println!("{}: Diğer", kod),
    }
}
```

### C. Nested Pattern Matching

``` 
fn main() {
    let sayilar = (Some(5), None, Some(10));

    match sayilar {
        (Some(x), Some(y), _) => println!("İlk iki değer: {} ve {}", x, y),
        (Some(x), None, Some(z)) => println!("İlk ve son: {} ve {}", x, z),
        (None, _, _) => println!("İlk değer yok"),
        _ => println!("Diğer durumlar"),
    }
}
```

---

## 11. Sık Yapılan Hatalar

### ❌ Hata 1: Tüm durumları kapsamamak

``` 
let x = 5;
match x {
    1 => println!("Bir"),
    // HATA: 2, 3, 4, 5 için durum yok
}
```

### ❌ Hata 2: Son koşulsuz match (Option/Result için)

``` 
let x: Option<i32> = Some(5);
match x {
    Some(5) => println!("Beş"),
    // HATA: None ve diğer Some değerleri için durum yok
}
```

### ❌ Hata 3: Guard'da değişkeni yanlış kullanma

``` 
let x = 5;
match x {
    y if y > 10 => println!("Büyük"),
    y => println!("Küçük: {}", y), // y burada tanımlı
}
```

---

## Özet ve İpuçları

1. **Exhaustiveness  'ın Süper Gücüdür:** `match` tüm durumları kapsamak zorundadır. Bu, birçok hatayı önler.

2. **Enum + Match = Mükemmel İkili:** Enum'ları işlemek için her zaman `match` kullanın.

3. **Wildcard (_) Kullanın:** Tüm geriye kalan durumları yakalamak için `_` kullanın.

4. **if let ile Kısaltın:** Sadece tek bir durumu kontrol ediyorsanız `if let` daha temizdir.

5. **Guard Ekleyin:** Desene ek koşul eklemek için `if` guard kullanabilirsiniz.

6. **Destructuring Güçlüdür:** Tuple, struct ve enum'ları parçalamak için `match` kullanın.

7. **@ Operatörünü Öğrenin:** Hem desene uydurup hem de değişkene bağlamak için `@` kullanın.

8. **Match Bir İfadedir:** `match` değer döndürebilir, bu yüzden değişkene atanabilir.

9. **Nested Pattern Kullanın:** İç içe yapıları tek bir `match` ile işleyebilirsiniz.

10. **Derleyici Arkadaşınızdır:** `match` ile ilgili hatalar alırsanız, derleyicinin mesajlarını dikkatlice okuyun. Size neyi unuttuğunuzu söyler.

`match`,  'ın en zarif ve güçlü özelliklerinden biridir. Doğru kullanıldığında kodunuzu hem güvenli hem de okunabilir yapar. Enum'larla birlikte kullanıldığında ise  'ın tip sisteminin tüm gücünü ortaya çıkarır.