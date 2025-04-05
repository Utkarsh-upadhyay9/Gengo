use leptos::*;
use crate::components::recorder::AudioRecorder;

#[component]
pub fn Practice() -> impl IntoView {
    let (current_step, set_current_step) = create_signal(1);
    let (selected_level, set_selected_level) = create_signal("intermediate");
    let (selected_exercise, set_selected_exercise) = create_signal(0);
    
    let exercises = vec![
        "Basic Pronunciation", 
        "Daily Conversation", 
        "Travel Phrases", 
        "Business Meeting"
    ];
    
    let exercises_list = create_memo(move |_| {
        exercises.iter().enumerate().map(|(idx, title)| (idx, title.clone())).collect::<Vec<_>>()
    });
    
    // Go to next step with animation
    let next_step = move |_| {
        set_current_step.update(|step| *step += 1);
    };
    
    // Go back to previous step
    let prev_step = move |_| {
        set_current_step.update(|step| if *step > 1 { *step -= 1 } );
    };
    
    // Select an exercise
    let select_exercise = move |idx: usize| {
        set_selected_exercise.set(idx);
    };
    
    view! {
        <div class="practice-container">
            <div class="progress-bar">
                <div class="progress-steps">
                    <div class=move || format!("step {}", if current_step.get() >= 1 { "active" } else { "" })>
                        <div class="step-number">1</div>
                        <div class="step-label">Choose Level</div>
                    </div>
                    <div class=move || format!("step {}", if current_step.get() >= 2 { "active" } else { "" })>
                        <div class="step-number">2</div>
                        <div class="step-label">Select Exercise</div>
                    </div>
                    <div class=move || format!("step {}", if current_step.get() >= 3 { "active" } else { "" })>
                        <div class="step-number">3</div>
                        <div class="step-label">Practice</div>
                    </div>
                    <div class=move || format!("step {}", if current_step.get() >= 4 { "active" } else { "" })>
                        <div class="step-number">4</div>
                        <div class="step-label">Feedback</div>
                    </div>
                </div>
                <div class="progress-line">
                    <div 
                        class="progress-fill" 
                        style=move || format!("width: {}%", (current_step.get() - 1) * 33)
                    ></div>
                </div>
            </div>
            
            <div class="practice-content">
                {move || match current_step.get() {
                    1 => view! {
                        <div class="step-content fade-in">
                            <h2 class="step-title">Choose Your Level</h2>
                            <div class="level-selection">
                                <button 
                                    class=move || format!("level-btn {}", if selected_level.get() == "beginner" { "active" } else { "" })
                                    on:click=move |_| set_selected_level.set("beginner")
                                >
                                    <div class="level-icon beginner"></div>
                                    <h3>Beginner</h3>
                                    <p>For those just starting out</p>
                                </button>
                                
                                <button 
                                    class=move || format!("level-btn {}", if selected_level.get() == "intermediate" { "active" } else { "" })
                                    on:click=move |_| set_selected_level.set("intermediate")
                                >
                                    <div class="level-icon intermediate"></div>
                                    <h3>Intermediate</h3>
                                    <p>For conversational speakers</p>
                                </button>
                                
                                <button 
                                    class=move || format!("level-btn {}", if selected_level.get() == "advanced" { "active" } else { "" })
                                    on:click=move |_| set_selected_level.set("advanced")
                                >
                                    <div class="level-icon advanced"></div>
                                    <h3>Advanced</h3>
                                    <p>For fluent speakers</p>
                                </button>
                            </div>
                            <div class="step-actions">
                                <button class="btn btn-primary btn-next" on:click=next_step>
                                    Continue
                                </button>
                            </div>
                        </div>
                    },
                    2 => view! {
                        <div class="step-content fade-in">
                            <h2 class="step-title">Select an Exercise</h2>
                            <div class="exercise-list">
                                {move || exercises_list.get().into_iter().map(|(idx, title)| {
                                    let idx_clone = idx;
                                    view! {
                                        <button 
                                            class=move || format!("exercise-item {}", 
                                                if selected_exercise.get() == idx { "active" } else { "" })
                                            on:click=move |_| select_exercise(idx_clone)
                                        >
                                            <div class="exercise-number">{idx + 1}</div>
                                            <div class="exercise-details">
                                                <h3 class="exercise-title">{title}</h3>
                                                <p class="exercise-description">Practice your pronunciation with common phrases</p>
                                            </div>
                                            <div class="exercise-duration">5-10 min</div>
                                        </button>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                            <div class="step-actions">
                                <button class="btn btn-outline" on:click=prev_step>
                                    Back
                                </button>
                                <button class="btn btn-primary btn-next" on:click=next_step>
                                    Start Exercise
                                </button>
                            </div>
                        </div>
                    },
                    3 => view! {
                        <div class="step-content fade-in">
                            <h2 class="step-title">Practice Speaking</h2>
                            <div class="practice-exercise">
                                <div class="exercise-prompt card">
                                    <h3>Repeat this phrase:</h3>
                                    <p class="prompt-text">"The weather is very nice today, isn't it?"</p>
                                    <button class="btn btn-small">
                                        <span class="btn-icon"> "▶" </span>
                                        <span class="btn-text">Listen</span>
                                    </button>
                                </div>
                                
                                <AudioRecorder />
                            </div>
                            <div class="step-actions">
                                <button class="btn btn-outline" on:click=prev_step>
                                    Back
                                </button>
                                <button class="btn btn-primary" on:click=next_step>
                                    Get Feedback
                                </button>
                            </div>
                        </div>
                    },
                    _ => view! {
                        <div class="step-content fade-in">
                            <h2 class="step-title">Your Feedback</h2>
                            <div class="feedback-container">
                                <div class="feedback-score-card">
                                    <div class="score-circle">
                                        <div class="score-number">85</div>
                                        <svg viewBox="0 0 36 36" class="score-chart">
                                            <path class="score-circle-bg"
                                                d="M18 2.0845
                                                  a 15.9155 15.9155 0 0 1 0 31.831
                                                  a 15.9155 15.9155 0 0 1 0 -31.831"
                                            />
                                            <path class="score-circle-fill"
                                                stroke-dasharray="85, 100"
                                                d="M18 2.0845
                                                  a 15.9155 15.9155 0 0 1 0 31.831
                                                  a 15.9155 15.9155 0 0 1 0 -31.831"
                                            />
                                        </svg>
                                    </div>
                                    <div class="score-details">
                                        <h3>Very Good!</h3>
                                        <p>Your pronunciation is clear with minor improvements needed.</p>
                                    </div>
                                </div>
                                
                                <div class="feedback-details card">
                                    <h3>Detailed Feedback</h3>
                                    <div class="feedback-areas">
                                        <div class="feedback-area">
                                            <div class="area-title">
                                                <span class="area-name">Pronunciation</span>
                                                <span class="area-score good">90%</span>
                                            </div>
                                            <div class="progress-bar">
                                                <div class="progress-fill" style="width: 90%"></div>
                                            </div>
                                            <p class="area-comment">Excellent clarity on most words. Work on "weather" pronunciation.</p>
                                        </div>
                                        
                                        <div class="feedback-area">
                                            <div class="area-title">
                                                <span class="area-name">Intonation</span>
                                                <span class="area-score good">85%</span>
                                            </div>
                                            <div class="progress-bar">
                                                <div class="progress-fill" style="width: 85%"></div>
                                            </div>
                                            <p class="area-comment">Good rise in tone for the question. Continue practicing question intonation.</p>
                                        </div>
                                        
                                        <div class="feedback-area">
                                            <div class="area-title">
                                                <span class="area-name">Fluency</span>
                                                <span class="area-score average">75%</span>
                                            </div>
                                            <div class="progress-bar">
                                                <div class="progress-fill" style="width: 75%"></div>
                                            </div>
                                            <p class="area-comment">Good pace but slight pause before "isn't it". Try to make it more natural.</p>
                                        </div>
                                    </div>
                                </div>
                            </div>
                            <div class="step-actions">
                                <button class="btn btn-outline" on:click=prev_step>
                                    Try Again
                                </button>
                                <button class="btn btn-primary">
                                    Next Exercise
                                </button>
                            </div>
                        </div>
                    }
                }}
            </div>
        </div>
    }
}
