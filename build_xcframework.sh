#!/bin/bash

xcodebuild -create-xcframework \
-framework ./build/ios/Release-iphoneos/rust_lib_web3_rust_bridge/rust_lib_web3_rust_bridge.framework \
-framework ./build/ios/Debug-iphonesimulator/rust_lib_web3_rust_bridge/rust_lib_web3_rust_bridge.framework \
-output ./rust_lib_web3_rust_bridge.xcframework

