package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

type window struct {
	count     int
	sum       int
	isInitial bool
}

func (w *window) isFull() bool {
	if w.count == 3 {
		return true
	}
	return false
}

func (w *window) add(v int) {
	w.sum = w.sum + v
	w.count++
	w.isInitial = false
}

func (w *window) reset() {
	w.sum = 0
	w.count = 0
}

func main() {
	file, err := os.Open("data.in")
	if err != nil {
		fmt.Println(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	// Count the incrementations.
	var incCount int
	prevDepth := -1
	// Three windows needed.
	wins := [3]window{
		{
			isInitial: true,
		},
		{
			isInitial: true,
		},
		{
			isInitial: true,
		},
	}
	for scanner.Scan() {
		depth, err := strconv.Atoi(scanner.Text())
		if err != nil {
			fmt.Printf("Failed to convert to int: %v\n", err)
			continue
		}
		// Fill windows.
		for i := range wins {
			w := wins[i]
			// If a window is initial, no additional windows need to be filled.
			// This happens when the windows are initially filled within the first three
			// measurements.
			if w.isInitial {
				w.add(depth)
				wins[i] = w
				break
			}
			w.add(depth)
			wins[i] = w
		}
		// Evaluate full windows
		for i := range wins {
			w := wins[i]
			// If a window is full, take its depth.
			if w.isFull() {
				if prevDepth != -1 &&
					prevDepth < w.sum {
					incCount++
				}
				prevDepth = w.sum
				w.reset()
				wins[i] = w
			}
		}
	}

	fmt.Printf("Result: %v\n", incCount)
}
