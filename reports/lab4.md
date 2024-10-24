# chapter6练习: 硬链接

## 任务实现

重写所有旧的系统调用。

在 easy-fs 中加入方法，查找 inode 属性对应的 inode id，在 TCB 中保存每个文件描述符对应的 inode id。
添加硬链接添加和删除的方法。硬链接添加参照创建文件；删除则是对 disk_inode 直接进行操作，删除对应的 DirEntry（与尾部交换位置，并缩小 size ）。
查询当前链接数通过简单遍历 disk_inode 实现。

## 简答题

1. root inode 记录根目录下的文件，并有一些读写文件的方法。如果 root inode 损坏，很可能无法访问并丢失根目录下的文件。
2. sudo ps aux | grep qemu 查询包含 qemu 字符串的进程 pid。as 汇编器等可以通过管道输入源程序。
3. 建立一个节点连接所有需要建立管道连接的进程，控制管道连接哪一对进程。

## 说明

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

* 未进行过交流

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

[Rust 参考手册](https://rustwiki.org/)

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。