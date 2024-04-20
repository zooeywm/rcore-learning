# Ch2

## Introduce

Protect system safety and support for multiple app is two kernel targets of OS. In this chapter, we will realize:

- Build single executable program contains os kernel and multiple applications.
- Support multiple programs' auto load and run via batch processing.
- Use hardware privilege mechanism to realize self protecting.
- Realize privilege level crossing.
- Support system call by crossing privilege level.

Batch System is for executing programs with a few or no user input. It can apply programs to run automatically with plenty of resources, which is called `Batch Job`.

The main idea of Batch System is to package multiple programs into the computer, after a program finished, the computer will automatically load the next program.

`Privilege Mechanism` - protect computer from destructed by error programs: terminate the error program, and run the next program. Make the applications run on User Mode, the os run on the Supervisor Mode.

## Privilege Mechanism

We limit the app can't access any address, and can't execute any instruction possibly to destroy os.

We also need to make sure the app can access os's service, make os as an important part of app execution environment.

To realize it, the CPU set two different level of privilege: User Mode and Supervisor Mode, and explicitly specify the instructions may destroy the os. Make sure these instructions only can be executed in Kernel Mode, if they are executed on User Mode, will generate exception.

When use `call` or `ret`, will dismiss CPU privilege check, RISC-V provides `ecall`(Execution Environment Call) and `eret`(Execution Environment Return).

- `ecall` - switch from User Mode to Supervisor Mode
- `sret` - switch form Supervisor Mode to User Mode

> Association between `sret` and `eret`: `eret` represent for a set of instructions, `sret` specially is executed from `Supervisor Mode`, which OS runs on meanwhile, `mret` is executed from `Machine Mode`, which RustSBI runs on.

As the CPU has these mechanism, it still needs the corporation of OS to finally complete the protection of OS. Firstly, the OS needs to provide function codes, which can prepare and restore User Mode Context before `sret`. Besides, when app calls `ecall`, OS needs to check the call arguments, ensure it won't destroy the OS.

In RISC-V, there are four privilege levels:

Level | Code | Name
-- | -- | --
0 | 00 | U, User/Appication
1 | 01 | S, Supervisor
2 | 10 | H, Hypervisor
3 | 11 | M, Machine

For our OS, we only need U, and S modes, which runs on the M mode.

The Higher Level can monitor Lower Level, it is, when Lower Level generate error, it will paused and run Higher Level code, this progress usually(but not always) come with CPU privilege Level switch. After Higher Level code finished, we back to the point where Lower Level paused and continue. This is called `ECF`(Exception Control Flow), is one of the `Trap`s on RISC-V.

There are two situations Exception is generated from calling from Lower Level to Higher Level:

- App execute special instruction to acquire OS service
- Error generated during execution in an U Mode instruction, and catched by CPU.

Interrupt | Exception Code | Description
-- | -- | --
0 | 0 | Instruction address misaligned
0 | 1 | Instruction access fault
0 | 2 | Illegal instruction
0 | 3 | Breakpoint
0 | 4 | Load address misaligned
0 | 5 | Load address fault
0 | 6 | Store/AMO access misaligned
0 | 7 | Store/AMO access fault
0 | 8 | Environment call from U-mode
0 | 9 | Environment call from S-mode
0 | 11 | Environment call from M-mode
0 | 12 | Instruction page fault
0 | 13 | Load page fault
0 | 14 | Store/AMO page fault

`Breakpoint` and `Environment Call` are intensional exceptions, also called `trap`. Others are faults.

Interface between SEE(Supervisor Execution Environment) runs on M-mode and S-mode is called (Supervisor Binary Interface).
Interface between OS runs on S-mode and U-mode is called (Application Binary Interface), also called `syscall`(System Call).
They are instructions in form of Machine code or Assembly code, so are called Binary Interface.
The switch between them are Trap Exception Control Flow, shown like below:

```
  Application                OS                      SEE       
                                                               
┌─────────────┐         ┌─────────────┐         ┌─────────────┐
│             │         │             │         │             │
│             │         │             │         │             │
├─────────────┤ U into S├─────────────┤ S into M├─────────────┤
│             │     ┌──►│             │     ┌──►│             │
│             │     │   │             │     │   │             │
│  U Code     │     │   │  S Code     │     │   │             │
│             │     │   │             │     │   │             │
├─────────────┤     │   ├─────────────┤     │   │             │
│    ecall    ├─────┘   │   ecall     ├─────┘   │    M Code   │
├─────────────┤         ├─────────────┤         │             │
│             │◄──┐     │             │◄──┐     │             │
│             │   │     │             │   │     │             │
│  U Code     │   │     │  S Code     │   │     │             │
│             │   └─────┤             │   └─────┤             │
├─────────────┤ S back U├─────────────┤ M back S│             │
└─────────────┘         └─────────────┘         └─────────────┘
```

