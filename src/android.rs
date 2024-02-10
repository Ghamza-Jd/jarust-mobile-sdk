use jni::objects::JClass;
use jni::objects::JObject;
use jni::JNIEnv;

#[no_mangle]
#[export_name = "Java_com_ghamza_jarust_wrappers_JaContext_initTls"]
pub extern "system" fn java_init(env: JNIEnv, _class: JClass, context: JObject) {
    log::info!("initTls");
    _ = rustls_platform_verifier::android::init_hosted(&env, context);
}
