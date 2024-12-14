package problems

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/ctheil/aoc/2024/util"
)

func DayTwo() {
	part_one()
	part_two()
}

const L_INPUT = `7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9`

type ReportNode struct {
	data int
	next *ReportNode
	prev *ReportNode
}
type ReportList struct {
	head *ReportNode
}

func (l *ReportList) Add(data int) {
	if l.head == nil {
		l.head = &ReportNode{
			data: data,
			next: nil,
			prev: nil,
		}
		return
	}
	curr := l.head
	for curr.next != nil {
		curr = curr.next
	}
	newNode := &ReportNode{
		data: data,
		next: nil,
		prev: curr,
	}
	curr.next = newNode
}

// func (l *ReportList) Remove(data int) {
//
// }

func apply_tolerance(n *ReportNode, tolerance int, used_tolerance *int) (ok bool) {
	if *used_tolerance >= tolerance {
		return false
	}
	*used_tolerance++
	// REMOVE NODE
	if n.prev != nil {
		n.prev.next = n.next
	}
	if n.next != nil {
		n.next.prev = n.prev
	}
	return true
}

func is_report_safe(report *ReportList, tolerance int) bool {
	// start with 2nd node && use prev
	curr := report.head.next
	report_print := []int{report.head.data}
	used_tolerance := 0

	var dir string
	// fmt.Print("IS_REPORT_SAFE: \n", report, tolerance)

	for curr != nil {
		if dir == "" {
			if curr.prev.data < curr.data {
				dir = "inc"
			} else {
				dir = "dec"
			}
		}
		// fmt.Printf("\n     Checking inc/dec: \n     prev: %d\n     curr: %d\n", curr.prev.data, curr.data)
		if dir == "inc" && curr.prev.data > curr.data {
			ok := apply_tolerance(curr, tolerance, &used_tolerance)
			if !ok {
				fmt.Print("\nUNSAFE: inc failed: REPORT_PRINT: \n", report_print)
				return false
			}
			fmt.Printf("Applied tolerance (inc failed): %d\n", used_tolerance)

			curr = report.head.next // restart after removing
			report_print = []int{report.head.data}
			continue
		} else if dir == "dec" && curr.prev.data < curr.data {
			ok := apply_tolerance(curr, tolerance, &used_tolerance)
			if !ok {
				fmt.Print("\nUNSAFE: dec failed: REPORT_PRINT: \n", report_print)
				return false
			}
			fmt.Printf("Applied tolerance (dec failed): %d\n", used_tolerance)
			// fmt.Printf("UNSAFE: dec failed; dec tolerance from: %d, to %d\n", used_tolerance, used_tolerance+1)
			curr = report.head.next // restart after removing
			report_print = []int{report.head.data}
			continue
		}

		// CHECK DIF
		diff := util.Abs(curr.prev.data - curr.data)
		// fmt.Printf("\n     Checking diff: %d", diff)
		if diff > 3 || diff < 1 {
			ok := apply_tolerance(curr, tolerance, &used_tolerance)
			if !ok {
				fmt.Print("\nUNSAFE: dec failed: REPORT_PRINT: \n", report_print)
				return false
			}
			fmt.Printf("Applied tolerance (unsafe diff): %d\n", used_tolerance)
			curr = report.head.next // restart after removing
			report_print = []int{report.head.data}
			continue
		}

		report_print = append(report_print, curr.data)

		curr = curr.next
	}

	fmt.Print("\nREPORT_PRINT: \n", report_print)
	return true
}

func str_to_report(str string) (ReportList, error) {
	strs := strings.Split(str, " ")
	var r_list ReportList

	for _, s := range strs {
		n, err := strconv.Atoi(s)
		if err != nil {
			return r_list, fmt.Errorf("error parsing string to int: %e", err)
		}
		r_list.Add(n)
	}

	return r_list, nil
}

func part_one() {
	data, err := util.GetInput("day_two")
	if err != nil {
		panic(err)
	}
	lines := util.SplitByLine(data)
	// lines := util.SplitByLine(L_INPUT)

	num_safe_reports := 0
	for _, line := range lines {
		report, err := str_to_report(line)
		if err != nil {
			panic(err)
		}
		is_safe := is_report_safe(&report, 0)
		if is_safe {
			num_safe_reports++
		}

	}

	fmt.Printf("[day_two/part_one]: Safe Reports: %d\n", num_safe_reports)
}

func part_two() {
	data, err := util.GetInput("day_two")
	if err != nil {
		panic(err)
	}
	lines := util.SplitByLine(data)
	fmt.Print("\nPartTwo: lines: ", len(lines))
	// lines := util.SplitByLine(L_INPUT)

	num_safe_reports := 0
	for _, line := range lines {
		report, err := str_to_report(line)
		if err != nil {
			panic(err)
		}
		is_safe := is_report_safe(&report, 1)
		if is_safe {
			num_safe_reports++
		}

	}

	fmt.Printf("[day_two/part_two]: Safe Reports: %d\n", num_safe_reports)
}
