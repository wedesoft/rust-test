// method accepting and calling an optional closure
fn foo(x: Option<Box<dyn Fn() + 'static>>)
{
    // call x
    x.unwrap()();
}

// method accepting and calling an optional closure using reference instead of Box
fn foo2(x: Option<&dyn Fn()>)
{
    // call x
    x.unwrap()();
}

fn main() {
    // Call foo2 with a closure printing Hello World.
    foo2(Some(&|| println!("Hello World")));
}
