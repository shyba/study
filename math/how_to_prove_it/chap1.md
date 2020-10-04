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

a) to c)
b) to e)
c) to a)
d) none
e) to b)
