use reqwest_dav::re_exports::serde::Serializer;
use reqwest_dav::re_exports::serde_json;
use reqwest_dav::re_exports::serde_json::{json, Map, Value};
use serde::Serialize;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::string::String;

#[derive(Debug)]
pub struct Tree<T: Treeable + Serialize> {
    data: HashMap<String, T>,
    root_node: TreeNode,
    separator: char,
}

impl<T: Treeable + Serialize> Default for Tree<T> {
    fn default() -> Self {
        Self {
            data: HashMap::new(),           // Use new() instead of default()
            root_node: TreeNode::default(), // Or however you create an empty node
            separator: '/',
        }
    }
}
#[derive(Debug)]
pub struct TreeNode {
    id: String,
    prefix: String,
    children: Vec<TreeNode>,
}
impl TreeNode {
    pub fn to_json_value(&self) -> Value {
        if self.children.is_empty() {
            // Return just the prefix as a string
            json!(self.prefix)
        } else {
            // Return an array: [prefix, child1, child2, ...]
            let children_values: Vec<Value> = self
                .children
                .iter()
                .map(|child| child.to_json_value())
                .collect();

            json!([self.prefix.clone(), children_values])
        }
    }
    pub fn to_nested_json(&self) -> Value {
        let mut children_map = Map::new();

        for child in &self.children {
            // Recursively convert each child
            children_map.insert(child.id.clone(), child.to_nested_json());
        }

        // Return an object where the key is the current node's ID
        // and the value is the map of its children.
        let mut root_map = Map::new();
        root_map.insert(self.id.clone(), Value::Object(children_map));

        Value::Object(root_map)
    }
}
impl PartialEq for TreeNode {
    fn eq(&self, other: &Self) -> bool {
        if self.id != other.id {
            return false;
        }
        if self.prefix != other.prefix {
            return false;
        }
        if self.children != other.children {
            return false;
        }
        return true;
    }
}
impl TreeNode {
    pub fn new(id: String, prefix: String) -> Self {
        TreeNode {
            id,
            prefix,
            children: Vec::new(),
            // parent:RefCell::new(parent)
        }
    }
}
impl Default for TreeNode {
    fn default() -> Self {
        TreeNode {
            id: "".to_string(),
            prefix: "".to_string(),
            children: Vec::new(),
            // parent:RefCell::new(Weak::new())
        }
    }
}
enum TrType {
    Folder(HashMap<String, TrType>),
    File(String),
}

