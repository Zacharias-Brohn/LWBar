use wayland_client::{
    protocol::{wl_compositor, wl_surface},
    Connection, Proxy, QueueHandle,
};
use wayland_protocols_wlr::layer_shell::v1::client::{
    zwlr_layer_shell_v1::{self, ZwlrLayerShellV1},
    zwlr_layer_surface_v1::{self, ZwlrLayerSurfaceV1},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Wayland display
    let conn = Connection::connect_to_env()?;
    let mut event_queue = conn.new_event_queue();
    let qh = event_queue.handle();
    let globals = conn.display_handle();
    
    // Get required globals
    let compositor = globals.bind::<wl_compositor::WlCompositor, _, _>(&qh, 4..=5, ())?;
    let layer_shell = globals.bind::<ZwlrLayerShellV1, _, _>(&qh, 1..=1, ())?;

    // Create surface
    let surface = compositor.create_surface(&qh, ());

    // Create layer surface
    let layer_surface = layer_shell.get_layer_surface(
        &surface,
        None,
        zwlr_layer_shell_v1::Layer::Top,
        "status-bar".to_string(),
    );

    // Configure layer surface
    layer_surface.set_size(0, 30);
    layer_surface.set_anchor(
        zwlr_layer_surface_v1::Anchor::Top
        | zwlr_layer_surface_v1::Anchor::Left
        | zwlr_layer_surface_v1::Anchor::Right,
    );
    layer_surface.set_exclusive_zone(30);
    
    // Commit the surface
    surface.commit();

    // Event loop
    loop {
        event_queue.blocking_dispatch(())?;
    }
}