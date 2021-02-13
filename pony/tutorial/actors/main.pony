actor Printer
    let env: Env

    new create(env': Env) =>
        env = env'

    be print(message: String) =>
        env.out.print(message)

actor Sum
    var _rolling_sum: U64 = 0

    be sum(value: U64) =>
        _rolling_sum = _rolling_sum + value
    
    be print_report(p: Printer) =>
        p.print("The sum is " + _rolling_sum.string())

actor Main
    new create(env: Env) =>
        let p: Printer = Printer(env)
        let s: Sum = Sum
        s.sum(22)
        s.print_report(p)