📌 Yazılım ve bilgisayar bilimlerinde en sık kullanılan veri yapıları:

Array → Dizi

Linked List → Bağlı liste
Stack → Yığın
Queue → Kuyruk
Hash Table / Hash Map → Hash tablosu / eşleme
Tree → Ağaç
Graph → Grafik

Bu terimler genelde "Data Structures and Algorithms" (Veri Yapıları ve Algoritmalar) başlığı altında öğretilir.

Rust'ta **Veri Yapıları (Data Structures)**, verileri organize etmenin, depolamanın ve verimli bir şekilde erişmenin yollarını sağlar. Rust'ın standart kütüphanesi bazı veri yapılarını doğrudan sunarken, bazılarını kendiniz implemente etmeniz veya harici kütüphaneler kullanmanız gerekir.

İşte Rust'ta temel veri yapılarının detaylı anlatımı:

---

## 1. Linked List (Bağlı Liste)

### Nedir?
Linked List, her bir elemanın (node) hem veriyi hem de bir sonraki elemana işaretçi (pointer) tuttuğu doğrusal bir veri yapısıdır. Array'lerin aksine bellekte kesintisiz olmak zorunda değildir.

### Rust'ta Durum
⚠️ **Önemli:** Rust'ın standart kütüphanesinde `LinkedList` tipi **vardır** ancak **neredeyse hiç kullanılmaz**. Çünkü:
- Cache dostu değildir (bellekte dağınıktır)
- Her eleman için ayrı heap tahsisi gerekir
- `Vec` genellikle daha hızlıdır

Ancak öğrenme amaçlı implemente etmek Rust'ın ownership ve borrowing kavramlarını anlamak için mükemmeldir.

### Manuel Implementasyon (Tek Yönlü)

```rust
use std::fmt;

// Node yapısı
#[derive(Debug)]
struct Node<T> {
    veri: T,
    sonraki: Option<Box<Node<T>>>,
}

// Linked List yapısı
#[derive(Debug)]
struct LinkedList<T> {
    baslangic: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    fn yeni() -> Self {
        LinkedList { baslangic: None }
    }

    // Başa eleman ekle
    fn basa_ekle(&mut self, veri: T) {
        let yeni_node = Box::new(Node {
            veri,
            sonraki: self.baslangic.take(),
        });
        self.baslangic = Some(yeni_node);
    }

    // Elemanları yazdır
    fn yazdir(&self) 
    where 
        T: fmt::Debug 
    {
        let mut mevcut = &self.baslangic;
        print!("[");
        while let Some(node) = mevcut {
            print!("{:?}", node.veri);
            mevcut = &node.sonraki;
            if mevcut.is_some() {
                print!(" -> ");
            }
        }
        println!("]");
    }
}

fn main() {
    let mut liste = LinkedList::yeni();
    liste.basa_ekle(3);
    liste.basa_ekle(2);
    liste.basa_ekle(1);
    
    liste.yazdir(); // Çıktı: [1 -> 2 -> 3]
}
```

### Ne Zaman Kullanılır?
- Çok sık ekleme/çıkarma yapılıyorsa ve indeksleme gerekmiyorsa
- Bellek fragmentasyonu sorun değilse
- **Genel kural:** Çoğu durumda `Vec` kullanın!

---

## 2. Stack (Yığın)

### Nedir?
**LIFO (Last In, First Out)** prensibiyle çalışan veri yapısıdır. Son eklenen eleman ilk çıkar.

### Rust'ta Durum
Rust'ta **ayrı bir Stack tipi yoktur**. Bunun yerine `Vec` kullanılır çünkü `Vec`'in `push()` ve `pop()` metodları tam olarak stack davranışı gösterir.

### Vec ile Stack Kullanımı

```rust
fn main() {
    let mut stack = Vec::new();

    // Push (Ekleme)
    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("Stack: {:?}", stack); // [1, 2, 3]

    // Pop (Çıkarma - son eklenen çıkar)
    let ust = stack.pop(); // Some(3)
    println!("Çıkarılan: {:?}", ust);

    // Peek (Üst elemana bakma, çıkarmadan)
    if let Some(ust_deger) = stack.last() {
        println!("Üst eleman: {}", ust_deger); // 2
    }

    // Boş mu kontrolü
    println!("Boş mu? {}", stack.is_empty()); // false
}
```

### Kullanım Alanları
- Fonksiyon çağrıları (Call stack)
- Parantez eşleştirme `((()))`
- Geri alma (Undo) mekanizmaları
- DFS (Depth-First Search) algoritması

---

## 3. Queue (Kuyruk)

### Nedir?
**FIFO (First In, First Out)** prensibiyle çalışan veri yapısıdır. İlk eklenen eleman ilk çıkar.

