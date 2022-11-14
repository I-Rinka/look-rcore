# 裸机环境

## 初始

创建工程： `cargo new os --bin` 使用 `--bin` 选项告诉Cargo创建的是可执行项目而不是库函数

mac 安装工具的方法：

```shell
brew tap riscv/riscv
brew install riscv-tools
```

对于m1 mac，需要自行编译qemu

### 移除依赖

把rust目标编译平台设置为 `unknown`

```shell
rustup target add riscv64gc-unknown-none-elf
```

设置cargo生成的目标

```t
# os/.cargo/config
[build]
target = "riscv64gc-unknown-none-elf"
```

> riscv GC: G: IMAFD（基本整数指令，乘除法，原子，浮点）, C: 压缩指令

在开头加上 `#![no_std]` 不用标准库，加上 `#![no_main]` 移除初始化工作

> The C runtime then invokes the entry point of the Rust runtime, which is marked by the start language item. Rust only has a very minimal runtime, which takes care of some small things such as setting up stack overflow guards or printing a backtrace on panic.

然后自行实现panic (和rust语言相关的都放在 `lang_item.rs` 文件中)

### 检查生成的二进制代码

使用 `binutils` 工具集可以检查header和文件头等信息

```shell
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

cargo生成的文件在 `./target/debug` 中

```shell
file target/riscv64gc-unknown-none-elf/debug/os # 文件格式

rust-readobj -h target/riscv64gc-unknown-none-elf/debug/os # 文件头信息

rust-objdump -S target/riscv64gc-unknown-none-elf/debug/os # 反汇编导出汇编程序
```

## 内核执行裸机代码

### 开机

开机的三个阶段：

1. qemu汇编代码初始化 - 将计数器初始化为 `0x1000` ，之后再跳到 `0x80000000`
2. bootloader初始化 - 存放sbi，跳到内核地址入口地址，为我们选定的固定的 `0x80200000`
3. 内核镜像

命令详解:

```shell
qemu-system-riscv64 \
    -machine virt \ # 计算机使用virt的配置
    -nographic \ # 无图形界面
    -bios ../bootloader/rustsbi-qemu.bin \
    -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000 # 将文件加载到物理内存指定位置，file: 文件的路径，addr: 物理地址

```

virt的起始地址为 `0x80000000`，我们会将 bootloader 加载到 `0x80000000` ， 将内核镜像加载到 `0x80200000` 。 `0x80200000` 是 rustSBI 选定的

在 `.cargo/config` 中调整链接器的行为来自定义内存布局：

```t
[target.riscv64gc-unknown-none-elf]
rustflags = [
    "-Clink-arg=-Tsrc/linker.ld", "-Cforce-frame-pointers=yes"
]
```

这里表明使用 `src/linker.ld` 文件进行布局

### 内核第一条指令

```assembly
# os/src/entry.asm
    .section .text.entry # 把后面的内容放到 .text.entry 的段中
    .globl _start # 全局富豪
_start:
    li x1, 100 
```

之后在rust中使用`global_asm`宏来实现汇编的调用。

### 数据的丢弃

在编译后， `BASE_ADDR` `0x80200000` 会放很多和elf文件格式有关的元数据，导致对应地址读出来的不是我们的指令，因此需要对编译后文件进行裁剪.

```shell
riscv64-unknown-elf-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os -O binary target/riscv64gc-unknown-none-elf/release/os.bin
```

### GDB 验证启动流程

```shell
qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios ../bootloader/rustsbi-qemu.bin \
    -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000 \
    -s -S # GDB连接参数
```

`-s` 选项：让 `qemu` 监听TCP 1234端口，等待GDB连接；`-S` 是收到GDB请求后才开始允许。

GDB的连接：

```shell
riscv64-unknown-elf-gdb \
    -ex 'file target/riscv64gc-unknown-none-elf/release/os' \
    -ex 'set arch riscv:rv64' \
    -ex 'target remote localhost:1234'
```
