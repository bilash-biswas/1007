package main

import "fmt"

func main() {
	var num1 int
	var num2 int
	var num3 int
	var num4 int
	fmt.Scanln(&num1)
	fmt.Scanln(&num2)
	fmt.Scanln(&num3)
	fmt.Scanln(&num4)
	var result int = (num1 * num2 - num3 * num4)
	fmt.Printf("DIFERENCA = %d\n", result)
}
