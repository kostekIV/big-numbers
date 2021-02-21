extern crate cc;

fn main() {
    let asm_srcs = if cfg!(target_arch = "x86_64") {
        [
            "src/asm_ops/asm/x86_64/add.s",
            "src/asm_ops/asm/x86_64/add_const.s",
            "src/asm_ops/asm/x86_64/cmp.s",
            "src/asm_ops/asm/x86_64/div_const.s",
            "src/asm_ops/asm/x86_64/sub.s",
            "src/asm_ops/asm/x86_64/sub_const.s",
            "src/asm_ops/asm/x86_64/mul.s",
            "src/asm_ops/asm/x86_64/mul_const.s",
        ]
        .to_vec()
    } else {
        [].to_vec()
    };

    cc::Build::new().files(asm_srcs).compile("libasm.a");
}
