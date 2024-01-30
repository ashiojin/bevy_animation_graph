use crate::core::animation_graph::{PinId, PinMap};
use crate::core::animation_node::{AnimationNode, AnimationNodeType, NodeLike};
use crate::prelude::{OptParamSpec, ParamSpec, ParamValue, PassContext, SpecContext};
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Reflect, Clone, Debug, Default)]
#[reflect(Default)]
pub struct DivF32 {}

impl DivF32 {
    pub const INPUT_1: &'static str = "F32 In 1";
    pub const INPUT_2: &'static str = "F32 In 2";
    pub const OUTPUT: &'static str = "F32 Out";

    pub fn new() -> Self {
        Self {}
    }

    pub fn wrapped(self, name: impl Into<String>) -> AnimationNode {
        AnimationNode::new_from_nodetype(name.into(), AnimationNodeType::DivF32(self))
    }
}

impl NodeLike for DivF32 {
    fn parameter_pass(&self, mut ctx: PassContext) -> HashMap<PinId, ParamValue> {
        let input_1 = ctx.parameter_back(Self::INPUT_1).unwrap_f32();
        let input_2 = ctx.parameter_back(Self::INPUT_2).unwrap_f32();

        HashMap::from([(Self::OUTPUT.into(), ParamValue::F32(input_1 / input_2))])
    }

    fn parameter_input_spec(&self, _: SpecContext) -> PinMap<OptParamSpec> {
        [
            (Self::INPUT_1.into(), ParamSpec::F32.into()),
            (Self::INPUT_2.into(), ParamSpec::F32.into()),
        ]
        .into()
    }

    fn parameter_output_spec(&self, _: SpecContext) -> PinMap<ParamSpec> {
        [(Self::OUTPUT.into(), ParamSpec::F32)].into()
    }

    fn display_name(&self) -> String {
        "÷ Divide".into()
    }
}
