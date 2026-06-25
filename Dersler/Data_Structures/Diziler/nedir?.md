Rust'ta **Array (Dizi)**, aynı veri tipindeki elemanları bir arada tutan, **sabit uzunluğa (fixed-length)** sahip ve bellekte **kesintisiz (contiguous)** olarak saklanan temel bir veri koleksiyonudur.

Rust'ta en çok karıştırılan konulardan biri Array ve Vector (`Vec`) farkıdır. Array'lerin en belirgin özelliği, bir kez oluşturulduklarında **boyutlarının asla değişememesidir**.

İşte Rust Array yapısının detaylı anlatımı:

---

### 1. Array Oluşturma ve Tanımlama
Array oluştururken köşeli parantez `[]` kullanılır. Rust tipleri otomatik algılayabilir ancak açıkça belirtmek de mümkündür.

```rust
fn main() {
    // 1. Tip belirtilmeden (Rust otomatik algılar: [i32; 3])
    let sayilar = [1, 2, 3];

    // 2. Tip ve uzunluk açıkça belirtilerek
    // Sözdizimi: [Tür; Uzunluk]
    let sayilar2: [i32; 3] = [1, 2, 3];

    // 3. Aynı değeri tekrarlayarak dizi oluşturma (Çok kullanışlı!)
    // [0, 0, 0, 0, 0] dizisini oluşturur.
    let sifirlar = [0; 5]; 
    
    // 4. Farklı tipler HATA verir (Array homojendir)
    // let hata = [1, "merhaba", true]; // ❌ Derleme hatası!
}
```

---

### 2. Elemanlara Erişim ve Değiştirme
Elemanlara erişmek için `indeks` kullanılır. İndeksler `0`'dan başlar. Diziyi değiştirmek istiyorsanız `mut` (mutable) anahtar kelimesini kullanmalısınız.

```rust
fn main() {
    let mut haftanin_gunleri = ["Pzt", "Sal", "Çar", "Per", "Cum", "Cmt", "Paz"];

    // Okuma
    println!("İlk gün: {}", haftanin_gunleri[0]); // Çıktı: Pzt

    // Değiştirme
    haftanin_gunleri[2] = "Çarşamba"; 

    // ⚠️ SINIR KONTROLÜ (Bounds Checking)
    // Rust, indeksin dizi sınırları içinde olup olmadığını kontrol eder.
    // println!("{}", haftanin_gunleri[10]); // ❌ PANIC! Program çöker.
}
```
*💡 Not: Rust, sınırları hem derleme zamanında (mümkünse) hem de çalışma zamanında (runtime) kontrol eder. Geçersiz bir indeks erişimi programı `panic!` ile durdurur, bu da bellek güvenliğini (memory safety) sağlar.*

---

### 3. Döngülerle Kullanım (Iteration)
Array elemanları üzerinde döngüyle gezmek için `for` döngüsü kullanılır.

```rust
fn main() {
    let sayilar = [10, 20, 30, 40, 50];

    // 1. Sadece Okuma (Borrowing / Ödünç Alma)
    // &sayilar veya &a yazmak, dizinin kopyasını oluşturmadan referansla döngüye sokar.
    for sayi in &sayilar {
        print!("{} ", sayi); // Çıktı: 10 20 30 40 50
    }
    println!();

    // 2. Değiştirme (Mutable Borrowing)
    let mut sayilar = [1, 2, 3];
    for sayi in &mut sayilar {
        *sayi *= 2; // Referansın gösterdiği değeri 2 ile çarp
    }
    println!("Değişmiş dizi: {:?}", sayilar); // Çıktı: [2, 4, 6]
}
```

---

### 4. Array vs Vec (Vector) Karşılaştırması
Bu, Rust'ta en kritik kararlardan biridir.

| Özellik | Array (Dizi) `[T; N]` | Vec (Vektör) `Vec<T>` |
| :--- | :--- | :--- |
| **Uzunluk** | **Sabit** (Compile-time'da bilinmelidir) | **Dinamik** (Runtime'da büyüyüp küçülebilir) |
| **Bellek Yeri** | **Stack** (Yığın) | **Heap** (Öbek) |
| **Performans** | Çok yüksek (Stack'te olduğu ve boyutu bilindiği için) | Yüksek (Heap tahsisi ve dinamik boyut kontrolü nedeniyle Array'den bir tık yavaş) |
| **Boyut Değişimi**| `push()` veya `pop()` **yapılamaz** | `push()`, `pop()`, `remove()` yapılabilir |
| **Kullanım Yeri** | Boyutu kesinlikle bilinen, değişmeyen veriler (örn: RGB renkleri, 3x3 matris, haftanın günleri) | Boyutu sonradan değişecek, kullanıcıdan alınacak veya dosyadan okunacak veriler |

