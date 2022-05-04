#[derive(Eq, PartialEq)]
struct A(i32);

#[derive(PartialEq, PartialOrd)]
struct B(f32);

#[derive(Clone, Copy)]
struct C;

#[derive(Clone)]
struct D;

#[derive(Debug)]
struct E;

#[derive(Default)]
struct F;

fn main() {
    // Aは比較一致可能
    println!("{:?}", A(0) == A(1));

    // Bは大証比較可能
    println!("{:?}", B(1.0) > B(0.0));

    // Cはムーブではなくコピーされる
    let c0 = C;
    let _c1 = c0;
    let _c2 = c0; // Cがムーブなら、c0は_c1へムーブしているはずなので、ここでコンパイルエラー

    // Dはclone可能
    let d0 = D;
    let _d1 = d0.clone();

    // Eはデバッグプリント可能
    println!("{:?}", E);

    // Fはdefault可能
    let _f = F::default();
}
