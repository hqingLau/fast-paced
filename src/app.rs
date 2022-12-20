use core::time;
use std::{thread, sync::{mpsc::{self, Receiver, Sender}, Arc, Mutex}};

use piston_window::{ Key};

use crate::ball::Ball;

pub const WIDTH: u32 = 640;
pub const HEIGHT: u32 = 480;
pub const DELAY: u64 = 300;
pub const STEP: f64 = 30.0;



pub struct App {
    pub window_width: u32,
    pub window_height: u32,
    pub ball1: Arc<Mutex<Ball>>,
    pub ball2: Arc<Mutex<Ball>>,
    pub ball3: Arc<Mutex<Ball>>,
    tx: Sender<crate::ball::order>,
}


impl App {
    pub fn new(wid: u32, height: u32) -> (Self,Receiver<crate::ball::order>) {
        let ball1 = Ball::default(100.0, 100.0);
        let ball1 = Arc::new(Mutex::new(ball1));
        let ball2 = Ball::default(100.0, 250.0);
        let ball2 = Arc::new(Mutex::new(ball2));
        let ball3 = Ball::default(100.0,400.0);
        let ball3 = Arc::new(Mutex::new(ball3));

        let (tx, rx):(Sender<crate::ball::order>, Receiver<crate::ball::order>) = mpsc::channel();
        (Self { window_width: wid, window_height: height, ball1,ball2,ball3,tx},
            rx)
    }


    pub fn get_ball_pos(&self, ball: Arc<Mutex<Ball>>) -> [f64;4] {
        let ball = ball.lock().unwrap();
        [ball.x - ball.radius, ball.y - ball.radius,
             ball.radius, ball.radius,]
    }

    // 更新参数
    pub fn update(&mut self, _delta_time: f64) {
        // client1
        let mut ball1 = self.ball1.lock().unwrap();
        ball1.x += (ball1.nextx - ball1.x)/20.0;
        ball1.y += (ball1.nexty - ball1.y)/20.0;
        
        // client2
        let mut ball3 = self.ball3.lock().unwrap();
        ball3.x += (ball3.nextx - ball3.x)/20.0;
        ball3.y += (ball3.nexty - ball3.y)/20.0;
    }

    // 按下了哪个键
    // 用户按下之后，自身立即响应，并且传递给服务端（模拟）
    pub fn press(&mut self, key: Key) {
        let mut ball1 = self.ball1.lock().unwrap();
        match key {
            Key::Right | Key::D => {
                // 3是队列，不然用户狂点就没边了
                if ball1.orders.len()<3 {
                    ball1.orders.push(crate::ball::order::RIGHT);
                    ball1.nextx += STEP;
                    let tx1 = Sender::clone(&self.tx);
                    thread::spawn(move || {
                        // 模拟网络延迟
                        thread::sleep(time::Duration::from_millis(DELAY));
                        tx1.send(crate::ball::order::RIGHT).unwrap();
                    });
                }
            },
            Key::Left | Key::A => {
                if ball1.orders.len()<3 {
                    ball1.orders.push(crate::ball::order::LEFT);
                    ball1.nextx -= 30.0;
                    let tx1 = Sender::clone(&self.tx);
                   
                    thread::spawn(move || {
                        thread::sleep(time::Duration::from_millis(DELAY));
                        tx1.send(crate::ball::order::LEFT).unwrap();
                    });
                }
            },
            Key::Down | Key::S => {
                if ball1.orders.len()<3 {
                    ball1.orders.push(crate::ball::order::DOWN);
                    ball1.nexty += 30.0;
                    let tx1 = Sender::clone(&self.tx);
                   
                    thread::spawn(move || {
                        thread::sleep(time::Duration::from_millis(DELAY));
                        // {
                        //     ball.lock.lock().unwrap();
                        //     ball.orders.push(crate::ball::order::DOWN);
                        // }
                        tx1.send(crate::ball::order::DOWN).unwrap();
                    });
                    
                }
            },
            Key::Up   | Key::W => {
                if ball1.orders.len()<3 {
                    ball1.orders.push(crate::ball::order::UP);
                    ball1.nexty -= 30.0;
                    let tx1 = Sender::clone(&self.tx);
                   
                    thread::spawn(move || {
                        thread::sleep(time::Duration::from_millis(DELAY));
                        tx1.send(crate::ball::order::UP).unwrap();
                    });
                    
                }
            },
            _ => ()
        }
        //println!("{} {}", self.ball1.x, self.ball1.y);
    }
}