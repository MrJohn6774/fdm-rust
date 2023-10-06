clang -c -Wall -o libuipc.o .\IPCUser.cc
llvm-ar rcs libfsuipc.lib .\libfsuipc.o
bindgen .\IPCUser.h -o _libuipc.rs