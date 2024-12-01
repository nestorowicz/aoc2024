package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
  var (
    nums1 []int
    nums2 = make(map[int]int)
  ) 
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
    nums := strings.Split(scanner.Text(), "   ")

    num1, err := strconv.ParseInt(nums[0], 10, 32)
    if err != nil {
      panic(err)
    }
    nums1 = append(nums1, int(num1))

    num2int64, err := strconv.ParseInt(nums[1], 10, 32)
    if err != nil {
      panic(err)
    }
    num2 := int(num2int64)
    nums2[num2] += 1
	}

  score := 0
  for i:=0; i<len(nums1); i++ {
    score += nums1[i] * nums2[nums1[i]]
  }

  fmt.Printf("%d", score)
}
