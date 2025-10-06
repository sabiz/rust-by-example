// This declaration will look for a file named `my.rs` and will
// insert its contents inside a module named `my` under this scope
// このように宣言すると、`my.rs`という名のファイルを探し、
// その内容をこのファイル中で`my`という名から使用することができるようにします。
mod my;

fn function() {
    println!("called `function()`");
}

fn main() {
    my::function();

    function();

    my::indirect_access();

    my::nested::function();
}

