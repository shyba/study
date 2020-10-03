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
