//! Startup code and minimal runtime for TriCore(TM) TC162 microcontrollers
//!
//! This crate contains all the required parts to build a `no_std` application (binary crate) that
//! targets a TC3xx AURIX(TM) microcontroller family.
//!
//! ARCHITECTURE: TC162
//!
//! UC FAMILY: TC3xx
//!
//! # Features
//!
//! This crate takes care of:
//!
//! - C/C++/RUST runtime environment startup file.
//!
//! - To initialize ABI registers
//! - To initialize CSA context area
//! - To initialize Stack area needed for 'C' code
//! - To initialize ECC in RAM memories (not done for AURIX)
//! - To clear BSS (not-initialized) data
//! - To initialize DATA (initialized) data
//! - To initialize C++ global object instances
//! - To provide startup hooks for user defined code
//!
//! ## Multicore Startup
//!
//! Crt0 startup on multicore systems is driven by a configuration provided by a corresponding
//! linker file. Configuration table contains one entry for each Core present in uC
//!
//! ## Startup Hooks
//!
//! Crt0 provides two startup hooks the user can use to implement system dependent early HW
//! initialization.
//!
//! ### 'Crt0PreInit()' hook called
//! - after STACK area and CSA list on given core is initialized
//! - before Clear and Copy table operation is executed
//! ! HW initialization code cannot rely on global data (BSS or DATA)
//!
//! ### 'Crt0PostInit()' hook called
//! - when all startup initialization is finished (data can be used)
//! - before calling shared_main() application entry point
//!
//! ## C++ support
//! The crt0 startup code provides a default global object initialization based on ctor table
//! registered per core.
//! The format of 'ctor' table expected:
//! ctor_setion
//! {
//! __CTOR_LIST__ = .;
//! LONG((__CTOR_END__ - __CTOR_LIST__) / 4 - 2)
//! KEEP (*(.ctors));
//! LONG(0);
//! __CTOR_END__ = .;
//! }
//! By default, the linker gathers all C++ module ctor entries into one common ctor table. In case
//! of multicore object initialization, the user must provide their system of CTOR initialization.
//!
//! ## Application specific crt0 Routines Implementation]
//! Startup routines are implemented as WEAK functions to allow application-specific replacement
//! in case of need.
//! - Crt0BssInit
//! - Crt0DataInit
//! - Crt0CtorInit
//! - Crt0PreInit
//! - Crt0PostInit
//!
//! # FILE HISTORY:
//!
//!    V1  01.2023  MF
//!    - Base reference example
//!
// *************************************************************************************************
// Copyright (C) 2015-2023 HighTec EDV-Systeme GmbH.  All rights reserved.
// This is proprietary software. Any use of the software requires a written
// license agreement with HighTec EDV-Systeme GmbH. Please contact for
// detailed license information: info@hightec-rt.com.
// *************************************************************************************************
#![no_std]

use core::arch::global_asm;
use core::panic::PanicInfo;

extern "C" {
    pub fn _exit(status: u32) -> !;
}
    

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {
        unsafe { _exit(0xff); } 
    }
}

#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[export_name = "main"]
        pub unsafe fn __main() -> ! {
            // type check the given path
            let f: fn() -> ! = $path;

            f()
        }
    }
}

#[no_mangle]
pub unsafe fn abort() -> ! {
    _exit (0xff);
}

/// Parse cfg attributes inside a global_asm call.
macro_rules! cfg_global_asm {
    {@inner, [$($x:tt)*], } => {
        global_asm!{$($x)*}
    };
    (@inner, [$($x:tt)*], #[cfg($meta:meta)] $asm:literal, $($rest:tt)*) => {
        #[cfg($meta)]
        cfg_global_asm!{@inner, [$($x)* $asm,], $($rest)*}
        #[cfg(not($meta))]
        cfg_global_asm!{@inner, [$($x)*], $($rest)*}
    };
    {@inner, [$($x:tt)*], $asm:literal, $($rest:tt)*} => {
        cfg_global_asm!{@inner, [$($x)* $asm,], $($rest)*}
    };
    {$($asms:tt)*} => {
        cfg_global_asm!{@inner, [], $($asms)*}
    };
}

