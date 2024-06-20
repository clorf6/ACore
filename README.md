# ACore

A simple RISC-V microkernel, which can be seen as a imcomplete transformation of [rCore](https://github.com/rcore-os/rCore-Tutorial-v3).

### Plan

- [x] Allocator
	- [x] Buddy Allocator: Using binary tree structure.
	- [x] Frame Allocator
- [x] Page Table: SV39
- [x] Console

  - [x] Read
  - [x] Write
- [x] Message & data transfer: Shared buffer

  - [x] User to Kernel: syscall or shared buffer

  - [x] Kernel to User: shared buffer

  - [x] Kernel to Kernel

  - [x] User to User: service queue in scheduler
- [x] Process
  - [x] Process loading
  - [x] Syscall
    - [x] Kick off a new process: fork, exec
    - [x] Wait for child processes: waitpid
    - [x] Exit from a process: exit

  - [x] Process manager
  - [x] Scheduler: RR scheduler
    - [x] Context switch
    - [x] Timer interrupt

##### Futuer plan if time permitted

- [ ] Scheduler with stride and priority
- [ ] File System
- [ ] SLAB allocator
