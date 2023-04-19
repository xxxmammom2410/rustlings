// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a hint.

struct Book <'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    // 構造体が文字列スライスへの参照を持つ
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
