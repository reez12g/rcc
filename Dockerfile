FROM ubuntu:latest

RUN apt update
RUN DEBIAN_FRONTEND=noninteractive apt install -y gcc make git binutils libc6-dev gdb sudo curl
RUN adduser --disabled-password --gecos '' user
RUN echo 'user ALL=(root) NOPASSWD:ALL' > /etc/sudoers.d/user

RUN curl https://sh.rustup.rs -sSf | sh
ENV PATH="/root/.cargo/bin:$PATH"

USER user

WORKDIR /home/user
