; vim: set filetype=nasm :
section .data
fail_msg: db 'panic: buffer too small', 0
char_buf: db 0
numbers: db '0123456789'

section .text

_write: ; whatever is on rsi, rdx len!
    mov rax, 1 ; write
    mov rdi, 1 ; stdout
    syscall
    ret

_fail:
    mov rsi, fail_msg
    mov rdx, 1
    call _write

string_length:
    xor rax, rax
    .loop:
        cmp byte[rdi+rax], 0
        jz .done
        inc rax
        jmp .loop
    .done:
        ret

print_string:
    call string_length
    mov rdx, rax
    mov rsi, rdi
    call _write
    ret

string_copy:
    call string_length
    cmp rdx, rax
    jl _fail
    mov rdx, rax
    xor rax, rax
    .loop:
        mov al, byte[rdi+rdx]
        mov byte[rsi+rdx], al
        dec rdx
        cmp rdx, 0
        jl .exit
        jmp .loop
    .exit:
        ret


print_char:
	push rdi
    mov rsi, rsp
    mov rdx, 1
    call _write
	pop rdi
    ret

print_newline:
    xor rax, rax
    ret

print_uint:
    mov rax, rdi ; store quotient, which starts as the argument rdi
    mov r10, 10
    push 0
    .loop:
        xor rdx, rdx ; clean rdx as its used on divisor (rdx:rax)
        div r10 ; divide by 10
        mov r9b, byte[numbers+rdx]
        push r9
        cmp rax, 0 ; test quotient
        jnz .loop ; repeat if its not zero
    .print_loop:
        pop rdi
        cmp rdi, 0
        jz .exit
        call print_char
        jmp .print_loop
    .exit:
        ret

print_int:
    xor rax, rax
    cmp rdi, 0
    jge .positive
    mov r9, rdi
    mov rdi, 45
    call print_char
    mov rdi, r9
    neg rdi
    .positive:
    call print_uint
    ret

string_equals:
    xor rax, rax
    ret


read_char:
	push 0
    xor rax, rax
    mov rsi, rsp
    mov rdi, 0 ; stdint
    mov rdx, 1 ; size
    syscall
	pop rax
    mov rdi, rax
    ret 

read_word:
	; accepts a buffer address (rdi) and size (rsi)
    ; writes to buffer skipping whitespaces
    ; returns 0 if buffer too small
    mov rdx, 0 ; tracks where in the buffer we are
	.loop:
    push rsi
    push rdi
    push rdx
	call read_char
    pop rdx
    pop rdi
    pop rsi
    cmp rax, 0
    je .exit
    cmp rax, 0x20
    je .check_end
    cmp rax, 0x9
    je .check_end
    cmp rax, 0xA
    je .check_end
	mov [rdi+rdx], rax
    add rdx, 1
	cmp rdx, rsi
	jg .too_big
    jmp .loop
	.exit:
    mov byte[rdi+rdx], 0
    mov rax, rdi
    ret
    .too_big:
    mov rax, 0
    ret
    .check_end:
    cmp rdx, 0
    jg .exit
    jmp .loop

; rdi points to a string
; returns rax: number, rdx : length
parse_uint:
    xor rax, rax
    ret

; rdi points to a string
; returns rax: number, rdx : length
parse_int:
    xor rax, rax
    ret 

