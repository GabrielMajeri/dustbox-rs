#[derive(Clone, Default)]
pub struct DACPalette {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub fn default_vga_palette() -> Vec<DACPalette> {
    let pal = [
        // 0-15: EGA palette
        DACPalette {r: 0x00, g: 0x00, b: 0x00}, // 0. black
        DACPalette {r: 0x00, g: 0x00, b: 0xAA}, // 1. blue
        DACPalette {r: 0x00, g: 0xAA, b: 0x00}, // 2. green
        DACPalette {r: 0x00, g: 0xAA, b: 0xAA}, // 3. cyan
        DACPalette {r: 0xAA, g: 0x00, b: 0x00}, // 4. red
        DACPalette {r: 0xAA, g: 0x00, b: 0xAA}, // 5. magenta
        DACPalette {r: 0xAA, g: 0x55, b: 0x00}, // 6. brown
        DACPalette {r: 0xAA, g: 0xAA, b: 0xAA}, // 7. gray
        DACPalette {r: 0x55, g: 0x55, b: 0x55}, // 8. dark gray
        DACPalette {r: 0x55, g: 0x55, b: 0xFF}, // 9. bright blue
        DACPalette {r: 0x55, g: 0xFF, b: 0x55}, // 10. bright green
        DACPalette {r: 0x55, g: 0xFF, b: 0xFF}, // 11. bright cyan
        DACPalette {r: 0xFF, g: 0x55, b: 0x55}, // 12. bright red
        DACPalette {r: 0xFF, g: 0x55, b: 0xFF}, // 13. bright magenta
        DACPalette {r: 0xFF, g: 0xFF, b: 0x55}, // 14. yellow
        DACPalette {r: 0xFF, g: 0xFF, b: 0xFF}, // 15. white

        // 16-31: gray scale
        DACPalette {r: 0x00, g: 0x00, b: 0x00}, // 16. black
        DACPalette {r: 0x14, g: 0x14, b: 0x14},
        DACPalette {r: 0x20, g: 0x20, b: 0x20},
        DACPalette {r: 0x2C, g: 0x2C, b: 0x2C},
        DACPalette {r: 0x38, g: 0x38, b: 0x38},
        DACPalette {r: 0x45, g: 0x45, b: 0x45},
        DACPalette {r: 0x51, g: 0x51, b: 0x51},
        DACPalette {r: 0x61, g: 0x61, b: 0x61},
        DACPalette {r: 0x71, g: 0x71, b: 0x71},
        DACPalette {r: 0x82, g: 0x82, b: 0x82},
        DACPalette {r: 0x92, g: 0x92, b: 0x92},
        DACPalette {r: 0xA2, g: 0xA2, b: 0xA2},
        DACPalette {r: 0xB6, g: 0xB6, b: 0xB6},
        DACPalette {r: 0xCB, g: 0xCB, b: 0xCB},
        DACPalette {r: 0xE3, g: 0xE3, b: 0xE3},
        DACPalette {r: 0xFF, g: 0xFF, b: 0xFF}, // 31. white

        // 32-55: rainbow
        DACPalette {r: 0x00, g: 0x00, b: 0xFF}, // 32. blue
        DACPalette {r: 0x41, g: 0x00, b: 0xFF},
        DACPalette {r: 0x7D, g: 0x00, b: 0xFF},
        DACPalette {r: 0xBE, g: 0x00, b: 0xFF},
        DACPalette {r: 0xFF, g: 0x00, b: 0xFF}, // 36. magenta
        DACPalette {r: 0xFF, g: 0x00, b: 0xBE},
        DACPalette {r: 0xFF, g: 0x00, b: 0x7D},
        DACPalette {r: 0xFF, g: 0x00, b: 0x41},
        DACPalette {r: 0xFF, g: 0x00, b: 0x00}, // 40. red
        DACPalette {r: 0xFF, g: 0x41, b: 0x00},
        DACPalette {r: 0xFF, g: 0x7D, b: 0x00},
        DACPalette {r: 0xFF, g: 0xBE, b: 0x00},
        DACPalette {r: 0xFF, g: 0xFF, b: 0x00}, // 44. yellow
        DACPalette {r: 0xBE, g: 0xFF, b: 0x00},
        DACPalette {r: 0x7D, g: 0xFF, b: 0x00},
        DACPalette {r: 0x41, g: 0xFF, b: 0x00},
        DACPalette {r: 0x00, g: 0xFF, b: 0x00}, // 48. green
        DACPalette {r: 0x00, g: 0xFF, b: 0x41},
        DACPalette {r: 0x00, g: 0xFF, b: 0x7D},
        DACPalette {r: 0x00, g: 0xFF, b: 0xBE},
        DACPalette {r: 0x00, g: 0xFF, b: 0xFF}, // 52. cyan
        DACPalette {r: 0x00, g: 0xBE, b: 0xFF},
        DACPalette {r: 0x00, g: 0x7D, b: 0xFF},
        DACPalette {r: 0x00, g: 0x41, b: 0xFF},

        // 56-79: 49% whitemix of 32-55
        DACPalette {r: 0x7D, g: 0x7D, b: 0xFF}, // 56. blue
        DACPalette {r: 0x9E, g: 0x7D, b: 0xFF},
        DACPalette {r: 0xBE, g: 0x7D, b: 0xFF},
        DACPalette {r: 0xDF, g: 0x7D, b: 0xFF},
        DACPalette {r: 0xFF, g: 0x7D, b: 0xFF}, // 60. magenta
        DACPalette {r: 0xFF, g: 0x7D, b: 0xDF},
        DACPalette {r: 0xFF, g: 0x7D, b: 0xBE},
        DACPalette {r: 0xFF, g: 0x7D, b: 0x9E},
        DACPalette {r: 0xFF, g: 0x7D, b: 0x7D}, // 64. red
        DACPalette {r: 0xFF, g: 0x9E, b: 0x7D},
        DACPalette {r: 0xFF, g: 0xBE, b: 0x7D},
        DACPalette {r: 0xFF, g: 0xDF, b: 0x7D},
        DACPalette {r: 0xFF, g: 0xFF, b: 0x7D}, // 68. yellow
        DACPalette {r: 0xDF, g: 0xFF, b: 0x7D},
        DACPalette {r: 0xBE, g: 0xFF, b: 0x7D},
        DACPalette {r: 0x9E, g: 0xFF, b: 0x7D},
        DACPalette {r: 0x7D, g: 0xFF, b: 0x7D}, // 72. green
        DACPalette {r: 0x7D, g: 0xFF, b: 0x9E},
        DACPalette {r: 0x7D, g: 0xFF, b: 0xBE},
        DACPalette {r: 0x7D, g: 0xFF, b: 0xDF},
        DACPalette {r: 0x7D, g: 0xFF, b: 0xFF}, // 76. cyan
        DACPalette {r: 0x7D, g: 0xDF, b: 0xFF},
        DACPalette {r: 0x7D, g: 0xBE, b: 0xFF},
        DACPalette {r: 0x7D, g: 0x9E, b: 0xFF},

        // 80-103: 72% whitemix of 32-55
        DACPalette {r: 0xB6, g: 0xB6, b: 0xFF}, // 80. blue
        DACPalette {r: 0xC7, g: 0xB6, b: 0xFF},
        DACPalette {r: 0xDB, g: 0xB6, b: 0xFF},
        DACPalette {r: 0xEB, g: 0xB6, b: 0xFF},
        DACPalette {r: 0xFF, g: 0xB6, b: 0xFF}, // 84. magenta
        DACPalette {r: 0xFF, g: 0xB6, b: 0xEB},
        DACPalette {r: 0xFF, g: 0xB6, b: 0xDB},
        DACPalette {r: 0xFF, g: 0xB6, b: 0xC7},
        DACPalette {r: 0xFF, g: 0xB6, b: 0xB6}, // 88. red
        DACPalette {r: 0xFF, g: 0xC7, b: 0xB6},
        DACPalette {r: 0xFF, g: 0xDB, b: 0xB6},
        DACPalette {r: 0xFF, g: 0xEB, b: 0xB6},
        DACPalette {r: 0xFF, g: 0xFF, b: 0xB6}, // 92. yellow
        DACPalette {r: 0xEB, g: 0xFF, b: 0xB6},
        DACPalette {r: 0xDB, g: 0xFF, b: 0xB6},
        DACPalette {r: 0xC7, g: 0xFF, b: 0xB6},
        DACPalette {r: 0xB6, g: 0xFF, b: 0xB6}, // 96. green
        DACPalette {r: 0xB6, g: 0xFF, b: 0xC7},
        DACPalette {r: 0xB6, g: 0xFF, b: 0xDB},
        DACPalette {r: 0xB6, g: 0xFF, b: 0xEB},
        DACPalette {r: 0xB6, g: 0xFF, b: 0xFF}, // 100. cyan
        DACPalette {r: 0xB6, g: 0xEB, b: 0xFF},
        DACPalette {r: 0xB6, g: 0xDB, b: 0xFF},
        DACPalette {r: 0xB6, g: 0xC7, b: 0xFF},

        // 104-175: 56% blackmix of 32-103
        DACPalette {r: 0x00, g: 0x00, b: 0x71}, // 104
        DACPalette {r: 0x1C, g: 0x00, b: 0x71},
        DACPalette {r: 0x38, g: 0x00, b: 0x71},
        DACPalette {r: 0x55, g: 0x00, b: 0x71},
        DACPalette {r: 0x71, g: 0x00, b: 0x71}, // 108
        DACPalette {r: 0x71, g: 0x00, b: 0x55},
        DACPalette {r: 0x71, g: 0x00, b: 0x38},
        DACPalette {r: 0x71, g: 0x00, b: 0x1C},
        DACPalette {r: 0x71, g: 0x00, b: 0x00}, // 112
        DACPalette {r: 0x71, g: 0x1C, b: 0x00},
        DACPalette {r: 0x71, g: 0x38, b: 0x00},
        DACPalette {r: 0x71, g: 0x55, b: 0x00},
        DACPalette {r: 0x71, g: 0x71, b: 0x00}, // 116
        DACPalette {r: 0x55, g: 0x71, b: 0x00},
        DACPalette {r: 0x38, g: 0x71, b: 0x00},
        DACPalette {r: 0x1C, g: 0x71, b: 0x00},
        DACPalette {r: 0x00, g: 0x71, b: 0x00}, // 120
        DACPalette {r: 0x00, g: 0x71, b: 0x1C},
        DACPalette {r: 0x00, g: 0x71, b: 0x38},
        DACPalette {r: 0x00, g: 0x71, b: 0x55},
        DACPalette {r: 0x00, g: 0x71, b: 0x71}, // 124
        DACPalette {r: 0x00, g: 0x55, b: 0x71},
        DACPalette {r: 0x00, g: 0x38, b: 0x71},
        DACPalette {r: 0x00, g: 0x1C, b: 0x71},

        DACPalette {r: 0x38, g: 0x38, b: 0x71}, // 128
        DACPalette {r: 0x45, g: 0x38, b: 0x71},
        DACPalette {r: 0x55, g: 0x38, b: 0x71},
        DACPalette {r: 0x61, g: 0x38, b: 0x71},
        DACPalette {r: 0x71, g: 0x38, b: 0x71}, // 132
        DACPalette {r: 0x71, g: 0x38, b: 0x61},
        DACPalette {r: 0x71, g: 0x38, b: 0x55},
        DACPalette {r: 0x71, g: 0x38, b: 0x45},
        DACPalette {r: 0x71, g: 0x38, b: 0x38}, // 136
        DACPalette {r: 0x71, g: 0x45, b: 0x38},
        DACPalette {r: 0x71, g: 0x55, b: 0x38},
        DACPalette {r: 0x71, g: 0x61, b: 0x38},
        DACPalette {r: 0x71, g: 0x71, b: 0x38}, // 140
        DACPalette {r: 0x61, g: 0x71, b: 0x38},
        DACPalette {r: 0x55, g: 0x71, b: 0x38},
        DACPalette {r: 0x45, g: 0x71, b: 0x38},
        DACPalette {r: 0x38, g: 0x71, b: 0x38}, // 144
        DACPalette {r: 0x38, g: 0x71, b: 0x45},
        DACPalette {r: 0x38, g: 0x71, b: 0x55},
        DACPalette {r: 0x38, g: 0x71, b: 0x61},
        DACPalette {r: 0x38, g: 0x71, b: 0x71}, // 148
        DACPalette {r: 0x38, g: 0x61, b: 0x71},
        DACPalette {r: 0x38, g: 0x55, b: 0x71},
        DACPalette {r: 0x38, g: 0x45, b: 0x71},

        DACPalette {r: 0x51, g: 0x51, b: 0x71}, // 152
        DACPalette {r: 0x59, g: 0x51, b: 0x71},
        DACPalette {r: 0x61, g: 0x51, b: 0x71},
        DACPalette {r: 0x69, g: 0x51, b: 0x71},
        DACPalette {r: 0x71, g: 0x51, b: 0x71}, // 156
        DACPalette {r: 0x71, g: 0x51, b: 0x69},
        DACPalette {r: 0x71, g: 0x51, b: 0x61},
        DACPalette {r: 0x71, g: 0x51, b: 0x59},
        DACPalette {r: 0x71, g: 0x51, b: 0x51}, // 160
        DACPalette {r: 0x71, g: 0x59, b: 0x51},
        DACPalette {r: 0x71, g: 0x61, b: 0x51},
        DACPalette {r: 0x71, g: 0x69, b: 0x51},
        DACPalette {r: 0x71, g: 0x71, b: 0x51}, // 164
        DACPalette {r: 0x69, g: 0x71, b: 0x51},
        DACPalette {r: 0x61, g: 0x71, b: 0x51},
        DACPalette {r: 0x59, g: 0x71, b: 0x51},
        DACPalette {r: 0x51, g: 0x71, b: 0x51}, // 168
        DACPalette {r: 0x51, g: 0x71, b: 0x59},
        DACPalette {r: 0x51, g: 0x71, b: 0x61},
        DACPalette {r: 0x51, g: 0x71, b: 0x69},
        DACPalette {r: 0x51, g: 0x71, b: 0x71}, // 172
        DACPalette {r: 0x51, g: 0x69, b: 0x71},
        DACPalette {r: 0x51, g: 0x61, b: 0x71},
        DACPalette {r: 0x51, g: 0x59, b: 0x71},

        // 176-247: 75% blackmix of 32-103
        DACPalette {r: 0x00, g: 0x00, b: 0x41}, // 176
        DACPalette {r: 0x10, g: 0x00, b: 0x41},
        DACPalette {r: 0x20, g: 0x00, b: 0x41},
        DACPalette {r: 0x30, g: 0x00, b: 0x41},
        DACPalette {r: 0x41, g: 0x00, b: 0x41}, // 180
        DACPalette {r: 0x41, g: 0x00, b: 0x30},
        DACPalette {r: 0x41, g: 0x00, b: 0x20},
        DACPalette {r: 0x41, g: 0x00, b: 0x10},
        DACPalette {r: 0x41, g: 0x00, b: 0x00}, // 184
        DACPalette {r: 0x41, g: 0x10, b: 0x00},
        DACPalette {r: 0x41, g: 0x20, b: 0x00},
        DACPalette {r: 0x41, g: 0x30, b: 0x00},
        DACPalette {r: 0x41, g: 0x41, b: 0x00}, // 188
        DACPalette {r: 0x30, g: 0x41, b: 0x00},
        DACPalette {r: 0x20, g: 0x41, b: 0x00},
        DACPalette {r: 0x10, g: 0x41, b: 0x00},
        DACPalette {r: 0x00, g: 0x41, b: 0x00}, // 192
        DACPalette {r: 0x00, g: 0x41, b: 0x10},
        DACPalette {r: 0x00, g: 0x41, b: 0x20},
        DACPalette {r: 0x00, g: 0x41, b: 0x30},
        DACPalette {r: 0x00, g: 0x41, b: 0x41}, // 196
        DACPalette {r: 0x00, g: 0x30, b: 0x41},
        DACPalette {r: 0x00, g: 0x20, b: 0x41},
        DACPalette {r: 0x00, g: 0x10, b: 0x41},

        DACPalette {r: 0x20, g: 0x20, b: 0x41}, // 200
        DACPalette {r: 0x28, g: 0x20, b: 0x41},
        DACPalette {r: 0x30, g: 0x20, b: 0x41},
        DACPalette {r: 0x38, g: 0x20, b: 0x41},
        DACPalette {r: 0x41, g: 0x20, b: 0x41}, // 204
        DACPalette {r: 0x41, g: 0x20, b: 0x38},
        DACPalette {r: 0x41, g: 0x20, b: 0x30},
        DACPalette {r: 0x41, g: 0x20, b: 0x28},
        DACPalette {r: 0x41, g: 0x20, b: 0x20}, // 208
        DACPalette {r: 0x41, g: 0x28, b: 0x20},
        DACPalette {r: 0x41, g: 0x30, b: 0x20},
        DACPalette {r: 0x41, g: 0x38, b: 0x20},
        DACPalette {r: 0x41, g: 0x41, b: 0x20}, // 212
        DACPalette {r: 0x38, g: 0x41, b: 0x20},
        DACPalette {r: 0x30, g: 0x41, b: 0x20},
        DACPalette {r: 0x28, g: 0x41, b: 0x20},
        DACPalette {r: 0x20, g: 0x41, b: 0x20}, // 216
        DACPalette {r: 0x20, g: 0x41, b: 0x28},
        DACPalette {r: 0x20, g: 0x41, b: 0x30},
        DACPalette {r: 0x20, g: 0x41, b: 0x38},
        DACPalette {r: 0x20, g: 0x41, b: 0x41}, // 220
        DACPalette {r: 0x20, g: 0x38, b: 0x41},
        DACPalette {r: 0x20, g: 0x30, b: 0x41},
        DACPalette {r: 0x20, g: 0x28, b: 0x41},

        DACPalette {r: 0x2C, g: 0x2C, b: 0x41}, // 224
        DACPalette {r: 0x30, g: 0x2C, b: 0x41},
        DACPalette {r: 0x34, g: 0x2C, b: 0x41},
        DACPalette {r: 0x3C, g: 0x2C, b: 0x41},
        DACPalette {r: 0x41, g: 0x2C, b: 0x41}, // 228
        DACPalette {r: 0x41, g: 0x2C, b: 0x3C},
        DACPalette {r: 0x41, g: 0x2C, b: 0x34},
        DACPalette {r: 0x41, g: 0x2C, b: 0x30},
        DACPalette {r: 0x41, g: 0x2C, b: 0x2C}, // 232
        DACPalette {r: 0x41, g: 0x30, b: 0x2C},
        DACPalette {r: 0x41, g: 0x34, b: 0x2C},
        DACPalette {r: 0x41, g: 0x3C, b: 0x2C},
        DACPalette {r: 0x41, g: 0x41, b: 0x2C}, // 236
        DACPalette {r: 0x3C, g: 0x41, b: 0x2C},
        DACPalette {r: 0x34, g: 0x41, b: 0x2C},
        DACPalette {r: 0x30, g: 0x41, b: 0x2C},
        DACPalette {r: 0x2C, g: 0x41, b: 0x2C}, // 240
        DACPalette {r: 0x2C, g: 0x41, b: 0x30},
        DACPalette {r: 0x2C, g: 0x41, b: 0x34},
        DACPalette {r: 0x2C, g: 0x41, b: 0x3C},
        DACPalette {r: 0x2C, g: 0x41, b: 0x41}, // 244
        DACPalette {r: 0x2C, g: 0x3C, b: 0x41},
        DACPalette {r: 0x2C, g: 0x34, b: 0x41},
        DACPalette {r: 0x2C, g: 0x30, b: 0x41},

        // 248-255: all black
        DACPalette {r: 0x00, g: 0x00, b: 0x00}, // 248
        DACPalette {r: 0x00, g: 0x00, b: 0x00},
        DACPalette {r: 0x00, g: 0x00, b: 0x00},
        DACPalette {r: 0x00, g: 0x00, b: 0x00},
        DACPalette {r: 0x00, g: 0x00, b: 0x00}, // 252
        DACPalette {r: 0x00, g: 0x00, b: 0x00},
        DACPalette {r: 0x00, g: 0x00, b: 0x00},
        DACPalette {r: 0x00, g: 0x00, b: 0x00},
    ];

    pal.to_vec()
}
