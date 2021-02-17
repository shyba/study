actor Misbehave
    let env: Env
    new create(env': Env) =>
        env = env'
    be oh_no(number: U32) =>
        try
            U32(10) /? number
            env.out.print("ok")
        else
            env.out.print("boom")
            return
        end

actor Main
    new create(env: Env) =>
        env.out.print("f")
        literals(env)
    
    fun literals(env: Env) =>
        var unsigned: U32 = 42_000
        unsigned = U32(20)
        unsigned = 'hehe'
        env.out.print(unsigned.string())
        unsigned = '\x00\x00\x00\x01'
        env.out.print(unsigned.string())
        unsigned = '🐎'
        unsigned = unsigned * 4
        env.out.print(if ((U32('🐎') * 4) / 4) == U32('🐎') then "same" else "not quite" end)
        env.out.print(if ((U64('🐎') * 4) / 4) == U64('🐎') then "same" else "not quite" end)
        env.out.print(((U32('🐎') * 4) / 4).string())
        env.out.print((U32('🐎') == U32('🐎')).string())
        env.out.print(U32('🐎').string())
        env.out.print(U64('🐎').string())
        env.out.print((U32('🐎') * U32(4)).string())
        env.out.print("🐎")
        Misbehave(env).oh_no(1)
        Misbehave(env).oh_no(0)