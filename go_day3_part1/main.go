package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type rowSlice []int

func calcGamma(colOnes rowSlice, rowCnt int) int64 {
	var gammaBin []string
	for _, onesCnt := range colOnes {
		// If the amount of '1' in a column is larger than half of the row count, then
		// '1' is the most common bit.
		if onesCnt > rowCnt/2 {
			gammaBin = append(gammaBin, "1")
		} else {
			gammaBin = append(gammaBin, "0")
		}
	}
	i, err := strconv.ParseInt(strings.Join(gammaBin, ""), 2, 64)
	if err != nil {
		fmt.Println(err)
	}
	return i
}

func calcEpsilon(colOnes rowSlice, rowCnt int) int64 {
	var gammaBin []string
	for _, onesCnt := range colOnes {
		// If the amount of '1' in a column is smaller than half of the row count, then
		// '1' is the least common bit.
		if onesCnt < rowCnt/2 {
			gammaBin = append(gammaBin, "1")
		} else {
			gammaBin = append(gammaBin, "0")
		}
	}
	i, err := strconv.ParseInt(strings.Join(gammaBin, ""), 2, 64)
	if err != nil {
		fmt.Println(err)
	}
	return i
}

func main() {
	file, err := os.Open("data.in")
	if err != nil {
		fmt.Println(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	// Number of rows in the data
	var rowCnt int
	// Amount of '1' per 'column'.
	var columnOnes rowSlice
	for scanner.Scan() {
		rowCnt++
		// For each row: Go through all binary digits from left to right.
		binRow := scanner.Text()
		for i := range binRow {
			val, err := strconv.Atoi(string(binRow[i]))
			if err != nil {
				fmt.Println(err)
			}
			if len(columnOnes) <= i {
				columnOnes = append(columnOnes, val)
			} else {
				columnOnes[i] = columnOnes[i] + val
			}
		}
	}

	if err := scanner.Err(); err != nil {
		fmt.Println(err)
	}
	res := calcGamma(columnOnes, rowCnt) * calcEpsilon(columnOnes, rowCnt)
	fmt.Printf("Result:%v\n", res)
}
