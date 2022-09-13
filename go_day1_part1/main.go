package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

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
    for scanner.Scan() {
        depth, err := strconv.Atoi(scanner.Text())
        if err != nil {
            fmt.Printf("Failed to convert to int: %v\n", err)
            continue
        }
        if (prevDepth != -1 &&
            prevDepth < depth) {
                incCount++
        } 
        prevDepth = depth
    }

    fmt.Printf("Result: %v\n", incCount)
}
