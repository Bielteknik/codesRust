# Rust'ta İfadeler (Expressions) — Kapsamlı Ders Anlatımı

Rust By Example'ın "Expressions" sayfası, Rust programlamanın en temel ama en çok yanlış anlaşılan kavramlarından birini ele alıyor: **Statements (Deyimler)** ve **Expressions (İfadeler)** arasındaki fark. Bu ayrımı anlamak, Rust'ın neden diğer dillerden farklı davrandığını kavramak için kritik öneme sahip [[1]].

Hazırsanız, bu konuyu adım adım, bol örnekli bir ders gibi işleyelim.

---

## 📌 Bölüm 1: Temel Kavram — Statement vs Expression

Rust'ta bir program, temelde bir dizi **statement**'tan (deyim) oluşur [[1]]. Ancak Rust'ı diğer dillerden (C, Java, Python) ayıran en önemli özellik şudur:

> **Rust'ta "expression-based" (ifade tabanlı) bir dildir.** Yani neredeyse her şey bir değer döndürür.

### Expression (İfade) Nedir?
Bir **değer üreten** her şey expression'dır.
- `5` → bir expression (değeri: 5)
- `x + y` → bir expression (değeri: toplam)
- `if koşul { 1 } else { 2 }` → bir expression (değeri: 1 veya 2)
- `{ ... }` blokları → bir expression

### Statement (Deyim) Nedir?
Bir **eylem gerçekleştiren** ama değer döndürmeyen (ya da `()` döndüren) yapıdır.
- `let x = 5;` → bir statement
- `fn main() { ... }` → bir statement
- `;` ile biten her şey → statement haline gelir

### Altın Kural: Noktalı Virgül (`;`) Farkı
```rust
let x = 5;     // Bu bir statement (değer döndürmez, sadece bağlar)
5              // Bu bir expression (değeri 5)
5;             // Bu artık bir statement (değer () olur, yani "unit")
```

> 💡 **Akılda kalıcı ipucu:** `;` koyarsan expression, statement'a dönüşür ve değeri `()` (birim/unit) olur.

---

## 📌 Bölüm 2: Rust'taki Statement Türleri

Rust'ta iki ana statement türü vardır [[1]]:

### 1. Değişken Bağlama (Variable Binding)
```rust
let x = 10;
let mut y = 20;
```
Bu, Rust'ta en sık karşılaştığınız statement türüdür. Bir değeri bir isme bağlar.

### 2. İfade Statement'ı (Expression Statement)
Herhangi bir expression'ın sonuna `;` koyarak onu statement'a çevirebilirsiniz:
```rust
x + 5;       // Expression olan `x + 5`, statement'a dönüştü
fonksiyon(); // Function call, sonuna ; gelince statement oldu
```

⚠️ **Dikkat:** Expression'ı statement'a çevirdiğinizde, ürettiği değer kaybolur (çöp toplayıcı tarafından temizlenir ya da yok sayılır).

---

## 📌 Bölüm 3: Block Expressions — Blokların İfade Olarak Kullanımı

İşte Rust'ın en güçlü özelliklerinden biri: **Bloklar (`{ ... }`) aynı zamanda birer expression'dır** [[1]].

Bir blok, içindeki son expression'ın değerini döndürür. Tıpkı bir fonksiyonun `return` kullanmadan değer döndürmesi gibi!

### Temel Örnek:
```rust
let x = {
    let a = 10;
    let b = 20;
    a + b    // ← Dikkat: burada ; YOK! Bu yüzden bu expression'ın değeri döner.
};

println!("{}", x); // Çıktı: 30
```

### Kritik Nokta: Noktalı Virgül Farkı
```rust
let x = {
    let a = 10;
    let b = 20;
    a + b;   // ← DİKKAT: burada ; VAR! Değer () olur.
};

// x'in tipi artık () (unit), yani hiçbir şey değil!
```

> 🎯 **Kural:** Bloğun sonundaki expression'da `;` varsa → dönüş değeri `()` olur. `;` yoksa → o expression'ın değeri bloğun değeri olur.

---

## 📌 Bölüm 4: Pratik Kullanım Senaryoları

