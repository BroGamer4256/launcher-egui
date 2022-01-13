use crate::{get_ini_value, IniConfig, IniConfigWrite};

pub struct AppGraphics {
	taa_enabled: bool,
	mlaa_enabled: bool,
	glare_enabled: bool,
	depth_of_field_enabled: bool,
	pub(crate) frame_rate: i32,
	gamma: i32,
	reflections_enabled: bool,
	shadows_enabled: bool,
	transparency_enabled: bool,
	disable_3d: bool,
}

impl IniConfig for AppGraphics {
	const SECTION: &'static str = "Graphics";

	type Error = std::io::Error;

	fn read_body(section: &ini::Properties) -> Result<Self, Self::Error> {
		let taa_enabled = get_ini_value(section, "TAA");
		let mlaa_enabled = get_ini_value(section, "MLAA");
		let glare_enabled = get_ini_value(section, "Glare");
		let depth_of_field_enabled = get_ini_value(section, "DOF");
		let frame_rate = get_ini_value(section, "FPS.Limit");
		let gamma = get_ini_value(section, "Gamma");
		let reflections_enabled = get_ini_value(section, "Reflections");
		let shadows_enabled = get_ini_value(section, "Shadows");
		let transparency_enabled = get_ini_value(section, "Punchthrough");
		let disable_3d = get_ini_value(section, "2D");
		Ok(Self {
			taa_enabled,
			mlaa_enabled,
			glare_enabled,
			depth_of_field_enabled,
			frame_rate,
			gamma,
			reflections_enabled,
			shadows_enabled,
			transparency_enabled,
			disable_3d,
		})
	}
}

impl IniConfigWrite for AppGraphics {
	fn write_body<'a, 'b>(&'a self, section: &'b mut ini::SectionSetter<'b>) {
		section
			.set("TAA", (self.taa_enabled as i32).to_string())
			.set("MLAA", (self.mlaa_enabled as i32).to_string())
			.set("Glare", (self.glare_enabled as i32).to_string())
			.set("DOF", (self.depth_of_field_enabled as i32).to_string())
			.set("FPS.Limit", self.frame_rate.to_string())
			.set("Gamma", self.gamma.to_string())
			.set("Reflections", (self.reflections_enabled as i32).to_string())
			.set("Shadows", (self.shadows_enabled as i32).to_string())
			.set(
				"Punchthrough",
				(self.transparency_enabled as i32).to_string(),
			)
			.set("2D", (self.disable_3d as i32).to_string());
	}
}

impl Default for AppGraphics {
	fn default() -> Self {
		Self {
			taa_enabled: false,
			mlaa_enabled: true,
			glare_enabled: false,
			depth_of_field_enabled: true,
			frame_rate: 60,
			gamma: 100,
			reflections_enabled: true,
			shadows_enabled: true,
			transparency_enabled: true,
			disable_3d: false,
		}
	}
}
