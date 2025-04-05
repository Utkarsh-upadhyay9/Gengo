use leptos::*;
use leptos_router::*;
use web_sys::Event;

#[component]
pub fn Navbar() -> impl IntoView {
    let (menu_open, set_menu_open) = create_signal(false);
    
    // Toggle mobile menu
    let toggle_menu = move |_| {
        set_menu_open.update(|open| *open = !*open);
    };
    
    // Close menu when clicking a link
    let close_menu = move |_| {
        set_menu_open.set(false);
    };
    
    // Scroll animation
    let handle_scroll = move |_: web_sys::Event| {
        // Existing scroll handling code
    };
    
    view! {
        <nav class="navbar">
            <div class="navbar-logo">
                <A href="/" class="logo-link">
                    <img src="/public/images/logo.svg" alt="Gengo Logo" />
                    <span class="logo-text">Gengo</span>
                </A>
            </div>
            
            <button 
                class=move || format!("navbar-toggle {}", if menu_open.get() { "active" } else { "" })
                on:click=toggle_menu
            >
                <span class="toggle-bar"></span>
                <span class="toggle-bar"></span>
                <span class="toggle-bar"></span>
            </button>
            
            <div class=move || format!("navbar-menu {}", if menu_open.get() { "active" } else { "" })>
                <div class="navbar-links">
                    <A href="/" class="nav-link" on:click=close_menu>Home</A>
                    <A href="/practice" class="nav-link" on:click=close_menu>Practice</A>
                    <A href="/dashboard" class="nav-link" on:click=close_menu>Dashboard</A>
                </div>
                <div class="navbar-auth">
                    <A href="/login" class="btn btn-secondary" on:click=close_menu>Log In</A>
                    <A href="/signup" class="btn btn-primary" on:click=close_menu>Sign Up</A>
                </div>
            </div>
        </nav>
    }
}
