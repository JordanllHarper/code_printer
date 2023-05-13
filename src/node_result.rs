//Represents the result of collecting contents in a node
//Holds state in which the contents should be included in the final output + the contents
pub mod node_result {
    pub struct NodeResult {
        pub include_content: bool,
        pub contents: String,
    }


    impl NodeResult {
        //Defines a default for the result
        pub(crate) fn new() -> NodeResult {
            NodeResult{ include_content: false, contents: "".to_string()}
        }
    }
}
