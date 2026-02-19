use jni::{jni_sig, jni_str, Env, JValue};
use jni::objects::{JObject, JString};
use jni::refs::Global;
use crate::material::Material;

#[allow(dead_code)]
pub trait Cancellable<'caller> {
    fn cancelled(&self, env: &mut Env<'caller>) -> Result<bool, jni::errors::Error>;
    fn set_cancelled(&self, env: &mut Env<'caller>, cancelled: bool)-> Result<(), jni::errors::Error>;
}

pub struct WPluginManager {
    _internal_object: Global<JObject<'static>>
}

impl<'caller> WPluginManager {
    pub(crate) fn new(obj: Global<JObject<'static>>) -> Self {
        Self { _internal_object: obj }
    }

    pub fn register_events(&self, env: &mut Env<'caller>, listener: JObject, plugin: WJavaPlugin) -> Result<(), jni::errors::Error> {
        env.call_method(
            self._internal_object.as_obj(),
            jni_str!("registerEvents"),
            jni_sig!("(Lorg/bukkit/event/Listener;Lorg/bukkit/plugin/Plugin;)V"),
            &[JValue::Object(&listener), JValue::Object(plugin._internal_object.as_obj())]
        )?;
        Ok(())
    }
}

pub struct WServer {
    _internal_object: Global<JObject<'static>>
}

impl<'caller> WServer {
    pub(crate) fn new(obj: Global<JObject<'static>>) -> Self {
        Self { _internal_object: obj }
    }

    pub fn plugin_manager(&self, env: &mut Env<'caller>) -> Result<WPluginManager, jni::errors::Error> {
        let result = env.call_method(
            self._internal_object.as_obj(),
            jni_str!("getPluginManager"),
            jni_sig!("()Lorg/bukkit/plugin/PluginManager;"),
            &[]
        )?;
        let obj = result.l()?;
        let global = env.new_global_ref(obj)?;
        Ok(WPluginManager::new(global))
    }
}

pub struct WJavaPlugin {
    _internal_object: Global<JObject<'static>>
}

impl<'caller> WJavaPlugin {
    pub(crate) fn new(obj: Global<JObject<'static>>) -> Self {
        Self { _internal_object: obj }
    }

    pub fn name(&self, env: &mut Env<'caller>) ->  Result<String, jni::errors::Error> {
        let result = env.call_method(self._internal_object.as_obj(), jni_str!("getName"), jni_sig!("()Ljava/lang/String;"), &[]);
        let obj = result?.l()?;
        let jstr = env.cast_local::<JString>(obj)?;
        Ok(jstr.try_to_string(&env)?)
    }

    pub fn server(&self, env: &mut Env<'caller>) -> Result<WServer, jni::errors::Error> {
        let result = env.call_method(
            self._internal_object.as_obj(),
            jni_str!("getServer"),
            jni_sig!("()Lorg/bukkit/Server;"),
            &[]
        )?;
        let obj = result.l()?;
        let global = env.new_global_ref(obj)?;
        Ok(WServer::new(global))
    }
}

pub struct WBlock {
    _internal_object: Global<JObject<'static>>
}

impl<'caller> WBlock {
    pub(crate) fn new(obj: Global<JObject<'static>>) -> Self {
        Self { _internal_object: obj }
    }

    pub fn x(&self, env: &mut Env<'caller>) -> Result<i32, jni::errors::Error> {
        let result = env.call_method(self._internal_object.as_obj(), jni_str!("getX"), jni_sig!("()I"), &[]);
        result?.i()
    }

    pub fn y(&self, env: &mut Env<'caller>) -> Result<i32, jni::errors::Error> {
        let result = env.call_method(self._internal_object.as_obj(), jni_str!("getY"), jni_sig!("()I"), &[]);
        result?.i()
    }

    pub fn z(&self, env: &mut Env<'caller>) -> Result<i32, jni::errors::Error> {
        let result = env.call_method(self._internal_object.as_obj(), jni_str!("getZ"), jni_sig!("()I"), &[]);
        result?.i()
    }

    pub fn set_type(&self, env: &mut Env<'caller>, material: Material) -> Result<(), jni::errors::Error> {
        let java_material = material.to_java(env)?;
        env.call_method(self._internal_object.as_obj(), jni_str!("setType"), jni_sig!("(Lorg/bukkit/Material;)V"), &[JValue::Object(&java_material)])?;

        Ok(())
    }
}


pub struct WBlockBreakEvent {
    _internal_object: Global<JObject<'static>>
}

impl<'caller> WBlockBreakEvent {
    pub(crate) fn new(obj: Global<JObject<'static>>) -> Self {
        Self { _internal_object: obj }
    }

    pub fn block(&self, env: &mut Env<'caller>) -> Result<WBlock, jni::errors::Error> {
        let result = env.call_method(self._internal_object.as_obj(), jni_str!("getBlock"), jni_sig!("()Lorg/bukkit/block/Block;"), &[])?;
        let global = env.new_global_ref(result.l()?)?;
        Ok(WBlock::new(global))
    }
}

impl<'caller> Cancellable<'caller> for WBlockBreakEvent {
    fn cancelled(&self, env: &mut Env<'caller>) -> Result<bool, jni::errors::Error> {
        let result = env.call_method(self._internal_object.as_obj(), jni_str!("isCancelled"), jni_sig!("()Z"), &[]);
        result?.z()
    }

    fn set_cancelled(&self, env: &mut Env<'caller>, cancelled: bool) -> Result<(), jni::errors::Error> {
         env.call_method(self._internal_object.as_obj(), jni_str!("setCancelled"), jni_sig!("(Z)V"), &[JValue::Bool(cancelled)])?;
        Ok(())
    }
}