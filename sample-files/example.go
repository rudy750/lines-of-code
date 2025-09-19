package main

import "fmt"

// Sample Go file
func factorial(n int) int {
    if n <= 1 {
        return 1
    }
    /* Recursive calculation
       of factorial */
    return n * factorial(n-1)
}

func main() {
    fmt.Println("Factorial of 5:", factorial(5))
}