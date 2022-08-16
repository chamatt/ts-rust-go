function getInput(): string {
  return `forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2`;
}

function parseLine(line: string): [number, number] {
  const [direction, a] = line.trim().split(" ");
  const amount = +a;

  if (direction === "forward") {
    return [amount, 0];
  } else if (direction === "up") {
    return [0, -amount];
  }
  return [0, amount];
}

function main() {
  const input = getInput();

  const [x, y] = input
    .split("\n")
    .map((x) => parseLine(x))
    .reduce(
      ([accX, accY], [dx, dy]) => {
        return [accX + dx, accY + dy];
      },
      [0, 0]
    );

  console.log(`x: ${x}, y: ${y}, x * y: ${x * y}`);
}

main();
