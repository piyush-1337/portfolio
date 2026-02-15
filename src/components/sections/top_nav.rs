use leptos::{ev, prelude::*};
use std::rc::Rc;

#[component]
pub fn TopNav() -> impl IntoView {
    let active_section = RwSignal::new("about");

    let update_active_section: Rc<dyn Fn()> = Rc::new(move || {
        let marker = 140.0;
        let sections = ["about", "experience", "skills", "projects"];
        let doc = document();

        for id in sections {
            if let Some(el) = doc.get_element_by_id(id) {
                let rect = el.get_bounding_client_rect();
                if rect.top() <= marker && rect.bottom() > marker {
                    active_section.set(id);
                    return;
                }
            }
        }

        if let Some(last) = sections.last() {
            active_section.set(*last);
        }
    });

    update_active_section.as_ref()();

    let updater_scroll = update_active_section.clone();
    let scroll_listener = window_event_listener(ev::scroll, move |_| {
        updater_scroll.as_ref()();
    });

    let updater_resize = update_active_section.clone();
    let resize_listener = window_event_listener(ev::resize, move |_| {
        updater_resize.as_ref()();
    });

    let updater_load = update_active_section.clone();
    let load_listener = window_event_listener(ev::load, move |_| {
        updater_load.as_ref()();
    });

    std::mem::forget(scroll_listener);
    std::mem::forget(resize_listener);
    std::mem::forget(load_listener);

    view! {
        <header class="fixed top-3 z-50 w-full px-3 md:top-5">
            <nav class="nav-float-wrap mx-auto max-w-3xl">
                <ul class="grid grid-cols-4 gap-1 p-0.25 text-xs md:text-sm">
                    <li>
                        <a
                            class=move || {
                                format!(
                                    "nav-float-link {}",
                                    if active_section.get() == "about" { "is-active" } else { "" },
                                )
                            }
                            href="#about"
                        >
                            "About"
                        </a>
                    </li>
                    <li>
                        <a
                            class=move || {
                                format!(
                                    "nav-float-link {}",
                                    if active_section.get() == "experience" {
                                        "is-active"
                                    } else {
                                        ""
                                    },
                                )
                            }
                            href="#experience"
                        >
                            "Education"
                        </a>
                    </li>
                    <li>
                        <a
                            class=move || {
                                format!(
                                    "nav-float-link {}",
                                    if active_section.get() == "skills" { "is-active" } else { "" },
                                )
                            }
                            href="#skills"
                        >
                            "Skills"
                        </a>
                    </li>
                    <li>
                        <a
                            class=move || {
                                format!(
                                    "nav-float-link {}",
                                    if active_section.get() == "projects" {
                                        "is-active"
                                    } else {
                                        ""
                                    },
                                )
                            }
                            href="#projects"
                        >
                            "Projects"
                        </a>
                    </li>
                </ul>
            </nav>
        </header>
    }
}
