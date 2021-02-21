.intel_syntax noprefix
.text

.globl      div_const

div_const:
    ## Params:
    ## rdi - dst of the division
    ## rsi - const that rdi will be divided by
    ## rdx - len of rdi
    ## Returns:
    ## rax - remainder of the division

    push r12
    push r13
    push r14
    push r15

    xor r8, r8
    mov r9, rdx

    mov rax, -1
    xor rdx, rdx

    div rsi            ## 2^64 - 1 = rsi * rax + rdx
    mov r10, rax       ## 2^64 - 1 = rsi * r10 + rdx
    mov r11, rdx       ## 2^64 - 1 = rsi * r10 + r11

    inc r11

    cmp rsi, r11
    ja .skip_adj
        inc r10
        sub r11, rsi
    .skip_adj:

    ## 2^64 = rsi * r10 + r11


    xor r12, r12
    mov r14, 1

    .begin_loop:
    cmp r9, r8
    jle .end_loop

        cmp r12, 0
        je .set_value
            mov r13, [rdi]
            mov rax, r12
            mul r10 ## no overflow
            mov [rdi], rax

            mov rax, r12
            mul r11

            jnc .no_carry
                .carry:
                push rax
                push rdx
                mov rax, rdx
                mul r10
                add [rdi], rax

                pop rax
                mul r11
                pop r14
                add rax, r14
                jnc .no_add_carry
                    inc rdx
                .no_add_carry:
                cmp rdx, 0
                jne .carry
            .no_carry:

            mov rdx, 1
            add rax, r13

            jnc .no_carry_2
                .carry_2:
                push rax
                push rdx
                mov rax, 1
                mul r10
                add [rdi], rax

                pop rax
                mul r11
                pop r14
                add rax, r14
                jnc .no_add_carry_2
                    inc rdx
                .no_add_carry_2:
                cmp rdx, 0
                jne .carry_2

            .no_carry_2:
            mov r12, rax



            jmp .after_set
        .set_value:
            mov r12, [rdi]
            xor r13, r13
            mov [rdi], r13
        .after_set:


        cmp r12, rsi
        jb .skip
            mov rax, r12
            xor rdx, rdx
            div rsi
            add [rdi], rax
            mov r12, rdx
        .skip:
            ## nop

        inc r8
        lea rdi, [rdi + 8]
    jmp .begin_loop
    .end_loop:

    mov rax, r12

    pop r15
    pop r14
    pop r13
    pop r12

    ret

