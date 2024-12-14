package problems

import (
	"fmt"
	"sort"
	"strconv"
	"strings"

	"github.com/ctheil/aoc/2024/util"
)

func DayOne() {
	partOne()
	partTwo()
}

const INPUT = `3   4
4   3
2   5
1   3
3   9
3   3`

func partOne() {
	// PARSE DATA
	file_input, err := util.GetInput("day_one")
	if err != nil {
		msg := fmt.Errorf("error parsing file input: %e", err)
		panic(msg)
	}
	// lines := strings.Split(file_input, "\n")
	lines := util.SplitByLine(file_input)

	l_data := []int{}
	r_data := []int{}
	fmt.Printf("data len: %d \n", len(lines))

	// LOOP AND STORE DATA IN l_data AND r_data
	for _, line := range lines {
		data := strings.Split(line, "   ")
		// fmt.Printf("data: %s \n data[0]: %s \n data[1]: %s", data, data[0], data[1])
		l_data_point, err := strconv.Atoi(data[0])
		if err != nil {
			msg := fmt.Errorf("error parsing int from string: %e", err)
			panic(msg)
		}
		l_data = append(l_data, l_data_point)

		r_data_point, err := strconv.Atoi(data[1])
		if err != nil {
			msg := fmt.Errorf("error parsing int from string: %e", err)
			panic(msg)
		}
		r_data = append(r_data, r_data_point)
	}

	// SORT DATA
	sort.Slice(l_data, func(i, j int) bool {
		return l_data[i] < l_data[j]
	})
	sort.Slice(r_data, func(i, j int) bool {
		return r_data[i] < r_data[j]
	})

	dist := 0

	for idx := range l_data {
		l := l_data[idx]
		r := r_data[idx]

		diff := util.Abs(r - l)
		dist += diff
	}

	fmt.Printf("\n Part One: %d \n", dist)
}

func partTwo() {
	// LOCAL TEST
	// lines := util.SplitByLine(INPUT)

	file_input, err := util.GetInput("day_one")
	if err != nil {
		msg := fmt.Errorf("error parsing file input: %e", err)
		panic(msg)
	}
	// lines := strings.Split(file_input, "\n")
	lines := util.SplitByLine(file_input)
	r_data := make(map[int]int)

	for _, line := range lines {
		data := strings.Split(line, "   ")
		r_int, err := strconv.Atoi(data[1])
		if err != nil {
			msg := fmt.Errorf("error parsing int from string: %e", err)
			panic(msg)
		}
		_, ok := r_data[r_int]
		if ok {
			r_data[r_int] += 1
		} else {
			r_data[r_int] = 1
		}
	}

	sim_score := 0

	for _, line := range lines {
		data := strings.Split(line, "   ")
		l_int, err := strconv.Atoi(data[0])
		if err != nil {
			msg := fmt.Errorf("error parsing int from string: %e", err)
			panic(msg)
		}

		i, ok := r_data[l_int]
		if !ok {
			i = 0
		}
		score := l_int * i
		sim_score += score
	}

	fmt.Printf("\n Part Two: %d \n", sim_score)
}
