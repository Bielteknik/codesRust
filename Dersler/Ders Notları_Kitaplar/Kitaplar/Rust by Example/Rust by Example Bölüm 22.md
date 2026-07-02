# Rust'ta `unsafe` - Kapsamlı Ders Anlatımı

## 🎯 Giriş: Unsafe Nedir?

Rust, bellek güvenliği ve sıfır maliyetli soyutlamaları ile ünlü bir dildir. Ancak bazı durumlarda derleyicinin sağladığı güvenlik kontrollerini atlayıp doğrudan donanımla veya düşük seviyeli sistemlerle etkileşime girmemiz gerekir. İşte bu nokta `unsafe` bloklarının devreye girdiği yerdir.

> **Altın Kural:** *"Kod tabanındaki unsafe kod miktarını minimize etmeye çalışmalı."*

### Unsafe Neden Kullanılır?

Rust'ta `unsafe` dört temel amaç için kullanılır:

1. **Ham işaretçileri (raw pointers) referanslarını çözmek**
2. **`unsafe` olarak işaretlenmiş fonksiyonları çağırmak** (FFI dahil)
3. **Statik mutable değişkenlere erişmek veya değiştirmek**
4. **Unsafe trait'leri implemente etmek**

### Önemli Kavram: Undefined Behavior (Tanımsız Davranış)

`unsafe` kod yazmak, kodun tehlikeli olduğu anlamına gelmez. Aksine, **doğruluğu kanıtlama sorumluluğunun derleyiciden programcıya geçtiği** anlamına gelir. Eğer invariant'ları (değişmezleri) ihlal ederseniz, program **tanımsız davranış** sergiler - bu da çökme, veri bozulması veya güvenlik açıkları anlamına gelebilir.

---

## 📚 Bölüm 1: Ham İşaretçiler (Raw Pointers)

### Referanslar vs. Ham İşaretçiler

Rust'ta iki tür işaretçi vardır:

| Özellik | Referans (`&T`) | Ham İşaretçi (`*const T`, `*mut T`) |
|---------|-----------------|--------------------------------------|
| Null olabilir mi? | ❌ Hayır | ✅ Evet |
| Geçerli bellek garantisi | ✅ Evet | ❌ Hayır |
| Borrowing kuralları | ✅ Uyar | ❌ Uymak zorunda değil |
| Aynı anda birden fazla mutable | ❌ Hayır | ✅ Evet |
| Otomatik temizleme | ✅ Evet | ❌ Hayır |
| Dereference için `unsafe` gerekli mi? | ❌ Hayır | ✅ Evet |

### Ham İşaretçi Oluşturma

```rust
fn main() {
    let mut num = 5;
    
    // Ham işaretçiler oluşturmak GÜVENLİDİR
    let r1 = &raw const num;  // *const i32 - salt okunur
    let r2 = &raw mut num;    // *mut i32 - yazılabilir
    
    // Ama dereference etmek UNSAFE bloğu gerektirir!
    unsafe {
        println!("r1 is: {}", *r1);  // 5
        println!("r2 is: {}", *r2);  // 5
    }
}
```

### Tehlikeli Senaryolar

Ham işaretçiler şu durumlarda tehlikelidir:

```rust
fn main() {
    let address = 0x012345usize;
    let r = address as *const i32;
    
    let numbers = vec![1, 2, 3];
    let r2 = numbers.as_ptr();
    
    unsafe {
        // Dangling pointer - bellek serbest bırakılmış!
        drop(numbers); // numbers artık geçerli değil
        
        // Bu TANIMSIZ DAVRANIŞ!
        println!("r2 is: {}", *r2);
    }
}
```

### Pratik Örnek: `split_at_mut` Implementasyonu

