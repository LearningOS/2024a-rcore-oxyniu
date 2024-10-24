# chapter3练习: 获取任务信息

## 任务实现

我在 task/mod.rs 中 TaskManager 的可变部分 TaskManagerInner 添加了新成员：

* tasks_syscall_times：是一个长度为 MAX_APP_NUM 的列表，保存对应任务 id 进行每种系统调用的次数，使用 alloc::collections::BTreeMap 的字典实现，相比桶计数减少了内存占用。

* tasks_first_run_time：是一个长度为 MAX_APP_NUM 的列表，记录了每个任务初次运行的时间（以毫秒计）

对应这些新成员，我在 TaskManager 中添加了新方法，对新成员进行查询和修改

最后，我在系统调用 syscall/mod.rs 中对 tasks_syscall_times 进行更新；在 sys_task_info 中利用实现的方法查询所需信息，并修改 _ti；在 TaskManager 移动到新任务时记录当时时间。

## 问题解决

在 run_next_task 中记录初次运行时间时，最初我直接更新 tasks_first_run_time 的值。但是由于抢占式分时多任务系统中每个用户程序不断轮换，run_next_task 很可能被执行多次，导致 tasks_first_run_time 存放的不再是初次运行时间。我在调试中发现了此问题并进行解决。

## 简答题

### 问题 1

* ch2b_bad_address：程序试图对地址 0x0 进行写入操作
* ch2b_bad_instructions：程序试图执行 S 态特权指令 sret
* ch2b_bad_register：程序试图访问 S 态寄存器 sstatus

当程序执行这些非法操作时，CPU 陷入（Trap）到 S 特权级，并跳转到 stvec 指向的 Trap 处理入口地址。内核首先保存现场（trap.S），随后跳转到 Rust 函数 trap_handler，函数识别到 Trap 类型为非法操作，打印错误信息并直接执行下一程序。

```
RustSBI version 0.3.0-alpha.2

[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003a4, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
```

### 问题 2

1. 刚进入 __restore 时，a0 存放上次函数调用的第一个参数。__restore 用于 Trap 处理（如系统调用）结束后，从栈中上下文恢复现场，由 S 特权级返回到 U 特权级，并继续执行用户程序；或是在执行第一个程序前，内核构造特殊的上下文并调用 __restore，开始执行第一个程序
2. 这段汇编代码首先恢复 sstatus，sepc 和 sscratch 三个 S 特权级寄存器（由于恢复过程需要读写寄存器，所以需要最先恢复）。sstatus 包含一系列状态位，例如其 SPP 字段表示 Trap 发生前 CPU 处于的特权级，sepc 记录 Trap 发生之前执行的最后一条指令的地址，在用户态，sscratch 保存内核栈的地址；在内核态，sscratch 的值为 0。
3. x4（tp）一般不会被用到，x2（sp）此前与 sscratch 交换过，指向内核栈的地址，需要之后读出 sscratch 再进行恢复
4. 这条指令交换 sscratch 和 sp,交换后，sscratch 指向内核栈，sp 指向用户栈。
5. sret 发生状态切换，将特权级设置为 SPP 中保存的 U,并跳转到 sepc 保存的地址。
6. sscratch 指向用户栈，sp 指向内核栈。
7. 系统调用使用 ecall,非法指令也会引发 Trap 从 U 特权级切换到 S 特权级。

## 说明

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

* 未进行过交流

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

* 部分简答题参考了 [The RISC-V Instruction Set Manual: Volume II](https://github.com/riscv/riscv-isa-manual/releases/download/20240411/priv-isa-asciidoc.pdf)

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。