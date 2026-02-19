plugins {
    id("java")
}

val osName = System.getProperty("os.name").lowercase()
val archName = System.getProperty("os.arch").lowercase()

val osFolder = when {
    osName.contains("win") -> "windows"
    osName.contains("mac") -> "macos"
    osName.contains("nix") || osName.contains("nux") -> "linux"
    else -> error("Unsupported OS: $osName")
}

val archFolder = when (archName) {
    "amd64", "x86_64" -> "amd64"
    "x86", "i386", "i686" -> "x86"
    "aarch64", "arm64" -> "aarch64"
    else -> error("Unsupported architecture: $archName")
}

group = "me.andreasmelone"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
    maven {
        name = "spigotmc-repo"
        url = uri("https://hub.spigotmc.org/nexus/content/repositories/snapshots/")
    }
}

dependencies {
    compileOnly("org.spigotmc:spigot-api:1.21.11-R0.1-SNAPSHOT")
}

val targetJavaVersion = 21

java {
    val javaVersion = JavaVersion.toVersion(targetJavaVersion)
    sourceCompatibility = javaVersion
    targetCompatibility = javaVersion

    if (JavaVersion.current() < javaVersion) {
        toolchain.languageVersion.set(JavaLanguageVersion.of(targetJavaVersion))
    }
}

tasks.withType<JavaCompile>().configureEach {
    options.encoding = "UTF-8"

    if (targetJavaVersion >= 10 || JavaVersion.current().isJava10Compatible) {
        options.release.set(targetJavaVersion)
    }
}

tasks.processResources {
    dependsOn("copyNatives")

    val props = mapOf("version" to project.version)

    inputs.properties(props)
    filteringCharset = "UTF-8"

    filesMatching("plugin.yml") {
        expand(props)
    }
}

tasks.register("buildNativesRelease") {
    group = "build"

    doLast {
        println("Building natives")
        val startTime = System.currentTimeMillis()

        project.exec {
            workingDir = file("rust")
            commandLine("cargo", "build", "--release")
        }

        println("Finished building natives in ${System.currentTimeMillis() - startTime}ms")
    }
}

tasks.register<Copy>("copyNatives") {
    group = "build"
    dependsOn("buildNativesRelease")

    val rustTargetDir = file("rust/target/release")

    from(rustTargetDir) {
        include("*.dll", "*.so", "*.dylib")
    }

    into(layout.buildDirectory.dir("resources/main/$osFolder/$archFolder"))
}