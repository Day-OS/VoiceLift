build-android:
	cargo ndk --manifest-path Cargo.toml -t arm64-v8a -o android/app/src/main/jniLibs build
	cd android/ && ./gradlew assembleDebug
	adb install -t ./android/app/build/outputs/apk/debug/app-debug.apk
	adb shell monkey -p com.daytheipc.VoiceLift -c android.intent.category.LAUNCHER 1

log-android:
	adb logcat -s AndroidRuntime