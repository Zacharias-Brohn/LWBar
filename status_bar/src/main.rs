// filepath: src/main.rs
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Label, Orientation};
use std::sync::{Arc, Mutex};
use chrono;
use glib::source::timeout_add_seconds_local;
use glib::Continue;
use gdk::{Screen, WindowTypeHint, Gravity}; // Import gdk::Screen
use gtk_layer_shell::Layer;

trait Component {
    fn update(&self);
    fn widget(&self) -> gtk::Widget;
}

struct ClockComponent {
    label: Label,
}

impl ClockComponent {
    fn new() -> Self {
        let label = Label::new(None);
        
        // Fix: Use style_context instead of get_style_context
        label.style_context().add_class("clock");
        
        label.set_halign(gtk::Align::End);
        label.set_margin_end(10);
        
        ClockComponent { label }
    }
}

impl Component for ClockComponent {
    fn update(&self) {
        let label = self.label.clone();
        timeout_add_seconds_local(1, move || {
            let current_time = chrono::Local::now().format("%H:%M:%S").to_string();
            label.set_text(&current_time);
            Continue(true)
        });
    }

    fn widget(&self) -> gtk::Widget {
        self.label.clone().upcast()
    }
}

struct StatusBar {
    container: Box,
    components: Vec<Arc<Mutex<dyn Component>>>,
}

impl StatusBar {
    fn new() -> Self {
        let container = Box::new(Orientation::Horizontal, 0);
        container.set_homogeneous(false);
        container.set_hexpand(true);
        
        StatusBar {
            container,
            components: Vec::new(),
        }
    }

    fn add_component(&mut self, component: Arc<Mutex<dyn Component>>) {
        // Fix: Clone Arc before locking
        let widget = {
            let comp = component.lock().unwrap();
            comp.widget()
        };
        
        widget.set_hexpand(true);
        self.container.add(&widget);
        
        // Fix: Add the original Arc
        self.components.push(component);
    }

    fn update_components(&self) {
        for component in &self.components {
            component.lock().unwrap().update();
        }
    }
}

fn main() {
    let application = Application::new(
        Some("com.example.status_bar"),
        Default::default(),
    );

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        
        // Initialize as layer shell surface
        gtk_layer_shell::init_for_window(&window);
        
        // Configure layer shell
        gtk_layer_shell::set_layer(&window, Layer::Top);
        gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Top, true);
        gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Left, true);
        gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Right, true);
        gtk_layer_shell::set_margin(&window, gtk_layer_shell::Edge::Top, 0);
        
        // Basic window setup
        window.set_title("Status Bar");
        window.set_decorated(false);

        if let Some(screen) = Screen::default() {
            if let Some(monitor) = screen.display().primary_monitor() {
                let geometry = monitor.geometry();
                let screen_width = geometry.width();
                let bar_height = 30;
                
                window.set_size_request(screen_width, bar_height);
                
                // Rest of the existing window setup...
                let css_provider = gtk::CssProvider::new();
                css_provider.load_from_data(b"
                    window {
                        background-color: rgba(0, 0, 0, 0.5);
                    }
                    window {
                        background-color: #2d2d2d;
                    }
                    .clock {
                        color: white;
                        font-size: 14px;
                        padding: 0 10px;
                    }
                ").unwrap();
                
                // Fix: Use GtkWindowExt trait explicitly
                if let Some(screen) = gtk::prelude::GtkWindowExt::screen(&window) {
                    gtk::StyleContext::add_provider_for_screen(
                        &screen,
                        &css_provider,
                        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
                    );
                }
            }
        } else {
            eprintln!("Error: Unable to get default screen.");
        }

        let status_bar = Arc::new(Mutex::new(StatusBar::new()));
        let clock_component = Arc::new(Mutex::new(ClockComponent::new()));

        status_bar.lock().unwrap().add_component(clock_component.clone());
        status_bar.lock().unwrap().update_components();

        window.add(&status_bar.lock().unwrap().container);
        window.show_all();
    });

    application.run();
}