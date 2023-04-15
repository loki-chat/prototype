mod app;
mod components;
mod ecs;

#[cfg(test)]
mod test {
	use crate::app::App;

	#[test]
	fn test() {
		let app = App::default();
	}
}
