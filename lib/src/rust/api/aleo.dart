// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.32.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

String privateKeyFromSeed({required List<int> seed, dynamic hint}) =>
    RustLib.instance.api.privateKeyFromSeed(seed: seed, hint: hint);

String privateKeyToViewKey({required String privateKey, dynamic hint}) =>
    RustLib.instance.api
        .privateKeyToViewKey(privateKey: privateKey, hint: hint);

String privateKeyToAddress({required String privateKey, dynamic hint}) =>
    RustLib.instance.api
        .privateKeyToAddress(privateKey: privateKey, hint: hint);

String signMessage(
        {required List<int> messageBytes,
        required String privateKey,
        dynamic hint}) =>
    RustLib.instance.api.signMessage(
        messageBytes: messageBytes, privateKey: privateKey, hint: hint);