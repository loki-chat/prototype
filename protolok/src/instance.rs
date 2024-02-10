use crate::Object;

pub trait Instance {
    /// Gets an object from its Snowflake ID
	fn from_object(&self, id: u64) -> impl Object;
}
