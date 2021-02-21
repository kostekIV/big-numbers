.intel_syntax noprefix
.text

.globl      add_two_slices

add_two_slices:
    ## rdi, rsi - adresses of slices
    ## rdx      - dst of the addition (at least len of bigger slice + 1)
    ## rcx      - len of the larger slice
    ## r8       - len of the smaller slices

    push r12
    push r13

    xor r10, r10
    xor r13, r13
    xor r12, r12

    .for_begin: ## r10 = 0, r10 < r8, r10++
        cmp r8, r10
        jle .for_end

            ## add from the first slice
            mov r11, [rdi]

            ## add from the second slice
            add r11, [rsi]
            jnc .if_end_1
                inc r13
            .if_end_1:
            add r11, r12
            jnc .if_end_2
                inc r13
            .if_end_2:
            mov r12, r13
            xor r13, r13

            ## if addition result is larger than base rebase and add carry to next value
            mov [rdx], r11


        lea rsi, [rsi + 8]
        lea rdx, [rdx + 8]
        lea rdi, [rdi + 8]

        inc r10
        jmp .for_begin
    .for_end:

    ## loop only through bigger slice
    .for_one_slice_begin:  ## r10 = r9, r10 < r8, r10++
        cmp rcx, r10
        jle .for_one_slice_end

            ## add from first slice
            xor r12, r12
            mov r11, [rdi]
            add r11, r12
            jnc .if_end_3
                mov r12, 1
            .if_end_3:
            mov [rdx], r11

        lea rdx, [rdx + 8]
        lea rdi, [rdi + 8]

        inc r10
        jmp .for_one_slice_begin
    .for_one_slice_end:

    mov [rdx], r12

    pop r13
    pop r12
    ret
