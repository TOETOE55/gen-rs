.text
.globl switch_ctx
.type switch_ctx,@function
.align 16
switch_ctx:
    mov     %rsp, 0x00(%rdi)
    mov     %r15, 0x08(%rdi)
    mov     %r14, 0x10(%rdi)
    mov     %r13, 0x18(%rdi)
    mov     %r12, 0x20(%rdi)
    mov     %rbx, 0x28(%rdi)
    mov     %rbp, 0x30(%rdi)

    mov     0x00(%rsi), %rsp
    mov     0x08(%rsi), %r15
    mov     0x10(%rsi), %r14
    mov     0x18(%rsi), %r13
    mov     0x20(%rsi), %r12
    mov     0x28(%rsi), %rbx
    mov     0x30(%rsi), %rbp
    mov     0x38(%rsi), %rdi
    ret
.size switch_ctx,.-switch_ctx

.text
.globl set_ctx
.type set_ctx,@function
.align 16
set_ctx:
    mov     0x00(%rdi), %rsp
    mov     0x08(%rdi), %r15
    mov     0x10(%rdi), %r14
    mov     0x18(%rdi), %r13
    mov     0x20(%rdi), %r12
    mov     0x28(%rdi), %rbx
    mov     0x30(%rdi), %rbp
    ret
.size set_ctx,.-set_ctx

.section .note.GNU-stack,"",%progbits