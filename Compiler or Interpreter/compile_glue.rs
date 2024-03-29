use std::process::Command;
use anyhow::Result;

fn main() -> Result<()> {
    let source_file = "glue_program.glue"; // Your GLUE source file
    let output_file = "glue_executable";   // Desired output executable name

    transpile_glue_to_rust(source_file, output_file)?;
    compile_rust_to_binary(output_file)?;
    println!("Compilation successful: {}", output_file);

    Ok(())
}

fn transpile_glue_to_rust(source_file: &str, output_file: &str) -> Result<()> {
    // Placeholder: Implement the actual transpilation logic from GLUE to Rust
    println!("Transpiling {} to Rust...", source_file);
    // Assuming transpilation success
    Ok(())
}

fn compile_rust_to_binary(output_file: &str) -> Result<()> {
    let rustc_output = Command::new("rustc")
        .arg(format!("{}.rs", output_file)) // Assuming the Rust file is named after the output
        .arg("-o")
        .arg(output_file)
        // Add any additional flags or configurations as needed
        .output()?;

    if !rustc_output.status.success() {
        anyhow::bail!("Failed to compile Rust file: {}", String::from_utf8_lossy(&rustc_output.stderr));
    }

    println!("Rust file compiled to {}", output_file);
    Ok(())
}
