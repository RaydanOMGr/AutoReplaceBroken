package me.andreasmelone.autoreplacebroken.jni;

import java.io.FileOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.nio.file.Files;
import java.nio.file.Path;

public class NativeLibraryLoader {
    private static final OperatingSystem OS = OperatingSystem.get();
    private static final Architecture ARCH = Architecture.get();

    public static String extractLibrary(String libName) {
        String fullLibName = OS.getPrefix() + libName + OS.getExtension();
        String directory = "/" + OS.name().toLowerCase() + "/" + ARCH.name().toLowerCase() + "/";

        String libPath = directory + fullLibName;

        String extractedLibPath;
        try(InputStream in = NativeLibraryLoader.class.getResourceAsStream(libPath)) {
            if(in == null) throw new IOException("No such file as " + libPath);

            Path tempFile = Files.createTempFile(fullLibName, OS.getExtension());
            tempFile.toFile().deleteOnExit();
            extractedLibPath = tempFile.toAbsolutePath().toString();

            OutputStream out = new FileOutputStream(tempFile.toFile());
            in.transferTo(out);
            out.close();
        } catch (IOException e) {
            throw new RuntimeException("Failed to extract library!", e);
        }

        return extractedLibPath;
    }

    public enum OperatingSystem {
        WINDOWS("", ".dll"),
        MACOS("lib", ".dylib"),
        LINUX("lib", ".so");

        private final String prefix;
        private final String extension;

        OperatingSystem(String prefix, String extension) {
            this.prefix = prefix;
            this.extension = extension;
        }

        public String getPrefix() {
            return prefix;
        }

        public String getExtension() {
            return extension;
        }

        public static OperatingSystem get() {
            String os = System.getProperty("os.name").toLowerCase();

            if (os.contains("win")) {
                return WINDOWS;
            }
            if (os.contains("mac")) {
                return MACOS;
            }
            if (os.contains("nix") || os.contains("nux") || os.contains("aix")) {
                return LINUX;
            }

            throw new IllegalStateException("Unsupported operating system: " + os);
        }
    }

    public enum Architecture {
        AMD64,
        X86,
        AARCH64;

        public static Architecture get() {
            String arch = System.getProperty("os.arch").toLowerCase();

            return switch (arch) {
                case "amd64", "x86_64" -> AMD64;
                case "x86", "i386", "i686" -> X86;
                case "aarch64", "arm64" -> AARCH64;
                default -> throw new IllegalStateException("Unsupported architecture: " + arch);
            };

        }
    }
}
