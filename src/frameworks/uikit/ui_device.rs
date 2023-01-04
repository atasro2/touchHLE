//! `UIDevice.h`

use crate::frameworks::foundation::NSInteger;

pub type UIDeviceOrientation = NSInteger;
#[allow(dead_code)]
pub const UIDeviceOrientationUnknown: UIDeviceOrientation = 0;
pub const UIDeviceOrientationPortrait: UIDeviceOrientation = 1;
#[allow(dead_code)]
pub const UIDeviceOrientationPortraitUpsideDown: UIDeviceOrientation = 2;
pub const UIDeviceOrientationLandscapeLeft: UIDeviceOrientation = 3;
#[allow(dead_code)]
pub const UIDeviceOrientationLandscapeRight: UIDeviceOrientation = 4;
#[allow(dead_code)]
pub const UIDeviceOrientationFaceUp: UIDeviceOrientation = 5;
#[allow(dead_code)]
pub const UIDeviceOrientationFaceDown: UIDeviceOrientation = 6;
