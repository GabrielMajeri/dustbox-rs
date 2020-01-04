#[derive(Clone)]
pub enum ColorSpace {
    RGB(u8, u8, u8), // 6 + 6 + 6 bit rgb color
    None,
}

fn rgb6(r: u8, b: u8, g: u8) -> ColorSpace {
    ColorSpace::RGB(r << 2, b << 2, g << 2)
}

pub fn text_palette() -> [ColorSpace; 64] {[
    rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x2a),rgb6(0x00,0x2a,0x00),rgb6(0x00,0x2a,0x2a),
    rgb6(0x2a,0x00,0x00),rgb6(0x2a,0x00,0x2a),rgb6(0x2a,0x2a,0x00),rgb6(0x2a,0x2a,0x2a),
    rgb6(0x00,0x00,0x15),rgb6(0x00,0x00,0x3f),rgb6(0x00,0x2a,0x15),rgb6(0x00,0x2a,0x3f),
    rgb6(0x2a,0x00,0x15),rgb6(0x2a,0x00,0x3f),rgb6(0x2a,0x2a,0x15),rgb6(0x2a,0x2a,0x3f),
    rgb6(0x00,0x15,0x00),rgb6(0x00,0x15,0x2a),rgb6(0x00,0x3f,0x00),rgb6(0x00,0x3f,0x2a),
    rgb6(0x2a,0x15,0x00),rgb6(0x2a,0x15,0x2a),rgb6(0x2a,0x3f,0x00),rgb6(0x2a,0x3f,0x2a),
    rgb6(0x00,0x15,0x15),rgb6(0x00,0x15,0x3f),rgb6(0x00,0x3f,0x15),rgb6(0x00,0x3f,0x3f),
    rgb6(0x2a,0x15,0x15),rgb6(0x2a,0x15,0x3f),rgb6(0x2a,0x3f,0x15),rgb6(0x2a,0x3f,0x3f),
    rgb6(0x15,0x00,0x00),rgb6(0x15,0x00,0x2a),rgb6(0x15,0x2a,0x00),rgb6(0x15,0x2a,0x2a),
    rgb6(0x3f,0x00,0x00),rgb6(0x3f,0x00,0x2a),rgb6(0x3f,0x2a,0x00),rgb6(0x3f,0x2a,0x2a),
    rgb6(0x15,0x00,0x15),rgb6(0x15,0x00,0x3f),rgb6(0x15,0x2a,0x15),rgb6(0x15,0x2a,0x3f),
    rgb6(0x3f,0x00,0x15),rgb6(0x3f,0x00,0x3f),rgb6(0x3f,0x2a,0x15),rgb6(0x3f,0x2a,0x3f),
    rgb6(0x15,0x15,0x00),rgb6(0x15,0x15,0x2a),rgb6(0x15,0x3f,0x00),rgb6(0x15,0x3f,0x2a),
    rgb6(0x3f,0x15,0x00),rgb6(0x3f,0x15,0x2a),rgb6(0x3f,0x3f,0x00),rgb6(0x3f,0x3f,0x2a),
    rgb6(0x15,0x15,0x15),rgb6(0x15,0x15,0x3f),rgb6(0x15,0x3f,0x15),rgb6(0x15,0x3f,0x3f),
    rgb6(0x3f,0x15,0x15),rgb6(0x3f,0x15,0x3f),rgb6(0x3f,0x3f,0x15),rgb6(0x3f,0x3f,0x3f),
]}

pub fn mtext_palette() -> [ColorSpace; 64] {[
    rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),
    rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),
    rgb6(0x2a,0x2a,0x2a),rgb6(0x2a,0x2a,0x2a),rgb6(0x2a,0x2a,0x2a),rgb6(0x2a,0x2a,0x2a),
    rgb6(0x2a,0x2a,0x2a),rgb6(0x2a,0x2a,0x2a),rgb6(0x2a,0x2a,0x2a),rgb6(0x2a,0x2a,0x2a),
    rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),
    rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),
    rgb6(0x3f,0x3f,0x3f),rgb6(0x3f,0x3f,0x3f),rgb6(0x3f,0x3f,0x3f),rgb6(0x3f,0x3f,0x3f),
    rgb6(0x3f,0x3f,0x3f),rgb6(0x3f,0x3f,0x3f),rgb6(0x3f,0x3f,0x3f),rgb6(0x3f,0x3f,0x3f),
    rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),
    rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),
    rgb6(0x2a,0x2a,0x2a),rgb6(0x2a,0x2a,0x2a),rgb6(0x2a,0x2a,0x2a),rgb6(0x2a,0x2a,0x2a),
    rgb6(0x2a,0x2a,0x2a),rgb6(0x2a,0x2a,0x2a),rgb6(0x2a,0x2a,0x2a),rgb6(0x2a,0x2a,0x2a),
    rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),
    rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),
    rgb6(0x3f,0x3f,0x3f),rgb6(0x3f,0x3f,0x3f),rgb6(0x3f,0x3f,0x3f),rgb6(0x3f,0x3f,0x3f),
    rgb6(0x3f,0x3f,0x3f),rgb6(0x3f,0x3f,0x3f),rgb6(0x3f,0x3f,0x3f),rgb6(0x3f,0x3f,0x3f),
]}

