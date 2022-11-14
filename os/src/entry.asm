# os/src/entry.asm
    .section .text.entry # 把后面的内容放到 .text.entry 的段中
    .globl _start # 全局符号
_start:
    li x1, 100 