import 'package:flutter/material.dart';
import 'package:web3_rust_bridge/src/rust/api/aleo.dart';
import 'package:web3_rust_bridge/src/rust/api/simple.dart';
import 'package:web3_rust_bridge/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

const testSeed = [
  110,
  226,
  76,
  139,
  138,
  102,
  149,
  114,
  86,
  182,
  255,
  41,
  89,
  215,
  168,
  130,
  167,
  121,
  29,
  246,
  251,
  144,
  73,
  66,
  126,
  103,
  13,
  199,
  251,
  110,
  66,
  221
];

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  String privateKey = "";

  @override
  void initState() {
    genPrivateKey();
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('web3_rust_bridge')),
        body: Column(
          children: [
            Text("privateKey: $privateKey"),
            Text("viewKey: ${privateKeyToViewKey(privateKey: privateKey)}"),
            Text("address: ${privateKeyToAddress(privateKey: privateKey)}"),
            Text("signMessage: ${signMessage(messageBytes:[0],privateKey: privateKey)}"),
          ],
        ),
      ),
    );
  }

  void genPrivateKey() {
    privateKey = privateKeyFromSeed(seed: testSeed);
  }
}
