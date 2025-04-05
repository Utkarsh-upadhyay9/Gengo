use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="hero">
            <div class="container">
                <h1>"Enhance your language skills with AI"</h1>
                <p class="subheading">
                    "Join our community for fun practice sessions and real-time feedback. 
                    Track your progress and improve your speaking effortlessly."
                </p>
                <div class="button-group">
                    <a href="/dashboard" class="button">"Get started"</a>
                    <a href="#features" class="button secondary-button">"Learn more"</a>
                </div>
            </div>
        </div>

        <section class="section" id="features">
            <div class="container">
                <h2 class="h2-heading">"Explore a new way to learn languages"</h2>
                <div class="eyebrow">"AI-Powered Learning"</div>
                
                <div class="practice-options">
                    <div class="feature-card">
                        <h3 class="h3-heading">"Perfect your pronunciation"</h3>
                        <p>"Get real-time feedback to speak clearly and confidently."</p>
                    </div>
                    
                    <div class="feature-card">
                        <h3 class="h3-heading">"Interactive speaking tasks"</h3>
                        <p>"Join activities that adapt to you, offering instant insights."</p>
                    </div>
                    
                    <div class="feature-card">
                        <h3 class="h3-heading">"Track your progress"</h3>
                        <p>"Use dashboards to see your growth and celebrate achievements."</p>
                    </div>
                </div>
            </div>
        </section>
        
        <section class="section">
            <div class="container">
                <div class="card utility-backdrop-filter-blur">
                    <div class="card-body">
                        <h2>"Boost your speaking confidence with AI"</h2>
                        <p class="paragraph-lg">"Leverage cutting-edge AI to refine your pronunciation and grammar with ease."</p>
                        <div class="button-group">
                            <a href="/practice" class="button">"Start practicing now"</a>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
