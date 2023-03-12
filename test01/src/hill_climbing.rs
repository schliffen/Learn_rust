/*
Hill climbing algorithm
*/
use rand::prelude::*;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Task<T1,T2>{
    pub processing_time: T1,
    pub deadline: T2,
     
}

impl <T1, T2> Task <T1, T2>
where
T1: std::cmp::Eq + std::hash::Hash,
T2: std::cmp::Eq + std::hash::Hash,
{
    pub fn new(p_time:T1, deadl:T2)->Self {
        Self { processing_time : p_time, deadline: deadl }
    } 
}



pub fn hill_climbing_algo(tasks: &[Task<i32, i32>] )-> Vec<Task<i32, i32>>{

    
    let mut rng = rand::thread_rng();
    let mut schedule = tasks.to_vec();
    schedule.shuffle(&mut rng);

    let mut best_schedule = schedule.clone();
    let mut best_missed_deadlines = evaluate_schedule(&schedule); 

    let mut found_better = true;

    let mut counter = 0;
    let max_iter= 10;

    while found_better && counter < max_iter {
        found_better = false;
        for i in 0..schedule.len(){
            for j in i+1..schedule.len(){
                let mut neighbor = schedule.clone();
                neighbor.swap(i, j);
                let missed_deadlines = evaluate_schedule(&neighbor );
                if missed_deadlines < best_missed_deadlines{
                    schedule = neighbor;
                    best_missed_deadlines = missed_deadlines;
                    found_better = true;
                    println!("best missed deadline: {} ", best_missed_deadlines);
                    for item in &schedule{
                        println!("process time: {}, deadline: {} ", item.processing_time, item.deadline );
                    }
                    println!(" *** iteration: {} ", counter );
                    //for item in &best_schedule{
                    //    println!("process time: {}, deadline: {} ", item.processing_time, item.deadline );
                    //}
                }
            }
        }
        //best_schedule = schedule;
        counter+=1;

        
    }



    return schedule;
}


pub fn evaluate_schedule(in_schedule:&Vec<Task<i32, i32>>) -> u32{

    let mut time = 0;
    let mut missed_deadline = 0;

    for task in in_schedule{
        time += task.processing_time;
        if time> task.deadline{
            missed_deadline +=1;
        }
    }

    missed_deadline
}