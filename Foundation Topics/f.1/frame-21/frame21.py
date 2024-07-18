def prime_factors(n):
    factors = []
    while n % 2 == 0:
        factors.append(2)
        n = n // 2

    divisor = 3
    while n != 1 and divisor * divisor <= n:
        while n % divisor == 0:
            factors.append(divisor)
            n = n // divisor
        divisor += 2
    if n > 2:
        factors.append(n)

    return factors


numbers = [84, 512]
for number in numbers:
    print(f"Prime factorisation of {number} is {prime_factors(number)}")
