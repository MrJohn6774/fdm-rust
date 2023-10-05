clang -c -Wall -o libfsuipc.o .\IPCuser64.c
llvm-ar rcs libfsuipc.lib .\libfsuipc.o
bindgen .\FSUIPC_User64.h -o libfsuipc.rs