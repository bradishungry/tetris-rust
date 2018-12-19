extern crate shaderc;

use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    use shaderc::ShaderKind;

    // Tell the build script to only run again if we change our source shaders
    //println!("cargo:rerun-if-changed=assets/shaders");

    // Create destination path if necessary
    std::fs::create_dir_all("assets/gen/shaders")?;

    for entry in std::fs::read_dir("assets/shaders")? {
        let entry = entry?;

        if entry.file_type()?.is_file() {
            let in_path = entry.path();

            // Support only vertex and fragment shaders currently
            let shader_type =
                in_path
                    .extension()
                    .and_then(|ext| match ext.to_string_lossy().as_ref() {
                        "glslv" => Some(ShaderKind::Vertex),
                        "glslf" => Some(ShaderKind::Fragment),
                        "vert" => Some(ShaderKind::Vertex),
                        "frag" => Some(ShaderKind::Fragment),
                        _ => None,
                    });

            if let Some(shader_type) = shader_type {
                use std::io::Read;

                let mut compiler = shaderc::Compiler::new().unwrap();
                let mut options = shaderc::CompileOptions::new().unwrap();

                let source = std::fs::read_to_string(&in_path)?;
                let mut compiled_file = compiler.compile_into_spirv(&source, shader_type, 
                                        entry.file_name().into_string().unwrap().as_str(), "main", Some(&options)).unwrap();

                print!("{:?}", compiled_file.as_binary_u8());
                let mut compiled_bytes = Vec::new();
                compiled_file.as_binary_u8().read_to_end(&mut compiled_bytes)?;

                let out_path = format!(
                    "assets/gen/shaders/{}.spv",
                    in_path.file_name().unwrap().to_string_lossy()
                );

                std::fs::write(&out_path, &compiled_bytes)?;
            }
        }
    }

    Ok(())
}
