extern crate cc;

fn main() {
    let asm_srcs = &[
        "src/asm_ops/add.s",
    ];
    
    cc::Build::new()
        .files(asm_srcs)
        .compile("libasm.a");
}
