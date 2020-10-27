symbols ¬∧∨
1)
a)
P = "have a reading assignment"
Q = "have homework problems
(P∨Q)∧¬(P∧Q)

b)
P = "go skiing"
Q = "have snow"
¬P∨(P∧¬Q)

c)
P = "√7 > 2"
Q = "√7 = 2"
P∨Q

2)
a)
P = "John is telling the truth"
Q = "Bill is telling the truth"
(P∧Q)∨¬(P∨Q)

b)
P = "have fish"
Q = "have chicken"
R = "have mashed potatoes"
(P∨Q)∧¬(P∧R)

c)
P = "3 is a divisor of 6"
Q = "3 is a divisor of 9"
R = "3 is a divisor of 15"
P∧Q∧R

3)
P = "Alice is in the room"
Q = "Bob is in the room"
a) ¬P∨¬Q
b) ¬P∧¬Q
c) ¬P∨¬Q
d) ¬P∧¬Q

4)
P = "Ed is tall"
Q = "Ralph is tall"
R = "Ed is handsome"
S = "Ralph is handsome"
a) (P∧Q)∨(R∧S)
b) (P∨R)∨(Q∨S)
c) (¬P∨¬R)∨(¬Q∨¬S)
d) ¬(P∧R)∨¬(Q∨S)

5) a and c

6)
P = "I will buy the pants"
S = "I will buy the shirt"
a) I will not buy the pants without the shirt
b) I will not buy the pants and the shirt
c) Either I will not buy the pants or I will not buy the shirt

7)
S = "Steve is happy"
G = "George is happy"
a) Either Steve or George is happy and either Steve or George is not happy
b) George is not happy or either Steve is happy or George is happy and steve is not happy
c) Either Steve is happy or George is happy and Steve is unhappy

8)
T = "Taxes will go up"
D = "The deficit will go up"
a) Either taxes or deficit will go up
b) Taxes and deficit wont both go up or down together
c) Either taxes will go up and deficit go down or deficit will go up and taxes go down

9)
a)
Jm = "Jane will win the math prize"
Pm = "Pete will win the math prize"
Jc = "Jane will win the chemistry prize"
Pc = "Pete will win the chemistry prize"
¬(Jm∧Pm)∧(Pm∨Pc)∧Jm -> Pc
(¬Jm∨¬Pm)∧(Pm∨Pc)∧Jm -> Pc
Jm has to be True due the last AND
PM has to be False due the first OR with JM being True
PC has to be True as PM is False on the second OR
Therefore, the conclusion is valid

b)
B = "main course is beef"
F = "main course is fish"
P = "vegetable is peas"
C = "vegetable is corn"

(B∨F)∧(P∨C)∧¬(F∧C) -> ¬(B∧P)
- either B or P needs to be false for the reasoning to be valid
- if B is false:
  - F is True so the first OR is True
  - C is False so the last AND is False, being True after the NOT
  - P is True due the middle OR
- if P is false:
  - C is True due the middle OR
  - F is False so the last AND becomes False then negated to True
  - B is True due the first OR
- Therefore, if both B and P are False:
  - F is True due the first OR
  - C is True due the second OR
  - last AND is True, negating to False and making the expression false
  - which is a contradiction
so, this is a contradiction when B and P are False

c)
J = "John is telling the truth"
B = "Bill is telling the truth"
S = "Sam is telling the truth"
(J∨B)∧(¬S∨¬B) -> J∨¬S

- J is False:
  - B is True due the first OR
  - S is False due the second OR
- J is True:
  - S and B are free as the second OR doesnt depend on J
- J is False and S is True
  - both premise and conclusion are False, which is ok
that is a valid statement

d)
S = "sales will go up"
B = "boss will be happy"
E = "expenses will go up"
(S∧B)∨(E∧¬B) -> ¬S∨¬E

- S and E are False:
  - First AND is False
  - Second AND is False
  - Premise is False but conclusion is True, which is a contradiction
Therefore this expression isnt valid

----------------------------------------------------

## 1.2

1)
a)
 P | Q | ¬P∨Q |
---|---|------|
T  | T |  T   |
T  | F |  F   |
F  | T |  T   |
F  | F |  T   |

b)
 S | G | (S∨G)∧(¬S∨¬G) |
---|---|---------------|
T  | T |  F            |
T  | F |  T            |
F  | T |  T            |
F  | F |  F            |

2)
a)
 P | Q | ¬[P∧(Q∨¬P)] |
---|---|-------------|
T  | T |     F       |
T  | F |     T       |
F  | T |     T       |
F  | F |     T       |

