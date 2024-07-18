

```
divisor = 3
while n != 1 and divisor * divisor <= n:
    while n % divisor == 0:
        factors.append(divisor)
        n = n // divisor
    divisor += 2
```

  

This code block handles all odd numbers from 3 onwards
The outer while loop continues as long as n is not 1 and divisor * divisor is less than or equal to n

**n != 1**: This part of the condition ensures that the loop continues as long as n is not reduced to 1. When n becomes 1, it means we've completely factorized the number into its prime factors.

**divisor * divisor <= n**: This part of the condition ensures that we only check potential divisors up to the square root of n. The reason for checking up to the square root is based on mathematical properties of prime factorization.

### Why Check the Square of the Divisor?

To understand why we check divisor * divisor <= n, consider the following points:

 - Prime Factors Beyond Square Root: If n has a factor larger than its square root, it must be paired with a factor smaller than its square root.
 
 - Efficiency: By checking divisors only up to the square root of n, we reduce the number of iterations significantly. For a number n, checking up to its square root means we only need to check roughly $$ \sqrt{n} $$ potential divisors, rather than n potential divisors, which makes the algorithm much more efficient.