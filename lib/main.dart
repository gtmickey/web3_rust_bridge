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
        body: ListView(
          children: [
            Text("privateKey: $privateKey"),
            const Divider(),
            Text("viewKey: ${privateKeyToViewKey(privateKey: privateKey)}"),
            const Divider(),
            Text("address: ${privateKeyToAddress(privateKey: privateKey)}"),
            const Divider(),
            Text("signMessage: ${signMessage(messageBytes: [
                  0
                ], privateKey: privateKey)}"),
            const Divider(),
            FutureBuilder(
                future: genDelegateData(),
                builder: (context, snapshot) {
                  if (snapshot.hasData) {
                    return Column(
                      crossAxisAlignment: CrossAxisAlignment.stretch,
                      children: snapshot.data!.map((e) => Text(e)).toList(),
                    );
                  } else {
                    return const Text("generating");
                  }
                })
          ],
        ),
      ),
    );
  }

  void genPrivateKey() {
    privateKey = privateKeyFromSeed(seed: testSeed);
  }

  Future<List<String>> genDelegateData() async {
    return await delegateTransferPublic(
      privateKey: privateKey,
      amountCredits: 0.001,
      recipient:
          "aleo19jjmsrusvuduyxgufd7ax24p2sp73eedx0agky7tzfa0su66wcgqlmqz4x",
      feeCredits: 0.28,
    );
  }
}
