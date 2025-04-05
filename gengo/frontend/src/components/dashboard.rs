use leptos::*;

#[component]
pub fn Dashboard() -> impl IntoView {
    // Mock data for now
    let (sessions, _) = create_signal(vec![
        ("Yesterday", "Greetings", 85),
        ("2 days ago", "Food vocabulary", 72),
    ]);
    
    view! {
        <div class="dashboard-page">
            <h1>"Your Language Dashboard"</h1>
            
            <div class="practice-section">
                <h2>"Start Practicing"</h2>
                <div class="practice-options">
                    <a href="/practice?topic=greetings" class="practice-card">
                        <h3>"Greetings & Introductions"</h3>
                        <p>"Practice everyday conversations"</p>
                    </a>
                    <a href="/practice?topic=travel" class="practice-card">
                        <h3>"Travel Phrases"</h3>
                        <p>"Learn essential travel vocabulary"</p>
                    </a>
                </div>
            </div>
            
            <div class="recent-sessions">
                <h2>"Recent Sessions"</h2>
                <div class="sessions-list">
                    {move || sessions.get().into_iter().map(|(date, topic, score)| {
                        view! {
                            <div class="session-item">
                                <div class="session-info">
                                    <span class="session-date">{date}</span>
                                    <span class="session-topic">{topic}</span>
                                </div>
                                <div class="session-score">
                                    <span class="score-value">{score}"%"</span>
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}
