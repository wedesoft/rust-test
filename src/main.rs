use std::io;
use std::fs::read_to_string;
use bigdecimal::BigDecimal;
use ndarray::array;
use ndarray::prelude::*;

fn the_answer() -> usize
{
    42
}

const fn run_at_compile(x: u8) -> u8
{
    x + 1
}

fn ensure_positive(x: i32) -> Result<i32, std::io::Error>
{
    if x > 0 {
        Ok(x)
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "number was negative"))
    }
}

fn call_ensure_positive(x: i32) -> Result<i32, std::io::Error>
{
    ensure_positive(x)?;
    Ok(0)
}

fn add_numbers(x: i32, y: i32) -> i32
{
    x + y
}

fn factorial(x: i32) -> BigDecimal
{
    if x == 1 {
        BigDecimal::from(1)
    } else {
        x * factorial(x - 1)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add_numbers()
    {
        assert_eq!(super::add_numbers(1, 2), 3);
    }

    #[test]
    fn test_factorial()
    {
        assert_eq!(super::factorial(5), super::BigDecimal::from(120));
    }
}


fn main() {
    println!("Hello, world!");
    println!("the_answer() = {}", the_answer());
    println!("run_at_compile(4) = {}", run_at_compile(4));

    for i in 0..5 {
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

    #[derive(Debug, Clone, Copy, PartialEq)]
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

    // Each value can only have one owner at any one time.
    let s1 = String::from("hello");
    let s2 = s1;
    // Can't print s1 any more because String does not implement Copy.
    println!("s2 = {}", s2);
    let s3 = s2.clone();
    // String implements Clone (an explicit copy operation).
    println!("s2 = {}", s2);
    println!("s3 = {}", s3);

    // Function moves ownership.
    fn print_string(s: String) {
        println!("s = {}", s);
    }
    let s4 = String::from("hello");
    print_string(s4);
    // print_string(s4);

    // Use reference, i.e. borrow it.
    fn print_string_ref(s: &String)
    {
        println!("s = {}", s);
    }
    let s5 = String::from("hello");
    print_string_ref(&s5);
    print_string_ref(&s5);

    // Use reference to a string slice.
    fn print_string_slice(s: &str)
    {
        println!("s = {}", s);
    }
    let s6 = "hello";
    print_string_slice(&s6);
    print_string_slice(&s6);

    // Mutable variables.
    let mut m = 0;
    m += 1;
    assert_eq!(m, 1);

    // You can only have one mutable reference to a variable at a time.
    let m2 = &mut m;
    *m2 += 1;
    // You can have multiple immutable references.
    let m3 = &m;
    assert_eq!(m, 2);
    assert_eq!(*m3, 2);

    // Static lifetime means that it exists for the duration of the program.
    let s7: &'static str = "hello";
    println!("s7 = {}", s7);

    enum Pokemon2<'a>
    {
        AnythingElse(&'a str),
    }
    let pokemon2 = Pokemon2::AnythingElse("Other");
    {
        let Pokemon2::AnythingElse(s) = pokemon2;
        println!("s = {}", s);
    }

    println!("add_numbers(1, 2) = {}", add_numbers(1, 2));

    println!("call_ensure_positive(1) = {}", call_ensure_positive(1).unwrap());
    println!("call_ensure_positive(-1) = {:?}", call_ensure_positive(-1));

    println!("factorial({}) = {}", 100, factorial(100));

    // Read first line from file "test.txt"
    let file = read_to_string("resources/test.txt");
    let line = file.expect("Could not open file").lines().next().expect("No lines in file").to_string();
    println!("first line = {}", line);

    // Mutable variable
    let mut x = 5;
    println!("x = {}", x);
    x = 6;
    println!("x = {}", x);

    // Shadowing
    let u = 5;
    println!("u = {}", u);
    let u = u + 1;
    println!("u = {}", u);

    let option: Option<bool> = Some(true);
    let option = option.unwrap();
    println!("option = {}", option);

    let value: Vec<i32> = (0..10).map(|x| x * x).collect();
    println!("value = {:?}", value);

    let v = (0..10).reduce(|x, y| x + y);
    println!("v = {:?}", v);

    // Implement a trait for Point
    impl std::fmt::Display for Point
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
        {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
    let point3 = Point { x: 2.0, y: 3.0 };
    println!("point3 = {}", point3);

    let v = [0, 1, 2, 3];
    for _ in [0, 1] {
        for i in v {
            println!("i = {}", i);
        }
    };

    // Dictionary
    let mut h = std::collections::HashMap::new();
    h.insert("Jan", 31);
    h.insert("Feb", 30);
    h.insert("Mar", 31);
    println!("h = {:?}", h);
    println!("type of h = {}", std::any::type_name_of_val(&h));
    for (k, v) in &h {
        println!("k = {}, v = {}", k, v);
    }
    assert!(h.contains_key("Mar"));

    trait Steppable: Copy + std::cmp::PartialOrd + std::ops::AddAssign + std::ops::SubAssign {}
    impl Steppable for i32 {}
    impl Steppable for f32 {}
    impl Steppable for f64 {}

    // Generic range object for comparable type
    #[derive(Copy, Clone)]
    struct NumericRange<T: Steppable> {
        start: T,
        end: T,
        step: T,
    }
    // Implement iterator for generic range object of comparable type
    impl<T: Steppable> Iterator for NumericRange<T> {
        type Item = T;
        fn next(&mut self) -> Option<T> {
            if self.start > self.end {
                None
            } else {
                let res = self.start;
                self.start += self.step;
                Some(res)
            }
        }
    }
    impl<T: Steppable> DoubleEndedIterator for NumericRange<T> {
        fn next_back(&mut self) -> Option<T> {
            if self.start > self.end {
                None
            } else {
                let res = self.end;
                self.end -= self.step;
                Some(res)
            }
        }
    }

    let range = NumericRange { start: 0.0, end: 1.0, step: 0.25 };
    for x in range {
        println!("x = {}", x);
    }
    for x in range.rev() {
        println!("x = {}", x);
    }

    let u = Some(42);
    let v = u.map(|x| x + 1);
    println!("v = {}", v.unwrap());

    // Define graph type as vector of tuples of integers
    type Graph = Vec<(usize, usize)>;
    let g: Graph = [(0, 1), (0, 2), (1, 2), (2, 3)].to_vec();
    println!("g = {:?}", g);
    for (i, j) in g.iter() {
        println!("i = {}, j = {}", i, j);
    }

    // Adjacency map
    let mut am = std::collections::HashMap::new();
    for (k, v) in g.iter() {
        am.entry(k).or_insert(vec![]).push(*v);
    };
    for (k, v) in am {
        println!("k = {}, v = {:?}", k, v);
    }

    let a = array![[1, 2, 3], [4, 5, 6]];
    println!("a = {:?}", a);
    println!("a.shape() = {:?}", a.shape());
    let b = a.map(|x| x + 1);
    println!("b = {:?}", b);
    let c = &a + array!([0, 100, 200]);
    println!("c = {:?}", c);
    println!("c.sum() = {}", &a.sum());

    println!("a[1..2, ..] = {:?}", a.slice(s![1..2, ..]));
    // Get row of array
    println!("a[1, ..] = {:?}", a.slice(s![1, ..]));

    // Format string
    let x = 5;
    println!("{x}");

    // Vector
    let mut v: Vec<usize> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("v = {:?}", v);
    assert_eq!(None, v.get(3));
    assert_eq!(Some(&1), v.get(0));

    fn return_first_element(x: &[usize]) -> Option<usize> {
        match x.len() {
            0 => None,
            _ => Some(x[0]),
        }
    }
    assert_eq!(Some(1), return_first_element(&v));
    assert_eq!(Some(2), return_first_element(&v[1..]));

    // Fifo queue
    let mut q = std::collections::VecDeque::new();
    q.push_back(1);
    q.push_back(2);
    println!("q = {:?}", q);
    assert_eq!(Some(1), q.pop_front());
    q.push_back(3);
    assert_eq!(Some(2), q.pop_front());
    assert_eq!(Some(3), q.pop_front());

    let mut a = [1, 2, 3];
    a[1] = 4;
    println!("a = {:?}", a);

    // Turbofish ::<>
    let a = [1, 2, 3];
    let s = a.iter().map(|x| x + 1).filter(|x| x < &4).sum::<usize>();
    assert_eq!(s, 5);

    // Linked list
    let mut list = std::collections::LinkedList::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    println!("list = {:?}", list);
    assert_eq!(Some(3), list.pop_front());
    assert_eq!(Some(2), list.pop_front());
    assert_eq!(Some(1), list.pop_front());

    fn identity<T>(x: T) -> T {
        x
    }
    let x = identity(42);
    println!("x = {}", x);
    let y = identity("test");
    println!("y = {}", y);

    fn add<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
        x + y
    }
    let z = add(1, 2);
    println!("z = {}", z);

    fn print<T: std::fmt::Display>(x: T) {
        println!("x = {}", x);
    }
    print("hello");

    fn print_sum<T: std::ops::Add<Output = T> + std::fmt::Display>(x: T, y: T) {
        println!("x + y = {}", x + y);
    }
    print_sum(1, 2);

    // Type coercion
    trait Add<T, U, V> {
        fn plus(t: T, u: U) -> V;
    }
    impl Add<i16, u32, i32> for (i16, u32) {
        fn plus(x: i16, y: u32) -> i32 {
            x as i32 + y as i32
        }
    }
    impl Add<u32, i16, i32> for (u32, i16) {
        fn plus(x: u32, y: i16) -> i32 {
            x as i32 + y as i32
        }
    }

    let a = 1_i16;
    let b = 2_u32;
    println!("plusx(1, 2) = {}", <(i16, u32)>::plus(a, b));
    println!("type of plusx(1, 2) is {}", std::any::type_name_of_val(&<(i16, u32)>::plus(a, b)));

    struct Human;
    struct Dog;

    trait Speak {
        fn speak(&self) -> String;
    }
    impl Speak for Human {
        fn speak(&self) -> String {
            "Hello".to_string()
        }
    }
    impl Speak for Dog {
        fn speak(&self) -> String {
            "Woof".to_string()
        }
    }
    fn make_speak(t: &dyn Speak) {
        println!("{}", t.speak());
    }
    let human = Human {};
    make_speak(&human);
    let dog = Dog {};
    make_speak(&dog);
    fn mammal_with_legs(num_legs: i32) -> Box<dyn Speak> {
        if num_legs == 4 {
            Box::new(Dog {})
        } else {
            Box::new(Human {})
        }
    }
    let x = mammal_with_legs(4);
    println!("4 legs -> {}", x.speak());
    let y = mammal_with_legs(2);
    println!("2 legs -> {}", y.speak());

    // trait Coerce {
    //     type First;
    //     type Second;
    //     type Item;
    // }

    // impl Coerce for (i16, u32) {
    //     type First = i16;
    //     type Second = u32;
    //     type Item = i32;
    // }
    // fn plusx<(T, U): Coerce>(x: T, y: U) -> <(T, U) as Coerce>::Item {
    //     x as <(T, U) as Coerce>::Item + y as <(T, U) as Coerce>::Item
    // }
    // fn plusx<C: Coerce>(x: <C as Coerce>::First, y: <C as Coerce>::Second) -> <C as Coerce>::Item
    //     where <C as Coerce>::Item: std::ops::Add<Output = <C as Coerce>::Item>
    // {
    //     <<C as Coerce>::First as Into<T>>::into(x);
    // }
    // println!("plusx(1i36, 2u32) = {}", plusx::<(i16, u32)>(1i16, 2u32));

    // Closures
    let square = |x| x * x;
    assert_eq!(9, square(3));
    fn takes_a_lambda(x: usize, f: fn(usize) -> usize) -> usize {
        f(x)
    }
    assert_eq!(9, takes_a_lambda(3, square));
    fn cv<T: Into<usize>>(a: T) -> usize {
        a.into()
    }
    assert_eq!(1, cv(true));

    let mut v: Vec<usize> = vec![1, 2, 3];
    let x: usize = 2;
    let v = v.into_iter().filter(|i| *i > x).collect::<Vec<usize>>();
    assert_eq!(v, vec![3]);
}
