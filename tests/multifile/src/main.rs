mod test;

use crate::test::imported_function;

fn main() {
    println!("Hello, world!");

    imported_function();
    test::imported_function();
}