pub trait Treeable {
    fn get_tree_id(&self) -> String;
}
impl<T: Treeable + Serialize + Clone> Tree<T> {
    pub fn add_node(&mut self, node: T) {
        let mut id = node.get_tree_id();
        //Strip leading separator
        id = match id.strip_prefix(self.separator) {
            Some(x) => x,
            None => &id,
        }
        .to_string();
        if self.data.contains_key(&node.get_tree_id()) {
            self.data.insert(node.get_tree_id(), node);
            return;
        }
        self.data.insert(id.clone(), node);
        let mut active_node = &mut self.root_node;
        let mut path_builder = String::new();
        for k in id.split(self.separator) {
            path_builder += k;
            let mut found = false;
            let res = active_node
                .children
                .iter()
                .position(|val| val.id.contains(&path_builder));
            match res {
                None => {
                    let n = TreeNode::new(path_builder.to_string(), active_node.id.clone());
                    active_node.children.push(n);
                    active_node = active_node.children.last_mut().unwrap();
                }
                Some(val) => {
                    active_node = &mut active_node.children[val];
                }
            }
            path_builder += &self.separator.to_string();
        }
    }
    pub fn delete_node(&mut self, id: String) -> Option<T> {
        /*        if !self.data.contains_key(&id) {
            return None;
        }*/
        let path_id = String::from("/") + &id;
        let parent_node = self.get_parent_node_mutable(id.clone());
        let children = &mut parent_node?.children;
        let idx = children.iter().position(|val| val.id == path_id.clone());
        children.remove(idx?);
        return self.data.remove(&path_id);
    }
    fn get_parent_node(&self, id: String) -> Option<&TreeNode> {
        let parent_id_split: Vec<_> = id.split(self.separator).collect();
        let mut range = 0..parent_id_split.len() - 1;
        if parent_id_split.len() > 2 {
            range = 0..parent_id_split.len() - 2;
        }
        let parent_id = parent_id_split[range].join(&self.separator.to_string());
        println!("{}", parent_id);
        self.get_node(parent_id)
    }
    fn get_parent_node_mutable(&mut self, id: String) -> Option<&mut TreeNode> {
        let parent_id_split: Vec<_> = id.split(self.separator).collect();
        let mut range = 0..parent_id_split.len() - 1;
        if parent_id_split.len() > 2 {
            range = 0..parent_id_split.len() - 2;
        }
        let parent_id = parent_id_split[range].join(&self.separator.to_string());
        self.get_node_mutable(parent_id)
    }
    fn get_node(&self, id: String) -> Option<&TreeNode> {
        let mut active_node = &self.root_node;
        if id == "" {
            return Some(active_node);
        }
        let mut path_builder = String::new();
        for k in id.split(self.separator) {
            path_builder += k;
            let mut found = false;
            let res = active_node
                .children
                .iter()
                .position(|val| val.id.contains(&path_builder));
            match res {
                None => {
                    return None;
                }
                Some(val) => {
                    if active_node.children[val].id == id {
                        return Some(&active_node.children[val]);
                    }
                    active_node = &active_node.children[val];
                }
            }
        }
        return None;
    }
    fn get_node_mutable(&mut self, id: String) -> Option<&mut TreeNode> {
        let mut active_node = &mut self.root_node;
        if id == "" {
            return Some(active_node);
        }
        let mut path_builder = String::new();
        for k in id.split(self.separator) {
            path_builder += k;
            let mut found = false;
            let res = active_node
                .children
                .iter()
                .position(|val| val.id.contains(&path_builder));
            match res {
                None => {
                    return None;
                }
                Some(val) => {
                    if active_node.children[val].id == id {
                        return Some(&mut active_node.children[val]);
                    }
                    active_node = &mut active_node.children[val];
                }
            }
        }
        return None;
    }
    pub fn get_at_node(&self, id: String) -> Option<&T> {
        let node = self.data.get(&id);
        return node;
    }
    pub fn print(&self) -> Value {
        return self.root_node.to_nested_json();
    }
    pub fn build_disp_tree(&self) -> reqwest_dav::re_exports::serde_json::Result<Value> {
        let val = Self::build_display_tree_from_node(&self.root_node, self.separator, &self.data);
        serde_json::to_value(&val)
    }

    fn build_display_tree_from_node(
        tree_node: &TreeNode,
        separator: char,
        data: &HashMap<String, T>,
    ) -> DisplayTree<T> {
        let mut disp: DisplayTree<T> = HashMap::new();
        tree_node.children.iter().for_each(|val| {
            let temp = val.id.split(separator);
            let final_path = temp.last().unwrap();
            if val.children.len() == 0 {
                let thing = (*data.get(&val.id).unwrap()).clone();
                disp.insert(final_path.to_string(), DisplayTreeTypes::File(thing));
            } else {
                let t = DisplayTreeTypes::Folder(Self::build_display_tree_from_node(
                    val, separator, data,
                ));
                disp.insert(final_path.to_string(), t);
            }
        });

        return disp;
    }
}

pub trait IntoTree<T: Treeable + Serialize, E> {
    fn get_tree_representation(&self) -> Result<Tree<T>, E>;
}
#[derive(Serialize)]
#[serde(untagged)]
enum DisplayTreeTypes<T: Serialize> {
    File(T),
    Folder(HashMap<String, DisplayTreeTypes<T>>),
}

type DisplayTree<T> = HashMap<String, DisplayTreeTypes<T>>;

#[cfg(test)]
mod test_tree {
    use crate::tree::{Tree, Treeable};
    use rand::random;
    use serde::Serialize;
    use std::cell::RefCell;
    use std::rc::Weak;

    #[derive(Default, Debug, Serialize, PartialEq, Clone)]
    struct TestStruct {
        id: String,
        data: String,
    }
    impl TestStruct {
        pub fn new(id: String) -> Self {
            TestStruct {
                id,
                data: random::<i64>().to_string(),
            }
        }
    }
    impl Treeable for TestStruct {
        fn get_tree_id(&self) -> String {
            self.id.clone()
        }
    }
    #[test]
    fn it_adds_node() {
        let val1 = TestStruct::new("val1".to_string());
        let val2 = TestStruct::new("hello/val1".to_string());
        let val3 = TestStruct::new("hello/val2".to_string());
        let mut tr = Tree::<TestStruct>::default();
        tr.separator = '/';
        tr.add_node(val1);
        tr.add_node(val2);
        tr.add_node(val3);
        println!("{:#?}", tr);
    }

