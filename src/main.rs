fn the_answer() -> usize
{
    42
}

const fn run_at_compile(x: u8) -> u8
{
    x + 1
}

fn ensure_positive(x: i32) -> Result<i32, &'static str>
{
    if x > 0 {
        Ok(x)
    } else {
        Err("number was negative")
    }
}

fn main() {
    println!("Hello, world!");
    println!("the_answer() = {}", the_answer());
    println!("run_at_compile(4) = {}", run_at_compile(4));
    for i in 0..10 {
        println!("i = {}", i);
    }
    let v = vec![2, 3, 5];
    for x in v.iter() {
        println!("x = {}", x);
    }
    let a : Option<i32> = Some(42);
    println!("a = {}", a.unwrap());
    for i in -1..3 {
        match ensure_positive(i) {
            Ok(result) => println!("ensure_positive({}) = {}", i, result),
            Err(error) => println!("ensure_positive({}) = {}", i, error),
        };
    };
}
