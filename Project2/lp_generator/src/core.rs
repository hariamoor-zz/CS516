use resource::Resource;
use std::io::{Read, Result};

use serde_xml_rs::from_reader;

impl Resource {
    fn generate_vars(&mut self) {
        for knob in self.knobs {
            for layer in knob.layers {
                for node in layer.basic_nodes {
                    // self.vars.insert(node.name, var);
                }
            }
        }
    }

    pub fn from_reader<T: Read>(source: T) -> Self {
        let ret: Self = from_reader(source)?;

        ret
    }
}
