.intel_syntax noprefix
.text

.globl      sub_two_slices

sub_two_slices:
    ## rdi, rsi - adresses of slices
    ## rdx      - dst of the substraction (at least len of bigger slice)
    ## rcx      - base
    ## r8       - len of the larger slice
    ## r9       - len of the smaller slice

    xor r10, r10
    xor rax, rax ## rax carry

    push r12
    push r13

    .for_begin: ## r10 = 0, r10 < r9, r10++
        cmp r9, r10
        jle .for_end

            ## add from first slice
            mov r11, [rdi]
            mov r12, [rsi]
            add r12, rax

            xor r13, r13
            cmp r12, r11
            setg al
            cmovg r13, rcx
            mov [rdx], r13

            sub [rdx], r12
            add [rdx], r11


        lea rsi, [rsi + 8]
        lea rdx, [rdx + 8]
        lea rdi, [rdi + 8]

        inc r10
        jmp .for_begin
    .for_end:

    .for_one_slice_begin:  ## r10 = r9, r10 < r8, r10++
        cmp r8, r10
        jle .for_one_slice_end

            ## add from first slice
            mov r11, [rdi]
            mov r12, rax

            xor r13, r13
            cmp rax, r11
            setg al
            cmovg r13, rcx
            mov [rdx], r13

            sub [rdx], r12
            add [rdx], r11

        lea rdx, [rdx + 8]
        lea rdi, [rdi + 8]

        inc r10
        jmp .for_one_slice_begin
    .for_one_slice_end:

    pop r13
    pop r12
    ret
