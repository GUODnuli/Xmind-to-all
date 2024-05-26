use std::sync::{Arc, Mutex};

use serde::de::value::Error;

use crate::{ Sheet, Topic };

pub struct CasePath {
    pub title: String,
}
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
    CasePath(CasePath),
    TestCase(TestCase),
    TestStep(TestStep),
    TestResult(TestResult),
}

pub struct TestcaseTree {
    pub nodes: Vec<Arc<Mutex<TestNode>>>,
}

impl TestcaseTree {
    pub fn new() -> Self {
        TestcaseTree { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, node: TestNode) -> Result<(), Error> {
        let rc_node = Arc::new(Mutex::new(node));
        self.nodes.push(rc_node.clone());
        Ok(())
    }

    pub fn from(sheet: &Sheet) -> Self {
        let mut tree = TestcaseTree::new();
        if let Some(root) = &sheet.root_topic.children {
            for topic in &root.attached {
                tree.process_node(topic, &sheet.root_topic.title);
            }
        }
        tree
    }
    pub fn process_node(&mut self, topic: &Topic, parent_title: &str) {
        let current_title = format!("{}-{}", parent_title, &topic.title);
        if let Some(markers) = &topic.markers {
            let case = TestCase {
                title: current_title,
                marker_id: markers.first().map(|m| m.marker_id.clone()),
            };
            let step = TestStep {
                title: topic
                    .children
                    .as_ref()
                    .unwrap()
                    .attached
                    .get(0)
                    .unwrap()
                    .title
                    .clone(),
            };
            let result = TestResult {
                title: topic
                    .children
                    .as_ref()
                    .unwrap()
                    .attached
                    .get(0)
                    .unwrap()
                    .children
                    .as_ref()
                    .unwrap()
                    .attached
                    .get(0)
                    .unwrap()
                    .title
                    .clone(),
            };
            self.add_node(TestNode::TestCase(case)).unwrap();
            self.add_node(TestNode::TestStep(step)).unwrap();
            self.add_node(TestNode::TestResult(result)).unwrap();
        } else if let Some(children) = &topic.children {
            for child in &children.attached {
                self.process_node(child, &current_title);
            }
        }
    }

    pub fn traverse_tree(&self) {
        for node in &self.nodes {
            let node = node.lock().unwrap();
            match &*node {
                TestNode::TestCase(case) => {
                    println!("TestCase: {} - Marker ID: {:?}", case.title, case.marker_id);
                }
                TestNode::TestStep(step) => {
                    println!("TestStep: {}", step.title);
                }
                TestNode::TestResult(result) => {
                    println!("TestResult: {}", result.title);
                }
            }
        }
    }
}
