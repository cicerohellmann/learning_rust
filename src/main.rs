fn main() {
    // println!("Hello, world!");
    // introducing_variable();
    // asserting_value();
    // looping();
    // ifing();
    sum()
}

fn introducing_variable() {
    // An interesting thing happens when you misspell "value",
    // the compiler will warn you about it:
    // ^^^^ help: a local variable with a similar name exists: `value`
    let value = 42;
    println!("Hello {}", value)
}

fn asserting_value() {
    // Another situation where the compiler will warn you if the values are different:
    // assertion `left == right` failed
    //   left: 42
    //  right: 41
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    let value = 42;
    assert_eq!(value, 42)
}

fn looping() {
    for i in 0..5 {
        println!("Hello {}", i)
    }
}

fn ifing() {
    for i in 0..5 {
        if i % 2 == 0 {
            println!("even {}", i);
        } else {
            println!("odd {}", i);
        }
    }
    // Just playing around with it
    for i in 0..5 {
        let even_odd = if i % 2 == 0 { "even" } else { "odd" };
        println!("{} {}", even_odd, i);
    }
}

fn sum() {
    //Here we are tackling mutability,
    // it's practical to simply add "mut"
    let mut sum = 0;
    for i in 0..5 {
        sum += i;
    }
    println!("sum is {}", sum)
}