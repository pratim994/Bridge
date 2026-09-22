use leptos::prelude::*;

#[component]
pub fn Scoreboard() -> impl IntoView {
    view! {
        <section class="scoreboard">
            <div>
                <span>"Contract"</span>
                <strong>"4♥"</strong>
            </div>

            <div>
                <span>"Declarer"</span>
                <strong>"South"</strong>
            </div>

            <div>
                <span>"Tricks"</span>
                <strong>"3 / 10"</strong>
            </div>
        </section>
    }
}