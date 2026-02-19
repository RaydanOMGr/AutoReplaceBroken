#![allow(non_snake_case)]

mod wrappers;
mod material;

use jni::{jni_sig, jni_str, EnvUnowned};
use jni::objects::{JClass, JObject};
use crate::material::Material;
use crate::wrappers::{Cancellable, WBlockBreakEvent, WJavaPlugin};

#[unsafe(no_mangle)]
pub extern "system" fn Java_me_andreasmelone_autoreplacebroken_jni_ARBRustGlue_pluginOnEnable<'caller>(
    mut unowned_env: EnvUnowned<'caller>,
    _class: JClass<'caller>,
    pluginObj: JObject<'caller>
) {
    let outcome = unowned_env.with_env(|env| -> Result<_, jni::errors::Error> {
        let globalObj = env.new_global_ref(pluginObj)?;
        let plugin = WJavaPlugin::new(globalObj);

        println!("Loading plugin {}", plugin.name(env)?);

        let listenerClass = env.find_class(jni_str!("me/andreasmelone/autoreplacebroken/ARBListener"))?;
        let listener = env.new_object(listenerClass, jni_sig!("()V"), &[])?;
        plugin.server(env)?.plugin_manager(env)?.register_events(env, listener, plugin)?;

        Ok(())
    });


    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>();
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_me_andreasmelone_autoreplacebroken_jni_ARBRustGlue_pluginOnDisable<'caller>(
    mut unowned_env: EnvUnowned<'caller>,
    _class: JClass<'caller>,
    pluginObj: JObject<'caller>
) {
    let outcome = unowned_env.with_env(|env| -> Result<_, jni::errors::Error> {
        let globalObj = env.new_global_ref(pluginObj)?;
        let plugin = WJavaPlugin::new(globalObj);

        println!("Disabling plugin {}", plugin.name(env)?);

        Ok(())
    });


    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>();
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_me_andreasmelone_autoreplacebroken_jni_ARBRustGlue_blockBrokenListener<'caller>(
    mut unowned_env: EnvUnowned<'caller>,
    _class: JClass<'caller>,
    event: JObject<'caller>
) {
    let outcome = unowned_env.with_env(|env| -> Result<_, jni::errors::Error> {
        let globalObj = env.new_global_ref(event)?;
        let event = WBlockBreakEvent::new(globalObj);
        let block = event.block(env)?;

        println!("Replacing block at {}, {}, {}", block.x(env)?, block.y(env)?, block.z(env)?);
        event.set_cancelled(env, true)?;
        block.set_type(env, Material::Bedrock)?;

        Ok(())
    });


    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>();
}