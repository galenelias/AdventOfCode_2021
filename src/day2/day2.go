package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
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
	hPos := 0
	depth := 0

	for _, v := range lines {
		amt := 0
		if parsed, _ := fmt.Sscanf(v, "forward %d", &amt); parsed > 0 {
			hPos += amt
		} else if parsed, _ := fmt.Sscanf(v, "down %d", &amt); parsed > 0 {
			depth += amt
		} else if parsed, _ := fmt.Sscanf(v, "up %d", &amt); parsed > 0 {
			depth -= amt
		}
	}

	fmt.Printf("Part 1: %d\n", hPos*depth)
}

func Part2(lines []string) {
	hPos := 0
	depth := 0
	aim := 0

	for _, v := range lines {
		amt := 0
		if parsed, _ := fmt.Sscanf(v, "forward %d", &amt); parsed > 0 {
			hPos += amt
			depth += aim * amt
		} else if parsed, _ := fmt.Sscanf(v, "down %d", &amt); parsed > 0 {
			aim += amt
		} else if parsed, _ := fmt.Sscanf(v, "up %d", &amt); parsed > 0 {
			aim -= amt
		}
	}

	fmt.Printf("Part 2: %d\n", hPos*depth)
}

func main() {
	lines := ReadToLines(os.Stdin)

	Part1(lines)
	Part2(lines)
}