b)
 P | Q | R | (P∨Q)∧(¬P∨R)|
---|---|---|-------------|
T  | T | T |      T      |
T  | T | F |      F      |
T  | F | T |      T      |
T  | F | F |      F      |
F  | T | T |      T      |
F  | T | F |      F      |
F  | F | T |      F      |
F  | F | F |      F      |

3)
a)
 P | Q |    P+Q      |
---|---|-------------|
T  | T |     F       |
T  | F |     T       |
F  | T |     T       |
F  | F |     F       |

b)
 P | Q |(P∧¬Q)∨(¬P∧Q)|
---|---|-------------|
T  | T |     F       |
T  | F |     T       |
F  | T |     T       |
F  | F |     F       |

4)
P∨Q
¬(¬P∧¬Q)
 P | Q |  ¬(¬P∧¬Q)   |     P∨Q     |
---|---|-------------|-------------|
T  | T |      T      |      T      |
T  | F |      T      |      T      |
F  | T |      T      |      T      |
F  | F |      F      |      F      |

5)
a and b)
 P | Q |   ¬(P∨Q)    |     P↓Q     |
---|---|-------------|-------------|
T  | T |      F      |      F      |
T  | F |      F      |      F      |
F  | T |      F      |      F      |
F  | F |      T      |      T      |

c)
 P | Q | ¬P|(P∨Q)|(P∧Q)|P↓Q|P↓P|¬(P↓Q)|¬P↓¬Q|
---|---|---|-----|-----|---|---|------|-----|
T  | T | F |  T  |  T  | F | F |    T |  T  |
T  | F | F |  T  |  F  | F | F |    T |  F  |
F  | T | T |  T  |  F  | F | T |    T |  F  |
F  | F | T |  F  |  F  | T | T |    F |  F  |

6)

P|Q|¬P|(P∨Q)|(P∧Q)|P⊼Q|¬(P∧P)| P⊼P|¬P⊼¬Q|¬(P⊼Q)|
-|-|--|-----|-----|---|------|-----|-----|-------|
T|T|F |  T  |  T  | F |   F  |  F  |  T  |    T |
T|F|F |  T  |  F  | T |   T  |  F  |  T  |    F |
F|T|T |  T  |  F  | T |   T  |  T  |  T  |    F |
F|F|T |  F  |  F  | T |   T  |  T  |  F  |    F |

7)
a)
Jm | Pm | Pc | ¬(Jm∧Pm)∧(Pm∨Pc)∧Jm | Pc |
---|----|----|---------------------|----|
 T |  T | T  |         F           |  T |
 T |  T | F  |         F           |  F |
 T |  F | T  |         T           |  T |
 T |  F | F  |         F           |  F |
 F |  T | T  |         F           |  T |
 F |  T | F  |         F           |  F |
 F |  F | T  |         F           |  T |
 F |  F | F  |         F           |  F |

b)
(B∨F)∧(P∨C)∧¬(F∧C) -> ¬(B∧P)
 B |  F | P  | C  |(B∨F)∧(P∨C)∧¬(F∧C)|¬(B∧P)| valid |
---|----|----|----|------------------|------|-------|
 T |  T | T  | T  |         F        |   F  |   T   |
 T |  T | T  | F  |         T        |   F  |   F   |
 T |  T | F  | T  |         F        |   T  |   T   |
 T |  T | F  | F  |         F        |   T  |   T   |
 T |  F | T  | T  |         T        |   F  |   F   |
 T |  F | T  | F  |         T        |   F  |   F   |
 T |  F | F  | T  |         T        |   T  |   T   |
 T |  F | F  | F  |         F        |   T  |   T   |
 F |  T | T  | T  |         F        |   T  |   T   |
 F |  T | T  | F  |         T        |   T  |   T   |
 F |  T | F  | T  |         F        |   T  |   T   |
 F |  T | F  | F  |         F        |   T  |   T   |
 F |  F | T  | T  |         F        |   T  |   T   |
 F |  F | T  | F  |         F        |   T  |   T   |
 F |  F | F  | T  |         F        |   T  |   T   |
 F |  F | F  | F  |         F        |   T  |   T   |

c)
(J∨B)∧(¬S∨¬B) -> J∨¬S
 B |  J | S  | (J∨B)∧(¬S∨¬B) |J∨¬S|
---|----|----|---------------|-----|
 T |  T | T  |       F       |  T  |
 T |  T | F  |       T       |  T  |
 T |  F | T  |       F       |  F  |
 T |  F | F  |       T       |  T  |
 F |  T | T  |       T       |  T  |
 F |  T | F  |       T       |  T  |
 F |  F | T  |       F       |  F  |
 F |  F | F  |       F       |  T  |