    #[test]
    fn it_gets_node() {
        let val1 = TestStruct::new("val1".to_string());
        let val2 = TestStruct::new("hello/val1".to_string());
        let val3 = TestStruct::new("hello/val2".to_string());
        let mut tr = Tree::<TestStruct>::default();
        tr.separator = '/';
        tr.add_node(val1);
        tr.add_node(val2);
        tr.add_node(val3);
        let node = tr.get_node("hello".to_string());
        // assert_ne!(node,None);
        assert_eq!(node.unwrap().children.len(), 2);
    }

    #[test]
    fn it_gets_node_mut() {
        let val1 = TestStruct::new("val1".to_string());
        let val2 = TestStruct::new("hello/val1".to_string());
        let val3 = TestStruct::new("hello/val2".to_string());
        let mut tr = Tree::<TestStruct>::default();
        tr.separator = '/';
        tr.add_node(val1);
        tr.add_node(val2);
        tr.add_node(val3);
        let node = tr.get_node_mutable("hello".to_string());
        println!("{:#?}", node);
        // assert_ne!(node,None);
        assert_eq!(node.unwrap().children.len(), 2);
    }

    #[test]
    fn it_gets_parent_node() {
        let val1 = TestStruct::new("val1".to_string());
        let val2 = TestStruct::new("hello/val1".to_string());
        let val3 = TestStruct::new("hello/val2".to_string());
        let val4 = TestStruct::new("hello/val3".to_string());
        let mut tr = Tree::<TestStruct>::default();
        tr.separator = '/';
        tr.add_node(val1);
        tr.add_node(val2);
        tr.add_node(val3);
        tr.add_node(val4);
        let node = tr.get_parent_node("hello/val1".to_string());
        println!("{:#?}", node);
        // assert_ne!(node,None);
        assert_eq!(node.unwrap().children.len(), 3);
    }

    struct TestThing {
        value: i32,
        parent: RefCell<Weak<TestThing>>,
        children: RefCell<Vec<TestThing>>,
    }
    #[test]
    fn it_gets_parent_node_mutable() {
        let val1 = TestStruct::new("val1".to_string());
        let val2 = TestStruct::new("hello/val1".to_string());
        let val3 = TestStruct::new("hello/val2".to_string());
        let val4 = TestStruct::new("hello/val3".to_string());
        let mut tr = Tree::<TestStruct>::default();
        tr.separator = '/';
        tr.add_node(val1);
        tr.add_node(val2);
        tr.add_node(val3);
        tr.add_node(val4);
        let node = tr.get_parent_node_mutable("hello/val1".to_string());
        println!("{:#?}", node);
        // assert_ne!(node,None);
        assert_eq!(node.unwrap().children.len(), 3);
        let val = TestThing {
            parent: RefCell::new(Weak::new()),
            value: 0,
            children: RefCell::new(vec![]),
        };
    }

    #[test]
    fn it_replaces_node_when_adding_same_path() {
        let val1 = TestStruct::new("val1".to_string());
        let val2 = TestStruct::new("hello/val1".to_string());
        let val2_clone = val2.clone();
        let val3 = TestStruct::new("hello/val1".to_string());
        let val3_clone = val3.clone();
        let mut tr = Tree::<TestStruct>::default();
        tr.separator = '/';
        tr.add_node(val1);
        tr.add_node(val2);
        let node = tr.get_at_node("hello/val1".to_string());
        assert_eq!(*node.unwrap(), val2_clone);

        tr.add_node(val3);

        let node = tr.get_at_node("hello/val1".to_string());
        println!("{:#?}", node);
        // assert_ne!(node,None);
        assert_eq!(node.unwrap().clone(), val3_clone);
    }
    #[test]
    fn it_builds_display_tree() {
        let val1 = TestStruct::new("val1".to_string());
        let val2 = TestStruct::new("hello/val1".to_string());
        let val3 = TestStruct::new("hello/val2/vala".to_string());
        let val3 = TestStruct::new("hello/val2/valb".to_string());
        let mut tr = Tree::<TestStruct>::default();
        tr.separator = '/';
        tr.add_node(val1);
        tr.add_node(val2);
        tr.add_node(val3);
        let disp_tree = tr.build_disp_tree();
        println!("{}", disp_tree.unwrap());
    }
}