cfg_global_asm!{
    // SYMBOLS & DEFINES & MACROS
    // crt0_config : Entry offsets
    // Must correspond to __crt0_config structure defined in the Linker file
    ".equ STACK,         0x00
    .equ STACK_SIZE,    0x04
    .equ CSA,           0x08
    .equ CSA_SIZE,      0x0C
    .equ SDATA,         0x10
    .equ SDATA2,        0x14
    .equ SDATA3,        0x18
    .equ SDATA4,        0x1C
    .equ CLEAR_TABLE,   0x20
    .equ COPY_TABLE,    0x24
    .equ CTOR_TABLE,    0x28
    .equ CPUINIT_SIZE,  0x2C",  

    // CSA_ENTRY supportive symbols
    // CSA_ENTRY_SIZE  : in bytes, given by TC3xx architecture
    // LW_OFFSET_SHIFT : Link Word, low part shift offset
    ".equ CSA_ENTRY_SIZE,  64
    .equ LW_OFFSET_SHIFT, -6",

    // CLEAR_TABLE structure offsets
    // Offsets must correspond to the Linker __clear_table structure
    // Offsets are in bytes
    // Structure
    //     1. LONG : DST  - Destaination base address
    //     2. LONG : SIZE - number of data to clear (write 0) in bytes 
    ".equ CLEAR_TABLE_DST,     0x00
    .equ CLEAR_TABLE_SIZE,    0x04
    .equ CLEAR_TABLE_OFFSET,  0x08",  // Size of one entry in bytes 

    // COPY_TABLE structure offsets
    // Offsets must correspond to the Linker __copy_table structure
    // Offsets are in bytes
    // Structure
    //     0. LONG : SRC  - Source address to copy data from
    //     1. LONG : DST  - Destaination address to copy data to
    //     2. LONG : SIZE - number of data to copy in bytes    
    ".equ COPY_TABLE_SRC,      0x00
    .equ COPY_TABLE_DST,      0x04
    .equ COPY_TABLE_SIZE,     0x08
    .equ COPY_TABLE_OFFSET,   0x0C",  // Size of one entry in bytes

    // EXTERNAL SYMBOLS REQUIRED BY CRT0  
    // application shared main entry
    ".extern main, STT_FUNC, 0",

    // crt0 config structure from linker
    ".extern __crt0_config, STT_OBJECT, 0",

    // EXPORTED SYMBOLS FROM CRT0
    // _crt0_reset : reset entry point from where to start inactive cores
    ".global _crt0_reset
    .type _crt0_reset STT_FUNC",

    // _start : shared multicore crt0 startup code entry point
    ".global _start
    .type _start STT_FUNC",

    // CRT0 RESET VECTOR
    // Here execution starts after the Reset.
    // The first duty is to force eventual address segment change in Aurix core
    // from non-cached memory to a cacheable one
   ".section .crt0_boot.code, \"ax\"
   _crt0_reset:
       movh.a  %a15, hi:_start
       lea     %a15, [%a15] lo:_start
       ji      %a15",

    // CRT0 STARTUP CODE
    // A multicore shared code implementation of 'C' runtime intialization
    // located in a standard .text section that might be in cacheable region
    ".section .text, \"ax\"
    _start:",

    // CRT0 CONFIG TABLE BASE POINTER SETUP
    // Config table contains parameters controlling crt0 startup execution.
    // It is prepared by the linker file with the knowledge of the final placement.
    // Registers used by the crt0 startup code
    // 'A14` : is used as Core Entry Base Pointer in crt0 configuration structure
    //         throughout the statup asm code.
    // The A14 register value is saved by Aurix core in upper context during subroutine calls.
    "movh.a  %a14, hi:__crt0_config
    lea     %a14, [%a14]lo:__crt0_config
    mfcr    %d15, $core_id                /* read CoreID(0xfe1c) SFR register */
    and     %d15, 7                       /* mask the lowest 3 bits */
    mul     %d15, %d15, CPUINIT_SIZE      /* get the core entry base address */
    addsc.a %a14, %a14, %d15, 0",

    // SMALL ADDRESS REGISTERS INIT
    // Values given by crt0 configuration structure from the linker file.
    // Four dedicated registers, if they are used
    // a0 - usually small data (rw)
    // a1 - usually small const data (r)
    // a8 - usually OS / application specific
    // a9 - usually OS / application specific
    "mfcr    %d15, $psw
    or    %d15, %d15, 0x7f
    andn  %d15, %d15, 0x80
    or    %d15, %d15, 0x100
    mtcr    $psw, %d15",

    "ld.w    %d15, [%a14] SDATA
    mov.a   %a0, %d15
    ld.w    %d15, [%a14] SDATA2
    mov.a   %a1, %d15
    ld.w    %d15, [%a14] SDATA3
    mov.a   %a8, %d15
    ld.w    %d15, [%a14] SDATA4
    mov.a   %a9, %d15",

    "mfcr    %d15, $psw
    andn  %d15, %d15, 0x100
    mtcr    $psw, %d15",

    // CSA CONTEXT AREA INIT
    // Linked list initialization of CSA entries (TriCore specific feature) used to save
    // function context during standard 'C' function calls.
    // CSA entry and linked list has fixed structure given by AURIX architecture.
    // Number of CSA entries (max 256 entries) is part of crt0 configuration in the linker file.
    "mov     %d4, 0
    mtcr    $pcxi, %d4                 
    ld.w    %d4, [%a14] CSA_SIZE       
    sh      %d4, %d4, LW_OFFSET_SHIFT  
    mov.a   %a4, %d4                   
    add.a   %a4, -1                    
    mov     %d4, CSA_ENTRY_SIZE
    mov.a   %a3, %d4                   
    ld.a    %a15, [%a14] CSA           
    movh    %d5, 0x000F                
    mov.d   %d15, %a15                 
    sh      %d15, %d15, -12            
    and     %d15, %d5                  
    mov.u   %d5, 0xFFFF                
    mov.d   %d4, %a15                  
    sh      %d4, %d4, LW_OFFSET_SHIFT  
    and     %d4, %d5                   
    or      %d4, %d15                  
    mtcr    $fcx, %d4                  
    loop_csa:
    add     %d4, 1                     
    st.w    [%a15], %d4                
    add.a   %a15, %a3                  
    loop    %a4, loop_csa              
    mtcr    $lcx, %d4",                  

    // STACK INIT
    // Mandatory operation before calling any 'C' function
    // Two things to do
    // 1. correct ECC checksum syndroms for complete Stack area by writing with required
    //    data size instructions
    // 2. Stack pointer init used by 'C' code
    // Startup code initialize both TriCore stack pointers, User and Interrupt, to the same area.
    // - the code runs with 'PSW.IS = 1' after the reset -> shared stack
    // - the separation of User and Interrupt stack is left on Application (usually OS)
    //   later on. 
    "ld.w    %d4, [%a14] STACK       
    mov.a   %a4, %d4                
    ld.w    %d4, [%a14] STACK_SIZE  
    mov.d   %d1, %a4                
    add     %d1, %d1, %d4           
    mov.a   %sp, %d1                
    mtcr    $isp, %d1               
    call    clear_exec",   

    //	install trap handlers
	"movh	%d0,hi:first_trap_table		#; load $btv
	addi	%d0,%d0,lo:first_trap_table
	mtcr	$btv,%d0
	isync",
    // CRT0 PRE-INIT 'C' USER CODE
    // Chance for user to execute HW init at the very beginning, before longer operations
    // take place, like memory clear and copy of init data from Flash to RAM.
    // In case of CORE dependent Hook execution,
    // the application must read it ourselves (physical CoreID might not correspond
    // to a consecutive sequence needed for array operations).
    // Pre-init code MUST rely only on Stack variables only !
    "call    Crt0PreInit",

    // CLEAR .BSS SECTIONS
    // Areas to clear are given in the __clear_table config entry.
    // The crt0 function is of WEAK type to allow the user implementation in the application
    // by for example by 'C' specific routine
 
    "ld.w    %d4, [%a14] CLEAR_TABLE",
    "mov.a   %a4, %d4",
    "call    Crt0BssInit",

    // COPY INITIALIZED DATA
    // Initialization of data regions provided in __copy table in crt0 configuration structure.
    // The crt0 function is of WEAK type to allow the user implementation in the application.
    "ld.w    %d4, [%a14] COPY_TABLE",
    "mov.a   %a4, %d4",
    "call    Crt0DataInit",

    // C++ GLOBAL OBJECT INITIALIZATION
    // The ctor table (constructors to call) is provided as one of the crt0_configr structure entry.
    // Each core can have its own ctor table array, if implemented in the linker file
    // (not in BSP case)
    "ld.w    %d4, [%a14] CTOR_TABLE",
    "mov.a   %a4, %d4",
    "call    Crt0CtorInit",
    
    // CRT0 POST-INIT 'C' USER CODE
    // Chance for user to execute specific code before jump to application entry,
    // 'shared main()' in case of BSP
    // In case of core dependent Hook execution,
    // the application must read it ourselves (physical CoreID might not correspond
    // to a consecutive sequence needed for array operations).
    "call    Crt0PostInit",

    //	install interrupt handlers
	"movh	%d0,hi:__INTERRUPT_TABLE		#; load $btv
	addi	%d0,%d0,lo:__INTERRUPT_TABLE
	mtcr	$biv,%d0
	isync",

    // CRT0 END : ENTRY TO APPLICATION
    // Jump to the application entry point, shared across all cores in case of BSP examples
    // In case of core dependent Hook execution, the application must read it ourselves,
    // physical CoreID might not correspond to a consecutive sequence needed for array operations
    // The return from the application is not expected, hard to say what the embedded system
    // shall do here
    "call    main",
    "j      _exit",

}

