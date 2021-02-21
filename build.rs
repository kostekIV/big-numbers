extern crate cc;

fn main() -> Result<(), String> {
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

    if asm_srcs.is_empty() {
        return Err("Unsupported architecture".to_string());
    }

    cc::Build::new().files(asm_srcs).compile("libasm.a");

    Ok(())
}
