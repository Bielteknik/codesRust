# 🦀 Ders Notları: Bölüm 15 - Rust Smart Pointers - Kapsamlı Ders

Merhaba! Bu derste Rust'ın **Smart Pointers** (Akıllı İşaretçiler) konusunu tüm alt başlıklarıyla birlikte detaylı bir şekilde öğreneceğiz. Hazırsanız başlayalım!

---

## 📚 Bölüm 1: Smart Pointer Nedir?

### Temel Kavramlar

**Pointer (İşaretçi)**, bellekteki bir adresi tutan değişkendir. Bu adres, başka bir veriye işaret eder.

Rust'ta en yaygın pointer türü **reference**'lardır (referanslar). Bunlar `&` sembolü ile gösterilir ve işaret ettikleri değeri **ödünç alırlar** (borrow). Referansların özel yetenekleri yoktur ve overhead (ek yük) oluşturmazlar.

**Smart Pointer**'lar ise:
- Pointer gibi davranan veri yapılarıdır
- **Ek metadata** ve **ek yetenekler** sunarlar
- C++'tan gelir, Rust'ta da mevcuttur
- Çoğunlukla **struct** olarak implement edilirler
- **`Deref`** ve **`Drop`** trait'lerini implement ederler

### Referans vs Smart Pointer Farkı

| Özellik | Referans (&) | Smart Pointer |
|---------|--------------|---------------|
| Veri sahipliği | Sadece ödünç alır | Çoğunlukla verinin **sahibidir** |
| Ek özellikler | Yok | Metadata ve ek yetenekler |
| Trait implementasyonu | Gerekmez | `Deref` ve `Drop` |

### Rust Standart Kütüphanesindeki Smart Pointer'lar

1. **`Box<T>`** - Heap'te değer saklamak için
2. **`Rc<T>`** - Referans sayma ile çoklu sahiplik
3. **`RefCell<T>`** - Runtime'da borrowing kurallarını zorlayan tür (interior mutability)

---

## 📦 Bölüm 2: `Box<T>` - Heap'te Veri Saklama

### Box Nedir?

`Box<T>`, en basit smart pointer'dır. Veriyi **stack yerine heap'te** saklamanızı sağlar. Stack'te sadece heap'teki veriye işaret eden pointer kalır.

```rust
fn main() {
    let b = Box::new(5);
    println!("b = {b}");
}
```

Bu kod `b = 5` yazdırır. `b` scope dışına çıktığında hem box (stack'te) hem de işaret ettiği veri (heap'te) deallocate edilir.

### Box Ne Zaman Kullanılır?

**1. Compile time'da boyutu bilinmeyen türler için:**
```rust
// Recursive type örneği
enum List {
    Cons(i32, Box<List>),  // Box ile boyut biliniyor
    Nil,
}
```

**2. Büyük veri transferlerinde:**
- Stack'te sadece pointer kopyalanır
- Heap'teki büyük veri olduğu yerde kalır
- Performans artışı sağlar

**3. Trait object'ler için:**
- Sadece belirli bir trait'i implement etmesi önemli olan değerler

### Recursive Types ve Box

**Recursive type**, kendi türünden bir değer içeren türdür. Rust, compile time'da her türün boyutunu bilmek zorundadır.

**Sorun:**
```rust
enum List {
    Cons(i32, List),  // ❌ HATA: Infinite size
    Nil,
}
```

Bu kod çalışmaz çünkü:
- `Cons` bir `i32` + bir `List` içeriyor
- O `List` de bir `Cons` içerebilir
- Bu sonsuza kadar devam eder
- Rust boyutu hesaplayamaz

**Çözüm - Box kullanmak:**
```rust
enum List {
    Cons(i32, Box<List>),  // ✅ Box pointer olduğu için boyutu sabit
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
```

**Neden çalışır?**
- `Box<T>` bir pointer'dır
- Pointer'ın boyutu her zaman aynıdır (64-bit sistemde 8 byte)
- Veri heap'te, pointer stack'te
- Rust artık boyutu hesaplayabilir: `i32` boyutu + pointer boyutu

### Cons List Nedir?

**Cons list**, Lisp programlama dilinden gelen bir veri yapısıdır. Linked list'in basit bir versiyonudur.

```
(1, (2, (3, Nil)))
```

Her eleman:
- Mevcut değer
- Sonraki eleman (başka bir cons list)

Son eleman `Nil` (boş) olur.

Günümüzde Rust'ta genellikle `Vec<T>` tercih edilir, ama cons list recursive type'ları anlamak için mükemmel bir örnektir.

---

## 🔍 Bölüm 3: `Deref` Trait - Dereference Operatörü

### Deref Operatörü (*)

`*` operatörü, bir referansı takip ederek işaret ettiği değere erişmenizi sağlar.

**Normal referanslarla:**
```rust
fn main() {
    let x = 5;
    let y = &x;  // y, x'e referans

    assert_eq!(5, x);
    assert_eq!(5, *y);  // * ile y'nin işaret ettiği değere eriş
}
```

**Box<T> ile:**
```rust
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);  // Box da * ile kullanılabilir
}
```

### Deref Trait'i Implement Etmek

Kendi smart pointer'ımızı yapalım:

```rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
```

Bu struct'ı `*` operatörüyle kullanmak için `Deref` trait'ini implement etmeliyiz:

```rust
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0  // Tuple struct'ın ilk elemanına referans döndür
    }
}
```

**Nasıl çalışır?**
- `*y` yazdığınızda Rust bunu `*(y.deref())` olarak çevirir
- `deref()` metodu iç veriye referans döndürür
- `*` operatörü bu referansı takip eder

