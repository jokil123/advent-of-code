fn main() {
    let mut my_test = Test {
        my_bool: true,
        my_int: 69_i64,
        my_string: String::from("I'm Stuff"),
    };

    takes_reference(&my_test);
    takes_mut_reference(&mut my_test);
    takes_ownership(my_test);
}

fn takes_ownership(test: Test) -> bool {
    println!("{:?}", test.my_int);
    return test.my_bool && true;
}

fn takes_reference(test: &Test) -> bool {
    println!("{:?}", test.my_int);
    return test.my_bool && true;
}

fn takes_mut_reference(test: &mut Test) -> bool {
    println!("{:?}", test.my_int);

    test.my_bool = !test.my_bool;

    return test.my_bool;
}

struct Test {
    my_bool: bool,
    my_int: i64,
    my_string: String,
}
