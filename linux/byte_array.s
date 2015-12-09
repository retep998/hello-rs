[BITS 64]
mov rsi, rdi
mov edi, 1
mov edx, 14
mov eax, 1
syscall
mov eax, 60
xor edi, edi
syscall
