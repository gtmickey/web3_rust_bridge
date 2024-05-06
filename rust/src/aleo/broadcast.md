## send data to remote to proof
```sh
curl --location 'https://testnet3.aleorpc.com' \
--header 'Content-Type: application/json' \
--data '{
"jsonrpc": "2.0",
"id": 1,
"method": "generateTransaction",
"params": {
"authorization": "{\"requests\":[{\"signer\":
\"aleo1m8gqcxedmqfp2ylh8f96w6n3z7zw0ucahenq0symvxpqg0f8sugqd4we6f\",\"network\":\"3u16\",\"program\":
\"credits.aleo\",\"function\":
\"transfer_public\",\"input_ids\":[{\"type\":\"public\",\"id\":\"2533024833492545021680503518219816669553518990471008452114661788061726291626field\"},{\"type\":\"public\",\"id\":\"886200034544229187440529508525069490334320776590572047360057075269369876field\"}]
,\"inputs\":[\"aleo19jjmsrusvuduyxgufd7ax24p2sp73eedx0agky7tzfa0su66wcgqlmqz4x\",\"12340u64\"],\"signature\":
\"sign17r33mqtu6yvhrv8vj7gtgptn8k74y4m4tz54c3d9y2aydnr4rvq6prpfk82h9d6q92q0yu3sfrmpgsksw4twx36k6th6tqfvq7r9zqg0e9ecyvv7l2ekqytdkzw8upulsdtf02qt9wafma4h2yfzmyqjp57tdcx3t8mrqs7f4av3h63k8z6uynqkegzwfpv5222r9p0x6cdqvhwh3jg\",\"sk_tag\":
\"712427325382097979802266333337952996706198585861800260066519815736813730312field\",\"tvk\":
\"5395221624627577698502505025302703100714607146518612992519525869820074925976field\",\"tcm\":
\"1876600899613494633058641059669265427183324236058665783455266274444170901520field\"}],\"transitions\":[{\"id\":
\"au1hw9jh6j6tkk5wk7fyzanpd9m89rjl9y6w44mgu7pkltgav5e3uxqxhc0yw\",\"program\":\"credits.aleo\",\"function\":
\"transfer_public\",\"inputs\":[{\"type\":\"public\",\"id\":\"2533024833492545021680503518219816669553518990471008452114661788061726291626field\",\"value\":\"aleo19jjmsrusvuduyxgufd7ax24p2sp73eedx0agky7tzfa0su66wcgqlmqz4x\"},{\"type\":\"public\",\"id\":\"886200034544229187440529508525069490334320776590572047360057075269369876field\",\"value\":\"12340u64\"}]
,\"outputs\":[{\"type\":\"future\",\"id\":
\"5920280785837407732071966475002833698283734267289773347475191165567489299837field\",\"value\":\"{\\n program_id:
credits.aleo,\\n function_name: transfer_public,\\n
arguments: [\\n    aleo1m8gqcxedmqfp2ylh8f96w6n3z7zw0ucahenq0symvxpqg0f8sugqd4we6f,\\n    aleo19jjmsrusvuduyxgufd7ax24p2sp73eedx0agky7tzfa0su66wcgqlmqz4x,\\n    12340u64\\n  ]
\\n}\"}],\"tpk\":\"1063296491145922724520991992157282320766032811596271492534716679368062212203group\",\"tcm\":
\"1876600899613494633058641059669265427183324236058665783455266274444170901520field\"}]}",
"program": "program credits.aleo;\n\nmapping committee:\n key as address.public;\n value as
committee_state.public;\n\nstruct committee_state:\n microcredits as u64;\n is_open as boolean;\n\nmapping bonded:\n key
as address.public;\n value as bond_state.public;\n\nstruct bond_state:\n validator as address;\n microcredits as
u64;\n\nmapping unbonding:\n key as address.public;\n value as unbond_state.public;\n\nstruct unbond_state:\n
microcredits as u64;\n height as u32;\n\nmapping account:\n key as address.public;\n value as u64.public;\n\nrecord
credits:\n owner as address.private;\n microcredits as u64.private;\n\nfunction bond_public:\n input r0 as
address.public;\n input r1 as u64.public;\n gte r1 1000000u64 into r2;\n assert.eq r2 true ;\n async bond_public
self.caller r0 r1 into r3;\n output r3 as credits.aleo/bond_public.future;\n\nfinalize bond_public:\n input r0 as
address.public;\n input r1 as address.public;\n input r2 as u64.public;\n is.eq r0 r1 into r3;\n branch.eq r3 true to
bond_validator;\n branch.eq r3 false to bond_delegator;\n position bond_validator;\n cast 0u64 true into r4 as
committee_state;\n get.or_use committee[r0] r4 into r5;\n assert.eq r5.is_open true ;\n add r5.microcredits r2 into
r6;\n cast r6 r5.is_open into r7 as committee_state;\n cast r1 0u64 into r8 as bond_state;\n get.or_use bonded[r0] r8
into r9;\n assert.eq r9.validator r1 ;\n add r9.microcredits r2 into r10;\n gte r10 1000000000000u64 into r11;\n
assert.eq r11 true ;\n cast r1 r10 into r12 as bond_state;\n get account[r0] into r13;\n sub r13 r2 into r14;\n set r7
into committee[r0];\n set r12 into bonded[r0];\n set r14 into account[r0];\n branch.eq true true to end;\n position
bond_delegator;\n contains committee[r0] into r15;\n assert.eq r15 false ;\n get committee[r1] into r16;\n assert.eq
r16.is_open true ;\n add r16.microcredits r2 into r17;\n cast r17 r16.is_open into r18 as committee_state;\n cast r1
0u64 into r19 as bond_state;\n get.or_use bonded[r0] r19 into r20;\n assert.eq r20.validator r1 ;\n add r20.microcredits
r2 into r21;\n gte r21 10000000u64 into r22;\n assert.eq r22 true ;\n cast r1 r21 into r23 as bond_state;\n get
account[r0] into r24;\n sub r24 r2 into r25;\n set r18 into committee[r1];\n set r23 into bonded[r0];\n set r25 into
account[r0];\n position end;\n\nfunction unbond_public:\n input r0 as u64.public;\n async unbond_public self.caller r0
into r1;\n output r1 as credits.aleo/unbond_public.future;\n\nfinalize unbond_public:\n input r0 as address.public;\n
input r1 as u64.public;\n cast 0u64 0u32 into r2 as unbond_state;\n get.or_use unbonding[r0] r2 into r3;\n add
block.height 360u32 into r4;\n contains committee[r0] into r5;\n branch.eq r5 true to unbond_validator;\n branch.eq r5
false to unbond_delegator;\n position unbond_validator;\n get committee[r0] into r6;\n sub r6.microcredits r1 into r7;\n
get bonded[r0] into r8;\n assert.eq r8.validator r0 ;\n sub r8.microcredits r1 into r9;\n gte r9 1000000000000u64 into
r10;\n branch.eq r10 true to decrement_validator;\n branch.eq r10 false to remove_validator;\n position
decrement_validator;\n cast r7 r6.is_open into r11 as committee_state;\n set r11 into committee[r0];\n cast r0 r9 into
r12 as bond_state;\n set r12 into bonded[r0];\n add r3.microcredits r1 into r13;\n cast r13 r4 into r14 as
unbond_state;\n set r14 into unbonding[r0];\n branch.eq true true to end;\n position remove_validator;\n assert.eq
r6.microcredits r8.microcredits ;\n remove committee[r0];\n remove bonded[r0];\n add r3.microcredits r8.microcredits
into r15;\n cast r15 r4 into r16 as unbond_state;\n set r16 into unbonding[r0];\n branch.eq true true to end;\n position
unbond_delegator;\n get bonded[r0] into r17;\n sub r17.microcredits r1 into r18;\n gte r18 10000000u64 into r19;\n
branch.eq r19 true to decrement_delegator;\n branch.eq r19 false to remove_delegator;\n position decrement_delegator;\n
get committee[r17.validator] into r20;\n sub r20.microcredits r1 into r21;\n cast r21 r20.is_open into r22 as
committee_state;\n set r22 into committee[r17.validator];\n cast r17.validator r18 into r23 as bond_state;\n set r23
into bonded[r0];\n add r3.microcredits r1 into r24;\n cast r24 r4 into r25 as unbond_state;\n set r25 into unbonding[r0]
;\n branch.eq true true to end;\n position remove_delegator;\n get committee[r17.validator] into r26;\n sub
r26.microcredits r17.microcredits into r27;\n cast r27 r26.is_open into r28 as committee_state;\n set r28 into
committee[r17.validator];\n remove bonded[r0];\n add r3.microcredits r17.microcredits into r29;\n cast r29 r4 into r30
as unbond_state;\n set r30 into unbonding[r0];\n position end;\n\nfunction unbond_delegator_as_validator:\n input r0 as
address.public;\n async unbond_delegator_as_validator self.caller r0 into r1;\n output r1 as
credits.aleo/unbond_delegator_as_validator.future;\n\nfinalize unbond_delegator_as_validator:\n input r0 as
address.public;\n input r1 as address.public;\n get committee[r0] into r2;\n assert.eq r2.is_open false ;\n contains
committee[r1] into r3;\n assert.eq r3 false ;\n get bonded[r1] into r4;\n assert.eq r4.validator r0 ;\n sub
r2.microcredits r4.microcredits into r5;\n cast r5 r2.is_open into r6 as committee_state;\n cast 0u64 0u32 into r7 as
unbond_state;\n get.or_use unbonding[r1] r7 into r8;\n add r8.microcredits r4.microcredits into r9;\n add block.height
360u32 into r10;\n cast r9 r10 into r11 as unbond_state;\n set r6 into committee[r0];\n remove bonded[r1];\n set r11
into unbonding[r1];\n\nfunction claim_unbond_public:\n async claim_unbond_public self.caller into r0;\n output r0 as
credits.aleo/claim_unbond_public.future;\n\nfinalize claim_unbond_public:\n input r0 as address.public;\n get
unbonding[r0] into r1;\n gte block.height r1.height into r2;\n assert.eq r2 true ;\n get.or_use account[r0] 0u64 into
r3;\n add r1.microcredits r3 into r4;\n set r4 into account[r0];\n remove unbonding[r0];\n\nfunction
set_validator_state:\n input r0 as boolean.public;\n async set_validator_state self.caller r0 into r1;\n output r1 as
credits.aleo/set_validator_state.future;\n\nfinalize set_validator_state:\n input r0 as address.public;\n input r1 as
boolean.public;\n get committee[r0] into r2;\n cast r2.microcredits r1 into r3 as committee_state;\n set r3 into
committee[r0];\n\nfunction transfer_public:\n input r0 as address.public;\n input r1 as u64.public;\n async
transfer_public self.caller r0 r1 into r2;\n output r2 as credits.aleo/transfer_public.future;\n\nfinalize
transfer_public:\n input r0 as address.public;\n input r1 as address.public;\n input r2 as u64.public;\n get account[r0]
into r3;\n sub r3 r2 into r4;\n set r4 into account[r0];\n get.or_use account[r1] 0u64 into r5;\n add r5 r2 into r6;\n
set r6 into account[r1];\n\nfunction transfer_private:\n input r0 as credits.record;\n input r1 as address.private;\n
input r2 as u64.private;\n sub r0.microcredits r2 into r3;\n cast r1 r2 into r4 as credits.record;\n cast r0.owner r3
into r5 as credits.record;\n output r4 as credits.record;\n output r5 as credits.record;\n\nfunction
transfer_private_to_public:\n input r0 as credits.record;\n input r1 as address.public;\n input r2 as u64.public;\n sub
r0.microcredits r2 into r3;\n cast r0.owner r3 into r4 as credits.record;\n async transfer_private_to_public r1 r2 into
r5;\n output r4 as credits.record;\n output r5 as credits.aleo/transfer_private_to_public.future;\n\nfinalize
transfer_private_to_public:\n input r0 as address.public;\n input r1 as u64.public;\n get.or_use account[r0] 0u64 into
r2;\n add r1 r2 into r3;\n set r3 into account[r0];\n\nfunction transfer_public_to_private:\n input r0 as
address.private;\n input r1 as u64.public;\n cast r0 r1 into r2 as credits.record;\n async transfer_public_to_private
self.caller r1 into r3;\n output r2 as credits.record;\n output r3 as
credits.aleo/transfer_public_to_private.future;\n\nfinalize transfer_public_to_private:\n input r0 as address.public;\n
input r1 as u64.public;\n get account[r0] into r2;\n sub r2 r1 into r3;\n set r3 into account[r0];\n\nfunction join:\n
input r0 as credits.record;\n input r1 as credits.record;\n add r0.microcredits r1.microcredits into r2;\n cast r0.owner
r2 into r3 as credits.record;\n output r3 as credits.record;\n\nfunction split:\n input r0 as credits.record;\n input r1
as u64.private;\n sub r0.microcredits r1 into r2;\n sub r2 10000u64 into r3;\n cast r0.owner r1 into r4 as
credits.record;\n cast r0.owner r3 into r5 as credits.record;\n output r4 as credits.record;\n output r5 as
credits.record;\n\nfunction fee_private:\n input r0 as credits.record;\n input r1 as u64.public;\n input r2 as
u64.public;\n input r3 as field.public;\n assert.neq r1 0u64 ;\n assert.neq r3 0field ;\n add r1 r2 into r4;\n sub
r0.microcredits r4 into r5;\n cast r0.owner r5 into r6 as credits.record;\n output r6 as credits.record;\n\nfunction
fee_public:\n input r0 as u64.public;\n input r1 as u64.public;\n input r2 as field.public;\n assert.neq r0 0u64 ;\n
assert.neq r2 0field ;\n add r0 r1 into r3;\n async fee_public self.caller r3 into r4;\n output r4 as
credits.aleo/fee_public.future;\n\nfinalize fee_public:\n input r0 as address.public;\n input r1 as u64.public;\n get
account[r0] into r2;\n sub r2 r1 into r3;\n set r3 into account[r0];\n",
"fee_authorization": "{\"requests\":[{\"signer\":
\"aleo1m8gqcxedmqfp2ylh8f96w6n3z7zw0ucahenq0symvxpqg0f8sugqd4we6f\",\"network\":\"3u16\",\"program\":
\"credits.aleo\",\"function\":
\"fee_public\",\"input_ids\":[{\"type\":\"public\",\"id\":\"525428757546279155890949540248607055818017932703299244218582609603846870130field\"},{\"type\":\"public\",\"id\":\"167728931086940746984196295256217455955262186380036487381110244498719049485field\"},{\"type\":\"public\",\"id\":\"5410115237092390163066972709172943872459963809344597740778590141095482237341field\"}]
,\"inputs\":[\"268000u64\",\"0u64\",\"1759448136754468268842952667481043455728991261481191864247177858696997339485field\"]
,\"signature\":
\"sign1wu2e2z7y3gklyd33edpzjtgsjgrngsw3kv5nyjfspwgmxpusj5quugnzzp58at5jvcxthsqqrnk54z2kfrkk65g6y9pl3hv4u0udkqg0e9ecyvv7l2ekqytdkzw8upulsdtf02qt9wafma4h2yfzmyqjp57tdcx3t8mrqs7f4av3h63k8z6uynqkegzwfpv5222r9p0x6cdqvs09p9k\",\"sk_tag\":
\"712427325382097979802266333337952996706198585861800260066519815736813730312field\",\"tvk\":
\"2445799512714021924017254052383204681066284846137222462061738637720989944624field\",\"tcm\":
\"6577873821262412848537105211812271191338240391832196970362515098303531018218field\"}],\"transitions\":[{\"id\":
\"au1hfwaeecsmza3wnryk80mstzryg6ulmn2fzumvyh9nx346qds3czs9tlrwq\",\"program\":\"credits.aleo\",\"function\":
\"fee_public\",\"inputs\":[{\"type\":\"public\",\"id\":\"525428757546279155890949540248607055818017932703299244218582609603846870130field\",\"value\":\"268000u64\"},{\"type\":\"public\",\"id\":\"167728931086940746984196295256217455955262186380036487381110244498719049485field\",\"value\":\"0u64\"},{\"type\":\"public\",\"id\":\"5410115237092390163066972709172943872459963809344597740778590141095482237341field\",\"value\":\"1759448136754468268842952667481043455728991261481191864247177858696997339485field\"}]
,\"outputs\":[{\"type\":\"future\",\"id\":
\"3793983165497572092640339191828542706750606960066358008312120273822886048724field\",\"value\":\"{\\n program_id:
credits.aleo,\\n function_name: fee_public,\\n
arguments: [\\n    aleo1m8gqcxedmqfp2ylh8f96w6n3z7zw0ucahenq0symvxpqg0f8sugqd4we6f,\\n    268000u64\\n  ]\\n}\"}]
,\"tpk\":\"7501750370823479310398976988633090923260070287193608952282381194099902783732group\",\"tcm\":
\"6577873821262412848537105211812271191338240391832196970362515098303531018218field\"}]}",
"function": "transfer_public",
"broadcast": true,
"imports": {}
}
}'

```


return:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": "3f985531-d15a-4c3a-9815-cb208d3e02b9"
}

```

query statie

```sh
curl --location 'https://testnet3.aleorpc.com' \
--header 'Content-Type: application/json' \
--data '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getGeneratedTransaction",
    "params": {
        "request_id": "e3ded361-b5c8-48c2-95a3-1b9602ba0f49"
    }
}'

```

return as

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "transaction": "{\"type\":\"execute\",\"id\":\"at10ffnl9lqhhp996z4gwnclq975m9g6nqu4jd3fcrztngzw8x2psxqcq9yn8\",\"execution\":{\"transitions\":[{\"id\":\"au1jsuh05x4f4nqh6ysvk0afp3zd2yn3l52rcj7ey0zz2u9w2y57sxskvktfr\",\"program\":\"credits.aleo\",\"function\":\"transfer_public\",\"inputs\":[{\"type\":\"public\",\"id\":\"6303302255977421980251709226523010417697211452054344960346458448559412527664field\",\"value\":\"aleo19jjmsrusvuduyxgufd7ax24p2sp73eedx0agky7tzfa0su66wcgqlmqz4x\"},{\"type\":\"public\",\"id\":\"4188309871571405672958046136373880011274643971536562842848858729711388031352field\",\"value\":\"1000000000u64\"}],\"outputs\":[{\"type\":\"future\",\"id\":\"1391140366526189652124783991141399773173679395678182775039968883097448968086field\",\"value\":\"{\\n  program_id: credits.aleo,\\n  function_name: transfer_public,\\n  arguments: [\\n    aleo1m8gqcxedmqfp2ylh8f96w6n3z7zw0ucahenq0symvxpqg0f8sugqd4we6f,\\n    aleo19jjmsrusvuduyxgufd7ax24p2sp73eedx0agky7tzfa0su66wcgqlmqz4x,\\n    1000000000u64\\n  ]\\n}\"}],\"tpk\":\"4516004568108043986603101194530444389310641964267059618614778249222583062003group\",\"tcm\":\"2128648669777133605373953708151387999485180407424572271300522022482858975103field\"}],\"global_state_root\":\"sr1ynkl9mx5nly3z26l0p3jakmqqenevt6v5hjrwsumasml5hfdsqqsada49u\",\"proof\":\"proof1qyqsqqqqqqqqqqqpqqqqqqqqqqqfwkkemqud8kurnqhyzx5y9yxa0dsg9zquv24q45zntyhnvmvmlkkh2q72y7rfdcwrajdr8q9cv9cpq9j5s5ujlmne3lvkcypt2c8nvn6zte3a98h3a44wx5vvv8qryvmsm8d6ez07cvu4c72ne8gtmvqw3q9s4z6d3csuc549whxjev7x2gm487lf8zysgrqfuuptfprlymd8dcl2yq47s2n6k5acussxdv26jvqacqhkje7qt7gr05dspcxk8kn022ertv3fw3y09w2xfmwtessg4pgqwgtnay0cdejezc4ruf33mrsqmtwy6rjqthfwps80kep5d828zm5g8va9use99l02edg479s4spcms69hacysqzlgm4fymy9huergpzlvrpym76uwx99cyfje6kpz7u4gnsv6gpd3rg8vxc4vzcsurgntzkazfvk3l25k7xuwu2w7ye5rq8k8tcgg6gpftcauhufvfqc35av2v8z4qes6pcu8c9s4as4p9kltyav8r6a6hm690usgjgqgv5f43qzveam5j46mhh4mg5l2qj724fwtsupqzn0mmen9atwv6p6yy9huvjlwsfxfh23deflzyhv6pw0wt6q252qr2vgh0p2mw5aqx9a3rve253swll0ahljew9kmvw9f62fc82phpdv5nv2cpynxuxc3tjmz0fuq6pw7sl2w9uyzc2sl3874je75q8fc6j79rztxvpxn0weme44zh5855nvgs5hcl05u25eq9f4sxet35flfqwd0a5wgv94dxklnl2rfjywllfmkv9xz3y803lzz2tznd78ny4rankeaglqegaruezfh2ue8qpm2uqggwfzg8h85pj26a4t2l6z09kks6g3mg2aq8zgg0qj722zqwg0zeafd7ntckpwdyyrw5cl7yn9380a5s8fjaez65tgjysz6vcc0kmxhuvw8zvvf3rx03uzmaeqv940e3clcyfef9p9v6u7hdkztugpz3rmzfdhfhlk0zg84cv8afaxh8c95nygleal06skjgfqk6966crvj7nahzz608tdped0xq2s96m7swfnuryf87n6t5skhxr7lufklplqhl3tl4wlkgvgwyemv6g75vkmuusd88jqpqu5rzm6jk32lssgpqlexf26njue28hhnz0m8trlf8v2fnn7ukzuzfsh58xlevwnu08q8qvqqqqqqqqqqqmgg6zzah4dsxa7pjnwvsj4gcph664yjhpmda5a5g2752jhmf30js7c3hercnagkzrlzemmq745tsyqz7y7tx2zaa2dkpjgrg6k6pa4s5hznvphqv2zvfgsakr3gfshus9zxrk9d607zp8eff50tskpz2duqq992yfqtuk3cuygeszur0jgdzsnnxvhp3yf3fp2h5jwnx64d4emqm9glcszwsje5f98v3uq5gzdfpz7ke6pr83epj5kanpu5dmfaupczmtaec62565j3xmehv0ljntfhqqqq7u90dr\"},\"fee\":{\"transition\":{\"id\":\"au1t8afrhd5v43sp4dlnerp0l5xc5q3cetdq77yq6tgle5uk5gnhc8s58d2hd\",\"program\":\"credits.aleo\",\"function\":\"fee_public\",\"inputs\":[{\"type\":\"public\",\"id\":\"4516625680625243162611658308440666004342723429851208573779920099557476021363field\",\"value\":\"1000u64\"},{\"type\":\"public\",\"id\":\"8196707133670764268936130536179229236720039397709760048342040010027819576203field\",\"value\":\"0u64\"},{\"type\":\"public\",\"id\":\"2278210745184668348828336084480196836569503677851479115322244300181815326621field\",\"value\":\"83399486352960520006361205527967063854989717957857785926278837434399634685field\"}],\"outputs\":[{\"type\":\"future\",\"id\":\"1626132976308187472560168729064859261733112082086109458397482670780286221336field\",\"value\":\"{\\n  program_id: credits.aleo,\\n  function_name: fee_public,\\n  arguments: [\\n    aleo1m8gqcxedmqfp2ylh8f96w6n3z7zw0ucahenq0symvxpqg0f8sugqd4we6f,\\n    1000u64\\n  ]\\n}\"}],\"tpk\":\"743300292200735910177853457954880803153076165283767817395571740053400079667group\",\"tcm\":\"8273047587921265862334563139821610463135807837267648302590662837391455559868field\"},\"global_state_root\":\"sr1wk0esjvzaa5p72aaytpcvy2fe560q2kspgthrrrzm04zp36ncv8qm22ym6\",\"proof\":\"proof1qyqsqqqqqqqqqqqpqqqqqqqqqqq83f3fkgj64vt825207823akr7wyqk0rtnkn8lf7hlrdjd8u6xrsyrupzelj8xr9u5kf69xvjqkwspq8qnftzw2c89ngd2wpzea8t6x37uejy7lcfzmygqtkhnvg04zx0lm6xs83z596mfjrwtwuckdgkzlqx6aelyed7lv8surhx5qa44dh90jg2zjrc9ekvk9n236qe50el0eup93nth6atlfaytqg9w0uzz7zqxr5xxgktysa7qh4l0z2cfs22907ugng8d3j69dd4jjr0h4ahzljl2z7q0dmxlfr3f7cv3n0f58xuptr5mlmy9ecl6v8ukg5gfjchjkg0dguftphu4l5kaq2kgmnp23hr8v6s6g3kf8367dczcclx7yy9gp8376kxq8crnlnmq05ds8ls2rm93yvr0ucq50yaduvqzlzh2hk6nkd4828ufhh5vvmj6v5x2yqjzsq2w49v23mxvds4dewcm4q3cuwfrdszfcl8qv4r05zvy5pnuc32lg43rt5708zf8g98996w5cnapdq8s8dxk6gtexd2fx9hhfma6kats66jr8rg5azzhpcc5sq2qkh7sh4pef3fkhh088h85vdrfnyuejqqy9ce8ux8cul5ya33euyp5jtcs9wqtsja8sccqxt8wkj96fhxg2qpct7j5nsd0x36smckckswu7guquygeyyfrfjn2579rf9uf4xs7ca7hgrt4pp2l4remu7805evnzyyq4erv30uxun902a3vahegxzwv2d63vy53henc37vtz09rl09g6rvg0xhg4s6fsp323syakxpr2j3gnwdtt8sv7wg4jdflud76c6yzpg7mxp6wnypweta5wdxaxecskeh8v2e2g2fhn2zu8zm0nu48axpqt8cyc5cm8ur7t7yjjx0hnlzlvsqyf06uj06nacny6tu66dna5ms0x6xqnsqj64nkkpfrdakshnktv05wftgf4auhq3hq5xzaa4ez4c9lzmlq90knf2n730sqwrhe4erd0q5fajtej6j889ld3n8kd4wwjznksvtal0efgk05xu6v7f7j9fxe86yzsvjgdw5z4ux7l6xzdekfp8v0r7hsmmrfshz27y68qsf5l9ja7dusefeq0hrjevnsypdazxvqgewmky62dj4wzzvcgwwclcq3edjh3dvv2cnraxgfhkehvq76q4gwqvqqqqqqqqqqqgs0wsf2cqgpatu2fdez9fjg0449eu93yhzprq8n953858fj4svjr3lfe07ag9kldh2m3yfapd6vqqqystdaph6sg7w0tftmcjt7ld02dnxg4zp3vh5chhda8xyesgyuyvxfhazsvfvks87qjru0638gglqqqxs0tcwr0g6mr7gaam8dm6hdcu3v4zme7vzszcmuegeeflwy994sp7ys730lux0kjgwcdag7w40tk6m4f9j9jw27xnc9l73tw326n5anqlzdwjcjld2l3t4cpymzwpensqqq0ael4s\"}}",
    "status": "Broadcasted",
    "error": null,
    "updated_at": "2024-04-25T02:50:44.293980328Z"
  }
}
```

查询状态

```sh

curl --location --request POST 'https://testnet3.aleorpc.com' \  --header 'Content-Type: application/json' \  --data-raw '{    "jsonrpc": "2.0",    "id": 1,    "method": "aleoTransaction",    "params": {       "id": "at10ffnl9lqhhp996z4gwnclq975m9g6nqu4jd3fcrztngzw8x2psxqcq9yn8"    } }'

```
