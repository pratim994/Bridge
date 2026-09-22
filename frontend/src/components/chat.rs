use leptos::prelude::*;

#[component]
pub fn Chat() -> impl IntoView {
    view! {
        <section class="chat">
            <h2>"Chat"</h2>

            <div class="chat-messages">
                <div class="chat-message">
                    <strong>"North"</strong>
                    <span>"Good luck!"</span>
                </div>

                <div class="chat-message">
                    <strong>"South"</strong>
                    <span>"Have fun."</span>
                </div>
            </div>

            <form class="chat-input">
                <input
                    type="text"
                    placeholder="Message the table..."
                />

                <button type="submit">
                    "Send"
                </button>
            </form>
        </section>
    }
}