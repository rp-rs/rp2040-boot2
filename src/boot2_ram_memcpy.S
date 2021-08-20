// ----------------------------------------------------------------------------
// Start of 2nd Stage Boot Code
// ----------------------------------------------------------------------------
#define ROM_FN_TABLE             0x00000014
#define ROM_DATA_TABLE           0x00000016
#define ROM_TABLE_LOOKUP         0x00000018

#define SRAM_BASE                0x20000000
#define SRAM_END                 0x20040000 // 256KB
#define SRAM_STACK0_BASE         0x20040000
#define SRAM_STACK0_END          0x20041000 // 4KB
#define SRAM_STACK1_BASE         0x20041000
#define SRAM_STACK1_END          0x20042000 // 4KB

#define XIP_BASE                 0x10000000
#define XIP_SSI_BASE             0x18000000
#define SSI_CTRLR0_OFFSET        0x00000000
#define SSI_CTRLR1_OFFSET        0x00000004
#define SSI_SSIENR_OFFSET        0x00000008
#define SSI_BAUDR_OFFSET         0x00000014
#define SSI_SPI_CTRLR0_OFFSET    0x000000f4
#define PPB_BASE                 0xe0000000
#define M0PLUS_VTOR_OFFSET       0x0000ed08

#define PICO_FLASH_SPI_CLKDIV 4
#define CMD_READ              0x03
#define ADDR_L                6
#define CTRLR0_XIP \
    (0x0 << 21) |  /* Standard 1-bit SPI serial frames */ \
    (31  << 16) |  /* 32 clocks per data frame */ \
    (0x3 << 8)     /* Send instr + addr, receive data */
#define SPI_CTRLR0_XIP \
    (CMD_READ << 24) |  /* Value of instruction prefix */ \
    (2 << 8)         |  /* 8 bit command prefix (field value is bits divided by 4) */ \
    (ADDR_L << 2)    |  /* Total number of address + mode bits */ \
    (0x0 << 0)          /* command and address both in serial format */


.cpu cortex-m0plus
.thumb

.section .text

// The exit point is passed in lr. If entered from bootrom, this will be the
// flash address immediately following this second stage (0x10000100).
// Otherwise it will be a return address -- second stage being called as a
// function by user code, after copying out of XIP region. r3 holds SSI base,
// r0...2 used as temporaries. Other GPRs not used.
.global _stage2_boot
.type _stage2_boot,%function
.thumb_func
_stage2_boot:
    //b . // Wait for the debugger
    push {lr}
    bl _enable_xip
    bl _do_memcpy44
    bl _disable_xip

// If entered from the bootrom, lr (which we earlier pushed) will be 0,
// and we vector through the table at the start of the main flash image.
// Any regular function call will have a nonzero value for lr.
check_return:
    pop {r0}
    cmp r0, #0
    beq vector_into_flash
    bx r0
vector_into_flash:
    ldr r0, =SRAM_BASE
    ldr r1, =(PPB_BASE + M0PLUS_VTOR_OFFSET)
    str r0, [r1]
    ldmia r0, {r0, r1}
    msr msp, r0
    bx r1

.global _enable_xip
.type _enable_xip,%function
.thumb_func
_enable_xip:
    push {lr}

    ldr r3, =XIP_SSI_BASE                // Use as base address where possible

    // Disable SSI to allow further config
    mov r1, #0
    str r1, [r3, #SSI_SSIENR_OFFSET]

    // Set baud rate
    mov r1, #PICO_FLASH_SPI_CLKDIV
    str r1, [r3, #SSI_BAUDR_OFFSET]

    ldr r1, =(CTRLR0_XIP)
    str r1, [r3, #SSI_CTRLR0_OFFSET]

    ldr r1, =(SPI_CTRLR0_XIP)
    ldr r0, =(XIP_SSI_BASE + SSI_SPI_CTRLR0_OFFSET)
    str r1, [r0]

    // NDF=0 (single 32b read)
    mov r1, #0x0
    str r1, [r3, #SSI_CTRLR1_OFFSET]

    // Re-enable SSI
    mov r1, #1
    str r1, [r3, #SSI_SSIENR_OFFSET]

    pop {pc}

.global _do_memcpy44
.type _do_memcpy44,%function
.thumb_func
_do_memcpy44:
    push {lr}

    ldr r0, =ROM_FN_TABLE
    ldrh r0, [r0]
    ldr r2, =ROM_TABLE_LOOKUP
    ldrh r2, [r2]

    // Query the bootrom function pointer
    ldr r1, =0x3443 // 'C','4' for _memcpy44
    blx r2

    //uint8_t *_memcpy44(uint32_t *dest, uint32_t *src, uint32_t n)
    mov r3, r0
    ldr r0, =SRAM_BASE
    ldr r1, =XIP_BASE+0x100 // Skip this boot2 region
    ldr r2, =SRAM_END-SRAM_BASE
    blx r3
    
    pop {pc}


.global _disable_xip
.type _disable_xip,%function
.thumb_func
_disable_xip:
    push {lr}

    ldr r3, =XIP_SSI_BASE                // Use as base address where possible

    // Disable SSI to allow further config
    mov r1, #0
    str r1, [r3, #SSI_SSIENR_OFFSET]

    pop {pc}


.global literals
literals:
.ltorg

.end
