use ksni;
use std::process::Command;

#[derive(Debug)]
struct MyTray {
    selected_option: usize,
    checked: bool,
}

impl ksni::Tray for MyTray {
    fn icon_name(&self) -> String {
        "help-about".into()
    }
    fn title(&self) -> String {
        "Rusty fan control".into()
    }
    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        use ksni::menu::*;
        vec![
            SubMenu {
                label: "Fan duty".into(),
                submenu: vec![
                    StandardItem {
                        label: "Set 50%".into(),
                        activate: Box::new(|_| {
                            Command::new("rusty_clevo_fan")
                            .arg("50")
                            .spawn()
                            .expect("Error: failed to execute command");
                        }),
                        ..Default::default()
                    }
                    .into(),
                    StandardItem {
                        label: "Set 60%".into(),
                        activate: Box::new(|_| {
                            Command::new("rusty_clevo_fan")
                            .arg("60")
                            .spawn()
                            .expect("Error: failed to execute command");
                        }),
                        ..Default::default()
                    }
                    .into(),
                    StandardItem {
                        label: "Set 65%".into(),
                        activate: Box::new(|_| {
                            Command::new("rusty_clevo_fan")
                            .arg("65")
                            .spawn()
                            .expect("Error: failed to execute command");
                        }),
                        ..Default::default()
                    }
                    .into(),
                    StandardItem {
                        label: "Set 70%".into(),
                        activate: Box::new(|_| {
                            Command::new("rusty_clevo_fan")
                            .arg("70")
                            .spawn()
                            .expect("Error: failed to execute command");
                        }),
                        ..Default::default()
                    }
                    .into(),
                    StandardItem {
                        label: "Set 75%".into(),
                        activate: Box::new(|_| {
                            Command::new("rusty_clevo_fan")
                            .arg("75")
                            .spawn()
                            .expect("Error: failed to execute command");
                        }),
                        ..Default::default()
                    }
                    .into(),
                    StandardItem {
                        label: "Set 80%".into(),
                        activate: Box::new(|_| {
                            Command::new("rusty_clevo_fan")
                            .arg("80")
                            .spawn()
                            .expect("Error: failed to execute command");
                        }),
                        ..Default::default()
                    }
                    .into(),
                    StandardItem {
                        label: "Set 90%".into(),
                        activate: Box::new(|_| {
                            Command::new("rusty_clevo_fan")
                            .arg("90")
                            .spawn()
                            .expect("Error: failed to execute command");
                        }),
                        ..Default::default()
                    }
                    .into(),
                    StandardItem {
                        label: "Set 100%".into(),
                        activate: Box::new(|_| {
                            Command::new("rusty_clevo_fan")
                            .arg("100")
                            .spawn()
                            .expect("Error: failed to execute command");
                        }),
                        ..Default::default()
                    }
                    .into(),
                ],
                ..Default::default()
            }
            .into(),
            MenuItem::Sepatator,
            StandardItem {
                label: "Exit".into(),
                icon_name: "application-exit".into(),
                activate: Box::new(|_| std::process::exit(0)),
                ..Default::default()
            }
            .into(),
        ]
    }
}

fn main() {
    let service = ksni::TrayService::new(MyTray {
        selected_option: 0,
        checked: false,
    });
    let handle = service.handle();
    service.spawn();

    // We can modify the tray
    handle.update(|tray: &mut MyTray| {
        tray.checked = true;
    });
    // Run forever
    loop {
        std::thread::park();
    }
}
