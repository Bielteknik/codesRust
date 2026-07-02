# 🦀 Ders Notları: Bölüm 14 - Cargo

Merhaba! Rust'ın kalbine, yani paket yöneticisi ve yapı sistemi olan **Cargo**'nun derinliklerine hoş geldin. Bugün, resmi Rust Kitabı'nın (The Rust Programming Language Book) 14. Bölümü olan **"More About Cargo and Crates.io" (Cargo ve Crates.io Hakkında Daha Fazlası)** konusunu senin için bir "Bootcamp" dersi formatında, en ince ayrıntısına kadar işleyeceğiz.

Bu dersin sonunda; projelerini nasıl optimize edeceğini, kendi kütüphanelerini (crate) dünyaya nasıl açacağını, büyük projeleri nasıl yöneteceğini ve komut satırı araçlarını nasıl kuracağını öğreneceksin.

Hazırsan, defterini ve kod editörünü kap, dersimize başlıyoruz! 🚀

---

# Bölüm 14: Cargo ve Crates.io Hakkında Daha Fazlası

Şu ana kadar Cargo'yu temel olarak projemizdeki bağımlılıkları indirmek ve kodumuzu derlemek (`cargo build`) için kullandık. Ancak Cargo, basit bir derleyiciden çok daha fazlasıdır. Bu bölümde Cargo'nun "ustalık" seviyesindeki özelliklerini inceleyeceğiz.

Dersimizin 5 ana alt başlığı var:
1. **Sürüm Profilleri (Release Profiles)**
2. **Bir Sandığı (Crate) Crates.io'da Yayınlamak**
3. **Cargo Çalışma Alanları (Cargo Workspaces)**
4. **İkili (Binary) Dosyaları `cargo install` ile Yüklemek**
5. **Cargo'yu Özel Komutlarla Genişletmek**

---

## 1. Sürüm Profilleri (Release Profiles)

Geliştirme yaparken kodumuzun *hızlı* derlenmesini isteriz ki yazdığımız hatayı hemen görebilelim. Ancak ürünümüzü (production) canlıya alırken kodun *hızlı* çalışmasını ve az yer kaplamasını isteriz. İşte **Sürüm Profilleri** tam burada devreye girer.

Cargo, derleme sürecini kontrol etmemiz için farklı profiller sunar. En çok kullanılan iki profil şunlardır:

*   **`dev` Profili:** `cargo build` yazdığında kullanılır. Geliştirme için optimize edilmiştir (Derleme hızlıdır, çalışma zamanı yavaştır).
*   **`release` Profili:** `cargo build --release` yazdığında kullanılır. Kullanıcıya sunulacak son sürüm içindir (Derleme yavaştır, ancak optimizasyonlar sayesinde kod çok hızlı çalışır).

### Profilleri Özelleştirmek
Cargo varsayılan ayarlarla gelir, ancak `Cargo.toml` dosyana ekleyerek bu ayarları değiştirebilirsin. Örneğin, geliştirme ortamında bile optimizasyon seviyesini artırmak istersen:

```toml
# Cargo.toml
[profile.dev]
opt-level = 0 # Geliştirme için varsayılan (Optimizasyon yok, hata ayıklama bilgisi tam)

[profile.release]
opt-level = 3 # Canlı ortam için varsayılan (Maksimum hız optimizasyonu)
```
*💡 **İpucu:** `opt-level` 0'dan 3'e kadar değerler alır. 3 en yüksek optimizasyondur ancak derleme süresini ciddi oranda uzatır.*

---

## 2. Crates.io'da Bir Sandık (Crate) Yayınlamak

Rust ekosistemi **Crates.io** üzerinde kuruludur. Sen de yazdığın harika kodları tüm dünyayla paylaşabilirsin. Ancak bir kütüphane yayınlamak sadece kodu yüklemek değildir; iyi bir dokümantasyon ve API tasarımı gerektirir.

