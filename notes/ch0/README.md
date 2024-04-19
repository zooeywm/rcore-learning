## Divzero

Write a program that can produce an exception, and debug it, explain the process result of the OS.

```zsh
❯ gcc --version
gcc (GCC) 13.2.1 20230801
Copyright (C) 2023 Free Software Foundation, Inc.
This is free software; see the source for copying conditions.  There is NO
warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
$ gcc -g -o bin/divzero divzero.c
$ ./divzero
[1]    39030 floating point exception (core dumped)  ./divzero
$ strace ./divzero 
[...]
--- SIGFPE {si_signo=SIGFPE, si_code=FPE_INTDIV, si_addr=0x5d1149b38148} ---
+++ killed by SIGFPE (core dumped) +++
[1]    39929 floating point exception (core dumped)  strace ./divzero
```

[![dbg divzero](https://asciinema.org/a/654234.svg)](https://asciinema.org/a/654234)

From the gdb, we can infer that the exception is due to divide zero when the `idiv` instruction executed. Then jump to OS to handle the exception, the OS transfer it to a signal `SIGFPE`, use signal mechanism to handle the exception. The behavior of receiving `SIGFPE` of this program is abnormal termination, so OS terminate it and report the information to shell.

Note that the exception handle and signal is related to CPU architecture, for example, the divide zero behavior is not an exception on RISC-V, but a certain result.

## Program-a

Write a program that sleep 5s and print a string. Meanwhile, write it to a file.

## Program-b

Write a program that can reflect concurrency, asynchronism, shareability, and persistency.

## Debug Program-a

Use GDB to debug Program-a, can set breakpoint, single step execution, and view variable info.

[![asciicast](https://asciinema.org/a/654267.svg)](https://asciinema.org/a/654267)
