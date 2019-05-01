#[macro_use] extern crate conrod_core;
extern crate conrod_glium;
extern crate conrod_winit;
extern crate find_folder;
extern crate glium;
extern crate image;

use glium::Surface;

mod support;
mod board;
mod piece;

fn main() {
    const WIDTH: u32 = 600;
    const HEIGHT: u32 = 600;

    let mut chess_board = board::ChessBoard::default();

    // Build the window.
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("RustyChess")
        .with_dimensions((WIDTH, HEIGHT).into());
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let display = support::GliumDisplayWinitWrapper(display);

    // construct our `Ui`.
    let mut ui = conrod_core::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();

    // A type used for converting `conrod_core::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    let mut renderer = conrod_glium::Renderer::new(&display.0).unwrap();

    // The image map describing each of our widget->image mappings (in our case, none).
    let mut image_map = conrod_core::image::Map::<glium::texture::Texture2d>::new();
    chess_board.init(&display.0, &mut image_map);

    // Instantiate the generated list of widget identifiers.
    let ids = &mut Ids::new(ui.widget_id_generator());

    // Poll events from the window.
    let mut event_loop = support::EventLoop::new();
    'main: loop {

        // Handle all events.
        for event in event_loop.next(&mut events_loop) {

            // Use the `winit` backend feature to convert the winit event to a conrod one.
            if let Some(event) = conrod_winit::convert_event(event.clone(), &display) {
                ui.handle_event(event);
                event_loop.needs_update();
            }

            match event {
                glium::glutin::Event::WindowEvent { event, .. } => match event {
                    // Break from the loop upon `Escape`.
                    glium::glutin::WindowEvent::CloseRequested |
                    glium::glutin::WindowEvent::KeyboardInput {
                        input: glium::glutin::KeyboardInput {
                            virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => break 'main,
                    _ => (),
                },
                _ => (),
            }
        }

        // Instantiate all widgets in the GUI.
        set_widgets(ui.set_widgets(), ids, &mut chess_board);

        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display.0, primitives, &image_map);
            let mut target = display.0.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display.0, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}

// Draw the Ui.
fn set_widgets(ref mut ui: conrod_core::UiCell, ids: &mut Ids, chess_board: &mut board::ChessBoard) {
    use conrod_core::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget};

    // Construct our main `Canvas` tree.
    widget::Canvas::new().flow_down(&[
        (ids.header, widget::Canvas::new().color(color::BLUE).length(50.0)),
        (ids.body, widget::Canvas::new().length(500.0)),
        (ids.footer, widget::Canvas::new().color(color::BLUE).length(50.0))
    ]).set(ids.master, ui);

    widget::Text::new("Rusty Chess")
    .color(color::LIGHT_ORANGE)
    .font_size(25)
    .middle_of(ids.header)
    .set(ids.title, ui);

    widget::Text::new("Footer")
    .color(color::LIGHT_ORANGE)
    .font_size(25)
    .middle_of(ids.footer)
    .set(ids.footer_text, ui);

    let body_wh = ui.wh_of(ids.body).unwrap();

    //Create chess board background
    let mut background_tiles = widget::Matrix::new(8, 8)
        .w_h(body_wh[0], body_wh[1])
        .middle_of(ids.body)
        .set(ids.background_tiles, ui);
    while let Some(tile) = background_tiles.next(ui) {
        let (r, c) = (tile.row, tile.col);

        let start_color  = if r % 2 == 0 {1} else {0};
        let tile_color = if (c + start_color) % 2 == 0 {color::BLUE} else {color::LIGHT_ORANGE};

        let tile_background = widget::Rectangle::fill_with([50.0, 50.0], tile_color);

        tile.set(tile_background, ui);
    }

    //Create chess board pieces
    let mut piece_board = widget::Matrix::new(8, 8)
        .w_h(body_wh[0], body_wh[1])
        .middle_of(ids.body)
        .set(ids.chess_board, ui);
    while let Some(tile) = piece_board.next(ui) {
        let (r, c) = (tile.row, tile.col);
        match chess_board.fetch_piece_type(r, c) {
            Some(piece_type) => {
                let image_id = chess_board.fetch_piece_picture_id(&piece_type);

                let tile_button = widget::Button::image(image_id)
                                    .image_color_with_feedback(color::BLACK)
                                    .w_h(25.0, 25.0);

                for _click in tile.set(tile_button, ui) {
                    println!("Clicked on piece! {:?}", (r, c));
                }
            }
            None => {
                let tile_button = widget::Button::new()
                                    .color(color::TRANSPARENT);
                for _click in tile.set(tile_button, ui) {
                    println!("Clicked on empty! {:?}", (r, c));
                }
            }
        };
    }
}

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    struct Ids {
        master,
        header,
        body,
        footer,

        test_image,

        title,
        chess_board,
        background_tiles,
        footer_text
    }
}
