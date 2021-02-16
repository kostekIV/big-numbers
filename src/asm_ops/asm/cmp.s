.intel_syntax noprefix
.text

.globl      cmp_slices

cmp_slices:
    ## Params:
    ## rdi, rsi - adresses of slices
    ## rdx      - len of slices
    ## Return:
    ## rax      - one of 1 - rdi is greater, -1 - rsi is greater, 0 - both are same.

    mov r10, 1
    mov r11, -1

    lea rdi, [rdi + 8*rdx]
    lea rsi, [rsi + 8*rdx]

    xor rax, rax
    xor rcx, rcx

    .begin_loop:
        cmp rdx, rcx
        jle .end_loop

        lea rdi, [rdi - 8]
        lea rsi, [rsi - 8]

        mov r8, [rdi]
        mov r9, [rsi]

        cmp r8, r9
        cmovb rax, r11
        cmova rax, r10
        jne .end_loop

        inc rcx
        jmp .begin_loop
    .end_loop:

    ret
