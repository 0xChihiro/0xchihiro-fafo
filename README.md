# 0xChihiro FAFO Discovery

The purpose of this repositroy was for me to familiarize myself with [Layer Zero FAFO](https://github.com/LayerZero-Labs/fafo). It contains the vast majority of the code exactly as is written inside of the original repository,
with only minor changes.Changes for example like using rkyv instead of bincode or other small things, as changing certain masking operations just to mess around with the code. All of the original tests have stayed exactly
the same andbefore any code is pushed up they pass as they were originally written unless stated otherwise in the tests themselves. Micro benchmarks have been added for specific things and I plan to mess around with those
as well justto see if any optimizations are easily implemented without changing to much of the underlying logic. I will also be adding pocket guides for each crate and section that is meant to help developers who want
to quickly getfoundational knowledge about the crate so that they can begin building things with it.

### Pocket Guides

- Pocket guides are documentation that is meant to help developers and users quickly understand the make up of the crates and what each section does and its role in the large system.
The guides will be updated as I continue going through the codebase and gain better understanding of the whole project. In the end it will be difficult to tell where to start if you
are attempting to gain a understanding of the entire project so a 'compass' which will be provided, so that you can gain an understanding of which pieces rely on what other pieces and
can quickly build up knowledge without needing to jump through the repository to much. 

- [ ] Codedb
  - [ ] Cache
  - [ ] Def
  - [ ] Codedb
  - [ ] Uring
- [ ] Exepipe-Common
  - [ ] Executor
  - [ ] Account Data
  - [ ] Access Set
  - [ ] Def
  - [ ] Exepipe-Common
  - [ ] REVM
  - [ ] Slot Entry Value
  - [ ] State Cache
  - [ ] Utils 
- [ ] Exepipe
  - [ ] Context
  - [ ] Dispatcher
  - [ ] Simulator
  - [ ] Utils
  - [ ] Def
  - [ ] Exetask
  - [ ] Framer
  - [ ] Exepipe
  - [ ] Mempool 
- [ ] HPFile
  - [ ] File
  - [ ] HPFile
- [ ] Qmdb
  - [ ] Entry file
  - [ ] Flusher
  - [ ] Indexer
  - [ ] Merkletree
  - [ ] Mutator
  - [ ] Prefetcher
  - [ ] Tasks
  - [ ] Utils
  - [ ] Compactor
  - [ ] Config
  - [ ] Def
  - [ ] Qmdb
  - [ ] Metadb
  - [ ] Updater
