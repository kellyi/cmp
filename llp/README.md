# LLP

## Compile NASM & create executable

```sh
nasm -felf64 hello.asm -o hello.o
ld -o hello.out hello.o
chmod u+x hello.out
```

## Compile C & create executable

```sh
gcc -o hello.out -ansi -pedantic-errors -Wall -Werror hello.c
```

## Disassemble

```sh
ndisasm -b 64 hello.out > _hello.asm
```

## Resources

- [NASM documentation](https://www.nasm.us/doc/)
- [Intel manual](https://www.intel.com/content/www/us/en/architecture-and-technology/64-ia-32-architectures-software-developer-vol-3a-part-1-manual.html)
- [sandpile.org](http://sandpile.org/)
- [20 part linker essay](https://lwn.net/Articles/276782/)
- [Programming from the Ground Up](https://download-mirror.savannah.gnu.org/releases/pgubook/ProgrammingGroundUp-1-0-booksize.pdf)
- [asmtutor](http://asmtutor.com/)
