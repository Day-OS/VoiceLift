use jni::errors::Error as JNIError;
use jni::objects::{JObject, JValue};

use thiserror::Error;

#[derive(Error, Debug)]
enum JNIManagerError {
    #[error("No JavaVM found. Reason: {0}")]
    NoJavaVM(JNIError),

    #[error("JNIEnv could not be attached. Reason: {0}")]
    JNIEnvNotAttached(JNIError),

    #[error("Context class could not be found. Reason: {0}")]
    ContextClassNotFound(JNIError),

    #[error(
        "Constant of Input Method Service could not be found. Reason: {0}"
    )]
    InputMethodServiceConstNotFound(JNIError),

    #[error("Input Method Service could not be found. Reason: {0}")]
    InputMethodServiceNotFound(JNIError),

    #[error("Could not get Window Object. Reason: {0}")]
    WindowCouldNotBeGet(JNIError),

    #[error("Could not get View Object. Reason: {0}")]
    ViewCouldNotBeGet(JNIError),
}

struct JNIManager<'a> {
    _activity: JObject<'a>,
    _jni_env: jni::AttachGuard<'a>,
    /// Interface to global information about an application environment.
    /// This is an abstract class whose implementation is provided by the Android system.
    /// It allows access to application-specific resources and classes, as well as up-calls
    /// for application-level operations such as launching activities,
    /// broadcasting and receiving intents, etc.
    /// https://developer.android.com/reference/android/content/Context
    _class_ctxt: jni::objects::JClass<'a>,
    input_method_service: JObject<'a>,
    /// Abstract base class for a top-level window look and behavior policy. An instance of
    /// this class should be used as the top-level view added to the window manager.
    /// It provides standard UI policies such as a background, title area, default
    /// key processing, etc.
    /// https://developer.android.com/reference/android/view/Window
    window: JObject<'a>,
    /// This class represents the basic building block for user interface components.
    /// A View occupies a rectangular area on the screen and is responsible for drawing
    /// and event handling. View is the base class for widgets, which are used to create
    /// interactive UI components (buttons, text fields, etc.). The ViewGroup subclass is the
    /// base class for layouts, which are invisible containers that hold other Views
    /// (or other ViewGroups) and define their layout properties.
    view: JObject<'a>,
}

impl JNIManager<'_> {
    fn new() -> Result<Self, JNIManagerError> {
        let ctx: ndk_context::AndroidContext =
            ndk_context::android_context();

        let vm: jni::JavaVM =
            unsafe { jni::JavaVM::from_raw(ctx.vm() as _) }
                .map_err(|e| JNIManagerError::NoJavaVM(e))?;

        let activity: JObject<'_> = unsafe {
            jni::objects::JObject::from_raw(ctx.context() as _)
        };

        let mut jni_env: jni::AttachGuard<'_> = vm
            .attach_current_thread()
            .map_err(|e| JNIManagerError::JNIEnvNotAttached(e))?;

        let class_ctxt: jni::objects::JClass<'_> = jni_env
            .find_class("android/content/Context")
            .map_err(|e| JNIManagerError::ContextClassNotFound(e))?;

        // string used to find the actual Input Method Service
        let ims_const: jni::objects::JValueGen<JObject<'_>> = jni_env
            .get_static_field(
                class_ctxt,
                "INPUT_METHOD_SERVICE",
                "Ljava/lang/String;",
            )
            .map_err(|e| {
                JNIManagerError::InputMethodServiceConstNotFound(e)
            })?;

        let input_method_service: JObject<'_> = jni_env
            .call_method(
                &activity,
                "getSystemService",
                "(Ljava/lang/String;)Ljava/lang/Object;",
                &[ims_const.borrow()],
            )
            .unwrap()
            .l()
            .map_err(|e| {
                JNIManagerError::InputMethodServiceNotFound(e)
            })?;

        let window: JObject<'_> = jni_env
            .call_method(
                &activity,
                "getWindow",
                "()Landroid/view/Window;",
                &[],
            )
            .unwrap()
            .l()
            .map_err(|e| JNIManagerError::WindowCouldNotBeGet(e))?;

        let view = jni_env
            .call_method(
                window,
                "getDecorView",
                "()Landroid/view/View;",
                &[],
            )
            .unwrap()
            .l()
            .map_err(|e| JNIManagerError::ViewCouldNotBeGet(e))?;

        return Ok(Self {
            _activity: activity,
            _jni_env: jni_env,
            _class_ctxt: class_ctxt,
            input_method_service,
            view,
        });
    }
}
