section .data
newline: db 10
codes:
    db  '0123456789ABCDEF'

section .text
global _start

print_newline:
    mov rax, 1 ; write
    mov rdi, 1 ; stdout
    mov rsi, newline ; what
    mov rdx, 1 ; 1 byte
    syscall
    ret

print_hex:
    mov rax, rdi
    mov rdi, 1
    mov rdx, 1
    mov rcx, 64
    iterate:
        push rax
        sub rcx, 4
        sar rax, cl ; shift 60, 56, 52 ..
        and rax, 0xf
        lea rsi, [codes + rax]
        mov rax, 1
        push rcx
        syscall
        pop rcx
        pop rax
        test rcx, rcx
        jnz iterate
    
    ret

exit:; 1 parameter, exit value on rdi
    mov rax, 60; syscall exit
    syscall

_start:
    mov rdi, 0x123456890ABCDEF
    mov r12, 2000000
    alot:
        mov rdi, r12
        call print_hex
        call print_newline
        sub r12, 1
        test r12, r12
        jnz alot
    mov rdi, 0
    call exit