Standart kütüphanedeki `split_at_mut` fonksiyonunu düşünelim. Bu fonksiyon bir dilimi (slice) belirli bir indeksten ikiye böler:

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    
    let (a, b) = r.split_at_mut(3);
    
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}
```

Bu fonksiyonu safe Rust ile implement etmeye çalışırsak, borrow checker hata verir çünkü aynı bellek bölgesine iki mutable referans oluşturmaya çalışırız. Unsafe ile çözümü:

```rust
use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();
    
    assert!(mid <= len);
    
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```

**Önemli:** `split_at_mut` fonksiyonunun kendisi `unsafe` olarak işaretlenmez! İçinde unsafe kod barındırsa da, dışarıya safe bir API sunar. Bu, **safe abstraction** (güvenli soyutlama) kavramının temelidir.

---

## 📚 Bölüm 2: Unsafe Fonksiyonlar

### Unsafe Fonksiyon Tanımlama

Bir fonksiyon `unsafe` olarak işaretlendiğinde, bu fonksiyonun **belirli gereksinimleri** (contracts) olduğu anlamına gelir. Bu gereksinimleri sağlamak çağrı yapanın sorumluluğundadır.

```rust
unsafe fn danger_will_robinson() {
    // Bu fonksiyonun bazı invariant'ları vardır
    // Çağrı yapan bu invariant'ları sağlamak zorundadır
}

fn main() {
    // danger_will_robinson(); // HATA! unsafe blok gerekli
    
    unsafe {
        danger_will_robinson(); // Tamam
    }
}
```

### Örnek: `std::slice::from_raw_parts`

Bu fonksiyon ham bir işaretçi ve uzunluk alarak bir dilim oluşturur:

```rust
use std::slice;

fn main() {
    let mut values = [1, 2, 3, 4, 5];
    
    unsafe {
        // Güvenli: geçerli bellek ve doğru tip
        let slice: &[i32] = slice::from_raw_parts(
            values.as_ptr(),
            values.len()
        );
        println!("{:?}", slice);
        
        // TEHLİKELİ: Geçersiz uzunluk
        let bad_slice: &[i32] = slice::from_raw_parts(
            values.as_ptr(),
            1000 // values sadece 5 elemanlı!
        );
        // Tanımsız davranış!
    }
}
```

### Safe Abstraction Pattern

En iyi pratik, unsafe kodu safe bir arayüzün arkasına saklamaktır:

```rust
use std::slice;

/// Güvenli bir fonksiyon - unsafe kod içerir ama caller için güvenlidir
pub fn safe_wrapper(data: &mut [i32]) -> (&mut [i32], &mut [i32]) {
    let mid = data.len() / 2;
    let ptr = data.as_mut_ptr();
    
    // Invariant kontrolü
    assert!(mid <= data.len());
    
    unsafe {
        // Sadece geçerli işaretçiler oluşturuyoruz
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), data.len() - mid),
        )
    }
}
```

---

## 📚 Bölüm 3: Inline Assembly (Satıriçi Montaj)

### `asm!` Makrosu

Rust, `asm!` makrosu üzerinden inline assembly desteği sunar. Bu, derleyicinin ürettiği assembly koduna elle yazılmış assembly gömmenizi sağlar.

**Ne zaman kullanılır?**
- Maksimum performans veya zamanlama gerektiren durumlar
- Düşük seviyeli donanım primitive'lerine erişim (kernel kodu)
- Özel CPU talimatları

**Desteklenen mimariler:**
- x86 ve x86-64
- ARM
- AArch64
- RISC-V

### Temel Kullanım

```rust
#![allow(unused)]
fn main() {
    #[cfg(target_arch = "x86_64")] {
        use std::arch::asm;
        
        unsafe {
            // NOP (No Operation) - hiçbir şey yapmaz
            asm!("nop");
        }
    }
}
```

⚠️ **Tüm `asm!` çağrıları `unsafe` bloğu içinde olmalıdır** çünkü keyfi talimatlar ekleyip çeşitli invariant'ları bozabilirler.

### Veriyle Çalışmak

```rust
#[cfg(target_arch = "x86_64")] {
    use std::arch::asm;
    
    let x: u64;
    unsafe {
        // x'e 5 değerini yaz
        asm!("mov {}, 5", out(reg) x);
    }
    assert_eq!(x, 5);
}
```

**Operand türleri:**
- `out`: Sadece çıktı
- `in`: Sadece girdi
- `inout`: Hem girdi hem çıktı (aynı register)
- `inlateout`: Girdi ve çıktı (farklı register olabilir)
- `lateout`: Sadece çıktı (tüm girdiler okunduktan sonra yazılır)

### Girdi ve Çıktı Birlikte

```rust
#[cfg(target_arch = "x86_64")] {
    use std::arch::asm;
    
    let i: u64 = 3;
    let o: u64;
    unsafe {
        asm!(
            "mov {0}, {1}",  // i'yi o'ya kopyala
            "add {0}, 5",    // o'ya 5 ekle
            out(reg) o,
            in(reg) i,
        );
    }
    assert_eq!(o, 8);
}
```

### `inout` Kullanımı

```rust
#[cfg(target_arch = "x86_64")] {
    use std::arch::asm;
    
    let mut x: u64 = 3;
    unsafe {
        // x hem girdi hem çıktı - aynı register'da
        asm!("add {0}, 5", inout(reg) x);
    }
    assert_eq!(x, 8);
}
```

### `inlateout` ile Farklı Değişkenler

```rust
#[cfg(target_arch = "x86_64")] {
    use std::arch::asm;
    
    let x: u64 = 3;
    let y: u64;
    unsafe {
        // x girdi, y çıktı - farklı register'larda olabilir
        asm!("add {0}, 5", inout(reg) x => y);
    }
    assert_eq!(y, 8);
}
```

### Belirli Register'lar

Bazı talimatlar belirli register'lar gerektirir:

```rust
#[cfg(target_arch = "x86_64")] {
    use std::arch::asm;
    
    let cmd = 0xd1;
    unsafe {
        // out talimatı sadece eax kabul eder
        asm!("out 0x64, eax", in("eax") cmd);
    }
}
```

### 128-bit Çarpma Örneği

```rust
#[cfg(target_arch = "x86_64")] {
    use std::arch::asm;
    
    fn mul(a: u64, b: u64) -> u128 {
        let lo: u64;
        let hi: u64;
        
        unsafe {
            asm!(
                // x86 mul talimatı rax'ı girdi olarak alır
                // 128-bit sonucu rax:rdx'e yazar
                "mul {}",
                in(reg) a,
                inlateout("rax") b => lo,
                lateout("rdx") hi
            );
        }
        
        ((hi as u128) << 64) + lo as u1228
    }
}
```

### Clobbered Register'lar

Assembly kodu bazı register'ları değiştirebilir ama çıktı olarak kullanmayabiliriz. Bunları compiler'a bildirmeliyiz:

```rust
use std::arch::asm;

