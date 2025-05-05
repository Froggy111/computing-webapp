use crate::app::button::Button;
use crate::app::triangle::{Triangle, Triangle2};
use leptos::prelude::*;

#[component]
pub fn Body() -> impl IntoView {
    let triangle_spread = view! { <{..} class="block w-screen h-screen" /> };
    view! { <Triangle2 {..triangle_spread} /> }
}
