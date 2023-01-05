use crate::messages::prelude::*;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[remain::sorted]
#[impl_message(Message, Broadcast)]
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, TS)]
pub enum BroadcastMessage {
	// Sub-messages
	#[remain::unsorted]
	#[child]
	TriggerEvent(BroadcastEvent),

	// Messages
	SubscribeEvent {
		on: BroadcastEvent,
		send: Box<Message>,
	},
	UnsubscribeEvent {
		on: BroadcastEvent,
		message: Box<Message>,
	},
}
