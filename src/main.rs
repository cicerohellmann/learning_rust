fn main() {
    println!("Hello, world!");
    introducing_variable();
    asserting_value();
}

fn introducing_variable(){
    // An interesting thing happens when you misspell "value",
    // the compiler will warn you about it:
    // ^^^^ help: a local variable with a similar name exists: `value`
    let value = 42;
    println!("Hello {}", value)
}

fn asserting_value(){
    // Another situation where the compiler will warn you if the values are different:
    // assertion `left == right` failed
    //   left: 42
    //  right: 41
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    let value = 42;
    assert_eq!(value, 41)
}