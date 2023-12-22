clang -c -Wall -o libuipc.o .\IPCUser.cc
llvm-ar rcs libuipc.lib .\libuipc.o
bindgen .\IPCUser.hpp -o _libuipc.rs