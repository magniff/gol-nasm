section .text
global factorial, add_two

add_two:
    mov rax, rdi
    add rax, rsi
    ret

factorial:
    cmp rdi, 1
    jle .base_case
    push rdi
    dec rdi
    call factorial
    pop rdi
    imul rax, rdi
    ret

.base_case:
    mov rax, 1
    ret