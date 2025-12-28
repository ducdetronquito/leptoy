use automerge::{transaction::Transactable, AutoCommit, ReadDoc, ScalarValue};
use leptos::prelude::*;

fn get_counter_value(read_signal: &ReadSignal<AutoCommit>) -> u64 {
    read_signal
        .get()
        .get(automerge::ROOT, "counter")
        .expect("Get counter value")
        .map(|(value, _)| value.to_u64().unwrap())
        .unwrap()
}
#[component]
pub fn Button(#[prop(default = 1)] increment: i64) -> impl IntoView {
    let mut doc = AutoCommit::new();
    doc.put(automerge::ROOT, "counter", ScalarValue::counter(0))
        .expect("Failed to define counter");
    let (doc_read, doc_write) = signal(doc);
    view! {
        <button on:click=move |_| {
            doc_write
                .write()
                .increment(&automerge::ROOT, "counter", increment)
                .expect("Failed to increment counter")
        }>"Click me: " {move || { get_counter_value(&doc_read) }}</button>
    }
}
