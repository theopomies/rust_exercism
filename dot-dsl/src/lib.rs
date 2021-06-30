pub mod graph {
    pub mod graph_items {
        pub mod edge {
            use maplit::hashmap;
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Edge {
                first_vertex: &'static str,
                second_vertex: &'static str,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(first_vertex: &'static str, second_vertex: &'static str) -> Self {
                    Edge {
                        first_vertex,
                        second_vertex,
                        attrs: hashmap![],
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for &(key, value) in attrs {
                        self.attrs.insert(key.into(), value.into());
                    }
                    self
                }
            }
        }
        pub mod node {
            use maplit::hashmap;
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Node {
                pub name: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.into(),
                        attrs: hashmap![],
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for &(key, value) in attrs {
                        self.attrs.insert(key.into(), value.into());
                    }
                    self
                }

                pub fn get_attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|attr| attr.as_ref())
                }
            }
        }
    }
    use graph_items::{edge::Edge, node::Node};
    use maplit::hashmap;
    use std::collections::HashMap;

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: hashmap![],
            }
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = nodes.to_vec();
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = edges.to_vec();
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for (key, value) in attrs {
                self.attrs.insert((*key).into(), (*value).into());
            }
            self
        }

        pub fn get_node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|node| node.name == name)
        }
    }
}
