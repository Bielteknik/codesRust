# ⚡ Rust for .NET Developers: Benchmarking (Performans Ölçümü)

Bu bölüm, Rust'ta performans ölçümü (benchmarking) yapmanın temellerinden ileri düzey tekniklere kadar kapsamlı bir rehberdir. C# dünyasında **BenchmarkDotNet** ile performans testleri yazmaya alışkınız. Rust ise hem yerleşik (nightly) hem de **Criterion** gibi güçlü harici araçlar sunar.

> 🎯 **Temel Fark:** C#'ta BenchmarkDotNet "kraldır" ve neredeyse tek seçenektir. Rust'ta ise iki yol vardır: **Nightly `#[bench]`** (eski, kararsız) ve **Criterion** (modern, stabil, önerilen).

---

# 📚 BÖLÜM 1: Benchmarking Nedir ve Neden Önemlidir?

## 1.1 Tanım

**Benchmarking**, kodunuzun belirli bölümlerinin **performansını ölçmek**, **darboğazları (bottleneck) tespit etmek** ve **optimizasyonların etkisini quantifiye etmek** için kullanılan tekniktir.

## 1.2 C# vs Rust Yaklaşımı

| Özellik | C# | Rust |
|---|---|---|
| Ana araç | BenchmarkDotNet | Criterion (önerilen) / `#[bench]` (nightly) |
| Kurulum | NuGet paketi | Cargo dependency |
| İstatistiksel analiz | ✅ Var | ✅ Var (Criterion) |
| Regression detection | ✅ Var | ✅ Var |
| HTML rapor | ✅ Var | ✅ Var (Criterion) |
| CI/CD entegrasyonu | ✅ Var | ✅ Var |
| Zero-cost abstraction | ❌ JIT overhead | ✅ Derleme zamanı optimizasyonu |

## 1.3 Neden Benchmark Yazmalıyız?

1. **Darboğazları bulmak:** Kodun neresinin yavaş olduğunu görmek
2. **Optimizasyonları doğrulamak:** "Daha hızlı oldu mu?" sorusuna bilimsel cevap
3. **Regression'ları yakalamak:** Yeni kodun performansı düşürmesini önlemek
4. **Mimari kararlar:** Veri yapısı/algoritma seçimi için kanıt
5. **SLA uyumu:** Gerçek zamanlı sistemlerde (RP2354B gibi) zamanlama garantisi

---

# 📚 BÖLÜM 2: Yöntem 1 - Nightly `#[bench]` (Eski Yöntem)

> ⚠️ **Uyarı:** Bu yöntem **nightly Rust** gerektirir ve kararsızdır. Production projeler için **önerilmez**. Sadece bilgi amaçlı gösterilmektedir.

## 2.1 Kurulum

```bash
# Nightly toolchain kur
rustup toolchain install nightly

# Nightly'yi varsayılan yap (opsiyonel)
rustup default nightly
```

## 2.2 Temel Kullanım

```rust
#![feature(test)]  // Nightly feature gerekli

#[cfg(test)]
mod bench {
    extern crate test;
    use test::Bencher;
    
    #[bench]
    fn bench_toplama(b: &mut Bencher) {
        b.iter(|| {
            let mut toplam = 0;
            for i in 0..1000 {
                toplam += i;
            }
            toplam
        });
    }
    
    #[bench]
    fn bench_iterator(b: &mut Bencher) {
        b.iter(|| {
            (0..1000).sum::<i32>()
        });
    }
}
```

## 2.3 Çalıştırma

```bash
cargo +nightly bench
```

**Örnek Çıktı:**
```
running 2 tests
test bench::bench_toplama ... bench:         642 ns/iter (+/- 28)
test bench::bench_iterator ... bench:         412 ns/iter (+/- 15)
```

## 2.4 Sorunlar

- ❌ Nightly Rust gerektirir
- ❌ İstatistiksel analiz zayıf
- ❌ Regression detection yok
- ❌ HTML rapor yok
- ❌ Stabil değil, her sürümde değişebilir

> 💡 **Tavsiye:** Bu yöntemi **kullanmayın**. Bunun yerine Criterion kullanın.

---

