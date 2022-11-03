---
title: Substrate Overview
description: Substrate Overview for Web3 engineers
duration: 1 hour
instructors: ["Shawn Tabrizi"]
teaching-assistants: ["Sacha Lansky"]
---

# Block Concepts

---

## Block Concepts

In this section, we will learn how Substrate interprets a block and what it expects from it.

---

## What is a Block in Substrate?

<div class="flex-container">
<div class="left">

A Block in Substrate is just two simple parts:

- The block header
- The block body (extrinsics)

</div>
<div class="right">

<!-- TODO: update diagram of a block, with just header and body. -->

```
┌──────────────────┐
│  Block Header    │
│                  │
│__________________|
│                  │
│   Block Body     │
│                  │
└──────────────────┘
```

</div>
</div>

<br>
<div class="small-text">

These parts are generally opaque in the client, which gives them minimal assumptions, and maximal flexibility.

</div>

---

## The Block Trait Definition

From `/primitives/runtime/src/traits.rs`

```rust [0|8-9|11-12|14-15|17-18]
/// Something which fulfills the abstract idea of a Substrate block. It has types for
/// `Extrinsic` pieces of information as well as a `Header`.
///
/// You can get an iterator over each of the `extrinsics` and retrieve the `header`.
pub trait Block:
	Clone + Send + Sync + Codec + Eq + MaybeSerialize + Debug + MaybeMallocSizeOf + 'static
{
	/// Type for extrinsics.
	type Extrinsic: Member + Codec + Extrinsic + MaybeSerialize + MaybeMallocSizeOf;

	/// Header type.
	type Header: Header<Hash = Self::Hash> + MaybeMallocSizeOf;

	/// Block hash type.
	type Hash: Member + /* lots of other traits */;

	/// Creates new block from header and extrinsics.
	fn new(header: Self::Header, extrinsics: Vec<Self::Extrinsic>) -> Self;

	/// Returns a reference to the header.
	fn header(&self) -> &Self::Header;
	/// Returns a reference to the list of extrinsics.
	fn extrinsics(&self) -> &[Self::Extrinsic];
	/// Split the block into header and list of extrinsics.
	fn deconstruct(self) -> (Self::Header, Vec<Self::Extrinsic>);
	/// Returns the hash of the block.
	fn hash(&self) -> Self::Hash {
		<<Self::Header as Header>::Hashing as Hash>::hash_of(self.header())
	}
	/// Creates an encoded block from the given `header` and `extrinsics` without requiring the
	/// creation of an instance.
	fn encode_from(header: &Self::Header, extrinsics: &[Self::Extrinsic]) -> Vec<u8>;
}
```

---

## The "Generic" Block Struct

From `/primitives/runtime/src/generic/block.rs`

```rust
pub struct Block<Header, Extrinsic: MaybeSerialize> {
	/// The block header.
	pub header: Header,
	/// The accompanying extrinsics.
	pub extrinsics: Vec<Extrinsic>,
}
```

We will look more closely at these concrete types next.

---

## What is the Block Header?

<div class="flex-container">
<div class="left">

- Block number
- Extrinsic root
  - A fingerprint of all the extrinsics in the block
- State root
  - A fingerprint of the current state of the whole chain
- Parent hash
  - A link to the previous block
- ## Digests
  - (light clients, author signature etc etc)

</div>
<div class="right">

<!-- TODO: make diagram - expanded from previous -->

</div>
</div>

---

## The Header Trait Definition

From `/primitives/runtime/src/traits.rs`

