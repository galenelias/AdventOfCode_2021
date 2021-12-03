package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
)

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

func Part1(lines []string) {
	var gammaStr string
	var epsilonStr string

	for i := 0; i < len(lines[0]); i++ {
		countOnes := 0
		for _, v := range lines {
			if v[i] == '1' {
				countOnes++
			}
		}

		if countOnes >= len(lines)/2 {
			gammaStr += "1"
			epsilonStr += "0"
		} else {
			gammaStr += "0"
			epsilonStr += "1"
		}
	}

	gamma, _ := strconv.ParseInt(gammaStr, 2, 64)
	epsilon, _ := strconv.ParseInt(epsilonStr, 2, 64)
	fmt.Printf("Part 1 = %d\n", gamma*epsilon)
}

func FilterNumbers(lines []string, bitOffset int, computeOxygen bool) []string {
	var result []string

	if len(lines) == 1 {
		return lines
	}

	countOnes := 0
	for _, v := range lines {
		if v[bitOffset] == '1' {
			countOnes++
		}
	}

	var filterBit byte
	if computeOxygen {
		if countOnes >= (len(lines)+1)/2 {
			filterBit = '1'
		} else {
			filterBit = '0'
		}
	} else {
		if countOnes >= (len(lines)+1)/2 {
			filterBit = '0'
		} else {
			filterBit = '1'
		}
	}

	for _, v := range lines {
		if v[bitOffset] == filterBit {
			result = append(result, v)
		}
	}
	return result
}

func Part2(lines []string) {
	oxygenNums := lines
	co2Nums := lines

	for i := 0; i < len(lines[0]); i++ {
		oxygenNums = FilterNumbers(oxygenNums, i, true /*computeOxygen*/)
		co2Nums = FilterNumbers(co2Nums, i, false /*computeOxygen*/)
	}

	oxygen, _ := strconv.ParseInt(oxygenNums[0], 2, 64)
	co2, _ := strconv.ParseInt(co2Nums[0], 2, 64)

	fmt.Printf("Part 2 = %d\n", oxygen*co2)
}

func main() {
	lines := ReadToLines(os.Stdin)

	Part1(lines)
	Part2(lines)
}
