# Sample Python file
def fibonacci(n):
    """Calculate the nth Fibonacci number."""
    if n <= 1:
        return n
    else:
        # Recursive approach
        return fibonacci(n-1) + fibonacci(n-2)

if __name__ == "__main__":
    # Print first 10 Fibonacci numbers
    for i in range(10):
        print(f"F({i}) = {fibonacci(i)}")