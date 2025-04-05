use leptos::*;
use wasm_bindgen::prelude::*;
fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

#[component]
pub fn AudioRecorder() -> impl IntoView {
    let (recording, set_recording) = create_signal(false);
    let (audio_url, set_audio_url) = create_signal::<Option<String>>(None);
    let (visualization_data, set_visualization_data) = create_signal(vec![10, 20, 15, 30, 25, 40, 35, 50, 45]);
    
    // Animation effect for visualization bars
    let visualizer_view = move || {
        visualization_data.get().into_iter().enumerate().map(|(i, height)| {
            let adjusted_height = if recording.get() {
                // Randomize heights when recording to simulate audio visualization
                (height as f64 * (1.0 + (js_sys::Math::random() * 1.2))).round() as usize
            } else {
                height
            };
            
            view! {
                <div 
                    class="visualizer-bar"
                    style:height=format!("{}px", adjusted_height)
                    style:animation-delay=format!("{}ms", i * 100)
                ></div>
            }
        }).collect::<Vec<_>>()
    };
    
    // Start recording with animation
    let start_recording = move |_| {
        set_recording.set(true);
        
        // This would be replaced with actual recording logic
        create_effect(move |_| {
            if recording.get() {
                let closure = Closure::wrap(Box::new(move || {
                    // Update visualization in real-time
                    set_visualization_data.update(|data| {
                        *data = data.iter().map(|&h| {
                            let random = js_sys::Math::random() * 40.0;
                            (10.0 + random) as usize
                        }).collect::<Vec<_>>();
                    });
                }) as Box<dyn Fn()>);

                let handle = window().set_interval_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    1000,
                );
                closure.forget(); // Prevent the closure from being dropped
                
                on_cleanup(move || {
                    window().clear_interval_with_handle(handle);
                });
            }
        });
    };
    
    // Stop recording with animation
    let stop_recording = move |_| {
        set_recording.set(false);
        set_audio_url.set(Some("example-recording.mp3".to_string()));
    };
    
    view! {
        <div class="recorder-container card">
            <h3 class="recorder-title">
                <span class="primary-text">Record</span> 
                <span class="accent-text">Your Speech</span>
            </h3>
            
            <div class="visualizer-container">
                <div class=move || format!("visualizer {}", if recording.get() { "active" } else { "" })>
                    {visualizer_view}
                </div>
                
                {move || if recording.get() {
                    view! {
                        <div class="recording-indicator">
                            <div class="pulse"></div>
                            <span>Recording...</span>
                        </div>
                    }
                } else {
                    view! { <div class="recording-placeholder">Ready to record</div> }
                }}
            </div>
            
            <div class="recorder-controls">
                {move || if recording.get() {
                    view! {
                        <button class="btn btn-stop" on:click=stop_recording>
                            <span class="btn-icon">"⏹"</span>
                            <span class="btn-text">Stop</span>
                        </button>
                    }
                } else if audio_url.get().is_some() {
                    view! {
                        <div class="btn-group">
                            <button class="btn btn-secondary" on:click=move |_| set_audio_url.set(None)>        
                                <span class="btn-text">Record Again</span>
                            </button>
                            <button class="btn btn-primary">
                                <span class="btn-text">Submit</span>
                            </button>
                        </div>
                    }
                } else {
                    view! {
                        <div class="btn-group">
                            <button class="btn btn-record" on:click=start_recording>
                                <span class="btn-icon">"⏺"</span>
                                <span class="btn-text">Start Recording</span>
                            </button>
                        </div>
                    }
                }}
            </div>
            
            {move || audio_url.get().map(|url| {
                view! {
                    <div class="audio-player fade-in">
                        <audio controls src={url}></audio>
                    </div>
                }
            })}
        </div>
    }
}