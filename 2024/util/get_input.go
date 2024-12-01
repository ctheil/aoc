package util

import (
	"os"
	"strings"
)

func GetInput(path string) (string, error) {
	p := "./data/" + path
	data, err := os.ReadFile(p)
	return string(data[:]), err
}

func SplitByLine(contents string) []string {
	lines := strings.Split(contents, "\n")

	if len(lines) > 0 && lines[len(lines)-1] == "" {
		lines = lines[:len(lines)-1]
	}

	return lines
}
