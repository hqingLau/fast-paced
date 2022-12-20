use std::{thread, sync::{Arc, mpsc::{channel, Sender}}, time::Duration};

use piston_window::{PistonWindow, clear, ellipse,  UpdateEvent, PressEvent, Button};

use crate::{app::{App, STEP, self, DELAY}};


pub fn render_window(mut window: PistonWindow) {
    let (mut my_app, rx) = App::new(app::WIDTH,app::HEIGHT);

    let ball_server = Arc::clone(&my_app.ball2);
    let ball_client1 = Arc::clone(&my_app.ball1);
    let ball_client2 = Arc::clone(&my_app.ball3);
    
    let (tx_server, rx_cli2) = channel();
    thread::spawn(move || {
        for msg in rx {
            // println!("ggg");
            let mut ball_server = ball_server.lock().unwrap();
            let mut ball_client1 = ball_client1.lock().unwrap();
            
            match msg {
                crate::ball::order::DOWN => {
                    ball_server.movey(STEP);
                }
                crate::ball::order::UP => {
                    ball_server.movey(-STEP);
                }
                crate::ball::order::RIGHT => {
                    ball_server.movex(STEP);
                }
                crate::ball::order::LEFT => {
                    ball_server.movex(-STEP);
                }
            }
            ball_client1.orders.remove(0);

            let tx_srv = Sender::clone(&tx_server);
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(DELAY));
                tx_srv.send(msg).unwrap();
            });
            
        }
    });
    
    thread::spawn(move || {
        for msg in rx_cli2 {
            let mut ball_client2 = ball_client2.lock().unwrap();
            match msg {
                crate::ball::order::DOWN => {
                    ball_client2.nexty += STEP;
                }
                crate::ball::order::UP => {
                    ball_client2.nexty -= STEP;
                }
                crate::ball::order::RIGHT => {
                    ball_client2.nextx += STEP;
                }
                crate::ball::order::LEFT => {
                    ball_client2.nextx -= STEP;
                }
            }
        }
    });
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _device| {
            clear([1.0; 4], g);
            // println!("haha");
            // rectangle([1.0, 0.0, 0.0, 1.0], // red
            //             [0.0, 0.0, 100.0, 100.0],
            //             c.transform, g);
            ellipse([1.0,0.0, 0.0, 1.0], my_app.get_ball_pos(Arc::clone(&my_app.ball1)), c.transform, g);
            ellipse([0.0,1.0, 0.0, 1.0], my_app.get_ball_pos(Arc::clone(&my_app.ball2)), c.transform, g);
            ellipse([0.0,0.0, 1.0, 1.0], my_app.get_ball_pos(Arc::clone(&my_app.ball3)), c.transform, g);
        });
        if let Some(args) = e.update_args() {
            my_app.update(args.dt);
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
            my_app.press(key);
        }
    }
}