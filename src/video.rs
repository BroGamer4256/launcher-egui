use eframe::egui;
use ini::{Properties, SectionSetter};

use crate::utils::*;
use crate::{get_ini_value, graphics::AppGraphics, int_text_box, DisplayFormat};

pub struct AppVideo {
	pub(crate) display_format: DisplayFormat,
	pub(crate) window_size_x: i32,
	pub(crate) window_size_y: i32,
	pub(crate) internal_resolution_enabled: bool,
	pub(crate) internal_resolution_x: i32,
	pub(crate) internal_resolution_y: i32,
}

impl IniConfig for AppVideo {
	const SECTION: &'static str = "Resolution";
	type Error = std::io::Error;

	fn read_body(resolution_section: &Properties) -> Result<Self, Self::Error> {
		//TODO: handle the error case
		let display_format =
			DisplayFormat::from_u8(get_ini_value::<u8>(resolution_section, "Display"))
				.unwrap_or_default();
		let window_size_x = get_ini_value(resolution_section, "Width");
		let window_size_y = get_ini_value(resolution_section, "Height");
		let internal_resolution_enabled = get_ini_value(resolution_section, "r.Enable");
		let internal_resolution_x = get_ini_value(resolution_section, "r.Width");
		let internal_resolution_y = get_ini_value(resolution_section, "r.Height");
		Ok(Self {
			display_format,
			window_size_x,
			window_size_y,
			internal_resolution_enabled,
			internal_resolution_x,
			internal_resolution_y,
		})
	}
}

impl IniConfigWriteCtx for AppVideo {
	type Context = AppGraphics;

	fn write_body<'a, 'b>(&'a self, section: &'b mut SectionSetter<'b>) {
		section
			.set("Display", (self.display_format as i32).to_string())
			.set("Width", self.window_size_x.to_string())
			.set("Height", self.window_size_y.to_string())
			.set(
				"r.Enable",
				(self.internal_resolution_enabled as i32).to_string(),
			)
			.set("r.Width", self.internal_resolution_x.to_string())
			.set("r.Height", self.internal_resolution_y.to_string());
	}

	fn write_additional<'a, 'b>(add: &'a Self::Context, section: &'b mut SectionSetter<'b>) {
		section.set("RefreshRate", add.frame_rate.to_string());
	}
}

impl DrawUi for AppVideo {
	fn draw(&mut self, ui: &mut eframe::egui::Ui) {
		const VARIANTS: [DisplayFormat; 4] = [
			DisplayFormat::Windowed,
			DisplayFormat::Popup,
			DisplayFormat::Exclusive,
			DisplayFormat::Borderless,
		];
		ui.horizontal(|ui| {
			ui.label("Display type");
			egui::ComboBox::from_label("")
				.selected_text(self.display_format.to_string())
				.width(ui.available_width() / 4.0)
				.show_ui(ui, |ui| {
					for variant in VARIANTS {
						let display = variant.to_string();
						ui.selectable_value(&mut self.display_format, variant, display);
					}
				});
		});

		ui.horizontal(|ui| {
			ui.add(egui::Label::new("Window Size"))
				.on_hover_text("-1 in both means to match the size of the screen");

			int_text_box(&mut self.window_size_x, 2.0, ui);
			int_text_box(&mut self.window_size_y, 1.0, ui);
		});

		ui.horizontal(|ui| {
			ui.add(egui::Label::new("Internal Resolution"))
				.on_hover_text("-1 in both means to match the size of the window");
			ui.checkbox(&mut self.internal_resolution_enabled, "");

			int_text_box(&mut self.internal_resolution_x, 2.0, ui);
			int_text_box(&mut self.internal_resolution_y, 1.0, ui);
		});
	}
}

impl Default for AppVideo {
	fn default() -> Self {
		Self {
			display_format: DisplayFormat::default(),
			window_size_x: -1,
			window_size_y: -1,
			internal_resolution_enabled: true,
			internal_resolution_x: 1920,
			internal_resolution_y: 1080,
		}
	}
}
