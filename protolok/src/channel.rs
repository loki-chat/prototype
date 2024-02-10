use crate::Message;

pub trait Channel<ErrorType, Ctx> {
	fn fetch_newest_messages(
		&self,
		amount: u32,
		begin_at_id: Option<u64>,
	) -> Result<Vec<Message>, ErrorType>;
	fn send_message(&self, message: Message, ctx: Ctx) -> Result<(), ErrorType>;
}
