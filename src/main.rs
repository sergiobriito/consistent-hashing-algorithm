use std::collections::HashMap;
use md5;

struct ConsistentHashing {
    hash_ring: HashMap<String,String>,
    sorted_keys: Vec<String>,
    virtual_nodes: i32
}

impl ConsistentHashing {
    fn new(nodes: &[String], virtual_nodes: &i32) -> ConsistentHashing {
        let mut hash_ring: HashMap<String, String> = HashMap::new();
        let mut sorted_keys: Vec<String> = Vec::<String>::new();
        for node in nodes {
            for i in 0..*virtual_nodes {
                let key = ConsistentHashing::hash(&format!("{}_{}", node, i));
                hash_ring.insert(key.clone(), format!("{}_{}", node, i));
                sorted_keys.push(key.clone());
            }
        }
        sorted_keys.sort();
        ConsistentHashing {
            hash_ring: hash_ring,
            virtual_nodes: *virtual_nodes,
            sorted_keys: sorted_keys
        }
    }

    fn hash(key: &String) -> String {
        let hash = md5::compute(key);
        format!("{:x}", hash)
    }

    fn add_node(&mut self, node: &String) {
        for i in 0..self.virtual_nodes {
            let key: String = ConsistentHashing::hash(&format!("{}_{}", node, i));
            self.hash_ring.insert(key.clone(), format!("{}_{}", node, i));
            self.sorted_keys.push(key.clone());
        }
        self.sorted_keys.sort();
    }

    fn remove_node(&mut self, node: &String) {
        for i in 0..self.virtual_nodes {
            let key: String = ConsistentHashing::hash(&format!("{}_{}", node, i));
            self.hash_ring.remove(&key);
            self.sorted_keys.retain(|x| x != &key);
        }
        self.sorted_keys.sort();
    }

    fn get_node(&self, data: &String) -> Option<String> {
        let data_key = ConsistentHashing::hash(data);
    
        for key in &self.sorted_keys {
            if data_key < String::from(key) {
                if let Some(node) = self.hash_ring.get(key) {
                    return Some(String::from(node));
                }
            }
        }
    
        for key in &self.sorted_keys {
            if let Some(node) = self.hash_ring.get(key) {
                return Some(String::from(node));
            }
        }
    
        None
    }
    


}

fn main() {

    let nodes = [String::from("Node1"),String::from("Node2"),String::from("Node3")];
    let virtual_nodes = 3;
    let mut consistent_hashing = ConsistentHashing::new(&nodes, &virtual_nodes);

    println!("-----Consistent Hashing-----");
    println!("-----Add Node (1,2,3)-----");
    println!("Data: {} -> Node: {:?}",String::from("Data1"),consistent_hashing.get_node(&String::from("Data1")).unwrap_or(String::from("None")));
    println!("Data: {} -> Node: {:?}",String::from("Data2"),consistent_hashing.get_node(&String::from("Data2")).unwrap_or(String::from("None")));
    println!("Data: {} -> Node: {:?}",String::from("Data3"),consistent_hashing.get_node(&String::from("Data3")).unwrap_or(String::from("None")));
   

    println!("-----Remove Node (1)-----");
    consistent_hashing.remove_node(&String::from("Node1"));
    println!("Data: {} -> Node: {:?}",String::from("Data1"),consistent_hashing.get_node(&String::from("Data1")).unwrap_or(String::from("None")));
    println!("Data: {} -> Node: {:?}",String::from("Data2"),consistent_hashing.get_node(&String::from("Data2")).unwrap_or(String::from("None")));
    println!("Data: {} -> Node: {:?}",String::from("Data3"),consistent_hashing.get_node(&String::from("Data3")).unwrap_or(String::from("None")));
    
    println!("-----Add Node (0)-----");
    consistent_hashing.add_node(&String::from("Node0"));
    println!("Data: {} -> Node: {:?}",String::from("Data1"),consistent_hashing.get_node(&String::from("Data1")).unwrap_or(String::from("None")));
    println!("Data: {} -> Node: {:?}",String::from("Data2"),consistent_hashing.get_node(&String::from("Data2")).unwrap_or(String::from("None")));
    println!("Data: {} -> Node: {:?}",String::from("Data3"),consistent_hashing.get_node(&String::from("Data3")).unwrap_or(String::from("None")));
    
    println!("-----Remove Node (0,2)-----");
    consistent_hashing.remove_node(&String::from("Node0"));
    consistent_hashing.remove_node(&String::from("Node2"));
    println!("Data: {} -> Node: {:?}",String::from("Data1"),consistent_hashing.get_node(&String::from("Data1")).unwrap_or(String::from("None")));
    println!("Data: {} -> Node: {:?}",String::from("Data2"),consistent_hashing.get_node(&String::from("Data2")).unwrap_or(String::from("None")));
    println!("Data: {} -> Node: {:?}",String::from("Data3"),consistent_hashing.get_node(&String::from("Data3")).unwrap_or(String::from("None")));

}















