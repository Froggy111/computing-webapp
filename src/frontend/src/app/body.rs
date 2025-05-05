use crate::app::button::Button;
use crate::app::triangle::{Triangle, Triangle2};
use leptos::prelude::*;

#[component]
pub fn Body() -> impl IntoView {
    view! {
        <Button />
        <div class="h-screen flex justify-center items-center">
            <Triangle2 />
        </div>
    }
}
