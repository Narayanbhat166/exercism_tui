use std::sync::{mpsc::Receiver, Arc, Mutex};

use crate::{fsm, transition, App};

#[tokio::main]
pub async fn network_calls_task(
    app: Arc<Mutex<App>>,
    input_receiver: Receiver<fsm::TransitionInput>,
) {
    {
        // Send init event so that the current window can be initialized
        transition(app.clone(), fsm::TransitionInput::Init).await;
    }

    while let Ok(input) = input_receiver.recv() {
        if input == fsm::TransitionInput::Quit {
            break;
        } else {
            transition(app.clone(), input).await;
        }
    }
}
