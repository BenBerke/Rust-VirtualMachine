use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

use crate::constants::*;
use crate::hardware::timer::Timer;
use std::time::Duration;

pub struct Bus {
    pub mem: [u8; SIZE_MEMORY as usize],
    pub disk_drive: File,
    pub timer: Timer,

    interrupts: u64,
    last_timer_interrupt_tick: u64
}

impl Bus {
    pub fn new(disk_drive: File) -> Self {
        Self {
            mem: [0; SIZE_MEMORY as usize],
            disk_drive,
            timer: Timer::new(TIMER_TICKS_PER_SECOND), // 1000 ticks/sec = 1 tick per ms
            interrupts: 0,
            last_timer_interrupt_tick: 0,
        }
    }

    pub fn push_key_event(&mut self, keycode: u8) {
        let head = self.mem[IO_INPUT_QUEUE_HEAD] as usize;
        let tail = self.mem[IO_INPUT_QUEUE_TAIL] as usize;

        let next_tail = (tail + 1) % IO_INPUT_QUEUE_CAPACITY;

        // Queue full: drop newest event.
        if next_tail == head { return; }

        self.mem[IO_INPUT_QUEUE_DATA + tail] = keycode;
        self.mem[IO_INPUT_QUEUE_TAIL] = next_tail as u8;

        self.set_interrupt_mask(INT_MASK_KEYBOARD as u64);
    }

    pub fn read_byte(&self, addr: usize) -> u8 {
        if addr >= IO_TIMER_START && addr < IO_TIMER_START + IO_TIMER_SIZE {
            let offset = addr - IO_TIMER_START;
            return self.timer.read_byte(offset);
        }

        if addr >= SIZE_MEMORY as usize {
            println!("[BUS] Out-of-bounds read at 0x{:05X}", addr);
            return 0;
        }

        self.mem[addr]
    }

    pub fn write_byte(&mut self, addr: usize, value: u8) -> bool {
        if addr >= SIZE_MEMORY {
            println!("[BUS] Out-of-bounds write at 0x{:05X}", addr);
            return false;
        }

        self.mem[addr] = value;

        if addr == IO_INPUT_START {
            if value == 0 { self.clear_interrupt_mask(INT_MASK_KEYBOARD as u64); }
            else { self.set_interrupt_mask(INT_MASK_KEYBOARD as u64); }
        }

        true
    }

    pub fn read_u64(&self, addr: usize) -> u64 {
        if addr == IO_TIMER_START {
            return self.timer.read_ticks();
        }

        if addr + 8 > SIZE_MEMORY as usize {
            println!("[BUS] Out-of-bounds u64 read at 0x{:05X}", addr);
            return 0;
        }

        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.mem[addr..addr + 8]);
        u64::from_le_bytes(bytes)
    }

    pub fn write_u64(&mut self, addr: usize, value: u64) -> bool {
        if addr + 8 > SIZE_MEMORY {
            println!("[BUS] Out-of-bounds u64 write at 0x{:05X}", addr);
            return false;
        }

        let bytes = value.to_le_bytes();
        self.mem[addr..addr + 8].copy_from_slice(&bytes);

        true
    }

    pub fn read_timer_ticks(&self) -> u64 {
        self.timer.read_ticks()
    }

    pub fn set_interrupt_mask(&mut self, mask: u64){
        self.interrupts |= mask;
    }
    pub fn get_interrupts(&self) -> u64 {
        self.interrupts
    }
    pub fn clear_interrupt_mask(&mut self, mask: u64) {
        self.interrupts &= !mask;
    }

    pub fn load_sector_from_disk(&mut self, sector_number: u64, ram_target_address: usize,) -> bool {
        let disk_offset = sector_number * SIZE_SECTOR;

        if let Err(err) = self.disk_drive.seek(SeekFrom::Start(disk_offset)) {
            println!("[DISK ERROR] Seek failed: {}", err);
            return false;
        }

        let start = ram_target_address;
        let end = start + SIZE_SECTOR as usize;

        if end > SIZE_MEMORY {
            println!(
                "[DISK ERROR] Sector load target out of RAM bounds: {}..{}",
                start,
                end
            );
            return false;
        }

        if let Err(err) = self.disk_drive.read_exact(&mut self.mem[start..end]) {
            println!(
                "[DISK ERROR] Failed to read sector {}: {}",
                sector_number,
                err
            );
            return false;
        }

        true
    }

    pub fn write_to_disk(
        &mut self,
        ram_start: usize,
        ram_end: usize,
        sector: u64,
    ) -> bool {
        if ram_start > ram_end || ram_end > SIZE_MEMORY as usize {
            println!(
                "[DISK ERROR] Invalid RAM range for disk write: {}..{}",
                ram_start,
                ram_end
            );
            return false;
        }

        let disk_offset = sector * SIZE_SECTOR;

        if let Err(err) = self.disk_drive.seek(SeekFrom::Start(disk_offset)) {
            println!("[DISK ERROR] Seek failed: {}", err);
            return false;
        }

        if let Err(err) = self.disk_drive.write_all(&self.mem[ram_start..ram_end]) {
            println!("[DISK ERROR] Write failed: {}", err);
            return false;
        }

        true
    }
}