### Deref Coercion (Otomatik Dönüşüm)

**Deref coercion**, Rust'ın referansları otomatik olarak dönüştürmesidir.

```rust
fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    
    // Normalde şöyle yazmanız gerekir:
    hello(&(*m)[..]);
    
    // Ama deref coercion sayesinde:
    hello(&m);  // ✅ Otomatik dönüşüm!
}
```

**Dönüşüm zinciri:**
1. `&MyBox<String>` → `&String` (MyBox'ın Deref'i ile)
2. `&String` → `&str` (String'in Deref'i ile)

Rust bu dönüşümleri otomatik yapar!

### Deref Coercion Kuralları

Rust üç durumda deref coercion uygular:

1. `&T` → `&U` (eğer `T: Deref<Target=U>`)
2. `&mut T` → `&mut U` (eğer `T: DerefMut<Target=U>`)
3. `&mut T` → `&U` (eğer `T: Deref<Target=U>`)

**Not:** `DerefMut` trait'i mutable referanslar için `*` operatörünü override eder.

---

## 🗑️ Bölüm 4: `Drop` Trait - Scope Sonunda Temizlik

### Drop Nedir?

`Drop` trait'i, bir değer scope dışına çıktığında ne olacağını özelleştirmenizi sağlar.

**Kullanım alanları:**
- Dosya handle'larını kapatma
- Network bağlantılarını sonlandırma
- Memory deallocation
- Lock'ları serbest bırakma

### Drop Trait Implementasyonu

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created");
}
```

**Çıktı:**
```
CustomSmartPointers created
Dropping CustomSmartPointer with data `other stuff`!
Dropping CustomSmartPointer with data `my stuff`!
```

**Dikkat:** Değerler **ters sırada** drop edilir (LIFO - Last In First Out).

### Erken Drop Etme

Bazen bir değeri scope sonundan önce drop etmek isteyebilirsiniz.

**Yanlış yöntem:**
```rust
fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    c.drop();  // ❌ HATA: explicit drop çağrılamaz
}
```

**Hata mesajı:**
```
error[E0040]: explicit use of destructor method
```

**Neden çalışmaz?**
- Rust, scope sonunda otomatik olarak `drop` çağırır
- Manuel çağırırsanız **double free** hatası olur
- Aynı veri iki kez temizlenir

**Doğru yöntem - `std::mem::drop` fonksiyonu:**
```rust
fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created");
    
    drop(c);  // ✅ std::mem::drop fonksiyonu
    
    println!("CustomSmartPointer dropped before the end of main");
}
```

**Çıktı:**
```
CustomSmartPointer created
Dropping CustomSmartPointer with data `some data`!
CustomSmartPointer dropped before the end of main
```

**Önemli:** `std::mem::drop` fonksiyonu, `Drop` trait'inin `drop` metodundan farklıdır!

### Drop'un Önemi

`Drop` trait'i sayesinde:
- ✅ Kaynak sızıntısı olmaz
- ✅ Temizlik kodunu her yere yazmak zorunda değilsiniz
- ✅ Ownership sistemi, `drop`'un sadece bir kez çağrılmasını garanti eder
- ✅ Kendi memory allocator'ınızı bile yazabilirsiniz!

---

## 🔗 Bölüm 5: `Rc<T>` - Referans Sayma ile Çoklu Sahiplik

### Çoklu Sahiplik Problemi

Rust'ta genellikle bir değerin tek bir sahibi vardır. Ama bazı durumlarda birden fazla sahip gerekebilir:

- **Graph veri yapıları** - Birden fazla edge aynı node'a işaret edebilir
- **Paylaşılan konfigürasyon** - Birden fazla component aynı config'i kullanabilir
- **Circular references** - Birbirine referans veren yapılar

### Rc<T> Nedir?

`Rc<T>` (**Reference Counting**), bir değere kaç referans olduğunu sayar. Referans sayısı 0 olduğunda veri temizlenir.

**TV benzetmesi:**
- Bir kişi TV'yi açar (ilk referans)
- Diğerleri gelip izleyebilir (referans sayısı artar)
- Son kişi çıkarken TV'yi kapatır (referans sayısı 0 olur)

### Rc<T> Kullanımı

**Problem - Box ile çoklu sahiplik:**
```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));  // a, b'ye taşındı
    let c = Cons(4, Box::new(a));  // ❌ HATA: a zaten taşındı!
}
```

**Hata:**
```
error[E0382]: use of moved value: `a`
```

**Çözüm - Rc<T> kullanmak:**
```rust
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));  // Referans sayısı: 2
    let c = Cons(4, Rc::clone(&a));  // Referans sayısı: 3
}
```

### Rc::clone vs clone()

```rust
// ✅ Önerilen: Rc::clone
let b = Cons(3, Rc::clone(&a));

