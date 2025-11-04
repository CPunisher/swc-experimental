use crate::{AstNode, ExtraData, NodeData};

const _: () = {
    assert!(size_of::<AstNode>() == 16);
    assert!(size_of::<NodeData>() == 4);
    assert!(size_of::<ExtraData>() == 8);
};
