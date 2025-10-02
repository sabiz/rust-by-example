// A function `age` which returns a `u32`.
// `age`関数は`u32`の値を返す。
fn age() -> u32 {
    15
}

fn some_number() -> Option<u32> {
    Some(42)
}

fn main() {
    println!("Tell me what type of person you are");

    match age() {
        0             => println!("I haven't celebrated my first birthday yet"),
        // Could `match` 1 ..= 12 directly but then what age
        // would the child be? Instead, bind to `n` for the
        // sequence of 1 ..= 12. Now the age can be reported.
        // `1 ... 12`の値を一挙に`match`させることができる。
        // しかしその場合、子供は正確には何歳?
        // マッチした値を`n`にバインディングすることで値を使用できる。
        n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        // Nothing bound. Return the result.
        // マッチしなかった場合の処理
        n             => println!("I'm an old person of age {:?}", n),
    }

    match some_number() {
        // Got `Some` variant, match if its value, bound to `n`,
        // is equal to 42.
        // `n`にバインドされた`Some`の値が42に等しいときにマッチ。
        Some(n @ 42) => println!("The Answer: {}!", n),
        // Match any other number.
        // それ以外の数値にマッチ。
        Some(n)      => println!("Not interesting... {}", n),
        // Match anything else (`None` variant).
        // それ以外にマッチ（`None`の値）。
        _            => (),
    }
}