---

### 5. Sık Kullanılan Faydalı Metodlar
Array'ler (ve Slice'lar) üzerinde kullanabileceğiniz birçok hazır metod bulunur:

```rust
fn main() {
    let meyveler = ["Elma", "Armut", "Muz", "Çilek"];

    // Uzunluk
    println!("Uzunluk: {}", meyveler.len()); // 4

    // İçerik kontrolü
    println!("Muz var mı?: {}", meyveler.contains(&"Muz")); // true

    // İlk ve son eleman (Güvenli erişim - Option döndürür)
    println!("İlk: {:?}", meyveler.first()); // Some("Elma")
    println!("Son: {:?}", meyveler.last());  // Some("Çilek")

    // Boş mu kontrolü
    let bos_dizi: [i32; 0] = [];
    println!("Boş mu?: {}", bos_dizi.is_empty()); // true
}
```

---

### 6. Bellek Yapısı ve Performans (Stack vs Heap)
Array'ler **Stack (Yığın)** bellekte tutulur. Bu onları inanılmaz derecede hızlı yapar çünkü:
1. Bellekte **kesintisiz (contiguous)** olarak yan yana dizilirler.
2. CPU Cache (Önbellek) dostudurlar (Spatial locality).
3. Heap tahsisi (memory allocation) gerektirmezler.

⚠️ **Önemli Uyarı (Stack Overflow):**
Stack bellek boyutu sınırlıdır (genellikle birkaç MB). Eğer çok büyük bir Array oluşturursanız (örneğin `let dev_dizi = [0u8; 10_000_000];`), program **Stack Overflow** hatası vererek çöker. 
*Çözüm:* Çok büyük veriler için her zaman `Vec` kullanın veya Array'i Heap'e taşımak için `Box` kullanın (`let buyuk_dizi = Box::new([0; 10_000_000]);`).

---

### 7. İleri Seviye: Array'leri Fonksiyonlara Geçirmek
Bir fonksiyona Array geçerken, eğer fonksiyonun her boyuttaki Array'i kabul etmesini istiyorsanız **Slice (`&[T]`)** kullanmalısınız. Çünkü `[i32; 3]` ile `[i32; 5]` Rust için **farklı tiplerdir**.

```rust
// ❌ Sadece tam olarak 3 elemanlı i32 array'lerini kabul eder.
fn sadece_ucluk_dizi_al(dizi: [i32; 3]) { ... }

// ✅ Her boyuttaki i32 array'lerini (ve vec'leri) kabul eder. (En iyi pratik)
fn esnek_fonksiyon(dilim: &[i32]) {
    println!("Dizinin uzunluğu: {}", dilim.len());
}

fn main() {
    let a = [1, 2, 3];
    let b = [1, 2, 3, 4, 5];
    
    esnek_fonksiyon(&a); // Çalışır
    esnek_fonksiyon(&b); // Çalışır
}
```

---

### Özet ve İpuçları
1. **Sabit Boyut Kuralı:** Eğer veri kümenizin boyutu program çalışırken asla değişmeyecekse (örn. bir oyunun başlangıç koordinatları `[x, y]`, bir RGB rengi `[r, g, b]`), kesinlikle **Array** kullanın.
2. **Hız Odaklılık:** Array'ler Stack'te olduğu için `Vec`'lerden daha hızlıdır. Kritik performans gerektiren küçük döngülerde Array tercih edin.
3. **Kolay Başlatma:** `[değer; adet]` sözdizimi (örn. `[0.0; 100]`) bir diziyi hızlıca sıfırlamak veya başlatmak için harikadır.
4. **Güvenlik:** Rust'ın indeks sınır kontrolü (bounds checking) sayesinde, C/C++'taki gibi "Out of Bounds" (sınır dışı) okuma/yazma yaparak güvenlik açıkları yaratmanız imkansızdır.