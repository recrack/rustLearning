	.section	__TEXT,__text,regular,pure_instructions
	.globl	__ZN9intrinsic6tydesc6tydesc6tydesc17_c5d326ea82a38e173_00E
	.align	4, 0x90
__ZN9intrinsic6tydesc6tydesc6tydesc17_c5d326ea82a38e173_00E:
	.cfi_startproc
	cmpq	%gs:816, %rsp
	ja	LBB0_0
	movabsq	$24, %r10
	movabsq	$0, %r11
	callq	___morestack
	ret
LBB0_0:
	pushq	%rbp
Ltmp2:
	.cfi_def_cfa_offset 16
Ltmp3:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
Ltmp4:
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	movq	%rdx, -8(%rbp)
	movq	%rdi, -16(%rbp)
	jmp	LBB0_1
LBB0_1:
	jmp	LBB0_3
LBB0_2:
	addq	$16, %rsp
	popq	%rbp
	ret
LBB0_3:
	movq	-16(%rbp), %rax
	movq	-8(%rbp), %rcx
	movq	(%rcx), %rdx
	movq	8(%rcx), %rsi
	movq	16(%rcx), %rcx
	movq	%rcx, 16(%rax)
	movq	%rsi, 8(%rax)
	movq	%rdx, (%rax)
	jmp	LBB0_2
	.cfi_endproc

	.globl	__ZN9intrinsic5rusti12visit_tydesc17_398aa9f5e737e8543_00E
	.align	4, 0x90
__ZN9intrinsic5rusti12visit_tydesc17_398aa9f5e737e8543_00E:
	.cfi_startproc
	cmpq	%gs:816, %rsp
	ja	LBB1_0
	movabsq	$24, %r10
	movabsq	$0, %r11
	callq	___morestack
	ret
LBB1_0:
	pushq	%rbp
Ltmp7:
	.cfi_def_cfa_offset 16
Ltmp8:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
Ltmp9:
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	movq	%rcx, -8(%rbp)
	movq	%rdx, -16(%rbp)
	jmp	LBB1_1
LBB1_1:
	jmp	LBB1_3
LBB1_2:
	addq	$16, %rsp
	popq	%rbp
	ret
LBB1_3:
	movabsq	$0, %rax
	movq	-8(%rbp), %rcx
	movq	%rax, %rdi
	movq	%rax, %rsi
	movq	%rax, %rdx
	movq	-16(%rbp), %rax
	callq	*48(%rax)
	jmp	LBB1_2
	.cfi_endproc

	.globl	__ZN4main17_25cf8af9d5dceaa63_00E
	.align	4, 0x90
__ZN4main17_25cf8af9d5dceaa63_00E:
	.cfi_startproc
	.cfi_personality 155, _upcall_rust_personality
Leh_func_begin2:
	.cfi_lsda 16, Lexception2
	cmpq	%gs:816, %rsp
	ja	LBB2_0
	movabsq	$56, %r10
	movabsq	$0, %r11
	callq	___morestack
	ret
LBB2_0:
	pushq	%rbp
Ltmp15:
	.cfi_def_cfa_offset 16
Ltmp16:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
Ltmp17:
	.cfi_def_cfa_register %rbp
	subq	$48, %rsp
	jmp	LBB2_3
LBB2_2:
	addq	$48, %rsp
	popq	%rbp
	ret
LBB2_3:
	## InlineAsm Start
	# io::println("amit"); (hello.rs:7:1: 7:21)
	## InlineAsm End
	movl	$4, %esi
	leaq	_str1(%rip), %rdi
	callq	_upcall_str_new_uniq
	movq	%rax, %rsi
Ltmp10:
	movq	%rsi, -24(%rbp)
	movq	%rcx, %rsi
	movq	%rax, %rdx
	callq	__ZN2io7println17_7bd51147c9de282c3_03E
Ltmp11:
	jmp	LBB2_5
LBB2_5:
	jmp	LBB2_9
LBB2_6:
Ltmp12:
	movl	%edx, %ecx
	movq	%rax, -32(%rbp)
	movl	%ecx, -36(%rbp)
	callq	_upcall_reset_stack_limit
	movl	-36(%rbp), %ecx
	movl	%ecx, -8(%rbp)
	movq	-32(%rbp), %rax
	movq	%rax, -16(%rbp)
	xorl	%eax, %eax
	movq	%rax, %rdi
	movq	%rax, %rsi
	movq	%rax, %rdx
	movq	-24(%rbp), %rcx
	callq	_glue_free3
	movl	-8(%rbp), %r8d
	movq	-16(%rbp), %rdi
	movl	%r8d, -40(%rbp)
	callq	__Unwind_Resume
LBB2_8:
	jmp	LBB2_2
LBB2_9:
	movabsq	$0, %rax
	movq	-24(%rbp), %rcx
	movq	%rax, %rdi
	movq	%rax, %rsi
	movq	%rax, %rdx
	callq	_glue_free3
	jmp	LBB2_8
	.cfi_endproc
Leh_func_end2:
	.section	__TEXT,__gcc_except_tab
	.align	2
GCC_except_table2:
Lexception2:
	.byte	255
	.byte	155
	.byte	41
	.byte	3
	.byte	39
