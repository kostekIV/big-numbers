.intel_syntax noprefix
.text

.globl      add_const

add_const:
    ## rdi - dst of the addition
    ## rsi - const to added to rdi
    ## rdx - base

    mov r8, [rdi]
    clc

    .begin:
        add r8, rsi
        jnc .end
        mov [rdi], r8
        lea rdi, [rdi + 8]
        mov r8, [rdi]
        mov rsi, 1
        jmp .begin
    .end:

    mov [rdi], r8

    ret
