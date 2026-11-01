plugins {
    alias(libs.plugins.android.application)
    alias(libs.plugins.kotlin.compose)
}

android {
    namespace = "dev.dbatrainer.app"
    compileSdk {
        version = release(37)
    }

    defaultConfig {
        applicationId = "dev.dbatrainer.app"
        minSdk = 26
        targetSdk = 37
        versionCode = 7
        versionName = "0.7.0"

        // Direct-download release APKs target modern physical Android devices.
        // Without an ABI filter the Godot AAR contributes arm64-v8a,
        // armeabi-v7a, x86 and x86_64 copies of libgodot_android.so, inflating
        // a ~70 MB ARM64 game runtime into a ~300 MB universal APK.
        ndk {
            abiFilters += "arm64-v8a"
        }

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
    }

    buildTypes {
        release {
            optimization {
                enable = false
            }
        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_11
        targetCompatibility = JavaVersion.VERSION_11
    }
    buildFeatures {
        compose = true
    }

    sourceSets {
        getByName("main").assets.srcDir("../../dba-world")
    }

    androidResources {
        // Godot projects may contain hidden metadata directories. Keep the
        // official Godot-compatible asset ignore pattern instead of Android's
        // default pattern that drops hidden project data.
        ignoreAssetsPattern = "!.svn:!.git:!.gitignore:!.ds_store:!*.scc:<dir>_*:!CVS:!thumbs.db:!picasa.ini:!*~"
    }
}

dependencies {
    implementation("org.godotengine:godot:4.7.2.stable")
    implementation("androidx.fragment:fragment:1.8.6")
    implementation(platform(libs.androidx.compose.bom))
    implementation(libs.androidx.activity.compose)
    implementation(libs.androidx.compose.material3)
    implementation(libs.androidx.compose.ui)
    implementation(libs.androidx.compose.ui.graphics)
    implementation(libs.androidx.compose.ui.tooling.preview)
    implementation(libs.androidx.core.ktx)
    implementation(libs.androidx.lifecycle.runtime.ktx)
    testImplementation(libs.junit)
    androidTestImplementation(platform(libs.androidx.compose.bom))
    androidTestImplementation(libs.androidx.compose.ui.test.junit4)
    androidTestImplementation(libs.androidx.espresso.core)
    androidTestImplementation(libs.androidx.junit)
    debugImplementation(libs.androidx.compose.ui.test.manifest)
    debugImplementation(libs.androidx.compose.ui.tooling)
    implementation(
        "net.java.dev.jna:jna:5.17.0@aar"
    )
}