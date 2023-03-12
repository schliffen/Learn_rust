
use rand::prelude::*;
use std::collections::HashSet;

use crate::hill_climbing::{Task, evaluate_schedule};



pub fn tabu_search( tasks: &[Task<i32, i32>], max_iterations: usize, tabu_list_length: usize) -> Vec<Task<i32, i32>> {

    let mut rng = rand::thread_rng();
    let mut schedule = tasks.to_vec();
    schedule.shuffle(&mut rng);

    //let mut best_schedule = schedule.clone();
    let mut best_missed_deadlines = evaluate_schedule(&schedule);

    // tabu list
    let mut tabu_list = HashSet::new();

    let mut found_better = true;

    while found_better {
        
        found_better = false;
        
        for i in 0..schedule.len() {
            for j in i+1..schedule.len() {


                let neighbor = {
                    let mut neighbor = schedule.clone();
                    neighbor.swap(i, j);
                    neighbor
                };

                let missed_deadlines = evaluate_schedule(&neighbor);

                
                if missed_deadlines < best_missed_deadlines && !tabu_list.contains( &(i,j) ) {

                    schedule = neighbor;
                    best_missed_deadlines = missed_deadlines;
                    found_better = true;

                    println!("best missed deadline: {} ", best_missed_deadlines);
                    for item in &schedule{
                        println!("process time: {}, deadline: {} ", item.processing_time, item.deadline );
                    }

            
                    break;
                }

                tabu_list.insert((i, j));
                if tabu_list.len() > tabu_list_length {
                    tabu_list.remove(&( i, j));
                }


            }

        }
    }



    return schedule; 

}

