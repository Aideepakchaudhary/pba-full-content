# Lesson 2: Different Types of Consensus Systems

## Why is this topic Important?

Runtime developers should understand the building blocks that enable blockchain's service guarantees. Consensus systems form one of those blocks. Students might have some economic considerations in their runtimes as well.

## Core Ideas to Convey

- What is consensus?
  - We need a multi-agent system to agree on something
  - We don't trust any of these agents
  - Some of them could be faulty (not respond, respond with jibberish)
  - Some of them could be malicious (attempting to deceive, coordinating with others)
- Desired properties:
  - Termination
  - Integrity
  - Agreement
- Byzantine Generals Problem

- Synchronous vs. Async Processes

  - Obviously the real world is async, but easier to model synchronous systems
  - Synchronous systems typically progress in "rounds"

- Permissioned vs. Permissionless participation
- Probabilistic vs. Deterministic Finality
- Authority set size
  - Permissionless/unlimited: Anyone can participate at any time (e.g. PoW)
  - Permissionless/limited: Anyone can participate, but must be "chosen" to join (e.g. NPoS)
  - Permissioned/limited: Somebody decides who joins (e.g. distributed flight computer)
- Very simple example:

  - Flight computer on an airplane
  - Obviously a closed system
  - There is a leader
  - Some backups decide if the leader is faulty and when to replace it with a new leader.

- Start focusing on blockchain: Desired properties for a consensus algorithm
  - Need to select an authority set (who gets to participate at all)
  - Need to build a chain (who authors blocks)
  - Need to agree on a chain (which chain is final)
- Keeping participants in line without a central authority
  - Why would anyone _want_ to participate?
  - How do we define misbehavior?
  - What are the consequences for misbehavior?
- Concrete example:
  - Let's say a slash is 100 DOT, and the reporter gets 10%.
  - I ask another authority to cooperate with me: "I'll pay you 20 DOT to _not_ rat me out for my attack".
  - The other authority should respond: "But I don't believe you can carry out the attack because _someone else_ will report you and take the 10 DOT, leaving me with 0."
- Now we can dive into some blockchain specifics

- Nakamoto Consensus
  - A.k.a. Proof of Work
  - Anyone can participate, but must invest in special hardware to compute hashes
  - Security based on
    - large amount of overall hash power in the network (inability for a coordinated group to control a large share)
    - large amount of hash power relative to other networks (inability for their miners to attack you)
  - Blocks added to the chain by computing hashes below some difficulty target
  - Network agrees on chains based on most amount of work (not strictly correct but generall the longest chain)
  - Only provides probabilistic finality because the authority set size is unknown (there could be hash power that is mining a chain that is not being broadcast)
- Finite Participant Systems
  - Elect some group of authorities for some amount of time (at which point there will be another election)
  - Allows deterministic finality
    - View: Some authority might see one block as final while another one does not, but two authorities should never see conflicting blocks as final
  - But comes with different assumptions (`n = 3f + 1` instead of 51%)
  - Requires elected authorities to be online
- Proof of Stake (Tokens)
  - Election based on some mapping of staking in the system (namely, usually, number of tokens)
  - Can be own tokens, delegated, nominated
  - Typically uses slashing as a mechanism to dissuade misbehavior
- Consensus with a finite set
  - Simple round-robin voting
    - e.g. Tendermint
    - Couples block production with finality
    - Take turns being a leader proposing a block, everyone votes on it, the next round doesn't start until it is accepted or rejected (rejection could be via round timeout)
  - Slot assignment
    - Ouroborous, BABE, Sassafras
    - Why is this an improvement over round robin? How is round robin attackable?
    - Time is broken into slots, and in each slot someone may propose a block, but the upcoming authors should not be predictable to an observer
    - Possible to use with finality mechanisms like longest chain rule, or a deterministic gadget
- GRANDPA
  - Chains not blocks
  - Allows more consistent block production rate - no need to wait on voting rounds
  - Allows for a challenge period to delay finality if needed
  - Applies typical BFT assumptions to historical blocks based on tip votes
- Slashing
  - Typically used in Proof of Stake systems
  - Offlineness
  - Equivocation
  - Invalid state transition
- Slashing motivations
  - Nodes in the network should have an incentive to report on misbahaving nodes
  - Usually handled by giving them a percentage of any punishment
- Elections
  - Consensus vs. governance decisions
  - Don't conflate, isolate
- Scaling
  - What are the bottlenecks?
  - Sharding
  - Availability and Validity
- Next up:
  - Fees and ordering: How to go about constructing these blocks
  - Attacks: How do these systems get attacked

## Learning Outcome

By the end of this lesson, students should understand a variety of consensus mechanisms as applied to blockchains and how separate consensus systems may be related.

## Learning Objectives

- Describe a consensus system in terms of participation, liveness, and safety

## Prerequisite Knowledge or Skills

- Blockchain architecture
- Hash based data structures
- Economics

## Manual Consensus Activity

Students will perform consensus tasks by hand to gain a better understanding of how they work.

Tasks performed manually include:

- Authoring blocks
- Checking blocks
- Choosing a fork to author on
- Determining finality
- Mining proof of work
- Signing blocks

Read the [full writeup](../3.2-Consensus_Systems/3.2-Manual_Consensus_Activity.md).