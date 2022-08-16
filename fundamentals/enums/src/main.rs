// enum RSEnum {
//     Foo(i32),
//     OptionFoo(Option<i32>),
//     Bar(String),
//     Baz(Vec<String>),
//     FuncFoo(fn() -> i32),
// }

// fn bar() -> i32 {
//     42
// }

// fn main2() {
//     let foo = RSEnum::FuncFoo(bar);
//     if let RSEnum::Foo(value) = foo {
//         println!("{}", value);
//     }

//     match foo {
//         RSEnum::Foo(value) => {
//             println!("{}", value);
//         }
//         RSEnum::OptionFoo(Some(value)) => {
//             println!("{}", value);
//         }
//         RSEnum::OptionFoo(None) => {
//             println!("None");
//         }
//         RSEnum::FuncFoo(func) => {
//             println!("{}", func());
//         }
//         _ => {}
//     }
// }

fn main() {
    let foo = Some(5);
    if let Some(value) = foo {
        println!("{}", value);
    }

    match foo {
        Some(value) => {
            println!("{}", value);
        }
        None => {
            println!("None");
        }
    }

    let double = foo.map(|x| x * 2);
    match double {
        Some(value) => println!("{}", value),
        None => println!("None"),
    }

    let double_if_less_than10 = foo.filter(|&x| x < 3).map(|x| x * 2);
    match double_if_less_than10 {
        Some(value) => println!("{}", value),
        None => println!("None"),
    }
}
