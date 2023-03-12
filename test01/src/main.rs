

mod hill_climbing;
mod tabu;
use hill_climbing::{Task, hill_climbing_algo, evaluate_schedule};
use tabu::{tabu_search};
use std::error::Error;
use std::collections::{HashSet, HashMap};

struct tst1{
    width: u16,
    height: u16
}
impl tst1 {
    fn new( w:u16, h:u16) -> tst1{
        tst1{width :w,
            height: h}
    }
    
}

impl tst1 {
    pub fn area(w:u16, h:u16)->u16{
        w*h
    }
    
}


trait Shape<'a> : Clone {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn new(&self, w: f64, h: f64)-> Self;

}

#[derive(Copy, Clone)]
struct Rectangle {
    width:  f64,
    height:  f64,
    //radius: &'a f64
}

impl<'a> Shape<'a> for Rectangle {
    fn new(&self, w:f64, h:f64)->Self{
        Rectangle { width: w, height: h }
    }
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

}


#[derive(Clone)]
pub struct ScheduleState{
    pub schedule : [usize; 13],
    pub count: HashMap<usize, [usize;13]>
}



fn main() -> Result<(), Box<dyn Error>>  {
/* 
    let tasks = vec![
        Task::new(2,7),
        Task::new(2, 8),
        Task::new(4,9),
        Task::new(5,10),
        Task::new(7,11),
        Task::new(5, 18),
        Task::new(6,15),
        Task::new(8,17),
        Task::new(7, 13),
    ];
    // hill climbing 
    println!(" start ---------------------- ");
    //let best_schedule = hill_climbing_algo( &tasks );
    let best_schedule = tabu_search( &tasks.to_vec() , 100, 10);    
    println!(" end ---------------------- ");
    let missed_dd =  evaluate_schedule(&best_schedule);
    println!(" final best missed: {} ", missed_dd );
    for item in best_schedule{
        println!("process time: {}, deadline: {} ", item.processing_time, item.deadline );
    }

*/



    
    let mut mystate : HashMap<usize, ScheduleState> = HashMap::new();        
    
    let v1 = [0;13];

    let agentInfo = ScheduleState{
        schedule:[0;13],
        count : HashMap::new()
    }


    agentInfo.count.insert(0, v1);

    mystate.insert(1, agentInfo);













    println!("Finished!");
    





    Ok(())

}
