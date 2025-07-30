Here's a clean and well-formatted version of your `README.md` for the **flash-loan** smart contract using instruction introspection.

---

# 💸 Flash Loan

A native Solana flash loan smart contract that leverages **instruction introspection** for secure loan validation.

---

## 📦 Setup Guide


## 📌 Program Deployments

| Network | Program ID                                     |
| ------- | ---------------------------------------------- |
| Mainnet |  |
| Devnet  |  |

---

## 📚 Overview

The flash loan contract uses:

* 🧠 **Instruction introspection** to verify that repayment occurred in the **same transaction**
* 🧮 A **constant product model** for token borrowing/repayment
* 🔁 Safe composability with other Solana programs

---

## ✅ Environment Setup

1. **Install Rust**
   [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

2. **Install Solana CLI**
   [https://docs.solana.com/cli/install-solana-cli-tools](https://docs.solana.com/cli/install-solana-cli-tools)

3. **Generate a keypair**

   ```bash
   solana-keygen new
   ```

---

## 🧪 Build Instructions

Clone the repository and enter the source directory:

```bash
git clone https://github.com/SolanaCore/flash-loan
cd flash-loan/program
```

### 🔧 Mainnet Build

```bash
cargo build-sbf
```

### 🔧 Devnet Build

```bash
cargo build-sbf --features devnet
```

### 🔧 Localnet Build

Before building for localnet, update the program IDs in the `config_feature` file with your local keys:

```bash
cargo build-sbf --features localnet
```

> 🔍 After building, the smart contract `.so` and `.json` artifacts will be found in the `./target/deploy/` directory.

---

## 🚀 Deploy

Deploy your built program:

```bash
solana program deploy ./target/deploy/flash_loan.so
```

> ⚠️ **Always double-check** your Solana config:

```bash
solana config get
```

Make sure you're deploying to the correct cluster (`localnet`, `devnet`, `mainnet-beta`).

---

## 🦀Script
```sh
chmod +x ../cicd.sh
../cicd.sh
``` 

## 📑 Resources

* [Flash Loan Dev Docs](#) *(https://learn.blueshift.gg/en/challenges/anchor-flash-loan)*

---
Builder: [Dhruv Khandelwal](#)(gitbub.com/DhruvWebDev)