# 📚 BÖLÜM 3: Yöntem 2 - Criterion (Önerilen) ⭐

**Criterion**, Rust'ın **BenchmarkDotNet'i** olarak düşünülebilir. Stabil Rust'ta çalışır, gelişmiş istatistiksel analiz sunar ve detaylı raporlar üretir.

## 3.1 Kurulum

**Cargo.toml**:
```toml
[package]
name = "benchmark_projesi"
version = "0.1.0"
edition = "2021"

[dependencies]
# Normal bağımlılıklar

[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "benches"
harness = false  # ÖNEMLİ: Criterion kendi test harness'ını kullanır
```

## 3.2 Proje Yapısı

```
benchmark_projesi/
├── src/
│   └── lib.rs
├── benches/
│   └── benches.rs         # Benchmark dosyası
├── Cargo.toml
└── target/
    └── criterion/         # Raporlar buraya yazılır
        └── report/
            └── index.html
```

## 3.3 İlk Benchmark

**src/lib.rs**:
```rust
pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

pub fn fibonacci_iterative(n: u64) -> u64 {
    if n == 0 { return 0; }
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}
```

**benches/benches.rs**:
```rust
use criterion::{criterion_group, criterion_main, Criterion};
use benchmark_projesi::{fibonacci, fibonacci_iterative};

fn bench_fibonacci(c: &mut Criterion) {
    c.bench_function("fibonacci 20", |b| {
        b.iter(|| fibonacci(20))
    });
}

fn bench_fibonacci_iterative(c: &mut Criterion) {
    c.bench_function("fibonacci_iterative 20", |b| {
        b.iter(|| fibonacci_iterative(20))
    });
}

criterion_group!(benches, bench_fibonacci, bench_fibonacci_iterative);
criterion_main!(benches);
```

## 3.4 Çalıştırma

```bash
cargo bench
```

**Örnek Çıktı:**
```
fibonacci 20              time:   [120.45 µs 121.23 µs 122.01 µs]
fibonacci_iterative 20    time:   [85.23 ns 85.67 ns 86.12 ns]
```

> 🎯 **Görülüyor:** Iterative versiyon, recursive versiyondan **~1400 kat** daha hızlı!

## 3.5 HTML Raporu

Criterion otomatik olarak detaylı HTML raporlar üretir:

```bash
# Raporu aç
open target/criterion/report/index.html
# veya
xdg-open target/criterion/report/index.html
```

Rapor şunları içerir:
- 📊 Zaman dağılımı grafiği
- 📈 Performans eğrisi
- 📉 Outlier analizi
- 📋 İstatistiksel özet (mean, median, std dev)

---

# 📚 BÖLÜM 4: BenchmarkDotNet ile Detaylı Karşılaştırma

## 4.1 C# BenchmarkDotNet Örneği

**C#**:
```csharp
using BenchmarkDotNet.Attributes;
using BenchmarkDotNet.Running;

public class FibonacciBenchmark
{
    [Params(10, 20, 30)]
    public int N { get; set; }
    
    [Benchmark(Baseline = true)]
    public long Recursive() => FibRecursive(N);
    
    [Benchmark]
    public long Iterative() => FibIterative(N);
    
    private long FibRecursive(int n)
    {
        if (n <= 1) return n;
        return FibRecursive(n - 1) + FibRecursive(n - 2);
    }
    
    private long FibIterative(int n)
    {
        long a = 0, b = 1;
        for (int i = 0; i < n; i++)
        {
            long temp = a + b;
            a = b;
            b = temp;
        }
        return b;
    }
}

// Çalıştırma
var summary = BenchmarkRunner.Run<FibonacciBenchmark>();
```

## 4.2 Rust Criterion ile Eşdeğer

**Rust**:
```rust
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

fn fib_recursive(n: u64) -> u64 {
    match n {
        0 | 1 => n,
        _ => fib_recursive(n - 1) + fib_recursive(n - 2),
    }
}

fn fib_iterative(n: u64) -> u64 {
    if n <= 1 { return n; }
    let (mut a, mut b) = (0, 1);
    for _ in 0..n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

fn bench_fibonacci(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci");
    
    for i in [10, 20, 30].iter() {
        group.bench_with_input(BenchmarkId::new("recursive", i), i, |b, &i| {
            b.iter(|| fib_recursive(i))
        });
        
        group.bench_with_input(BenchmarkId::new("iterative", i), i, |b, &i| {
            b.iter(|| fib_iterative(i))
        });
    }
    
    group.finish();
}

criterion_group!(benches, bench_fibonacci);
criterion_main!(benches);
```