// ❌ Kaçınılmalı: a.clone()
let b = Cons(3, a.clone());
```

**Fark:**
- `Rc::clone` - Sadece referans sayısını artırır (hızlı)
- `clone()` - Deep copy yapabilir (yavaş olabilir)

`Rc::clone` kullanarak kodda performans sorunlarını daha kolay tespit edebilirsiniz.

### Referans Sayısını İzleme

```rust
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    // 1
    
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    // 2
    
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
        // 3
    }
    // c scope dışına çıktı
    
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    // 2
}
```

**Nasıl çalışır?**
- `Rc::clone` - `strong_count`'u artırır
- Scope sonu - `Drop` trait otomatik olarak `strong_count`'u azaltır
- `strong_count == 0` - Veri temizlenir

### Rc<T>'nin Kısıtlamaları

1. **Sadece single-threaded** - Multithread için `Arc<T>` kullanın
2. **Sadece immutable borrows** - `Rc<T>` ile mutable referans alamazsınız
3. **Runtime overhead** - Referans sayma küçük bir performans maliyeti

**Neden immutable?**
- Birden fazla mutable referans data race oluşturabilir
- Rust'ın borrowing kurallarını ihlal eder
- Çözüm: `RefCell<T>` ile interior mutability (sonraki bölüm)

---

## 🔄 Bölüm 6: `RefCell<T>` ve Interior Mutability

### Interior Mutability Nedir?

**Interior mutability**, immutable bir referansınız varken veriyi değiştirmenizi sağlayan bir design pattern'dir.

**Normal borrowing kuralları:**
- Ya bir mutable referans
- Ya da birden fazla immutable referans
- İkisi aynı anda olamaz

**Interior mutability:**
- Dışarıdan immutable görünür
- İçeride mutable erişim sağlar
- Borrowing kurallarını **runtime'da** kontrol eder

### RefCell<T> vs Box<T> vs Rc<T>

| Tür | Sahiplik | Borrowing Kontrolü | Mutable Erişim |
|-----|----------|-------------------|----------------|
| `Box<T>` | Tek sahip | Compile time | ✅ (mutable borrow ile) |
| `Rc<T>` | Çoklu sahip | Compile time | ❌ (sadece immutable) |
| `RefCell<T>` | Tek sahip | **Runtime** | ✅ (interior mutability) |

### RefCell<T> Kullanımı

**Senaryo:** Test için mock object yapıyoruz

```rust
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        
        let percentage = self.value as f64 / self.max as f64;
        
        if percentage >= 1.0 {
            self.messenger.send("Error: Over quota!");
        } else if percentage >= 0.9 {
            self.messenger.send("Urgent: 90% used!");
        } else if percentage >= 0.75 {
            self.messenger.send("Warning: 75% used!");
        }
    }
}
```

**Problem - Mock object yapamıyoruz:**
```rust
struct MockMessenger {
    sent_messages: Vec<String>,  // ❌ Değiştiremeyiz
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        self.sent_messages.push(String::from(message));  // ❌ HATA
        // `self` immutable referans, push mutable işlem
    }
}
```

**Çözüm - RefCell<T> kullanmak:**
```rust
use std::cell::RefCell;

struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,  // ✅ RefCell ile sarmala
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            sent_messages: RefCell::new(vec![]),
        }
    }
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        self.sent_messages.borrow_mut().push(String::from(message));
        // ✅ borrow_mut() ile mutable erişim
    }
}

#[test]
fn test_sends_over_75_percent_warning() {
    let mock = MockMessenger::new();
    let mut tracker = LimitTracker::new(&mock, 100);
    
    tracker.set_value(80);
    
    assert_eq!(mock.sent_messages.borrow().len(), 1);
    // borrow() ile immutable erişim
}
```

### RefCell<T> Nasıl Çalışır?

**Metodlar:**
- `borrow()` - Immutable borrow, `Ref<T>` döndürür
- `borrow_mut()` - Mutable borrow, `RefMut<T>` döndürür

**Runtime kontrolü:**
```rust
use std::cell::RefCell;

let data = RefCell::new(5);

let first = data.borrow_mut();
let second = data.borrow_mut();  // ❌ PANIC: Runtime hata!

// Aynı anda iki mutable borrow olamaz
```

**Panic mesajı:**
```
thread 'main' panicked at 'already borrowed: BorrowMutError'
```

### RefCell<T>'nin Avantaj ve Dezavantajları

**Avantajlar:**
- ✅ Compile time'da reddedilen geçerli kodları çalıştırabilir
- ✅ Immutable context'te mutable işlemler yapabilir
- ✅ Test mock'ları için mükemmel

**Dezavantajlar:**
- ⚠️ Runtime panic riski
- ⚠️ Küçük performans overhead'i
- ⚠️ Hataları production'da keşfedebilirsiniz

### Rc<T> + RefCell<T> Kombinasyonu

Çoklu sahiplik **ve** mutable erişim için:

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let value = Rc::new(RefCell::new(5));
    
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    
    // Değeri değiştir!
    *value.borrow_mut() += 10;
    
    println!("a after = {a:?}");  // Cons(RefCell { value: 15 }, ...)
    println!("b after = {b:?}");  // Cons(RefCell { value: 3 }, ...)
    println!("c after = {c:?}");  // Cons(RefCell { value: 4 }, ...)
}
```

**Sonuç:**
- `Rc<T>` - Çoklu sahiplik
- `RefCell<T>` - Interior mutability
- İkisi birlikte - Paylaşılan, değiştirilebilir veri

**Önemli:** `RefCell<T>` thread-safe değildir! Multithread için `Mutex<T>` kullanın.

---

## 🔄 Bölüm 7: Reference Cycles ve Weak<T>

### Reference Cycle Problemi

`Rc<T>` ve `RefCell<T>` kullanarak **reference cycle** (referans döngüsü) oluşturabilirsiniz. Bu **memory leak**'e yol açar!

**Örnek:**
```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    
    // a'nın tail'ini b'ye bağla - DÖNGÜ OLUŞTUR!
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    
    // Şimdi: a -> b -> a -> b -> ...
    
    println!("a rc count = {}", Rc::strong_count(&a));  // 2
    println!("b rc count = {}", Rc::strong_count(&b));  // 2
}
```

**Sorun:**
- `main` sonunda `b` drop edilir → `b`'nin count'u 2'den 1'e düşer
- Sonra `a` drop edilir → `a`'nın count'u 2'den 1'e düşer
- Ama her ikisi de birbirine referans veriyor!
- Count asla 0 olmaz
- **Memory leak!**

