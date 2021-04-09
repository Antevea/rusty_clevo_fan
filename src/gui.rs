use ksni;
use std::process::Command;
use orbtk::prelude::*;

#[derive(Debug)]
struct MyTray {
	selected_option: usize,
	checked: bool,
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

fn orbtk_main() {
	orbtk::initialize();

	Application::new()
		.window(|ctx| {
			Window::new()
				.title("OrbTK Window")
				.size(200.0, 100.0)
				.child(
					TextBlock::new()
					.text("TextBlock")
					.v_align("center")
					.h_align("center")
					.build(ctx)
				)
				.build(ctx)
		}) 
		.run();
}

impl ksni::Tray for MyTray {
	/// Path to find new installed icons
	fn icon_theme_path(&self) -> String {
		"$HOME/.local/share/icons/hicolor".into()
	}

	/// Set installed icon w/ xdg-icon-resource
	fn icon_name(&self) -> String {
		"debug-run".into()//"rust-icon-light".into() 
	}

	/// Set title
	fn title(&self) -> String {
		"Rusty fan control".into()
	}

	fn activate(&mut self, _x: i32, _y: i32) {
		let orbtk_hndl_thr = std::thread::spawn(||{
			orbtk_main();
		});
		orbtk_hndl_thr.join().unwrap();
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