pub fn mtext_s3_palette() -> [ColorSpace; 64] {[
    rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),
    rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),
    rgb6(0x2a,0x2a,0x2a),rgb6(0x2a,0x2a,0x2a),rgb6(0x2a,0x2a,0x2a),rgb6(0x2a,0x2a,0x2a),
    rgb6(0x2a,0x2a,0x2a),rgb6(0x2a,0x2a,0x2a),rgb6(0x2a,0x2a,0x2a),rgb6(0x2a,0x2a,0x2a),
    rgb6(0x2a,0x2a,0x2a),rgb6(0x2a,0x2a,0x2a),rgb6(0x2a,0x2a,0x2a),rgb6(0x2a,0x2a,0x2a),
    rgb6(0x2a,0x2a,0x2a),rgb6(0x2a,0x2a,0x2a),rgb6(0x2a,0x2a,0x2a),rgb6(0x2a,0x2a,0x2a),
    rgb6(0x3f,0x3f,0x3f),rgb6(0x3f,0x3f,0x3f),rgb6(0x3f,0x3f,0x3f),rgb6(0x3f,0x3f,0x3f),
    rgb6(0x3f,0x3f,0x3f),rgb6(0x3f,0x3f,0x3f),rgb6(0x3f,0x3f,0x3f),rgb6(0x3f,0x3f,0x3f),
    rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),
    rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),
    rgb6(0x2a,0x2a,0x2a),rgb6(0x2a,0x2a,0x2a),rgb6(0x2a,0x2a,0x2a),rgb6(0x2a,0x2a,0x2a),
    rgb6(0x2a,0x2a,0x2a),rgb6(0x2a,0x2a,0x2a),rgb6(0x2a,0x2a,0x2a),rgb6(0x2a,0x2a,0x2a),
    rgb6(0x2a,0x2a,0x2a),rgb6(0x2a,0x2a,0x2a),rgb6(0x2a,0x2a,0x2a),rgb6(0x2a,0x2a,0x2a),
    rgb6(0x2a,0x2a,0x2a),rgb6(0x2a,0x2a,0x2a),rgb6(0x2a,0x2a,0x2a),rgb6(0x2a,0x2a,0x2a),
    rgb6(0x3f,0x3f,0x3f),rgb6(0x3f,0x3f,0x3f),rgb6(0x3f,0x3f,0x3f),rgb6(0x3f,0x3f,0x3f),
    rgb6(0x3f,0x3f,0x3f),rgb6(0x3f,0x3f,0x3f),rgb6(0x3f,0x3f,0x3f),rgb6(0x3f,0x3f,0x3f),
]}

pub fn cga_palette() -> [ColorSpace; 16] {[
    rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x2a),rgb6(0x00,0x2a,0x00),rgb6(0x00,0x2a,0x2a),
    rgb6(0x2a,0x00,0x00),rgb6(0x2a,0x00,0x2a),rgb6(0x2a,0x15,0x00),rgb6(0x2a,0x2a,0x2a),
    rgb6(0x15,0x15,0x15),rgb6(0x15,0x15,0x3f),rgb6(0x15,0x3f,0x15),rgb6(0x15,0x3f,0x3f),
    rgb6(0x3f,0x15,0x15),rgb6(0x3f,0x15,0x3f),rgb6(0x3f,0x3f,0x15),rgb6(0x3f,0x3f,0x3f),
]}