d)
(S∧B)∨(E∧¬B) -> ¬S∨¬E
 B |  E | S  | (S∧B)∨(E∧¬B)  |¬S∨¬E|invalid|
---|----|----|---------------|-----|-------|
 T |  T | T  |       T       |  F  |   *   |
 T |  T | F  |       F       |  T  |       |
 T |  F | T  |       T       |  T  |       |
 T |  F | F  |       F       |  T  |       |
 F |  T | T  |       T       |  F  |   *   |
 F |  T | F  |       T       |  T  |       |
 F |  F | T  |       F       |  T  |       |
 F |  F | F  |       F       |  T  |       |


8)
P|Q|(P∧Q)∨(¬P∧¬Q)| ¬P∨Q | (P∨¬Q)∧(Q∨¬P) | ¬(P∨Q) | (Q∧P)∨¬P |
-|-|-------------|------|---------------|--------|----------|
T|T|      T      |  T   |       T       |    F   |     T    |
T|F|      F      |  F   |       F       |    F   |     F    |
F|T|      F      |  T   |       F       |    F   |     T    |
F|F|      T      |  T   |       T       |    T   |     T    |

- a) to c)
- b) to e)
- c) to a)
- d) none
- e) to b)

9)
P|Q| (P∨Q) | ¬P∨¬Q | ¬P∧¬Q |
-|-|-------|-------|-------|
T|T|   T   |   F   |   F   |
T|F|   T   |   T   |   F   |
F|T|   T   |   T   |   F   |
F|F|   F   |   T   |   T   |

P|Q|R| [P∧(Q∨¬R)]∨(¬P∨R) |
-|-|-|-------------------|
T|T|T|         T         |
T|T|F|         T         |
T|F|T|         T         |
T|F|F|         T         |
F|T|T|         T         |
F|T|F|         T         |
F|F|T|         T         |
F|F|F|         T         |

a) neither
b) contradiction
c) tautology
d) tautology

10)

a)
2nd DeMorgan law: ¬(P∨Q) is equivalent to ¬P∧¬Q

P|Q| ¬(P∨Q) | ¬P∧¬Q | P∧(Q∨R) | (P∧Q)∨(P∧R) | P∨(Q∧R) | (P∨Q)∧(P∨R) |
-|-|--------|-------|---------|-------------|---------|-------------|
T|T|   F    |   F   |         |             |         |             |
T|F|   F    |   F   |         |             |         |             |
F|T|   F    |   F   |         |             |         |             |
F|F|   T    |   T   |         |             |         |             |

b)
Distributive lars:
- P∧(Q∨R) is equivalent to (P∧Q)∨(P∧R)
- P∨(Q∧R) is equivalent to (P∨Q)∧(P∨R)

P|Q|R| P∧(Q∨R) | (P∧Q)∨(P∧R) | P∨(Q∧R) | (P∨Q)∧(P∨R) |
-|-|-|---------|-------------|---------|-------------|
T|T|T|    T    |      T      |    T    |      T      |
T|T|F|    T    |      T      |    T    |      T      |
T|F|T|    T    |      T      |    T    |      T      |
T|F|F|    F    |      F      |    T    |      T      |
F|T|T|    F    |      F      |    T    |      T      |
F|T|F|    F    |      F      |    F    |      F      |
F|F|T|    F    |      F      |    F    |      F      |
F|F|F|    F    |      F      |    F    |      F      |

11)
a)
```
¬(¬P∧¬Q)
P∨Q
```

b)
```
(P∧Q)∨(P∧¬Q)
P∧(Q∨¬Q)
P
```

c)
```
¬(P∧¬Q)∨(¬P∧Q)
(¬P∨Q)∨(¬P∧Q)
((¬P∨Q)∨¬P)∧((¬P∨Q)∨Q)
(Q∨(¬P∨¬P))∧(¬P∨(Q∨Q))
(Q∨¬P)∧(¬P∨Q)
Q∨(¬P∧¬P)
Q∨¬P
```

12)
a)
```
¬(¬P∨Q)∨(P∧¬R)
(P∧¬Q)∨(P∧¬R)
P∧(¬Q∨¬R)
P∧¬(Q∧R)
```

b)
```
¬(¬P∧Q)∨(P∧¬R)
(P∨¬Q)∨(P∧¬R)
((P∨¬Q)∨P)∧((P∨¬Q)∨¬R)
(P∨¬Q)∧(P∨(¬Q∨¬R))
P∨(¬Q∧(¬Q∨¬R)
P∨¬Q

pitfall: absorption
```

