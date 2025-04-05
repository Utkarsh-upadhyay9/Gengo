use leptos::*;

#[component]
pub fn FeedbackView() -> impl IntoView {
    // In a real app, get from params
    let _session_id = "example-id";
    
    view! {
        <div class="feedback-container">
            <h1>"Your Feedback"</h1>
            
            <div class="feedback-score">
                <div class="score-circle">
                    <span class="score-value">"85%"</span>
                </div>
                <p class="score-label">"Great job!"</p>
            </div>
            
            <div class="feedback-details">
                <h2>"Pronunciation Analysis"</h2>
                <p>"Your pronunciation was mostly correct. Focus on the 'r' sound in 'morning'."</p>
                
                <h2>"Suggested Improvements"</h2>
                <ul>
                    <li>"Practice the 'r' sound in isolation"</li>
                    <li>"Try speaking more slowly for better clarity"</li>
                </ul>
            </div>
            
            <div class="feedback-actions">
                <a href="/practice" class="btn btn-primary">"Practice Again"</a>
                <a href="/dashboard" class="btn btn-secondary">"Back to Dashboard"</a>
            </div>
        </div>
    }
}
