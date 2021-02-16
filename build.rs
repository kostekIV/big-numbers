extern crate cc;

fn main() {
    let asm_srcs = &[
        "src/asm_ops/asm/add.s",
        "src/asm_ops/asm/add_const.s",
        "src/asm_ops/asm/cmp.s",
        "src/asm_ops/asm/sub.s",
        "src/asm_ops/asm/sub_const.s",
        "src/asm_ops/asm/mul.s",
        "src/asm_ops/asm/mul_const.s",
    ];

    cc::Build::new().files(asm_srcs).compile("libasm.a");
}
