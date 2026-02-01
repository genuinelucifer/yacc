    .globl main
main:
    pushq %rbp
    movq %rsp, %rbp
    subq $24, %rsp
    movl $2, -4(%rbp)
    negl -4(%rbp)
    movl -4(%rbp), %r10d
    movl %r10d, -8(%rbp)
    notl -8(%rbp)
    movl -8(%rbp), %r10d
    movl %r10d, -12(%rbp)
    negl -12(%rbp)
    movl -12(%rbp), %r10d
    movl %r10d, -16(%rbp)
    notl -16(%rbp)
    movl -16(%rbp), %r10d
    movl %r10d, -20(%rbp)
    notl -20(%rbp)
    movl -20(%rbp), %r10d
    movl %r10d, -24(%rbp)
    negl -24(%rbp)
    movl -24(%rbp), %eax
    movq %rbp, %rsp
    popq %rbp
    ret
.section .note.GNU-stack,"",@progbits
