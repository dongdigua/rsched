use crate::proc::Proc;
use crate::prio;

use std::collections::LinkedList;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
//use std::cmp::Ordering;
//use std::time::Duration;

use rand::prelude::*;

const TIMESLICE: u64 = 200;

#[derive (Debug)]
struct Entity {
    normal_prio: u8, // change if it reached bottom
    prio: u8,        // to sort
    timeslice: u64,  // time it should run and stop (park)
    //start_time: u64,
    proc: Proc,
    runable: bool,
}

impl Entity {
    pub fn new(prio: u8) -> Self {
        Entity {
            normal_prio: prio,
            prio,
            timeslice: TIMESLICE,
            proc: Proc::new(),
            runable: true,
        }
    }

    pub fn new_random() -> Self {
        let mut rng = thread_rng();
        let prio: u8 = rng.gen_range(prio::MAX_RR_PRIO..prio::MAX_PRIO);
        Entity::new(prio)
    }
}

#[derive (Debug)]
struct Rq {
    stair: LinkedList<Entity>,
}

impl Rq {
    // spawn the scheduler thread that manipulates Rq
    pub fn spawn(mut initial_task: Vec<Entity>) -> Sender<Entity> {
        let (tx, rx) = channel();
        thread::spawn(move|| {
            let mut rq = Rq::new(initial_task);
            loop {
                let recv = rx.try_recv();
                if recv.is_ok() {
                    // println!("{:?}", &recv);
                    rq.insert(recv.unwrap());
                }
                rq.run_one();
                rq.schedule();
                rq.print();
            }
        });
        return tx;
    }

    // don't need to borrow
    pub fn new(mut entities: Vec<Entity>) -> Self {
        let mut ll = LinkedList::new();
        // lower value means higher priority
        entities.sort_by(|a, b| a.prio.partial_cmp(&b.prio).unwrap());

        let len = entities.len();
        for _i in 1..=len {
            let ent = entities.pop().unwrap();
            ll.push_front(ent);
        }

        drop(entities);
        Rq { stair: ll }
    }

    pub fn schedule(&mut self) {
        // now first implement a single-cpu one
        if self.stair.len() == 0 { return }

        let mut curr = self.stair.pop_front().unwrap();
        if curr.prio <= prio::MAX_PRIO {
            curr.prio = prio::MAX_PRIO -1;
            curr.normal_prio = prio::MAX_PRIO -1;
        }

        if self.stair.len() <= 1 {
            curr.timeslice += TIMESLICE;
            curr.normal_prio += 1;
            curr.prio = curr.normal_prio;
        } else {
            let ll = &mut self.stair;
            curr.prio += 1;
            insert_ab(ll, curr, InsertOption::After);
        }
    }

    pub fn run_one(&mut self) {
        if self.stair.len() == 0 { return }

        let mut curr = self.stair.pop_front().unwrap();
        curr.runable = ! curr.proc.run(curr.timeslice);
        if curr.runable { self.stair.push_front(curr) };
    }

    pub fn insert(&mut self, entity: Entity) {
        let ll = &mut self.stair;
        insert_ab(ll, entity, InsertOption::Before);
        // print!("after insert: ");
        // self.print();
    }

    pub fn print(&self) {
        // clear screen and move cursor at row 1 column 1
        if self.stair.len() == 0 { return }

        //print!("\x1B[2J\x1B[1;1H");
        println!("=======");
        self.stair
            .iter()
            .for_each(|x| {
                println!("normal:{} prio:{} timeslice:{}", x.normal_prio, x.prio, x.timeslice);
            });
    }
}

enum InsertOption { Before, After }

// after of before the equal one
#[inline]
fn insert_ab(ll: &mut LinkedList<Entity>, entity: Entity, option: InsertOption) {
    let mut idx = 0;
    let mut iter = ll.iter();
    let len = ll.len();

    for i in 0..=len-1 {
        let cur = iter.next().unwrap(); // uhh, we don't have get()
        idx = i;
        match option {
            InsertOption::Before =>
                if cur.prio >= entity.prio { break }
            else { if idx == len-1 { idx+=1 } },
            InsertOption::After =>
                if cur.prio >  entity.prio { break }
            else { if idx == len-1 { idx+=1 } },
        };
    }
    if let InsertOption::After = option { if idx == 0 { idx = len }} // important

    // Returns everything after the given index,
    // including the index.
    let mut split = ll.split_off(idx);
    split.push_front(entity);

    ll.append(&mut split);
}


#[cfg(test)]
use super::*;
use std::time::Duration;

#[test]
fn sort_random_entity() {
    let mut entities: Vec<Entity> =
        (1..5)
        .into_iter()
        .map(|_|  Entity::new_random() )
        .collect();
    let rq = Rq::new(entities);
    //println!("random: {:#?}", rq);
    rq.print();
}

#[test]
fn insert_entity() {
    let mut entities = vec![
        Entity::new(101),
        Entity::new(102),
        Entity::new(102),
        Entity::new(103),
    ];
    let mut rq = Rq::new(entities);
    rq.insert(Entity::new(100));
    rq.insert(Entity::new(101));
    rq.insert(Entity::new(102));
    rq.insert(Entity::new(103));
    rq.insert(Entity::new(104));
    println!("insert: {:#?}", rq);
}

#[test]
fn sched_entity() {
    let mut entities = vec![
        Entity::new(103),
        Entity::new(102),
        Entity::new(102),
        Entity::new(101),
    ];
    let mut rq = Rq::new(entities);
    rq.schedule();
    rq.schedule();
    println!("schedule: {:#?}", rq);
}

#[test]
fn spawn_rq() {
    let mut entities: Vec<Entity> =
        (1..5)
        .into_iter()
        .map(|_|  Entity::new_random() )
        .collect();
    println!("random: {:?}", &entities);
    let tx = Rq::spawn(entities);
    (1..10)
        .into_iter()
        .for_each(|_|  {
            thread::sleep(Duration::from_millis(100));
            tx.send(Entity::new_random()).unwrap();
        } );
    thread::park();
    println!("parked, shouldn't reach this");
}
