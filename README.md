# DCPU-text-screen
a text screen for the DCPU emulator

# Usage
run ```cargo run``` in this projects directory
go to the directory of the DCPU emulator and run a program that works with the screen check the screens terminal.

to print a character we use the following asm code:
```
add 0 0 p1
add x 0 p2
add 2 0 p0
add 0 0p0
```
in this example x is the UTF-8 character code that we want to print
code 27 clears the terminal

just change the file pahths to your emulator files and it should work