// FUNCTION: _exit
global_asm!(
    ".weak _exit",
    ".type _exit, %function",
    "_exit:",
        "debug",  /* debug stop in case of active debugging process,
                  otherwise 'nop' instruction */
        "j .",    /* infinitive loop, waiting for eventual timeout watchdog */
);
// FUNCTION: Crt0PreInit
// User hook before 'C' runtime initialization. Empty routine in case of crt0 startup code.
global_asm!(
    ".weak Crt0PreInit",
    ".type Crt0PreInit, %function",
    "Crt0PreInit:",
        "ret",
);

// FUNCTION: Crt0PostInit
// User hook after 'C' runtime initialization. Empty routine in case of crt0 startup code.
global_asm!(
    ".weak Crt0PostInit",
    ".type Crt0PostInit, %function",
    "Crt0PostInit:",
        "ret",
);

// FUNCTION: Crt0BssInit
// Default Crt0 BSS clear function. It goes through clear_table entries and calls the clear
// operation for each of them
//
// Input
// A[4] : core's clear_table base pointer
global_asm!(
    ".weak Crt0BssInit",
    ".type Crt0BssInit, %function",
    "Crt0BssInit:",
        "mov.aa  %a13, %a4",                            // Local pointer for clear_table entry 
        "mov.a   %a12, CLEAR_TABLE_OFFSET",             // Clear_table next entry offset 
    "_table_bss_clear_loop:",
        "ld.w    %d15, [%a13] CLEAR_TABLE_DST",         // Base address of the area to clear 
        "jeq     %d15, -1, _table_bss_clear_loop_end",  // Checks table termination value -1, 
        "mov.a   %a4, %d15",                            // Prepare area start pointer for clear routine 
        "ld.w    %d4, [%a13] CLEAR_TABLE_SIZE",         // Get size of the area
        "call    clear_exec",                           // Call Clear routine with saving Upper Context 
        "add.a   %a13,%a12",                            // Next row from BSS clear table 
        "j       _table_bss_clear_loop",
    "_table_bss_clear_loop_end:",
        "ret",
);

