.intel_syntax noprefix
.text

.globl      mul_const

mul_const:
    ## rdi - dst of the multiplication
    ## rsi - const to added to rdi
    ## rdx - base
    ## rcx - len of rdi

    xor r9, r9
    xor r10, r10
    mov r11, rdx
    .loop_begin: ## for r9 = 0; r9 < rcx; r9++
        cmp rcx, r9
        jle .loop_end

        mov rax, [rdi]
        mul rsi             ## rax = a[i] * const
        xor rdx, rdx
        div r11             ## rax = a[i] * const / base, rdx = a[i] * const % base

        add rdx, r10

        cmp rdx, r11
        jl .end_adjust1
            sub rdx, r11
            inc rax
        .end_adjust1:

        mov r10, rax
        mov [rdi], rdx

        lea rdi, [rdi + 8]

        inc r9
        jmp .loop_begin
    .loop_end:

    mov [rdi], r10

    ret
