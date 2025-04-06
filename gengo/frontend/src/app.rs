
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// Import all page-level and shared components
use crate::components::navbar::Navbar;
use crate::components::dashboard::Dashboard;
use crate::components::practice::Practice;
use crate::components::feedback::FeedbackView;
use crate::components::home::HomePage;

/// App - Root component that defines application structure and routing
/// 
/// Sets up:
/// - Global metadata and styling
/// - Client-side routing
/// - Main layout structure (navbar, content area, footer)
#[component]
pub fn App() -> impl IntoView {
    // Initialize metadata context for managing document head
    provide_meta_context();

    view! {
        // Global styles and document title
        <Stylesheet id="main-stylesheet" href="/public/styles.css"/>
        <Title text="Gengo - AI Language Learning"/>
        
        // Router component manages client-side navigation
        <Router>
            // Global navigation bar - appears on all pages
            <Navbar/>
            
            // Main content area
            <main>
                // Routes define the application's URL structure and corresponding components
                <Routes>
                    // Path: / - Landing page for new users
                    <Route path="/" view=HomePage/>
                    
                    // Path: /dashboard - User dashboard showing progress and activities
                    <Route path="/dashboard" view=Dashboard/>
                    
                    // Path: /practice - Practice session page with audio recording
                    <Route path="/practice" view=move || view! { <Practice /> }/>
                    
                    // Path: /practice/:id/feedback - Feedback page after practice session
                    // The :id parameter captures the practice session ID
                    <Route path="/practice/:id/feedback" view=FeedbackView/>
                    
                    // Catch-all route for 404 handling - matches any undefined path
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
            
            // Global footer - appears on all pages
            <Footer/>
        </Router>
    }
}

/// NotFound - 404 error page component
/// 
/// Displayed when user navigates to a URL that doesn't match any defined routes
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

/// 
/// Contains:
/// - Call to action
/// - Contact information
/// - Navigation links
#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="footer">
            <div class="container">
                // Call to action heading
                <h2 class="h2-heading">"Start your language journey today"</h2>
                
                // Footer content with responsive layout
                <div class="flex-horizontal x-space-between tablet-flex-vertical tablet-flex-x-center flex-gap-sm utility-margin-top-3rem">
                    // Contact email
                    <a href="#" class="footer-link">
                        <div>"contact@gengo.app"</div>
                    </a>
                    
                    // Footer navigation links
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
