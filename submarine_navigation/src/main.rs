#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn get_input() -> &'static str {
    return "forward 5
      down 5
      forward 8
      up 3
      down 8
      forward 2";
}

fn parse_line(line: &str) -> Point {
    let (dir, amount) = line
        .trim()
        .split_once(" ")
        .expect("must contain a whitespace");
    let amount: i32 = amount.parse().expect("must be a number");

    if dir == "forward" {
        return Point { x: amount, y: 0 };
    } else if dir == "down" {
        return Point { x: 0, y: amount };
    }
    return Point { x: 0, y: -amount };
}

fn main() {
    let result =
        get_input()
            .lines()
            .map(parse_line)
            .fold(Point { x: 0, y: 0 }, |mut acc, point| {
                acc.x += point.x;
                acc.y += point.y;
                return acc;
            });

    println!("{:?} x*y={:?}", result, result.x * result.y);
}
