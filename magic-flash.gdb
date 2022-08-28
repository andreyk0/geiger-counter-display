#
# https://black-magic.org/usage/gdb-automation.html
#
target extended-remote /dev/ttyACM0

monitor jtag_scan
monitor swdp_scan
attach 1
load
compare-sections
kill
