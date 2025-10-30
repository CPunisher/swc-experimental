use crate::{AstNode, ExtraData, NodeData};

const _: () = {
    assert!(size_of::<AstNode>() == 20);
    assert!(size_of::<NodeData>() == 8);
    assert!(size_of::<ExtraData>() == 8);
};