pub fn cga_palette_2() -> [ColorSpace; 64] {[
    rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x2a),rgb6(0x00,0x2a,0x00),rgb6(0x00,0x2a,0x2a),
    rgb6(0x2a,0x00,0x00),rgb6(0x2a,0x00,0x2a),rgb6(0x2a,0x15,0x00),rgb6(0x2a,0x2a,0x2a),
    rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x2a),rgb6(0x00,0x2a,0x00),rgb6(0x00,0x2a,0x2a),
    rgb6(0x2a,0x00,0x00),rgb6(0x2a,0x00,0x2a),rgb6(0x2a,0x15,0x00),rgb6(0x2a,0x2a,0x2a),
    rgb6(0x15,0x15,0x15),rgb6(0x15,0x15,0x3f),rgb6(0x15,0x3f,0x15),rgb6(0x15,0x3f,0x3f),
    rgb6(0x3f,0x15,0x15),rgb6(0x3f,0x15,0x3f),rgb6(0x3f,0x3f,0x15),rgb6(0x3f,0x3f,0x3f),
    rgb6(0x15,0x15,0x15),rgb6(0x15,0x15,0x3f),rgb6(0x15,0x3f,0x15),rgb6(0x15,0x3f,0x3f),
    rgb6(0x3f,0x15,0x15),rgb6(0x3f,0x15,0x3f),rgb6(0x3f,0x3f,0x15),rgb6(0x3f,0x3f,0x3f),
    rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x2a),rgb6(0x00,0x2a,0x00),rgb6(0x00,0x2a,0x2a),
    rgb6(0x2a,0x00,0x00),rgb6(0x2a,0x00,0x2a),rgb6(0x2a,0x15,0x00),rgb6(0x2a,0x2a,0x2a),
    rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x2a),rgb6(0x00,0x2a,0x00),rgb6(0x00,0x2a,0x2a),
    rgb6(0x2a,0x00,0x00),rgb6(0x2a,0x00,0x2a),rgb6(0x2a,0x15,0x00),rgb6(0x2a,0x2a,0x2a),
    rgb6(0x15,0x15,0x15),rgb6(0x15,0x15,0x3f),rgb6(0x15,0x3f,0x15),rgb6(0x15,0x3f,0x3f),
    rgb6(0x3f,0x15,0x15),rgb6(0x3f,0x15,0x3f),rgb6(0x3f,0x3f,0x15),rgb6(0x3f,0x3f,0x3f),
    rgb6(0x15,0x15,0x15),rgb6(0x15,0x15,0x3f),rgb6(0x15,0x3f,0x15),rgb6(0x15,0x3f,0x3f),
    rgb6(0x3f,0x15,0x15),rgb6(0x3f,0x15,0x3f),rgb6(0x3f,0x3f,0x15),rgb6(0x3f,0x3f,0x3f),
]}

pub fn ega_palette() -> [ColorSpace; 64] {[
    rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x2a),rgb6(0x00,0x2a,0x00),rgb6(0x00,0x2a,0x2a),
    rgb6(0x2a,0x00,0x00),rgb6(0x2a,0x00,0x2a),rgb6(0x2a,0x15,0x00),rgb6(0x2a,0x2a,0x2a),
    rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x2a),rgb6(0x00,0x2a,0x00),rgb6(0x00,0x2a,0x2a),
    rgb6(0x2a,0x00,0x00),rgb6(0x2a,0x00,0x2a),rgb6(0x2a,0x15,0x00),rgb6(0x2a,0x2a,0x2a),
    rgb6(0x15,0x15,0x15),rgb6(0x15,0x15,0x3f),rgb6(0x15,0x3f,0x15),rgb6(0x15,0x3f,0x3f),
    rgb6(0x3f,0x15,0x15),rgb6(0x3f,0x15,0x3f),rgb6(0x3f,0x3f,0x15),rgb6(0x3f,0x3f,0x3f),
    rgb6(0x15,0x15,0x15),rgb6(0x15,0x15,0x3f),rgb6(0x15,0x3f,0x15),rgb6(0x15,0x3f,0x3f),
    rgb6(0x3f,0x15,0x15),rgb6(0x3f,0x15,0x3f),rgb6(0x3f,0x3f,0x15),rgb6(0x3f,0x3f,0x3f),
    rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x2a),rgb6(0x00,0x2a,0x00),rgb6(0x00,0x2a,0x2a),
    rgb6(0x2a,0x00,0x00),rgb6(0x2a,0x00,0x2a),rgb6(0x2a,0x15,0x00),rgb6(0x2a,0x2a,0x2a),
    rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x2a),rgb6(0x00,0x2a,0x00),rgb6(0x00,0x2a,0x2a),
    rgb6(0x2a,0x00,0x00),rgb6(0x2a,0x00,0x2a),rgb6(0x2a,0x15,0x00),rgb6(0x2a,0x2a,0x2a),
    rgb6(0x15,0x15,0x15),rgb6(0x15,0x15,0x3f),rgb6(0x15,0x3f,0x15),rgb6(0x15,0x3f,0x3f),
    rgb6(0x3f,0x15,0x15),rgb6(0x3f,0x15,0x3f),rgb6(0x3f,0x3f,0x15),rgb6(0x3f,0x3f,0x3f),
    rgb6(0x15,0x15,0x15),rgb6(0x15,0x15,0x3f),rgb6(0x15,0x3f,0x15),rgb6(0x15,0x3f,0x3f),
    rgb6(0x3f,0x15,0x15),rgb6(0x3f,0x15,0x3f),rgb6(0x3f,0x3f,0x15),rgb6(0x3f,0x3f,0x3f),
]}

