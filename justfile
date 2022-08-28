name := "main"
build := "debug" # debug | release

elf_target := "target" / "thumbv7m-none-eabi" / build / name
bin_target := "target" / name + ".bin"

cargo_build_flags := if build == "release" { "--release" } else { "" }

build: fmt
	cargo build {{cargo_build_flags}}

# Requires openocd running
debug-openocd: build
	arm-none-eabi-gdb -x openocd.gdb -q {{elf_target}}

# Debug with Black Magic probe
debug-magic: build
	arm-none-eabi-gdb -x magic.gdb -q {{elf_target}}

bin: build
	arm-none-eabi-objcopy -O binary {{elf_target}} {{bin_target}}

disassemble: build
	arm-none-eabi-objdump --disassemble {{elf_target}} | less -S

doc:
	cargo doc --open

fmt:
	find src -type f -name '*.rs' | xargs rustfmt

flash-stlink: bin erase
	st-info --descr
	st-flash write {{bin_target}} 0x8000000

flash-magic: bin
	arm-none-eabi-gdb -nx --batch -x magic-flash.gdb {{elf_target}}

erase:
	st-flash erase

clean:
	cargo clean

picocom:
	picocom -b 115200 --imap lfcrlf /dev/ttyACM0
