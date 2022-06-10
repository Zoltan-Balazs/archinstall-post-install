use inquire::{error::InquireResult, Confirm, MultiSelect,};
use ansi_term::Style;
use std::process::Command;

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

fn run_command(string: &str) {
    let command = string.split_whitespace().next().unwrap();
    let args = string.split_whitespace().skip(1).collect::<Vec<_>>();
    Command::new(command).args(args).status().unwrap_or_else(|_| panic!("{}", format!("failed to run command {}", command)));
}

fn run_enable_services(services: Vec<&str>) {
    for service in services {
        match service {
            "bluez" => run_command("sudo systemctl enable --now bluetooth.service"),
            "cups" => run_command("sudo systemctl enable --now cups.service"),
            "bluez_utils" | "hplip" | "samsung-unified-driver" | "samsung-unified-driver-printer" => (),
            _ => run_command(format!("sudo systemctl enable --now {}", service).as_str())
        }
        
    }
}

fn main() -> InquireResult<()> {
    run_command("sudo pacman -S rustup --noconfirm");
    run_command("rustup install stable");
    run_command("rustup default stable");

    run_command("git clone https://aur.archlinux.org/paru-bin");
    std::env::set_current_dir("paru-bin").expect("failed to execute process");
    run_command("makepkg -si");
    std::env::set_current_dir("..").expect("failed to execute process");
    run_command("rm -rf paru-bin");

    let software_packages = vec![
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

    let service_packages = vec![
        "auto-cpufreq",
        "bluez",
        "bluez-utils",
        "cups",
        "samsung-unified-driver",
        "samsung-unified-driver-printer",
        "system76-power",
    ];

    let font_packages = vec![
        "nerd-fonts-cascadia-code",
        "nerd-fonts-jetbrains-mono",
        "noto-fonts-cjk",
        "noto-fonts-emoji",
        "noto-fonts-extra",
    ];

    let programming_language_packages = vec![
        "dotnet-sdk-bin",
        "go",
        "rustup",
    ];

    let utilities_packages = vec![
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
        "zoxide",
    ];

    println!("{}", Style::new().bold().paint("Zolee's Post x86_64 Archinstall Setup Program"));

    let software_list = get_list_from_packages("Which software packages do you want to install?", software_packages);
    let service_list = get_list_from_packages("Which services do you want to install?", service_packages);
    let font_list = get_list_from_packages("Which fonts do you want to install?", font_packages);
    let programming_language_list = get_list_from_packages("Which programming languages do you want to install?", programming_language_packages);
    let utilities_list = get_list_from_packages("Which utilities do you want to install?", utilities_packages);

    let install_paru = get_bool_answer_to_question("Install Paru?");
    let install_bedrock = get_bool_answer_to_question("Install Bedrock Linux?");
    let install_theme = get_bool_answer_to_question("Install KDE theme?");
    let install_omf = get_bool_answer_to_question("Install Oh My Fish?");
    let change_shell = get_bool_answer_to_question("Change Shell to Fish?");
    let enable_services = get_bool_answer_to_question("Enable installed services?");

    let mut install_software: Vec<&str> = vec![];
    software_list.iter().for_each(|s| install_software.push(s));
    service_list.iter().for_each(|s| install_software.push(s));
    font_list.iter().for_each(|s| install_software.push(s));
    programming_language_list.iter().for_each(|s| install_software.push(s));
    utilities_list.iter().for_each(|s| install_software.push(s));

    
    for package in &install_software {
        run_command(format!("paru -S --noconfirm {}", package).as_str());
    }

    if !install_paru {
        run_command("sudo pacman -Rns paru");
    }

    if !install_software.contains(&"rustup") {
        run_command("rustup self uninstall");
    }

    if enable_services {
        run_enable_services(service_list);
    }

    if install_omf {
        run_command("curl https://raw.githubusercontent.com/oh-my-fish/oh-my-fish/master/bin/install > install");
        run_command("fish install");
        run_command("rm install");
    }

    if change_shell {
        run_command("chsh -s /usr/bin/fish");
    }

    if install_theme {

    }

    if install_bedrock {
        run_command("curl https://github.com/bedrocklinux/bedrocklinux-userland/releases/download/0.7.27/bedrock-linux-0.7.27-x86_64.sh > bedrock-linux-0.7.27-x86_64.sh");
        run_command("sh ./bedrock-linux-0.7.27-x86_64.sh --hijack");
        run_command("rm bedrock-linux-0.7.27-x86_64.sh");
    }

    if install_software.contains(&"tealdeer") {
        run_command("tldr --update");
    }

    Ok(())
}