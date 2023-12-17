clang -c -Wall -o libuipc.o .\IPCUser.cc
llvm-ar rcs libuipc.lib .\libuipc.o
bindgen .\IPCUser.h -o _libuipc.rs