name = strlen
all:
	nasm -felf64 $(name).asm -o $(name).o
	ld -o $(name) $(name).o
	chmod u+x $(name)
	./$(name)