Lset0 = Leh_func_begin2-Leh_func_begin2
	.long	Lset0
Lset1 = Ltmp10-Leh_func_begin2
	.long	Lset1
	.long	0
	.byte	0
Lset2 = Ltmp10-Leh_func_begin2
	.long	Lset2
Lset3 = Ltmp11-Ltmp10
	.long	Lset3
Lset4 = Ltmp12-Leh_func_begin2
	.long	Lset4
	.byte	0
Lset5 = Ltmp11-Leh_func_begin2
	.long	Lset5
Lset6 = Leh_func_end2-Ltmp11
	.long	Lset6
	.long	0
	.byte	0
	.align	2

	.section	__TEXT,__text,regular,pure_instructions
	.globl	__rust_main
	.align	4, 0x90
__rust_main:
	.cfi_startproc
	cmpq	%gs:816, %rsp
	ja	LBB3_0
	movabsq	$24, %r10
	movabsq	$0, %r11
	callq	___morestack
	ret
LBB3_0:
	pushq	%rbp
Ltmp20:
	.cfi_def_cfa_offset 16
Ltmp21:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
Ltmp22:
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	movq	%rsi, -8(%rbp)
	movq	%rdi, -16(%rbp)
	jmp	LBB3_1
LBB3_1:
	jmp	LBB3_3
LBB3_2:
	addq	$16, %rsp
	popq	%rbp
	ret
LBB3_3:
	movq	-16(%rbp), %rdi
	movq	-8(%rbp), %rsi
	callq	__ZN4main17_25cf8af9d5dceaa63_00E
	jmp	LBB3_2
	.cfi_endproc

	.globl	_main
	.align	4, 0x90
_main:
	.cfi_startproc
	cmpq	%gs:816, %rsp
	ja	LBB4_0
	movabsq	$24, %r10
	movabsq	$0, %r11
	callq	___morestack
	ret
LBB4_0:
	pushq	%rbp
Ltmp25:
	.cfi_def_cfa_offset 16
Ltmp26:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
Ltmp27:
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	leaq	__rust_main(%rip), %rax
	leaq	__rust_crate_map_toplevel(%rip), %rcx
	movq	%rdi, -8(%rbp)
	movq	%rax, %rdi
	movq	-8(%rbp), %rax
	movq	%rsi, -16(%rbp)
	movq	%rax, %rsi
	movq	-16(%rbp), %rdx
	callq	_rust_start
	addq	$16, %rsp
	popq	%rbp
	ret
	.cfi_endproc

	.align	4, 0x90
_glue_free3:
	.cfi_startproc
	cmpq	%gs:816, %rsp
	ja	LBB5_0
	movabsq	$24, %r10
	movabsq	$0, %r11
	callq	___morestack
	ret
LBB5_0:
	pushq	%rbp
Ltmp30:
	.cfi_def_cfa_offset 16
Ltmp31:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
Ltmp32:
	.cfi_def_cfa_register %rbp
	subq	$16, %rsp
	movq	%rcx, -8(%rbp)
	jmp	LBB5_1
LBB5_1:
	jmp	LBB5_3
LBB5_2:
	addq	$16, %rsp
	popq	%rbp
	ret
LBB5_3:
	movq	-8(%rbp), %rax
	cmpq	$0, %rax
	movq	%rax, -16(%rbp)
	jne	LBB5_5
LBB5_4:
	jmp	LBB5_2
LBB5_5:
	movq	-16(%rbp), %rax
	movq	%rax, %rdi
	callq	_upcall_exchange_free
	jmp	LBB5_4
	.cfi_endproc

	.section	__DATA,__data
	.globl	__rust_crate_map_toplevel
	.align	4
__rust_crate_map_toplevel:
	.quad	__rust_mod_map
	.quad	__rust_crate_map_core_0.3_d27e4777a53c3e50
	.quad	0

	.section	__DATA,__const
	.align	3
_shapes:
	.quad	_tag_shapes
	.quad	_resource_shapes

	.section	__TEXT,__const
	.globl	__ZN9intrinsic6tydesc6tydesc7discrim17_cd5bf4e8b2de066c3_00E
	.align	3
__ZN9intrinsic6tydesc6tydesc7discrim17_cd5bf4e8b2de066c3_00E:
	.quad	0

_str1:
	.asciz	 "amit"

	.section	__DATA,__const
	.align	4
_tydesc2:
	.quad	0
	.quad	0+8
	.quad	0+8
	.quad	0
	.quad	0
	.quad	_glue_free3
	.quad	0
	.quad	0
	.quad	0
	.quad	0
	.quad	0
	.quad	_shape4
	.quad	_shapes
	.quad	0
	.quad	0

.zerofill __DATA,__bss,__rust_mod_map,16,3
	.section	__TEXT,__const
_shape4:
	.asciz	 "\026\005\000\"\001\001\000\000"

_tag_shapes:
	.space	1

	.align	3
_resource_shapes:
	.byte	0

	.globl	_rust_abi_version
	.align	3
_rust_abi_version:
	.quad	1

	.section	__TEXT,__eh_frame,coalesced,no_toc+strip_static_syms+live_support

.subsections_via_symbols
