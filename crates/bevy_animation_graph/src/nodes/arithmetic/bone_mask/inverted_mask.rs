use crate::core::animation_graph::PinMap;
use crate::core::animation_node::{AnimationNode, AnimationNodeType, NodeLike};
use crate::core::edge_data::BoneMask;
use crate::core::errors::GraphError;
use crate::core::prelude::DataSpec;
use crate::prelude::{PassContext, SpecContext};
use crate::utils::unwrap::UnwrapVal;
use bevy::prelude::*;

#[derive(Reflect, Clone, Debug, Default)]
#[reflect(Default)]
pub struct InvertedMask {}

impl InvertedMask {
    pub const INPUT: &'static str = "BoneMask In";
    pub const OUTPUT: &'static str = "BoneMask Out";

    pub fn new() -> Self {
        Self {}
    }

    pub fn wrapped(self, name: impl Into<String>) -> AnimationNode {
        AnimationNode::new_from_nodetype(name.into(), AnimationNodeType::InvertedMask(self))
    }
}

fn inverse(mask: &BoneMask) -> BoneMask {
    match mask {
        BoneMask::Positive { bones } => {
            let mut bones = bones.clone();
            bones.iter_mut()
                .for_each(|d| *d.1 = 1.0 - *d.1);
            BoneMask::Negative { bones }
        },
        BoneMask::Negative { bones } => {
            let mut bones = bones.clone();
            bones.iter_mut()
                .for_each(|d| *d.1 = 1.0 - *d.1);
            BoneMask::Positive { bones }
        }
    }
}

impl NodeLike for InvertedMask {
    fn display_name(&self) -> String {
        "|~| Inverted Mask".into()
    }

    fn update(&self, mut ctx: PassContext) -> Result<(), GraphError> {
        let input: BoneMask = ctx.data_back(Self::INPUT)?.val();
        let output = inverse(&input);
        ctx.set_data_fwd(Self::OUTPUT, output);
        Ok(())
    }

    fn data_input_spec(&self, _ctx: SpecContext) -> PinMap<DataSpec> {
        [(Self::INPUT.into(), DataSpec::F32)].into()
    }

    fn data_output_spec(&self, _ctx: SpecContext) -> PinMap<DataSpec> {
        [(Self::OUTPUT.into(), DataSpec::F32)].into()
    }
}

