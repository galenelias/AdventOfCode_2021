package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
)

func StringsToInts(input []string) []int {
	var result []int

	for _, current := range input {
		convertedInt, err := strconv.Atoi(current)
		if err != nil {
			fmt.Printf("Error parsing string: %s\n", err)
		} else {
			result = append(result, convertedInt)
		}
	}
	return result
}

func ReadToLines(rd io.Reader) []string {
	var result []string
	reader := bufio.NewReader(rd)

	for {
		text, err := reader.ReadString('\n')

		if err == io.EOF {
			break
		}
		result = append(result, strings.TrimSuffix(text, "\n"))
	}
	return result
}

func SumSlice(elements []int) int {
	result := 0
	for _, v := range elements {
		result += v
	}
	return result
}

func main() {
	inputs := ReadToLines(os.Stdin)
	intInputs := StringsToInts(inputs)

	part1 := 0
	lastMeasurement := intInputs[0]
	for _, val := range intInputs[1:] {
		if val > lastMeasurement {
			part1 += 1
		}
		lastMeasurement = val
	}

	fmt.Printf("Part 1: %d\n", part1)

	part2 := 0
	lastWindowMeasurement := SumSlice(intInputs[0:3])
	for start := 1; start <= len(intInputs)-3; start++ {
		windowMeasurement := SumSlice(intInputs[start : start+3])
		if windowMeasurement > lastWindowMeasurement {
			part2++
		}
		lastWindowMeasurement = windowMeasurement
	}

	fmt.Printf("Part 2: %d\n", part2)
}
