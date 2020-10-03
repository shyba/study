1. a) 3 as factor, so 2³-1, which give us 7. the other is 32767/7=4681
b) 2³²⁷⁶⁷-1, so 7 is a factor. 2⁷-1=127

2) Suppose n is an integer greater than 1, 3^n-1 is even.

3)
a) 2,3,5,7 so
M=2x3x5x7+1=211

b) 2,5,11
M=2x3x5x7x11+1=2311

4)
```python
isprime = lambda x: all(x%y!=0 for y in range(2,x))

primes = []
for n in range(10000):
    if len(primes) == 5: break
    if isprime(n): primes.clear()
    else: primes.append(n)

primes
[24, 25, 26, 27, 28]
```

5)
from the table, 3 and 7 are valid options, giving 28 and 8128 as perfect

then, continuing it:

n | isPrime | 2^n-1 | is 2^n-1 prime? |
--|---------|-------|-----------------|
11|   yes   | 2047  |       no        |
12|   no    | 4095  |       no        |
13|   yes   | 8191  |       yes       |
14|   no    | 16383 |       no        |
15|   no    | 32767 |       no        |
16|   no    | 65535 |       no        |
17|   yes   | 131071|       yes       |

candidates are 8191 and 131071
from Euclid, 2^(n-1)*(2^n-1) is perfect, so
33550336 and 8589869056 are too

```python
perfect = lambda x: 2**(x-1)*(2**x-1)
isperfect = lambda x: x == sum(i for i in range(1, x) if x%i==0)
for x in range(1,13):
     print(x, isprime(x), 2**x-1, isprime(2**x-1), perfect(x), isperfect(perfect(x)))```
```

output:
```
1 True 1 True 1 False
2 True 3 True 6 True
3 True 7 True 28 True
4 False 15 False 120 False
5 True 31 True 496 True
6 False 63 False 2016 False
7 True 127 True 8128 True
8 False 255 False 32640 False
9 False 511 False 130816 False
10 False 1023 False 523776 False
11 True 2047 False 2096128 False
12 False 4095 False 8386560 False
```
