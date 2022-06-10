use inquire::{error::InquireResult, Confirm, MultiSelect,};
use ansi_term::Style;
use std::process::Command;
use sudo;

fn get_list_from_packages<'a>(message: &'a str, packages : Vec<&'a str>) -> Vec<&'a str> {
    MultiSelect::new(message, packages)
        .raw_prompt()
        .unwrap()
        .into_iter()
        .map(|x| x.value)
        .collect::<Vec<_>>()
}

fn get_bool_answer_to_question<'a>(question: &'a str) -> bool {
    Confirm::new(question)
        .with_default(true)
        .prompt_skippable()
        .unwrap()
        .unwrap_or(false)
}

fn run_command(command: &str, args: Vec<&str>) {
    Command::new(command).args(args).status().expect(format!("failed to run command {}", command).as_str());
}

fn main() -> InquireResult<()> {
    run_command("sudo", vec!["pacman", "-S", "rustup"]);
    run_command("rustup", vec!["install", "stable"]);
    run_command("rustup", vec!["default", "stable"]);

    run_command("git", vec!["clone", "https://aur.archlinux.org/paru-bin"]);
    std::env::set_current_dir("paru-bin").expect("failed to execute process");
    run_command("makepkg", vec!["-si"]);
    std::env::set_current_dir("..").expect("failed to execute process");
    run_command("rm", vec!["-rf", "paru-bin"]);

    run_command("sudo", vec!["pacman", "-S", "base-devel"]);
    run_command("git", vec!["clone", "https://aur.archlinux.org/yay.git"]);
    std::env::set_current_dir("yay").expect("failed to execute process");
    run_command("makepkg", vec!["-si"]);
    std::env::set_current_dir("..").expect("failed to execute process");
    run_command("rm", vec!["-rf", "yay"]);

    // sudo::escalate_if_needed().unwrap();

    let _software_packages = vec![
        "blender",
        "celluloid",
        "ferdium",
        "ghidra",
        "gimp",
        "gwenview",
        "jetbrains-toolbox",
        "kate",
        "kdeconnect",
        "kitty",
        "kolourpaint",
        "latte-dock",
        "libreoffice",
        "okular",
        "qalculate-qt5",
        "qbittorrent-qt5",
        "simplescreenrecorder",
        "skanlite",
        "skanpage",
        "spectacle",
        "timeshift-autosnap",
        "timeshift-bin",
        "visual-studio-code-insiders-bin",
    ];

    let _service_packages = vec![
        "auto-cpufreq",
        "bluez",
        "bluez-utils",
        "cups",
        "samsung-unified-driver",
        "samsung-unified-driver-printer",
        "system76-power",
    ];

    let _font_packages = vec![
        "nerd-fonts-cascadia-code",
        "nerd-fonts-jetbrains-mono",
        "noto-fonts-cjk",
        "noto-fonts-emoji",
        "noto-fonts-extra",
    ];

    let _programming_language_packages = vec![
        "dotnet-sdk-bin",
        "go",
        "rustup",
    ];

    let _utilities_packages = vec![
        "bat",
        "exa",
        "fd",
        "fish",
        "hunspell-hu",
        "git-delta",
        "grex",
        "ntfs-3g",
        "procs",
        "ripgrep",
        "starship",
        "tealdeer",
        "yt-dlp",
        "zoixde",
    ];

    println!("{}", Style::new().bold().paint("Zolee's Post x86_64 Archinstall Setup Program"));

    let _software_list = get_list_from_packages("Which software packages do you want to install?", _software_packages);
    let _service_list = get_list_from_packages("Which services do you want to install?", _service_packages);
    let _font_list = get_list_from_packages("Which fonts do you want to install?", _font_packages);
    let _programming_language_list = get_list_from_packages("Which programming languages do you want to install?", _programming_language_packages);
    let _utilities_list = get_list_from_packages("Which utilities do you want to install?", _utilities_packages);

    let _install_bedrock = get_bool_answer_to_question("Install Bedrock Linux?");
    let _install_theme = get_bool_answer_to_question("Install KDE theme?");
    let _enable_serivces = get_bool_answer_to_question("Enable installed services?");

    let mut _install_software: Vec<&str> = vec![];
    _software_list.iter().for_each(|s| _install_software.push(s));
    _service_list.iter().for_each(|s| _install_software.push(s));
    _font_list.iter().for_each(|s| _install_software.push(s));
    _programming_language_list.iter().for_each(|s| _install_software.push(s));
    _utilities_list.iter().for_each(|s| _install_software.push(s));

    let mut s_arg = vec!["-S"];
    s_arg.append(&mut _install_software);
    run_command("yay", s_arg);

    run_command("paru", vec!["-Rns", "yay"]);

    Ok(())
}