```rust
/// Something which fulfills the abstract idea of a Substrate header. It has types for a `Number`,
/// a `Hash` and a `Hashing`. It provides access to an `extrinsics_root`, `state_root` and
/// `parent_hash`, as well as a `digest` and a block `number`.
///
/// You can also create a `new` one from those fields.
pub trait Header:
	Clone + Send + Sync + Codec + Eq + MaybeSerialize + Debug + MaybeMallocSizeOf + 'static
{
	/// Header number.
	type Number: Member + /* lots of other traits */;

	/// Header hash type
	type Hash: Member  + /* lots of other traits */;

	/// Hashing algorithm
	type Hashing: Hash<Output = Self::Hash>;

	/// Creates new header.
	fn new(
		number: Self::Number,
		extrinsics_root: Self::Hash,
		state_root: Self::Hash,
		parent_hash: Self::Hash,
		digest: Digest,
	) -> Self;

	/// Returns a reference to the header number.
	fn number(&self) -> &Self::Number;
	/// Sets the header number.
	fn set_number(&mut self, number: Self::Number);

	/// Returns a reference to the extrinsics root.
	fn extrinsics_root(&self) -> &Self::Hash;
	/// Sets the extrinsic root.
	fn set_extrinsics_root(&mut self, root: Self::Hash);

	/// Returns a reference to the state root.
	fn state_root(&self) -> &Self::Hash;
	/// Sets the state root.
	fn set_state_root(&mut self, root: Self::Hash);

	/// Returns a reference to the parent hash.
	fn parent_hash(&self) -> &Self::Hash;
	/// Sets the parent hash.
	fn set_parent_hash(&mut self, hash: Self::Hash);

	/// Returns a reference to the digest.
	fn digest(&self) -> &Digest;
	/// Get a mutable reference to the digest.
	fn digest_mut(&mut self) -> &mut Digest;

	/// Returns the hash of the header.
	fn hash(&self) -> Self::Hash {
		<Self::Hashing as Hash>::hash_of(self)
	}
}
```

---

## The "Generic" Header Struct

From `/primitives/runtime/src/generic/header.rs`

```rust
pub struct Header<Number: Copy + Into<U256> + TryFrom<U256>, Hash: HashT> {
	/// The parent hash.
	pub parent_hash: Hash::Output,
	/// The block number.
	#[cfg_attr(
		feature = "std",
		serde(serialize_with = "serialize_number", deserialize_with = "deserialize_number")
	)]
	#[codec(compact)]
	pub number: Number,
	/// The state trie merkle root
	pub state_root: Hash::Output,
	/// The merkle root of the extrinsics.
	pub extrinsics_root: Hash::Output,
	/// A chain-specific digest of data useful for light clients or referencing auxiliary data.
	pub digest: Digest,
}
```

---

## What is the Block Body?

<!-- TODO: make diagram - expanded from previous. -->

A block body is simply a collection of opaque data called "extrinsics".

---

## What is an extrinsic?

<div class="text-left">

<h3>extrinsic</h3>

_/ɪkˈstrɪnsɪk,ɛkˈstrɪnsɪk/_

<br>

&nbsp;&nbsp;&nbsp;&nbsp;_adjective_

> not part of the essential nature of someone or something; coming or operating from outside.
> "a complex interplay of extrinsic and intrinsic factors"

</div>

---

## What an Extrinsic really is?

> An Extrinsic is data that come from outside of the runtime.

Yes, transactions are a type of extrinsic, but not all extrinsics are transactions.

<br>
<div class="text-left">

### Extrinsic Types

- Signed Extrinsics
- Unsigned Extrinsics
- Inherent Extrinsics

</div>

---

## Signed Extrinsics

- a.k.a. "transactions"
- signed and submitted by external accounts
- signature verified in the "standard way"

---

## Unsigned Extrinsics

- extrinsics which are NOT signed in the "standard way"
- usually still requires a signature to be safe
- extra layers of programmability is provided to developers
  - `VerifyUnsigned`, `SignedExtensions`, etc...

---

## Inherent Extrinsics

- data that is provided by block authors
- it may not be strictly deterministic
- "soft" verification by others

Notes:

All extrinsics must have some kind of check, else they are not sybil resistant.

---

## Extrinsic Examples

- Signed Extrinsic
  - Balance Transfer
  - Most everything...
- Unsigned Extrinsic
  - Claim DOT Presale Tokens
  - I'm Online
- Inherent Extrinsic
  - Update the Timestamp

---

## Block for Runtime vs Client

- The client sees all extrinsics as opaque (`Vec<u8>`).
- Contrary, the runtime should decode, interpret, and execute the transactions.

<!-- TODO: Diagram to show the flow of transaction from client to runtime, and where it gets executed. -->

---

## Workshops and Activities

- [Block Iteration Workshop](./4.2-Workshops_and_Activities/4.2-Block_Concepts_Workshop.md) (1 hour)