#[cfg(target_arch = "x86_64")]
fn main() {
    let mut name_buf = [0_u8; 12];
    
    unsafe {
        asm!(
            "push rbx",
            "cpuid",
            "mov [rdi], ebx",
            "mov [rdi + 4], edx",
            "mov [rdi + 8], ecx",
            "pop rbx",
            in("rdi") name_buf.as_mut_ptr(),
            inout("eax") 0 => _,  // _ = çıktıyı at
            out("ecx") _,         // clobbered
            out("edx") _,         // clobbered
        );
    }
}
```

### `clobber_abi` ile Otomatik Clobber

```rust
#[cfg(target_arch = "x86_64")] {
    use std::arch::asm;
    
    extern "C" fn foo(arg: i32) -> i32 {
        arg * 2
    }
    
    fn call_foo(arg: i32) -> i32 {
        unsafe {
            let result;
            asm!(
                "call {}",
                in(reg) foo,
                in("rdi") arg,
                out("rax") result,
                // C calling convention'a göre tüm clobbered register'ları işaretle
                clobber_abi("C"),
            );
            result
        }
    }
}
```

### Optimizasyon Seçenekleri

```rust
#[cfg(target_arch = "x86_64")] {
    use std::arch::asm;
    
    let mut a: u64 = 4;
    let b: u64 = 4;
    unsafe {
        asm!(
            "add {0}, {1}",
            inlateout(reg) a, in(reg) b,
            options(pure, nomem, nostack),
        );
    }
    assert_eq!(a, 8);
}
```

**Seçenekler:**
- `pure`: Kodun gözlemlenebilir side effect'i yok, sadece girdilere bağlı
- `nomem`: Bellek okuma/yazma yapmaz
- `nostack`: Stack'e veri push etmez

---

## 📚 Bölüm 4: Intrinsics (Dahili Fonksiyonlar)

### Intrinsics Nedir?

Intrinsics, derleyici tarafından özel olarak ele alınan ve genellikle doğrudan makine koduna çevrilen fonksiyonlardır. Bunlar standart kütüphane fonksiyonları gibi davranır ama derleyicinin derin bilgilerine erişim sağlar.

### Yaygın Intrinsics

```rust
#![feature(core_intrinsics)]

use std::intrinsics;

