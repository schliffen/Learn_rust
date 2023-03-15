

mod hill_climbing;
mod tabu;
use hill_climbing::{Task, hill_climbing_algo, evaluate_schedule};
use tabu::{tabu_search};
use std::borrow::Borrow;
use std::error::Error;
use std::collections::{HashSet, HashMap};

use std::cell::RefCell;
use std::fmt::Pointer;
use std::rc::Rc;
use std::ops::Deref;

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
/*
pub struct ScheduleState{
   pub schedule : [usize; 13],
   pub count: HashMap<usize, [usize;13]>
}
*/
pub struct ScheduleState{
    pub schedule : Rc<RefCell<[usize; 13]>>,
    pub count: Rc<RefCell<HashMap<usize, [usize;13]>>>
}





fn main() -> Result<(), Box<dyn Error>>  {


    
    let agentInfo = Rc::new(ScheduleState{
        schedule:Rc::new(RefCell::new([0;13])),
        count : Rc::new(RefCell::new(HashMap::new()))
    });
    let ref1 = Rc::clone(&agentInfo.schedule);
    let ref2 = Rc::clone(&agentInfo.count);

    ref1.borrow_mut()[0] = 2;
    ref2.borrow_mut().insert(0, *ref1.borrow_mut());
    println!("{:?}", agentInfo.schedule.borrow_mut()); // prints [(1, 2)]
    println!("{:?}", agentInfo.count.borrow_mut()); // prints {3: 4}
    


 /* 
    let mystruct = ScheduleState {
        schedule: [0; 13],
        count: HashMap::new(),
    };
*/

    // let mystruct_rc = Rc::new(RefCell::new(mystruct));
    //let y_rc = Rc::new(RefCell::new( mystruct_rc.borrow_mut().count.borrow() ));

    // Modify the count field of mystruct
    // mystruct_rc.borrow_mut().count.insert(0, [1; 13]);

    // println!("{:?}", mystruct_rc.borrow_mut().count);

    //y_rc.borrow_mut().insert(1, [1;13]);


    //println!("{:?}", mystruct_rc.clone().schedule);
    //println!("{:?}", x_rc);
    
    

    println!("Finished!");
    

/* 
    let mystruct_rc = Rc::new(RefCell::new(mystruct));
    let x1 = Rc::clone(&mystruct_rc);
    let x2 = Rc::clone(&mystruct_rc);
    x1.borrow_mut().schedule[0] = 3;
    x2.borrow_mut().count.insert(0, x1.borrow_mut().schedule);
*/


    Ok(())

}