### A. Dokümantasyon Yorumları (Documentation Comments)
Normalde kod içine `//` ile yorum yazarız. Ancak bir kütüphane yazıyorsan, kütüphaneni kullanacak olan geliştiricilere HTML formatında şık bir dokümantasyon sunman gerekir. Bunun için **üç eğik çizgi `///`** kullanırız. Rust, Markdown formatını destekler.

```rust
/// Verilen sayıya bir ekler.
///
/// # Örnekler
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

Bu dokümantasyonu HTML olarak oluşturmak ve tarayıcıda görmek için terminalde şu komutu çalıştırırız:
👉 `cargo doc --open`

#### Modül Düzeyinde Dokümantasyon (`//!`)
Eğer yazdığın şey bir fonksiyon değil de, **tüm modülü veya sandığı** (crate) açıklayan bir giriş yazısıysa `//!` kullanırız.
```rust
//! # Sanatçıların Yönetimi
//!
//! `artist` sanatçıları yönetmek için bir araçtır...
```

### B. Ortak API'yi Dışa Aktarmak (`pub use`)
Kütüphanenin iç yapısı (modüller ağacı) çok karmaşık olabilir. Kullanıcıların `kutuphane::modul1::alt_modul::fonksiyon` gibi uzun uzadıya yollar yazmasını istemezsin.
`pub use` komutu, bir öğeyi içe aktarıp (use) aynı anda dışarıya açık (public) hale getirir. Böylece kullanıcıların API'sini "düzleştirip" (flatten) kolaylık sağlarsın.

