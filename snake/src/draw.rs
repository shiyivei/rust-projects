use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

const BLOCK_SIZE: f64 = 25.0; // 常量需要显式指定类型

//坐标系，以（0，0）为原点，输入值会会转变为坐标中的点
pub fn to_coord_u32(game_coord: i32) -> f64 {
     (game_coord as f64) * BLOCK_SIZE
}

//画块
pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
     //将输入转变为坐标

     let gui_x = to_coord_u32(x); //转x
     let gui_y = to_coord_u32(y); //转y

     //以坐标原点和输入的点为基准构建块 25 *25
     rectangle(
          color,
          [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
          con.transform,
          g,
     );
}

//画画面
pub fn draw_rectangle(
     color: Color,
     x: i32,
     y: i32,
     width: i32,
     height: i32,
     con: &Context,
     g: &mut G2d,
) {
     let x = to_coord_u32(x);
     let y = to_coord_u32(y);

     //画布
     rectangle(
          color,
          [
               x,
               y,
               BLOCK_SIZE * (width as f64),
               BLOCK_SIZE * (height as f64),
          ],
          con.transform,
          g,
     )
}
