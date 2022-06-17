use minifb::{Key, ScaleMode, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn main() {
    let mut window = Window::new(
        "Noise Test - Press ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: false,
            scale_mode: ScaleMode::UpperLeft,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to create window");

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut buffer: Vec<u32> = Vec::with_capacity(WIDTH * HEIGHT);

    let mut size = (0, 0);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let new_size = (window.get_size().0, window.get_size().1);
        if new_size != size {
            size = new_size;
            buffer.resize(size.0 * size.1, 0);
            println!("window size is {}x{}", size.0, size.1);
        }

        buffer.fill(0);

        let left = size.0 - 1;
        let bottom = size.1 - 1;
        let center_x = left / 2;
        let center_y = bottom / 2;

        buffer[calc_buffer_index( &size, 0, 0 )] = u32_from_rgb( 0xff,0,0 );
        buffer[calc_buffer_index( &size, left, 0 )] = u32_from_rgb( 0, 0xff, 0);
        buffer[calc_buffer_index( &size, 0, bottom )] = u32_from_rgb(0xff, 0xff, 0);
        buffer[calc_buffer_index( &size, left, bottom )] = u32_from_rgb(0xff,0xff,0xff);

        for py in 0..size.1 {
            for px in 0..size.0 {
                let bufferIndex = calc_buffer_index( &size, px, py );
                
                let red = ( ( py as f32 / bottom as f32 ) * 255.0 ) as u32;
                let green = ( ( px as f32 / left as f32 ) * 255.0 ) as u32;
                //let blue = ( ( px as f32 / left as f32 ) * 255.0 ) as u32;
                buffer[bufferIndex] = u32_from_rgb(red, green, 0);
            }
        }

        //for i in buffer.iter_mut() {

            // *i = (r << 16) | (g << 8) | b;
            // *i = 0;
            // *i = 0xff;
        // }

        window.get_keys().iter().for_each(|key| match key {
            Key::W => println!("holding w!"),
            Key::T => println!("holding t!"),
            _ => (),
        });

        window.get_keys_released().iter().for_each(|key| match key {
            Key::W => println!("released w!"),
            Key::T => println!("released t!"),
            _ => (),
        });

        window
            .update_with_buffer(&buffer, new_size.0, new_size.1)
            .unwrap();
    }
}

fn calc_buffer_index( size: &(usize,usize), x: usize, y: usize ) -> usize
{
    let w = size.0;
    // let h = size.1;

    y * w + x
}

fn u32_from_rgb( r: u32, g: u32, b: u32 ) -> u32
{
    ( ( r & 0xff ) << 16 ) | ( ( g & 0xff ) << 8 ) | ( b & 0xff )
}