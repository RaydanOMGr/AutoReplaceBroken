package me.andreasmelone.autoreplacebroken.jni;

import org.bukkit.event.block.BlockBreakEvent;
import org.bukkit.plugin.java.JavaPlugin;

public class ARBRustGlue {
    public static native void blockBrokenListener(BlockBreakEvent event);
    public static native void pluginOnEnable(JavaPlugin plugin);
    public static native void pluginOnDisable(JavaPlugin plugin);

    static {
        System.load(NativeLibraryLoader.extractLibrary("rust_arb"));
    }
}
