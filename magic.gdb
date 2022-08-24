#
# https://black-magic.org/usage/gdb-commands.html
#
target extended-remote /dev/ttyACM0

monitor jtag_scan
monitor swdp_scan

# print demangled symbols
set print asm-demangle on

# set backtrace limit to not have infinite backtrace loops
set backtrace limit 32

attach 1
load

# detect unhandled exceptions, hard faults and panics
break DefaultHandler
break HardFault
break rust_begin_unwind

# *try* to stop at the user entry point (it might be gone due to inlining)
#break main

# start the process but immediately halt the processor
stepi
