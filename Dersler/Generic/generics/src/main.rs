/*
### Mevcut Kodunuz:
``` 
struct Point {
    x: f64,
    y: f64,
}
```
Bu struct sadece `f64` (ondalıklı sayı) tipinde koordinatları saklayabilir.

### Önerilen Generic Versiyon:
``` 
struct Point<T> {
    x: T,
    y: T,
}
```

## Fark Nedir?

**`T`** bir **generic tip parametresidir**. Bu sayede struct'ınızı farklı tiplerle kullanabilirsiniz:

``` 
// Generic versiyon ile:
let p1 = Point { x: 5.0, y: 10.0 };        // T = f64
let p2 = Point { x: 5i32, y: 10i32 };      // T = i32
let p3 = Point { x: 5u64, y: 10u64 };      // T = u64

// Ama mevcut versiyonunuzla sadece:
let p1 = point { x: 5.0, y: 10.0 };        // Sadece f64 çalışır
// let p2 = point { x: 5, y: 10 };         // ❌ HATA! i32 kullanamazsınız
```

## Özetle:
- **`→`** işareti: IDE'nin "bunu kullanabilirsin" önerisi
- **`T`**: Herhangi bir tip (Type'ın kısaltması)
- **`x: T, y: T`**: x ve y alanları aynı generic tipte olacak

Eğer struct'ınızı sadece `f64` ile kullanacaksanız mevcut haliyle bırakabilirsiniz. Ancak farklı sayı tipleriyle de kullanmak isterseniz generic versiyonu tercih edebilirsiniz.
*/

struct PointA {
    x:i32,
    y:i32,
}

struct PointB<T> {
    x:T,
    y:T,
}

struct PointC<T, U> {
    x:T,
    y:U,
}

struct Line<T,V> {
    start: PointC<T,V>,
    end: PointC<T,V>,
}

fn main() {

    let a = PointA{x: 3, y:18};
    let b = PointB{x: 4.4, y:8.2};
    let c = PointC{x: 5, y: 10.0};
    let d = PointC{x: 5, y: "Hello"};
    let line = Line { start: PointC{x:1, y:2}, end: PointC{x:3, y:4} };

    println!("a = ({}, {})", a.x, a.y);
    println!("b = ({}, {})", b.x, b.y);
    println!("c = ({}, {})", c.x, c.y);
    println!("d = ({}, {})", d.x, d.y);
    println!("line = ({}, {})", line.start.x, line.start.y);
    println!("line = ({}, {})", line.end.x, line.end.y);

}