### Senaryo 1: Karmaşık Hesaplamaları Gruplama
Bazen bir değeri hesaplarken ara değişkenlere ihtiyaç duyarsınız. Blok expression bunu zarifçe çözer:

```rust
let sonuc = {
    let taban = 5;
    let us = 3;
    let mut carpim = 1;
    for _ in 0..us {
        carpim *= taban;
    }
    carpim  // Blok bu değeri döndürür
};

println!("5^3 = {}", sonuc); // Çıktı: 125
```

### Senaryo 2: If-Else Bir Expression'dır
Rust'ta `if-else` bir statement değil, expression'dır. Bu yüzden doğrudan bir değişkene atanabilir:

```rust
let yas = 18;

let durum = if yas >= 18 {
    "Yetişkin"
} else {
    "Çocuk"
};

println!("{}", durum); // Çıktı: Yetişkin
```

⚠️ **Önemli:** Rust'ta `if-else` expression olduğu için, her iki dal da **aynı tipte** değer döndürmelidir:

```rust
let x = if true {
    5       // i32
} else {
    "hata"  // &str — ❌ HATA! Tipler uyuşmuyor
};
```

### Senaryo 3: Match de Bir Expression'dır
```rust
let sayi = Some(42);

let mesaj = match sayi {
    Some(n) => format!("Sayı: {}", n),
    None    => String::from("Sayı yok"),
};
```

### Senaryo 4: Scope Sınırlama (Isolated Scope)
Blokları, değişkenlerin kapsamını sınırlamak için de kullanabilirsiniz:

```rust
let x = 10;

let y = {
    let x = 20;  // Bu x, dışarıdaki x'ten farklı (gölgeleme/shadowing)
    let z = 30;
    x + z        // 50 döndürür
};

println!("x = {}, y = {}", x, y); // x = 10, y = 50
// z burada erişilemez, scope dışına çıktı
```

---

## 📌 Bölüm 5: Fonksiyonlarda Return Davranışı

Rust'ta fonksiyonlar, blokların expression özelliğini kullanır. Fonksiyonun gövdesi de bir bloktur ve son expression otomatik olarak dönüş değeri olur:

```rust
fn topla(a: i32, b: i32) -> i32 {
    a + b   // ← ; yok! Bu, fonksiyonun dönüş değeri
}

fn topla_Explicit(a: i32, b: i32) -> i32 {
    return a + b;  // ← Alternatif: explicit return
}
```

Ancak `;` koyarsanız:
```rust
fn hatali(a: i32, b: i32) -> i32 {
    a + b;  // ← ; var! Bu () döndürür, ama fonksiyon i32 bekliyor
            // ❌ Derleme hatası: expected `i32`, found `()`
}
```

---

## 📌 Bölüm 6: Sık Yapılan Hatalar ve Çözümleri

### Hata 1: Blok Sonunda Unutulan Noktalı Virgül
```rust
let x = {
    5 + 3;  // Yanlış: () döner
};
// Doğrusu:
let x = {
    5 + 3   // Doğru: 8 döner
};
```

### Hata 2: If-Else'de Tip Uyumsuzluğu
```rust
// ❌ Yanlış
let x = if true { 5 };  // else dalı yok, tipi () olur, uyuşmaz

// ✅ Doğru
let x = if true { 5 } else { 0 };
```

### Hata 3: Let'in Expression Olarak Kullanılması
```rust
// ❌ Yanlış: let bir statement'tır, expression değildir
let y = (let x = 5);  // Bu çalışmaz!

// ✅ Doğru: Blok kullan
let y = {
    let x = 5;
    x
};
```

---

## 📌 Bölüm 7: İleri Seviye — Loop'lar da Expression'dır!

Rust'ta `loop` keyword'ü `break` ile birlikte bir değer döndürebilir:

```rust
let sayac = loop {
    // Bir şeyler yap...
    break 42;  // loop, bu değeri döndürür
};

println!("{}", sayac); // 42
```

Bu özellik, `while` ve `for` döngülerinde yoktur — sadece `loop`'a özeldir.

---

## 📌 Bölüm 8: Özet ve Temel Kurallar

