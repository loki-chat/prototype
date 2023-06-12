pub struct ServerConfig {
	pub port: u16,
	pub url: &'static str,
	pub loki_auth: &'static str, // DO NOT CHANGE UNLESS YOU KNOW WHAT YOU ARE DOING!
}
