from random import randint


n = 500

m = 10 ** 1000

files = [
    "mul_test.csv",
    "add_test.csv",
    "sub_test.csv"
]

ops = [
    lambda x,y: x * y,
    lambda x,y: x + y,
    lambda x,y: x - y,
]

for op, fi in zip(ops, files):
    with open(fi, "w") as f:
        for i in range(n):
            a, b = randint(-m, m), randint(-m, m)

            print(f"{a},{b},{op(a, b)}", file=f)


