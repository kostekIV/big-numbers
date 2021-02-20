.intel_syntax noprefix
.text

.globl      mul_const

mul_const:
    ## rdi - dst of the multiplication
    ## rsi - const to added to rdi
    ## rdx - len of rdi

    xor r9, r9
    xor r10, r10
    mov r11, rdx
    xor rdx, rdx
    .loop_begin: ## for r9 = 0; r9 < rdx; r9++
        cmp r11, r9
        jle .loop_end

        mov rax, [rdi]
        mul rsi             ## rax = a[i] * const, rdx = overflow

        add rax, r10
        jnc .if_end_1
            inc rdx
        .if_end_1:

        mov r10, rdx
        mov [rdi], rax

        lea rdi, [rdi + 8]

        inc r9
        jmp .loop_begin
    .loop_end:

    xor r9, r9
    cmp r10, r9
    je .end
    mov [rdi], r10

    .end:
    ret
