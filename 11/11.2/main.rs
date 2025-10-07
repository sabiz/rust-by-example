// extern crate rary; // May be required for Rust 2015 edition or earlier
                      // Rust 2015以前で必要

fn main() {
    rary::public_function();

    // Error! `private_function` is private
    // エラー!`private_function`はプライベート
    //rary::private_function();

    rary::indirect_access();
}
