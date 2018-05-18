FROM debian:jessie

RUN apt-get update -y && apt-get install build-essential gdb nasm vim less ruby-full -y

RUN groupadd -g 999 llp && useradd -r -u 999 -g llp llp

USER llp
