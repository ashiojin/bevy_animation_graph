use crate::core::animation_graph::PinMap;
use crate::core::animation_node::{AnimationNode, AnimationNodeType, NodeLike};
use crate::core::edge_data::BoneMask;
use crate::core::errors::GraphError;
use crate::core::pose::Pose;
use crate::core::prelude::DataSpec;
use crate::prelude::{PassContext, SpecContext};
use crate::utils::unwrap::UnwrapVal;
use bevy::prelude::*;

#[derive(Reflect, Clone, Debug, Default)]
#[reflect(Default)]
pub struct MaskNode;

impl MaskNode {
    pub const IN_POSE: &'static str = "pose";
    pub const IN_TIME: &'static str = "time";
    pub const OUT_POSE: &'static str = "pose";

    pub const TARGETBONE: &'static str = "Target bone mask";

    pub fn new() -> Self {
        Self
    }

    pub fn wrapped(self, name: impl Into<String>) -> AnimationNode {
        AnimationNode::new_from_nodetype(name.into(), AnimationNodeType::Mask(self))
    }
}

// TODO: Move logics to proper source
fn mask_pose(pose: &Pose, mask: &BoneMask) -> Pose {
    let mut result = Pose::default();

    let mut cnt_sn = (0, 0);

    for (bone_id, idx) in pose.paths.iter() {
        let mut pose = pose.bones[*idx].clone();
        let weight = mask.bone_weight(bone_id);

        if weight > 0. {
            cnt_sn.0 += 1;
            if let Some(weights) = pose.weights.as_mut() {
                weights.iter_mut().for_each(|w| {
                    *w *= weight;
                });
            } else {
                // TODO: what None means?
                if weight < 1.0 {
                    warn_once!("!!! Weights for bone_id {:?} is None, but weight {} is set. What we should do for it", bone_id, weight);
                    warn_once!("Currently we treat it as 1.0 were specified...");
                }
            }

            result.add_bone(pose, bone_id.clone());
        } else {
            cnt_sn.1 += 1;
        }
    }

    result.timestamp = pose.timestamp;

    result
}

impl NodeLike for MaskNode {
    fn duration(&self, mut ctx: PassContext) -> Result<(), GraphError> {
        let duration = ctx.duration_back(Self::IN_TIME)?;
        ctx.set_duration_fwd(duration);
        Ok(())
    }

    fn update(&self, mut ctx: PassContext) -> Result<(), GraphError> {
        let input = ctx.time_update_fwd()?;
        ctx.set_time_update_back(Self::IN_TIME, input);
        let in_pose: Pose = ctx.data_back(Self::IN_POSE)?.val();
        ctx.set_time(in_pose.timestamp);
        let mask: BoneMask = ctx.data_back(Self::TARGETBONE)?.val();

        let masked_pose = mask_pose(&in_pose, &mask);

        ctx.set_data_fwd(Self::OUT_POSE, masked_pose);
        Ok(())
    }

    fn data_input_spec(&self, _ctx: SpecContext) -> PinMap<DataSpec> {
        [
            (Self::TARGETBONE.into(), DataSpec::BoneMask),
            (Self::IN_POSE.into(), DataSpec::Pose),
        ]
        .into()
    }

    fn data_output_spec(&self, _ctx: SpecContext) -> PinMap<DataSpec> {
        [(Self::OUT_POSE.into(), DataSpec::Pose)].into()
    }

    fn time_input_spec(&self, _ctx: SpecContext) -> PinMap<()> {
        [(Self::IN_TIME.into(), ())].into()
    }

    fn time_output_spec(&self, _ctx: SpecContext) -> Option<()> {
        Some(())
    }

    fn display_name(&self) -> String {
        "BoneMask".into()
    }
}
