.code

switch_ctx PROC FRAME
    .endprolog
    movapd      [rcx + 0*8], xmm6
    movapd      [rcx + 2*8], xmm7
    movapd      [rcx + 4*8], xmm8
    movapd      [rcx + 6*8], xmm9
    movapd      [rcx + 8*8], xmm10
    movapd      [rcx + 10*8], xmm11
    movapd      [rcx + 12*8], xmm12
    movapd      [rcx + 14*8], xmm13
    movapd      [rcx + 16*8], xmm14
    movapd      [rcx + 18*8], xmm15
    mov         [rcx + 20*8], rsp
    mov         [rcx + 21*8], r15
    mov         [rcx + 22*8], r14
    mov         [rcx + 23*8], r13
    mov         [rcx + 24*8], r12
    mov         [rcx + 25*8], rbx
    mov         [rcx + 26*8], rbp
    mov         [rcx + 27*8], rdi
    mov         [rcx + 28*8], rsi
    mov         rax, gs:[08h]
    mov         [rcx + 29*8], rax
    mov         rax, gs:[10h]
    mov         [rcx + 30*8], rax

    movapd      xmm6, [rdx + 0*8]
    movapd      xmm7, [rdx + 2*8]
    movapd      xmm8, [rdx + 4*8]
    movapd      xmm9, [rdx + 6*8]
    movapd      xmm10, [rdx + 8*8]
    movapd      xmm11, [rdx + 10*8]
    movapd      xmm12, [rdx + 12*8]
    movapd      xmm13, [rdx + 14*8]
    movapd      xmm14, [rdx + 16*8]
    movapd      xmm15, [rdx + 18*8]
    mov         rsp, [rdx + 20*8]
    mov         r15, [rdx + 21*8]
    mov         r14, [rdx + 22*8]
    mov         r13, [rdx + 23*8]
    mov         r12, [rdx + 24*8]
    mov         rbx, [rdx + 25*8]
    mov         rbp, [rdx + 26*8]
    mov         rdi, [rdx + 27*8]
    mov         rsi, [rdx + 28*8]

    mov         rax, [rdx + 29*8]
    mov         gs:[08h], rax
    mov         rax, [rdx + 30*8]
    mov         gs:[10h], rax

    mov         rcx, [rdx + 31*8]

    ret
switch_ctx ENDP


set_ctx PROC FRAME
    .endprolog
    movapd      xmm6, [rcx + 0*8]
    movapd      xmm7, [rcx + 2*8]
    movapd      xmm8, [rcx + 4*8]
    movapd      xmm9, [rcx + 6*8]
    movapd      xmm10, [rcx + 8*8]
    movapd      xmm11, [rcx + 10*8]
    movapd      xmm12, [rcx + 12*8]
    movapd      xmm13, [rcx + 14*8]
    movapd      xmm14, [rcx + 16*8]
    movapd      xmm15, [rcx + 18*8]
    mov         rsp, [rcx + 20*8]
    mov         r15, [rcx + 21*8]
    mov         r14, [rcx + 22*8]
    mov         r13, [rcx + 23*8]
    mov         r12, [rcx + 24*8]
    mov         rbx, [rcx + 25*8]
    mov         rbp, [rcx + 26*8]
    mov         rdi, [rcx + 27*8]
    mov         rsi, [rcx + 28*8]

    mov         rax, [rcx + 29*8]
    mov         gs:[08h], rax
    mov         rax, [rcx + 30*8]
    mov         gs:[10h], rax

    ret
set_ctx ENDP

END