### Weak<T> ile Çözüm

**Weak<T>**, zayıf referanstır:
- Sahiplik ifade etmez
- `strong_count`'u artırmez
- `weak_count`'u artırır
- `weak_count == 0` olmak zorunda değil (temizlik için)

**Kullanım:**
```rust
use std::rc::{Rc, Weak};

let strong = Rc::new(5);
let weak = Rc::downgrade(&strong);  // Weak<T> oluştur

// Weak<T>'den değere erişmek için:
if let Some(value) = weak.upgrade() {
    println!("Value: {}", value);
} else {
    println!("Value already dropped");
}
```

### Tree Yapısı Örneği

**Parent-child ilişkisi için Weak<T> kullanımı:**

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,      // Zayıf referans (sahiplik yok)
    children: RefCell<Vec<Rc<Node>>>, // Güçlü referans (sahiplik var)
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // None (henüz parent yok)
    
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    
    // leaf'in parent'ını branch'e ayarla
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // Some(Node { value: 5, ... })
}
```

**Neden çalışır?**
- `branch`, `leaf`'in sahibidir (`Rc<T>` ile)
- `leaf`, `branch`'e sadece bakar (`Weak<T>` ile)
- `branch` drop edilirse → `leaf` de drop edilir
- Döngü olmaz, memory leak yok!

### Strong vs Weak Referanslar

| Özellik | Strong (Rc<T>) | Weak (Weak<T>) |
|---------|----------------|----------------|
| Sahiplik | ✅ Var | ❌ Yok |
| Count artırır | `strong_count` | `weak_count` |
| Temizlik için | Count 0 olmalı | Gerekli değil |
| Değere erişim | Direkt | `upgrade()` ile (Option) |
| Cycle oluşturur | ✅ Evet | ❌ Hayır |

### Best Practices

**Reference cycle'ları önlemek için:**

1. **Sahiplik ilişkilerini düşünün:**
   - Parent → Child: `Rc<T>` (sahiplik)
   - Child → Parent: `Weak<T>` (sahiplik yok)

2. **Graph yapılarında:**
   - Node → Neighbor: `Rc<T>` veya `Weak<T>`
   - Döngü oluşturmadığından emin olun

3. **Test yazın:**
   - Reference cycle'ları tespit etmek için
   - Memory leak'leri önlemek için

4. **Code review yapın:**
   - `Rc<T>` ve `RefCell<T>` kombinasyonlarına dikkat
   - Potansiyel döngüleri kontrol edin

---

## 🎯 Özet ve En İyi Uygulamalar

### Smart Pointer Seçim Rehberi

```
Veriyi heap'te saklamak mı istiyorsunuz?
└─> Box<T>

Çoklu sahiplik mi gerekiyor?
├─> Evet, immutable erişim yeterli → Rc<T>
└─> Evet, mutable erişim de lazım → Rc<RefCell<T>>

Tek sahip, ama mutable erişim gerekiyor mu?
├─> Evet, compile time kontrolü → Box<T> (mutable borrow ile)
└─> Evet, runtime kontrolü kabul edilebilir → RefCell<T>

Thread-safe mi olmalı?
├─> Rc<T> → Arc<T>
└─> RefCell<T> → Mutex<T>
```

### Trait Özetleri

**Deref Trait:**
```rust
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}
```
- `*` operatörünü özelleştirir
- Deref coercion sağlar

**Drop Trait:**
```rust
impl Drop for MyType {
    fn drop(&mut self) {
        // Temizlik kodu
    }
}
```
- Scope sonunda otomatik çağrılır
- Manuel çağrılamaz (`std::mem::drop` kullanın)

### Performans Düşünceleri

| Smart Pointer | Overhead | Kullanım |
|---------------|----------|----------|
| `Box<T>` | Minimal (sadece heap allocation) | Basit pointer ihtiyacı |
| `Rc<T>` | Referans sayma | Çoklu sahiplik |
| `RefCell<T>` | Runtime borrowing kontrolü | Interior mutability |
| `Rc<RefCell<T>>` | Her ikisi de | Paylaşılan mutable veri |

### Yaygın Hatalar ve Çözümleri

**1. Recursive type hatası:**
```rust
// ❌ Yanlış
enum List { Cons(i32, List), Nil }

// ✅ Doğru
enum List { Cons(i32, Box<List>), Nil }
```

**2. Moved value hatası:**
```rust
// ❌ Yanlış
let a = Cons(5, Box::new(Nil));
let b = Cons(3, Box::new(a));
let c = Cons(4, Box::new(a));  // a zaten taşındı

// ✅ Doğru
let a = Rc::new(Cons(5, Rc::new(Nil)));
let b = Cons(3, Rc::clone(&a));
let c = Cons(4, Rc::clone(&a));
```

**3. BorrowMutError panic:**
```rust
// ❌ Yanlış
let data = RefCell::new(5);
let a = data.borrow_mut();
let b = data.borrow_mut();  // PANIC!

// ✅ Doğru
let data = RefCell::new(5);
{
    let mut a = data.borrow_mut();
    *a += 1;
}  // a scope dışına çıktı
let b = data.borrow_mut();  // Artık sorun yok
```

**4. Reference cycle memory leak:**
```rust
// ❌ Yanlış
struct Node {
    parent: RefCell<Rc<Node>>,  // Cycle oluşturur!
}