## 4.3 Özellik Karşılaştırması

| Özellik | BenchmarkDotNet | Criterion |
|---|---|---|
| Parametreli test | `[Params]` | `BenchmarkId::new()` |
| Baseline | `[Benchmark(Baseline = true)]` | `compare_function()` |
| Setup/Cleanup | `[GlobalSetup]` | Closure içinde |
| Gruplama | `[GroupBenchmarksBy]` | `benchmark_group()` |
| Memory allocation | `[MemoryDiagnoser]` | Ölçülmez (farklı yaklaşım) |
| Statistical tests | Welch's t-test | Student's t-test |
| Output formats | Markdown, HTML, CSV | HTML, CSV, bencher |

---

# 📚 BÖLÜM 5: Grup Benchmarkları ve Parametreli Testler

## 5.1 Benchmark Grupları

Benzer benchmarkları gruplamak, karşılaştırmayı kolaylaştırır:

```rust
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

fn bench_string_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_islemleri");
    
    // Farklı boyutlarda test
    for size in [10, 100, 1000, 10000].iter() {
        let test_string: String = "a".repeat(*size);
        
        group.bench_with_input(
            BenchmarkId::new("to_uppercase", size),
            &test_string,
            |b, s| b.iter(|| s.to_uppercase())
        );
        
        group.bench_with_input(
            BenchmarkId::new("to_lowercase", size),
            &test_string,
            |b, s| b.iter(|| s.to_lowercase())
        );
        
        group.bench_with_input(
            BenchmarkId::new("trim", size),
            &test_string,
            |b, s| b.iter(|| s.trim())
        );
    }
    
    group.finish();
}

criterion_group!(benches, bench_string_operations);
criterion_main!(benches);
```

## 5.2 Compare Function (Karşılaştırma)

İki veya daha fazla fonksiyonu doğrudan karşılaştırmak:

```rust
use criterion::{criterion_group, criterion_main, Criterion, black_box};

fn linear_search(arr: &[i32], target: i32) -> Option<usize> {
    for (i, &val) in arr.iter().enumerate() {
        if val == target { return Some(i); }
    }
    None
}

fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    arr.binary_search(&target).ok()
}

fn bench_search(c: &mut Criterion) {
    let data: Vec<i32> = (0..10000).collect();
    
    c.bench_function("linear_search", |b| {
        b.iter(|| linear_search(black_box(&data), 9999))
    });
    
    c.bench_function("binary_search", |b| {
        b.iter(|| binary_search(black_box(&data), 9999))
    });
    
    // Doğrudan karşılaştırma
    c.bench_function("compare", |b| {
        b.iter(|| {
            let _ = linear_search(black_box(&data), 9999);
            let _ = binary_search(black_box(&data), 9999);
        })
    });
}

criterion_group!(benches, bench_search);
criterion_main!(benches);
```

---

# 📚 BÖLÜM 6: `black_box` ve Doğru Ölçüm

## 6.1 `black_box` Nedir?

Rust derleyicisi çok akıllıdır ve kullanılmayan kodu **optimize edebilir**. `black_box`, derleyiciye "bu değeri kullanıyorum, optimize etme" mesajı verir.

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_with_black_box(c: &mut Criterion) {
    c.bench_function("with_black_box", |b| {
        b.iter(|| {
            // black_box, derleyicinin bu kodu optimize etmesini engeller
            let result = black_box(42);
            black_box(result * 2)
        })
    });
}

criterion_group!(benches, bench_with_black_box);
criterion_main!(benches);
```

## 6.2 Neden Önemli?

**Yanlış** (Derleyici optimize edebilir):
```rust
c.bench_function("yanlis", |b| {
    b.iter(|| {
        let x = fibonacci(20);  // x kullanılmıyor, derleyici silebilir!
    })
});
```

**Doğru**:
```rust
use criterion::black_box;

