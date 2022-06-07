use piston_window::types::Color;
use piston_window::{Context, G2d};
use std::collections::LinkedList;

use crate::draw::draw_block;

const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];

#[derive(Copy, Clone, PartialEq)]

//方向
pub enum Direction {
     Up,
     Down,
     Left,
     Right,
}

//画面方向和用户方向镜像
impl Direction {
     pub fn opposite(&self) -> Direction {
          match *self {
               Direction::Up => Direction::Down,
               Direction::Down => Direction::Up,
               Direction::Left => Direction::Right,
               Direction::Right => Direction::Left,
          }
     }
}
//实现trait
#[derive(Debug, Clone)]

struct Block {
     x: i32,
     y: i32,
}

//蛇是块的链表
pub struct Snake {
     direction: Direction,
     body: LinkedList<Block>,
     tail: Option<Block>,
}

impl Snake {
     pub fn new(x: i32, y: i32) -> Snake {
          let mut body = LinkedList::new();

          //初始状态蛇由三个块组成
          body.push_back(Block { x: x + 2, y: y });
          body.push_back(Block { x: x + 1, y: y });
          body.push_back(Block { x: x, y: y });

          //初始状态朝右
          Snake {
               direction: Direction::Right,
               body,
               tail: None,
          }
     }
     //画蛇
     pub fn draw(&self, con: &Context, g: &mut G2d) {
          for block in &self.body {
               draw_block(SNAKE_COLOR, block.x, block.y, con, g);
          }
     }
     //蛇头等于第一个块
     pub fn head_position(&self) -> (i32, i32) {
          let head_block = self.body.front().unwrap();
          (head_block.x, head_block.y)
     }
     //把结构体装进枚举中再匹配，令结构体实例为d，然后将该值赋值给需要的snake的前进方向
     pub fn move_forward(&mut self, dir: Option<Direction>) {
          //给蛇一个方向
          match dir {
               Some(d) => self.direction = d,
               Node => (),
          }

          //取出蛇头
          let (last_x, last_y): (i32, i32) = self.head_position(); //解构
                                                                   //以蛇头为参考对象，确定新块
          let new_block = match self.direction {
               Direction::Up => Block {
                    x: last_x,
                    y: last_y - 1, //下移
               },
               Direction::Down => Block {
                    x: last_x,
                    y: last_y + 1, //上移
               },
               Direction::Left => Block {
                    x: last_x - 1,
                    y: last_y,
               },
               Direction::Right => Block {
                    x: last_x + 1,
                    y: last_y,
               },
          };
          //以新块为头
          self.body.push_front(new_block);
          //去掉最后一个块
          let removed_block = self.body.pop_back().unwrap();
          //让蛇尾等于移出的块
          self.tail = Some(removed_block);
     }

     //
     pub fn head_direction(&self) -> Direction {
          self.direction
     }

     //
     pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
          //取出蛇头
          let (head_x, head_y) = self.head_position();

          //以蛇头方向为方向，以蛇头为参考点
          let mut moving_dir = self.direction;
          match dir {
               Some(d) => moving_dir = d,
               None => {}
          }
          //移动第二块的方向
          match moving_dir {
               Direction::Up => (head_x, head_y - 1),
               Direction::Down => (head_x, head_y + 1),
               Direction::Left => (head_x - 1, head_y),
               Direction::Right => (head_x + 1, head_y),
          }
     }

     //尾块入队
     pub fn restore_tail(&mut self) {
          let blk = self.tail.clone().unwrap();
          self.body.push_back(blk);
     }

     //和尾块重叠
     pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
          let mut ch = 0;

          for block in &self.body {
               if x == block.x && y == block.y {
                    return true;
               }
               ch += 1;

               if ch == self.body.len() - 1 {
                    break;
               }
          }
          return false;
     }
}
