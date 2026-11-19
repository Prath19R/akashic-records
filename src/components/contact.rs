use leptos::prelude::*;
use leptos::task::spawn_local;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

#[cfg(debug_assertions)]
const WORKER_URL: &str = "http://localhost:8787";

#[cfg(not(debug_assertions))]
const WORKER_URL: &str = "https://athena.prathmeshranadive19.workers.dev";

#[derive(Serialize)]
struct ContactPayload {
    name: String,
    email: String,
    message: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct ContactResponse {
    success: bool,
    message: String,
}

#[component]
pub fn Contact() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (message, set_message) = signal(String::new());

    let (is_submitting, set_is_submitting) = signal(false);
    let (feedback, set_feedback) = signal(None::<Result<String, String>>);

    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        if is_submitting.get() {
            return;
        }

        let n = name.get().trim().to_string();
        let e = email.get().trim().to_string();
        let m = message.get().trim().to_string();

        if e.is_empty() || m.is_empty() {
            set_feedback.set(Some(Err("Please provide both email and a message.".into())));
            return;
        }

        set_is_submitting.set(true);
        set_feedback.set(None);

        spawn_local(async move {
            let payload = ContactPayload {
                name: n,
                email: e,
                message: m,
            };

            let url = format!("{WORKER_URL}/api/contact");
            let req = Request::post(&url)
                .header("Content-Type", "application/json")
                .json(&payload);

            match req {
                Ok(r) => match r.send().await {
                    Ok(resp) if resp.ok() => {
                        let parsed = resp.json::<ContactResponse>().await;
                        let msg = parsed
                            .map(|p| p.message)
                            .unwrap_or_else(|_| "Message sent successfully!".into());
                        set_feedback.set(Some(Ok(msg)));
                        set_name.set(String::new());
                        set_email.set(String::new());
                        set_message.set(String::new());
                    }
                    Ok(resp) => {
                        set_feedback.set(Some(Err(format!(
                            "Server responded with status {}. Please email directly.",
                            resp.status()
                        ))));
                    }
                    Err(err) => {
                        set_feedback.set(Some(Err(format!("Network error: {err}"))));
                    }
                },
                Err(err) => {
                    set_feedback.set(Some(Err(format!("Serialization error: {err}"))));
                }
            }

            set_is_submitting.set(false);
        });
    };

    view! {
        <section id="contact" class="py-20 px-4 sm:px-6 lg:px-8 max-w-6xl mx-auto border-t border-zinc-900">
            <div class="space-y-4 mb-10">
                <div class="text-xs font-mono text-zinc-500 uppercase tracking-widest">"// 05. Get In Touch"</div>
                <h2 class="text-2xl sm:text-3xl font-bold tracking-tight text-zinc-100">"Initiate Communication"</h2>
                <p class="text-sm text-zinc-400 max-w-xl font-sans">
                    "Have a systems engineering challenge, distributed platform inquiry, or role? Send a message directly or connect via email."
                </p>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-5 gap-8">
                // Left info
                <div class="md:col-span-2 space-y-4 text-xs font-mono">
                    <div class="bg-zinc-900/40 border border-zinc-800/80 p-5 rounded-2xl space-y-3">
                        <span class="text-zinc-500 uppercase tracking-wider block text-[10px]">"Direct Phone / WhatsApp"</span>
                        <a
                            href="tel:+918319057558"
                            class="text-sm font-semibold text-emerald-400 hover:text-emerald-300 transition-colors block font-mono"
                        >
                            "+91 8319057558"
                        </a>
                        <p class="text-zinc-400 font-sans text-xs">
                            "Available for direct calls, SMS, and WhatsApp."
                        </p>
                    </div>

                    <div class="bg-zinc-900/40 border border-zinc-800/80 p-5 rounded-2xl space-y-3">
                        <span class="text-zinc-500 uppercase tracking-wider block text-[10px]">"Direct Email"</span>
                        <a
                            href="mailto:prathmeshranadive@gmail.com"
                            class="text-sm font-semibold text-zinc-200 hover:text-white transition-colors block font-mono"
                        >
                            "prathmeshranadive@gmail.com"
                        </a>
                        <p class="text-zinc-400 font-sans text-xs">
                            "Monitored daily. PGP or plain communication welcome."
                        </p>
                    </div>

                    <div class="bg-zinc-900/40 border border-zinc-800/80 p-5 rounded-2xl space-y-3">
                        <span class="text-zinc-500 uppercase tracking-wider block text-[10px]">"Social & Repositories"</span>
                        <div class="flex flex-col gap-2">
                            <a
                                href="https://linkedin.com/in/prathmesh-ranadive"
                                target="_blank"
                                rel="noreferrer"
                                class="text-zinc-300 hover:text-white transition-colors flex items-center justify-between"
                            >
                                <span>"LinkedIn"</span>
                                <span>"↗"</span>
                            </a>
                            <a
                                href="https://github.com/Prath19R"
                                target="_blank"
                                rel="noreferrer"
                                class="text-zinc-300 hover:text-white transition-colors flex items-center justify-between"
                            >
                                <span>"GitHub (Prath19R)"</span>
                                <span>"↗"</span>
                            </a>
                        </div>
                    </div>
                </div>

                // Right form
                <div class="md:col-span-3 bg-zinc-900/40 border border-zinc-800/80 p-6 sm:p-7 rounded-2xl">
                    <form on:submit=handle_submit class="space-y-4">
                        <div>
                            <label class="block text-xs font-mono uppercase tracking-wider text-zinc-400 mb-1.5">
                                "Your Name"
                            </label>
                            <input
                                type="text"
                                prop:value=move || name.get()
                                on:input=move |ev| set_name.set(event_target_value(&ev))
                                placeholder="Jane Doe"
                                class="w-full bg-zinc-950 border border-zinc-800 rounded-xl px-3.5 py-2 text-xs sm:text-sm text-zinc-100 placeholder-zinc-600 focus:outline-none focus:border-zinc-700 transition-colors"
                            />
                        </div>

                        <div>
                            <label class="block text-xs font-mono uppercase tracking-wider text-zinc-400 mb-1.5">
                                "Your Email *"
                            </label>
                            <input
                                type="email"
                                required
                                prop:value=move || email.get()
                                on:input=move |ev| set_email.set(event_target_value(&ev))
                                placeholder="jane@example.com"
                                class="w-full bg-zinc-950 border border-zinc-800 rounded-xl px-3.5 py-2 text-xs sm:text-sm text-zinc-100 placeholder-zinc-600 focus:outline-none focus:border-zinc-700 transition-colors"
                            />
                        </div>

                        <div>
                            <label class="block text-xs font-mono uppercase tracking-wider text-zinc-400 mb-1.5">
                                "Message *"
                            </label>
                            <textarea
                                required
                                rows="4"
                                prop:value=move || message.get()
                                on:input=move |ev| set_message.set(event_target_value(&ev))
                                placeholder="Tell me about your distributed systems project or role..."
                                class="w-full bg-zinc-950 border border-zinc-800 rounded-xl px-3.5 py-2 text-xs sm:text-sm text-zinc-100 placeholder-zinc-600 focus:outline-none focus:border-zinc-700 transition-colors resize-none"
                            ></textarea>
                        </div>

                        {move || feedback.get().map(|res| match res {
                            Ok(msg) => view! {
                                <div class="p-3 rounded-lg bg-emerald-950/80 border border-emerald-800 text-emerald-300 text-xs font-mono">
                                    {msg}
                                </div>
                            }.into_any(),
                            Err(err) => view! {
                                <div class="p-3 rounded-lg bg-rose-950/80 border border-rose-800 text-rose-300 text-xs font-mono">
                                    {err}
                                </div>
                            }.into_any(),
                        })}

                        <button
                            type="submit"
                            disabled=move || is_submitting.get()
                            class="w-full sm:w-auto px-6 py-2.5 rounded-xl bg-zinc-100 text-zinc-950 font-semibold text-xs sm:text-sm hover:bg-white transition-all disabled:opacity-40 disabled:hover:bg-zinc-100 shadow-md"
                        >
                            {move || if is_submitting.get() { "Transmitting..." } else { "Send Message" }}
                        </button>
                    </form>
                </div>
            </div>
        </section>
    }
}
