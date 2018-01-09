#[macro_use] extern crate bencher;
extern crate dustbox;

use bencher::Bencher;

use dustbox::cpu::CPU;
use dustbox::mmu::MMU;

fn simple_loop(b: &mut Bencher) {
    let mmu = MMU::new();
    let mut cpu = CPU::new(mmu);
    let code: Vec<u8> = vec![
        0xB9, 0xFF, 0xFF, // mov cx,0xffff
        0x49,             // dec cx
        0xEB, 0xFA,       // jmp short 0x100
    ];

    cpu.load_com(&code);

    b.iter(|| cpu.execute_instruction())
}

fn disasm_small_prog(b: &mut Bencher) {
    let mmu = MMU::new();
    let mut cpu = CPU::new(mmu);
    let code: Vec<u8> = vec![
        0x80, 0x3E, 0x31, 0x10, 0x00, // cmp byte [0x1031],0x0
        0x80, 0x3E, 0x31, 0x10, 0x00, // cmp byte [0x1031],0x0
        0x80, 0x3E, 0x31, 0x10, 0x00, // cmp byte [0x1031],0x0
        0x80, 0x3E, 0x31, 0x10, 0x00, // cmp byte [0x1031],0x0
        0x80, 0x3E, 0x31, 0x10, 0x00, // cmp byte [0x1031],0x0
        0x80, 0x3E, 0x31, 0x10, 0x00, // cmp byte [0x1031],0x0
        0x80, 0x3E, 0x31, 0x10, 0x00, // cmp byte [0x1031],0x0
        0x80, 0x3E, 0x31, 0x10, 0x00, // cmp byte [0x1031],0x0
    ];
    cpu.load_com(&code);

    b.iter(|| cpu.decoder.disassemble_block(0x85F, 0x100, 8))
}

benchmark_group!(benches, simple_loop, disasm_small_prog);
benchmark_main!(benches);