package main

import (
	"fmt"
	"log"
	"strconv"
	"strings"
)

type Point struct {
	x int
	y int
}

func getInput() string {
	return `forward 5
	  down 5
	  forward 8
	  up 3
	  down 8
	  forward 2`
}

func parseLine(line string) Point {
	trimmed_line := strings.TrimSpace(line)
	parts := strings.Split(trimmed_line, " ")

	amount, err := strconv.Atoi(parts[1])
	if err != nil {
		log.Fatal("this should never happen")
	}

	if parts[0] == "forward" {
		return Point{
			x: amount,
			y: 0,
		}
	} else if parts[0] == "down" {
		return Point{
			x: 0,
			y: amount,
		}
	}
	return Point{
		x: 0,
		y: -amount,
	}

}

func main() {
	lines := strings.Split(getInput(), "\n")
	pos := Point{0, 0}
	for _, line := range lines {
		movement := parseLine(line)
		pos.x += movement.x
		pos.y += movement.y
	}

	fmt.Printf("point: %+v, x*y: %v\n", pos, pos.x*pos.y)
}
