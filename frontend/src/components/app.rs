use leptos::prelude::*;
use leptos_router::{components::{Route, Router, Routes}, path};

use crate::{api::{init_session}};

use crate::components::dashboard::Dashboard;
use crate::components::chat::ChatPanel;

#[component]
pub fn App() -> impl IntoView {
    let session_info = LocalResource::new(move || init_session());

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
        <Transition fallback=|| view! { <div>"Loading..."</div> }>
            <ErrorBoundary fallback>
                {move || Suspend::new(async move {
                    session_info.await
                        .map(|_| {
                            view! {
                                <MainRouter />
                            }
                        })
                })}
            </ErrorBoundary>
        </Transition>
    }
}

#[component]
fn MainRouter() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <MainMenu />
            </nav>
            <main>
                <Routes fallback=|| "Not found.">
                    <Route path=path!("/") view=Dashboard />
                    <Route path=path!("/chat") view=ChatPanel />
                    <Route path=path!("/*any") view=|| view! { <h1>"Not Found"</h1> } />
                </Routes>
            </main>
      </Router>
    }
}

#[component]
pub fn MainMenu() -> impl IntoView {
    view! {
        <nav class="navbar navbar-expand-lg bg-body-tertiary">
            <div class="container-fluid">
                <a class="navbar-brand" href="#">
                <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Rust_programming_language_black_logo.svg/106px-Rust_programming_language_black_logo.svg.png" alt="Logo" width="30" height="24" class="d-inline-block align-text-top" />
                    SME
                </a>
                <div class="collapse navbar-collapse" id="navbarNav">
                    <ul class="navbar-nav">
                        <li class="nav-item">
                            <a class="nav-link" href="/">Dashboard</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="/chat">Chat</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="/form">Form</a>
                        </li>
                    </ul>
                </div>
            </div>
        </nav>
    }
}

