# Ch1

In this chapter, we will make the isolation between software and hardware. To form a function library, which can be called by apps.

## Computer basics

Cpu can recognize Memory as a big byte array, the physical address corresponde to an index to access the array, but different from out daily coding habit, the index usually doesn't start with 0, but a const number, like `0x80000000`. In short, cpu can addressing by physical address, and access data in memory byte-by-byte.

When Cpu access memory(physical memory or I/O peripherals' data space) in multiple bytes(2/4/8), will introduce endian and memory address alignment problems.

> Endian or Tailorder: also called bytes order, little-endian: lower bit in lower memory address; big-endian: lower bit in higher memory address. x86,RISC-V use little-endian.

> Memory Address Alignment: Include basic type data alignment and struct data alignment. Based on physical component of computer, the CPU read or write data is based on byte-block. Data-Bus decide each r/w's data bits. Address-Bus decide addressing range. In demand of perfermance, Cpu usually require the first address of each data are integer multipled by 4 or 8.</br>basic type data alignment, means that the offset address for each data need to be interger multipled by one word. Struct data alignment, means fill some zero bytes in the gaps of every data field in struct, to ensure each data field can align based on basic type data.

## Load Bootloader and OS to QEMU

The boot of QEMU devide into three stages:

1. The PC of QEMU initialized to `0x1000`, after several instructions, PC will point to `0x80000000`, and step to stage 2;
2. We need to put `rustsib-qemu.bin` into the memory address start with `0x80000000`, it will do some initialize work for boot, and step into next stage. We can use this step to move the control of computer to our `os.bin`, need to mention that different bootloader's next stage addresses are different, it is `0x80002000` for RustSBI.
3. We need to make sure our os address start with `0x80002000` to dock with RustSBI, so we need to load our os at the address `0x80002000`, and we need to make sure the first address of our os bin is the address of the first instruction. To complete this, we need to learn program memory layout and compile progress.

## Program Memory Layout and Compile Progress

### Program Memory Layout

After out source code compiled to executable file, it can al least be divided in to two pars: `Code` and `Data`. The data part is made by instructions which can be decode and executed by CPU, the data part is only recognized by CPU as memory spaces can be wrote and read.

In fact we can devide them into smaller unit - `Section`. Different section will be put in different location of memory, which form the Program's `Memory Layout`.

Below is a typical program relative memory layout
```
 ────────────┬────────────────┐            
             │                │High Address
             │    stack       │            
             │                │            
             ├───────┬────────┤            
             │       │        │            
             │       │        │            
             │       ▼        │            
             │                │            
             │       ▲        │            
             │       │        │            
Data Memory  │       │        │            
             ├───────┴────────┤            
             │    heap        │            
             │                │            
             ├────────────────┤            
             │    .bss        │            
             │                │            
             ├────────────────┤            
             │    .data       │            
             │                │            
             ├────────────────┤            
             │    .rodata     │            
             │                │            
  ───────────┼────────────────┤            
Code Memory  │    .text       │            
             │                │Low Address 
  ───────────┴────────────────┘            
```

- Code section(functions) is only one `.text` section, contains all assembly codes.
- Data section can be divided into:
    - `.rodata` and `.data` - Initialized data section, the former contains contants, the latter contains global mutable data.
    - `.bss` section - Uninitialized data secion, usually is made zero by bootloader.
    - `heap` - Store dynamic allocated data at run time, like `malloc/new` in C/C++, which grows to higher address.
    - `stack` - Used as function context store and recover, and function local variables, grows to lower address.

> Global variables and Local variables: `.data`, `.bss` store global variables, which is accessed by `gp`(x3) register and a offset. </br>Functions' input arguments or local variables are store in some registers or the stack frame of this fuction. If is in stack frame, it is accessed by stack pointer(sp) and a offset.</br>Dynamic variables in heap is located in heap, the size of them only can be confirmed at run time. We need to access them by a pointer to heap. This pointer can be put in stack frame or global variable section.

### Compile Progress

1. Compiler transfer source codes from high level to assembly language, at this time they are still text files.
2. Assembler transfer each assembly file to machine code, to get a binary Object File.
3. Linker link all Object Files generated by source codes, and some possible external Object Files together, to form a complete executable file.

> Each Object File has its own Program Memory Layout, the Linker link all of them to a whole Memory Layout. Then replace the symbol with certain address. Because when we program in modules, each module will export global variables, functions to other modules, and access other modules' exported contents. In machine level, to index the content from other modules, is using their address.</br>In each Object File, the symbols are transferred to certain interior addresses. But to confirm symbols from other modules, we need a `Symbol Table`, external modules' symbols will be transfer to address at the time it was linked, their memory layouts will be merged. Their addresses are also `Relocated`.

So to get a os image which can run on QEMU, we need to adjust memory layout to make sure the first instruction in `.text` section located in `0x80002000`. Because QEMU's simply copy executable file byte-by-byte to target memory address, we need to remove our executable file's metadata to make sure the first instruction is in the load point.

> Static link will link all instructions to one executable file; Dynamic link will only record used external libraries' symbols, load these libraries at runtime, which will reduce executable file's size. 

## Function call and stack

For normal jump instruction, we just need to set pc register to a specified address.

But for `Function Call`, the return address of each function call is dynamic generated at runtime.

There are two instructions for jump in function call:

instruction | Function
-- | --
jal rd, imm[20:1] | rd <- pc + 4</br>pc <- pc + imm
jalr rd, (imm[11:0])rs | rd <- pc + 4</br>pc <- rs + imm

> For most instruction with general registers, `rs` refers to `Source Register`, `imm` refers to `Immediate`, the two forms input part; And `rd` refers to `Destination Register`, which is the output part. `rs` and `rd` can be chosen between `x0~x31` 

In RISC-V, `rd <- pc + 4`'s rd usually is `ra`(`x1`), so when the function returns, just jump to the address stored in `ra`. In fact, at return of function, we usually use an `pseudo` instruction - `ret`, which will be translated to `jalr x0, (0)x1`, means `x0 <- pc + 4 && pc <- x1 + 0`, because the `x0` is constant to 0, `x0 <- pc + 4` is ignored, so the pc will jump to `x1`(`ra`) stored address.

In conclusion, when process function call, we use `jalr` to save return address and realize jump; At the time the function returns, we use pseudo instruction `ret` to jump to the address stored in `ra`.

But we need to ensure the `ra` stay unchanged in the whole lifetime the function executed, or the `ret` will jump to wrong address, for nested function call, we need to ensure the `ra` equals at the function entry and when function returns, as well as other general registers which the function will use as local variables. The registers needs to retain unchanged before and after the function call, are called `Function Call Context`.

Because CPU only has one suit of registers, we need to use physical memory to `Save` `Function Call Context`, after the call, we `Restore` them. The registers in `Function Call Context` are divided into two classes:
- Callee-Saved - Saved by callee.
- Caller-Saved - Saved by caller.

The process is:
- Firstly, caller saves the registers don't want to be changed, and use `jal/jalr` to call function, after return, restore these registers.
- Then, at the beginning of callee, save the registers that this function will rewrite, then do things, before return, restore these registers.

The save and restore operation also calls `Prologue` and `Epilogue`, which is generated automatically by compilor.

> There could be optimization in compilor, because the caller saved registers don't need to be saved by callee. And the registers that the callee don't rewrite are also don't need to be saved by caller. That can shrink the time that function call uses, and memory space.

## Calling Convention

Promise the realization of function call for a programming language on a instruction architecture set. Includes:

- The transfer method of function arguments and return value
- The divide rule for Caller/Callee saved registers
- Registers' usage in function call

On RISC-V, they are:

register group | saver | function
-- | -- | --
a0~a7(x10~x17) | Caller | for pass arguments, need to store before call function, a0 and a1 also used to save return value
t0~t6(x5~x7, x28~x31) | Caller | as temp register, callee never need to save
s0~s11(x8~x9,x18~x27) | Callee | as temp register, callee need to store before using it
ra(x1) | Callee | record the return address
sp(x2) | Callee | Stack Pointer, record the next stack top address
fp(s0) | Callee | Frame Pointer, record current stack frame's start address, also can be used as s0 temp register.
gp(x3) tp(x4)| | Won't change in the whole execute lifetime
zero(x0) | | Constant to be 0

> We use a bss.stack area as our stack space, but we don't need to make this space to all-zero as bss area, so we set .sbss after bss.stack.


## First introduction to RISCV privilege levels

- The OS kernel we write is Supervisor privilege level
- The RustSBI is Machine privilege level, also called Supervisor Execution Environment(SEE)

The interface between Machine level software and Supervisor is called Supervisor Binary Interface(SBI).

SBI Specification stipulated the functions that the SBI included, which is maintained by RISCV community.

RustSBI implements most of the SBI stipulated functions, but not the only one SBI.

## Call RustSBI

We can't call RustSBI by function call, because RustSBI isn't linked to our OS kernel, but the community provides `sbi_rt`, which encapsulations interfaces to call SBI service.

We can use `console_putchar(79); console_putchar(75);` which print utf-8 code, to pritn "OK" to console.

> `sbi-rt` under the hood: We just need to put function extension ID and Function ID in `a7` and `a6` register, and follow RISC-V's Calling Convention to put variables in other registers, then execute `ecall`, the control will transfer to RustSBI, after the procedure, the return value will store in `a0` and `a1` register.
