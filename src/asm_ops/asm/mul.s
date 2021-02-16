.intel_syntax noprefix
.text

.globl      mul_two_slices

mul_two_slices:
    ## rdi, rsi - adresses of slices
    ## rdx      - dst of the addition (at least len of r8 + r9)
    ## rcx      - base
    ## r8       - len of the larger slice
    ## r9       - len of the smaller slice

    push r12
    push r13
    push r14
    push r15

    ## store address of dest in r15 
    lea r15, [rdx]

    xor r10, r10
    xor r13, r13

    .outer_loop_begin: ## for r10 = 0; r10 < r8; r10++
        cmp r8, r10
        jle .outer_loop_end

        mov r13, r10
        xor r11, r11
        xor r12, r12
        .inner_loop_begin: ## for r11 = 0; r11 < r9; r11++
            cmp r9, r11
            jle .inner_loop_end

            mov rax, [rdi + 8*r10]
            mov r14, [rsi + 8*r11]

            mul r14             ## rax = a[i] * b[j]
            xor rdx, rdx
            div rcx             ## rax = a[i] * b[j] / base, rdx = a[i] * b[j] % base

            add rdx, r12        ## add carry

            ## check if rdx is bigger than base and if it is substract base and increment future carry
            cmp rdx, rcx
            jb .end_adjust1
                sub rdx, rcx
                inc rax
            .end_adjust1:

            ## add to dest[i + j] value in rdx
            add [r15 + 8*r13], rdx

            ## check if dest[i + j] is bigger than base and if it is substract base and increment future carry
            cmp [r15 + 8*r13], rcx
            jb .end_adjust
                sub [r15 + 8*r13], rcx
                inc rax
            .end_adjust:

            ## set carry to value from rax
            mov r12, rax

            inc r11
            inc r13
            jmp .inner_loop_begin
        .inner_loop_end:

        ## forward carry if needed
        xor rax, rax
        .begin_while:
            cmp r12, rax
            je .end_while
            add [r15 + 8*r13], r12
            cmp [r15 + 8*r13], rcx
            jb .end_while
                sub [r15 + 8*r13], rcx
                mov r12, 1
                inc r13
            jmp .begin_while
        .end_while:

        inc r10
        jmp .outer_loop_begin
    .outer_loop_end:

    pop r15
    pop r14
    pop r13
    pop r12

    ret
