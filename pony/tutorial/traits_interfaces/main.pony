trait Noisy
    fun make_noise(): String => "@@##$@#"

trait Aesthetic 
    fun aesthetics(): String => "w o w"

interface Human
    fun get_opinion(): String

class AngryMan is (Human & Noisy)
    fun get_opinion(): String => "NOPE"

class AngryAestheticDog is (Aesthetic & Noisy)
    fun make_noise(): String => "b a r k s"

actor NoiseMaker
    let env: Env

    new create(env': Env) =>
        env = env'
    
    be make_some_noise(from: Noisy val) =>
        let noise: String = from.make_noise()
        env.out.print(noise)
    
    be make_some_opinions(from: Human val) =>
        env.out.print(from.get_opinion())

actor Main
    new create(env: Env) =>
        let noisy: AngryMan val = AngryMan
        let noisy2: Noisy val = AngryAestheticDog
        let noise_maker: NoiseMaker = NoiseMaker(env)
        noise_maker.make_some_noise(noisy)
        noise_maker.make_some_noise(noisy2)
        noise_maker.make_some_opinions(noisy)