fn main() {
    unsafe {
        // Olası optimizasyonları engelle
        intrinsics::black_box(42);
        
        // Unreachable kod işaretleme
        // intrinsics::unreachable();
        
        // Boyut bilgileri
        let size = intrinsics::size_of::<i32>();
        println!("i32 boyutu: {} byte", size);
        
        // Kesme (trap) oluşturma
        // intrinsics::breakpoint();
    }
}
```

### Transmute

En güçlü (ve en tehlikeli) intrinsic'lerden biri `transmute`'dur:

```rust
fn main() {
    unsafe {
        // Bit pattern'ı koruyarak tip değiştir
        let x: f32 = 1.0;
        let y: u32 = std::mem::transmute(x);
        println!("f32: {}, u32: {}", x, y);
        
        // DİZİ -> DİZİ dönüşümü
        let arr: [u8; 4] = [1, 2, 3, 4];
        let num: u32 = std::mem::transmute(arr);
        println!("u32: {}", num);
    }
}
```

⚠️ **Transmute tehlikelidir!** Yanlış kullanımda tanımsız davranış oluşur. Sadece aynı boyuttaki tipler arasında kullanılmalıdır.

---

## 📚 Bölüm 5: Static Mutable Değişkenler

### Global Mutable State

Rust'ta global mutable değişkenler `unsafe` gerektirir çünkü thread-safe değildir:

```rust
static mut COUNTER: u32 = 0;

fn increment() {
    unsafe {
        COUNTER += 1;  // Race condition riski!
    }
}

fn main() {
    unsafe {
        increment();
        increment();
        println!("Counter: {}", COUNTER);
    }
}
```

### Thread-Safe Alternatifler

Modern Rust'ta bunun yerine şunlar tercih edilir:

```rust
use std::sync::atomic::{AtomicU32, Ordering};

static COUNTER: AtomicU32 = AtomicU32::new(0);

fn increment() {
    COUNTER.fetch_add(1, Ordering::SeqCst);
}

fn main() {
    increment();
    increment();
    println!("Counter: {}", COUNTER.load(Ordering::SeqCst));
}
```

---

## 📚 Bölüm 6: Unsafe Trait'ler

### Unsafe Trait Tanımlama

Bir trait `unsafe` olarak işaretlendiğinde, bu trait'i implemente eden tipin belirli invariant'ları sağlaması gerekir:

```rust
unsafe trait SafeToTransfer {
    // Bu tipi thread'ler arasında taşımak güvenli
}

struct MyType {
    data: Vec<u8>,
}

// Manuel olarak güvenli olduğunu beyan et
unsafe impl SafeToTransfer for MyType {}

fn main() {
    // Kullanım
}
```

### Örnek: `Send` ve `Sync`

Standart kütüphanedeki en önemli unsafe trait'ler:

```rust
use std::rc::Rc;

fn main() {
    // Rc<T> Send değil - thread'ler arası taşınamaz
    // Rc<T> Sync değil - thread'ler arası paylaşılamaz
    
    // Ama Box<T> hem Send hem Sync
    let b = Box::new(5);
    
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}
    
    assert_send::<Box<i32>>();
    assert_sync::<Box<i32>>();
}
```

---

## 📚 Bölüm 7: Zero-Cost Abstractions

### Kavram

Zero-cost abstraction, "kullanmadığın şey için ödeme yapmazsın" ve "kullandığın şey için elle yazılmış koddan daha fazla ödeme yapmazsın" prensiplerine dayanır.

### Örnek: Iterator'lar

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Bu iterator zinciri, elle yazılmış bir for döngüsüyle
    // aynı makine koduna derlenir!
    let sum: i32 = numbers.iter()
        .filter(|&&x| x > 2)
        .map(|&x| x * 2)
        .sum();
    
    println!("Toplam: {}", sum); // 24
}
```

### Unsafe ile Zero-Cost Abstraction

```rust
use std::slice;

/// Güvenli soyutlama - unsafe kodu gizler
pub struct SafeBuffer {
    data: Vec<u8>,
}

impl SafeBuffer {
    pub fn new(size: usize) -> Self {
        SafeBuffer {
            data: vec![0; size],
        }
    }
    
    /// Güvenli API sunar
    pub fn split_at_mut(&mut self, mid: usize) -> (&mut [u8], &mut [u8]) {
        assert!(mid <= self.data.len());
        
        let ptr = self.data.as_mut_ptr();
        let len = self.data.len();
        
        unsafe {
            // Invariant'lar sağlandı:
            // - mid <= len
            // - ptr geçerli
            // - İki dilim örtüşmüyor
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }
}
```

