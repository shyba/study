actor Main
    new create(env: Env) =>
        var tuple: (String, U32)
        tuple = ("oi", 42)
        env.out.print(tuple._1)
        var age_or_name: (String | U32)
        age_or_name = "name"
        env.out.print(age_or_name.string())
        age_or_name = 42
        env.out.print(age_or_name.string())