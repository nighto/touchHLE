//! `UIApplication` and `UIApplicationMain`.

use crate::mem::MutPtr;
use crate::objc::{id, msg_class, nil, objc_classes, ClassExports};
use crate::Environment;

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIApplication: UIResponder

// This should only be called by UIApplicationMain
- (id)init {
    // TODO: handle the fact this is a singleton
    // TODO: app initialisation
    unimplemented!("[(UIApplication*){:?} init]", this);
}

@end

};

/// `UIApplicationMain`, the entry point of the application.
///
/// This function should never return.
pub(super) fn UIApplicationMain(
    env: &mut Environment,
    _argc: i32,
    _argv: MutPtr<MutPtr<u8>>,
    principal_class_name: id, // NSString*
    delegate_class_name: id,  // NSString*
) {
    if principal_class_name != nil || delegate_class_name != nil {
        unimplemented!()
    }

    let _ui_application: id = msg_class![env; UIApplication new];

    unimplemented!("Should enter main loop here")
}