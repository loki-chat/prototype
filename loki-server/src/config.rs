pub struct ServerConfig {
	pub port: u16,
	pub url: String,
	pub loki_auth: String, // note for config parser: DO NOT CHANGE THE VALUE UNLESS YOU KNOW WHAT YOU ARE DOING!
}
