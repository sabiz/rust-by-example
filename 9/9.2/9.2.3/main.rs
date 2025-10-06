// `F` must implement `Fn` for a closure which takes no
// inputs and returns nothing - exactly what is required
// for `print`.
// `F`は`Fn`を実装していなくてはならず、`Fn`は引数と返り値を持たない。
// `print`は文字をプリントするだけのクロージャなので、これが正しい。
fn apply<F>(f: F) where
    F: Fn() {
    f();
}

fn main() {
    let x = 7;

    // Capture `x` into an anonymous type and implement
    // `Fn` for it. Store it in `print`.
    // `x`を無名の構造体に入れ、それに対し`Fn`を実装する。
    // （訳注: ここでは`Fn`は`fn Fn(&self) -> {println!("{}", &self)}`）
    // その構造体を`print`にアサインする。
    let print = || println!("{}", x);

    apply(print);
}
