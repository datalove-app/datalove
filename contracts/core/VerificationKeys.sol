// SPDX-License-Identifier: MIT
pragma solidity ^0.7.0;

/// @title hard-coded verification keys
library VerificationKeys {
	function getKey(
		// uint blockType,
		// uint blockSize,
		// uint blockVersion
	) internal pure returns (
		uint256[14] memory vk,
		uint256[4] memory vk_gammaABC
		// bool found
	) {
		vk = [
			// alpha x, y
			0x0ac7697e0ed0dcd6af9d4c14589d977aa5119a1d7bbbf7041f36a4af43fce7b0,
			0x14762c22a35b88ee702e7755f181daea5efaa05fc594aa1909ca2d854f252219,

			// beta x.c1, x.c0, y.c1, y.c2
			0x0a0bbe7ad108d991b398ee06e6ac8cc54b753751f339693c5d185f80c9ad5d9f,
			0x1621a7554c35ce2e5d2a114ecf4c25ab166d4dadaf7544717e855f077cf4f13b,
			0x28ca93e9ee9628bc0f35e8ecc04db6d8c892517d0390adc927321e98a587c8e9,
			0x0c859e6ad3ea23a8484008089d978baff5b7b8f877eebb212708935326e6a1c1,

			// gamma
			0x1de03944e398132d92a6180b2aea2d315b085f99d284b95cf1d96fb3f934436b,
			0x097108d8f151c22e94a7b0f188a5bf85268a660b1aeec9eebf5f6a7cee09264c,
			0x22ab4b983de5b4cbd147548fd76cd99c1c886ede8f913979f4da6b6ca10b3643,
			0x182e64a34768d738bbdd1f8c517ac47e713dfb54cc04280a53be8074f98eed0a,

			// delta
			0x1bdce4526039ab388ef02e195f14f017e13cee115f1a932cfa3c4a301e322ea9,
			0x05af5c33457f0bc0d33a072fae49204daf17083935fc2816d61409871966d0c6,
			0x1b952fb1dc0d11beb42e363c327a2c6c1f903f81b46e389c918614d96407145c,
			0x09f68ff104abb661b8a1d81289cbe184bc8d051ffc3e8a1827df396dfc99359d
		];

		vk_gammaABC = [
			0x0dc4ef8e01f60babe16d098ee5887e4a5c994fd1a29bc4163352d39126f31a2c,
			0x20ffa91b8f18baaaa013212d2efb69565dd6929ccc971222d00e9a4e3f5a4508,
			0x146eda8bb8712d485cea2cac75088b3b1f4a9df80d5ee9c13c7cdaeda99afe99,
			0x2699f5e6218133c2e7c163f2c58639836f789222cb33a58c2e32387a07517228
		];
		// found = false;
	}
}
