# Connect to the debugger
target remote :3333

# Prevent panic corruption of LR
set backtrace limit 32

# Stop the device (just in case the debugger didnt)
monitor reset halt

# Program our code to flash
load

# Reset the device
monitor reset halt

# Set a breakpoint at main()
break main

# Run until we hit main()
continue