// FUNCTION: Crt0DataInit
// Default Crt0 DATA init function. It goes through copy_table entries and calls
// copy operation for each of them.
// 
// Input
// A[4] : core's copy_table pointer
global_asm!(
    ".weak Crt0DataInit",
    ".type Crt0DataInit, %function",

    "Crt0DataInit:",
        "mov.aa  %a13, %a4",                            // Local Pointer for copy table 
        "mov.a   %a12, COPY_TABLE_OFFSET",              // Copy table item offset in bytes 
    "_table_data_copy_loop:",
        "ld.w    %d15, [%a13]COPY_TABLE_DST",           // Start address of the destination copy area 
        "jeq     %d15, -1, _table_data_copy_loop_end",  // Checks table termination value -1, 
        "mov.a   %a4, %d15",
        "ld.w    %d4, [%a13]COPY_TABLE_SRC",            // First Address of the source copy table 
        "mov.a   %a5, %d4",                             // store it into address register %a5 
        "ld.w    %d4, [%a13]COPY_TABLE_SIZE",           // Get size of the area
        "call    copy_exec",                            // Call Copy routine 
        "add.a   %a13,%a12",                            // Next row from BSS copy table 
        "j       _table_data_copy_loop",
    "_table_data_copy_loop_end:",
        "ret",
);

// FUNCTION: Crt0CtorInit
// Default global C++ object initialization. It goes through ctor table and calls
// global constructors.
//
// Input
// A[4] : CTOR table base address
global_asm!(
    ".weak Crt0CtorInit",
    ".type Crt0CtorInit, %function",

    "Crt0CtorInit:",
        "jz.a    %a4, _ctor_exec_end",   // check against no table present 
        "ld.w    %d4, [%a4+]4",          // get number of entries 
        "mov.a   %a15, %d4 ",            // and store it into address register %a15 
        "jz.a    %a15, _ctor_exec_end",  // check against no entry (size = 0) 
        "add.a   %a15, -1",              // consider always one 'loop' execution 
    "_ctor_exec_loop:",
        "ld.w    %d4, [%a4+]4",          // read the function pointer
        "mov.a   %a13, %d4 ",            // and store it into the address register %a13
        "calli   %a13",                  // call the function
        "loop    %a15,_ctor_exec_loop",  // go through all functions
    "_ctor_exec_end:",
        "ret",
    );

