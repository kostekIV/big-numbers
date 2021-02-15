.intel_syntax noprefix
.text

.globl      sub_const

sub_const:
    ## rdi - dst of the substraction
    ## rsi - const to be subbed from rdi
    ## rdx - base

    mov r8, [rdi]
    mov rax, rsi

    .begin:
        cmp rax, r8
        jle .end
        add r8, rdx
        sub r8, rax
        mov rax, 1
        mov [rdi], r8
        lea rdi, [rdi + 8]
        mov r8, [rdi]
        jmp .begin
    .end:

    sub [rdi], rax

    ret
