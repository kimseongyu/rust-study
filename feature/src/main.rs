fn main() {
    ownership_study();
    struct_study();
    enum_study();
    error_handling_study();
    module_study();
}

// ownership
fn ownership_study() {
    println!("ownership_study");
    
    let mut a = vec![1, 2, 3];
    let b = &a;                 // 읽기 대여
    println!("a={:?}", a);
    println!("b={:?}", b);      // 읽기 대여 구간 종료

    add_one(&mut a);            // 쓰기 대여

    let cnt = get_even_cnt(&a);
    println!("cnt={}", cnt);
    println!("a={:?}", a);
}

fn add_one(a: &mut Vec<i32>) {
    for i in a {
        *i += 1;
    }
}

fn get_even_cnt(a: &Vec<i32>) -> i32 {
    let mut cnt = 0;
    for i in a {
        if i % 2 == 0 {
            cnt += 1;
        }
    }
    cnt
}

// struct
#[derive(Debug)]
struct Student {
    name: String,
    score: i32,
}

impl Student {
    fn new(name: &str, score: i32) -> Student {
        Student {
            name: name.to_owned(),
            score,
        }
    }
    fn get_grade(&self) -> i32 {
        match self.score {
            90..=100 => 1,
            80..=89 => 2,
            70..=79 => 3,
            60..=69 => 4,
            _ => 5,
        }
    }
}

struct Color(i32, i32, i32);

fn struct_study() {
    println!("struct_study");

    let s = Student::new("sun", 100);
    println!("s={:?}", s);
    println!("grade={}", s.get_grade());
}

// enum
#[derive(Debug)]
enum Gender {
    Male,
    Female,
}

fn get_gender(name: &str) -> Gender {
    match name {
        "sun" => Gender::Male,
        "moon" => Gender::Female,
        _ => Gender::Male,
    }
}

fn divmod(n: i32, d: i32) -> Result<(i32, i32), String> {
    if d == 0 {
        Err("division by zero".to_owned())
    } else {
        Ok((n / d, n % d))
    }
}

fn enum_study() {
    println!("enum_study");

    let g = get_gender("sun");
    println!("g={:?}", g);

    match divmod(10, 2) {
        Ok((q, r)) => println!("q={}, r={}", q, r),
        Err(e) => println!("error={}", e),
    }
}

// trait
struct Dog;

trait Animal {
    type category;

    const LEGS: i32 = 4;
    const CALORIES: i32;

    fn get_legs(&self) -> i32 {
        Self::LEGS
    }
    fn get_category(&self) -> Self::category;
}

impl Animal for Dog {
    type category = String;

    const CALORIES: i32 = 100;

    fn get_category(&self) -> Self::category {
        "dog".to_owned()
    }
}

// generic
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// error handling
fn error_handling_study() {
    println!("error_handling_study");

    let result = divmod(10, 2);
    match result {
        Ok((q, r)) => println!("q={}, r={}", q, r),
        Err(e) => panic!("error={}", e),
    }
}

// module
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

fn module_study() {
    println!("module_study");
    println!("add={}", math::add(1, 2));
}