use macroquad::prelude::*;
use  macroquad::rand::gen_range;
use std::time::Instant;
use std::time::Duration;
use std::{thread, time};

const TEN_MILLIS: Duration = time::Duration::from_millis(10);

const M_T: i32 = 300;
const OBJ_C: usize = 50;

#[macroquad::main(window_conf)]
async fn main() {
    let background = Color::from_hex(0x0f0f0f);
    let target = vec![screen_width()/2.0, 100.0];
    let obj = vec![screen_width()/2.0,screen_height()-100.0];
    let mut objs = vec![obj.clone(); OBJ_C];
    let mut pos = vec![vec![]; OBJ_C];
    for l in pos.iter_mut() {
        for _ in 0..M_T {
            l.push(vec![gen_range(-2., 2.), gen_range(-2., 2.)])
        }
    }
    let mut f = 0;
    let mut index = 0;
    let mut gen = 0;
    loop {
        let now = Instant::now();
        clear_background(background);
        draw_circle(target[0], target[1], 10.0, WHITE);
        index = 0;
        for o in objs.iter_mut() {
            o[0] += pos[index][f][0];
            o[1] += pos[index][f][1];
            draw_rectangle(o[0], o[1], 10.0, 10.0, GREEN);
            index += 1;
        }
        f += 1;
        let mut top = pos[0].clone();
        let mut top_d = screen_width();
        if f == (M_T as usize) {
            index = 0;
            for i in objs.iter() {
                let d = dist(&i, &target);
                if d < top_d {
                    top = pos[index].clone();
                    top_d = d;
                }
                index += 1;
            }

            pos = vec![];
            for _ in 0..OBJ_C {
                pos.push(mutate(&top));
            }
            objs = vec![obj.clone(); OBJ_C];
            f = 0;
            gen += 1;
        }
        thread::sleep(TEN_MILLIS);
        let fps = (1000/now.elapsed().as_millis());
        draw_text(&(fps.to_string() + "fps"), 20.0, 20.0, 30.0, WHITE);
        draw_text(&(gen.to_string() + " gens"), 20.0, 60.0, 30.0, WHITE);
        next_frame().await
    }
}

fn dist(m: &Vec<f32>, o: &Vec<f32>) -> f32 {
    return ((m[0] - o[0]).powi(2) + (m[1] - o[1]).powi(2)).sqrt()
}

fn mutate(l: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let mut l2 = l.clone();
    for i in l2.iter_mut() {
        i[0] += gen_range(-2., 2.);
        i[1] += gen_range(-2., 2.);
    }
    return l2;
}

fn window_conf() -> Conf {
    Conf {
        window_resizable: false,
        window_title: "Genetic Algorithm".to_string(),
        ..Default::default()
    }
}