pub fn vga_palette() -> [ColorSpace; 256] {[
    rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x2a),rgb6(0x00,0x2a,0x00),rgb6(0x00,0x2a,0x2a),
    rgb6(0x2a,0x00,0x00),rgb6(0x2a,0x00,0x2a),rgb6(0x2a,0x15,0x00),rgb6(0x2a,0x2a,0x2a),
    rgb6(0x15,0x15,0x15),rgb6(0x15,0x15,0x3f),rgb6(0x15,0x3f,0x15),rgb6(0x15,0x3f,0x3f),
    rgb6(0x3f,0x15,0x15),rgb6(0x3f,0x15,0x3f),rgb6(0x3f,0x3f,0x15),rgb6(0x3f,0x3f,0x3f),
    rgb6(0x00,0x00,0x00),rgb6(0x05,0x05,0x05),rgb6(0x08,0x08,0x08),rgb6(0x0b,0x0b,0x0b),
    rgb6(0x0e,0x0e,0x0e),rgb6(0x11,0x11,0x11),rgb6(0x14,0x14,0x14),rgb6(0x18,0x18,0x18),
    rgb6(0x1c,0x1c,0x1c),rgb6(0x20,0x20,0x20),rgb6(0x24,0x24,0x24),rgb6(0x28,0x28,0x28),
    rgb6(0x2d,0x2d,0x2d),rgb6(0x32,0x32,0x32),rgb6(0x38,0x38,0x38),rgb6(0x3f,0x3f,0x3f),
    rgb6(0x00,0x00,0x3f),rgb6(0x10,0x00,0x3f),rgb6(0x1f,0x00,0x3f),rgb6(0x2f,0x00,0x3f),
    rgb6(0x3f,0x00,0x3f),rgb6(0x3f,0x00,0x2f),rgb6(0x3f,0x00,0x1f),rgb6(0x3f,0x00,0x10),
    rgb6(0x3f,0x00,0x00),rgb6(0x3f,0x10,0x00),rgb6(0x3f,0x1f,0x00),rgb6(0x3f,0x2f,0x00),
    rgb6(0x3f,0x3f,0x00),rgb6(0x2f,0x3f,0x00),rgb6(0x1f,0x3f,0x00),rgb6(0x10,0x3f,0x00),
    rgb6(0x00,0x3f,0x00),rgb6(0x00,0x3f,0x10),rgb6(0x00,0x3f,0x1f),rgb6(0x00,0x3f,0x2f),
    rgb6(0x00,0x3f,0x3f),rgb6(0x00,0x2f,0x3f),rgb6(0x00,0x1f,0x3f),rgb6(0x00,0x10,0x3f),
    rgb6(0x1f,0x1f,0x3f),rgb6(0x27,0x1f,0x3f),rgb6(0x2f,0x1f,0x3f),rgb6(0x37,0x1f,0x3f),
    rgb6(0x3f,0x1f,0x3f),rgb6(0x3f,0x1f,0x37),rgb6(0x3f,0x1f,0x2f),rgb6(0x3f,0x1f,0x27),

    rgb6(0x3f,0x1f,0x1f),rgb6(0x3f,0x27,0x1f),rgb6(0x3f,0x2f,0x1f),rgb6(0x3f,0x37,0x1f),
    rgb6(0x3f,0x3f,0x1f),rgb6(0x37,0x3f,0x1f),rgb6(0x2f,0x3f,0x1f),rgb6(0x27,0x3f,0x1f),
    rgb6(0x1f,0x3f,0x1f),rgb6(0x1f,0x3f,0x27),rgb6(0x1f,0x3f,0x2f),rgb6(0x1f,0x3f,0x37),
    rgb6(0x1f,0x3f,0x3f),rgb6(0x1f,0x37,0x3f),rgb6(0x1f,0x2f,0x3f),rgb6(0x1f,0x27,0x3f),
    rgb6(0x2d,0x2d,0x3f),rgb6(0x31,0x2d,0x3f),rgb6(0x36,0x2d,0x3f),rgb6(0x3a,0x2d,0x3f),
    rgb6(0x3f,0x2d,0x3f),rgb6(0x3f,0x2d,0x3a),rgb6(0x3f,0x2d,0x36),rgb6(0x3f,0x2d,0x31),
    rgb6(0x3f,0x2d,0x2d),rgb6(0x3f,0x31,0x2d),rgb6(0x3f,0x36,0x2d),rgb6(0x3f,0x3a,0x2d),
    rgb6(0x3f,0x3f,0x2d),rgb6(0x3a,0x3f,0x2d),rgb6(0x36,0x3f,0x2d),rgb6(0x31,0x3f,0x2d),
    rgb6(0x2d,0x3f,0x2d),rgb6(0x2d,0x3f,0x31),rgb6(0x2d,0x3f,0x36),rgb6(0x2d,0x3f,0x3a),
    rgb6(0x2d,0x3f,0x3f),rgb6(0x2d,0x3a,0x3f),rgb6(0x2d,0x36,0x3f),rgb6(0x2d,0x31,0x3f),
    rgb6(0x00,0x00,0x1c),rgb6(0x07,0x00,0x1c),rgb6(0x0e,0x00,0x1c),rgb6(0x15,0x00,0x1c),
    rgb6(0x1c,0x00,0x1c),rgb6(0x1c,0x00,0x15),rgb6(0x1c,0x00,0x0e),rgb6(0x1c,0x00,0x07),
    rgb6(0x1c,0x00,0x00),rgb6(0x1c,0x07,0x00),rgb6(0x1c,0x0e,0x00),rgb6(0x1c,0x15,0x00),
    rgb6(0x1c,0x1c,0x00),rgb6(0x15,0x1c,0x00),rgb6(0x0e,0x1c,0x00),rgb6(0x07,0x1c,0x00),
    rgb6(0x00,0x1c,0x00),rgb6(0x00,0x1c,0x07),rgb6(0x00,0x1c,0x0e),rgb6(0x00,0x1c,0x15),
    rgb6(0x00,0x1c,0x1c),rgb6(0x00,0x15,0x1c),rgb6(0x00,0x0e,0x1c),rgb6(0x00,0x07,0x1c),

    rgb6(0x0e,0x0e,0x1c),rgb6(0x11,0x0e,0x1c),rgb6(0x15,0x0e,0x1c),rgb6(0x18,0x0e,0x1c),
    rgb6(0x1c,0x0e,0x1c),rgb6(0x1c,0x0e,0x18),rgb6(0x1c,0x0e,0x15),rgb6(0x1c,0x0e,0x11),
    rgb6(0x1c,0x0e,0x0e),rgb6(0x1c,0x11,0x0e),rgb6(0x1c,0x15,0x0e),rgb6(0x1c,0x18,0x0e),
    rgb6(0x1c,0x1c,0x0e),rgb6(0x18,0x1c,0x0e),rgb6(0x15,0x1c,0x0e),rgb6(0x11,0x1c,0x0e),
    rgb6(0x0e,0x1c,0x0e),rgb6(0x0e,0x1c,0x11),rgb6(0x0e,0x1c,0x15),rgb6(0x0e,0x1c,0x18),
    rgb6(0x0e,0x1c,0x1c),rgb6(0x0e,0x18,0x1c),rgb6(0x0e,0x15,0x1c),rgb6(0x0e,0x11,0x1c),
    rgb6(0x14,0x14,0x1c),rgb6(0x16,0x14,0x1c),rgb6(0x18,0x14,0x1c),rgb6(0x1a,0x14,0x1c),
    rgb6(0x1c,0x14,0x1c),rgb6(0x1c,0x14,0x1a),rgb6(0x1c,0x14,0x18),rgb6(0x1c,0x14,0x16),
    rgb6(0x1c,0x14,0x14),rgb6(0x1c,0x16,0x14),rgb6(0x1c,0x18,0x14),rgb6(0x1c,0x1a,0x14),
    rgb6(0x1c,0x1c,0x14),rgb6(0x1a,0x1c,0x14),rgb6(0x18,0x1c,0x14),rgb6(0x16,0x1c,0x14),
    rgb6(0x14,0x1c,0x14),rgb6(0x14,0x1c,0x16),rgb6(0x14,0x1c,0x18),rgb6(0x14,0x1c,0x1a),
    rgb6(0x14,0x1c,0x1c),rgb6(0x14,0x1a,0x1c),rgb6(0x14,0x18,0x1c),rgb6(0x14,0x16,0x1c),
    rgb6(0x00,0x00,0x10),rgb6(0x04,0x00,0x10),rgb6(0x08,0x00,0x10),rgb6(0x0c,0x00,0x10),
    rgb6(0x10,0x00,0x10),rgb6(0x10,0x00,0x0c),rgb6(0x10,0x00,0x08),rgb6(0x10,0x00,0x04),
    rgb6(0x10,0x00,0x00),rgb6(0x10,0x04,0x00),rgb6(0x10,0x08,0x00),rgb6(0x10,0x0c,0x00),
    rgb6(0x10,0x10,0x00),rgb6(0x0c,0x10,0x00),rgb6(0x08,0x10,0x00),rgb6(0x04,0x10,0x00),

    rgb6(0x00,0x10,0x00),rgb6(0x00,0x10,0x04),rgb6(0x00,0x10,0x08),rgb6(0x00,0x10,0x0c),
    rgb6(0x00,0x10,0x10),rgb6(0x00,0x0c,0x10),rgb6(0x00,0x08,0x10),rgb6(0x00,0x04,0x10),
    rgb6(0x08,0x08,0x10),rgb6(0x0a,0x08,0x10),rgb6(0x0c,0x08,0x10),rgb6(0x0e,0x08,0x10),
    rgb6(0x10,0x08,0x10),rgb6(0x10,0x08,0x0e),rgb6(0x10,0x08,0x0c),rgb6(0x10,0x08,0x0a),
    rgb6(0x10,0x08,0x08),rgb6(0x10,0x0a,0x08),rgb6(0x10,0x0c,0x08),rgb6(0x10,0x0e,0x08),
    rgb6(0x10,0x10,0x08),rgb6(0x0e,0x10,0x08),rgb6(0x0c,0x10,0x08),rgb6(0x0a,0x10,0x08),
    rgb6(0x08,0x10,0x08),rgb6(0x08,0x10,0x0a),rgb6(0x08,0x10,0x0c),rgb6(0x08,0x10,0x0e),
    rgb6(0x08,0x10,0x10),rgb6(0x08,0x0e,0x10),rgb6(0x08,0x0c,0x10),rgb6(0x08,0x0a,0x10),
    rgb6(0x0b,0x0b,0x10),rgb6(0x0c,0x0b,0x10),rgb6(0x0d,0x0b,0x10),rgb6(0x0f,0x0b,0x10),
    rgb6(0x10,0x0b,0x10),rgb6(0x10,0x0b,0x0f),rgb6(0x10,0x0b,0x0d),rgb6(0x10,0x0b,0x0c),
    rgb6(0x10,0x0b,0x0b),rgb6(0x10,0x0c,0x0b),rgb6(0x10,0x0d,0x0b),rgb6(0x10,0x0f,0x0b),
    rgb6(0x10,0x10,0x0b),rgb6(0x0f,0x10,0x0b),rgb6(0x0d,0x10,0x0b),rgb6(0x0c,0x10,0x0b),
    rgb6(0x0b,0x10,0x0b),rgb6(0x0b,0x10,0x0c),rgb6(0x0b,0x10,0x0d),rgb6(0x0b,0x10,0x0f),
    rgb6(0x0b,0x10,0x10),rgb6(0x0b,0x0f,0x10),rgb6(0x0b,0x0d,0x10),rgb6(0x0b,0x0c,0x10),

     // 248-255: all black
    rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),
    rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),rgb6(0x00,0x00,0x00),
]}