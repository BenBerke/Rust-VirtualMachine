use crate::constants::*;

pub const PALETTE: [u32; 16] = [
    0x000000,
    0x555555,
    0xaaaaaa,
    0xffffff,
    0xffaaaa,
    0xaa5555,
    0x550000,
    0x555500,
    0xaaaa55,
    0xaaffaa,
    0x55aa55,
    0x005500,
    0x55aaaa,
    0xaaaaff,
    0x5555aa,
    0xaa55aa,
];

pub struct FontAtlas {
    pixels: &'static [u8; FONT_ATLAS_BYTE_SIZE],
}

impl FontAtlas {
    pub fn builtin() -> Self {
        Self { pixels: include_bytes!("../../assets/font_atlas_1bpp.bin"), }
    }

    pub fn glyph_pixel_enabled(&self, ascii: u8, local_x: usize, local_y: usize) -> bool {
        if local_x >= FONT_WIDTH || local_y >= FONT_HEIGHT {
            return false;
        }

        let glyph_index = ascii as usize;

        let glyph_col = glyph_index % FONT_ATLAS_COLS;
        let glyph_row = glyph_index / FONT_ATLAS_COLS;

        let atlas_x = glyph_col * FONT_WIDTH + local_x;
        let atlas_y = glyph_row * FONT_HEIGHT + local_y;

        let pixel_index = atlas_y * FONT_ATLAS_WIDTH + atlas_x;
        let byte_index = pixel_index / 8;
        let bit_index = 7 - (pixel_index % 8);

        ((self.pixels[byte_index] >> bit_index) & 1) != 0
    }
}

pub struct Screen {
    pub framebuffer: Vec<u32>,
    font_atlas: FontAtlas,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            framebuffer: vec![0; SCREEN_WIDTH * SCREEN_HEIGHT],
            font_atlas: FontAtlas::builtin(),
        }
    }

    pub fn render(&mut self, mem: &[u8]) {
        match mem[IO_SCREEN_MODE] {
            SCREEN_MODE_PIXEL => self.render_pixel_vram(mem),
            SCREEN_MODE_TEXT => self.render_text_vram(mem),
            _ => self.render_pixel_vram(mem),
        }
    }

    fn render_pixel_vram(&mut self, mem: &[u8]) {
        for i in 0..VRAM_SIZE {
            let pixel_byte = mem[VRAM_START + i];
            let color_index = pixel_byte & 0x0F;

            self.framebuffer[i] = lookup_palette(color_index as usize);
        }
    }

    fn render_text_vram(&mut self, mem: &[u8]) {
        self.framebuffer.fill(lookup_palette(0));

        for cell in 0..TEXT_CELL_COUNT {
            let cell_addr = VRAM_START + cell * TEXT_CELL_SIZE;

            let ascii = mem[cell_addr];
            let attr = mem[cell_addr + 1];

            let fg_color_index = (attr & 0x0F) as usize;
            let bg_color_index = ((attr >> 4) & 0x0F) as usize;

            let char_x = cell % TEXT_COLS;
            let char_y = cell / TEXT_COLS;

            self.draw_char_from_atlas(
                char_x * FONT_WIDTH,
                char_y * FONT_HEIGHT,
                ascii,
                fg_color_index,
                bg_color_index,
            );
        }
    }

    fn draw_char_from_atlas(&mut self, x: usize, y: usize, ascii: u8, fg_color_index: usize, bg_color_index: usize, ) {
        let fg = lookup_palette(fg_color_index);
        let bg = lookup_palette(bg_color_index);

        for row in 0..FONT_HEIGHT {
            for col in 0..FONT_WIDTH {
                let pixel_x = x + col;
                let pixel_y = y + row;

                if pixel_x >= SCREEN_WIDTH || pixel_y >= SCREEN_HEIGHT { continue; }

                let enabled = self.font_atlas.glyph_pixel_enabled(ascii, col, row);

                self.framebuffer[pixel_y * SCREEN_WIDTH + pixel_x] = if enabled { fg } else { bg };
            }
        }
    }
}

pub fn lookup_palette(color_index: usize) -> u32 { PALETTE[color_index & 0x0F] }