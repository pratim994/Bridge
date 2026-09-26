use leptos::prelude::*;

use std::sync::{Arc, Mutex};

use crate::{
    game::GameSession,
    network::chat::{ChatClient, ChatMessage, LocalChatClient},
};

#[component]
pub fn Chat(session: GameSession) -> impl IntoView {
    let (messages, set_messages) = signal(Vec::<ChatMessage>::new());
    let (draft, set_draft) = signal(String::new());
    let (error, set_error) = signal(None::<String>);
    let client: Arc<Mutex<Box<dyn ChatClient>>> =
        Arc::new(Mutex::new(Box::new(LocalChatClient::default())));
    let client_for_submit = client.clone();
    let session_for_submit = session.clone();

    view! {
        <section class="chat">
            <div class="section-heading">
                <h2>"Table chat"</h2>
                <span class="local-label">"Local"</span>
            </div>
            <div class="chat-messages" role="log" aria-live="polite" aria-label="Chat messages">
                {move || {
                    let entries = messages.get();
                    if entries.is_empty() {
                        view! { <p class="empty-state">"No messages yet. Say hello to the table."</p> }.into_any()
                    } else {
                        entries.into_iter().map(|message| view! {
                            <article class="chat-message">
                                <div class="chat-message-meta">
                                    <strong>{message.player_name}</strong>
                                    <time>"Just now"</time>
                                </div>
                                <p>{message.text}</p>
                            </article>
                        }).collect_view().into_any()
                    }
                }}
            </div>
            <form class="chat-input" on:submit=move |event| {
                event.prevent_default();
                let text = draft.get_untracked();
                let identity = session_for_submit.state.with_untracked(|state| {
                    state.player_at(state.my_seat)
                        .map(|player| (player.id.clone(), player.name.clone()))
                        .unwrap_or_else(|| ("local".to_string(), "You".to_string()))
                });
                match client_for_submit.lock().ok().and_then(|mut client| {
                    client.send(&identity.0, &identity.1, &text).ok()
                }) {
                    Some(message) => {
                        set_messages.update(|entries| entries.push(message));
                        set_draft.set(String::new());
                        set_error.set(None);
                    }
                    None => set_error.set(Some("Unable to send this local message.".to_string())),
                }
            }>
                <input
                    type="text"
                    aria-label="Message the table"
                    maxlength="300"
                    placeholder="Message the table…"
                    prop:value=move || draft.get()
                    on:input=move |event| set_draft.set(event_target_value(&event))
                />
                <button type="submit" disabled=move || draft.get().trim().is_empty()>
                    "Send"
                </button>
            </form>
            {move || error.get().map(|message| view! { <p class="inline-error" role="alert">{message}</p> })}
        </section>
    }
}
