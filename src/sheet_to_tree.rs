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

    pub fn from(sheet: &Sheet, user_config_data: &Arc<Mutex<HashMap<String, String>>>) -> Self {
        let mut tree = TestcaseTree::new();
        if let Some(root) = &sheet.root_topic.children {
            for topic in &root.attached {
                tree.process_node(topic, &sheet.root_topic.title, user_config_data);
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

    pub fn process_node(&mut self, topic: &Topic, parent_title: &str, user_config_data: &Arc<Mutex<HashMap<String, String>>>) {
        let root_title = user_config_data.lock().unwrap().get("root_title").unwrap().clone();
        let title = format!("{}-{}", root_title, parent_title);
        if let Some(markers) = &topic.markers {
            let path = CasePath {
                title
            };
            let case = TestCase {
            title: topic.title.clone(),
            marker_id: markers.first().map(|m| match m.marker_id.as_str() {
                "priority-1" => "高".to_string(),
                "priority-2" => "中".to_string(),
                "priority-3" => "低".to_string(),
                _ => "低".to_string(),
            }).or_else(|| Some("低".to_string())), // 如果没有优先级，使用默认值 "低"
            };

            // 处理步骤为空
            let step_title = topic.children.as_ref()
                .and_then(|children| children.attached.get(0))
                .map(|child| child.title.clone())
                .unwrap_or_else(|| "".to_string());
            let step = TestStep {
                title: step_title
            };

            // 处理结果为空
            let result_title = topic.children.as_ref()
                .and_then(|children| children.attached.get(0))
                .and_then(|child| child.children.as_ref())
                .and_then(|grandchildren| grandchildren.attached.get(0))
                .map(|grandchild| grandchild.title.clone())
                .unwrap_or_else(|| "".to_string()); // 如果结果为空，插入空字符串

            let result = TestResult {
                title: result_title,
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
                self.process_node(child, &current_title, &user_config_data);
            }
        }
    }
}

fn get_case_hash(
    return_result: &mut Vec<HashMap<String, String>>,
    node: &TestNode,
) {
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