| Kavram | Değer Döndürür mü? | Örnek |
|--------|---------------------|-------|
| Expression | ✅ Evet | `5`, `x + y`, `{ ... }` |
| Statement | ❌ Hayır (`()` döner) | `let x = 5;`, `x + 5;` |
| Blok (son satırda `;` yok) | ✅ Evet | `{ 5 + 3 }` → 8 |
| Blok (son satırda `;` var) | ❌ Hayır | `{ 5 + 3; }` → () |

### 🎓 Unutulmaması Gereken 3 Kural:
1. **`;` = Expression'ı Statement'a çevirir.** Değeri öldürür.
2. **Bloklar expression'dır.** Son satırdaki `;`'siz expression, bloğun değeridir.
3. **Rust expression-based bir dildir.** `if`, `match`, `loop`, bloklar — hepsi değer üretebilir.

---

## 📌 Bölüm 9: Pratik Alıştırma

Aşağıdaki kodu çalıştırmayı deneyin ve çıktılarını tahmin edin:

```rust
fn main() {
    let a = {
        let x = 5;
        let y = 10;
        x * y
    };
    
    let b = {
        let x = 5;
        let y = 10;
        x * y;
    };
    
    let c = if a > 40 { "Büyük" } else { "Küçük" };
    
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {}", c);
}
```

