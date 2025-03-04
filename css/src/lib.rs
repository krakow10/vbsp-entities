mod generated;

use vbsp_common::{AsPropPlacement, PropPlacement};
pub use generated::*;

impl<'a> AsPropPlacement<'a> for PropDynamic<'a> {
    fn as_prop_placement(&self) -> PropPlacement<'a> {
        PropPlacement {
            model: self.model,
            rotation: self.angles.as_quaternion(),
            scale: self.modelscale.unwrap_or(1.0),
            origin: self.origin,
            skin: self.skin as i32,
        }
    }
}

impl<'a> AsPropPlacement<'a> for PropDynamicOverride<'a> {
    fn as_prop_placement(&self) -> PropPlacement<'a> {
        PropPlacement {
            model: self.model,
            rotation: self.angles.as_quaternion(),
            scale: self.modelscale.unwrap_or(1.0),
            origin: self.origin,
            skin: 0,
        }
    }
}

impl<'a> AsPropPlacement<'a> for PropPhysics<'a> {
    fn as_prop_placement(&self) -> PropPlacement<'a> {
        PropPlacement {
            model: self.model,
            rotation: self.angles.as_quaternion(),
            scale: 1.0,
            origin: self.origin,
            skin: self.skin.unwrap_or_default() as i32,
        }
    }
}
