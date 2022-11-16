# os/src/entry.asm
    .section .text.entry # 把后面的内容放到 .text.entry 的段中，最低的地址放代码
    .globl _start # 全局符号
_start:
    la sp, boot_stack_top
    call rust_main # 跳到rust执行程序

    .section .bss.stack # 低地址存放bss
    .globl boot_stack

boot_stack: # 低地址
    .space 4096 * 16 # 64kB 栈空间
    .globl boot_stack_top
boot_stack_top: # 从高地址向低地址增长，所以把top放在预留空间的后面