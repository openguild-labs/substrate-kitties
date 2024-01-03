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

### Step 2: Learn about Pallet storage and write basic data structures

#### Reading Materials

I would recommend you to read these materials below first before looking at the code implmentation of the data structures. These materials below cover very well the concepts of FRAME storage in Substrate development.

- [Polkadot Blockchain Academy - FRAME Storage lecture](https://polkadot-blockchain-academy.github.io/pba-book/frame/storage/page.html)
- [Substrate Docs - Runtime storage structure](https://docs.substrate.io/build/runtime-storage/)

#### Data structures to work with Storage API

The FRAME Storage module simplifies access to these layered storage abstractions. You can use the FRAME storage data structures to read or write any value that can be encoded by the SCALE codec. The storage module provides the following types of storage structures:

- [**StorageValue**](https://paritytech.github.io/substrate/master/frame_support/storage/trait.StorageValue.html) to store any single value, such as a u64.
- [**StorageMap**](https://paritytech.github.io/substrate/master/frame_support/storage/trait.StorageMap.html) to store a single key to value mapping, such as a specific account key to a specific balance value.
- [**StorageDoubleMap**](https://paritytech.github.io/substrate/master/frame_support/storage/trait.StorageDoubleMap.html) to store values in a storage map with two keys as an optimization to efficiently remove all entries that have a common first key.
- [**StorageNMap**](https://paritytech.github.io/substrate/master/frame_support/storage/trait.StorageNMap.html) to store values in a map with any arbitrary number of keys.

#### Struct data for Kitty

The blow type alias `BalanceOf` llows easy access our Pallet's `Balance` type. Comes from `Currency` interface.

```rust
type BalanceOf<T> =
 <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
```

Struct for holding kitty information. You may notice a few macros used for the below struct like `Encode`, `Decode`, `TypeInfo`, `MaxEncodedLen`. Let's break down the use of these macros.

- `Encode`, `Decode`: Macros in `parity-scale-codec` which allows the struct to be serialized to and deserialized from binary format with [SCALE](https://github.com/paritytech/parity-scale-codec).
- `MaxEncodedLen`: By default the macro will try to bound the types needed to implement `MaxEncodedLen`, but the bounds can be specified manually with the top level attribute.
- `TypeInfo`: Basically, Rust macros are not that intelligent. In the case of the TypeInfo derive macro, we parse the underlying object, and try to turn it into some JSON expressed type which can be put in the metadata and used by front-ends. (Read more [Substrate Stack Exchange -
  What is the role of `#[scale_info(skip_type_params(T))]`?](https://substrate.stackexchange.com/questions/1423/what-is-the-role-of-scale-infoskip-type-paramst))

```rust
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Copy)]
#[scale_info(skip_type_params(T))]
pub struct Kitty<T: Config> {
 // [2-data-structure]: Implement other attributes for the Kitty struct
 pub dna: T::Hash,
 pub price: Option<BalanceOf<T>>,
 pub gender: Gender,
 pub owner: T::AccountId,
}
```

The Rust macros for automatically deriving MaxEncodedLen naively thinks that T must also be bounded by MaxEncodedLen, even though T itself is not being used in the actual types. ([Read more](https://substrate.stackexchange.com/questions/619/how-to-fix-parity-scale-codecmaxencodedlen-is-not-implemented-for-t/620#620))

Another way to do this without macros like `TypeInfo` and `#[scale_info(skip_type_params(T))]` is to pass in the generic type for `T::AccountId` and `T::Hash` directly instead of pointing them from the genenric `T` type (which does not implement `MaxEncodedLen`).

```rust
// The Gender type used in the `Kitty` struct
#[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum Gender {
 Male,
 Female,
}

impl<T: Config> Kitty<T> {
 pub fn generate_gender(random_hash: T::Hash) -> Gender {
  match random_hash.as_ref()[0] % 2 {
   0 => Gender::Male,
   _ => Gender::Female,
  }
 }

 fn new(dna: T::Hash, owner: T::AccountId) -> Self {
  Kitty { dna, gender: Kitty::<T>::generate_gender(dna), owner, price: None }
 }
}
```

In [4-onchain-randomness](https://github.com/lowlevelers/substrate-kitites/tree/4-onchain-randomness) we will cover the Onchain Randomness topic which is used to generate the DNA for the Kitty. Then this DNA is used to generate the gender for the Kitty as well.

One last thing about type object, you may notice there is `MaxKittiesOwned` type declared in the `Config` trait of the `Pallet`. The purpose of this type is to tell the Runtime which bounded value that can be passed in from the Runtime (Learn more from [Substrate Docs - Configure Runtime Constants](https://docs.substrate.io/reference/how-to-guides/basics/configure-runtime-constants/)).

> Question ⁉️: Why don't we store the `MaxKittiesOwned` with `StorageValue`. Because we want to bound the vector of kitties implemented later (below) with a constant which is not declared upfront. `StorageValue` does not allow us to do it so we need to config a constant on the runtime initialized.

```rust
	#[pallet::config]
	pub trait Config: frame_system::Config {
  /// Other type declarations...

		/// [2-data-structure]: The maximum amount of kitties a single account can own.
		#[pallet::constant]
		type MaxKittiesOwned: Get<u32>;
	}
```

#### Let's implement the storage variables for Substrate Kitties

In the context of Substrate Kitties, we will need data structures that can provide:

1. `Collection of Kitties`: We want a data structure that can helps to get the Kitty complete data by DNA whenever we need. Hence, `StorageMap` is a suitable data structure for this. We don't want to use `StorageValue` with `Vec` because this is expensive when we want to access the Kitty.

> `Twox64Concat` is a hashing technique that is used to hash the keys stored in the `StorageMap`

```rust
/// [2-data-structure]: Maps the kitty struct to the kitty DNA. (hint: using StorageMap)
#[pallet::storage]
#[pallet::getter(fn kitty_collection)]
pub type Kitties<T: Config> = StorageMap<_, Twox64Concat, T::Hash, Kitty<T>>;
```

2. `Relationship between Kitty and its Owner`: This is a `one-to-one` relationship that helps to identify who owns that Kitty. In this case, we need `O(1)` data structure that can help to traverse the relationship between `Owner` and `Kitty` quickly. Hence, we can use `StorageMap`.

```rust
/// [2-data-structure]: Track the kitties owned by each account. (hint: using StorageMap)
#[pallet::storage]
#[pallet::getter(fn owner_of)]
pub(super) type KittyOwner<T: Config> =
 StorageMap<_, Twox64Concat, T::Hash, Option<T::AccountId>, ValueQuery>;
```

3. `Relationship between Owner and their Kitties`: This is a `one-to-many` relationship that helps to identify Kitties owned by an Owner. In this case, we need `O(1)` data structure that can help to traverse the relationship between `Owner` and a list of `Kitty` quickly. Hence, we can use `StorageMap` with `BoundedVec` to store list of Kitty DNAs. Remember that any computation and memory space costs money, so we should use `Bounded` storage structure for memory efficiency.

```rust
/// [2-data-structure]: Keep track of kitties owned by the owner account
#[pallet::storage]
pub(super) type KittiesOwned<T: Config> = StorageMap<
 _,
 Twox64Concat,
 T::AccountId,
 BoundedVec<[u8; 16], T::MaxKittiesOwned>,
 ValueQuery,
>;
```

4. `Total number of Kitties minted`: We want to track the number of Kitties minted through our blockchain.

```rust
/// [2-data-structure]: Keeps track of the number of kitties in existence. (hint: using StorageValue)
#[pallet::storage]
#[pallet::getter(fn all_kitties_count)]
pub(super) type AllKittiesCount<T: Config> = StorageValue<_, u64, ValueQuery>;
```

### Step 3: Learn about dispatchable functions, event and write a method to mint a new kitty

#### Dispatchable functions

- [TheLowLevelers - What is Pallet? (Vietnamese)](https://lowlevelers.com/blog/polkadot/code-breakdown-pallet-template)
- [Substrate Docs - Specify the origin for a call](https://docs.substrate.io/tutorials/build-application-logic/specify-the-origin-for-a-call/)

When users interact with a blockchain they call dispatchable functions to do something. Because those functions are called from the outside of the blockchain interface, in Polkadot's terms any action that involves a dispatchable function is an **Extrinsic**.

```rust
#[pallet::call_index(0)]
#[pallet::weight(T::WeightInfo::dispatchable_function_name())]
pub fn dispatchable_function_name(origin: OriginFor<T>) -> DispatchResult
```

A function signature of a dispatchable function declared in the Pallet code must return a `DispatchResult` and accept a first parameter is an origin typed `OriginFor<T>`.

#### Events & Errors

Events and errors are used to notify about specific activity. Please use this for debugging purpose only. Events and Errors should not be used as a communication method between functionalities.

In our codebase, we will declare these errors and events. The syntax is basically Rust code but with macro `#[pallet::error]`

```rust
// Errors inform users that something went wrong.
#[pallet::error]
pub enum Error<T> {
 /// An account may only own `MaxKittiesOwned` kitties.
 TooManyOwned,
 /// This kitty already exists!
 DuplicateKitty,
 /// An overflow has occurred!
 Overflow,
 /// This kitty does not exist!
 NoKitty,
 /// You are not the owner of this kitty.
 NotOwner,
 /// Trying to transfer or buy a kitty from oneself.
 TransferToSelf,
 /// Ensures that the buying price is greater than the asking price.
 BidPriceTooLow,
 /// This kitty is not for sale.
 NotForSale,
}
```

And comment out the `Created` event so that we can deposit an event on new kitty minted.

```rust
#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
 // A new kitty was successfully created.
 Created { kitty: T::Hash, owner: T::AccountId },
}
```

To dispatch an event, we do

```rust
// deposit a new event when the kitty is created
Self::deposit_event(Event::Created { kitty: kitty_dna, owner: sender });
```

#### Write a method to mint a new kitty

```rust
/// Create a new unique kitty.
#[pallet::call_index(0)]
#[pallet::weight(T::WeightInfo::create_kitty())]
pub fn create_kitty(origin: OriginFor<T>) -> DispatchResult {
 // Ensure that sender did sign this extrinsic call
 let sender = ensure_signed(origin)?;

 // Generate a randome DNA (this will be guided in 4-onchain-randomness)
 let kitty_dna = Pallet::<T>::gen_dna(&sender);
 ensure!(!<Kitties<T>>::contains_key(kitty_dna), Error::<T>::DuplicateKitty);

 // 1. map the new DNA with the struct data of Kitty
 // ERROR: We throw an error if there exists a Kitty already
 <Kitties<T>>::insert(kitty_dna, Kitty::<T>::new(kitty_dna, sender.clone()));

 // 2. map the new DNA with its new owner
 // ERROR: We throw an error if there exists a Kitty already
 ensure!(!<KittyOwner<T>>::contains_key(kitty_dna), Error::<T>::DuplicateKitty);
 <KittyOwner<T>>::insert(kitty_dna, Some(&sender));

 // 3. update the total count of kitties
 let new_all_kitties_count =
  Self::all_kitties_count().checked_add(1).ok_or(Error::<T>::Overflow).unwrap();
 <AllKittiesCount<T>>::put(new_all_kitties_count);

 // 4. push the new kitty DNA to the list of existing kitties owned by a sender
 KittiesOwned::<T>::try_append(&sender, kitty_dna)
  // ERROR: We throw an error if there are too many Kitties owned by the sender
  .map_err(|_| Error::<T>::TooManyOwned)?;

 // EVENT: Deposit a new event when the kitty is created
 Self::deposit_event(Event::Created { kitty: kitty_dna, owner: sender });

 Ok(())
}
```

### Step 4: Learn about onchain randomness and how to generate a random DNA for the Kitty

- [Substrate Docs - Randomness](https://docs.substrate.io/build/randomness/)
- [Substrate How-to Guides - Incorporate Randomness](https://docs.substrate.io/reference/how-to-guides/pallet-design/incorporate-randomness/)
- [Substrate Stack Exchange - Onchain Pseudo Random Numbers Agreed by All](https://substrate.stackexchange.com/questions/346/on-chain-pseudo-random-numbers-agreed-by-all)

Onchain randomness is quite important for a Turing complete applications. There are many use cases involved the randomness like gambling, probabilistic computation or random factors in gaming. But in blockchain, state in the state machine must be deterministic so we don't have a real randomness but `pseudo randomness`.

To add randomness feature to our Kitty DNA generation logic, we will use `pallet_insecure_randomness_collective_flip` pallet. This Pallet is not supposed to be used on `production` so please be aware of it.

#### Logic behinds [`pallet_insecure_randomnes_collective_flip`](https://docs.rs/pallet-randomness-collective-flip/latest/src/pallet_randomness_collective_flip/lib.rs.html#18-308)

The logic behinds this Pallet crate is simple. It has an offchain worker running to store a block hash to the onchain storage when block is initialized.

```rust
#[pallet::hooks]
impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
  fn on_initialize(block_number: T::BlockNumber) -> Weight {
    // take the parent hash of the block (block hash of the previous block)
    let parent_hash = <frame_system::Pallet<T>>::parent_hash();

    // store into the onchain storage
    <RandomMaterial<T>>::mutate(|ref mut values| {
      if values.try_push(parent_hash).is_err() {
        let index = block_number_to_index::<T>(block_number);
        values[index] = parent_hash;
      }
    });

    T::DbWeight::get().reads_writes(1, 1)
  }
}
```

After 81 block hashes, it takes all thoses and generate a random value based on it. In the Pallet source code, you can see it defines this constant value for the number of random materials.

```rust
const RANDOM_MATERIAL_LEN: u32 = 81;
```

Take a look at the seed generation code

```rust
let hash_series = <RandomMaterial<T>>::get();
let seed = if !hash_series.is_empty() {
  // Always the case after block 1 is initialized.
  hash_series
    .iter()
    .cycle()
    .skip(index)
    .take(RANDOM_MATERIAL_LEN as usize)
    .enumerate()
    .map(|(i, h)| (i as i8, subject, h).using_encoded(T::Hashing::hash))
    .triplet_mix()
} else {
  T::Hash::default()
};
```

As you see, it is quite straightforward and simple to understand. This randomness can be predicted in advanced but still random enough to not be predicted.

#### Generate random DNA for the Kitty

Ok so now how can we implement this randomness feature for our Kitty DNA generation code?

We will add the below `KittyRandomness` type metadata into the `Config` trait of the pallet. Hence, on the `Runtime` side, we can add a Pallet that matches the type.

```rust
/// [4-onchain-randomness]: The type of Randomness we want to specify for this pallet.
type KittyRandomness: Randomness<Self::Hash, BlockNumberFor<Self>>;
```

On `Runtime` side, we add the `pallet_insecure_randomness_collective_flip` crate to the dependency list.

```rust
pallet-insecure-randomness-collective-flip = { git = "https://github.com/paritytech/substrate", package = "pallet-insecure-randomness-collective-flip", default-features = false, branch = "polkadot-v0.9.42" }
```

With this, we can define the Pallet on the `Runtime` side. Inside `construct_runtime!` macro, we add `RandomnessCollectiveFlip`.

```rust
construct_runtime!(
	pub struct Runtime
	where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
        /** Other pallet declarations **/
		RandomnessCollectiveFlip: pallet_insecure_randomness_collective_flip, // Newly added randomness pallet
		Kitties: pallet_substratekitties, // Our Kitties pallet
	}
);

/** Other code... */
```

We need to make changes to the Pallet Kitties config code. It simply plugs and play.

```diff
+ impl pallet_insecure_randomness_collective_flip::Config for Runtime {}

impl pallet_substratekitties::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_substratekitties::weights::SubstrateWeight<Runtime>;
	type Currency = Balances;
	type MaxKittiesOwned = frame_support::pallet_prelude::ConstU32<100>;
+   type KittyRandomness = RandomnessCollectiveFlip;
}
```

To check if Runtime code is functional, please run `cargo check`.

> Actually, I already add all these Randomness stuff in the `1-setup` but feel free to reimplement the feature yourself so you can fully grasp the idea of this tutorial

Now we are ready to generate DNA for our Kitty

```rust
// [4-onchain-randomness] Generates and returns DNA and Gender
fn gen_dna(minter: &T::AccountId) -> T::Hash {
  let (output, block_number) = T::KittyRandomness::random(&b"dna"[..]);
  // Experiment: You can experiment your self to have only block_number and minter as a parameters of the hashing function. This can be generated if you mint a new Kitty in different blocks. Otherwise, it will be double spending
  let payload = (output, block_number, minter);
  T::Hashing::hash_of(&payload)
}
```

### Step 5: Interact with the Substrate Node from the frontend.

That's enough for the Substrate part, now we have an API for `create_kitty` ready, let's implement a frontend to interact with the logic defined in our Pallet code. Before starting the frontend, please follow these steps:

> Frontend code implementation is complete already. Please follow the blog or clone `substrate-frontend-template` to work from scratch

```
cd frontend
npm run install
```

There are many abstractions on the frontend side as it uses multiple open-source libraries like `@polkadot/api`. I will keep it in a decent level of abstraction so you can still understand how does it work.

> You are supposed to have a decent knowledge of React and Javascript to grasp the idea of this tutorial step. Because we mainly use RPC methods from `@polkadot/api`, having a prior knowledge of frontend development is a must.

Let's break down API call to the Substrate Node. On the frontend code, you can log out the methods by doing

```js
// Kitties.js
console.log(api.query.kitties);
```

Simply speaking, this API call abstract a way RPC call to the storage getter.

```js
// Example of the code to fetch all Kitties
const asyncFetch = async () => {
  unsub = await api.query.kitties.allKittiesCount(async (count) => {
    // Fetch all kitty keys
    const entries = await api.query.kitties.kitties.entries();
    const kittiesMap = entries.map((entry) => {
      return {
        id: toHexString(entry[0].slice(-32)),
        ...parseKitty(entry[1].unwrap()),
      };
    });
    setKitties(kittiesMap);
  });
};
```

How about dispatching an extrinsic call to the Subtrate node backend? Please a full view of the `TxButton.js` file to get the overall idea.

```js
api.tx[palletRpc][callable];
```

This is what you actually see in that code file, in `Kitties.js`

```js
<TxButton
  label="Create Kitty"
  type="SIGNED-TX"
  setStatus={setStatus}
  attrs={{
    palletRpc: "kitties",
    callable: "createKitty",
    inputParams: [],
    paramFields: [],
  }}
/>
```

What it actually looks like is

```js
api.tx.kitties.createKitty(...params);
```

Reflect this with what we did on the Substrate backend

```rust
// runtime/lib.rs
construct_runtime!(
	pub struct Runtime
	where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
    // ...other pallet declaration
		Kitties: pallet_substratekitties,
	}
);

// substratekitties/lib.rs
#[pallet::call_index(0)]
#[pallet::weight(T::WeightInfo::create_kitty())]
pub fn create_kitty(origin: OriginFor<T>) -> DispatchResult
```

So it would be `Kitties.create_kitty` which is converted to the frontend code after deserializing is `api.tx.kitties.createKitty`.

That's it, it is quite simple about how to interact with your Pallet code right? Polkadot team definitely has many good engineers. There are a few other things like "How to get account in the network and its balance?"

```javascript
function BalanceAnnotation(props) {
  const { api, currentAccount } = useSubstrateState();
  const [accountBalance, setAccountBalance] = useState(0);

  // When account address changes, update subscriptions
  useEffect(() => {
    let unsubscribe;

    // If the user has selected an address, create a new subscription
    currentAccount &&
      api.query.system
        .account(acctAddr(currentAccount), (balance) =>
          setAccountBalance(balance.data.free.toHuman())
        )
        .then((unsub) => (unsubscribe = unsub))
        .catch(console.error);

    return () => unsubscribe && unsubscribe();
  }, [api, currentAccount]);

  return currentAccount ? (
    <Label pointing="left">
      <Icon name="money" color="green" />
      {accountBalance}
    </Label>
  ) : null;
}
```

It simply get the data from the `System` pallet about the accounts on the existing blockchain and their balances.

I guess that's enough for the overall idea of the frontend code. After retrieving the data from the Substrate node, the part of visualizing it is your thing.

Also, I might miss this. If you are interested into how the unique Kitty image is generated, please take a look at `KittyAvatar.js`.

```javascript
const dnaToAttributes = (dna) => {
  const attribute = (index, type) =>
    IMAGES[type][dna[index] % IMAGES[type].length];

  return {
    body: attribute(0, "body"),
    eyes: attribute(1, "eyes"),
    accessory: attribute(2, "accessory"),
    fur: attribute(3, "fur"),
    mouth: attribute(4, "mouth"),
  };
};
```

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
