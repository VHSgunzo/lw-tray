use std::{
    fs,
    env,
    thread,
    time::Duration,
    process::Stdio,
    process::exit
};

use ksni::menu::*;
use rust_embed::RustEmbed;
use execute::{shell, Execute};
use translation::{tr_init, tr};

#[derive(RustEmbed)]
#[folder = "locale"]
struct Translations;

fn get_env(var: &str) -> String {
    env::var(var).unwrap_or_default()
}

fn shellspawn(command: &str) {
    let mut child = shell(command).spawn()
        .expect("Shell command failed to start!");
    thread::spawn(move || { let _ = child.wait(); });
}

fn shellexec(command: &str) -> String {
    String::from_utf8(
        shell(command)
            .stdout(Stdio::piped())
            .execute_output().unwrap().stdout
    ).unwrap()
}

fn lwexec(command: String) {
    shellspawn(format!("{} {}", get_env("LW_SOURCE"), command).as_str())
}

struct LwTray {
    icon: String,
    title: String,
    lw_apps: String
}

fn lw_activate(cmd: String) -> Box<dyn Fn(&mut LwTray) + 'static> {
    Box::new(move |_| { lwexec(cmd.to_string()); })
}

fn to_vec_string(argv: Vec<&str>) -> Vec<String> {
    argv.iter().map(|&s| s.to_string()).collect::<Vec<String>>()
}