c.bench_function("dogru", |b| {
    b.iter(|| {
        let x = black_box(fibonacci(20));  // Kullanılmak zorunda
        black_box(x)
    })
});
```

## 6.3 C# Karşılığı

C#'ta BenchmarkDotNet otomatik olarak bu optimizasyonu engeller. Rust'ta ise **manuel** olarak `black_box` kullanmalısınız.

---

# 📚 BÖLÜM 7: Regression Detection (Gerileme Tespiti)

Criterion'ın en güçlü özelliklerinden biri: **önceki sonuçlarla otomatik karşılaştırma**.

## 7.1 Nasıl Çalışır?

1. İlk çalıştırmada Criterion sonuçları `target/criterion/` içine kaydeder
2. Sonraki çalıştırmalarda yeni sonuçları eski sonuçlarla karşılaştırır
3. İstatistiksel olarak anlamlı bir fark varsa **regression uyarısı** verir

## 7.2 Örnek

```bash
# İlk çalıştırma (baseline oluşturulur)
cargo bench

# Kodu değiştirdiniz...

# İkinci çalıştırma
cargo bench
```

**Çıktı (regression varsa)**:
```
fibonacci 20              time:   [150.23 µs 151.45 µs 152.67 µs]
                          change: [+24.567% +25.123% +25.678%] (p = 0.0000 < 0.05)
                          Performance has regressed.
```

## 7.3 Baseline Kaydetme

Belirli bir versiyonu baseline olarak kaydedebilirsiniz:

```bash
# Baseline kaydet
cargo bench -- --save-baseline v1.0

# Kodu değiştir...

# v1.0 ile karşılaştır
cargo bench -- --baseline v1.0
```

## 7.4 CI/CD'de Regression Detection

GitHub Actions ile otomatik regression tespiti:

```yaml
name: Benchmark

on:
  push:
    branches: [ main ]
  pull_request:

jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Run benchmarks
        run: cargo bench -- --save-baseline current
      
      - name: Compare with main
        if: github.event_name == 'pull_request'
        run: |
          git fetch origin main
          git checkout origin/main
          cargo bench -- --save-baseline main
          cargo bench -- --baseline main
```

---

# 📚 BÖLÜM 8: İstatistiksel Analiz ve Güven Aralıkları

## 8.1 Criterion'ın İstatistiksel Yaklaşımı

Criterion, **bootstrap resampling** yöntemiyle güven aralıkları hesaplar:

```
fibonacci 20              time:   [120.45 µs 121.23 µs 122.01 µs]
                                ^          ^          ^
                              lower      mean       upper
                              (95% CI)   (point)    (95% CI)
```

## 8.2 Örnek Çıktı Analizi

```
fibonacci 20
  time:   [120.45 µs 121.23 µs 122.01 µs]
  thrpt:  [8.1958 MiB/s 8.2487 MiB/s 8.3021 MiB/s]
                 ^^^ throughput (veri işleme hızı)
  change:
    time:   [-0.2345% -0.1234% +0.0123%] (p = 0.05 > 0.05)
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
            İstatistiksel olarak anlamlı değil
  Found 3 outliers among 100 measurements (3.00%)
    2 (2.00%) high mild
    1 (1.00%) high severe
```

## 8.3 Outlier (Aykırı Değer) Analizi

Criterion otomatik olarak outlier'ları tespit eder ve raporlar:
- **Mild outlier:** Hafif aykırı değer
- **Severe outlier:** Ciddi aykırı değer

Outlier'lar genellikle:
- Background process'lerden kaynaklanır
- CPU throttling
- Cache miss
- GC (Rust'ta yok ama sistem GC'si olabilir)

## 8.4 Sample Size ve Warmup

Criterion'ın varsayılan ayarları:

```rust
use criterion::{Criterion, SamplingMode};

fn bench_config(c: &mut Criterion) {
    c.bench_function("config", |b| {
        b.iter(|| fibonacci(20))
    });
}

