fn main() {
    // let mut a = vec![];
    // let mut b = a;

    // b.push(1);
    // a.push(1);

    // println!("{:?}", b);

    let mut x = 5;
    // let y = &x;

    {
        let z = &mut x;
        *z = 7;

        println!("{:?}", x);
    }

    x = 9;

    println!("{:?}", x);
}
