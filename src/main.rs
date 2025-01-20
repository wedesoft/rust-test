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

    let y1 = 256;
    println!("type of y1 = {}", std::any::type_name_of_val(&y1));
    let y2 : u8 = 255;
    println!("type of y2 = {}", std::any::type_name_of_val(&y2));
    let y3 = 256_u16;
    println!("type of y3 = {}", std::any::type_name_of_val(&y3));
    let y4 = 255 as usize;
    println!("type of y4 = {}", std::any::type_name_of_val(&y4));
    let f1 : f32 = 2.78;
    println!("type of f1 = {}", std::any::type_name_of_val(&f1));

    let t: (f32, &str) = (3.14, "this is close to pi");
    println!("t.0 = {}", t.0);
    let (p, _) = t;
    println!("p = {}", p);

    let arr: [usize; 3] = [2, 3, 5];
    assert_eq!(arr[1], 3);
    assert_eq!(arr, [2, 3, 5]);
    assert_eq!(&arr, &[2, 3, 5]);
    assert_eq!(&arr[0..1], &[2]);
    assert_eq!(&arr[1..], &[3, 5]);

    let s: &str = "hello";
    assert_eq!(&s[..2], "he");
    let ss: String = String::from("world");
    assert_eq!(&ss[..2], "wo");
    let sss = "horse".to_string();
    assert_eq!(&sss, "horse");

    let b: Box<usize> = Box::new(42);
    assert_eq!(*b, 42);

    let x = true;
    let res: usize = match x
    {
        true => 2,
        false => 1,
    };
    assert_eq!(res, 2);

    let q = Some(42);
    if let Some(i) = q {
        println!("q = Some({})", i);
    };

    enum Distance {
        Kilometres(f64),
        Miles(f64),
    }

    let d1 = Distance::Kilometres(10.0);
    let d2 = Distance::Miles(5.0);
    match d1
    {
        Distance::Kilometres(v) => println!("{} km", v),
        Distance::Miles(v) => println!("{} miles", v),
    }
    match d2
    {
        Distance::Kilometres(v) => println!("{} km", v),
        Distance::Miles(v) => println!("{} miles", v),
    }
    if let Distance::Miles(x) = d2
    {
        println!("{} miles", x);
    }
    enum Pokemon
    {
        Pichu,
        Pikachu,
        Raichu,
    }
    let pokemon = Pokemon::Pikachu;
    println!("Pokemon::Pichu = {}", Pokemon::Pichu as i8);
    println!("pokemon = {}", pokemon as i8);
    println!("Pokemon::Raichu = {}", Pokemon::Raichu as i8);

    #[derive(Debug, Clone, Copy)]
    struct Point
    {
        x: f64,
        y: f64,
    }

    let point = Point { x: 2.0, y: 3.0 };
    println!("point.x = {}", point.x);
    println!("point.y = {}", point.y);

    impl Point
    {
        fn new(x: f64, y: f64) -> Self
        {
            Point { x, y }
        }
    }
    let point2 = Point::new(2.0, 3.0);
    println!("point2.x = {}", point2.x);
    println!("point2.y = {}", point2.y);
}
