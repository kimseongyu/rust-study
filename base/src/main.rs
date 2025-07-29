use std::collections::HashMap;
use std::collections::HashSet;

/// main function
fn main() {
    println!("1+...+100={}", get_sum(100));

    let info = get_info();
    println!("age={}, height={}", info.0, info.1);

    let a = [1, 2, 3, 4, 5];
    println!("{:?}", &a[0..2]);

    let b = [1; 10];
    println!("{:?}", b);

    let mut v = vec![1, 2, 3, 4];
    v.push(5);
    println!("{:?}", v);

    let p1 = Point::new(0, 0);
    let p2 = Point::new(3, 4);
    assert_eq!(5.0, p1.distance(&p2));

    let add_one = |x: i32| x+ 1;
    println!("add_one(1)={}", add_one(1));

    let n = 10;
    if n % 2 == 0 {
        println!("n is even");
    } else {
        println!("n is odd");
    }

    let x = 1;
    match x {
        1 => println!("x is 1"),
        2 => println!("x is 2"),
        _ => println!("x is other"),
    }

    let mut sum = 0;
    'outer: for i in 1..=10 {
        sum += i;
        for j in 1..=10 {
            sum += j;
            if j == 5 {
                break 'outer;
            }
        }
    }
    println!("sum={}", sum);
    
    loop {
        sum -= 1;
        if sum == 0 {
            break;
        }
    }

    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}", v);

    let mut v2 = vec![1, 2, 3];
    v2.push(4);

    let mut map = HashMap::new();
    map.insert("name", "John");

    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    println!("{:?}", set);

    let s1 = "Hello world!";
    let s2 = String::from("Hello");
    let s3 = String::from(" world!");
    let s4 = s2 + &s3;
    println!("{}", s1);
    // println!("{}", s2); 소유권이 s4로 이동하므로 사용 불가
    println!("{}", s3);
    println!("{}", s4);
}

/// 1+...+n
fn get_sum(n:u32) -> u32 {
    let mut sum:u32 = 0;

    for i in 1..=n {
        sum += i;
    }

    return sum;
}

/// get age and height
fn get_info() -> (i32, f64){
    let age = 20;
    let height = 60.5;

    return (age, height);
}

/// method
struct Point {
    x: i32, y: i32
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point {x, y}
    }
    fn distance(&self, p:&Point) -> f64 {
        (((self.x - p.x).pow(2) + (self.y - p.y).pow(2)) as f64).sqrt()
    }
}