def isPrime(n: int) -> bool:
    for i in range(2, n):
        if n % i == 0:
            return False
    return True

p: int = 2

for i in range(1000):
    while not isPrime(p):
        p += 1
    print(p)
    p += 1
