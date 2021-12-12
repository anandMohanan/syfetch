use std::process::Command;

fn r#continue(output_check: String) -> std::io::Result<String> {
    let model = output_check.split(':').collect::<Vec<&str>>()[2]
        .trim()
        .to_string();
    if model.starts_with("Advanced Micro Devices, Inc.") {
        Ok(model.split('.').collect::<Vec<&str>>()[1]
            .trim()
            .replace("[", "")
            .replace("]", "")
            .replace("\n", ""))
    } else {
        Ok(model.replace("\n", ""))
    }
}

fn gpu_name() -> std::io::Result<String> {
    let output = Command::new("sh")
        .args(&["-c", "lspci | grep -I 'VGA\\|Display\\|3D'"])
        .output()?;
    let output_check: String = String::from_utf8_lossy(&output.stdout).to_string();
    if output_check.is_empty() {
        Ok("N/A (could not run lspci/grep, make sure they are installed)".to_string())
    } else {
        r#continue(output_check)
    }
}

/// Obtain the name of the GPU
pub fn gpu_info() {
    let gpu_name = match gpu_name() {
        Ok(yes) => yes,
        Err(e) => {
            panic!("error: {}", e)
        }
    };
    println!("| gpu: {}|", gpu_name);
}
