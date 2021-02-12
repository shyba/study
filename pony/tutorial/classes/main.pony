class Wombat
    let name: String
    var _hunger_level: U64

    new create(name': String) =>
        name = name'
        _hunger_level = 0
    
    new hungry(name': String, hunger': U64) =>
        name = name'
        _hunger_level = hunger'
    
    fun hunger(): U64 => _hunger_level
    
    fun ref set_hunger(to: U64): U64 => _hunger_level = to

class Hawk
    var _hunger_level: U64 = 0

class Owl
    var _hunger_level: U64 = 0

actor Main
    new create(env: Env) =>
        let bat = Wombat("batman")
        let hungry_bat = Wombat.hungry("robin", 42)
        let hawk = Hawk
        let owl = Owl
        env.out.print(hungry_bat.set_hunger(30).string())
        let hungry_batman = Wombat.hungry("batman", hungry_bat.hunger())
        env.out.print(hungry_batman.hunger().string())
        var c = bat
        var b = bat
        env.out.print((b = c = hungry_batman).hunger().string())
