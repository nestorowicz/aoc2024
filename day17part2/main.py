expected = list(reversed([2,4,1,6,7,5,4,4,1,7,0,3,5,5,3,0]))
def check(a, n):
    for i in range(n,-1,-1):
        if ((a % 8 ^ 6) ^ (a // pow(2, a % 8 ^ 6) ^ 7)) % 8 != expected[i]:
            return False
        a//=8
    return True
a = 1
for i in range(len(expected)):
    while not check(a, i):
        a += 1
    a *= 8
print(a//8)
