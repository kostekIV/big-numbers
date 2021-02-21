.intel_syntax noprefix
.text

.globl      sub_const

sub_const:
    ## rdi - dst of the substraction
    ## rsi - const to be subbed from rdi

    mov r8, [rdi]
    mov rax, rsi

    .begin:
        sub r8, rax
        jae .end
        mov [rdi], r8
        mov rax, 1
        lea rdi, [rdi + 8]
        mov r8, [rdi]
        jmp .begin
    .end:

    sub [rdi], rax

    ret