impl ksni::Tray for LwTray {
    fn icon_name(&self) -> String {
        self.icon.clone()
    }
    fn title(&self) -> String {
        self.title.clone()
    }
    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {

        let mut apps_submenu = Vec::new();
        if ! self.lw_apps.is_empty() {
            for app in self.lw_apps.lines() {
                let app_split = to_vec_string(app.split(" ##&## ").collect());
                let name = app_split[0].clone();
                let icon = app_split[1].clone();
                apps_submenu.push(
                    StandardItem {
                        label: name.clone(),
                        icon_name: icon.clone(),
                        activate: lw_activate(format!("-runapp \"{}\"", name.clone())),
                        ..Default::default()
                    }.into()
                );
            }
        }

        let mut appcfg_submenu = Vec::new();
        if ! self.lw_apps.is_empty() {
            for app in self.lw_apps.lines() {
                let app_split = to_vec_string(app.split(" ##&## ").collect());
                let name = app_split[0].clone();
                let icon = app_split[1].clone();
                appcfg_submenu.push(
                    StandardItem {
                        label: name.clone(),
                        icon_name: icon.clone(),
                        activate: lw_activate(format!("-appcfg \"{}\"", name.clone())),
                        ..Default::default()
                    }.into()
                );
            }
        }

        vec![
            SubMenu {
                label: tr!("Apps"),
                icon_name: self.icon.clone(),
                submenu: apps_submenu,
                visible: !self.lw_apps.is_empty(),
                ..Default::default()
            }.into(),
            SubMenu {
                label: tr!("Open"),
                icon_name: self.icon.clone(),
                visible: get_env("LU_EXE").is_empty(),
                submenu: vec![
                    StandardItem {
                        label: tr!("Lux Wine"),
                        icon_name: self.icon.clone(),
                        activate: lw_activate("".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: tr!("DEBUG"),
                        icon_name: self.icon.clone(),
                        activate: lw_activate("-debug".to_string()),
                        ..Default::default()
                    }.into()
                ],
                ..Default::default()
            }.into(),
            SubMenu {
                label: tr!("Wine"),
                icon_name: self.icon.clone(),
                submenu: vec![
                    StandardItem {
                        label: tr!("Explorer"),
                        icon_name: self.icon.clone(),
                        activate: lw_activate("-explorer".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: tr!("Task manager"),
                        icon_name: self.icon.clone(),
                        activate: lw_activate("-taskmgr".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: tr!("CMD"),
                        icon_name: self.icon.clone(),
                        activate: lw_activate("-cmd".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: tr!("Control panel"),
                        icon_name: self.icon.clone(),
                        activate: lw_activate("-control".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: tr!("Settings"),
                        icon_name: self.icon.clone(),
                        activate: lw_activate("-winecfg".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: tr!("Registry editor"),
                        icon_name: self.icon.clone(),
                        activate: lw_activate("-regedit".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: tr!("Uninstaller"),
                        icon_name: self.icon.clone(),
                        activate: lw_activate("-uninstaller".to_string()),
                        ..Default::default()
                    }.into()
                ],
                ..Default::default()
            }.into(),
            SubMenu {
                label: tr!("Kill"),
                icon_name: self.icon.clone(),
                submenu: vec![
                    StandardItem {
                        label: tr!("EXE"),
                        icon_name: self.icon.clone(),
                        activate: lw_activate("-killexe".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: tr!("Wine"),
                        icon_name: self.icon.clone(),
                        activate: lw_activate("-killwine".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: tr!("SHELL"),
                        icon_name: self.icon.clone(),
                        activate: lw_activate("-killshell".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: tr!("ALL"),
                        icon_name: self.icon.clone(),
                        activate: lw_activate("-exit".to_string()),
                        ..Default::default()
                    }.into(),
                ],
                ..Default::default()
            }.into(),
            SubMenu {
                label: tr!("Prefix"),
                icon_name: self.icon.clone(),
                submenu: vec![
                    StandardItem {
                        label: tr!("Open drive C:\\"),
                        icon_name: self.icon.clone(),
                        activate: lw_activate("-openpfx".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: tr!("Backup"),
                        icon_name: self.icon.clone(),
                        activate: lw_activate("-pfxbackup".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: tr!("Restore"),
                        icon_name: self.icon.clone(),
                        activate: lw_activate("-pfxrestore".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: tr!("Clear"),
                        icon_name: self.icon.clone(),
                        activate: lw_activate("-clearpfx".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: tr!("Mount backup"),
                        icon_name: self.icon.clone(),
                        activate: lw_activate("-backupmnt".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: tr!("Unmount backup"),
                        icon_name: self.icon.clone(),
                        activate: lw_activate("-backupunmnt".to_string()),
                        ..Default::default()
                    }.into()
                ],
                ..Default::default()
            }.into(),
            SubMenu {
                label: tr!("Settings"),
                icon_name: self.icon.clone(),
                submenu: vec![
                    StandardItem {
                        label: self.title.clone(),
                        icon_name: self.icon.clone(),
                        activate: lw_activate("-config".to_string()),
                        ..Default::default()
                    }.into(),
                    SubMenu {
                        label: tr!("Apps settings"),
                        icon_name: self.icon.clone(),
                        submenu: appcfg_submenu,
                        visible: !self.lw_apps.is_empty(),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: tr!("Wine manager"),
                        icon_name: self.icon.clone(),
                        activate: lw_activate("-winemgr".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: tr!("Runtime updater"),
                        icon_name: self.icon.clone(),
                        activate: lw_activate("-update".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: tr!("Winetricks"),
                        icon_name: self.icon.clone(),
                        activate: lw_activate("-winetricks".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: tr!("Forced init"),
                        icon_name: self.icon.clone(),
                        activate: lw_activate("-init".to_string()),
                        ..Default::default()
                    }.into(),
                    StandardItem {
                        label: tr!("Open SHELL"),
                        icon_name: self.icon.clone(),
                        activate: lw_activate("-shell".to_string()),
                        ..Default::default()
                    }.into(),
                    SubMenu {
                        label: tr!("Shortcut"),
                        icon_name: self.icon.clone(),
                        submenu: vec![
                            StandardItem {
                                label: tr!("Create"),
                                icon_name: self.icon.clone(),
                                activate: lw_activate("-shortcut".to_string()),
                                ..Default::default()
                            }.into(),
                            StandardItem {
                                label: tr!("Remove"),
                                icon_name: self.icon.clone(),
                                activate: lw_activate("-rmapp".to_string()),
                                ..Default::default()
                            }.into()
                        ],
                        ..Default::default()
                    }.into(),
                    SubMenu {
                        label: tr!("Info"),
                        icon_name: self.icon.clone(),
                        submenu: vec![
                            StandardItem {
                                label: tr!("Usage"),
                                icon_name: self.icon.clone(),
                                activate: lw_activate("-help".to_string()),
                                ..Default::default()
                            }.into(),
                            StandardItem {
                                label: tr!("Version"),
                                icon_name: self.icon.clone(),
                                activate: lw_activate("-version".to_string()),
                                ..Default::default()
                            }.into()
                        ],
                        ..Default::default()
                    }.into(),
                ],
                ..Default::default()
            }.into(),
            StandardItem {
                label: tr!("Exit"),
                icon_name: "application-exit".into(),
                activate: Box::new(|_| exit(0)),
                ..Default::default()
            }.into()
        ]
    }
    fn id(&self) -> String {
        "LwTray".to_string()
    }
}

fn get_lw_apps() -> String {
    shellexec(
        "grep '^Categories=Lux Wine Apps' -lr ''$LW_APPS_DIR'' 2>/dev/null|\
                 xargs -I {{}} grep -m2 '^Name=\\|^Icon=' {{}} 2>/dev/null|sed 's|^Name=||g'|\
                 sed ':a;N;$!ba;s|\\nIcon=| ##\\&## |g'|sort -u"
    )
}

fn ls_lw_apps() -> Vec<String> {
    let mut apps = vec![];
    if let Ok(entries) = fs::read_dir(get_env("LW_APPS_DIR")) {
        for app in entries {
            apps.push(app.unwrap().file_name().to_str().unwrap().to_string())
        }
    }
    apps
}

fn main() {
    tr_init!("locale", Translations);

    let lw_apps = get_lw_apps();

    let mut icon = get_env("LW_EXE_ICON");
    if icon.is_empty() {
        icon = get_env("LW_DEF_ICO").to_string()
    }

    let mut title = get_env("EXE_NAME");
    if title.is_empty() {
        title = "Lux Wine".to_string()
    }

    let service = ksni::TrayService::new(LwTray {
        icon,
        title,
        lw_apps
    });
    let handle = service.handle();
    service.spawn();

    loop {
        let old_lw_apps = ls_lw_apps();
        std::thread::sleep(Duration::from_secs(1));
        let new_lw_apps = ls_lw_apps();
        if new_lw_apps != old_lw_apps {
            handle.update(|tray: &mut LwTray| {
                tray.lw_apps = get_lw_apps()
            });
        }
    }
}
