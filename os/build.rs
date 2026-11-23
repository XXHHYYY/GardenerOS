use std::fs::{read_dir, File};
use std::io::{Result, Write};
use std::path::Path;

const APP_DIR: &str = "../user/src/bin";
const TARGET_DIR: &str =
    "../user/target/riscv64gc-unknown-none-elf/release";

fn main() {
    println!("cargo:rerun-if-changed={}", APP_DIR);
    println!("cargo:rerun-if-changed={}", TARGET_DIR);
    println!("cargo:rerun-if-changed=src/linker.ld");

    insert_app_data().expect("Failed to generate link_app.S");
}

fn insert_app_data() -> Result<()> {
    let mut apps = Vec::new();

    for entry in read_dir(APP_DIR)? {
        let path = entry?.path();

        if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
            let name = path
                .file_stem()
                .and_then(|name| name.to_str())
                .expect("Invalid application filename");

            apps.push(name.to_owned());
        }
    }

    apps.sort();

    assert!(!apps.is_empty(), "No applications found");
    assert!(apps.len() <= 16, "Too many applications");

    let mut file = File::create("src/link_app.S")?;

    writeln!(
        file,
        r#"
    .section .data
    .align 3
    .global _num_app
_num_app:
    .quad {}
"#,
        apps.len()
    )?;

    for index in 0..apps.len() {
        writeln!(file, "    .quad app_{}_start", index)?;
    }

    writeln!(
        file,
        "    .quad app_{}_end",
        apps.len() - 1
    )?;

    for (index, app) in apps.iter().enumerate() {
        let binary = format!("{}/{}.bin", TARGET_DIR, app);

        assert!(
            Path::new(&binary).is_file(),
            "Missing application binary: {}. Run make build in user first.",
            binary
        );

        println!("cargo:rerun-if-changed={}", binary);
        println!("cargo:warning=app_{}: {}", index, app);

        writeln!(
            file,
            r#"
    .section .data
    .global app_{0}_start
    .global app_{0}_end
app_{0}_start:
    .incbin "{1}"
app_{0}_end:
"#,
            index,
            binary
        )?;
    }

    Ok(())
}
