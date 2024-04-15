use tutorial4_buffer::run;

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.5, 0.5, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // A
    Vertex {
        position: [-0.49513406, 0.06958647, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // B
    Vertex {
        position: [-0.21918549, -0.44939706, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // C
    Vertex {
        position: [0.35966998, -0.3473291, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // D
    Vertex {
        position: [0.44147372, 0.2347359, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // E
];

const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];

fn main() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Could't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let color = [0.5, 0.0, 0.5];
    let vertex_pos = vec![
        [-0.0868241, 0.49240386, 0.0],
        [-0.49513406, 0.06958647, 0.0],
        [-0.21918549, -0.44939706, 0.0],
        [0.35966998, -0.3473291, 0.0],
        [0.44147372, 0.2347359, 0.0],
    ];
    let mut vertices = vec![];
    for pos in vertex_pos {
        vertices.append(
            Vertex{
                position: pos,
                color,
            }
        )
    }

    let mut state = State::new(window).await;
    pollster::block_on(state.run());
}
