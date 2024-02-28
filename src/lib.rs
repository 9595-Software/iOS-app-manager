use std::process::Command;

pub fn get_device_id(device_name: &String) -> String {
    let output = Command::new("xcrun")
        .args(&["xctrace", "list", "devices"])
        .output()
        .expect("Failed to execute command");

    let mut xcrun_xctrace_list_devices_output = String::new();

    if check_if_command_executed_successfully(&output) {
        let stdout = output.stdout.clone();
        if let Ok(stdout) = String::from_utf8(stdout) {
            xcrun_xctrace_list_devices_output = stdout;
        } else {
            eprintln!("Error: Failed to parse stdout as UTF-8");
        }
    }
    println!("Stdout: {}", xcrun_xctrace_list_devices_output);

    let mut device_id = String::new();

    for line in xcrun_xctrace_list_devices_output.lines() {
        if line.contains(device_name) {
            device_id = line.split_whitespace().last().unwrap().to_string();
            break;
        }
    }

    // remove first and last character from device_id
    if device_id.chars().count() > 1 {
        device_id = device_id
            .chars()
            .skip(1)
            .take(device_id.chars().count() - 2)
            .collect::<String>();
    }
    println!("Device ID: {}", device_id);
    return device_id;
}

pub fn get_installation_url(device_id: &String, app_path: String) -> String {
    let output = Command::new("xcrun")
        .args(&[
            "devicectl",
            "device",
            "install",
            "app",
            "--device",
            &device_id,
            &app_path,
        ])
        .output()
        .expect("Failed to execute command");

    let mut xcrun_devicectl_device_install_app_output = String::new();

    if check_if_command_executed_successfully(&output) {
        if let Ok(stdout) = String::from_utf8(output.stdout) {
            xcrun_devicectl_device_install_app_output = stdout;
        } else {
            eprintln!("Error: Failed to parse stdout as UTF-8");
        }
    }
    println!("Stdout: {}", xcrun_devicectl_device_install_app_output);

    let mut installation_url = String::new();

    for line in xcrun_devicectl_device_install_app_output.lines() {
        if line.contains("installationURL") {
            installation_url = line.to_string().chars().skip(19).collect::<String>();
            break;
        }
    }

    println!("installationURL: '{}'", installation_url);
    return installation_url;
}

pub fn launch_app(device_id: String, installation_url: String) {
    let output = Command::new("xcrun")
        .args(&[
            "devicectl",
            "device",
            "process",
            "launch",
            "--device",
            &device_id,
            &installation_url,
        ])
        .output()
        .expect("Failed to execute command");

    let mut xcrun_devicectl_device_process_launch_output = String::new();

    if check_if_command_executed_successfully(&output) {
        if let Ok(stdout) = String::from_utf8(output.stdout) {
            xcrun_devicectl_device_process_launch_output = stdout;
        } else {
            eprintln!("Error: Failed to parse stdout as UTF-8");
        }
    }
    println!("Stdout: {}", xcrun_devicectl_device_process_launch_output);
}

fn check_if_command_executed_successfully(output: &std::process::Output) -> bool {
    if output.status.success() {
        return true;
    } else {
        eprintln!("Error: Command failed with status {:?}", output.status);
        let stderr = output.stderr.clone();
        if let Ok(stderr) = String::from_utf8(stderr) {
            eprintln!("Error message:\n{}", stderr);
        } else {
            eprintln!("Error: Failed to parse stderr as UTF-8");
        }
        return false;
    }
}
