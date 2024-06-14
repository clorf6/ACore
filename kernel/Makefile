# Building
TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := target/$(TARGET)/$(MODE)/kernel
KERNEL_BIN := $(KERNEL_ELF).bin

# Building mode argument
ifeq ($(MODE), release)
	MODE_ARG := --release
else
	MODE_ARG := --debug
endif

# BOARD
BOARD := qemu
BOOTLOADER := ../bootloader/$(BOARD).bin

# KERNEL ENTRY
KERNEL_ENTRY := 0x80000000

# Binutils
OBJCOPY := rust-objcopy --binary-architecture=riscv64

build: $(KERNEL_BIN)

$(KERNEL_BIN): kernel
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@

kernel:
	@echo Platform: $(BOARD)
	@cargo build $(MODE_ARG)

clean:
	@cargo clean

RUN_ARGS := -machine virt \
			 -nographic \
			 -bios $(BOOTLOADER) \
			 -device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY)

DEBUG_ARGS := -machine virt \
			 -nographic \
			 -bios $(BOOTLOADER) \
			 -device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY) \
			 -s -S 

run: build
	@qemu-system-riscv64 $(RUN_ARGS)

debug: build
	@qemu-system-riscv64 $(DEBUG_ARGS)

gdbclient:
	@gdb-multiarch -ex 'file $(KERNEL_ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'