// ✅ Doğru
struct Node {
    parent: RefCell<Weak<Node>>,  // Weak referans
}
```

---

## 🎓 Sonuç

Tebrikler! Rust'ın smart pointer sistemini tamamen öğrendiniz:

✅ **Box<T>** - Heap allocation ve recursive types  
✅ **Deref** - Dereference operatörü ve coercion  
✅ **Drop** - Otomatik temizlik ve resource management  
✅ **Rc<T>** - Reference counting ile çoklu sahiplik  
✅ **RefCell<T>** - Interior mutability ve runtime borrowing  
✅ **Weak<T>** - Reference cycle'ları önleme  

Bu bilgiler, Rust'ta güvenli ve verimli kod yazmanız için kritik öneme sahiptir. Smart pointer'lar, Rust'ın ownership sistemini esnetmenize olanak tanırken, güvenlik garantilerini korumanızı sağlar.

**Bir sonraki adım:**
- Concurrency bölümünde `Arc<T>` ve `Mutex<T>` öğrenin
- Trait objects ile `Box<dyn Trait>` kullanımını keşfedin
- Kendi smart pointer türlerinizi implement etmeyi deneyin!

Sorularınız varsa sormaktan çekinmeyin! 🚀

# DeepSeek Yorumu -----------------------

Elbette! Rust programlama dilinin en güçlü ve kendine has özelliklerinden biri olan **Akıllı İşaretçiler (Smart Pointers)** konusunu, tıpkı bir sınıfta ders anlatır gibi, en temelden ileri seviyeye doğru adım adım işleyeceğiz. Hemen hemen her Rust geliştiricisinin yolunun düştüğü bu konuyu anlamak, dilin sahiplik (ownership) ve ödünç alma (borrowing) mekanizmalarıyla nasıl dans ettiğini görmek için kritik öneme sahiptir.

Hazırsanız, Rust'ın 15. bölümüne doğru yola çıkalım.

---

## 1. Akıllı İşaretçi Nedir? Neden İhtiyaç Duyarız?

Öncelikle temel bir tanımla başlayalım. Hepimiz referansları (`&T` ve `&mut T`) biliyoruz. Bunlar, bir verinin kendisine sahip olmadan, sadece o veriyi "işaret eden" basit araçlardır. Peki ya bir işaretçi, sadece işaret etmekten fazlasını yapabilseydi? İşte akıllı işaretçiler burada devreye girer.

**Akıllı işaretçi**, bir yandan referans gibi davranırken (bir veriyi işaret ederken), diğer yandan **ekstra meta veri ve yetenekler** barındıran bir veri yapısıdır. Bu ek yetenekler arasında referans sayımı, ödünç alma kurallarını çalışma zamanında uygulama veya işaret ettiği veriyi temizleme gibi görevler bulunur. Rust'ta akıllı işaretçiler, genellikle `struct`'lar kullanılarak oluşturulur ve iki kritik trait'i implemente ederler:

*   **`Deref` Trait**: Akıllı işaretçinin bir referans gibi davranmasını sağlar. Yani `*` operatörü ile içindeki veriye erişmemize olanak tanır.
*   **`Drop` Trait**: Akıllı işaretçi kapsam dışına çıktığında ne yapılacağını (örneğin, işaret ettiği belleği serbest bırakmak) belirler.

Bugün bu iki temel trait'i ve Rust'ın standart kütüphanesinde yer alan en yaygın akıllı işaretçi türlerini derinlemesine inceleyeceğiz:
*   `Box<T>`: Veriyi heap'te saklamak için.
*   `Rc<T>`: Birden çok sahibe izin veren referans sayımlı işaretçi.
*   `RefCell<T>`: Ödünç alma kurallarını derleme zamanı yerine çalışma zamanında uygulayan, iç değişebilirlik sağlayan yapı.

---

## 2. `Box<T>`: Veriyi Heap'te Saklamak

İlk akıllı işaretçimiz `Box<T>`, en basit olanıdır. Veriyi stack'te değil, **heap** üzerinde tahsis etmemizi sağlar. Stack'te ise sadece heap'teki veriyi işaret eden bir işaretçi (pointer) kalır.

**Ne zaman kullanırız?**
1.  **Türü derleme zamanında bilinmeyen bir veriyle çalışırken:** Örneğin, bir trait objesini `Box<dyn Trait>` olarak saklayabiliriz.
2.  **Büyük bir veriyi taşırken:** Verinin kopyalanmasını önleyip, sadece sahipliğini (işaretçiyi) taşımak istediğimizde.
3.  **Tekrarlayan (recursive) türler tanımlarken:** Rust'ta `enum` veya `struct`'ların boyutu derleme zamanında bilinmelidir. Sonsuza kadar gidebilecek bir zincir (örneğin bir bağlı liste - cons list) oluşturamayız çünkü boyutu belirsizdir. `Box` ile bir dolaylama (indirection) katmanı ekleyerek, boyut bilinir hale gelir.

### Örnek: Cons List ile Tekrarlayan Tür

Lisp'ten tanıdığımız "cons list" yapısını düşünelim. Bir `List` enum'ı şöyle olabilir: Ya boştur (`Nil`) ya da bir değer ve bir sonraki `List`'i içeren bir ikilidir (`Cons`).

```rust
// Bu kod derlenmez!
enum List {
    Cons(i32, List),
    Nil,
}
```
Bu kod neden derlenmez? Çünkü `List` enum'ının `Cons` varyantı, bir `List` daha içeriyor. Bu da `Cons(i32, Cons(i32, Cons(i32, ...)))` gibi sonsuza gidebilecek bir zincir demek. Derleyici, `List`'in boyutunu hesaplayamaz. İşte `Box<T>` burada imdada yetişir. `Box<List>`, sabit boyutlu bir işaretçidir. Yani `Cons(i32, Box<List>)` artık derleme zamanında boyutu bilinen bir yapı olur.

```rust
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // En içteki eleman: (1, Nil)
    // Bir dışı: (2, -> (1, Nil) )
    // En dış: (3, -> (2, -> (1, Nil) ))
    let list = Cons(3, Box::new(Cons(2, Box::new(Cons(1, Box::new(Nil))))));
    println!("Liste: {:?}", list);
}
```
Burada `Box::new(...)` ile veriyi heap'e yerleştiriyoruz. Artık `list` değişkeni stack'te, ama içindeki `Cons` elemanlarının tümü heap'te, birbirlerine işaretçilerle bağlı. `Box<T>` sayesinde belirsiz boyut sorununu çözdük. `list` kapsam dışına çıktığında, `Box<T>`'ın `Drop` implementasyonu sayesinde heap'teki tüm zincir otomatik olarak temizlenir.

---

## 3. `Deref` Trait ile Akıllı İşaretçiyi Referans Gibi Kullanmak

Bir `Box<T>`'ı referans gibi kullanabilmemizin sırrı `Deref` trait'idir. Bu trait'i implemente eden bir tür, `*` (dereference) operatörü ile içindeki veriye erişebilir.

Hadi kendi akıllı işaretçimizi, `Kutum<T>`'yi yapalım.

```rust
use std::ops::Deref;

