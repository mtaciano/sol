fn sum(var_one, var_two) -> int {
    return (var_one / var_two) + (var_one - var_two);
}

fn main() {
    // Currently only int and none type exists
    let foo;
    foo = 999;

    let bar = 42;

    let baz[32];
    baz[0] = 123;

    let this_value;
    this_value = sum(foo, bar);
}
