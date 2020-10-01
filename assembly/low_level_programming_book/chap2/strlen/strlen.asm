section .data
test_string: db "abcdefg", 0
numbers: db "0123456789", 0

section .text
global _start

strlen:
    xor rax, rax
    .loop:
        cmp byte [rdi+rax], 0
        je .end
        inc rax
        jmp .loop
    .end:
        ret

print_newline:
    mov rsi, numbers
    add rsi, rdi
    mov rax, 1 ; write
    mov rdi, 1 ; stdout
    mov rdx, 1 ; 1 byte
    syscall
    ret

exit:; 1 parameter, exit value on rdi
    mov rax, 60; syscall exit
    syscall

_start:
    mov rdi, test_string
    call strlen
    mov rdi, rax
    call print_newline
    xor rdi, rdi
    call exit
