mov rax, 5              # store 5 in rax
mov r12, 10             # store 10 in r12
add rax, r12            # rax <- rax + r12 = 15
cmp rax, r12            # set flags in rflags
jge RAX_IS_LARGER       # read flags in rflags, jump to RAX_IS_LARGER

R12_IS_LARGER:
# some instructions
jmp END

RAX_IS_LARGER:
# some other instructions

END:
# mor