### Rust'ta Durum
Rust'ın standart kütüphanesinde **`VecDeque`** (Vector Double-Ended Queue) vardır ve queue olarak kullanılır. `Vec`'in `pop_front()` metodu olmadığı için `VecDeque` tercih edilir.

### VecDeque ile Queue Kullanımı

```rust
use std::collections::VecDeque;

fn main() {
    let mut queue = VecDeque::new();

    // Enqueue (Kuyruğa ekleme - sona ekler)
    queue.push_back(1);
    queue.push_back(2);
    queue.push_back(3);

    println!("Queue: {:?}", queue); // [1, 2, 3]

    // Dequeue (Kuyruktan çıkarma - baştan çıkar)
    let ilk = queue.pop_front(); // Some(1)
    println!("Çıkarılan: {:?}", ilk);

    // Peek (İlk elemana bakma)
    if let Some(ilk_deger) = queue.front() {
        println!("İlk eleman: {}", ilk_deger); // 2
    }

    // Son elemana bakma
    if let Some(son_deger) = queue.back() {
        println!("Son eleman: {}", son_deger); // 3
    }

    println!("Uzunluk: {}", queue.len()); // 2
}
```

### Kullanım Alanları
- BFS (Breadth-First Search) algoritması
- İş kuyrukları (Job queues)
- Mesaj kuyrukları (Message queues)
- Print spooler

---

## 4. Hash Table / HashMap (Hash Tablosu)

### Nedir?
Anahtar-değer (key-value) çiftlerini saklayan, **O(1)** ortalama erişim süresi sunan veri yapısıdır.

### Rust'ta Durum
Rust'ın standart kütüphanesinde **`HashMap`** vardır ve çok güçlüdür. Anahtar tipi `Hash` ve `Eq` trait'lerini implemente etmelidir.

### HashMap Kullanımı

```rust
use std::collections::HashMap;

fn main() {
    let mut puanlar = HashMap::new();

    // Ekleme
    puanlar.insert("Ali", 95);
    puanlar.insert("Ayşe", 87);
    puanlar.insert("Mehmet", 92);

    // Erişim
    if let Some(alinin_puani) = puanlar.get("Ali") {
        println!("Ali'nin puanı: {}", alinin_puani); // 95
    }

    // Olmayan anahtar
    match puanlar.get("Veli") {
        Some(puan) => println!("Puan: {}", puan),
        None => println!("Öğrenci bulunamadı"),
    }

    // Güncelleme veya varsayılan değer
    *puanlar.entry("Ali").or_insert(0) += 5; // Ali'nin puanını 5 artır

    // Silme
    puanlar.remove("Mehmet");

    // Tüm elemanları gezme
    for (isim, puan) in &puanlar {
        println!("{}: {}", isim, puan);
    }

    println!("Toplam öğrenci: {}", puanlar.len());
}
```

### Gelişmiş Özellikler

```rust
use std::collections::HashMap;

fn main() {
    let mut kelime_sayaci = HashMap::new();
    let metin = "merhaba dünya merhaba rust merhaba";

    // Kelime sayacı (Frequency counter)
    for kelime in metin.split_whitespace() {
        let sayac = kelime_sayaci.entry(kelime).or_insert(0);
        *sayac += 1;
    }

    println!("{:?}", kelime_sayaci);
    // {"merhaba": 3, "dünya": 1, "rust": 1}

    // Vec değerleri (Bir anahtara birden fazla değer)
    let mut gruplar = HashMap::new();
    gruplar.entry("A Grubu").or_insert_with(Vec::new).push("Ali");
    gruplar.entry("A Grubu").or_insert_with(Vec::new).push("Ayşe");
    
    println!("{:?}", gruplar);
    // {"A Grubu": ["Ali", "Ayşe"]}
}
```

### Kullanım Alanları
- Veritabanı indeksleri
- Cache sistemleri
- Kelime sayacı, frekans analizi
- JSON parsing
- Graph implementasyonları (adjacency list)

---

## 5. Tree (Ağaç)

### Nedir?
Hiyerarşik bir veri yapısıdır. Her node bir veri ve çocuk node'lara (children) sahiptir. En yaygın türü **Binary Tree** (İkili Ağaç) ve **Binary Search Tree (BST)**'dir.

### Rust'ta Durum
Standart kütüphanede **Tree yapısı yoktur**. Kendiniz implemente etmeniz veya harici kütüphane kullanmanız gerekir.

### Binary Search Tree (BST) Implementasyonu

