fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`. Destructure to `i32`.
    // ベクトル型に対する`iter`は`&i32`を`yield`するので、`i32`へとデストラクト
    println!("2 in vec1: {}", vec1.iter()     .any(|&x| x == 2));
    // `into_iter()` for vecs yields `i32`. No destructuring required.
    // `into_iter()`の場合は`i32`を`yield`するので、デストラクトする必要はない。
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    // `iter()` only borrows `vec1` and its elements, so they can be used again
    // `iter()`は`vec1`とその要素を借用するだけなので、再び利用できる。
    println!("vec1 len: {}", vec1.len());
    println!("First element of vec1 is: {}", vec1[0]);
    // `into_iter()` does move `vec2` and its elements, so they cannot be used again
    // `into_iter()`は`vec2`とその要素をムーブするので、再利用することはできない。
    // println!("First element of vec2 is: {}", vec2[0]);
    // println!("vec2 len: {}", vec2.len());
    // TODO: uncomment two lines above and see compiler errors.
    // TODO: 上の2行をアンコメントしてコンパイルエラーになることを確認

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` for arrays yields `&i32`.
    // 配列に対する`iter()`は`&i32`をyieldする。
    println!("2 in array1: {}", array1.iter()     .any(|&x| x == 2));
    // `into_iter()` for arrays yields `i32`.
    // 配列に`into_iter()`を使うと`&i32`を`yield`する。
    println!("2 in array2: {}", array2.into_iter().any(|x| *x == 2));
}
