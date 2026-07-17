# 🖥️ Rust 64-Bit Custom Virtual Machine & Monolithic Kernel

[![Project Status: Completed](https://img.shields.io/badge/Project%20Status-Completed%20%2F%20Archived-green.svg)](#)
[![Language: Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)

> **Architectural Milestone Achieved.**  
> This project represents a complete, ground-up implementation of a custom virtual platform and operating system. Having successfully reached a self-booting milestone with working graphics and interactive subsystems, this codebase is now archived as a stable portfolio reference. Future systems development has transitioned to targeting real-world hardware architectures (RISC-V/ARM via QEMU).

---

## 📖 Project Overview

This repository houses a custom-built, software-defined computer system designed entirely from first principles. Rather than relying on existing hardware architectures (such as x86_64 or ARM), this project defines its own **Instruction Set Architecture (ISA)**, emulates a physical hardware system in Rust, and implements a **bare-metal monolithic operating system kernel** that boots and runs directly on that virtual silicon.

The core objective of this project was to demystify the hardware-software boundary by owning the entire execution stack, from raw byte parsing and memory-mapped register manipulation up to a graphical display canvas.

---

## 📐 System Architecture

```
+-------------------------------------------------------------+
|                 User Programs / Custom ASM                  |
+-------------------------------------------------------------+
                               | 
                               v (Custom Bytecode)
+-------------------------------------------------------------+
|                      Monolithic Kernel                      |
|    - Interrupt Traps   - VRAM Draw Engine   - Syscall API   |
+-------------------------------------------------------------+
                               |
                               v [Privileged Register Access]
+-------------------------------------------------------------+
|                 Virtual Machine (Emulator)                  |
|  [64-bit CPU]       [Memory (RAM Bus)]      [Disk Driver]   |
|  - 16 GP Regs       - MMIO Window           - Sector-Based  |
+-------------------------------------------------------------+
                               |
                               v [Raw Framebuffer Dump]
+-------------------------------------------------------------+
|           Video Output Buffer (Text & Pixel Canvas)         |
+-------------------------------------------------------------+
```

---

## 🛠️ Technical Deep Dive & Features

### 1. The 64-Bit Virtual CPU & ISA
The virtual CPU is a custom software-defined processor executing a proprietary 64-bit instruction set:
* **Registers:** Features 8 general-purpose 64-bit registers, a Program Counter (PC), and a Stack Pointer (SP).
* **RAM Bus:** A simulated, byte-addressable memory bus with a built-in Memory Management Unit (MMU) paradigm to enforce address boundaries.
* **Instruction Decoder:** A fast, single-cycle instruction decoder that parses custom bytecode instructions (arithmetic, logic, register jumps, memory loads/stores, and system interrupts).

### 2. Bare-Metal Bootloader & Monolithic Kernel
The operating system bootstraps itself directly from virtual storage:
* **Disk Sector Parsing:** The bootloader acts as a raw-stage loader. It directly parses sector offsets on the virtual disk drive, reads the kernel image, maps it into the designated physical RAM space, and executes a far-jump to pass control.
* **Monolithic Core:** The kernel runs in a privileged processor state, managing memory allocations, system clocks, and coordinating input/output tasks.

### 3. Memory-Mapped I/O (MMIO) & Subsystems
Peripherals are exposed to the software stack via dedicated memory address windows (MMIO):
* **Graphics (VRAM):** Supports dual video modes—a fast **Text Terminal** mode for low-overhead console output, and a **Pixel Graphics** mode supporting custom coordinate plotting.
* **Interrupt Controller:** A hardware interrupt handler that traps keyboard inputs, timer ticks, and hardware faults, gracefully passing control to designated ISRs (Interrupt Service Routines).
* **System Calls (Syscalls):** A software interrupt interface (`SYS`) allowing user-space binaries to invoke kernel drawing, input, and exit routines safely.

---

## 🗺️ Virtual Memory Map

The system memory is strictly divided to isolate boot logic, executive kernel execution, and hardware-mapped peripherals:

| Address Range | Size | Description |
|---|---|---|
| `0x00000 - 0x001FF` | 512 B | **Boot Sector** (Loads the core bootloader) |
| `0x00200 - ` | Variable | **Monolithic Kernel space** |
| `0x10000 - 0x7FFFF` | 448 KB | **System Heap & User Stack** |
| `0x80000 - 0x9FFFF` | 128 KB | **VRAM Framebuffer** (Pixel/Text Output) |
| `0xA0000 - 0xA0FFF` | 4 KB | **MMIO Control Registers** (Keyboard state, Timers, Hardware interrupts) |

---

## 📜 Example Custom Instruction Set Architecture (ISA)

Programs are written in a proprietary assembly language and compiled down to bytecode. Below is a sample of how the ISA handles memory, math, and terminal output:

```assembly
; Example: Print a single character to the terminal via MMIO
LOAD r1, 0x41        ; Load ASCII 'A' into register 1
LOAD r2, 0x80000     ; Load VRAM starting memory address into register 2
STORE [r2], r1       ; Store 'A' directly in VRAM (instantly draws on screen)
HALT                 ; Terminate execution
```

---

## 🔧 Building & Running

Ensure you have a modern Rust toolchain installed.

### 1. Compile the Virtual Machine
```bash
cargo build --release
```

### 2. Run the System
Specify the disk image containing the compiled bootloader and kernel:
```bash
cargo run --release -- --disk path/to/disk.img
```

---

## 🏆 Portfolio Significance & Lessons Learned

Developing this project validated key low-level engineering skillsets:
* **System Prototyping:** Gained a deep understanding of standard processor execution cycles (Fetch, Decode, Execute).
* **Hardware Interfacing:** Designed and implemented MMIO interfaces, proving mastery over how software registers map directly to virtual physical screens and input components.
* **Toolchain Design:** Coordinated with a native assembler environment to output validated bytecode, establishing a highly aligned hardware-software interface.
* **Modern Systems Programming:** Leveraged Rust's safety, type-system guarantees, and zero-cost abstractions to manage memory-unsafe hardware emulations without undefined behavior.
