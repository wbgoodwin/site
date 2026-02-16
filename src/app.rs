use yew::prelude::*;

#[function_component(Header)]
fn header() -> Html {
    html! {
        <header class="header">
            <a href="/" class="logo">{ "wbgood" }</a>
            <nav class="nav-links">
                <a href="#home" class="nav-link">{ "HOME" }</a>
                <a href="#about" class="nav-link">{ "ABOUT" }</a>
                <a href="#projects" class="nav-link">{ "PROJECTS" }</a>
            </nav>
            <div class="social-icons">
                <a href="https://www.linkedin.com/in/william-goodwin-b7924a121?utm_source=share_via&utm_content=profile&utm_medium=member_ios" class="social-icon-link" target="_blank" rel="noopener noreferrer">
                    <svg class="social-icon" viewBox="0 0 24 24">
                        <path d="M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433c-1.144 0-2.063-.926-2.063-2.065 0-1.138.92-2.063 2.063-2.063 1.14 0 2.064.925 2.064 2.063 0 1.139-.925 2.065-2.064 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z"/>
                    </svg>
                </a>
            </div>
        </header>
    }
}

#[function_component(MainContent)]
fn main_content() -> Html {
    html! {
        <main class="main-content">
            <h1 class="title">{ "SOFTWARE ENGINEERING" }</h1>
            <p class="subtitle">
                { "Senior software engineer specializing in end-to-end application development, system architecture, and scalable solutions." }
            </p>
            <div class="buttons">
                <button class="btn btn-secondary">{ "About wbgood" }</button>
            </div>
        </main>
    }
}

#[function_component(Terminal)]
fn terminal() -> Html {
    html! {
        <section class="terminal">
            <div class="terminal-header">
                <div class="terminal-dots">
                    <div class="dot dot-red"></div>
                    <div class="dot dot-yellow"></div>
                    <div class="dot dot-green"></div>
                </div>
                <div class="terminal-title">{ "software.terminal" }</div>
            </div>
            <div class="terminal-content">
                <div>
                    <span class="terminal-prompt">{ "wbgood$" }</span>
                    <span class="terminal-text">{ " Fullstack Development Console with Modern Tools & Frameworks" }</span>
                </div>
            </div>
        </section>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="app">
            <Header />
            <MainContent />
            <Terminal />
        </div>
    }
}
