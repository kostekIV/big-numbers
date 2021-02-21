.intel_syntax noprefix
.text

.globl      sub_two_slices

sub_two_slices:
    ## rdi, rsi - adresses of slices
    ## rdx      - dst of the substraction (at least len of bigger slice)
    ## rcx      - len of the larger slice
    ## r8       - len of the smaller slice

    xor r10, r10
    xor rax, rax ## rax carry

    push r12

    .for_begin: ## r10 = 0, r10 < r9, r10++
        cmp r8, r10
        jle .for_end

            mov r11, [rdi]
            mov r12, [rsi]

            sub r11, rax
            jae .skip_1
                mov rax, 1
                jmp .skip_end_1
            .skip_1:
                xor rax, rax
            .skip_end_1:

            sub r11, r12
            jae .skip_2
                mov rax, 1
            .skip_2:

            mov [rdx], r11


        lea rsi, [rsi + 8]
        lea rdx, [rdx + 8]
        lea rdi, [rdi + 8]

        inc r10
        jmp .for_begin
    .for_end:

    .for_one_slice_begin:  ## r10 = r9, r10 < r8, r10++
        cmp rcx, r10
        jle .for_one_slice_end

            mov r11, [rdi]

            sub r11, rax
            jae .skip_3
                mov rax, 1
                jmp .skip_end_3
            .skip_3:
                xor rax, rax
            .skip_end_3:

            mov [rdx], r11

        lea rdx, [rdx + 8]
        lea rdi, [rdi + 8]

        inc r10
        jmp .for_one_slice_begin
    .for_one_slice_end:

    pop r12
    ret
