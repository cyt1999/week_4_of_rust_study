// 使用枚举
struct TypeA {
    data: String,
}

struct TypeB {
    data: i32,
}

struct TypeC {
    data: f64,
}

enum MyEnum {
    A(TypeA),
    B(TypeB),
    C(TypeC),
}

impl TypeA {
    fn do_something(&self) {
        println!("TypeA: {}", self.data);
    }
}

impl TypeB {
    fn do_something(&self) {
        println!("TypeB: {}", self.data);
    }
}

impl TypeC {
    fn do_something(&self) {
        println!("TypeC: {}", self.data);
    }
}

fn main() {
    let my_vec: Vec<MyEnum> = vec![
        MyEnum::A(TypeA { data: "Hello".to_string() }),
        MyEnum::B(TypeB { data: 42 }),
        MyEnum::C(TypeC { data: 3.14 }),
    ];

    for item in my_vec {
        match item {
            MyEnum::A(a) => a.do_something(),
            MyEnum::B(b) => b.do_something(),
            MyEnum::C(c) => c.do_something(),
        }
    }
}
// TypeA: Hello
// TypeB: 42
// TypeC: 3.14


// 使用 Trait 对象
// trait MyTrait {
//     fn do_something(&self);
// }

// struct TypeA {
//     data: String,
// }

// struct TypeB {
//     data: i32,
// }

// struct TypeC {
//     data: f64,
// }

// impl MyTrait for TypeA {
//     fn do_something(&self) {
//         println!("TypeA: {}", self.data);
//     }
// }

// impl MyTrait for TypeB {
//     fn do_something(&self) {
//         println!("TypeB: {}", self.data);
//     }
// }

// impl MyTrait for TypeC {
//     fn do_something(&self) {
//         println!("TypeC: {}", self.data);
//     }
// }

// fn main() {
//     let my_vec: Vec<Box<dyn MyTrait>> = vec![
//         Box::new(TypeA { data: "Hello".to_string() }),
//         Box::new(TypeB { data: 42 }),
//         Box::new(TypeC { data: 3.14 }),
//     ];

//     for item in my_vec {
//         item.do_something();
//     }
// }

// TypeA: Hello
// TypeB: 42
// TypeC: 3.14