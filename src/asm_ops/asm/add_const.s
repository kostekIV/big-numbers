.intel_syntax noprefix
.text

.globl      add_const

add_const:
    ## rdi - dst of the addition
    ## rsi - const to added to rdi
    ## rdx - base

    mov r8, [rdi]
    mov rax, rsi

    .begin:
        add r8, rax
        xor rax, rax
        cmp r8, rdx
        jl .end
        setge al
        sub r8, rdx
        mov [rdi], r8
        lea rdi, [rdi + 8]
        mov r8, [rdi]
        jmp .begin
    .end:

    mov [rdi], r8

    ret