```rust
use std::cmp::Ordering;

#[derive(Debug)]
struct Node<T: Ord> {
    veri: T,
    sol: Option<Box<Node<T>>>,
    sag: Option<Box<Node<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T: Ord> {
    kok: Option<Box<Node<T>>>,
}

impl<T: Ord> BinarySearchTree<T> {
    fn yeni() -> Self {
        BinarySearchTree { kok: None }
    }

    fn ekle(&mut self, veri: T) {
        self.kok = Some(Self::ekle_rekursive(self.kok.take(), veri));
    }

    fn ekle_rekursive(node: Option<Box<Node<T>>>, veri: T) -> Box<Node<T>> {
        match node {
            None => Box::new(Node {
                veri,
                sol: None,
                sag: None,
            }),
            Some(mut mevcut) => {
                match veri.cmp(&mevcut.veri) {
                    Ordering::Less => {
                        mevcut.sol = Some(Self::ekle_rekursive(mevcut.sol.take(), veri));
                    }
                    Ordering::Greater => {
                        mevcut.sag = Some(Self::ekle_rekursive(mevcut.sag.take(), veri));
                    }
                    Ordering::Equal => {} // Duplicate'ları görmezden geliyoruz
                }
                mevcut
            }
        }
    }

    fn ara(&self, veri: &T) -> bool {
        Self::ara_rekursive(&self.kok, veri)
    }

    fn ara_rekursive(node: &Option<Box<Node<T>>>, veri: &T) -> bool {
        match node {
            None => false,
            Some(mevcut) => {
                match veri.cmp(&mevcut.veri) {
                    Ordering::Equal => true,
                    Ordering::Less => Self::ara_rekursive(&mevcut.sol, veri),
                    Ordering::Greater => Self::ara_rekursive(&mevcut.sag, veri),
                }
            }
        }
    }

    // In-order traversal (Sıralı yazdırma)
    fn inorder(&self) {
        Self::inorder_rekursive(&self.kok);
        println!();
    }

    fn inorder_rekursive(node: &Option<Box<Node<T>>>) {
        if let Some(mevcut) = node {
            Self::inorder_rekursive(&mevcut.sol);
            print!("{:?} ", mevcut.veri);
            Self::inorder_rekursive(&mevcut.sag);
        }
    }
}

fn main() {
    let mut bst = BinarySearchTree::yeni();
    
    bst.ekle(50);
    bst.ekle(30);
    bst.ekle(70);
    bst.ekle(20);
    bst.ekle(40);
    bst.ekle(60);
    bst.ekle(80);

    println!("In-order traversal (sıralı):");
    bst.inorder(); // 20 30 40 50 60 70 80

    println!("40 var mı? {}", bst.ara(&40)); // true
    println!("45 var mı? {}", bst.ara(&45)); // false
}
```

