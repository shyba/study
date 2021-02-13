primitive OpenedDoor
    fun name(): String => "closed door"
primitive ClosedDoor
    fun name(): String => "opened door"

type DoorState is (OpenedDoor | ClosedDoor)

primitive Door
    fun open(): DoorState => OpenedDoor
    fun close(): DoorState => ClosedDoor


actor Main
    new create(env: Env) =>
        env.out.print(Door.open().name())
        env.out.print(Door.close().name())
        let some_door = Door
        let isOpen: Bool = match some_door.open()
            | OpenedDoor => true
            | ClosedDoor => false
        end
        let message = if isOpen then
            "welcome"
        else
            "get off"
        end
        env.out.print(message)