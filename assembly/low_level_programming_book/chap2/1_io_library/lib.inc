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
    jle .fail
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
    .fail:
        mov rax, 0
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

;rdi and rsi
string_equals:
    xor rax, rax
    mov r9, -1
    .loop:
        inc r9
        mov al, byte[rdi+r9]
        cmp al, byte[rsi+r9]
        jne .notsame
        cmp al, 0
        jg .loop
        cmp byte[rsi+r9], 0
        jne .notsame
    .same:
        mov rax, 1
        ret
    .notsame:
        mov rax, 0
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
    .numlen:
        cmp byte[rdi+rax], 48
        jl .numlen_exit
        cmp byte[rdi+rax], 57
        jg .numlen_exit
        inc rax
        jmp .numlen
    .numlen_exit:
        mov r8, rax
        dec rax
        mov r9, rax
    push r12
    ;xor r9, r9 ; counter
    mov r10, 1 ; multiplier
    mov r11, 0 ; sum
    mov r12, 10; factor
    .parse:
        xor rax, rax
        xor rdx, rdx
        mov al, byte[rdi+r9]
        sub rax, 48 ; 48 is 0 in ascii
        cmp rax, 0
        jl .exit
        cmp rax, 9
        jg .exit
        mul r10
        add r11, rax
        dec r9
        mov rax, r10
        mul r12
        mov r10, rax
        cmp r9, 0
        jl .exit
        jmp .parse
    .exit:
    mov rax, r11
    mov rdx, r8
    pop r12
    ret

; rdi points to a string
; returns rax: number, rdx : length
parse_int:
    xor rax, rax
    cmp byte[rdi], 45
    je .negative
    .positive:
        call parse_uint
        ret 
    .negative:
        inc rdi
        call parse_uint
        push rdx
        mov r9, -1
        mul r9
        pop rdx
        inc rdx
        ret