c)
```
(P∧R)∨[¬R∧(P∨Q)]
(P∧R)∨[(¬R∧P)∨(¬R∧Q)]
[(P∧R)∨(¬R∧P)]∨(¬R∧Q)]
[P∧(R∨¬R)]∨(¬R∧Q)]
P∨(¬R∧Q)
```

13)
```
start with what you want to prove
¬P∨¬Q
double negate
¬¬(¬P∨¬Q)
apply De Morgan first rule
¬(P∧Q)
```

14)
```
(P∧Q)∧(R∧S)
[(P∧Q)∧R]∧S
[P∧(Q∧R)]∧S
```

15) 2**n

16) ¬(¬P∧Q)

17) (¬P∧Q)∨(P∧¬Q) which is also an XOR

18)
In order to be valid, when a promise is true the conclusion has to be true. Otherwise they can take any value as long as we cant find a true promise with a false conclusion.

So, if the conclusion is a tautology then the statement is valid as all true values on premises will map to true.

If the conclusion is a contradiction, then its only valid if the premise is a contradiction.

For a premise that is a tautology, in order to be valid, the conclusion must be a also tautology, so all true premises maps to true conclusions. Otherwise, if the premise is a contradiction, then any conclusion is valid.

----------------

1.3 Functions
-----

1)
```
D(w,z) = "w is divisible by z"
```

a)
```
D(6, 3)∧D(9, 3)∧D(15, 3)
```

b)
```
D(x, 2)∧D(x, 3)∧¬D(x, 4)
```

c)
```
N(w) = "w is a natural number"
P(w) = "w is a prime number"

(N(x)∧N(y)∧(P(x) ⊻ P(y))
```

2)
a)
```
M(w) = "w is a man"
T(w, z) = "w is taller than z"
(M(x)∧M(y))∧(T(x, y)⊻T(y, x))
```

b)
```
B(w) = "w has brown eyes"
R(w) = "w has red hair"

(B(x)∨B(y))∧(R(x)∨R(y))
```

c)
```
using R and B from previous item:
S(w) = R(w) ∧ B(w)

S(x) ∨ S(y)
```

3)
```
a) {x| x is a planet in the same system as Earth}
b) {x| x is an american university in Ivy League}
c) {x| x is a state in USA}
d) {x| x is a province of Canada}
```

4)
```
a) {x ∈ ℕ| x²}
b) {n ∈ ℕ| 2ⁿ}
c) {n ∈ ℕ | n ∈ {0...9} | n + 10}
```

5)
```
a) x is bound and there is no free variable. Statement is true, as x satisfies it
b) x is bound and there is no free variable. It is false as x is positive, thus not belonging to the set of negative real numbers
c) x is bound and c is free.
13 - 2x > c
13 - 10 > c (replaces x with 5)
3 > c
negated, as 5 cant be x:
3 ≤ c
so
{c ∈ ℝ | c ≥ 3 }
```

6)
a)
```
w and c are free. x is bound.
simplification:
13-2x > c
13 > c + 2x
{c ∈ ℝ | x ∈ ℝ | c + 2x < 13 }
```

b)
```
no free variables.
statement is true:
13 - 2(4)
5, which is a prime
```

c)
```
statement is false as 4 isnt a prime number
no free variables.
```

7)
```
2x² + x - 1 = 0
(2x-1)(x+1)
x = -1 || x = 1/2
```

a)
```
{-1, 1/2}
```
b)
```
{1/2}
```
c)
```
{-1}
```
d)
```
{}
```

8)
a)
```
{x | x was once married to Elizabeth Taylor}
{Conrad Hilton Jr, Michael Wilding, Mike Todd, Eddie Fisher, Richard Burton, John Warner, Larry Fortensky}
```

b)
```
{x | x is a logical connection studied in Section 1.1}
{¬, ∨, ∧}
```

c)
```
{x | x is an author of How to Prove it}
{Daniel Velleman}
```

9)
a)
```
{x ∈ ℝ | x² - 4x + 3 = 0 }
x² - 4x + 3 = 0
x(x-3) + (3-x)
(x-3)(x(x-3)/(x-3) + (3-x)/(x-3))
(x-3)(x-1)
{3, 1}
```

b)
```
solution isnt a real number, so: ∅
```

c)
```
5 ∈ {y ∈ ℝ | x² + y² < 50 }
x²+y²<50
y as 5
x²+25<50
x²<50-25
x²<25
{x ∈ ℝ | x² < 25 }
{... -4, -3, -2, -1, 0, 1, 2, 3, 4, ...}
