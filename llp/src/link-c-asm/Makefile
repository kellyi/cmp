all: main

main: main.o lib.so
	gcc -o main.out main.o lib.so

lib.so: lib.o
	gcc -shared lib.o -o lib.so

lib.o: lib.asm
	nasm -felf64 lib.asm -o lib.o

main.o:
	gcc -ansi -c main.c -o main.o

clean:
	rm -rf *.o *.so main.out

