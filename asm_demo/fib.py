def fib(n):
    a = 0
    b = 1
    while n:
        tmp = a
        a = b
        b = tmp + b
        n -= 1
    return a

print(fib(6))