pub const SIZE_SECTOR: u64 = 512;
pub const SIZE_MEMORY: usize = 512 * 1024; // 524,288 Bytes (512KB)
pub const REG_COUNT: u8 = 8;

// Display Config (320x240 Indexed Palette)
pub const SCREEN_WIDTH: usize = 320;
pub const SCREEN_HEIGHT: usize = 240;
pub const SCREEN_VIRTUAL_WIDTH: usize = SCREEN_WIDTH * 3;
pub const SCREEN_VIRTUAL_HEIGHT: usize = SCREEN_HEIGHT * 3;

// =========================================================================
// MEMORY LAYOUT SEGMENTS (HEX ENCODED)
// =========================================================================

pub const SP_REG: usize = 1;

// Bank 0: Instructions / Executable Code (128 KB)
pub const INSTR_START: usize = 0x00000;
pub const INSTR_END: usize   = 0x1FFFF;

// Bank 1: Kernel Global Data & Data Scratchpad (128 KB)
pub const DATA_START: usize = 0x20000;
pub const DATA_END: usize = 0x3FFFF;

// Bank 2: Memory-Mapped I/O & Graphics (128 KB)
pub const MMIO_START: usize = 0x40000;
pub const VRAM_START: usize = 0x40000; // VRAM sits at the base of MMIO

pub const VRAM_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT; // 76,800 Bytes (~75KB)
pub const VRAM_END: usize = VRAM_START + VRAM_SIZE; // Ends at 0x52C00
pub const MMIO_END: usize = 0x5FFFF; // Remaining ~51KB for audio/input/palettes
pub const IO_INPUT_START: usize = VRAM_END; // 0x52C00
pub const SCREEN_MODE_PIXEL: u8 = 0;
pub const SCREEN_MODE_TEXT: u8 = 1;
pub const FONT_WIDTH: usize = 8;
pub const FONT_HEIGHT: usize = 8;
pub const TEXT_COLS: usize = SCREEN_WIDTH / FONT_WIDTH;   // 40
pub const TEXT_ROWS: usize = SCREEN_HEIGHT / FONT_HEIGHT; // 30
pub const TEXT_CELL_COUNT: usize = TEXT_COLS * TEXT_ROWS; // 1200
pub const IO_INPUT_SIZE: usize = 128;
pub const IO_INPUT_QUEUE_HEAD: usize = IO_INPUT_START + 0x5B;
pub const IO_INPUT_QUEUE_TAIL: usize = IO_INPUT_START + 0x5C;
pub const IO_INPUT_QUEUE_DATA: usize = IO_INPUT_START + 0x5D;
pub const IO_INPUT_QUEUE_CAPACITY: usize = 32;
pub const IO_SCREEN_MODE: usize = IO_INPUT_START + IO_INPUT_SIZE; // 0x52C80

// Bank 3: User Space Heap / Graphics Backbuffer (112 KB)
pub const USER_START: usize = 0x60000;
pub const USER_END: usize = 0x7BFFF;

// hardware Stack Region (16 KB)
// The stack pointer (SP) will start at 0x7FFFF and grow DOWNWARDS
pub const STACK_START: usize = 0x7C000;
pub const STACK_END: usize = 0x7FFFF;

pub const SECTION_DATA: &str = "data";
pub const SECTION_TEXT: &str = "text";

pub const BOOTLOADER_BASE_ADDRESS: usize = 0;
pub const KERNEL_LOAD_ADDRESS: usize = SIZE_SECTOR as usize; // 512
pub const KERNEL_HEADER_SIZE: usize = 4;
pub const KERNEL_CODE_ADDRESS: usize = KERNEL_LOAD_ADDRESS + KERNEL_HEADER_SIZE; // 516
pub const CYCLES_PER_FRAME: usize = 50_000;
pub const IO_TIMER_START: usize = 0x52D00;
pub const IO_TIMER_SIZE: usize = 8;
pub const TIMER_TICKS_PER_SECOND: u64 = 1000;

// ==========================
// Interrupt Mask
// ==========================
pub const INT_MASK_TIMER: usize = 1;
pub const INT_MASK_KEYBOARD: usize = 2;
pub const TIMER_INTERRUPT_PERIOD_TICKS: u64 = 16;

// ==========================
// SYSTEM CALLSs
// ==========================
pub const SYS_ERR_UNKNOWN: u64 = u64::MAX;
pub const SYS_TIMER: u64 = 1;
pub const SYS_HZ: u64 = 2;

// ==========================
// FONT ATLAS
// ==========================
pub const FONT_ATLAS_COLS: usize = 16;
pub const FONT_ATLAS_ROWS: usize = 16;
pub const FONT_ATLAS_WIDTH: usize = FONT_ATLAS_COLS * FONT_WIDTH;   // 128
pub const FONT_ATLAS_HEIGHT: usize = FONT_ATLAS_ROWS * FONT_HEIGHT; // 128
pub const FONT_ATLAS_BYTE_SIZE: usize = (FONT_ATLAS_WIDTH * FONT_ATLAS_HEIGHT) / 8;