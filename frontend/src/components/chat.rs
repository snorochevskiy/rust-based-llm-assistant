use leptos::{prelude::*, task::spawn_local};

use crate::{api, entity::ChatEntry};

#[component]
pub fn ChatPanel() -> impl IntoView {
    let (promt_getter, promt_setter) = signal::<String>("".to_string());
    let (chat_history_getter, chat_history_setter) = signal::<Vec<(usize, ChatEntry)>>(Vec::new());

    spawn_local(async move {
        let history = api::load_history().await.unwrap();
        chat_history_setter.write().extend(history.into_iter().enumerate());
    });

    let fallback = move |errors: ArcRwSignal<Errors>| {
        let error_list = move || {
            errors.with(|errors| {
                errors
                    .iter()
                    .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                    .collect::<Vec<_>>()
            })
        };

        view! {
            <div class="error">
                <h2>"Error"</h2>
                <ul>{error_list}</ul>
            </div>
        }
    };

    view! {
        <div class="container" >
            <div class="row scrollable-div" style="height: 50vh; overflow: auto; border: 1px solid #ccc; padding: 1rem;" >
                <Transition fallback=|| view! { <div>"Loading..."</div> }>
                    <ErrorBoundary fallback>
                        <ul>
                            <For
                                each=move || chat_history_getter.get()
                                key=|k| k.0.clone()
                                let(child)
                            >
                                {match child.1 {
                                    ChatEntry::UserProm(txt) => view! {
                                        <p class="border rounded-1 rounded-start-pill border-primary p-1" >{txt}</p>
                                    },
                                    ChatEntry::AssistantTextResponse(txt) => view! {
                                        <p class="border rounded-1 border-secondary p-1" >{txt}</p>
                                    }
                                }}
                            </For>
                        </ul>
                    </ErrorBoundary>
                </Transition>
            </div>

            <div class="row">
                <label for="textareaExample" class="form-label">Promt</label>
                <textarea
                    class="form-control" id="textareaExample" rows="4" placeholder="Type here..."
                    on:input:target=move |ev| {
                            let val = ev.target().value();
                            promt_setter.set(val);
                        }
                    prop:value=move || promt_getter.get()
                ></textarea>
            </div>

            <div class="row">
                <div class="col">
                    <button
                        class="btn btn-primary m-1"
                        on:click=move |_| {
                            let f = send_promt_and_update_chat(chat_history_setter, promt_getter.get());
                            promt_setter.set("".to_string());
                            spawn_local(f);
                        }
                    >
                        Send
                    </button>
                </div>
                <div class="col">
                    <button
                        class="btn btn-danger m-1"
                        on:click=move |_| {
                            let f = new_chat(chat_history_setter);
                            spawn_local(f);
                        }
                    >
                        New chat
                    </button>
                </div>
            </div>
        </div>
    }
}

// https://github.com/leptos-rs/leptos/blob/main/examples/fetch/src/lib.rs

async fn send_promt_and_update_chat(chat_history_setter: WriteSignal<Vec<(usize, ChatEntry)>>, promt: String) {
    chat_history_setter.update(|v| {
        v.push((v.len(), ChatEntry::UserProm(promt.clone())));
    });

    let new_vals = api::send_promt(&promt).await.unwrap();
    let existing_len = chat_history_setter.write().len();
    let received_len = new_vals.len();
    let keyed_messages: Vec<(usize, ChatEntry)> = (existing_len .. existing_len + received_len).into_iter().zip(new_vals)
        .collect();
    
    chat_history_setter.update(|v| {
        v.extend_from_slice(&keyed_messages[..]);
    });
}

async fn new_chat(chat_history_setter: WriteSignal<Vec<(usize, ChatEntry)>>) {
    api::new_chat().await.unwrap();
    chat_history_setter.update(|v|v.clear());
}