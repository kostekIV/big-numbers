.intel_syntax noprefix
.text

.globl      add_two_slices

add_two_slices:
    ## rdi, rsi - adresses of slices
    ## rdx      - dst of the addition (at least len of bigger slice + 1)
    ## rcx      - base
    ## r8       - len of the larger slice
    ## r9       - len of the smaller slice

    xor r10, r10

    push r12
    mov r12, 1

    .for_begin: ## r10 = 0, r10 < r9, r10++
        cmp r9, r10
        jle .for_end

            ## add from the first slice
            mov r11, [rdi]
            add [rdx], r11

            ## add from the second slice
            mov r11, [rsi]
            add [rdx], r11

            ## if addition result is larger than base rebase and add carry to next value
            cmp [rdx], rcx
            jl .if_end_1
                sub [rdx], rcx
                add [rdx + 8], r12
            .if_end_1:


        lea rsi, [rsi + 8]
        lea rdx, [rdx + 8]
        lea rdi, [rdi + 8]

        inc r10
        jmp .for_begin
    .for_end:

    ## loop only through bigger slice
    .for_one_slice_begin:  ## r10 = r9, r10 < r8, r10++
        cmp r8, r10
        jle .for_one_slice_end

            ## add from first slice
            mov r11, [rdi]
            add [rdx], r11


            ## if addition result is larger than base rebase and add carry to next value
            cmp [rdx], rcx
            jl .if_end_2
                sub [rdx], rcx
                add [rdx + 8], r12
            .if_end_2:

        lea rdx, [rdx + 8]
        lea rdi, [rdi + 8]

        inc r10
        jmp .for_one_slice_begin
    .for_one_slice_end:

    pop r12
    ret
