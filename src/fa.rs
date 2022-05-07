use std::f64;
use std::fmt::{Display, Formatter};
use rand::{Rng, thread_rng};
use crate::testfunctions::*;

//Settingsy

static LOWER_BOUND: f64 = -5 as f64;
static UPPER_BOUND: f64 = 5 as f64;
//TODO dodać setting wyboru rozkładu epsilona
//TODO zapytać o problemy dyskretne
static MAX_GENS: u32 = 2000;
static POP_SIZE:u32 = 25;
static ALFA0: f64 = 1 as f64; //Wspł początkowej losowości
static BETA0:f64 = 1 as f64; //Wspł atrakcyjności, zwykle może zostać 1
static GAMMA:f64 = 0.01 as f64; //Wspł absorpcji światła
static DELTA:f64 = 0.97; //Wspł spadku losowości, 0<delta<1


pub fn fireflies(dimensions:i8, f:fn(&Vec<f64>) -> f64){
    let mut population:Vec<Vec<f64>> = Vec::new();
    for _index in 0..POP_SIZE as usize{ //Generacja populacji
        let mut temp:Vec<f64> = Vec::new();
        for _dim in 0..dimensions {
            temp.push(thread_rng().gen_range(LOWER_BOUND as f64..UPPER_BOUND as f64));
        }
        population.push(temp);
    }
    let mut brightness:Vec<f64> = Vec::new();
    let temp = population.clone();
    for point in temp{
        brightness.push(1 as f64/ f(&point)); //TODO USUŃ TEMP CLONEA
    }
    let scale = UPPER_BOUND - LOWER_BOUND;
    let mut alfa = ALFA0;
    let mut rng = thread_rng();
    for generation in 0..MAX_GENS{
        for index in 0 as usize..POP_SIZE as usize{
            for innerindex in 0 as usize..POP_SIZE as usize{
                if brightness[index] < brightness[innerindex]{
                    let const1 = BETA0 * f64::powf(f64::consts::E, -1 as f64 * GAMMA * f64::powi(distance(&population[index], &population[innerindex]),2));
                    for dimension in 0 as usize..dimensions as usize{
                        population[index][dimension] += const1* (population[innerindex][dimension] - population [index][dimension]) + ALFA0*alfa * (rng.gen_range(0.01..0.99) - 0.5) * scale;
                    }
                    brightness[index] = 1 as f64/f(&population[index]);
                }
            }
        }
        alfa = alfa*DELTA;
        if generation % 25 == 0{
            //TODO LOG
            let mut maxpos = 0;
            let mut maxbright = 0 as f64;
            for index in 0 as usize..POP_SIZE as usize{ //TODO POPRAW ZNAJDOWANIE MAXA
                if brightness[index] == f64::INFINITY{ //TODO CHYBA TRZEBA UZGODNIĆ ORD DLA F64
                    maxpos = index;
                    break;
                }
                if brightness[index] > maxbright{
                    maxbright = brightness[index];
                    maxpos = index;
                }
            }
            println!("Gen: {}, x: {}, y: {}", generation, population[maxpos][0], population[maxpos][1]);
        }
    }
    println!("END");
}

fn distance(a:&Vec<f64>, b:&Vec<f64>) -> f64{
    let mut res:f64 = 0 as f64;
    for dimension in 0..a.len(){
        res += f64::powi(a[dimension] - b[dimension], 2)
    }
    f64::sqrt(res)
}