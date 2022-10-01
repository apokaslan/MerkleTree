// Import Crates
use std::fs::File;
use std::io::Read;
use std::fmt::Display;
use hex;
use sha2::{Sha256, Digest};
use merkle_tree::MerkleTree;

fn merkle_root(filename: String) -> String {
    // Read Input Data from txt file
    let mut file=File::open(filename).expect("Eror opening file");
    let mut data =String::new();
    file.read_to_string(&mut data).expect("Eror reading file");
    
    let mut list = data.split("\r\n");
       
    
    // Create vector of strings for leaves
    let mut vec = vec![];

    for i in list {
        vec.push(i.to_string());
    }

    // Hash inputs and append to vector

    for i in 0..vec.len() {
        vec[i]= hash_input(vec[i].to_string());
        
    
    }
    if(vec.len() % 2 != 0){
        vec.push(vec.last().unwrap().to_string());
    }
        // Then Write an algorithm that calculates the ROOT

    let t: MerkleTree = MerkleTree::build(&vec);

    print!("{}",t.root_hash());
    return t.root_hash();
    

    



    // Return the root hash as a String
}

// You can use templates below or just remove
// Helper function to create a next leaves level may help you :)
fn create_next_level(current_level: Vec::<String>) -> Vec::<String> {
    todo!();
}


// Helper function may help you to hash an input or You can write macro rules
fn hash_input(a: String) -> String {
// create a Sha256 object
let mut hasher = Sha256::new();

// write input message
hasher.update(a);

// read hash digest and consume hasher
let hash = hasher.finalize();
let hex = hex::encode(&hash);

return hex.to_string();}

fn main() { 
    merkle_root("src/input0.txt".to_string());

}

// Pass all tests!
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn input_0() {
        let result = merkle_root("src/input0.txt".to_string());
        assert_eq!(result, "ff41418be11ed77612aeb83ee0bcf97a5853a4c291e23bd4d4cc6435fcfabdf9");
    }

    #[test]
    fn input_1() {
        let result = merkle_root("src/input1.txt".to_string());
        assert_eq!(result, "98a77b2c3ff5f6c2aca697f60b2aa2a1a2733be36dbd35bae23d517c2ad5985e");
    }

    #[test]
    fn input_2() {
        let result = merkle_root("src/input2.txt".to_string());
        assert_eq!(result, "3c0fb0638de91551eae4e9d984d72034aa0693be37b51737e6b81bc489866e5e");
    }

    #[test]
    fn input_3() {
        let result = merkle_root("src/input3.txt".to_string());
        assert_eq!(result, "f03b1c9163babeb728ac011fe0c2c9c69700a2f8ddde211ec07d621cdb322cfe");
    }

    #[test]
    fn input_4() {
        let result = merkle_root("src/input4.txt".to_string());
        assert_eq!(result, "f83e74742fda659dfc07615881af796abafc434f591aeb23b9f4366abe03c597");
    }
}