### RISC-V Privilege Instructions

Instructions no relation with privilege and general registers `x0`~`x31` can be used under any Privilege Level. Each Privilege Level has its own special instructions and `CSR`(Control and Status Register), to control behaviors under this privilege and record its status.

If low level execute high level privilege instruction, will generate `Illegal instruction` fault. This fault is usually unrecoverable, the Execution Environment will kill this low level mode software.

RISC-V S-mode instructions

Instruction | Description
-- | --
sret | Return from S-mode to U-mode, will generated Illegal Instruction Exception when executed under U-mode
wfi | Make the cpu into low-power-comsumption when idle, will generated Illegal Instruction Exception when executed under U-mode
sfence.vma | Refresh TLB cache, will generated Illegal Instruction Exception when executed under U-mode
instructions that access S-mode CSR | Change system status by accessing S-mode CSR, will generated Illegal Instruction Exception when executed under U-mode

S-mode CSR table:

CSR Name | Function related to Trap
-- | --
sstatus | Fields like `SPP` give the information about the level before `Trap`
sepc | When Trap is an exception, record the address of last instruction before Trap
scause | Describe the reason why Trap
stval | Additional informations about Trap
stvec | Control the entry address of Trap handler code

## Realize Batch OS

In Batch OS, each time a application is finished, the next app's codes and data are load into memory. So we must realize app load mechanism first. That is: Under the premise that the os and app codes need to be put in one executable file, we need to design a app load mechanism as brief as possible.

- App use static binding method, bind with Batch OS.
- Based on the bind information, the OS can find every app's start address, and can load into memory.

### RISC-V switch between privilege levels

The S-mode Batch OS need to do some initialization for establishing `AEE`(Application Execute Environment), and monitor the execution of app:

- When app start, OS need to initialize app U-mode context, and switch to U-mode for execute app.
- When app Trap, need to produce in OS.
- When app error, need to go to OS, kill it, load and run the next app.
- When app completed, need to go to OS, load and run the next app.

These procedures need corporation of app, OS and hardware, to realize switching between privilege level.

Because we need to continue from the next instruction after `ecall` in U-mode when back from S-mode, we need to ensure the app context(general registers and stack) is unchanged before and after `ecall`. Because different privilege mode use the same set of general registers, the S-mode OS may change the app's registers.

So we need to store these registers like `Function Call Context` store. Besides the general registers, the CSR is also possibly changed, for example, the privilege that the CPU is situated. For trap, before it is U-mode, in processing it is S-mode, after it is return to U-mode. As for the stack context, as long as the two app's stack area are different, we don't need to worry about.

#### Hardware Operation

Before CPU trap from U to S due to instruction like `ecall`, the hardware will automatically do these things:

- `sstatus`'s SPP field modified to CPU current privilege level
- `sepc` modified to the next instruction after trap
- `scause/stval` modified to trap cause and additional information
- CPU jump to the entry address set by `stvec`, and set SPP as S, then process on Trap

> `stvec` is a 64-bit CSR, when disruptions are enabled, saves the entry address of disruption.
> 
> it has two fields:
> - MODE [1:0], length 2 bits
> - BASE [63:2], length 62 bits
> When MODE is set to 0, `stvec` is `Direct` mode, at this mode, whatever the cause of trap is, the entry address is always `BASE<<2`

When CPU finish Trap, ready to return, need S-mode instruction `sret`, it do these things:
- CPU set privilege using the `sstatus`'s SPP field.
- CPU jumps to `sepc` stored address, and continue execution.

### User Stack and Kernel Stack

When trap triggered, CPU is switched to S-mode, and jump to `svec` indicated address, but before enter Trap, the kernel stack need to store app's general registers status.

We first use `__all_traps` to store Trap Context in the kernel stack, then jump to Rust wrote `trap_handler` to finish trap distribution and handle. When `trap_handler` returns, use `__restore` to restore Trap Context from kernel stack. Finally use `sret` back to app.
