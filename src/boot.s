.code32

.global _start

_start:
    // multiboot2 doesn't guarantee us a stack, so we have to make one
    // assumes stack size is 16384
    lea esp, [INIT_STACK + {INIT_STACK_SIZE}]
    // preserve gprs in case multiboot left anything important in them
    push eax
    push ebx
    push ecx
    call init_gdt
    call init_paging
    mov ax, {KERNEL_DATA_SELECTOR}
    mov ds, ax
    mov es, ax 
    mov fs, ax
    mov gs, ax
    pop ecx
    pop ebx
    pop eax
    ljmp {KERNEL_CODE_SELECTOR}, offset kernel_main
// sets up a basic page table that identity maps the first 2mb of memory
// intended to be replaced with a proper page table by the kernel
// assumes kernel KERNEL_END < 2mb

init_paging:
    // assumes INIT_PT is already initialized in Rust
    mov eax, INIT_PT
    or eax, {PAGE_PRESENT} | {PAGE_RW}
    or [INIT_PDT], eax
    mov eax, INIT_PDT
    or eax, {PAGE_PRESENT} | {PAGE_RW}
    or [INIT_PDPT], eax
    
    mov eax, INIT_PDPT
    or eax, {PAGE_PRESENT} | {PAGE_RW}
    or [INIT_PML4T], eax
    mov eax, INIT_PML4T
    or eax, {PAGE_PRESENT} | {PAGE_RW}
    or [INIT_PML5T], eax
    // disable paging in case it's enabled for some reason
    mov ebx, cr0
    and ebx, ~{CR0_PG}
    mov cr0, ebx
    mov eax, cr4
    or eax, {CR4_PAE}
    mov cr4, eax
    mov ecx, {EFER}
    rdmsr
    or ecx, {EFER_LME}
    wrmsr
    mov eax, INIT_PML4T
    mov cr3, eax
    or ebx, {CR0_PG}
    mov cr0, ebx
    ret

// sets GDTR.base to addr_of GDT and loads GDT
init_gdt: 
    // assumes GDT is loaded in the lower 4GB of memory
    mov eax, GDT
    mov [GDTR+{GDTR_OFFSET}], eax
    lgdt [GDTR]
    ret