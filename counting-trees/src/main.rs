fn get_input() -> &'static str {
    return "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
}

fn main() {
    let tree_count = get_input()
        .lines()
        .enumerate()
        .flat_map(|(idx, line)| return line.chars().nth(idx * 3 % line.len()))
        .filter(|&c| c == '#')
        .count();

    println!("tree count: {}", tree_count);
}
