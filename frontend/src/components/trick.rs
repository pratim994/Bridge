use leptos::prelude::*;

#[component]
pub fn TrickArea() -> impl IntoView {
    view! {
        <div class="trick-area">
            <div class="trick-card north-card"></div>
            <div class="trick-card east-card"></div>
            <div class="trick-card south-card"></div>
            <div class="trick-card west-card"></div>
        </div>
    }
}