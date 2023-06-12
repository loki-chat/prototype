use crate::message::{Message, MessageContent};

pub trait Channel<ErrorType, Ctx> {
	fn fetch_newest_messages(
		&self,
		amount: u32,
		begin_at_id: Option<u64>,
	) -> Result<Vec<Message>, ErrorType>;
	fn send_message(&self, message: MessageContent, ctx: Ctx) -> Result<Message, ErrorType>;
}