struct Kutum<T>(T);

impl<T> Kutum<T> {
    fn new(x: T) -> Kutum<T> {
        Kutum(x)
    }
}

impl<T> Deref for Kutum<T> {
    type Target = T; // İşaret ettiğimiz türü belirtiyoruz.

    fn deref(&self) -> &Self::Target {
        &self.0 // Kutum'un içindeki T'ye bir referans döndürüyoruz.
    }
}

fn main() {
    let x = 5;
    let y = Kutum::new(5);

    assert_eq!(5, x);
    assert_eq!(5, *y); // Burada *y, aslında *(y.deref()) çağrısına dönüşür.
}
```
`*y` yazdığımızda, Rust perde arkasında `*(y.deref())` kodunu çalıştırır. `deref()` metodu `&self.0` döndürdüğü için, bir referans elde ederiz ve `*` ile onu takip ederiz. Böylece `Kutum<T>`, içindeki veriye şeffaf bir şekilde erişmemizi sağlar.

### Deref Coercion (Deref Zorlaması)

`Deref` trait'inin en büyük kolaylığı **Deref Coercion**'dır. Rust, fonksiyon ve metod argümanlarında otomatik olarak `Deref` uygulayarak bir türü diğerine dönüştürebilir. Mesela bir `&Kutum<String>` argümanını, bir `&str` bekleyen fonksiyona doğrudan verebiliriz.

```rust
fn selam_ver(isim: &str) {
    println!("Merhaba, {isim}!");
}

let m = Kutum::new(String::from("Rust"));
selam_ver(&m); // &Kutum<String> -> &String -> &str
```
Burada Rust şu adımları uygular:
1.  `&m` bir `&Kutum<String>`'dir.
2.  `Kutum<T>`, `Deref` implemente ettiği için, derleyici bunu `&String`'e dönüştürebilir.
3.  `String` de kendi `Deref` implementasyonu ile `&str`'e dönüşür.
4.  Sonuç: `selam_ver` fonksiyonu başarıyla çağrılır.

Eğer Deref zorlaması olmasaydı, `selam_ver(&(*m)[..]);` gibi karmaşık bir kod yazmamız gerekirdi. Deref zorlaması, kodu hem daha okunabilir hem de daha ergonomik hale getirir. Aynı şekilde değişebilirlik için `DerefMut` trait'i de mevcuttur.

---

## 4. `Drop` Trait ile Kaynakları Temizlemek

`Box<T>`'ın en önemli özelliklerinden biri de kapsam dışına çıktığında işaret ettiği heap belleğini temizlemesidir. Bu, `Drop` trait'i sayesinde olur. Bir tip, `Drop` trait'ini implemente ederek, değeri kapsam dışına çıkmak üzereyken çalışacak özel bir kod (`drop` metodu) tanımlayabilir.

```rust
struct AkilliIsaretci {
    veri: String,
}

impl Drop for AkilliIsaretci {
    fn drop(&mut self) {
        println!("`{}` verisine sahip AkilliIsaretci temizleniyor!", self.veri);
    }
}

