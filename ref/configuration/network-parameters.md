# Xerberus Network Parameters

## Table of Contents

- [Introduction](#introduction)
- [Validator Keys](#validator-keys)
  - [Aura Keys (Block Production)](#aura-keys-block-production)
  - [Grandpa Keys (Block Finalization)](#grandpa-keys-block-finalization)
- [Chain Specification Configuration](#chain-specification-configuration)
- [Usage Guidelines](#usage-guidelines)

## Introduction

This document contains the cryptographic keys and network parameters for configuring the Xerberus blockchain network. These parameters are primarily used for local testnet setup and testing.

**IMPORTANT**: In a production environment, never share or expose these keys!

## Validator Keys

### Aura Keys (Block Production)

Aura keys use the Sr25519 cryptographic scheme and are used for block production.

#### Validator 1 - Aura

```
Secret phrase:     ginger decrease bargain member learn business patch royal jacket company swamp tide
Network ID:        substrate
Secret seed:       0x6e03e61932a598054289f9ba7ede26069c097251deff71b5a41b49d23ac096c1
Public key (hex):  0x72a4fde8aeeab2fd90d577b588418eb0e175ef3f7984c10e11f887a2a158f07c
Account ID:        0x72a4fde8aeeab2fd90d577b588418eb0e175ef3f7984c10e11f887a2a158f07c
Public key (SS58): 5Ef2L2S2St68hZLkVx61HN3c75eD5Jdt23mSyZaDDygub9P8
SS58 Address:      5Ef2L2S2St68hZLkVx61HN3c75eD5Jdt23mSyZaDDygub9P8
```

#### Validator 2 - Aura

```
Secret phrase:     picnic dawn day sausage grunt flash endorse child under chaos fuel scorpion
Network ID:        substrate
Secret seed:       0xb50e989e5fa2e1a6d802a979a31d74c77dcfd8a364e7855379254d6030234f32
Public key (hex):  0x96411bf7a592e11b8b86059fa7b2dc683d07b2217c650481198221bfc6ca3152
Account ID:        0x96411bf7a592e11b8b86059fa7b2dc683d07b2217c650481198221bfc6ca3152
Public key (SS58): 5FTiP588D9TfkuLUeUMyBM5EQzVJdz9wyCD3reCoKRWkMcd3
SS58 Address:      5FTiP588D9TfkuLUeUMyBM5EQzVJdz9wyCD3reCoKRWkMcd3
```

#### Validator 3 - Aura

```
Secret phrase:     monster brother winter uncle produce verify wet stomach strategy design route rotate
Network ID:        substrate
Secret seed:       0x9363bd9bf76ddaf97b4759a46466ed8f53175c4df441d2a97878dae9f8b6b7d7
Public key (hex):  0x045885cbad88ff2539e9371833ec3ca2bd114bfdfdd34c869798e32340c65727
Account ID:        0x045885cbad88ff2539e9371833ec3ca2bd114bfdfdd34c869798e32340c65727
Public key (SS58): 5CAQLzERqAs2LCfs7CAioSSpCQ1QWjFqrGMLt6YBBPMeAeV9
SS58 Address:      5CAQLzERqAs2LCfs7CAioSSpCQ1QWjFqrGMLt6YBBPMeAeV9
```

### Grandpa Keys (Block Finalization)

Grandpa keys use the Ed25519 cryptographic scheme and are used for block finalization.

#### Validator 1 - Grandpa

```
Secret phrase:     park aerobic era zero sniff birth seven scorpion kite night axis name
Network ID:        substrate
Secret seed:       0x0982116baf14c51457170b83156745d91f4de17d80348baf154f8b18b24d7f79
Public key (hex):  0x4e558158a2afd092c7f655c2d63d210a36902ae76f7b124a44720f28063521c5
Account ID:        0x4e558158a2afd092c7f655c2d63d210a36902ae76f7b124a44720f28063521c5
Public key (SS58): 5DqQzX96dG7ycueJxapQKXbJjhHneWF7imkykbMUc3EDQyEe
SS58 Address:      5DqQzX96dG7ycueJxapQKXbJjhHneWF7imkykbMUc3EDQyEe
```

#### Validator 2 - Grandpa

```
Secret phrase:     basic inch decrease loyal always repair walnut minute carbon fashion addict bridge
Network ID:        substrate
Secret seed:       0x84cd27c47fa2bba5cd8841492f5f33c3e8f3217fc15e0be3b74042c475e789da
Public key (hex):  0x7935ec050ad8b02d103bb0e7c30a60da0d93aa14baf119255f8c95a8310cb71f
Account ID:        0x7935ec050ad8b02d103bb0e7c30a60da0d93aa14baf119255f8c95a8310cb71f
Public key (SS58): 5Eodfn2ntEUCttJgKQfBrLcvbXBYx9eiVmiLtevV4zterNar
SS58 Address:      5Eodfn2ntEUCttJgKQfBrLcvbXBYx9eiVmiLtevV4zterNar
```

#### Validator 3 - Grandpa

```
Secret phrase:     luggage entry example old vivid addict swap skill electric chest drastic blouse
Network ID:        substrate
Secret seed:       0x8fa47c31f0e19bee27c3180f1744611debe8cfeb1d0c7885db0034b14860f627
Public key (hex):  0x4b9667a0603960b40d17590c29ecc28bbc46614ba1246db45d2bd3b8b0852f5f
Account ID:        0x4b9667a0603960b40d17590c29ecc28bbc46614ba1246db45d2bd3b8b0852f5f
Public key (SS58): 5Dmp8P5PqRLsqVUZDF1HckPfRgwrcNeidtBZ3EEcjBHw3sfT
SS58 Address:      5Dmp8P5PqRLsqVUZDF1HckPfRgwrcNeidtBZ3EEcjBHw3sfT
```

## Chain Specification Configuration

When creating your chain specification (`chain-spec.json`), use the following formats:

### Aura Authorities Configuration

```json
"aura": {
  "authorities": [
    "5Ef2L2S2St68hZLkVx61HN3c75eD5Jdt23mSyZaDDygub9P8",
    "5FTiP588D9TfkuLUeUMyBM5EQzVJdz9wyCD3reCoKRWkMcd3",
    "5CAQLzERqAs2LCfs7CAioSSpCQ1QWjFqrGMLt6YBBPMeAeV9"
  ]
}
```

### Grandpa Authorities Configuration

```json
"grandpa": {
  "authorities": [
    ["5DqQzX96dG7ycueJxapQKXbJjhHneWF7imkykbMUc3EDQyEe", 1],
    ["5Eodfn2ntEUCttJgKQfBrLcvbXBYx9eiVmiLtevV4zterNar", 1],
    ["5Dmp8P5PqRLsqVUZDF1HckPfRgwrcNeidtBZ3EEcjBHw3sfT", 1]
  ]
}
```

### Balances Configuration

For initial token distribution (optional):

```json
"balances": {
  "balances": [
    ["5Ef2L2S2St68hZLkVx61HN3c75eD5Jdt23mSyZaDDygub9P8", 1152921504606846976],
    ["5FTiP588D9TfkuLUeUMyBM5EQzVJdz9wyCD3reCoKRWkMcd3", 1152921504606846976],
    ["5CAQLzERqAs2LCfs7CAioSSpCQ1QWjFqrGMLt6YBBPMeAeV9", 1152921504606846976]
  ]
}
```

## Usage Guidelines

1. Insert the keys into your validators using the RPC calls documented in the [Node Setup Guide](../node/node-setup-guide.md)
2. Add the SS58 addresses to your chain spec file before generating the raw spec
3. Keep the secret phrases secure - they're needed to restore keys if needed
4. The keys are referred to in `insertKey` commands by their secret phrases or seeds

---

## Related Resources

- [Node Setup Guide](../node/node-setup-guide.md)
- [Testnet Configuration](../node/testnet-configuration.md) 