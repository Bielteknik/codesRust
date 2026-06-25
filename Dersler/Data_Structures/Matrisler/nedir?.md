Rust'ta Python'daki (NumPy) veya MATLAB'daki gibi doğrudan dil içine gömülü, birinci sınıf (first-class) bir **Matris** veri tipi **yoktur**. 

Ancak bu, Rust'ta matrislerle çalışamayacağınız anlamına gelmez. Rust'ta matrisleri temsil etmenin ve işlemenin **4 farklı yolu** vardır. Hangi yolu seçeceğiniz, matrisin boyutunun ne zaman belli olduğuna ve performans gereksinimlerinize bağlıdır.

İşte Rust'ta matrislerin detaylı anlatımı:

---

### 1. İç İçe Diziler (Nested Arrays) - `[[T; Sütun]; Satır]`
Eğer matrisinizin boyutları **derleme zamanında (compile-time) kesin olarak belliyse**, standart kütüphanedeki iç içe dizileri kullanırsınız. Bu diziler **Stack (Yığın)** bellekte tutulur ve çok hızlıdırlar.

Sözdizimi: `[[Tür; Sütun_Sayısı]; Satır_Sayısı]`

```rust
fn main() {
    // 2 satırlı ve 3 sütunlu bir matris (Türü: i32)
    let matris: [[i32; 3]; 2] = [
        [1, 2, 3], // 1. Satır
        [4, 5, 6], // 2. Satır
    ];

    // Elemana erişim: matris[satir_indeksi][sutun_indeksi]
    println!("1. Satır, 2. Sütun: {}", matris[0][1]); // Çıktı: 2

    // Tüm matrisi yazdırma
    println!("Matris: {:#?}", matris);
}
```
* **Avantajı:** Bellekte kesintisiz (contiguous) durur, çok hızlıdır, heap tahsisi yapmaz.
* **Dezavantajı:** Boyutu sonradan değiştirilemez, çok büyük matrisler Stack Overflow'a yol açabilir.

---

### 2. İç İçe Vektörler (Nested Vectors) - `Vec<Vec<T>>`
Eğer matrisinizin boyutları **çalışma zamanında (runtime) belli olacaksa** (örneğin kullanıcıdan alınacaksa veya bir dosyadan okunacaksa) iç içe vektörler kullanılır.

```rust
fn main() {
    let satir_sayisi = 2;
    let sutun_sayisi = 3;

    // 0 ile başlatılan 2x3'lük dinamik bir matris
    let mut matris = vec![vec![0; sutun_sayisi]; satir_sayisi];

    // Değer atama
    matris[0][0] = 10;
    matris[1][2] = 99;

    println!("Matris: {:?}", matris);
}
```
* **Avantajı:** Boyutu dinamik olarak değiştirilebilir.
* **Dezavantajı (Çok Önemli):** Bellek dağılımı **sürekli değildir**. Her bir satır (iç vektör) Heap'te ayrı bir yere tahsis edilir. Bu durum CPU Cache (önbellek) verimsizliğine yol açar ve büyük matrislerde performansı ciddi şekilde düşürür.

---

### 3. Tek Boyutlu Vektör ile Simülasyon (Performans Odaklı)
Eğer harici kütüphane kullanmak istemiyor ama iç içe vektörlerin performans sorunundan da kaçınmak istiyorsanız, **tek boyutlu bir `Vec`** kullanıp indekslemeyi matematiksel olarak kendiniz yaparsınız. Bu, profesyonel yazılımcıların sıfırdan matris yazarken kullandığı yöntemdir.

Formül: `indeks = (satir * toplam_sutun) + sutun`

```rust
struct Matris {
    veri: Vec<f64>,
    satir: usize,
    sutun: usize,
}

impl Matris {
    fn yeni(satir: usize, sutun: usize) -> Self {
        Matris {
            veri: vec![0.0; satir * sutun], // Tek bir düz vektör
            satir,
            sutun,
        }
    }

    // Eleman okuma
    fn al(&self, r: usize, c: usize) -> f64 {
        self.veri[r * self.sutun + c]
    }

    // Eleman yazma
    fn yaz(&mut self, r: usize, c: usize, deger: f64) {
        self.veri[r * self.sutun + c] = deger;
    }
}

fn main() {
    let mut m = Matris::yeni(2, 3);
    m.yaz(0, 1, 5.5);
    println!("Değer: {}", m.al(0, 1)); // Çıktı: 5.5
}
```
* **Avantajı:** Bellekte kesintisiz (contiguous) durur, CPU Cache dostudur, en yüksek performansı verir.
* **Dezavantajı:** Matris çarpımı, tersini alma gibi işlemleri manuel kodlamanız gerekir (ki bu zordur).

