use eframe::egui;
use ini::{Ini, Properties, SectionSetter};

pub trait IniConfig: Sized {
	const SECTION: &'static str;
	type Error: std::error::Error;

	fn read(ini: &Ini) -> Option<Result<Self, Self::Error>> {
		let section = ini.section(Some(Self::SECTION))?;
		Some(Self::read_body(section))
	}
	fn read_body(props: &Properties) -> Result<Self, Self::Error>;
}

pub trait IniConfigWrite: IniConfig {
	fn write<'a, 'b>(&'a self, ini: &'b mut Ini) {
		let mut section = ini.with_section(Some(Self::SECTION));
		self.write_body(&mut section);
	}
	fn write_body<'a, 'b>(&'a self, section: &'b mut SectionSetter<'b>);
}

pub trait IniConfigWriteCtx: IniConfig {
	type Context;

	fn write<'a, 'b>(&'a self, ctx: &'a Self::Context, ini: &'b mut Ini) {
		{
			let mut section = ini.with_section(Some(Self::SECTION));
			self.write_body(&mut section);
		}
		let mut section = ini.with_section(Some(Self::SECTION));
		Self::write_additional(ctx, &mut section);
	}
	fn write_body<'a, 'b>(&'a self, section: &'b mut SectionSetter<'b>);
	fn write_additional<'a, 'b>(add: &'a Self::Context, section: &'b mut SectionSetter<'b>);
}

pub trait DrawUi {
	fn draw(&mut self, ui: &mut egui::Ui);
}
