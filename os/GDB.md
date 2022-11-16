# GDB 的使用

## remote connect

```shell
riscv64-unknown-elf-gdb \
    -ex 'file target/riscv64gc-unknown-none-elf/release/os' \
    -ex 'set arch riscv:rv64' \
    -ex 'target remote localhost:1234'
```

qemu specify port: `-gdb tcp:localhost:port -S` instead of `-s -S`

## frequent

- si 下一步
- x/10i $pc  打印pc
- p/x $t0  打印寄存器
- b *0x80200000  设置断点 **注意设置断点的时候在地址前加`\*`**
- c  continue
