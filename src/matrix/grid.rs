use raylib::prelude::*;

struct World {
    width: i32,
    height: i32,
    height_internal: i32,
    font_height: i32,
    font_width: i32,
}

// fn main() {
//     let mut world = World::new(1920, 1080, 100);
//     let (mut rl, thread) = raylib::init()
//         .size(world.width, world.height)
//         .title("Font grid calculator")
//         .build();
//     let mut initial_font_calculation = true;
//
//     let (mut grid_size_x, mut grid_size_y) = world.calculate_grid_size();
//     let (mut grid_offset_x, mut grid_offset_y) = world.get_grid_offset();
//     while !rl.window_should_close() {
//         let font: WeakFont = rl.get_font_default();
//
//         if initial_font_calculation {
//             //Can possibly be moved into intialization, but somehow need font info there then.
//             world.update_font_width(&font);
//             initial_font_calculation = false;
//             (grid_size_x, grid_size_y) = world.calculate_grid_size();
//             (grid_offset_x, grid_offset_y) = world.get_grid_offset();
//         }
//
//         if rl.is_key_pressed(KeyboardKey::KEY_A) {
//             world.update_font_size(true, &font);
//             (grid_size_x, grid_size_y) = world.calculate_grid_size();
//             (grid_offset_x, grid_offset_y) = world.get_grid_offset();
//         }
//         if rl.is_key_pressed(KeyboardKey::KEY_S) {
//             world.update_font_size(false, &font);
//             (grid_size_x, grid_size_y) = world.calculate_grid_size();
//             (grid_offset_x, grid_offset_y) = world.get_grid_offset();
//         }
//
//         let mut d = rl.begin_drawing(&thread);
//         d.clear_background(Color::BLACK);
//         let (min,max) =world.get_height_internal_offset_range();
//         let offset = world.get_character_offset(&font,"A"); // so the grid is not over /font offset
//         for x in min..grid_size_x+max {
//             for y in min..grid_size_y +max{
//                 d.draw_rectangle_lines(
//                     grid_offset_x + x * world.font_width,
//                     grid_offset_y + y * world.font_height,
//                     world.font_width,
//                     world.font_height,
//                     Color::RED,
//                 );
//                 d.draw_text_ex(
//                     &font,
//                     "A",
//                     Vector2::new(
//                         (x as f32 * world.font_width as f32 + offset as f32 +grid_offset_x as f32) as f32,
//                         (y as f32 * world.font_height as f32 + grid_offset_y as f32) as f32,
//                     ),
//                     world.font_height as f32,
//                     0.0,
//                     Color::GREEN,
//                 );
//             }
//         }
//     }
// }