---

## 🎓 En İyi Pratikler

### 1. Unsafe Kodu Minimize Edin

```rust
// ❌ Kötü: Tüm fonksiyon unsafe
unsafe fn process_data(data: &mut [u8]) {
    // ...
}

// ✅ İyi: Sadece gerekli kısım unsafe
fn process_data(data: &mut [u8]) {
    // Safe kontroller
    if data.is_empty() {
        return;
    }
    
    unsafe {
        // Sadece gerçekten unsafe olan kısım
    }
}
```

### 2. Safe Abstraction'lar Oluşturun

```rust
// Unsafe kodu safe bir API'nin arkasına saklayın
pub fn safe_api(/* ... */) -> Result<(), Error> {
    // Invariant kontrolleri
    if !invariants_hold() {
        return Err(Error::InvalidInput);
    }
    
    unsafe {
        // Artık invariant'lar sağlandığı için güvenli
        unsafe_operation();
    }
    
    Ok(())
}
```

### 3. Dokümantasyon Yazın

```rust
/// # Safety
///
/// Çağrı yapan aşağıdakileri garanti etmelidir:
/// - `ptr` geçerli bir bellek adresini göstermelidir
/// - `len` bellek bölgesinin gerçek uzunluğunu aşmamalıdır
/// - `ptr` doğru tipe işaret etmelidir
unsafe unsafe_function(ptr: *const u8, len: usize) {
    // ...
}
```

