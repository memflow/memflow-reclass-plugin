use glium::glutin;
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::platform::desktop::EventLoopExtDesktop;
use glium::glutin::window::WindowBuilder;
use glium::{Display, Surface};
use imgui::{Context, FontConfig, FontGlyphRanges, FontSource, Ui};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::time::Instant;

const CLEAR_COLOR: [f32; 4] = [0.25, 0.25, 0.25, 1.0];

pub fn show_window<F: FnMut(&mut bool, &mut Ui)>(
    title: &str,
    width: f32,
    height: f32,
    mut run_ui: F,
) {
    let title = match title.rfind('/') {
        Some(idx) => title.split_at(idx + 1).1,
        None => title,
    };
    let mut event_loop = EventLoop::new();
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let builder = WindowBuilder::new()
        .with_title(title.to_owned())
        .with_inner_size(glutin::dpi::LogicalSize::new(width as f64, height as f64))
        .with_visible(true)
        .with_always_on_top(true)
        .with_resizable(false);
    let display =
        Display::new(builder, context, &event_loop).expect("Failed to initialize display");

    let mut imgui = Context::create();
    imgui.set_ini_filename(None);

    // styling
    {
        let style = imgui.style_mut();
        style.window_border_size = 0f32;

        style.colors[imgui::StyleColor::Text as usize] = [1.00, 1.00, 1.00, 1.00];
        style.colors[imgui::StyleColor::TextDisabled as usize] = [0.40, 0.40, 0.40, 1.00];
        style.colors[imgui::StyleColor::ChildBg as usize] = [0.25, 0.25, 0.25, 1.00];
        style.colors[imgui::StyleColor::WindowBg as usize] = CLEAR_COLOR;
        style.colors[imgui::StyleColor::PopupBg as usize] = [0.25, 0.25, 0.25, 1.00];
        style.colors[imgui::StyleColor::Border as usize] = [0.12, 0.12, 0.12, 0.71];
        style.colors[imgui::StyleColor::BorderShadow as usize] = [1.00, 1.00, 1.00, 0.06];
        style.colors[imgui::StyleColor::FrameBg as usize] = [0.42, 0.42, 0.42, 0.54];
        style.colors[imgui::StyleColor::FrameBgHovered as usize] = [0.42, 0.42, 0.42, 0.40];
        style.colors[imgui::StyleColor::FrameBgActive as usize] = [0.56, 0.56, 0.56, 0.67];
        style.colors[imgui::StyleColor::TitleBg as usize] = [0.19, 0.19, 0.19, 1.00];
        style.colors[imgui::StyleColor::TitleBgActive as usize] = [0.22, 0.22, 0.22, 1.00];
        style.colors[imgui::StyleColor::TitleBgCollapsed as usize] = [0.17, 0.17, 0.17, 0.90];
        style.colors[imgui::StyleColor::MenuBarBg as usize] = [0.335, 0.335, 0.335, 1.000];
        style.colors[imgui::StyleColor::ScrollbarBg as usize] = [0.24, 0.24, 0.24, 0.53];
        style.colors[imgui::StyleColor::ScrollbarGrab as usize] = [0.41, 0.41, 0.41, 1.00];
        style.colors[imgui::StyleColor::ScrollbarGrabHovered as usize] = [0.52, 0.52, 0.52, 1.00];
        style.colors[imgui::StyleColor::ScrollbarGrabActive as usize] = [0.76, 0.76, 0.76, 1.00];
        style.colors[imgui::StyleColor::CheckMark as usize] = [0.65, 0.65, 0.65, 1.00];
        style.colors[imgui::StyleColor::SliderGrab as usize] = [0.52, 0.52, 0.52, 1.00];
        style.colors[imgui::StyleColor::SliderGrabActive as usize] = [0.64, 0.64, 0.64, 1.00];
        style.colors[imgui::StyleColor::Button as usize] = [0.54, 0.54, 0.54, 0.35];
        style.colors[imgui::StyleColor::ButtonHovered as usize] = [0.52, 0.52, 0.52, 0.59];
        style.colors[imgui::StyleColor::ButtonActive as usize] = [0.76, 0.76, 0.76, 1.00];
        style.colors[imgui::StyleColor::Header as usize] = [0.38, 0.38, 0.38, 1.00];
        style.colors[imgui::StyleColor::HeaderHovered as usize] = [0.47, 0.47, 0.47, 1.00];
        style.colors[imgui::StyleColor::HeaderActive as usize] = [0.76, 0.76, 0.76, 0.77];
        style.colors[imgui::StyleColor::Separator as usize] = [0.000, 0.000, 0.000, 0.137];
        style.colors[imgui::StyleColor::SeparatorHovered as usize] = [0.700, 0.671, 0.600, 0.290];
        style.colors[imgui::StyleColor::SeparatorActive as usize] = [0.702, 0.671, 0.600, 0.674];
        style.colors[imgui::StyleColor::ResizeGrip as usize] = [0.26, 0.59, 0.98, 0.25];
        style.colors[imgui::StyleColor::ResizeGripHovered as usize] = [0.26, 0.59, 0.98, 0.67];
        style.colors[imgui::StyleColor::ResizeGripActive as usize] = [0.26, 0.59, 0.98, 0.95];
        style.colors[imgui::StyleColor::PlotLines as usize] = [0.61, 0.61, 0.61, 1.00];
        style.colors[imgui::StyleColor::PlotLinesHovered as usize] = [1.00, 0.43, 0.35, 1.00];
        style.colors[imgui::StyleColor::PlotHistogram as usize] = [0.90, 0.70, 0.00, 1.00];
        style.colors[imgui::StyleColor::PlotHistogramHovered as usize] = [1.00, 0.60, 0.00, 1.00];
        style.colors[imgui::StyleColor::TextSelectedBg as usize] = [0.73, 0.73, 0.73, 0.35];
        style.colors[imgui::StyleColor::ModalWindowDimBg as usize] = [0.80, 0.80, 0.80, 0.35];
        style.colors[imgui::StyleColor::DragDropTarget as usize] = [1.00, 1.00, 0.00, 0.90];
        style.colors[imgui::StyleColor::NavHighlight as usize] = [0.26, 0.59, 0.98, 1.00];
        style.colors[imgui::StyleColor::NavWindowingHighlight as usize] = [1.00, 1.00, 1.00, 0.70];
        style.colors[imgui::StyleColor::NavWindowingDimBg as usize] = [0.80, 0.80, 0.80, 0.20];
    }

    let mut platform = WinitPlatform::init(&mut imgui);
    {
        let gl_window = display.gl_window();
        let window = gl_window.window();
        platform.attach_window(imgui.io_mut(), window, HiDpiMode::Rounded);
    }

    let hidpi_factor = platform.hidpi_factor();
    let font_size = (13.0 * hidpi_factor) as f32;
    imgui.fonts().add_font(&[
        FontSource::DefaultFontData {
            config: Some(FontConfig {
                size_pixels: font_size,
                ..FontConfig::default()
            }),
        },
        FontSource::TtfData {
            data: include_bytes!("../../resources/mplus-1p-regular.ttf"),
            size_pixels: font_size,
            config: Some(FontConfig {
                rasterizer_multiply: 1.75,
                glyph_ranges: FontGlyphRanges::japanese(),
                ..FontConfig::default()
            }),
        },
    ]);

    imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

    let mut renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

    let mut last_frame = Instant::now();

    event_loop.run_return(|event, _, control_flow| match event {
        Event::NewEvents(_) => {
            let now = Instant::now();
            imgui.io_mut().update_delta_time(now - last_frame);
            last_frame = now;
        }
        Event::MainEventsCleared => {
            let gl_window = display.gl_window();
            platform
                .prepare_frame(imgui.io_mut(), gl_window.window())
                .expect("Failed to prepare frame");
            gl_window.window().request_redraw();
        }
        Event::RedrawRequested(_) => {
            let mut ui = imgui.frame();

            let mut run = true;
            run_ui(&mut run, &mut ui);
            if !run {
                let gl_window = display.gl_window();
                let window = gl_window.window();
                window.set_visible(false);
                *control_flow = ControlFlow::Exit;
            }

            let gl_window = display.gl_window();
            let mut target = display.draw();
            target.clear_color_srgb(
                CLEAR_COLOR[0],
                CLEAR_COLOR[1],
                CLEAR_COLOR[2],
                CLEAR_COLOR[3],
            );
            platform.prepare_render(&ui, gl_window.window());
            let draw_data = ui.render();
            renderer
                .render(&mut target, draw_data)
                .expect("Rendering failed");
            target.finish().expect("Failed to swap buffers");
        }
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            let gl_window = display.gl_window();
            let window = gl_window.window();
            window.set_visible(false);
            *control_flow = ControlFlow::Exit;
        }
        event => {
            let gl_window = display.gl_window();
            platform.handle_event(imgui.io_mut(), gl_window.window(), &event);
        }
    });
}
