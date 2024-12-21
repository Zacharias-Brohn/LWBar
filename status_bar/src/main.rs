// filepath: src/main.rs
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Label, Orientation};
use std::time::Duration;
use std::thread;
use std::sync::{Arc, Mutex};

trait Component {
    fn update(&self);
}

struct ClockComponent {
    label: Label,
}

impl ClockComponent {
    fn new() -> Self {
        let label = Label::new(None);
        ClockComponent { label }
    }
}

impl Component for ClockComponent {
    fn update(&self) {
        let label = self.label.clone();
        thread::spawn(move || loop {
            let current_time = chrono::Local::now().format("%H:%M:%S").to_string();
            glib::idle_add_local(move || {
                label.set_text(&current_time);
                glib::Continue(false)
            });
            thread::sleep(Duration::from_secs(1));
        });
    }
}

struct StatusBar {
    container: Box,
    components: Vec<Arc<Mutex<dyn Component>>>,
}

impl StatusBar {
    fn new() -> Self {
        let container = Box::new(Orientation::Horizontal, 0);
        StatusBar {
            container,
            components: Vec::new(),
        }
    }

    fn add_component(&mut self, component: Arc<Mutex<dyn Component>>) {
        self.components.push(component.clone());
        let component = component.lock().unwrap();
        if let Some(clock) = component.downcast_ref::<ClockComponent>() {
            self.container.pack_start(&clock.label, false, false, 0);
        }
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
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Status Bar");
        window.set_default_size(400, 50);

        let status_bar = Arc::new(Mutex::new(StatusBar::new()));
        let clock_component = Arc::new(Mutex::new(ClockComponent::new()));

        status_bar.lock().unwrap().add_component(clock_component.clone());
        status_bar.lock().unwrap().update_components();

        window.add(&status_bar.lock().unwrap().container);
        window.show_all();
    });

    application.run(&[]);
}