1.1
```scheme
10
> 10
(+ 5 3 4)
> 12
(- 9 1)
> 8
(/ 6 2)
> 3
(+ (* 2 4) (- 4 6))
> 6
(define a 3)
>
(define b (+ a 1))
>
(+ a b (* a b))
> 19
(= a b)
> #f
(if (and (> b a) (< b (* a b)))
    b
    a)
> 4
(cond ((= a 4) 6)
      ((= b 4) (+ 6 7 a))
      (else 25))
> 16
(+ 2 (if (> b a) b a))
> 6
(* (cond ((> a b) a)
         ((< a b) b)
         (else -1))
   (+ a 1))
> 16
```

1.2
```scheme
(/ (+ 5 4 (- 2 (- 3 (+ 6 (/ 4 5)))))
   (* 3 (- 6 2) (- 2 7)))
```

1.3
```scheme
(define (sum-squares x y z)
    (define (squares p q) (+ (* p p) (* q q)))
    (cond ((> x y z) (squares x y))
          ((> x z y) (squares x z))
          ((> y z x) (squares y z))
          ((> y x z) (squares y x))
          ((> z x y) (squares z x))
          (else (squares z y))))
```

1.4
```
This function will change from a+b to a-b if b is less than 0, ensuring b as absolute.
```

1.5
```
In normal order the interpreter will "fully expand and then reduce", using values as needed. so:
(test 0 (p)) fully expands to:
(if (= 0 0) 0 (p))
Since its true, 0 is returned.

On applicative order, however, the interpreter will evaluate the arguments then apply. So it will evaluate (p) before applying test, which will make it loop forever.
```