### 4. Test Edin

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_unsafe_code() {
        // Normal durumlar
        // Edge case'ler
        // Hata durumları
    }
}
```

### 5. Miri Kullanın

Miri, Rust'ta undefined behavior tespit eden bir araçtır:

```bash
cargo +nightly miri test
```

---

## 📊 Özet Tablo

| Konu | Ne Zaman Kullanılır | Risk Seviyesi |
|------|---------------------|---------------|
| Raw Pointers | FFI, düşük seviyeli optimizasyonlar | 🔴 Yüksek |
| Unsafe Fonksiyonlar | Standart kütüphane, FFI | 🟡 Orta |
| Inline Assembly | Donanım erişimi, özel optimizasyonlar | 🔴 Yüksek |
| Intrinsics | Derleyici optimizasyonları | 🟡 Orta |
| Static Mutable | Global state (nadiren) | 🔴 Yüksek |
| Unsafe Traits | Özel thread-safety garantileri | 🟡 Orta |

---

## 🎯 Sonuç

`unsafe` Rust'ın en güçlü ama en tehlikeli özelliğidir. Doğru kullanıldığında:
- C/C++ ile aynı performansa ulaşabilirsiniz
- Düşük seviyeli donanımla etkileşim kurabilirsiniz
- Sıfır maliyetli soyutlamalar oluşturabilirsiniz

Ancak yanlış kullanımda:
- Tanımsız davranış
- Güvenlik açıkları
- Zor debug edilen hatalar

**Altın kurallar:**
1. Unsafe kodu minimize edin
2. Safe abstraction'lar oluşturun
3. Invariant'ları dokümante edin
4. Kapsamlı testler yazın
5. Miri gibi araçlar kullanın

Unutmayın: **`unsafe` yazmak zorunda değilsiniz, ama yazmanız gerektiğinde sorumluluk sizdedir!** 🚀

# DeepSeek Yorumu -----------------------

Rust By Example'deki **Unsafe Operations** sayfası, Rust'ın güvenlik garantilerini bir kenara bırakarak daha düşük seviyeli işlemler yapmamıza izin veren `unsafe` anahtar kelimesini ele alır. Bu bölüm, `unsafe`'in ne olduğunu, neden var olduğunu ve dört temel kullanım alanını detaylı bir şekilde açıklar.

## Unsafe Nedir ve Neden Kullanılır?

Rust'ın en büyük vaatlerinden biri, derleyicinin sağladığı güvenlik garantileridir. Ödünç alma denetleyicisi (borrow checker) ve tür sistemi sayesinde Rust, bellek hatları, veri yarışları (data races) ve geçersiz bellek erişimleri gibi birçok hatayı derleme aşamasında engeller.

Ancak bazı durumlarda, bu güvenlik kısıtlamaları esneklik ihtiyacımızın önüne geçebilir. Örneğin:
- **Donanıma doğrudan erişmek** (işletim sistemi çekirdeği veya sürücü yazarken)
- **Diğer dillerle etkileşim** (C veya C++ ile FFI üzerinden çalışırken)
- **Performans kritik işlemler** (güvenlik kontrollerini atlayarak hız kazanmak)
- **Düşük seviyeli bellek yönetimi** (kendi veri yapılarımızı oluştururken)

İşte bu noktada `unsafe` devreye girer. `unsafe`, derleyiciye "Bu kodu ben yazdım, güvenli olduğundan eminim, sen karışma" dememizi sağlar. Ancak bu büyük bir sorumluluktur: `unsafe` blokları içindeki kodun doğruluğunu garanti etmek artık tamamen **bize** aittir. Eğer `unsafe` kodunda bir hata yaparsak, programımız çökebilir, güvenlik açıklarına yol açabilir veya tamamen öngörülemez davranışlar sergileyebilir.

Rust ekibi, `unsafe` kodunu mümkün olduğunca az kullanmamızı ve güvenli soyutlamaların arkasına saklamamızı önerir. İdeal olan, `unsafe`'i yalnızca küçük, iyi test edilmiş çekirdek modüllerde kullanmak ve geri kalan her şeyi güvenli Rust ile inşa etmektir.

---

## Unsafe'in Dört Temel Kullanım Alanı

Rust'ta `unsafe` tam olarak dört şey yapmak için kullanılır:

### 1. Ham İşaretçileri (Raw Pointers) Kullanmak

Ham işaretçiler (`*const T` ve `*mut T`), referanslar (`&T` ve `&mut T`) gibi çalışır ancak önemli bir farkla: referanslar her zaman geçerli veriyi gösterir ve ödünç alma kurallarına tabidir. Ham işaretçiler ise bu garantileri sunmaz; null olabilirler, geçersiz belleği gösterebilirler veya artık kullanılmayan (zombi) verilere işaret edebilirler.

Ham işaretçilerin kullanımı `unsafe` gerektirir çünkü onları dereference etmek (içlerindeki veriye erişmek) tamamen bizim sorumluluğumuzdadır.

```rust
fn main() {
    let raw_p: *const u32 = &10;  // Ham işaretçi oluşturmak güvenlidir
    unsafe {
        assert!(*raw_p == 10);     // Dereference etmek unsafe gerektirir
    }
}
```

Bu örnekte, `&10` ile bir referans oluşturuyor, sonra bunu `*const u32` türünde bir ham işaretçiye dönüştürüyoruz. Ham işaretçiyi oluşturmak güvenlidir, ancak içindeki değeri okumak için (`*raw_p`) `unsafe` bloğu kullanmak zorundayız.

### 2. Unsafe Fonksiyonları Çağırmak

Bazı fonksiyonlar `unsafe` olarak işaretlenir. Bu, fonksiyonun doğru çalışması için çağıranın belirli koşulları sağlaması gerektiği anlamına gelir. Derleyici bu koşulları kontrol edemez, bu nedenle bunları sağlamak bizim sorumluluğumuzdur.

En yaygın örneklerden biri, `std::slice::from_raw_parts` fonksiyonudur. Bu fonksiyon, bir ham işaretçi ve uzunluk bilgisi alarak bir dilim (slice) oluşturur:

```rust
use std::slice;

