fn main() {
    a(&Test { my_int: 100 })
}

fn a(test: &Test) {
    change(*test);
}

fn change(mut test: Test) {
    test.my_int = 10;
}

struct Test {
    my_int: i64,
}
