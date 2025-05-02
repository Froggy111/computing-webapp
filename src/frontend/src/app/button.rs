use leptos::prelude::*;

#[component]
pub fn Button() -> impl IntoView {
    let (fibonacci_counter, set_fibonacci_counter) = signal(0i128);
    let (fibonacci_id, set_fibonacci_id) = signal(0);
    let (fibonacci_counter_prev, set_fibonacci_counter_prev) = signal(1i128);

    view! {
        <button
            on:click=move |_| {
                let fibonacci_counter_current = fibonacci_counter.get();
                *set_fibonacci_counter.write() += fibonacci_counter_prev.get();
                *set_fibonacci_counter_prev.write() = fibonacci_counter_current;
                *set_fibonacci_id.write() += 1;
            }
            class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
        >
            "Increment fibonacci"
        </button>
        <p>
            {move || {
                format!("Fibonacci number {} is {}", fibonacci_id.get(), fibonacci_counter.get())
            }}
        </p>
    }
}