// Özelleştirilmiş Criterion
let criterion = Criterion::default()
    .sample_size(100)        // Varsayılan: 100
    .warm_up_time(std::time::Duration::from_secs(3))  // Varsayılan: 3s
    .measurement_time(std::time::Duration::from_secs(5))  // Varsayılan: 5s
    .sampling_mode(SamplingMode::Auto);

criterion_group! {
    name = benches;
    config = criterion;
    targets = bench_config
}
criterion_main!(benches);
```

---

# 📚 BÖLÜM 9: Throughput (Veri İşleme Hızı) Ölçümü

## 9.1 Bytes/Second Ölçümü

Veri işleme fonksiyonları için throughput ölçümü:

```rust
use criterion::{criterion_group, criterion_main, Criterion, Throughput};

fn compress(data: &[u8]) -> Vec<u8> {
    // Sıkıştırma algoritması
    data.to_vec()  // Örnek
}

fn bench_compression(c: &mut Criterion) {
    let data = vec![0u8; 1024 * 1024];  // 1 MB
    
    let mut group = c.benchmark_group("compression");
    group.throughput(Throughput::Bytes(data.len() as u64));
    
    group.bench_function("compress 1MB", |b| {
        b.iter(|| compress(black_box(&data)))
    });
    
    group.finish();
}

criterion_group!(benches, bench_compression);
criterion_main!(benches);
```

**Çıktı:**
```
compression/compress 1MB
  time:   [1.2345 ms 1.2456 ms 1.2567 ms]
  thrpt:  [795.73 MiB/s 802.82 MiB/s 809.98 MiB/s]
          ^^^^^^^^^ throughput!
```

## 9.2 Elements/Second Ölçümü

Koleksiyon işlemleri için:

```rust
use criterion::{criterion_group, criterion_main, Criterion, Throughput};

fn process_items(items: &[i32]) -> i32 {
    items.iter().sum()
}

fn bench_processing(c: &mut Criterion) {
    let items: Vec<i32> = (0..10000).collect();
    
    let mut group = c.benchmark_group("processing");
    group.throughput(Throughput::Elements(items.len() as u64));
    
    group.bench_function("sum 10000 items", |b| {
        b.iter(|| process_items(black_box(&items)))
    });
    
    group.finish();
}

criterion_group!(benches, bench_processing);
criterion_main!(benches);
```

---

# 📚 BÖLÜM 10: Profiling Entegrasyonu

## 10.1 Flamegraph ile Profiling

Criterion, flamegraph üretimi için entegrasyon sunar:

```bash
# Linux'ta perf ile
cargo bench --bench benches -- --profile-time 5

# Flamegraph oluşturulur:
# target/criterion/<benchmark_name>/profile/flamegraph.svg
```

## 10.2 inferno ile Flamegraph

```bash
cargo install inferno

# Benchmark çalıştır ve perf data topla
cargo bench --bench benches -- --profile-time 5

