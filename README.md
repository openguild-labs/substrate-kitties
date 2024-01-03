# Substrate Kitties by TheLowLevelers

Original version from Substrate Developer Hub is here, please give it a few credits to the team behinds it: https://www.shawntabrizi.com/substrate-collectables-workshop/#/

> The interactive hands-on build-your-first-blockchain with [Substrate][] workshop

## 🖐️ What is this version made by TheLowLevelers?

The original version is outdated. `substrate-node-template` no longer has a concept of runtime modules but `pallet`. Hence, it is not likable to us the outdated material to learn about Substrate.

## What is this?

This is an interactive hands-on self-paced workshop. You will learn how to build your first blockchain using [Substrate][], the OpenSource [Rust][] Blockchain Development Kit by [Parity][]. Through the lessons of the workshop, you will build a collectables blockchain -- a chain that creates assets, and allows you to interact with and managing ownership of them.

As such, this material will focus on building the logic of this chain. It won't cover the networking, consensus or economic incentive aspects of blockchains. Fortunately, Substrate comes with decent networking and consensus engines built in, so we can just focus on the chain logic.

Substrate is built using [Rust][], a modern statically typed systems programming language. We won't go into the details of the language within this workshop. The language is quite easy to read and follow and if you have programmed before, you shouldn't have too much trouble following what is going on and finishing the exercises even if [Rust][] is new to you.

## Tutorial Steps

### Business logic

We are going to build a simple NFT marketplace for `Substrate Kitties` (Please take a look at CryptoZombies or CryptoKitties on Ethereum blockchain to get an idea of what is Substrate Kitties) that allows users to:

- `mint`: Mint a new NFT item (we call it a Kitty)
- `transfer`: Transfer a new NFT item from the sender to a destination account.
- `list_nft`: List the NFT on the marketplace so other users can buy
- `buy_nft`: User can buy NFT on the marketplace from other users if the NFT is listed

### Prerequisites

This requires you to finish a first few tutorials of Substrate development from the official documentation. If you have not walked through those first. Please take a look at these first before diving deeper into this interactive tutorial:

- [TheLowLevelers - Run a local Substrate Node (Vietnamese)](https://lowlevelers.com/blog/polkadot/polkadot-guide-chay-local-substrate-node)
- [Substrate Tutorial - Build a local blockchain](https://docs.substrate.io/tutorials/build-a-blockchain/build-local-blockchain/)
- [Substrate Tutorial - Pallet](https://docs.substrate.io/tutorials/build-application-logic/)

### Step 0: Setup your local environment

If your hardware is a modern M1 Apple sillicon chip, working with Substrate can be very painful because there is many unstable compilation issue happens during your development. To avoid this, please install Rust toolchain following these versions below.

```
❯ cargo --version
cargo 1.76.0-nightly (71cd3a926 2023-11-20)
❯ rustc --version
rustc 1.76.0-nightly (3a85a5cfe 2023-11-20)
❯ rustup --version
rustup 1.25.2 (17db695f1 2023-02-01)
```

### Step 1: Clone repository + Setup code template on your local

There are multiple version for this awesome Substrate Kitties tutorial. However, based on my experience, those are outdated and it takes you a lot of time to set up a right dependecy version to work on.

So please checkout `1-setup` to get well-tested code template for this tutorial.

```shell
git clone https://github.com/lowlevelers/substrate-kitites.git
git checkout 1-setup
```

After checking the branch, please run below command to test if you can run a node from your local environment.

```
cd substrate-kitties
cargo build --release
```

Let's break down the given template code and what you need to work on:

> I will suppose that you already understand the structure of a Pallet code and how Pallet interacts with the Substrate Runtime

The full flow for Substrate development will be `Pallet > Runtime > Frontend`

| Step | Modules                                                                                            | Description                                                                                     |
| ---- | -------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- |
| #0   | Prerequisites                                                                                      | Prepare your local environment to work with Substrate node and the code template                |
| #1   | [1-setup](https://github.com/lowlevelers/substrate-kitites/tree/1-setup)                           | Clone `substrate-kitties` and checkout branch `1-setup` to setup the template code on the local |
| #2   | [2-data-structure](https://github.com/lowlevelers/substrate-kitites/tree/2-data-structure)         | Learn about Pallet storage and write basic data structures for Substrate Kitties                |
| #3   | [3-mint-kitty](https://github.com/lowlevelers/substrate-kitites/tree/3-mint-kitty)                 | Learn about dispatchable functions, event and write a method to mint a new kitty                |
| #4   | [4-onchain-randomness](https://github.com/lowlevelers/substrate-kitites/tree/4-onchain-randomness) | Learn about onchain randomness and how to generate a random DNA for the Kitty                   |
| #5   | [5-call-from-frontend](https://github.com/lowlevelers/substrate-kitites/tree/5-call-from-frontend) | Interact with the Substrate Node from the frontend.                                             |
| #6   | [6-full-code](https://github.com/lowlevelers/substrate-kitites/tree/6-full-code)                   | Implement a full code for Substrate Kitties project                                             |

---

## How to contribute

Before committing to the tasks in the community, please skim through the guidelines below to grasp the overall idea of how the community works first. It does not take long but I believe it will give you a big picture of the vision and culture of TheLowLevelers.

- [TheLowLevelers Contribution Guidelines 🤝](https://github.com/orgs/lowlevelers/discussions/8)
- [TheLowLevelers Community Guidelines 🔥](https://github.com/orgs/lowlevelers/discussions/3)
- [FAQ Who own the community assets?](https://github.com/orgs/lowlevelers/discussions/9)

## Acknowledgements

Open source projects like Substrate and this workshop could not be successful without the collective minds and collaborative effort of the development community.

The Substratekitties workshop stands on the backs of giants like [Cryptokitties](https://www.cryptokitties.co/), [Cryptozombies](https://cryptozombies.io/), [Docsify](https://docsify.js.org/), [Monaco Editor](https://microsoft.github.io/monaco-editor/), [David Revoy's Cat Avatar Generator](https://framagit.org/Deevad/cat-avatar-generator), and numerous volunteers to report errors and bugs along the way.

We hope this educational material teaches you something new, and in turn, you teach others too.

---

[main link]: https://substrate-developer-hub.github.io/substrate-collectables-workshop/
[feedback]: https://substrate.dev/community/
[Substrate]: https://www.parity.io/substrate/
[Substrate docs]: https://substrate.dev/docs/
[Parity]: https://www.parity.io/
[Rust]: https://www.rust-lang.org/
