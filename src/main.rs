use curl::easy::Easy;
use inquire::{error::InquireResult, Confirm, MultiSelect, Text,};
use ansi_term::Style;
use std::{process::Command, io::Write, fs::File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Packages {
    software: Vec<String>,
    service: Vec<String>,
    font: Vec<String>,
    programming_language: Vec<String>,
    utility: Vec<String>,
}

impl Default for Packages {
    fn default() -> Self {
        Packages { software: vec![], service: vec![], font: vec![], programming_language: vec![], utility: vec![] }
    }
}

#[derive(Debug)]
pub struct Settings {
    install_paru: bool,
    install_bedrock: bool,
    install_kde_theme: bool,
    install_omf: bool,
    change_shell: bool,
    enable_services: bool,
    normalize_audio: bool,
    set_git_config: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings { install_paru: false, install_bedrock: false, install_kde_theme: false, install_omf: false, change_shell: false, enable_services: false, normalize_audio: false, set_git_config: false }
    }
}

#[derive(Debug)]
pub struct Installer {
    packages: Packages,
    settings: Settings,
}

impl Default for Installer {
    fn default() -> Self {
        Installer { packages: Packages::default(), settings: Settings::default() }
    }
}

fn get_list_from_packages<'a>(message: &'a str, packages : Vec<String>) -> Vec<String> {
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

fn run_command_with_args(input_string: &str) {
    let command = input_string.split_whitespace().next().unwrap();
    let args = input_string.split_whitespace().skip(1).collect::<Vec<_>>();
    Command::new(command).args(args).status().unwrap_or_else(|_| panic!("{}", format!("failed to run command {}", command)));
}

fn run_command(command: &str) {
    Command::new(command).status().unwrap_or_else(|_| panic!("{}", format!("failed to run command {}", command)));
}

fn run_enable_services(services: Vec<String>) {
    for service in services {
        match service.as_str() {
            "bluez" => run_command_with_args("sudo systemctl enable --now bluetooth.service"),
            "cups" => run_command_with_args("sudo systemctl enable --now cups.service"),
            "bluez_utils" | "hplip" | "samsung-unified-driver" | "samsung-unified-driver-printer" => (),
            _ => run_command_with_args(format!("sudo systemctl enable --now {}", service).as_str())
        }
        
    }
}

fn install_rust_and_paru() -> () {
    run_command_with_args("sudo pacman -S rustup --noconfirm");
    run_command_with_args("rustup install stable");
    run_command_with_args("rustup default stable");

    run_command_with_args("git clone https://aur.archlinux.org/paru-bin");
    std::env::set_current_dir("paru-bin").expect("failed to execute process");
    run_command_with_args("makepkg -si");
    std::env::set_current_dir("..").expect("failed to execute process");
    run_command_with_args("rm -rf paru-bin");
}

fn ask_for_user_input(packages: Packages, mut install: Installer) -> Installer {
    install.packages.software = get_list_from_packages("Which software packages do you want to install?", packages.software);
    install.packages.service = get_list_from_packages("Which services do you want to install?", packages.service);
    install.packages.font = get_list_from_packages("Which fonts do you want to install?", packages.font);
    install.packages.programming_language = get_list_from_packages("Which programming languages do you want to install?", packages.programming_language);
    install.packages.utility = get_list_from_packages("Which utilities do you want to install?", packages.utility);

    install.settings.install_paru = get_bool_answer_to_question("Install paru?");
    install.settings.install_bedrock = get_bool_answer_to_question("Install Bedrock Linux?");
    install.settings.install_kde_theme = get_bool_answer_to_question("Install KDE theme?");
    install.settings.install_omf = get_bool_answer_to_question("Install Oh My Fish?");
    install.settings.change_shell = get_bool_answer_to_question("Change shell to fish?");
    install.settings.enable_services = get_bool_answer_to_question("Enable installed services?");
    install.settings.normalize_audio = get_bool_answer_to_question("Normalize audio volume?");
    install.settings.set_git_config = get_bool_answer_to_question("Set git username and email?");

    install
}

fn install_packages(packages: Vec<String>) {
    for package in packages {
        run_command_with_args(format!("paru -S {} --noconfirm", package).as_str());
    }
}

fn download_file(url: &str, file_name: &str) {
    let mut dst = Vec::new();
    let mut easy = Easy::new();
    easy.url(url).unwrap();
    let _redirect = easy.follow_location(true);
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            dst.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    {
        let mut file = File::create(file_name).unwrap();
        file.write_all(dst.as_slice()).unwrap();
    }
}

fn finish_install(install: Installer) {
    run_command_with_args("paru -Syu");

    install_packages(install.packages.software.clone());
    install_packages(install.packages.service.clone());
    install_packages(install.packages.font.clone());
    install_packages(install.packages.programming_language.clone());
    install_packages(install.packages.utility.clone());

    if install.settings.set_git_config {
        let name = Text::new("Git name: ").prompt().unwrap();
        let email = Text::new("Git email: ").prompt().unwrap();

        run_command_with_args(format!("git config --global user.name \"{}\"", name).as_str());
        run_command_with_args(format!("git config --global user.email \"{}\"", email).as_str());
    }
    
    if !install.settings.install_paru {
        run_command_with_args("paru -Rns paru");
    }

    if !install.packages.programming_language.contains(&"rustup".to_string()) {
        run_command_with_args("paru -Rns rustup");
    }

    if install.settings.enable_services {
        run_enable_services(install.packages.service.clone());
    }

    if install.settings.install_omf {
        download_file("https://raw.githubusercontent.com/oh-my-fish/oh-my-fish/master/bin/install", "install");
        run_command_with_args("fish install");
        run_command_with_args("rm install");
    }

    if install.settings.change_shell {
        run_command_with_args("chsh -s /usr/bin/fish");
    }

    if install.settings.install_kde_theme {

    }

    if install.settings.normalize_audio {
        
    }

    if install.settings.install_bedrock {
        download_file("https://github.com/bedrocklinux/bedrocklinux-userland/releases/download/0.7.27/bedrock-linux-0.7.27-x86_64.sh", "bedrock-linux-0.7.27-x86_64.sh");
        run_command_with_args("sh ./bedrock-linux-0.7.27-x86_64.sh --hijack");
        run_command_with_args("rm bedrock-linux-0.7.27-x86_64.sh");
    }

    if install.packages.software.contains(&"tealdeer".to_string()) {
        run_command_with_args("tldr --update");
    }
}

fn main() -> InquireResult<()> {
    install_rust_and_paru();
    println!("{}", Style::new().bold().paint("Zolee's Post x86_64 Archinstall Setup Program"));

    let package_json = include_str!("packages.json");
    let packages: Packages = serde_json::from_str(package_json).unwrap();

    let install: Installer = ask_for_user_input(packages, self::Installer::default());

    finish_install(install);

    Ok(())
}