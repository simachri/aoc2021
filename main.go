package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

type rowSlice []int

// filter returns one number that is the row from the set of rows that is filtered by the
// "mostCommon" or "leastCommon" significant bit algorithm.
func filter(rows []string, invertToLeastCommon bool) string {
	if len(rows) == 0 {
		return ""
	}
	filteredRows := rows
	var filter string
	// For all columns...
	for i := 0; i < len(rows[0]); i++ {
		filter = calcMostCommonBit(filteredRows, i, invertToLeastCommon)
		filteredRows = filterRowsByBit(filteredRows, i, filter)
		if len(filteredRows) == 1 {
			break
		}
	}
	if len(filteredRows) != 1 {
		fmt.Printf("Failed to filter down to one record with filter %v\n", filter)
	}
	return filteredRows[0]
}

// filterRowsByBit returns the rows of the input set that have a specific bit at a
// given position.
// filterPos is the zero-based index within the number that is matched against the filter
// value.
// filterVal is either "0" or "1".
func filterRowsByBit(rows []string, filterPos int, filterVal string) []string {
	var filtered []string
	for _, row := range rows {
		valAtPos := string([]rune(row)[filterPos])
		if valAtPos == filterVal {
			filtered = append(filtered, row)
		}
	}
	return filtered
}

// calcMostCommonBit returns the most common bit at "pos" for the given set of binary
// numbers "binNums".
// eqResolver is the bit that is taken if both "1" and "0" have identical occurrences.
func calcMostCommonBit(binNums []string, pos int, invertToLeastCommon bool) string {
	var resultBit string
	var onesCnt int
	var zerosCnt int
	for _, binNum := range binNums {
		if string(binNum[pos]) == "1" {
			onesCnt++
		} else {
			zerosCnt++
        }
	}
    if onesCnt >= zerosCnt {
		resultBit = "1"
    } else {
		resultBit = "0"
    }

	if invertToLeastCommon == true {
		if resultBit == "1" {
			resultBit = "0"
		} else {
			resultBit = "1"
		}
	}

	return resultBit
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
	var rows []string
	// Count the '1' per column and create the slice 'rows'.
	for scanner.Scan() {
		rowCnt++
		// For each row: Go through all binary digits from left to right.
		binRow := scanner.Text()
		rows = append(rows, binRow)
	}
	if err := scanner.Err(); err != nil {
		fmt.Println(err)
	}

	oxygenBin := filter(rows, false)
	fmt.Printf("Oxygen binary: %v\n", oxygenBin)
	oxygenRating, err := strconv.ParseInt(oxygenBin, 2, 64)
	fmt.Printf("Oxygen rating: %v\n", oxygenRating)
	if err != nil {
		fmt.Println(err)
	}
	co2Bin := filter(rows, true)
	fmt.Printf("co2 binary: %v\n", co2Bin)
	co2Rating, err := strconv.ParseInt(co2Bin, 2, 64)
	fmt.Printf("co2 rating: %v\n", co2Rating)
	if err != nil {
		fmt.Println(err)
	}

	res := oxygenRating * co2Rating

	// res := calcGamma(mostCommonBits) * calcEpsilon(mostCommonBits)
	fmt.Printf("Result:%v\n", res)
}