// MODULE LOCAL ROUTINES
// Used only within this module

// FUNCTION: clear_exec
// Executes the erase loop from start address for specified number of bytes.
// It uses 64bit Store instruction
//
// Input
// A[4] : start address
// D[4] : size in bytes
global_asm!(
    "clear_exec:",
        "jz      %d4,_clear_exec_end    ",// Return if size is zero 
        "add     %d4,-1                 ",// decrement to take into account always one loop execution 
        "sh      %d4,-3                 ",// adjustment of the clear loop for the double word write instruction 
        "mov.a   %a15,%d4               ",// init loop counter 
        "mov     %e14,0                 ",// Zero value 
    "_clear_exec_loop:",
        "st.d    [%a4+]8,%e14           ",// Store 64bit value
        "loop    %a15,_clear_exec_loop  ",// execution loop until zero
    "_clear_exec_end:",
        "ret",
);

// FUNCTION: copy_exec
// Executes the copy loop from start address to end address.
// Routine is simple Byte copy without any optimization.
//
// Input
// A[4] : start write address
// A[5] : start read address
// D[4] : size in bytes
global_asm!(
    "copy_exec:",
        "mov     %d15,%d4",
        "jz      %d15,_copy_exec_end",  // Return if size is zero
        "add     %d15,-1",              // decrement to take into account value 0 in loop
        "mov.a   %a15,%d15",
    "_copy_exec_loop:",
        "ld.b    %d15, [%a5+]1",
        "st.b    [%a4+]1, %d15",
        "loop    %a15,_copy_exec_loop",
    "_copy_exec_end:",
        "ret",
);

// Initial simple trap table
global_asm!(
"	.section .traptab, \"ax\", @progbits",
".macro trapentry from=0, to=7",
"	debug",
"	mov.u	%d14, \\from << 8",
"	add	%d14,%d14,%d15",
"	mov.a	%a14,%d14",
"	addih.a	%a14,%a14,0xdead",
"	j	_exit",
"0:",
"	j	0b",
"	nop",
"	rfe",
"	.align 5",
"   .if \\to-\\from",
"	trapentry \"(\\from+1)\",\\to",
"   .endif",
".endm",
"	.align 8",
"	.global first_trap_table",
"first_trap_table:",
"	trapentry 0, 7",
);

// Defintion of the interrupt table
global_asm!(
    ".altmacro",
    ".macro inttab_entry.1 p,u ",
    "   .align 5",
    "   .globl __inttab_entry_\\u\\p ",
    "   .type __inttab_entry_\\u\\p ,@function ",
    "   __inttab_entry_\\u\\p : ",
    "    svlcx",
    "    movh.a  %a14, hi: __INTERRUPT_HANDLER\\u\\p ",
    "    lea     %a14, [%a14]lo: __INTERRUPT_HANDLER\\u\\p ",
    "    calli   %a14",
    "    rslcx",
    "    rfe",
    ".endm ",
    ".macro inttab_entry from=0, to=10",
    "   inttab_entry.1  %(\\from), _ ",
    "    .align 5",
    "   .if  \\to-\\from  ",
    "       inttab_entry  \"(\\from+1)\", \\to ",
    "   .endif",
    ".endm ",
    ".pushsection .interrupt_table, \"ax\",@progbits",
    "   .align 8",
    "   .globl __INTERRUPT_TABLE ",
    "   .type __INTERRUPT_TABLE, @object",
    "   __INTERRUPT_TABLE:",
    "   inttab_entry 0, 15",
    "   inttab_entry 16, 32",
    ".popsection",
);

// Defintion of the default interrupt handlers
global_asm!(
    ".altmacro",
    ".macro ih_name p,u",
    "   .weak __INTERRUPT_HANDLER\\u\\p",
    "   .type __INTERRUPT_HANDLER\\u\\p, @function",
    "    __INTERRUPT_HANDLER\\u\\p:",
    ".endm ",
    ".macro interrupt_hnd from=0, to=10",
    "   ih_name %(\\from), _ ",
    "   .if  \\to-\\from  ",
    "       interrupt_hnd  \"(\\from+1)\", \\to ",
    "   .endif",
    ".endm ",
    ".pushsection .text.default_int_handler, \"ax\",@progbits",
    "interrupt_hnd 0, 15",
    "interrupt_hnd 16, 32",
    "   ret",
    ".popsection",
);