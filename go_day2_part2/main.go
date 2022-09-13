package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type nav struct {
	x     int
	depth int
	aim   int
}

func (n *nav) move(dim string, amount int) {
	switch dim {
	case "forward":
		n.x = n.x + amount
		n.depth = n.depth + n.aim * amount
	case "up":
		n.aim = n.aim - amount
	case "down":
		n.aim = n.aim + amount
	}

}

func main() {
	file, err := os.Open("data.in")
	if err != nil {
		fmt.Println(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	var n nav
	for scanner.Scan() {
		// Each line has the format "<direction> <amount>"
		movement := strings.Split(scanner.Text(), " ")
		amount, err := strconv.Atoi(movement[1])
		if err != nil {
			fmt.Println(err)
			continue
		}
		n.move(movement[0], amount)
	}

	fmt.Printf("Result: %v\n", n.x*n.depth)
}
