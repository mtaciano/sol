/* Returns an int */
fun sum(x, y) int {
    return x + y;
}

/* Returns none (nothing) */
fun main() {
    decl foo;
    foo = 42;

    decl bar = 37;

    /* baz 0..63 = 1 */
    decl baz[64] = [ 1 ];
    // TODO: Maybe something more explicit
    // decl baz[64] = 64[ 1 ]; ?

    for (decl i = 0; i < length(baz); i += 1) {
        print("values: {v}, {bar}; sum: ");
        println("{sum(bar, v)}"); // 38
    }
}
