use std::rc::Rc;
use std::cell::RefCell;

use serde::de::value::Error;

use crate::{ Sheet, Topic };

pub struct TestCase {
    pub title: String,
    pub marker_id: Option<String>,
}

pub struct TestStep {
    pub title: String,
}

pub struct TestResult {
    pub title: String,
}

pub enum TestNode {
    TestCase(TestCase),
    TestStep(TestStep),
    TestResult(TestResult),
}

pub struct TestTree {
    pub nodes: Vec<Rc<RefCell<TestNode>>>,
}

impl TestTree {
    pub fn new() -> Self {
        TestTree { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, node: TestNode) -> Result<(), Error> {
        let rc_node = Rc::new(RefCell::new(node));
        self.nodes.push(rc_node.clone());
        Ok(())
    }

    pub fn create_testtree(&mut self, sheet: &Sheet){
        println!("{:?}", sheet);
    }
}

pub fn process_node(node: &Topic, depth: usize) {
    if let Some(children) = node.children.clone() {
        for child in children.attached {
            process_node(&child, depth + 1);
        }
    };
}