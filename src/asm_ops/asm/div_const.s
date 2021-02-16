.intel_syntax noprefix
.text

.globl      div_const

div_const:
    ## Params:
    ## rdi - dst of the division
    ## rsi - const that rdi will be divided by
    ## rdx - base
    ## rcx - len of rdi
    ## Returns:
    ## rax - remainder of the division

    push r12
    push r13
    push r14

    xor r8, r8

    mov rax, rdx
    xor rdx, rdx

    div rsi            ## base = rsi * rax + rdx
    mov r10, rax       ## base = rsi * r10 + rdx
    mov r13, rdx       ## base = rsi * r10 + r13

    xor r11, r11

    .begin_loop:
    cmp rcx, r8
    jle .end_loop

    mov r14, [rdi]

    xor r12, r12
    cmp r11, r12
    je .acc_empty
        mov rax, r13
        mul r11
        add r14, rax
        mov rax, r10
        mul r11
        mov r12, rax
    .acc_empty:


    mov r11, r14
    cmp r14, rsi
    jb .loop_epilogue
        mov rax, r14
        xor rdx, rdx
        div rsi
        add r12, rax
        mov r11, rdx
    .loop_epilogue:

    mov [rdi], r12
    lea rdi, [rdi + 8]
    inc r8
    jmp .begin_loop
    .end_loop:

    mov rax, r11

    pop r14
    pop r13
    pop r12

    ret

