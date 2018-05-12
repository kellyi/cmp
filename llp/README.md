# LLP

## Compile NASM & create executable

```sh
nasm -felf64 hello.asm -o hello.o
ld -o hello.out hello.o
chmod u+x hello.out
```

## Disassemble

```sh
ndisasm -b 64 hello.out > _hello.asm
```

## NASM documentation

[NASM](https://www.nasm.us/doc/)

## Intel manual

[Intel manual](https://www.intel.com/content/www/us/en/architecture-and-technology/64-ia-32-architectures-software-developer-vol-3a-part-1-manual.html)