fn main() {
    let some_vector = vec![1, 2, 3, 4];
    let pointer = some_vector.as_ptr();
    let length = some_vector.len();

    unsafe {
        let my_slice: &[u32] = slice::from_raw_parts(pointer, length);
        assert_eq!(some_vector.as_slice(), my_slice);
    }
}
```

Burada `slice::from_raw_parts` çağrısı `unsafe` olarak işaretlenmiştir. Çünkü bu fonksiyonun doğru çalışması için bazı **varsayımların** (invariant) sağlanması gerekir:
- `pointer` geçerli belleği göstermeli
- `pointer`'ın işaret ettiği bellek, doğru türde (`u32`) olmalı
- `length` kadar eleman erişilebilir olmalı
- Bellek, dilimin ömrü boyunca geçerli kalmalı

Eğer bu koşullardan herhangi biri ihlal edilirse, programın davranışı **tanımsız** (undefined) olur. Yani program çökebilir, yanlış sonuçlar üretebilir veya tamamen rastgele davranabilir.

> **Not:** `unsafe` fonksiyonlar yalnızca FFI (Foreign Function Interface) ile C fonksiyonlarını çağırırken de kullanılır. Örneğin, C kütüphanesindeki bir fonksiyonu çağırmak için `unsafe extern "C" fn` bildirimi kullanılır.

### 3. Değişken Statik Değişkenlere Erişmek veya Onları Değiştirmek

Statik değişkenler (`static`) programın tüm ömrü boyunca yaşar. Eğer bir statik değişken `mut` (değişebilir) olarak işaretlenirse, ona erişmek veya değerini değiştirmek `unsafe` gerektirir.

Bunun nedeni, değişebilir statik değişkenlere birden fazla iş parçacığından aynı anda erişilmesi durumunda veri yarışları (data races) oluşabilmesidir. Rust, bu tür yarışları otomatik olarak engelleyemez, bu nedenle bunu önlemek bizim sorumluluğumuzdur.

```rust
static mut COUNTER: u32 = 0;

fn main() {
    unsafe {
        COUNTER += 1;
        println!("COUNTER: {}", COUNTER);
    }
}
```

Bu tür kullanımlardan kaçınmak genellikle daha iyidir. Bunun yerine, `Mutex`, `Atomic` tipleri veya `OnceCell` gibi güvenli soyutlamalar tercih edilmelidir.

### 4. Unsafe Trait'leri Uygulamak

Bir trait `unsafe` olarak işaretlenebilir. Bu, trait'i uygulayan kişinin, trait'in belirttiği güvenlik garantilerini sağlamakla yükümlü olduğu anlamına gelir.

Örneğin, `std::marker::Send` ve `std::marker::Sync` trait'leri `unsafe`'tir. Bu trait'ler, bir türün iş parçacıkları arasında güvenle gönderilip gönderilemeyeceğini veya paylaşılıp paylaşılamayacağını belirtir. Derleyici bu trait'leri otomatik olarak uygulayabilir, ancak bazı durumlarda (örneğin, ham işaretçi içeren özel bir tür için) bunları manuel olarak uygulamamız gerekebilir. Bu durumda, türümüzün gerçekten iş parçacığı güvenli olduğundan emin olmak bize düşer.

```rust
unsafe trait MyUnsafeTrait {
    // ...
}

unsafe impl MyUnsafeTrait for MyType {
    // ...
}
```

---

## Unsafe Kullanırken Dikkat Edilmesi Gerekenler

1. **Minimize Edin:** Mümkün olduğunca az `unsafe` kullanın. Güvenli soyutlamalar oluşturun ve `unsafe`'i bu soyutlamaların içine hapsedin.

2. **Belgelendirin:** Her `unsafe` bloğunun neden güvenli olduğunu açıklayan yorumlar ekleyin. Hangi invariant'ların sağlandığını ve neden geçerli olduklarını belirtin.

3. **Test Edin:** `unsafe` kodunu kapsamlı bir şekilde test edin. Miri (Rust'ın yorumlayıcısı) gibi araçlar, tanımsız davranışları tespit etmeye yardımcı olabilir.

4. **Soyutlayın:** `unsafe` kodunu, güvenli bir API'nin arkasına saklayın. Kullanıcılarınızın `unsafe` görmesine gerek kalmasın.

---

## Özet

| Kullanım Alanı | Açıklama |
|---|---|
| **Ham İşaretçiler** | `*const T` ve `*mut T`'yi dereference etmek |
| **Unsafe Fonksiyonlar** | `unsafe fn` ile işaretlenmiş fonksiyonları çağırmak |
| **Değişken Statikler** | `static mut` değişkenlere erişmek |
| **Unsafe Trait'ler** | `unsafe trait` uygulamak |

`unsafe`, Rust'ın en güçlü ama aynı zamanda en tehlikeli araçlarından biridir. Doğru kullanıldığında, C veya C++'ın sunduğu esnekliği Rust'ın güvenlik garantileriyle birleştirir. Yanlış kullanıldığında ise, Rust'ın sizi koruduğu tüm hatalara kapı açar. Bu nedenle, `unsafe`'i bir süper güç olarak değil, büyük sorumluluk gerektiren bir araç olarak görün.
