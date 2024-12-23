.code32

.global _start

_start:
    cli
    // multiboot2 doesn't guarantee us a stack, so we have to make one
    // assumes stack size is 16384
    lea esp, [INIT_STACK + {INIT_STACK_SIZE}]
    mov ebp, esp
    // preserve eax and ebx for their multiboot info
    push eax
    push ebx
    call init_gdt
    call init_paging

    mov ax, {KERNEL_DATA_SELECTOR}
    mov ds, ax
    mov es, ax 
    mov fs, ax
    mov gs, ax
    pop ebx
    pop eax
    ljmp {KERNEL_CODE_SELECTOR}, offset long_mode
// sets up a basic page table that identity maps the first 2mb of memory
// intended to be replaced with a proper page table by the kernel
// assumes kernel KERNEL_END < 2mb

init_paging:
    // assumes INIT_PT is already initialized in Rust
    lea eax, [INIT_PT]
    or eax, {PAGE_PRESENT} | {PAGE_RW}
    mov [INIT_PDT], eax

    lea eax, [INIT_PDT]
    or eax, {PAGE_PRESENT} | {PAGE_RW}
    mov [INIT_PDPT], eax
    
    lea eax, [INIT_PDPT]
    or eax, {PAGE_PRESENT} | {PAGE_RW}
    mov [INIT_PML4T], eax
    
    lea eax, INIT_PML4T
    or eax, {PAGE_PRESENT} | {PAGE_RW}
    mov [INIT_PML5T], eax

    // disable paging in case it's enabled for some reason
    mov ebx, cr0
    and ebx, ~{CR0_PG}
    mov cr0, ebx

    mov eax, cr4
    or eax, {CR4_PAE}
    mov cr4, eax

    mov ecx, {EFER}
    rdmsr
    or eax, {EFER_LME}
    wrmsr

    lea eax, [INIT_PML4T]
    mov cr3, eax

    mov ebx, cr0
    or ebx, {CR0_PG}
    mov cr0, ebx

    ret

// sets GDTR.base to addr_of GDT and loads GDT
init_gdt: 
    // assumes GDT is loaded in the lower 4GB of memory
    lea eax, [GDT]
    mov [GDTR+{GDTR_OFFSET}], eax
    lgdt [GDTR]
    ret

.code64
long_mode:
    // parameters for kernel_main
    mov rdi, rax
    mov rsi, rbx
    
    // clear other gprs
    xor rax, rax
    xor rbx, rbx
    xor rcx, rcx
    xor rdx, rdx
    xor r9,  r9
    xor r10, r10
    xor r11, r11
    xor r12, r12
    xor r13, r13
    xor r14, r14
    xor r15, r15

    call kernel_main
kernel_exit:
    cli
    hlt
    jmp kernel_exit