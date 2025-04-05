use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::navbar::Navbar;
use crate::components::dashboard::Dashboard;
use crate::components::practice::Practice;
use crate::components::feedback::FeedbackView;
use crate::components::home::HomePage;

#[component]
pub fn App() -> impl IntoView {
    // Set up metadata
    provide_meta_context();

    view! {
        <Stylesheet id="main-stylesheet" href="/public/styles.css"/>
        <Title text="Gengo - AI Language Learning"/>
        
        <Router>
            <Navbar/>
            <main>
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/dashboard" view=Dashboard/>
                    <Route path="/practice" view=move || view! { <Practice /> }/>
                    <Route path="/practice/:id/feedback" view=FeedbackView/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
            <Footer/>
        </Router>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <div class="container">
            <div class="not-found">
                <h1>"404 - Page Not Found"</h1>
                <p>"The page you're looking for doesn't exist."</p>
                <a href="/" class="btn">"Go Home"</a>
            </div>
        </div>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="footer">
            <div class="container">
                <h2 class="h2-heading">"Start your language journey today"</h2>
                <div class="flex-horizontal x-space-between tablet-flex-vertical tablet-flex-x-center flex-gap-sm utility-margin-top-3rem">
                    <a href="#" class="footer-link">
                        <div>"contact@gengo.app"</div>
                    </a>
                    <ul class="flex-horizontal y-center x-center flex-gap-xs wrap utility-margin-bottom-0 utility-padding-all-0">
                        <li class="utility-margin-bottom-0">
                            <a href="#" class="footer-link">
                                <div>"Explore"</div>
                            </a>
                        </li>
                        <li class="utility-margin-bottom-0">
                            <a href="#" class="footer-link">
                                <div>"Connect"</div>
                            </a>
                        </li>
                    </ul>
                </div>
            </div>
        </footer>
    }
}
