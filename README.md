# SMILES Suite

A SMILES validation suite for [ChemCore](https::crates.io/chemcore). It's based on [this presentation](https://www.slideshare.net/NextMoveSoftware/a-de-facto-standard-or-a-freeforall) and [the smilesreading repository](https://github.com/nextmovesoftware/smilesreading.git).

# Installation

```bash
git clone ...
cd ...
git submodule init
```

# Run

```bash
cargo run
```

# Diffing

For example, to compare results for Open Babel and ChemCore, use the following:

```bash
diff results/openbabel_dev4Aug17.txt  <(gzip -dc smilesreading/3-results/chembl/openbabel_dev9May18_reading_openbabel_dev4Aug17.txt.gz) --strip-trailing-cr | code -
```