---

### 4. Harici Kütüphaneler (Crates) - *Gerçek Çözüm*
Rust'ta ciddi matematiksel işlemler (matris çarpımı, invers, özdeğerler, lineer cebir) yapacaksanız, tekerleği yeniden icat etmek yerine ekosistemdeki endüstri standardı kütüphaneleri kullanmalısınız. En popüler ikisi:

#### A. `ndarray` (NumPy benzeri, N-boyutlu diziler)
Daha çok veri bilimi ve genel N-boyutlu matris işlemleri için kullanılır.

```toml
# Cargo.toml
[dependencies]
ndarray = "0.15"
```

```rust
use ndarray::array;

fn main() {
    // 2x3 matris oluşturma
    let matris = array![
        [1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0]
    ];

    // Elemana erişim
    println!("Değer: {}", matris[[0, 1]]); // Çıktı: 2.0

    // Matris çarpımı için transpoze alma
    let transpoze = matris.t();
    println!("Transpoze:\n{}", transpoze);
}
```

#### B. `nalgebra` (Lineer Cebir ve 3D Grafikler odaklı)
Oyun geliştirme, fizik motorları ve katı lineer cebir işlemleri için kullanılır.

```toml
[dependencies]
nalgebra = "0.32"
```

```rust
use nalgebra::{Matrix2, Vector2};

fn main() {
    // 2x2'lik sabit boyutlu matris
    let m = Matrix2::new(1.0, 2.0,
                         3.0, 4.0);

    let v = Vector2::new(1.0, 2.0);

    // Matris vektör çarpımı
    let sonuc = m * v;
    println!("Sonuç: {}", sonuc);
    
    // Matrisin tersini (inverse) alma
    if let Some(ters) = m.try_inverse() {
        println!("Ters Matris:\n{}", ters);
    }
}
```

---

### Karşılaştırma Tablosu

| Yöntem | Tip | Bellek Yapısı | Boyut | Performans | Kullanım Yeri |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **İç İçe Array** | `[[T; N]; M]` | Stack (Sürekli) | Statik (Sabit) | ⭐⭐⭐⭐⭐ | Küçük, boyutu kesin bilinen matrisler (örn: 3D grafiklerde 4x4 dönüşüm matrisleri). |
| **İç İçe Vec** | `Vec<Vec<T>>` | Heap (Parçalı) | Dinamik | ⭐⭐ | Sadece hızlıca prototip çıkarılacaksa, çok nadiren önerilir. |
| **Tek Boyutlu Vec**| `Vec<T>` | Heap (Sürekli) | Dinamik | ⭐⭐⭐⭐⭐ | Kendi matris kütüphanenizi yazıyorsanız. |
| **nalgebra** | `MatrixN`, `DMatrix`| Stack/Heap | Statik/Dinamik | ⭐⭐⭐⭐⭐ | Lineer cebir, oyun geliştirme, fizik simülasyonları. |
| **ndarray** | `Array2<T>` | Heap (Sürekli) | Dinamik | ⭐⭐⭐⭐⭐ | Veri bilimi, makine öğrenmesi, NumPy benzeri işlemler. |

---

### Özet ve İpuçları
1. **Standart Kütüphane ile Sınırlıysanız:** Boyut belliyse `[[T; N]; M]`, dinamikse tek boyutlu `Vec<T>` kullanıp indeks formülü uygulayın. `Vec<Vec<T>>` kullanımından kaçının.
2. **Matris Çarpımı Yapacaksanız:** Asla manuel iç içe döngüler (`for` içinde `for`) yazmaya çalışmayın. Hata yapmaya çok müsaittir ve optimize edilmemiştir. Doğrudan `nalgebra` veya `ndarray` crate'lerini `Cargo.toml`'a ekleyin.
3. **Oyun Geliştirme (GameDev):** Eğer Rust ile oyun yapıyorsanız, `nalgebra` veya `glam` kütüphaneleri vektör ve matris işlemleri (özellikle 3D uzayda dönüşümler) için vazgeçilmezdir.
4. **Bellek Düzeni (Memory Layout):** Matrislerde "Row-major" (Satır öncelikli - C/Rust/NumPy) ve "Column-major" (Sütun öncelikli - Fortran/MATLAB) kavramları vardır. Rust ve `ndarray` varsayılan olarak **Row-major** kullanır. Harici bir C/C++ kütüphanesiyle (FFI) veri alışverişi yaparken bu sıralamaya dikkat etmelisiniz.