use gloo_console::log;
use reqwasm::http::Request;
use shared::{AppStatus, ServiceStatus};
use yew::prelude::*;

const API_BASE: &str = "/api";

enum Msg {
    StatusReceived(AppStatus),
    CheckTriggered,
    RepairTriggered,
    FetchError(String),
}

#[function_component(App)]
fn app() -> Html {
    let status = use_state(AppStatus::default);
    let error_message = use_state(|| None::<String>);

    // Fetch status on component mount and then periodically
    {
        let status = status.clone();
        let error_message = error_message.clone();
        use_effect_with((), move |_| {
            let status = status.clone();
            let error_message = error_message.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_status = Request::get(&format!("{}/status", API_BASE))
                    .send()
                    .await;

                match fetched_status {
                    Ok(response) => {
                        if response.ok() {
                             let parsed_status: Result<AppStatus, _> = response.json().await;
                             match parsed_status {
                                 Ok(s) => status.set(s),
                                 Err(e) => error_message.set(Some(format!("JSON parsing error: {}", e))),
                             }
                        } else {
                            let err_text = response.text().await.unwrap_or_default();
                            error_message.set(Some(format!("API error [{}]: {}", response.status(), err_text)));
                        }
                    }
                    Err(e) => error_message.set(Some(format!("Request error: {}", e))),
                }
            });
            || ()
        });
    }

    let on_run_check = {
        let error_message = error_message.clone();
        Callback::from(move |_| {
            let error_message = error_message.clone();
            wasm_bindgen_futures::spawn_local(async move {
                log!("Triggering check...");
                let result = Request::post(&format!("{}/run-check", API_BASE)).send().await;
                if result.is_err() {
                    error_message.set(Some("Failed to trigger check".to_string()));
                }
            });
        })
    };

    let on_run_repair = {
        let error_message = error_message.clone();
        Callback::from(move |_| {
            let error_message = error_message.clone();
            wasm_bindgen_futures::spawn_local(async move {
                log!("Triggering repair...");
                let result = Request::post(&format!("{}/run-repair", API_BASE)).send().await;
                if result.is_err() {
                     error_message.set(Some("Failed to trigger repair".to_string()));
                }
            });
        })
    };

    let status_text = format!("{:?}", status.status);
    let status_color = match status.status {
        ServiceStatus::Idle => "bg-green-100 text-green-800",
        ServiceStatus::Scanning | ServiceStatus::Checking | ServiceStatus::Repairing => "bg-yellow-100 text-yellow-800",
        ServiceStatus::Error(_) => "bg-red-100 text-red-800",
    };

    html! {
        <div class="bg-slate-50 min-h-screen font-sans">
            <header class="bg-slate-800 text-white shadow-lg">
                <div class="container mx-auto px-4 py-4">
                    <h1 class="text-3xl font-bold">{"RS Guard Status"}</h1>
                </div>
            </header>
            <main class="container mx-auto p-4">
                if let Some(err) = &*error_message {
                    <div class="bg-red-200 border-l-4 border-red-500 text-red-700 p-4 mb-4" role="alert">
                        <p class="font-bold">{"Error"}</p>
                        <p>{err}</p>
                    </div>
                }

                // --- Status & Actions ---
                <div class="bg-white p-6 rounded-lg shadow-md mb-6">
                    <h2 class="text-2xl font-semibold mb-4 text-slate-700">{"System State"}</h2>
                    <div class="flex items-center justify-between">
                        <div>
                            <span class="text-gray-500 mr-2">{"Status:"}</span>
                            <span class={classes!("px-3", "py-1", "text-sm", "font-semibold", "rounded-full", status_color)}>
                                {status_text}
                            </span>
                        </div>
                        <div class="flex space-x-2">
                            <button onclick={on_run_check} class="bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded transition-colors duration-200">
                                {"Run Integrity Check"}
                            </button>
                            <button onclick={on_run_repair} class="bg-orange-500 hover:bg-orange-600 text-white font-bold py-2 px-4 rounded transition-colors duration-200">
                                {"Attempt Repair"}
                            </button>
                        </div>
                    </div>
                </div>

                // --- Details Grid ---
                <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-6">
                    <div class="bg-white p-5 rounded-lg shadow-md">
                        <h3 class="font-semibold text-slate-600 mb-2">{"Shard Configuration"}</h3>
                        <p class="text-3xl font-bold text-slate-800">{format!("{}+{}", status.data_shards, status.parity_shards)}</p>
                        <p class="text-gray-500">{"Data + Parity Shards"}</p>
                    </div>
                     <div class="bg-white p-5 rounded-lg shadow-md">
                        <h3 class="font-semibold text-slate-600 mb-2">{"Protected Files"}</h3>
                        <p class="text-3xl font-bold text-slate-800">{status.protected_files}</p>
                        <p class="text-gray-500">{"out of "} {status.total_files} {" total files"}</p>
                    </div>
                     <div class="bg-white p-5 rounded-lg shadow-md">
                        <h3 class="font-semibold text-slate-600 mb-2">{"Last Check"}</h3>
                        <p class="text-xl font-bold text-slate-800">{status.last_check_time.clone().unwrap_or("Never".to_string())}</p>
                        <p class="text-gray-500">{status.last_check_result.clone()}</p>
                    </div>
                </div>

                // --- Monitored Directories & Logs ---
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                    <div class="bg-white p-5 rounded-lg shadow-md">
                        <h3 class="font-semibold text-slate-600 mb-2">{"Monitored Directories"}</h3>
                        <ul class="list-disc list-inside text-gray-700 space-y-1">
                            { for status.watched_dirs.iter().map(|dir| html!{ <li><code class="bg-slate-100 rounded px-1">{dir}</code></li> }) }
                        </ul>
                    </div>
                     <div class="bg-white p-5 rounded-lg shadow-md">
                        <h3 class="font-semibold text-slate-600 mb-2">{"Live Logs"}</h3>
                        <div class="bg-gray-800 text-white font-mono text-sm rounded p-3 h-64 overflow-y-auto">
                            { for status.logs.iter().map(|log| html!{ <p>{log}</p> }) }
                        </div>
                    </div>
                </div>
            </main>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
