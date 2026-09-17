package main

import "fmt"

func main() {

	var revenue float64
	var expenses float64
	var taxRate float32

	fmt.Print("Enter Revenue: ")
	fmt.Scan(&revenue)
	fmt.Print("Enter Expenses: ")
	fmt.Scan(&expenses)
	fmt.Print("Enter Tax Rate: ")
	fmt.Scan(&taxRate)

	var ebt float64
	var eat float64
	var ratio float32

	ebt = revenue - expenses
	eat = float64(1-taxRate/100) * ebt
	ratio = float32(eat / ebt)

	fmt.Printf("Earnings Before Taxes: $%.2f \nEarnings After Taxes: $%.2f \nDebt Ratio: %v \n", ebt, eat, ratio)
}