# Flamegraph oluştur
cat target/criterion/*/profile/perf.data | \
  inferno-collapse-perf | \
  inferno-flamegraph > flamegraph.svg
```

## 10.3 valgrind/callgrind Entegrasyonu

```bash
# valgrind kur
sudo apt-get install valgrind

# Benchmark'ı valgrind ile çalıştır
cargo bench --bench benches -- --profile-time 5

# callgrind ile
valgrind --tool=callgrind ./target/release/deps/benches-*
kcachegrind callgrind.out.*
```

---

# 📚 BÖLÜM 11: Pratik Örnekler

## 11.1 Koleksiyon Karşılaştırması

```rust
use criterion::{criterion_group, criterion_main, Criterion, black_box};
use std::collections::{HashMap, BTreeMap};

fn bench_hashmap_insert(c: &mut Criterion) {
    c.bench_function("HashMap insert 1000", |b| {
        b.iter(|| {
            let mut map = HashMap::new();
            for i in 0..1000 {
                map.insert(i, i);
            }
            black_box(map)
        })
    });
}

fn bench_btreemap_insert(c: &mut Criterion) {
    c.bench_function("BTreeMap insert 1000", |b| {
        b.iter(|| {
            let mut map = BTreeMap::new();
            for i in 0..1000 {
                map.insert(i, i);
            }
            black_box(map)
        })
    });
}

criterion_group!(benches, bench_hashmap_insert, bench_btreemap_insert);
criterion_main!(benches);
```

## 11.2 String vs &str Performansı

```rust
use criterion::{criterion_group, criterion_main, Criterion, black_box};

fn process_string(s: String) -> usize {
    s.len() + s.capacity()
}

fn process_str(s: &str) -> usize {
    s.len()
}

fn bench_string_vs_str(c: &mut Criterion) {
    let test_data = "a".repeat(1000);
    
    c.bench_function("String", |b| {
        b.iter(|| process_string(black_box(test_data.clone())))
    });
    
    c.bench_function("&str", |b| {
        b.iter(|| process_str(black_box(&test_data)))
    });
}

criterion_group!(benches, bench_string_vs_str);
criterion_main!(benches);
```

## 11.3 Parallel vs Sequential

```rust
use criterion::{criterion_group, criterion_main, Criterion};
use rayon::prelude::*;

fn sequential_sum(data: &[i32]) -> i32 {
    data.iter().sum()
}

fn parallel_sum(data: &[i32]) -> i32 {
    data.par_iter().sum()
}

fn bench_parallel_vs_sequential(c: &mut Criterion) {
    let data: Vec<i32> = (0..1_000_000).collect();
    
    c.bench_function("sequential", |b| {
        b.iter(|| sequential_sum(black_box(&data)))
    });
    
    c.bench_function("parallel", |b| {
        b.iter(|| parallel_sum(black_box(&data)))
    });
}

criterion_group!(benches, bench_parallel_vs_sequential);
criterion_main!(benches);
```

---

# 📚 BÖLÜM 12: Embedded Benchmarking (RP2354B için) 🎯

Step motor projeniz gibi embedded sistemlerde benchmarking özel teknikler gerektirir.

## 12.1 Zorluklar

- ❌ Criterion host'ta çalışır, hedef donanımda değil
- ❌ `std` kütüphanesi olmayabilir (`no_std`)
- ❌ Zamanlama hassasiyeti kritik

## 12.2 Çözüm: Mantıksal Benchmarking

Donanım bağımsız mantığı host'ta benchmark edin:

```rust
// src/motor.rs
pub fn calculate_steps(distance_mm: f64, steps_per_rev: u32, mm_per_rev: f64) -> u32 {
    ((distance_mm / mm_per_rev) * steps_per_rev as f64) as u32
}

pub fn calculate_speed_profile(total_steps: u32, max_speed: u32, acceleration: u32) -> Vec<u32> {
    // Hız profili hesaplama (saf mantık, donanım bağımsız)
    let mut profile = Vec::new();
    let mut speed = 0;
    let mut steps_done = 0;
    
    while steps_done < total_steps {
        if speed < max_speed {
            speed += acceleration;
        }
        profile.push(speed);
        steps_done += 1;
    }
    
    profile
}

// benches/motor_benches.rs
use criterion::{criterion_group, criterion_main, Criterion};
use motor_projesi::{calculate_steps, calculate_speed_profile};

fn bench_calculate_steps(c: &mut Criterion) {
    c.bench_function("calculate_steps", |b| {
        b.iter(|| calculate_steps(100.0, 200, 5.0))
    });
}

fn bench_speed_profile(c: &mut Criterion) {
    c.bench_function("speed_profile 1000 steps", |b| {
        b.iter(|| calculate_speed_profile(1000, 500, 10))
    });
}

criterion_group!(benches, bench_calculate_steps, bench_speed_profile);
criterion_main!(benches);
```

## 12.3 Hedef Donanımda Benchmarking

Gerçek donanımda zamanlama için **cycle counter** kullanın:

```rust
// RP2354B için (ARM Cortex-M33)
#[cfg(target_arch = "arm")]
use cortex_m::peripheral::DWT;

#[cfg(target_arch = "arm")]
pub fn benchmark_adim_at() -> u32 {
    unsafe {
        let dwt = &(*cortex_m::peripheral::DWT::ptr());
        
        // Cycle counter'ı sıfırla
        dwt.cyccnt.write(0);
        
        // Test edilen kod
        adim_at();
        
        // Geçen cycle sayısı
        dwt.cyccnt.read()
    }
}
```

## 12.4 defmt ile Zamanlama

```rust
use defmt::info;
use embassy_time::{Instant, Duration};

#[embassy_executor::task]
async fn motor_benchmark() {
    let start = Instant::now();
    
    for _ in 0..1000 {
        adim_at();
    }
    
    let elapsed = start.elapsed();
    info!("1000 adım: {} µs", elapsed.as_micros());
}
```

## 12.5 Host vs Target Benchmark Stratejisi

```
┌─────────────────────────────────────────────────────────┐
│ Host Benchmarking (Criterion)                           │
│ ├─ Algoritma performansı                                │
│ ├─ Veri yapısı seçimi                                   │
│ ├─ Matematiksel hesaplamalar                            │
│ └─ Hız profili üretimi                                  │
└─────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│ Target Benchmarking (Cycle Counter)                     │
│ ├─ GPIO toggle hızı                                     │
│ ├─ Interrupt latency                                    │
│ ├─ Step pulse timing                                    │
│ └─ Gerçek zamanlı davranış                              │
└─────────────────────────────────────────────────────────┘
```

---

# 📚 BÖLÜM 13: Benchmark Best Practices

## 13.1 ✅ İyi Pratikler

1. **`black_box` Kullanın:**
```rust
// ✅ İyi
b.iter(|| black_box(fibonacci(20)));

// ❌ Kötü
b.iter(|| fibonacci(20));  // Derleyici optimize edebilir
```

2. **Setup'ı `iter` Dışında Yapın:**
```rust
// ✅ İyi
let data = setup_data();
b.iter(|| process(black_box(&data)));

// ❌ Kötü
b.iter(|| {
    let data = setup_data();  // Her iterasyonda setup!
    process(&data);
});
```

3. **Yeterli Sample Size:**
```rust
// Varsayılan 100 sample genellikle yeterli
// Ama çok hızlı fonksiyonlar için artırın
Criterion::default().sample_size(200)
```

4. **Warmup'a İzin Verin:**
```rust
// İlk birkaç iterasyon cache warming için harcanır
// Criterion otomatik yapar ama custom benchmark'larda dikkat edin
```

5. **Gerçekçi Veri Kullanın:**
```rust
// ✅ İyi: Gerçek dünya verisi
let data = read_real_data();

// ❌ Kötü: Sentetik veri
let data = vec![0; 1000];  // Cache dostu, gerçekçi değil
```

## 13.2 ❌ Anti-Patterns

```rust
// ❌ I/O benchmark'da
b.iter(|| {
    std::fs::read_to_string("data.txt").unwrap()  // Disk I/O değişkenlik gösterir
});

// ❌ Network benchmark'da
b.iter(|| {
    reqwest::blocking::get("https://example.com").unwrap()  // Ağ gecikmesi
});

// ❌ Çok kısa benchmark
b.iter(|| {
    let x = 1 + 1;  // Ölçüm hatası >> ölçülen zaman
});

// ❌ Global state değiştirme
static mut COUNTER: i32 = 0;
b.iter(|| {
    unsafe { COUNTER += 1; }  // Thread-safe değil, sonuçlar tutarsız
});
```

---

# 📚 BÖLÜM 14: CI/CD Entegrasyonu

## 14.1 GitHub Actions

```yaml
name: Benchmark

on:
  push:
    branches: [ main ]
  pull_request:

jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Cache cargo
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Run benchmarks
        run: cargo bench -- --save-baseline current
      
      - name: Store benchmark result
        uses: benchmark-action/github-action-benchmark@v1
        with:
          tool: 'cargo'
          output-file-path: target/criterion/reports/index.html
          github-token: ${{ secrets.GITHUB_TOKEN }}
          auto-push: true
```

## 14.2 Criterion Bencher Output

GitHub Action ile uyumlu çıktı formatı:

```bash
cargo bench -- --format bencher
```

**Çıktı:**
```
test fibonacci_20 ... bench:     121234 ns/iter (+/- 5678)
test fibonacci_iterative_20 ... bench:         85 ns/iter (+/- 3)
```

## 14.3 Regression Alert

```yaml
- name: Check for regressions
  run: |
    cargo bench -- --baseline main --output-format bencher > bench_result.txt
    if grep -q "Performance has regressed" bench_result.txt; then
      echo "::error::Performance regression detected!"
      exit 1
    fi
```

---

# 🎯 ÖZET: Benchmarking Kontrol Listesi

| Özellik | C# (BenchmarkDotNet) | Rust (Criterion) |
|---|---|---|
| Kurulum | NuGet | Cargo dependency |
| Baseline | `[Benchmark(Baseline = true)]` | `--baseline` flag |
| Parametreli | `[Params]` | `BenchmarkId` |
| Gruplama | `[GroupBenchmarksBy]` | `benchmark_group()` |
| İstatistik | Welch's t-test | Bootstrap resampling |
| HTML rapor | ✅ | ✅ |
| Regression detection | ✅ | ✅ |
| Flamegraph | ✅ (native) | ✅ (perf entegrasyonu) |
| Throughput | `[MemoryDiagnoser]` | `Throughput::Bytes/Elements` |
| CI/CD | ✅ | ✅ (bencher format) |
| Embedded | ❌ | ✅ (cycle counter) |
| Zero-cost | ❌ JIT overhead | ✅ |

---

# 🚀 Son Tavsiyeler

1. **Criterion Kullanın:** Nightly `#[bench]` yerine her zaman Criterion tercih edin.

2. **`black_box` Kullanın:** Derleyicinin optimizasyonlarını engelleyin.

3. **Grup Benchmarkları:** Benzer testleri gruplayarak karşılaştırmayı kolaylaştırın.

4. **Regression Detection:** CI/CD pipeline'ınıza otomatik regression tespiti ekleyin.

5. **HTML Raporları:** Detaylı analiz için HTML raporlarını inceleyin.

6. **Throughput Ölçümü:** Veri işleme fonksiyonları için bytes/elements per second ölçün.

7. **Profiling Entegrasyonu:** Flamegraph ile darboğazları görselleştirin.

8. **Embedded Strateji:**
   - Host'ta: Criterion ile algoritma/mantık benchmark
   - Target'ta: Cycle counter ile gerçek zamanlı performans

9. **Step Motor Projeniz İçin:**
   ```
   Host Benchmark (Criterion):
   ├─ calculate_steps() - Adım hesaplama hızı
   ├─ calculate_speed_profile() - Hız profili üretimi
   ├─ trajectory_planning() - Yörünge planlama
   └─ interpolation() - Enterpolasyon algoritmaları
   
   Target Benchmark (Cycle Counter):
   ├─ GPIO toggle - Pin değiştirme hızı
   ├─ Interrupt handler - Kesme yanıt süresi
   ├─ Step pulse generation - Adım darbesi üretimi
   └─ Communication protocol - İletişim gecikmesi
   ```

10. **Benchmark Piramidi:**
    - **Mikro:** Tek fonksiyon (ns-µs)
    - **Mezo:** Modül/alt sistem (µs-ms)
    - **Makro:** Tüm sistem (ms-s)

11. **Sık Çalıştırın:** Her PR'da benchmark çalıştırın, regression'ları erken yakalayın.

12. **Gerçekçi Veri:** Sentetik veri yerine gerçek dünya verisi kullanın.

13. **Outlier'lara Dikkat:** Aykırı değerleri analiz edin, sistem arka plan process'lerini kontrol edin.

14. **`cargo bench -- --quick`**: Hızlı test için sample size'ı azaltın (geliştirme sırasında).

15. **Documentation:** Benchmark sonuçlarını dokümante edin, gelecek nesillere referans olsun.

> 🦀 **Unutmayın:** "Premature optimization is the root of all evil" (Donald Knuth). Önce **doğru** kod yazın, sonra **benchmark** ile yavaş yerleri bulun, sonra **optimize** edin. Rust'ın zero-cost abstraction'ları sayesinde çoğu zaman optimizasyona gerek kalmaz, ama ihtiyaç olduğunda Criterion ile bilimsel yaklaşım size rehberlik eder. RP2354B projenizde milisaniyelik hassasiyet için bu yaklaşım **kritik önem** taşır!