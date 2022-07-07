use wasm_bindgen::prelude::*; //js和rust交互的包
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

//在rust中调用js,必须说明是外部的方法，非常容易啊
#[wasm_bindgen(module = "/www/utils/random.js")]
extern "C" {
    fn random(max: usize) -> usize;
}

// #[wasm_bindgen] //宏用来标识是webassembly
// pub fn hello(name: &str) {
//     //在此处调用
//     alert(name);
// }

#[derive(Copy, Clone, PartialEq)]
pub struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

#[derive(Copy, Clone, PartialEq)]
#[wasm_bindgen]
pub enum GameStatus {
    Won,
    Lost,
    Palyed,
}

impl Snake {
    fn new(spawn_index: usize, size: usize) -> Self {
        //新建一个body切片
        let mut body = Vec::new();

        for i in 0..size {
            body.push(SnakeCell(spawn_index - i))
        }
        Self {
            //结构体数组，单元组结构体
            body,
            direction: Direction::Down,
        }
    }
}

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    reward_cell: Option<usize>,
    snake: Snake,
    next_cell: Option<SnakeCell>,
    status: Option<GameStatus>,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_index: usize) -> Self {
        let size = width * width;
        //让蛇为三个单位长度
        let snake = Snake::new(snake_index, 3);
        Self {
            width,
            size: width * width,
            reward_cell: Some(World::gen_reward_cell(size, &snake.body)),
            snake,
            next_cell: None,
            status: None,
        }
    }

    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }

    //产生随机数
    //蛋不能出现在蛇身上
    fn gen_reward_cell(max: usize, snake_body: &Vec<SnakeCell>) -> usize {
        let mut reward_cell;

        loop {
            reward_cell = random(max);
            if !snake_body.contains(&SnakeCell(reward_cell)) {
                break;
            }
        }

        random(max)
    }
    //转指针,返回一个指向数组的指针
    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }
    pub fn reward_cell(&self) -> Option<usize> {
        self.reward_cell
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_index(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn change_snake_direction(&mut self, direction: Direction) {
        // self.snake.direction = direction;
        let next_cell = self.gen_next_snake_cell(&direction);
        if self.snake.body[1].0 == next_cell.0 {
            return;
        }
        self.snake.direction = direction
    }
    pub fn game_status(&self) -> Option<GameStatus> {
        self.status
    }
    pub fn start_game(&mut self) {
        self.status = Some(GameStatus::Palyed)
    }

    pub fn game_status_info(&self) -> String {
        match self.status {
            Some(GameStatus::Won) => "You won".to_string(),
            Some(GameStatus::Lost) => "You lost".to_string(),
            Some(GameStatus::Palyed) => "You playing ...".to_string(),
            None => "You not play game".to_string(),
        }
    }

    pub fn update(&mut self) {
        // let snake_head_index = self.snake_head_index();
        // let (row, col) = self.index_to_cell(snake_head_index);
        // let (row, col) = match self.snake.direction {
        //     Direction::Left => (row, (col - 1) % self.width),
        //     Direction::Right => (row, (col + 1) % self.width),
        //     Direction::Up => ((row - 1) % self.width, col),
        //     Direction::Down => ((row + 1) % self.width, col),
        // };

        // let next_index = self.cell_to_index(row, col);
        // self.set_snake_head(next_index)

        //克隆一下蛇身
        let temp = self.snake.body.clone();
        // 使用option 来提供高性能
        match self.next_cell {
            Some(cell) => {
                self.snake.body[0] = cell;
                self.next_cell = None
            }
            None => self.snake.body[0] = self.gen_next_snake_cell(&self.snake.direction),
        }
        //获取蛇身
        let next_cell = self.gen_next_snake_cell(&self.snake.direction);
        //蛇头等于蛇身
        let len = self.snake.body.len();
        for i in 1..len {
            self.snake.body[i] = SnakeCell(temp[i - 1].0)
        }
        if self.snake.body[1..len].contains(&self.snake.body[0]) {
            self.status = Some(GameStatus::Lost)
        }

        //吞蛋
        if self.reward_cell == Some(self.snake_head_index()) {
            if self.snake_length() < self.size {
                self.reward_cell = Some(World::gen_reward_cell(self.size, &self.snake.body));
                self.snake.body.push(SnakeCell(self.snake.body[1].0))
            } else {
                self.reward_cell = None;
                self.status = Some(GameStatus::Won)
            }
            self.snake.body.push(SnakeCell(self.snake.body[1].0))
        }
    }

    fn set_snake_head(&mut self, index: usize) {
        self.snake.body[0].0 = index
    }

    //返回行和列
    fn index_to_cell(&self, index: usize) -> (usize, usize) {
        (index / self.width, index % self.width)
    }

    //返回位置
    fn cell_to_index(&self, row: usize, col: usize) -> usize {
        (row * self.width) + col
    }

    //生成蛇的next，传入方向
    fn gen_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
        //获取蛇头
        let snake_index = self.snake_head_index(); //蛇头坐标
                                                   //获取其所在行
        let row = snake_index / self.width; //取余数是列，取整是行 25 / 10 = 2

        //根据方向对蛇头操作,更新蛇头位置
        return match direction {
            //同通过边界来控制
            Direction::Up => {
                //现在位置是5，5 - （10 - 0）* 10 = -95
                let border_hold = snake_index - row * self.width;
                if snake_index == border_hold {
                    //相等时，说明蛇头此时就在上边界位置
                    //100 - 10 + 5 = 95，从下边界出来
                    SnakeCell((self.size - self.width) + border_hold)
                } else {
                    //25 - 10 = 10 //上移一位
                    SnakeCell(snake_index - self.width)
                }
            }
            Direction::Down => {
                //加入格子是10*10，现在位置是25，95 + （10 - 9）* 10 = 105
                let border_hold = snake_index + (self.width - row) * self.width;
                //95 + 10 = 105
                if snake_index + self.width == border_hold {
                    //相等时，说明蛇头此时就在上边界位置
                    //100 - 10 + 5 = 95，从下边界出来 95 - (9 + 1) * 10
                    SnakeCell(border_hold - (row + 1) * self.width)
                } else {
                    //25 - 10 = 10 //上移一位
                    SnakeCell(snake_index + self.width)
                }
            }
            Direction::Left => {
                //假如格子是10*10，现在位置是20，2 * 10
                let border_hold = row * self.width;

                if snake_index == border_hold {
                    //相等时，说明蛇头此时就在左边界位置
                    //    20 +10 -1 = 29
                    SnakeCell(border_hold + self.width - 1)
                } else {
                    SnakeCell(snake_index - 1)
                }
            }
            Direction::Right => {
                //假如格子是10*10，现在位置是29，2 * 10
                let border_hold = (row + 1) * self.width;

                if snake_index + 1 == border_hold {
                    //相等时，说明蛇头此时就在左边界位置
                    SnakeCell(border_hold - self.width)
                } else {
                    SnakeCell(snake_index + 1)
                }
            }
        };
    }
}
