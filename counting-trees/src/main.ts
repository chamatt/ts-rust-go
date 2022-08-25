function getInput(): string {
  return `..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#`;
}

enum Thing {
  Tree,
  Snow,
}

const things = getInput()
  .split("\n")
  .map((x) => x.split("").map((y) => (y === "#" ? Thing.Tree : Thing.Snow)));

const colLen = things[0].length;
let treeCount = 0;

things.forEach((thingRow, i) => {
  if (thingRow[(i * 3) % colLen] === Thing.Tree) {
    treeCount++;
  }
});

console.log("treeCount", treeCount);
