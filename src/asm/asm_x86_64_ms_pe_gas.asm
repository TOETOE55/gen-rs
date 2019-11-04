.file	"asm_x86_64_ms_pe_gas.asm"

.text
.p2align 4,,15
.globl	switch_ctx
.def	switch_ctx;	.scl	2;	.type	32;	.endef
.seh_proc	switch_ctx
switch_ctx:
.seh_endprologue
    movaps      %xmm6, 0x00(%rcx)
    movaps      %xmm7, 0x10(%rcx)
    movaps      %xmm8, 0x20(%rcx)
    movaps      %xmm9, 0x30(%rcx)
    movaps      %xmm10, 0x40(%rcx)
    movaps      %xmm11, 0x50(%rcx)
    movaps      %xmm12, 0x60(%rcx)
    movaps      %xmm13, 0x70(%rcx)
    movaps      %xmm14, 0x80(%rcx)
    movaps      %xmm15, 0x90(%rcx)
    mov         %rsp, 0xa0(%rcx)
    mov         %r15, 0xa8(%rcx)
    mov         %r14, 0xb0(%rcx)
    mov         %r13, 0xb8(%rcx)
    mov         %r12, 0xc0(%rcx)
    mov         %rbx, 0xc8(%rcx)
    mov         %rbp, 0xd0(%rcx)
    mov         %rdi, 0xd8(%rcx)
    mov         %rsi, 0xe0(%rcx)
    mov         %gs:0x08, %rax
    mov         %rax, 0xe8(%rcx)
    mov         %gs:0x10, %rax
    mov         %rax, 0xf0(%rcx)

    movaps      0x00(%rdx), %xmm6
    movaps      0x10(%rdx), %xmm7
    movaps      0x20(%rdx), %xmm8
    movaps      0x30(%rdx), %xmm9
    movaps      0x40(%rdx), %xmm10
    movaps      0x50(%rdx), %xmm11
    movaps      0x60(%rdx), %xmm12
    movaps      0x70(%rdx), %xmm13
    movaps      0x80(%rdx), %xmm14
    movaps      0x90(%rdx), %xmm15
    mov         0xa0(%rdx), %rsp
    mov         0xa8(%rdx), %r15
    mov         0xb0(%rdx), %r14
    mov         0xb8(%rdx), %r13
    mov         0xc0(%rdx), %r12
    mov         0xc8(%rdx), %rbx
    mov         0xd0(%rdx), %rbp
    mov         0xd8(%rdx), %rdi
    mov         0xe0(%rdx), %rsi

    mov         0xe8(%rdx), %rax
    mov         %rax, %gs:0x08
    mov         0xf0(%rdx), %rax
    mov         %rax, %gs:0x10
    mov         0xf8(%rdx), %rcx

    ret
.seh_endproc

.section .drectve
.ascii " -export:\"switch_ctx\""


.text
.p2align 4,,15
.globl	set_ctx
.def	set_ctx;	.scl	2;	.type	32;	.endef
.seh_proc	set_ctx
set_ctx:
.seh_endprologue
    movaps      0x00(%rcx), %xmm6
    movaps      0x10(%rcx), %xmm7
    movaps      0x20(%rcx), %xmm8
    movaps      0x30(%rcx), %xmm9
    movaps      0x40(%rcx), %xmm10
    movaps      0x50(%rcx), %xmm11
    movaps      0x60(%rcx), %xmm12
    movaps      0x70(%rcx), %xmm13
    movaps      0x80(%rcx), %xmm14
    movaps      0x90(%rcx), %xmm15
    mov         0xa0(%rcx), %rsp
    mov         0xa8(%rcx), %r15
    mov         0xb0(%rcx), %r14
    mov         0xb8(%rcx), %r13
    mov         0xc0(%rcx), %r12
    mov         0xc8(%rcx), %rbx
    mov         0xd0(%rcx), %rbp
    mov         0xd8(%rcx), %rdi
    mov         0xe0(%rcx), %rsi
    mov         0xe8(%rcx), %rax
    mov         %rax, %gs:0x08
    mov         0xf0(%rcx), %rax
    mov         %rax, %gs:0x10
    ret
.seh_endproc

.section .drectve
.ascii " -export:\"set_ctx\""