use std::mem;

// This function borrows a slice.
// この関数はスライスを借用する
fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn main() {
    // Fixed-size array (type signature is superfluous).
    // 固定長の配列（型シグネチャは冗長なので、なくても可）
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value.
    // すべての要素を0にする場合
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0.
    // インデックスは０から
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    // `len` returns the count of elements in the array.
    // `len`は配列の要素数を返す。
    println!("Number of elements in array: {}", xs.len());

    // Arrays are stack allocated.
    // 配列はスタック上に置かれる
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices.
    // 配列は自動的にスライスとして借用される。
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    // Slices can point to a section of an array.
    // They are of the form [starting_index..ending_index].
    // `starting_index` is the first position in the slice.
    // `ending_index` is one more than the last position in the slice.
    // スライスは配列の一部を指すことができる。
    // [starting_index..ending_index] の形をとり、
    // `starting_index` はスライスの先頭の位置を表し、
    // `ending_index` はスライスの末尾の1つ先の位置を表す。
    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1 .. 4]);

    // Example of empty slice `&[]`:
    // 空のスライスの例：`&[]`
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // Same but more verbose
                                       // 同じ意味だがより冗長な書き方

    // Arrays can be safely accessed using `.get`, which returns an
    // `Option`. This can be matched as shown below, or used with
    // `.expect()` if you would like the program to exit with a nice
    // message instead of happily continue.
    // 配列は、`Option`を返す`.get`で安全にアクセスできます。
    // `Option`は以下のようにマッチさせることもできますし、
    // 運よく処理を続ける代わりに、`.expect()`で素敵なメッセージとともに
    // プログラムを終了することもできます。
    for i in 0..xs.len() + 1 { // Oops, one element too far!
                               // おっと、1要素余分!
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

    // Out of bound indexing on array causes compile time error.
    // 配列のインデックスが範囲外のときはコンパイルエラー
    //println!("{}", xs[5]);
    // Out of bound indexing on slice causes runtime error.
    // スライスのインデックスが範囲外のときはランタイムエラー
    //println!("{}", xs[..][5]);
}
