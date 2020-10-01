global _start

section .data
message: db 'hello world', 10
endmessage:

section .text
_start:
    mov rax, 1 ; write system call
    mov rdi, 1 ; arg1, where to write (fd 1)
    mov rsi, message ; what to write, pointer to message
    mov rdx, endmessage-message ; size
    syscall
    
    mov rax, 60 ; exit syscall
    mov rdi, 0 ; return 0
    syscall
