mod hill_climbing;
mod tabu;


use std::collections::{HashMap};
use std::error::Error;


use std::cell::RefCell;


use std::rc::Rc;






use std::ops::Add;

fn add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

struct tst1 {
    width: u16,
    height: u16,
}
impl tst1 {
    fn new(w: u16, h: u16) -> tst1 {
        tst1 {
            width: w,
            height: h,
        }
    }
}

impl tst1 {
    pub fn area(w: u16, h: u16) -> u16 {
        w * h
    }
}

trait Shape<'a>: Clone {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn new(&self, w: f64, h: f64) -> Self;
}

#[derive(Copy, Clone)]
struct Rectangle {
    width: f64,
    height: f64,
    //radius: &'a f64
}

impl<'a> Shape<'a> for Rectangle {
    fn new(&self, w: f64, h: f64) -> Self {
        Rectangle {
            width: w,
            height: h,
        }
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
pub struct ScheduleState {
    pub schedule: Rc<RefCell<[usize; 13]>>,
    pub count: Rc<RefCell<HashMap<usize, [usize; 13]>>>,
}

type CallbackFn = fn(&Vec<i32>) -> Vec<i32>;
// define a callback function

// Function that takes a callback function as an argument and returns a vector of i32s
fn process_numbers(numbers: &Vec<i32>, callback: CallbackFn) -> Vec<i32> {
    // Double all the numbers in the input vector
    let doubled_numbers = numbers.iter().map(|x| x * 2).collect();

    // Call the callback function to modify the doubled numbers
    

    // Return the modified numbers
    callback(&doubled_numbers)
}

// Define a callback function that squares all the numbers in the input vector
fn square_numbers(numbers: &Vec<i32>) -> Vec<i32> {
    numbers.iter().map(|x| x * x).collect()
}
pub struct  Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl <T> Node<T>{
    fn new(data: T) -> Self {
        Node {
            data,
            next: None,
        }
    }

    fn append(&mut self, data: T) {
        let mut current = self;
        while let Some(ref mut next) = current.next {
            current = next;
        }
        current.next = Some(Box::new(Node::new(data)));
    }
}


pub fn  add_two_numbers(
    l1: Option<Box<Node<i32>>>,
    l2: Option<Box<Node<i32>>>,
) -> Option<Box<Node<i32>>> {
    
    let mut front = Box::new(Node::new(0));
    let mut current = &mut front;

    let mut num1 = &l1;
    let mut num2 = &l2;

    let mut total = 0;
    loop {
        match (num1, num2) {
            (Some(n1), Some(n2)) => {
                total += n1.data + n2.data ;
                num1 = &n1.next;
                num2 = &n2.next;
            }    
            (Some(n1), None) => {
                total += n1.data;
                num1 = &n1.next;
            }    
            (None, Some(n2)) => {
                total = n2.data;
                num2 = &n2.next;
            }    
            (None, None) => {
                break;
            }    
        }    

        let r = total % 10;
        total /= 10; 
        current.next = Some(Box::new(Node::new(r)));
        current = current.next.as_mut().unwrap();
    }    

    if total > 0 {
        current.next = Some(Box::new(Node::new(total)));
    }    

    front.next
}    

fn main() -> Result<(), Box<dyn Error>> {

    // creating linked list using box 
    let mut l1 = Node::new(1);
    l1.append(9);
    l1.append(8);
    l1.append(7);
    l1.append(0);

    let mut l2 = Node::new(1);
    l2.append(7);
    l2.append(5);
    l2.append(6);
    //
    let mut result = add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2))).unwrap();

    //println!("result data: {:?}", result.unwrap().data);

    let iterate = true;
    while  iterate {
        println!("{}", result.as_ref().data );
        result = result.next.unwrap();
    }




    // assume this lined list is given 

    /*
    // retuen data
    let mut return_list = Node::new(0);
    let iterate = true;
    while  iterate {
        println!("{}", current.data);
        let mut newdata = Node::new(current.data);
//        if  return_list.next.is_some() {
            newdata.next = return_list.next;
            return_list.next = Some(Box::new(newdata));
  //      }else{
  //          return_list.next = Some(Box::new(newdata));
  //      };
        current = next;
    }
    println!("{}", current.data);

    //let mut newdata = Node::new(current.data);
    //newdata.next = return_list.next;
    //return_list.next = Some(Box::new(newdata));
    if current.data > 0{
        return_list.data =  current.data;
    } else {
        return_list.data = 0;
    }

    
    // reverting the assignment
    let mut ncurrent = &return_list;
    while let Some(ref next) = ncurrent.next {
        println!("{}", ncurrent.data);
        ncurrent = next;
    }
    
    println!("{}", ncurrent.data);
    */






    /*
    FIXME: 2. Create a function that takes a schedule and a count and returns the fitness
    * this part was for testing the callback function
         let input = "hello callback";
    // Pass the callback function to the process_input function
    //let numbers = vec![1, 2, 3, 4, 5];
    //let modified_numbers = process_numbers(&numbers, square_numbers);
    // Print the modified numbers
    //println!("{:?}", modified_numbers);
    */
    /*
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

    let seed: [u8; 32] = [42; 32];
    let mut rng = StdRng::from_seed(seed);
    let random_number = rng.gen_range(0..=10);

    println!("Random number with seed {:?}: {}", seed, random_number);
    */
    /*
    let mystruct = ScheduleState {
        schedule: [0; 13],
        count: HashMap::new(),
    };

    // let mystruct_rc = Rc::new(RefCell::new(mystruct));
    //let y_rc = Rc::new(RefCell::new( mystruct_rc.borrow_mut().count.borrow() ));
    // Modify the count field of mystruct
    // mystruct_rc.borrow_mut().count.insert(0, [1; 13]);
    // println!("{:?}", mystruct_rc.borrow_mut().count);
    //y_rc.borrow_mut().insert(1, [1;13]);
    //println!("{:?}", mystruct_rc.clone().schedule);
    //println!("{:?}", x_rc);
    */

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
