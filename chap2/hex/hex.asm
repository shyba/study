global _start

section .data
codes:
    db  '0123456789ABCDEF'

section .text
_start:
    mov rax, 0x123456890ABCDEF
    mov rdi, 1
    mov rdx, 1
    mov rcx, 64
    .loop:
        push rax
        sub rcx, 4
        sar rax, cl ; lower part of rcx (c lower)
        and rax, 0xf

        lea rsi, [codes + rax] ; at addr
        mov rax, 1 ; write syscall

        push rcx
        syscall
        pop rcx

        pop rax
        test rcx, rcx ; 0?
        jnz .loop
    mov rax, 60 ; sycall exit
    xor rdi,rdi ; 0
    syscall
