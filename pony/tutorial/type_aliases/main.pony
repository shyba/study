interface HasAge
    fun age(): U32

interface HasHealth
    fun is_healthy(): Bool

type AgedWell is (HasAge & HasHealth)

primitive SomeJoke[A]

class Joke is AgedWell
    fun age(): U32 => 42
    fun is_healthy(): Bool => true

class GenericJoke[T: AgedWell val]
    let good_thing: T
    new create(what: T) =>
        good_thing = what
    
    fun get_joke(): AgedWell val => good_thing

actor Main
    let myenv: Env
    new create(env: Env) =>
        myenv = env
        let joke: Joke val = Joke
        let some_joke: GenericJoke[Joke val] = GenericJoke[Joke val](joke)
        this.check_aged_well(joke)
        this.check_aged_well(some_joke.get_joke())
    
    be check_aged_well(what: AgedWell val) =>
        if what.is_healthy() then
            myenv.out.print("healthy at age of " + what.age().string())
        end