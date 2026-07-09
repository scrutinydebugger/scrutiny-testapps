FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y \
    build-essential     \
    wget                \
    xz-utils            \
    libgmp-dev          \
    libmpfr-dev         \
    libmpc-dev          \
    texinfo             \
    ninja-build         \
    python3             \
    python3-venv        \
    python3-pip         \
    cmake               \
    binutils            \
    git                 \
    gcc-arm-none-eabi   \
    file                \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /tmp

#RUN git clone https://github.com/ttsiodras/checkStackUsage

ENV CHECK_STACK_USAGE_PY=/tmp/checkStackUsage/checkStackUsage.py