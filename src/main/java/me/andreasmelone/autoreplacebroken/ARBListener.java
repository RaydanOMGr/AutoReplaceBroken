package me.andreasmelone.autoreplacebroken;

import me.andreasmelone.autoreplacebroken.jni.ARBRustGlue;
import org.bukkit.event.EventHandler;
import org.bukkit.event.Listener;
import org.bukkit.event.block.BlockBreakEvent;

public class ARBListener implements Listener {
    @EventHandler
    public void onBlockBreak(BlockBreakEvent event) {
        ARBRustGlue.blockBrokenListener(event);
    }
}
