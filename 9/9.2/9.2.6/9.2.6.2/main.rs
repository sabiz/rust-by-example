fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`.
    // ベクトル型に対する`iter`は`&i32`を`yield`する。
    let mut iter = vec1.iter();
    // `into_iter()` for vecs yields `i32`.
    // `inter_iter()`の場合は`i32`を`yield`する。
    let mut into_iter = vec2.into_iter();

    // `iter()` for vecs yields `&i32`, and we want to reference one of its
    // items, so we have to destructure `&&i32` to `i32`
    // `yield`された要素へのリファレンスは`&&i32`となる。`i32`へとデストラクトする。
    println!("Find 2 in vec1: {:?}", iter     .find(|&&x| x == 2));
    // `into_iter()` for vecs yields `i32`, and we want to reference one of
    // its items, so we have to destructure `&i32` to `i32`
    // `into_iter`の場合は`&i32`が要素のリファレンス。
    println!("Find 2 in vec2: {:?}", into_iter.find(| &x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` for arrays yields `&i32`
    // 配列に対する`iter`も`&i32`を`yield`する。
    println!("Find 2 in array1: {:?}", array1.iter()     .find(|&&x| x == 2));
    // `into_iter()` for arrays yields `i32`
    // 配列に`into_iter()`を使うと`&i32`を`yield`する。
    println!("Find 2 in array2: {:?}", array2.into_iter().find(|&x| *x == 2));


    let vec = vec![1, 9, 3, 3, 13, 2];

    // `iter()` for vecs yields `&i32` and `position()` does not take a reference, so
    // we have to destructure `&i32` to `i32`
    let index_of_first_even_number = vec.iter().position(|&x| x % 2 == 0);
    assert_eq!(index_of_first_even_number, Some(5));
    
    // `into_iter()` for vecs yields `i32` and `position()` does not take a reference, so
    // we do not have to destructure    
    let index_of_first_negative_number = vec.into_iter().position(|x| x < 0);
    assert_eq!(index_of_first_negative_number, None);
}