**Cevap:**
- `a = 50` (blok, son expression'ın değerini döndürdü)
- `b = ()` (sonda `;` olduğu için değer unit oldu)
- `c = "Büyük"` (50 > 40 olduğu için)

---

Bu kavramlar, Rust'ın "idiomatic" (doğal) kod yazmanın anahtarıdır. Expression-based yapı, daha kısa, daha okunabilir ve daha güvenli kod yazmanızı sağlar. Fonksiyonel programlama dillerinden (Haskell, OCaml) gelenler bu kavrama yabancı değildir, ancak C/Java kökenli geliştiriciler için başta garip gelebilir. Biraz pratikten sonra, Rust'ın bu özelliğini çok seveceksiniz! 🦀

Başka bir konu sormak isterseniz, buradayım!

# DeepSeek Yorumu --------------------

Rust programlama dili, ifade tabanlı (expression-oriented) bir dildir. Bu, hemen hemen her yazdığınız kod parçasının bir değer döndürdüğü anlamına gelir. Rust By Example’ın **Expressions (İfadeler)** sayfası, bu temel kavramı bloklar, ifadeler ve deyimler bağlamında ayrıntılı olarak ele alır. Gelin bu sayfayı, alt başlıklarıyla birlikte bir ders anlatır gibi derinlemesine inceleyelim.

---

## 1. İfade Tabanlı Dil Olarak Rust

Çoğu programlama dilinde `if`, `while`, `{ ... }` gibi yapılar yalnızca kontrol akışını yönlendiren **deyimlerdir (statement)** ve bir değer üretmezler. Rust'ta ise bu yapıların neredeyse tamamı birer **ifadedir (expression)**. Bir ifade, çalıştırıldığında bir değer üretir; bu değer bir türe sahiptir. Bu özellik sayesinde kod çok daha kısa ve anlamlı hale gelir.

> **Önemli:** İfadeler değer üretir, deyimler ise bir eylem gerçekleştirip değer üretmezler (ya da `()` birim değerini üretirler). Rust'ta `let` bağlaması ve noktalı virgül (`;`) deyimleri oluşturan temel araçlardır.

---

## 2. Bloklar (Blocks)

Sayfanın ilk alt başlığı **Bloklar**dır. Rust’ta süslü parantezler `{ ... }` bir bloğu tanımlar. Blok, içinde birden fazla ifade ve deyim barındırabilen bir **ifadedir**. Bloğun değeri, içindeki son ifadenin değeridir.

```rust
fn main() {
    let x = {
        let y = 5;
        y + 1 // son ifade, noktalı virgül yok!
    };

    println!("x = {}", x); // x = 6
}
```

Yukarıdaki örnekte:

- İçerideki blok bir ifade olarak çalışır.
- `let y = 5;` bir **deyimdir**, `()` değerini üretir ama blok bunu dönmez; bloğun dönüş değerini etkilemez.
- `y + 1` son satırdır ve **noktalı virgül olmadığı için** bu bir ifadedir. Değeri `6`'dır ve bloğun son değeri olur.
- `x` değişkenine bu değer atanır.

Bloklar tek başlarına kullanılabildiği gibi, fonksiyon gövdesi olarak da birer bloktur. Bir fonksiyonun dönüş değeri, gövde bloğunun son ifadesidir (eğer `return` kullanılmamışsa). Bu yüzden Rust’ta çoğu zaman `return` anahtar kelimesine ihtiyaç duyulmaz:

```rust
fn topla(a: i32, b: i32) -> i32 {
    a + b // noktalı virgül yok → ifade, fonksiyonun dönüş değeri
}
```

Eğer son satıra `;` eklerseniz, ifade deyime dönüşür ve fonksiyon `()` döndürmeye çalışır, bu da tip uyuşmazlığı hatasına yol açar.

---

## 3. İfadeler ve Deyimler (Statements and Expressions)

Sayfanın ikinci önemli bölümü (genellikle "Statements and expressions" olarak geçer) bu iki kavram arasındaki ayrımı netleştirir. Rust kodu yazarken bir satırın ifade mi yoksa deyim mi olduğunu anlamak hayatidir.

### 3.1. Deyimler (Statements)

Deyimler bir işlem gerçekleştirir ama **değer döndürmezler**. Rust'ta iki temel deyim türü vardır:

- **`let` ile değişken bağlama:** `let x = 5;` bir deyimdir.
- **İfade + noktalı virgül:** `5 + 3;` aslında `8` değerini hesaplar ama `;` onu bir deyime çevirir, döndürülen değer `()` olur.

Bir deyimi başka bir deyimin içinde değer olarak kullanamazsınız. Örneğin C’de `let x = (y = 5);` gibi bir şey yapılabilir, ancak Rust’ta **`let` bir deyim olduğu için bir ifade bekleyen yerde kullanılamaz**:

```rust
let x = (let y = 6); // HATA: expected expression, found statement (`let`)
```

Aynı şekilde bir deyim başka bir değişkene atanamaz:

```rust
let x = let y = 6; // yine hata
```

Ancak blok kullanarak bu sınırlamayı aşabiliriz, çünkü blok bir ifadedir:

```rust
let x = {
    let y = 6;
    y
}; // x = 6, bu geçerlidir.
```

### 3.2. Noktalı Virgülün Gücü (`;`)

Rust’ta `;` bir ifadeyi bir deyime dönüştürür. Bu dönüşüm, ifadenin ürettiği değeri atar ve yerine `()` birim tipini koyar.

```rust
fn main() {
    let a = 5 + 3;   // a = 8, 5+3 ifade, atama deyimini tamamlayan noktalı virgül dış ifadeyi deyime çeviriyor.
    let b = { 5 + 3; }; // b = (), blok sonunda noktalı virgül var!
}
```

İkinci satırda `b` değişkeni `()` değerini alır, çünkü blok içinde `5 + 3;` deyimi çalışır ve blok bir sonraki ifade olmadığı için `()` döner. Eğer `b`'nin tipini yazdırmaya çalışırsak `()` olduğunu görürüz.

### 3.3. İfade Olarak Kullanılabilen Yapılar

Sayfa, bloklar dışında Rust’taki pek çok kontrol yapısının da birer ifade olduğunu vurgular. İşte bazı önemli örnekler:

- **`if` ifadesi:**
```rust
let sayi = if durum { 5 } else { 6 };
```
Burada `if` bloğunun değeri, seçilen kolun son ifadesidir. Her iki kolun döndürdüğü değer aynı tipte olmalıdır.

- **`loop` ve `break` ile değer döndürme:**
```rust
let sonuc = loop {
    sayac += 1;
    if sayac == 10 {
        break sayac * 2;
    }
}; // sonuc = 20
```
`break` ifadesi bir değerle birlikte kullanılabilir ve bu değer tüm `loop` ifadesinin değeri olur.

- **`match` ifadesi:**
```rust
let x = match deger {
    1 => "bir",
    2 => "iki",
    _ => "diğer",
};
```
Her kol bir ifade döndürür.

Bu özellik, Rust’ta gereksiz değişken atamalarını ve `return`'leri ortadan kaldırarak kodun daha temiz ve fonksiyonel olmasını sağlar.

### 3.4. Makro Çağrıları da Birer İfadedir

Sayfada doğrudan belirtilmese de, Rust’ta `println!`, `vec!` gibi makro çağrıları birer ifadedir. Çoğu `()` döndürürken, `vec!` gibi bazıları somut bir değer döndürür. Makroları da ifade bekleyen yerlerde kullanabiliriz.

---

## 4. Uç Durumlar ve Püf Noktaları

Dersimizin bu kısmında, ifade/deyim ayrımıyla ilgili kafa karıştırabilecek bazı durumları inceleyelim.

### 4.1. Boş Blok ve Boş Döngü

Tamamen boş bir blok `{}` ne döndürür? Cevap `()` birim tipidir. Çünkü içinde hiçbir ifade yoktur ve bloklar varsayılan olarak `()` döner.

```rust
let x = {};
// x: (), tipi ()
```

### 4.2. Atama İfadesi?

Rust’ta `x = 5` tek başına bir **ifade değil, deyimdir** (tam olarak "assignment statement"). Atama işlemi bir değer döndürmez (C’nin aksine). Bu yüzden `let x = (y = 5);` geçersizdir. Ancak `x = 5` ifadesi `()` değerini üretir, bu nedenle `let _ = (x = 5);` teknik olarak çalışır ama anlamsızdır. Pratikte atamanın kendisi bir değer üretmez, o yüzden zincirleme atamalar (`a = b = c`) yapamazsınız.

### 4.3. Dönüş İfadesi (`return`)

`return` anahtar kelimesinin kendisi bir ifade midir? Hayır, `return` bir **akış kontrol deyimidir** ve bulunduğu fonksiyondan erken çıkmayı sağlar. Ancak `return 5;` yazdığınızda `5` ifadesi döndürülür; buradaki `return` ifade değildir. Fonksiyonun dönüş tipi `!` (never) olan durumlar dışında `return` tek başına kullanılmaz. Zaten Rust’ta son ifade kullanımı `return`'e tercih edilir.

### 4.4. `if` İfadesinde Sonlandırıcı Olmaması

Eğer `if` bir ifade olarak kullanılacaksa (yani bir değişkene atanacaksa) tüm kolların değer üretmesi ve `;` ile sonlandırılmaması gerekir. Aşağıdaki kullanım hatalıdır:

```rust
let sonuc = if x > 5 {
    x * 2;   // noktalı virgül deyim yapar, () döner -> tip uyuşmazlığı
} else {
    0
};
```

Bu durumda derleyici, `if` kollarının tiplerinin uyuşmadığını (`()` vs `i32`) söyleyecektir.

---

## 5. Özet ve Tavsiyeler

- Rust’ta süslü parantezli **bloklar birer ifadedir**, değeri içindeki son ifadedir.
- **Deyimler** (`let`, `;` ile biten her şey) `()` değerini üretir ve başka bir ifadenin içinde değer olarak kullanılamazlar.
- Bir ifadenin sonuna `;` koyarak onu deyime dönüştürürsünüz; bu özellikle fonksiyon sonundaki dönüş değerini etkiler.
- Rust’taki hemen her kontrol yapısı (`if`, `loop`, `match`) bir ifadedir; bu sayede kod daha ifadesel ve güçlü olur.
- Derleyici, ifade-deyim ayrımına çok duyarlıdır; hata mesajlarını okurken bir `;` unuttuğunuzu veya fazladan koyduğunuzu sık sık görebilirsiniz.

Bu temel bilgileri kavradığınızda, Rust’ın neden bu kadar tutarlı ve esnek bir dil olduğunu daha iyi anlayacaksınız. Bir sonraki adımda *Flow of Control* (Kontrol Akışı) sayfasına geçerek bu ifade yapılarını daha karmaşık senaryolarda nasıl kullanacağınızı öğrenebilirsiniz.