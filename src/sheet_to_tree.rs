use std::{collections::HashMap, sync::{Arc, Mutex}};
use serde::de::value::Error;

use crate::{ Sheet, Topic };

#[derive(Debug)]
pub struct CasePath {
    pub title: String,
}

#[derive(Debug)]
pub struct TestCase {
    pub title: String,
    pub marker_id: Option<String>,
}

#[derive(Debug)]
pub struct TestStep {
    pub title: String,
}

#[derive(Debug)]
pub struct TestResult {
    pub title: String,
}

#[derive(Debug)]
pub struct TestGroup {
    pub path: CasePath,
    pub case: TestCase,
    pub step: TestStep,
    pub result: TestResult,
}

#[derive(Debug)]
pub enum TestNode {
    CasePath(CasePath),
    TestCase(TestCase),
    TestStep(TestStep),
    TestResult(TestResult),
    TestGroup(TestGroup),
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

    pub fn get(&self) -> Vec<HashMap<String, String>> {
        let mut return_result: Vec<HashMap<String, String>> = Vec::new();

        for node in &self.nodes {
            let node = node.lock().unwrap();
            get_case_hash(&mut return_result, &*node);
        }

        return_result
    }

    pub fn process_node(&mut self, topic: &Topic, parent_title: &str) {
        if let Some(markers) = &topic.markers {
            let path = CasePath {
                title: parent_title.to_string(),
            };
            let case = TestCase {
                title: topic.title.clone(),
                marker_id: markers.first().map(|m| m.marker_id.clone()),
            };
            let step = TestStep {
                title: topic
                    .children.as_ref().unwrap()
                    .attached.get(0).unwrap().title.clone(),
            };
            let result = TestResult {
                title: topic
                    .children.as_ref().unwrap()
                    .attached.get(0).unwrap()
                    .children.as_ref().unwrap()
                    .attached.get(0).unwrap().title.clone(),
            };
            let group = TestGroup {
                path,
                case,
                step,
                result,
            };

            self.add_node(TestNode::TestGroup(group)).unwrap();
        } else if let Some(children) = &topic.children {
            let current_title = format!("{}-{}", parent_title, &topic.title);
            for child in &children.attached {
                self.process_node(child, &current_title);
            }
        }
    }
}

fn get_case_hash(
    return_result: &mut Vec<HashMap<String, String>>,
    node: &TestNode,
) {
    // println!("{:?}\n", node);
    if let TestNode::TestGroup(group) = node {
        let mut case_map = HashMap::new();
        case_map.insert("Path".to_string(), group.path.title.clone());
        case_map.insert("Title".to_string(), group.case.title.clone());
        if let Some(marker) = &group.case.marker_id {
            case_map.insert("Marker".to_string(), marker.clone());
        }
        case_map.insert("Step".to_string(), group.step.title.clone());
        case_map.insert("Result".to_string(), group.result.title.clone());
        return_result.push(case_map);
    }
}