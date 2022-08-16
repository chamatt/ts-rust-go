enum CustomOption<T> {
    None,
    Some(T),
}

impl<T> CustomOption<T> {
    fn is_some(&self) -> bool {
        return match self {
            CustomOption::None => false,
            CustomOption::Some(_) => true,
        };
    }

    fn unwrap(self) -> T {
        return match self {
            CustomOption::Some(x) => x,
            CustomOption::None => panic!("called `CustomOption::unwrap()` on a `None` value"),
        };
    }
}

fn main() {
    let native_some = Some(5);
    let custom_some = CustomOption::Some(5);

    println!("Native Option is_some: {}", native_some.is_some());
    println!("Custom Option: is_some: {}", custom_some.is_some());

    if custom_some.is_some() {
        let value = custom_some.unwrap();
        print!("{:?}", value)
    }
}
