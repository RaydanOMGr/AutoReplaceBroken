package me.andreasmelone.autoreplacebroken;

import me.andreasmelone.autoreplacebroken.jni.ARBRustGlue;
import org.bukkit.plugin.java.JavaPlugin;

public final class AutoReplaceBroken extends JavaPlugin {
    @Override
    public void onEnable() {
        // Plugin startup logic
        ARBRustGlue.pluginOnEnable(this);
    }

    @Override
    public void onDisable() {
        // Plugin shutdown logic
        ARBRustGlue.pluginOnDisable(this);
    }
}