### C. Yayınlama Süreci (Publishing)
1.  **Hesap Oluştur:** [Crates.io](https://crates.io) üzerinden GitHub hesabınla giriş yap.
2.  **API Token Al:** Hesap ayarlarından yeni bir API anahtarı (token) oluştur.
3.  **Terminalde Giriş Yap:** `cargo login <api-anahtarin>`
4.  **Meta Verileri Doldur:** `Cargo.toml` dosyasında `description` (açıklama) ve `license` (lisans - örn: "MIT OR Apache-2.0") alanlarını doldurmak **zorunludur**.
5.  **Yayınla:** `cargo publish` komutunu çalıştır. Tebrikler, artık kodun dünyada! 🌍

### D. Sürüm Güncellemek ve "Yank" (Geri Çekme)
*   **Güncelleme:** Kodunu değiştirdin, `Cargo.toml`'daki `version = "0.1.0"` kısmını `"0.1.1"` (SemVer kurallarına göre) yap ve tekrar `cargo publish` çalıştır. Eski sürümler asla silinmez, üzerine yazılmaz.
*   **Yank (Geri Çekme):** Bir sürümde kritik bir hata (bug) buldun ve kimsenin **yeni projelerinde** bu sürümü indirmesini istemiyorsun.
    `cargo yank --vers 0.1.1` komutunu kullanırsın.
    *Önemli Not:* `yank` komutu kodu sunucudan **silmez**. Sadece bağımlılık çözümleyicinin (resolver) bu sürümü *yeni* indirmesini engeller. Zaten bu sürümü kullanıp `Cargo.lock` dosyası oluşturmuş projeler sorunsuz derlenmeye devam eder. Geri almak için `cargo yank --vers 0.1.1 --undo` yapabilirsin.

---

## 3. Cargo Çalışma Alanları (Cargo Workspaces)

Büyük bir proje yazıyorsun diyelim. Bu proje birden fazla kütüphaneden ve bir ana çalıştırılabilir dosyadan oluşuyor. Eğer her biri için ayrı klasörler açarsan, her birinin kendi `Cargo.lock` ve devasa `target` (derleme) klasörü olur. Diskin dolar, bağımlılıkları senkronize tutmak kâbus olur.

**Çözüm: Workspaces (Çalışma Alanları)**
Birden fazla paketi tek bir çatı altında toplar, hepsinin **aynı `Cargo.lock` dosyasını** ve **aynı `target` klasörünü** paylaşmasını sağlar.

### Çalışma Alanı Oluşturmak
Kök dizinde bir `Cargo.toml` oluştururuz:

```toml
# Kök dizindeki Cargo.toml
[workspace]
members = [
    "adder",      # Ana projemiz (binary)
    "add-one",    # Yardımcı kütüphanemiz (library)
]
```

Daha sonra `adder` ve `add-one` klasörlerini oluşturup içlerinde `cargo new` ve `cargo new --lib` ile projeleri yaratırız. `adder`'ın `Cargo.toml` dosyasına, `add-one`'u bir bağımlılık (path dependency) olarak ekleriz:

```toml
[dependencies]
add-one = { path = "../add-one" }
```

Kök dizinde `cargo build` yazdığında Cargo tek bir `target` klasörü oluşturur ve her iki projeyi aynı anda senkronize bir şekilde derler. Testleri toplu koşmak için `cargo test`, sadece `add-one` paketinde koşmak için `cargo test -p add-one` yazabilirsin.

---

## 4. İkili (Binary) Dosyaları `cargo install` ile Yüklemek

`cargo build` sadece *kendi* projeni derler. Peki ya internette bulduğun harika bir Rust komut satırı (CLI) aracını (örneğin `ripgrep` veya `bat`) bilgisayarına kalıcı olarak kurmak istersen?

İşte burada `cargo install` devreye girer.

*   **Ne yapar?** Crates.io'dan paketi indirir, derler ve çalıştırılabilir dosyayı (binary) sistemindeki `~/.cargo/bin/` (veya Windows'ta `%USERPROFILE%\.cargo\bin\`) klasörüne kopyalar.
*   **Şartı nedir?** İndirilen sandığın `src/main.rs` dosyasına (yani bir binary crate olmasına) ihtiyacı vardır. Library (kütüphane) paketleri `cargo install` ile kurulamaz.
*   **Kullanımı:** `cargo install ripgrep` yazıp enter'a bastığında, Rust arka planda kodu derler ve `rg` komutunu terminalinde her yerden kullanabilir hale gelirsin.

---

## 5. Cargo'yu Özel Komutlarla Genişletmek (Custom Commands)

Rust topluluğu Cargo'yu o kadar esnek tasarlamıştır ki, kendi özel komutlarını yazıp Cargo'nun bir parçasıymış gibi kullanabilirsin.

Eğer `PATH` (sistem yolu) içinde `cargo-bise` adında çalıştırılabilir bir dosya varsa, Cargo bunu otomatik olarak tanır. Terminalde:
`cargo bise` yazdığında, Cargo aslında arka planda `cargo-bise` programını çalıştırır ve ona `bise` argümanını iletir.

**Örnek Popüler Özel Komutlar:**
*   `cargo-expand`: Rust'ın makrolarını derlemeden önce nasıl genişlettiğini (expand) görmek için kullanılır.
*   `cargo-watch`: Kodunda bir değişiklik yaptığında otomatik olarak yeniden derleme yapar.
*   `cargo-tree`: Projenin bağımlılık ağacını görselleştirir.

Bu sayede Cargo, sadece Rust'a ait değil, tüm ekosistemin katkılarıyla büyüyen devasa bir araca dönüşür.

---

## Dersin Özeti (Kapanış)

Bugün Cargo'nun sadece bir `build` butonundan ibaret olmadığını gördük. Özetlersek:
1.  **Release Profilleri** ile geliştirme ve canlı ortam optimizasyonlarını ayırırız.
2.  **Dokümantasyon Yorumları (`///`)** ve **`pub use`** ile profesyonel, kullanımı kolay kütüphaneler yazar, **Crates.io** üzerinden dünyaya açarız.
3.  **Workspaces** ile büyük, çoklu paketlerden oluşan projeleri tek bir `target` klasörü altında ustalıkla yönetiriz.
4.  **`cargo install`** ile harika Rust araçlarını sistemimize kurarız.
5.  **Özel Komutlar** ile Cargo'nun yeteneklerini sınırsızca genişletebiliriz.

**Ödev:** Bugün öğrendiklerini pekiştirmek için basit bir matematik kütüphanesi yaz, içine `///` ile dokümantasyon ekle ve `cargo login` komutuyla Crates.io'da kendi isminle yayınla! 

Başka bir sorunuz veya derinleşmemizi istediğiniz bir detay olursa sormaktan çekinmeyin. İyi kodlamalar! 🦀

Harika bir konu seçimi! Rust'ın paket yöneticisi **Cargo**, sadece kodunuzu derleyen bir araçtan çok daha fazlasıdır; tüm Rust ekosistemini yönetmenizi sağlayan, oldukça yetenekli ve güçlü bir sistemdir. Rust Book'un **14. Bölümü: Cargo ve Crates (Paketler) Hakkında Daha Fazlası** kısmını senin için bir ders formatında, tüm detaylarıyla ve mantığını kavrayarak işleyeceğiz.

Hazırsan kahveni al, Rust ekosisteminin derinliklerine doğru yolculuğumuza başlıyoruz! 🚀

---

# DERS 1: Release Profilleri (Sürüm Profilleri) ile Derlemeleri Özelleştirme

Geliştirme yaparken kodumuzun hızlı derlenmesini isteriz, ancak uygulamayı canlıya (production) alırken kodun mümkün olduğunca hızlı ve optimize çalışmasını bekleriz. İşte Cargo, bu iki farklı ihtiyacı karşılamak için **Release Profilleri** (Sürüm Profilleri) sunar.

Cargo'nun iki ana profili vardır:
1. **`dev` Profili:** `cargo build` yazdığınızda arka planda çalışan profildir. Geliştirme için iyi varsayılanlarla donatılmıştır. Çıktısında `[unoptimized + debuginfo]` yazar.
2. **`release` Profili:** `cargo build --release` yazdığınızda çalışan profildir. Canlıya çıkmak (yayınlamak) için optimize edilmiştir. Çıktısında `[optimized]` yazar.

### Optimizasyon Seviyesi (`opt-level`)
Rust derleyicisi kodunuzu ne kadar optimize edeceğini `opt-level` ile belirler. Bu değer 0 ile 3 arasındadır.
* **Geliştirme (dev) aşamasında:** Sürekli kod yazıp derlediğiniz için derleme süresinin kısa olması önemlidir. Bu yüzden varsayılan `opt-level = 0`'dır (Optimizasyon yok).
* **Yayınlama (release) aşamasında:** Kodu sadece bir kez derlersiniz ama kullanıcılar bu kodu milyonlarca kez çalıştırır. Bu yüzden derleme süresi biraz uzasa bile kodun çok hızlı çalışması için varsayılan `opt-level = 3`'tür (Maksimum optimizasyon).

**Cargo.toml Dosyasında Özelleştirme Yapmak:**
Eğer geliştirme aşamasında bile biraz optimizasyon isterseniz, `Cargo.toml` dosyanıza şu satırları ekleyerek varsayılan ayarı ezebilirsiniz:

```toml
[profile.dev]
opt-level = 1 # Varsayılan 0'dır, biz biraz hızlandırdık.

[profile.release]
opt-level = 3 # Varsayılan zaten 3'tür.
```

---

# DERS 2: Crates.io Üzerinde Bir Paket (Crate) Yayınlamak

Kodunuzu yazdınız ve bunun tüm dünyadaki Rust geliştiricileri tarafından kullanılmasını istiyorsunuz. Bunu **crates.io** üzerinden yapabilirsiniz. Ancak yayınlamadan önce kodunuzun "kullanıcı dostu" olması gerekir.

### 1. Dokümantasyon Yorumları (Documentation Comments)
Rust'ta standart yorum satırları `//` ile başlar. Ancak dokümantasyon üreten özel yorum satırları `///` (üç slash) ile başlar ve **Markdown** formatını destekler.

* **`///` (Öğe Dokümantasyonu):** Hemen altında bulunduğu fonksiyonu veya yapıyı açıklar.
* **`cargo doc`:** Bu komut, yazdığınız yorumları alıp şık bir HTML web sitesine dönüştürür ve `target/doc` klasörüne kaydeder.
* **Test Edilebilir Örnekler:** Dokümantasyonun içine kod blokları (```rust) eklerseniz, `cargo test` komutunu çalıştırdığınızda **Cargo bu örnek kodları birer testmiş gibi çalıştırır.** Bu, dokümantasyonunuzun asla eskimemesini ve her zaman çalışmasını garanti eden muazzam bir özelliktir!

```rust
/// Verilen sayıya bir ekler.
///
/// # Örnekler
///
/// ```
/// let arg = 5;
/// let cevap = my_crate::add_one(arg);
/// assert_eq!(6, cevap);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

* **`//!` (Modül/Kasa Dokümantasyonu):** Üç slash ve ünlem işareti ile başlar. Altındaki öğeyi değil, **içinde bulunduğu modülü veya tüm kasayı** açıklar. Genelde `src/lib.rs` dosyasının en başına yazılarak kütüphanenin genel amacı anlatılır.

### 2. Genel API'yi Temizlemek (`pub use`)
Büyük projelerde kodunuzu derin klasörlere (modüllere) bölersiniz. Örneğin kullanıcının `my_crate::network::http::client::connect()` yazması zordur. `pub use` (yeniden dışa aktarım) kullanarak, kullanıcının sadece `my_crate::connect()` yazmasını sağlayabilir ve arka plandaki karmaşık hiyerarşiyi gizleyebilirsiniz.

### 3. Yayınlama Adımları
1. **Hesap Açma:** crates.io'ya GitHub hesabınızla giriş yapın.
2. **API Anahtarı:** Hesap ayarlarından API anahtarınızı alın ve terminalde `cargo login` komutunu çalıştırıp anahtarı yapıştırın.
3. **Metadata (Üstveri):** `Cargo.toml` dosyanıza **açıklama (description)** ve **lisans (license)** (Örn: `MIT`) eklemek zorunludur.
4. **Yayınlama:** `cargo publish` komutuyla kasanız tüm dünyaya açılır!

### 4. Bir Sürümü Geri Çekmek (Yanking)
Yanlışlıkla hatalı bir sürüm yayınladınız. Crates.io'da eski sürümleri **silemezsiniz** (çünkü bunu kullanan projelerin çökmemesi gerekir). Ancak `cargo yank --vers 1.0.1` komutu ile o sürümü "yanked" (geri çekilmiş) olarak işaretleyebilirsiniz. Bu, **yeni başlayan projelerin** bu hatalı sürümü indirip bağımlılık olarak eklemesini engeller, ancak halihazırda kullanmakta olan projeler etkilenmez.

---

# DERS 3: Cargo Workspaces (Çalışma Alanları)

Projeniz büyüdükçe tek bir kasa (crate) içine binlerce satır kod tıkıştırmak imkansızlaşır. Kodunuzu birden fazla kütüphaneye bölmek istersiniz. İşte tam burada **Workspaces (Çalışma Alanları)** devreye girer.

Bir workspace, aynı `Cargo.lock` dosyasını ve aynı `target/` (derleme çıktıları) dizinini paylaşan birbiriyle ilişkili paketler kümesidir.

### Neden Workspace Kullanırız?
Eğer her kasanın kendi `target` klasörü olsaydı, kasa A, kasa B'yi bağımlılık olarak kullandığında B'yi sıfırdan derlemek zorunda kalırdı. Workspace sayesinde tek bir ortak `target` klasörü kullanılır, böylece **diskten tasarruf eder ve derleme sürelerini inanılmaz hızlandırırsınız.** Ayrıca tek bir `Cargo.lock` dosyası olduğu için tüm alt projeleriniz aynı bağımlılık sürümlerini kullanır ve uyumsuzluk (versiyon çakışması) yaşanmaz.

### Nasıl Kurulur?
Ana dizinde bir `Cargo.toml` oluşturursunuz ancak içine `[package]` değil, `[workspace]` yazarsınız:

```toml
[workspace]
resolver = "3"
members = ["adder", "add_one"] # Alt kasaların klasör isimleri
```

Alt klasörlerde `cargo new` ile kasalarınızı oluşturursunuz.
**Önemli Not:** Workspace içindeki kasalar birbirini otomatik olarak görmez. Bir kasanın diğerini kullanabilmesi için kendi `Cargo.toml` dosyasında bağımlılık olarak açıkça belirtmesi gerekir:

```toml
[dependencies]
add_one = { path = "../add_one" } # Yerel dizin bağımlılığı
```

**Test Etmek:**
Ana dizinde `cargo test` yazdığınızda workspace içindeki **tüm** kasaların testleri çalışır. Sadece belirli bir kasayı test etmek isterseniz `-p` (package) parametresini kullanırsınız:
`cargo test -p add_one`

---

# DERS 4: `cargo install` ile Araçları (Binary) Yüklemek

Cargo sadece kütüphaneleri değil, doğrudan terminalden çalışan komut satırı araçlarını (CLI tools) da yönetmenizi sağlar.

* Sistem paket yöneticileri (apt, brew vb.) yerine Rust ile yazılmış araçları yüklemek için `cargo install` kullanılır.
* Bu komut **sadece `src/main.rs` dosyasına sahip (çalıştırılabilir/binary) olan kasaları** yükler. Kütüphaneler yüklenemez.
* Yüklenen araçlar bilgisayarınızda `~/.cargo/bin` klasörüne atılır.
* **Çok Önemli:** Bu araçları terminalin her yerinden çağırabilmek için `~/.cargo/bin` dizininin sistem `$PATH` değişkeninize eklenmiş olması gerekir (Rustup ile kurduysanız muhtemelen eklenmiştir).

**Örnek:** Rust ile yazılmış ve çok hızlı olan dosya arama aracı `ripgrep`'i yüklemek:
```bash
cargo install ripgrep
# Kurulum bittikten sonra terminalden direkt 'rg' komutunu kullanabilirsiniz.
```

---

# DERS 5: Özel Komutlarla Cargo'yu Genişletmek (Custom Commands)

Cargo'nun en sevilen özelliklerinden biri de **açık kaynaklı ve genişletilebilir** olmasıdır. Eğer bir geliştirici `cargo-bloat` adında bir araç yazar ve bunu crates.io'da paylaşırsa (veya siz kendiniz yazarsanız), Cargo bunu otomatik olarak kendi alt komutuymuş gibi kullanmanıza izin verir.

**Kural şudur:** Sisteminizin `$PATH` dizininde `cargo-` ile başlayan bir çalıştırılabilir dosya varsa (Örn: `cargo-bloat`), Cargo bu dosyayı bulur ve sizin terminalde `cargo bloat` yazmanıza olanak tanır.

Bu sayede Cargo'nun yetenekleri topluluk tarafından yazılan eklentilerle sınırsızca genişletilebilir. Örneğin kodunuzun hangi kısımlarının ne kadar yer kapladığını analiz eden araçlardan, kod formatlayıcılara kadar her şeyi bu mantıkla Cargo'ya entegre edebilirsiniz.

---

### Özet ve Ders Sonu 🎓
Bugün Cargo'nun sadece `build` ve `run` komutlarından ibaret olmadığını öğrendik:
1. **Profiller** ile kodumuzun canlıda ne kadar hızlı çalışacağını kontrol etmeyi,
2. **Dokümantasyon** yazarak kodumuzu dünyaya profesyonelce sunmayı,
3. **Workspace** mantığı ile devasa projeleri modüler ve hızlı yönetmeyi,
4. **`cargo install`** ile harika Rust araçlarını bilgisayarımıza kurmayı öğrendik.

Bu konular Rust'ta "Junior" seviyesinden "Mid-Level" ve "Senior" seviyesine geçişin en önemli kilometre taşlarıdır. Projelerinde bu özellikleri kullanarak çok daha profesyonel işler ortaya koyabilirsin. Başka bir bölümü veya konuyu merak edersen sormaktan çekinme! İyi kodlamalar! 🦀💙