use strum_macros::{ Display, EnumIter };

#[derive(Display, EnumIter)]
pub enum Options {
	Enable,
	Disable,
	Skip,
}
