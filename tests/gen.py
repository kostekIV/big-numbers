from random import randint


n = 500

m = 10 ** 1000

for i in range(n):
    a, b = randint(m // 10 ** 100, m), randint(m // 10 ** 100, m)

    print(f"{a},{b},{a + b}")