fn main() {
    let c = AkilliIsaretci { veri: String::from("ilki") };
    let d = AkilliIsaretci { veri: String::from("ikincisi") };
    println!("AkilliIsaretciler oluşturuldu.");
}
```
Kod çalıştığında, değişkenler kapsam dışına çıkar ve temizleme işlemi ters sırada (en son `d`, sonra `c`) gerçekleşir. Çıktı şöyle olur:
```
AkilliIsaretciler oluşturuldu.
`ikincisi` verisine sahip AkilliIsaretci temizleniyor!
`ilki` verisine sahip AkilliIsaretci temizleniyor!
```

### Değeri Erken Bırakmak: `std::mem::drop`

Bazen bir değeri kapsam dışına çıkmadan önce manuel olarak temizlemek isteyebiliriz. Ancak Rust, `Drop` trait'ine sahip bir türün `drop` metodunu doğrudan çağırmamıza izin **vermez**. Çünkü bu, çifte temizleme (double free) hatasına yol açar; Rust kapsam sonunda yine otomatik olarak `drop`'u çağıracaktır.

Bunun yerine standart kütüphanenin `std::mem::drop` fonksiyonunu kullanırız. Bu fonksiyon, argüman olarak aldığı değerin sahipliğini alır ve başka hiçbir şey yapmadan kapsam dışına çıkmasını sağlar, böylece `Drop` implementasyonu tetiklenir.

```rust
let c = AkilliIsaretci { veri: String::from("erken temizlenen") };
println!("c oluşturuldu.");
std::mem::drop(c); // c burada zorla temizlenir.
println!("c, main'in sonundan önce temizlendi.");
```

---

## 5. `Rc<T>`: Referans Sayımlı Akıllı İşaretçi

Sahiplik kuralları gereği, bir değerin normalde yalnızca bir sahibi olabilir. Peki ya bir graf yapısında olduğu gibi, bir verinin birden çok sahibi olması gerekiyorsa? İşte burada `Rc<T>` (Reference Counted) devreye girer. `Rc<T>`, bir veriye kaç tane referans olduğunu takip eder. Referans sayısı 0'a düştüğünde, veri otomatik olarak temizlenir.

`Rc<T>`'yi, programın **tek bir iş parçacığı (single-threaded)** ile çalışan kısımlarında kullanırız. Çoklu iş parçacığı için `Arc<T>` (Atomic Reference Counted) vardır, ancak bu bölümün konusu değil.

### Cons List'i `Rc<T>` ile Paylaşmak

Daha önce yaptığımız Cons list'i düşünelim. İki listenin, bir alt listeyi paylaşmasını isteyebiliriz.

```rust
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    // a = 5 -> 10 -> Nil
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("a oluşturulduktan sonra referans sayısı: {}", Rc::strong_count(&a)); // 1

    // b = 3 -> a (yani 5 -> 10 -> Nil)
    let b = Cons(3, Rc::clone(&a));
    println!("b oluşturulduktan sonra referans sayısı: {}", Rc::strong_count(&a)); // 2

    // c = 4 -> a (yani 5 -> 10 -> Nil)
    let c = Cons(4, Rc::clone(&a));
    println!("c oluşturulduktan sonra referans sayısı: {}", Rc::strong_count(&a)); // 3
    println!("c'nin sonundaki referans sayısı: {}", Rc::strong_count(&a)); // 3 (c hala canlı)
}
```
Burada `Rc::clone(&a)`, `a`'nın işaret ettiği verinin derin bir kopyasını almaz; sadece referans sayısını bir artırır. Bu işlem oldukça ucuzdur. Artık `b` ve `c`, `a`'nın işaret ettiği `5 -> 10 -> Nil` listesini paylaşır. `Rc::strong_count` ile herhangi bir andaki referans sayısını görebiliriz. `Rc<T>` sayesinde, bir değerin birden çok sahibi olabilir, ancak sadece **değişmez (immutable) referanslar** paylaşılabilir.

---

## 6. `RefCell<T>` ve İç Değişebilirlik (Interior Mutability)

`Rc<T>` ile birden çok değişmez referansı paylaşmak güzel, ama ya bu paylaşılan veriyi değiştirmemiz gerekirse? İşte burada `RefCell<T>` devreye girer. `RefCell<T>`, Rust'ın temel ödünç alma kurallarını (aynı anda ya bir tane değişebilir referans ya da birden çok değişmez referans) **derleme zamanı yerine çalışma zamanında** uygular. Bu, **iç değişebilirlik (interior mutability)** deseninin bir örneğidir: Dışarıdan değişmez bir referansınız olsa bile, içerideki veriyi değiştirebilirsiniz.

`RefCell<T>`, `borrow()` ve `borrow_mut()` metodlarını kullanır. `borrow()` ile değişmez bir `Ref<T>` (bir tür akıllı işaretçi), `borrow_mut()` ile değişebilir bir `RefMut<T>` elde ederiz. Eğer kuralları ihlal edersek (örneğin, zaten değişmez bir ödünç varken değişebilir bir ödünç almaya çalışırsak), program **panikler** ve sonlanır. Bu sayede, mantıksal hataları anında yakalarız.

### Örnek: Mock Objesi (Sahte Nesne) ile Test

Bir test senaryosunda, bir değerin çağrıldığı sayısını takip etmek isteyelim. `Messenger` trait'ine sahip bir `MockMessenger` yapalım.

```rust
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker { messenger, value: 0, max }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Hata: Kota aşıldı!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Acil Uyarı: Kotanın %90'ı kullanıldı!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Uyarı: Kotanın %75'i kullanıldı!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        // `send` metodu &self (değişmez referans) almasına rağmen, içeride veriyi değiştirebiliyoruz!
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);

        // borrow() ile değişmez erişim sağlıyoruz.
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
```
Burada `send` metodu `&self` alır, yani `MockMessenger` üzerinde değişmez bir referans ile çağrılır. Ama `RefCell<Vec<String>>` sayesinde `sent_messages` vektörüne yeni mesaj ekleyebildik (`borrow_mut()` ile). Eğer `RefCell<T>` yerine normal bir `Vec<String>` kullansaydık, derleyici `cannot borrow `self.sent_messages` as mutable` hatası verirdi.

---

## 7. `Rc<T>` ve `RefCell<T>`'i Birleştirmek: Çok Sahipli Değişebilir Veri

`Rc<T>` ile çoklu sahiplik, `RefCell<T>` ile değişebilirlik kazandık. Bu ikisini birleştirerek, birden çok sahibi olan ve üzerinde değişiklik yapılabilen bir veri yapısı elde ederiz. Bu kombinasyon (`Rc<RefCell<T>>`), Rust'ta karmaşık ve esnek graf benzeri yapılar kurmanın en yaygın yoludur.

### Cons List Örneğine Geri Dönüş

`Rc<T>` ile paylaşılan bir listenin sonuna yeni bir eleman eklemek veya bir elemanı değiştirmek istersek `RefCell<T>`'e ihtiyaç duyarız.

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // Değeri değiştirmek istiyoruz.
    *value.borrow_mut() += 10;

    println!("a listesi: {:?}", a); // Cons(RefCell { value: 15 }, Nil)
    println!("b listesi: {:?}", b); // Cons(RefCell { value: 3 }, ...)
    println!("c listesi: {:?}", c); // Cons(RefCell { value: 4 }, ...)
}
```
Bu kodda, `a`, `b` ve `c` listelerinin tümü, `value` ile işaret edilen aynı `RefCell<i32>`'ye sahip. `value.borrow_mut()` ile bu ortak değeri değiştirdiğimizde, bu değişiklik `a`, `b` ve `c` üzerinden görülebilir olur. İşte bu güçlü kombinasyon, Rust'ın katı kuralları içinde bile büyük bir esneklik sağlar.