### Harici Kütüphaneler
- **`std::collections::BTreeMap`**: Dengeli Binary Search Tree (HashMap'e alternatif, sıralı)
- **`petgraph`**: Graph ve Tree yapıları için

### Kullanım Alanları
- Veritabanı indeksleri (B-Tree)
- Dosya sistemi hiyerarşisi
- Huffman coding (sıkıştırma)
- Decision trees (makine öğrenmesi)

---

## 6. Graph (Grafik/Çizge)

### Nedir?
Node'lar (vertices) ve bunları bağlayan edge'lerden oluşan non-linear veri yapısıdır. Sosyal ağlar, haritalar, internet gibi ilişkisel verileri modellemek için kullanılır.

### Rust'ta Durum
Standart kütüphanede **Graph yapısı yoktur**. İki şekilde implemente edebilirsiniz:
1. Adjacency List (Komşuluk Listesi) - HashMap ile
2. `petgraph` kütüphanesi ile

### Adjacency List ile Graph Implementasyonu

```rust
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Graph {
    komsuluk_listesi: HashMap<String, HashSet<String>>,
}

impl Graph {
    fn yeni() -> Self {
        Graph {
            komsuluk_listesi: HashMap::new(),
        }
    }

    fn kenar_ekle(&mut self, kaynak: String, hedef: String) {
        self.komsuluk_listesi
            .entry(kaynak)
            .or_insert_with(HashSet::new)
            .insert(hedef);
    }

    fn cift_yonlu_kenar_ekle(&mut self, node1: String, node2: String) {
        self.kenar_ekle(node1.clone(), node2.clone());
        self.kenar_ekle(node2, node1);
    }

    fn komsulari_getir(&self, node: &str) -> Option<&HashSet<String>> {
        self.komsuluk_listesi.get(node)
    }

    // BFS (Breadth-First Search)
    fn bfs(&self, baslangic: &str) {
        use std::collections::VecDeque;

        let mut ziyaret_edilen = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(baslangic.to_string());
        ziyaret_edilen.insert(baslangic.to_string());

        print!("BFS Traversal: ");
        while let Some(mevcut) = queue.pop_front() {
            print!("{} ", mevcut);

            if let Some(komsular) = self.komsuluk_listesi.get(&mevcut) {
                for komsu in komsular {
                    if !ziyaret_edilen.contains(komsu) {
                        ziyaret_edilen.insert(komsu.clone());
                        queue.push_back(komsu.clone());
                    }
                }
            }
        }
        println!();
    }
}

fn main() {
    let mut graf = Graph::yeni();

    // Sosyal ağ örneği
    graf.cift_yonlu_kenar_ekle("Ali".to_string(), "Ayşe".to_string());
    graf.cift_yonlu_kenar_ekle("Ali".to_string(), "Mehmet".to_string());
    graf.cift_yonlu_kenar_ekle("Ayşe".to_string(), "Veli".to_string());
    graf.cift_yonlu_kenar_ekle("Mehmet".to_string(), "Veli".to_string());
    graf.cift_yonlu_kenar_ekle("Veli".to_string(), "Fatma".to_string());

    println!("Ali'nin arkadaşları: {:?}", graf.komsulari_getir("Ali"));

    graf.bfs("Ali");
    // BFS Traversal: Ali Ayşe Mehmet Veli Fatma
}
```

### petgraph Kütüphanesi ile Graph

```toml
# Cargo.toml
[dependencies]
petgraph = "0.6"
```

```rust
use petgraph::graph::UnGraph;
use petgraph::visit::Bfs;

fn main() {
    // Undirected Graph (Yönsüz grafik)
    let mut graf = UnGraph::<&str, f32>::new_undirected();

    let a = graf.add_node("A");
    let b = graf.add_node("B");
    let c = graf.add_node("C");
    let d = graf.add_node("D");

    graf.add_edge(a, b, 1.0);
    graf.add_edge(a, c, 2.0);
    graf.add_edge(b, d, 3.0);
    graf.add_edge(c, d, 4.0);

    // BFS ile traversal
    let mut bfs = Bfs::new(&graf, a);
    print!("BFS: ");
    while let Some(nx) = bfs.next(&graf) {
        print!("{} ", graf[nx]);
    }
    println!();
}
```

### Kullanım Alanları
- Sosyal ağlar (Facebook arkadaşları)
- Harita ve navigasyon (Google Maps)
- Internet (web sayfaları ve linkler)
- Tavsiye sistemleri
- Network routing

---

## Karşılaştırma Tablosu

| Veri Yapısı | Rust'ta Karşılığı | Ekleme | Arama | Silme | Kullanım Yeri |
|:---|:---|:---|:---|:---|:---|
| **Linked List** | `LinkedList` (nadiren kullanılır) | O(1) | O(n) | O(1) | Sık ekleme/çıkarma, indeksleme gerekmez |
| **Stack** | `Vec` (push/pop) | O(1) | O(n) | O(1) | LIFO gerektiren işlemler |
| **Queue** | `VecDeque` | O(1) | O(n) | O(1) | FIFO gerektiren işlemler |
| **HashMap** | `HashMap` | O(1)* | O(1)* | O(1)* | Anahtar-değer eşleştirme |
| **Tree** | `BTreeMap` (veya manuel) | O(log n) | O(log n) | O(log n) | Sıralı veri, hiyerarşi |
| **Graph** | `petgraph` (veya manuel) | - | O(V+E) | - | İlişkisel veriler, ağlar |

*Ortalama durum, worst case farklı olabilir

---

## Özet ve İpuçları

1. **Vec Her Şeydir:** Rust'ta çoğu zaman `Vec` yeterli olur. Stack, Queue, hatta basit Linked List ihtiyaçlarınız için `Vec` kullanın.

2. **HashMap Varsayılan Seçiminiz Olsun:** Anahtar-değer eşleştirmesi gerekiyorsa, sıralama önemli değilse `HashMap` kullanın.

3. **Sıralı Veri İçin BTreeMap:** Eğer anahtarların sıralı olması gerekiyorsa veya range queries yapacaksanız `BTreeMap` kullanın.

4. **Graph için petgraph:** Ciddi graph işlemleri için kendi implementasyonunuz yerine `petgraph` kütüphanesini kullanın.

5. **Linked List'ten Kaçının:** Rust'ta `LinkedList` neredeyse hiç kullanılmaz. `Vec` veya `VecDeque` genellikle daha performanslıdır.

6. **Ownership'a Dikkat:** Tree ve Graph implementasyonlarında Rust'ın ownership kuralları nedeniyle `Rc<RefCell<T>>` veya `Box` kullanmanız gerekebilir. Bu, Rust'ta veri yapıları implement etmenin en zor kısmıdır.

7. **Harici Crate'lerden Korkmayın:** Rust ekosistemi çok zengindir. `petgraph`, `ndarray`, `indexmap` gibi kütüphaneler production-ready'dir ve güvenle kullanılabilir.