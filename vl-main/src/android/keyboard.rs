use jni::objects::JValue;
use super::jni::JNIManager

fn get_() {}
/// Open Android IMS keyboard. This doesn't work for accentuated characters.
/// Taken from https://github.com/mvvvv/StereoKit-rust/
/// Under MIT License
#[cfg(target_os = "android")]
pub fn show_soft_input(jni_m: JNIManager, show: bool) -> bool {
    jni_m.kk()


    if show {
        let result = android_env
            .call_method(
                im_manager,
                "showSoftInput",
                "(Landroid/view/View;I)Z",
                &[JValue::Object(&view), 0i32.into()],
            )
            .unwrap()
            .z()
            .unwrap();
        result
    } else {
        let window_token = android_env
            .call_method(
                view,
                "getWindowToken",
                "()Landroid/os/IBinder;",
                &[],
            )
            .unwrap()
            .l()
            .unwrap();
        let jvalue_window_token =
            jni::objects::JValueGen::Object(&window_token);

        let result = android_env
            .call_method(
                im_manager,
                "hideSoftInputFromWindow",
                "(Landroid/os/IBinder;I)Z",
                &[jvalue_window_token, 0i32.into()],
            )
            .unwrap()
            .z()
            .unwrap();
        result
    }
}
