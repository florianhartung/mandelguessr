use std::{ops::Range, thread::current};

use leptos::{
    component, create_blocking_resource, create_effect, create_local_resource, create_render_effect, create_resource, create_rw_signal, html::A, view, ErrorBoundary, IntoView, ServerFnError, SignalGet, Suspense
};
use leptos_router::*;

use crate::{api, app::components::{common::Mandelbrot, Game, LogoutButton}};
#[component]
pub fn Content() -> impl IntoView {
    let username_resource =
        create_blocking_resource(|| (), |()| async { api::auth::current_user().await });

    let position = create_rw_signal((0.0, 0.0));
    let zoom_exponent = create_rw_signal(0.0);

    create_effect(move |_| {
        if username_resource.map(|x| x.is_err()).unwrap_or(false) {
            leptos_router::use_navigate()("/WebEng/Projekt", Default::default());
        }
    });

    view! {
        <ErrorBoundary fallback = move |_| {
            leptos_router::use_navigate()("/WebEng/Projekt/login", Default::default());
            view! {"Du bist nicht angemeldet, Weiterleitung zur Loginseite..."}
        }>
            <Suspense fallback=move || view!{}>
                <Game />
            </Suspense>
        </ErrorBoundary>
    }
}
