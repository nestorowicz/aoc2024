package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

func main() {
  var pattern = regexp.MustCompile(`mul\(([0-9]{1,3}),([0-9]{1,3})\)`)
  var result int64
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
    line := scanner.Text()
    matches := pattern.FindAllStringSubmatch(line, -1)
    for _, match := range matches {
      if len(match) < 3 {
        continue
      }
      num1, err := strconv.ParseInt(match[1], 10, 64)
      if err != nil {
        fmt.Println(err)
        continue
      }
      num2, err := strconv.ParseInt(match[2], 10, 64)
      if err != nil {
        fmt.Println(err)
        continue
      }
      result += num1*num2
    }
	}
  fmt.Println(result)
}
