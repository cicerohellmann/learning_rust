#![allow(dead_code)]

fn main() {
    // println!("Hello, world!");
    // introducing_variable();
    // asserting_value();
    // looping();
    // ifing();
    // sum()
    // print!("Square of 9 is {}", sqr(9.0));
    // print!("Square of 9 is {}", sqr_without_return(9.0));
    // reference();
    // mutable();
    no_import();
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
    let mut sum = 0.0;
    for i in 0..5 {
        sum += i as f64;
    }
    println!("sum is {}", sum)
}

fn sqr(x: f64) -> f64 {
    return x * x;
}

fn sqr_without_return(x: f64) -> f64 {
    // If You accidentally leave the semicolon here, you will get:
    //    |    implicitly returns `()` as its body has no tail or `return` expression
    // 66 |     x * x;
    //    |          - help: remove this semicolon to return this value
    x * x
}

// The idea of reference to avoid creating copies of data
// is a cool concept to help understand reference and dereference

fn reference() {
    let i = 10;
    let res1 = by_ref(&i);
    let res2 = by_ref(&41);
    println!("{} {}", res1, res2);
}

fn by_ref(x: &i32) -> i32 {
    *x + 1
}

fn mutable() {
    let mut res = 0.0;
    modifies(&mut res);
    println!("res is {}", res)
}

fn modifies(x: &mut f64) {
    *x = 1.0;
}

fn no_import() {
    let x = 2.0 * std::f64::consts::PI;
    let abs_difference = (x.cos() - 1.0).abs();
    println!("{}", abs_difference < 1e-10);
    assert!(abs_difference < 1e-10);
}
