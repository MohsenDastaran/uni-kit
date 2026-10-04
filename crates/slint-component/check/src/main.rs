//! `slint-component-check <file.slint>...` compiles each file to Rust exactly
//! as the gallery's build script does, and exits non-zero when any file has an
//! error. Warnings are printed; `scripts/check.sh` treats them as failures.
//!
//! `--output <path>` writes the generated Rust to a named path, which is how a
//! single page's code size is measured.

fn main() {
    let mut args = std::env::args().skip(1);
    let mut output = std::env::temp_dir().join("slint-component-check.rs");
    let mut failed = false;
    while let Some(arg) = args.next() {
        if arg == "--output" {
            match args.next() {
                Some(path) => output = path.into(),
                None => {
                    eprintln!("--output needs a path");
                    std::process::exit(2);
                }
            }
            continue;
        }
        let config = slint_build::CompilerConfiguration::new();
        if let Err(error) = slint_build::compile_with_output_path(&arg, &output, config) {
            eprintln!("{error}");
            failed = true;
        }
    }
    std::process::exit(i32::from(failed));
}