---

## 8. Referans Döngüleri ve Bellek Sızıntıları

Rust'ın bellek güvenliği garantileri, yanlışlıkla bellek sızıntısı oluşturmayı neredeyse imkansız kılar, ancak **tamamen** engellemez. Eğer `Rc<T>` ve `RefCell<T>` kullanarak iki değerin birbirine referans verdiği bir döngü oluşturursak, referans sayıları asla 0'a düşmez ve bellek sızıntısı meydana gelir.

### Döngü Örneği

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a başlangıç sayısı: {}", Rc::strong_count(&a)); // 1

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a'nın b oluştuktan sonraki sayısı: {}", Rc::strong_count(&a)); // 2
    println!("b başlangıç sayısı: {}", Rc::strong_count(&b)); // 1

    // a'nın tail'ini (ikinci eleman) b'ye bağlayarak bir döngü oluşturalım.
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("a'nın döngüden sonraki sayısı: {}", Rc::strong_count(&a)); // 2 (a -> b'yi gösteriyor)
    println!("b'nin döngüden sonraki sayısı: {}", Rc::strong_count(&b)); // 2 (b -> a'yı gösteriyor)

    // Bu noktada main sona erer, ancak a ve b'nin referans sayıları 2'den 1'e düşer, 0'a asla ulaşamaz.
    // Heap'teki veri temizlenmez -> BELLEK SIZINTISI.
}
```
Bu döngüyü kırmak için `Weak<T>` (zayıf referans) kullanırız. `Weak<T>`, `Rc<T>`'nin aksine referans sayısını artırmaz. Bir `Weak<T>`, referans verdiği verinin hala var olup olmadığını kontrol edebilir (`upgrade` metodu ile `Option<Rc<T>>` döndürür).

### `Weak<T>` ile Ağaç Yapısı

Bir düğümün (node) çocuklarına sahip olduğu, ancak çocukların ebeveynini zayıf bir referans olarak tuttuğu bir ağaç yapısı, döngüyü kırmak için ideal bir örnektir.

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>, // Ebeveyne zayıf referans
    children: RefCell<Vec<Rc<Node>>>, // Çocuklara güçlü referans
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent: {:?}", leaf.parent.borrow().upgrade()); // None

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // leaf'in parent'ını branch olarak ayarla.
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent: {:?}", leaf.parent.borrow().upgrade()); // Some(...)
    println!("branch'in güçlü sayısı: {}", Rc::strong_count(&branch)); // 1
    println!("branch'in zayıf sayısı: {}", Rc::weak_count(&branch));   // 1
}
```
Bu yapıda, `branch` kapsam dışına çıkarsa, güçlü referans sayısı 0'a düşer ve `branch` temizlenir. `leaf`'in `parent`'ındaki `Weak<Node>` ise geçersiz hale gelir; `upgrade()` metodu `None` döndürür. Böylece hem bellek sızıntısı önlenir hem de esnek bir ilişki kurulmuş olur.

---

## Dersin Özeti ve Kapanış

Bugün Rust'ın akıllı işaretçiler dünyasında uzun bir yolculuk yaptık. Gördük ki bu araçlar, Rust'ın katı derleme zamanı kurallarını aşmamıza değil, bu kurallarla uyum içinde çalışarak bellek güvenliğini korurken daha karmaşık veri yapıları kurmamıza olanak tanıyor.

*   **`Box<T>`** ile büyük verileri heap'e taşıdık, tekrarlayan türleri çözdük.
*   **`Deref` ve `Drop` trait'leri** sayesinde akıllı işaretçilerin referans gibi davranmasını ve kaynakları otomatik temizlemesini sağladık.
*   **`Rc<T>`** ile bir veriye birden çok sahip atayarak paylaşımlı yapılar inşa ettik.
*   **`RefCell<T>`** ile işleri bir adım öteye taşıyıp, dışarıdan değişmez görünen bir referansla bile iç veriyi güvenli bir şekilde değiştirdik.
*   **`Rc<RefCell<T>>`** kombinasyonuyla hem çok sahipli hem değişebilir veri modelleri oluşturduk.
*   Son olarak, bu gücün sorumluluğunu hatırlayıp, **referans döngüleri** ve `Weak<T>` ile bellek sızıntılarını nasıl önleyeceğimizi öğrendik.

Akıllı işaretçiler, Rust'ın en güçlü ve en Rust'a özgü konseptlerinden biridir. Onları ne zaman ve nasıl kullanacağınızı bilmek, sizi çok daha yetkin bir Rust geliştiricisi yapacaktır. Bir sonraki derste görüşmek üzere, kodlamaya devam edin!