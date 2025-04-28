# Digital Goods Rental Smart Contract

## 📘 Table of Contents
- [Project Title](#project-title)
- [Project Description](#project-description)
- [Project Vision](#project-vision)
- [Key Features](#key-features)
- [Contract Details](#contract-details)

---

## 📌 Project Title
**Digital Goods Rental**

---

## 📄 Project Description
The Digital Goods Rental smart contract allows users to list and rent digital assets such as e-books, music files, digital art, or software licenses on-chain. It simplifies the process of managing access to digital assets for a temporary period without relying on a centralized authority.

---

## 🚀 Project Vision
To provide a decentralized, trustless platform where digital assets can be temporarily shared or rented out, ensuring ownership rights remain intact while allowing secure short-term access.

---

## ✨ Key Features
- List a digital good with the owner's information
- Rent the listed item as a verified renter
- Return the item to make it available again
- View the current rental status

---

## 🔧 Contract Details

### Contract Address: CCDYMKZPJPYMOXXTZAIEDE37WZTD4VT64J2FAKXTFRZFOBKE67CWGA4N 

### `list_item(env, owner, item_name)`
List a new digital item for rent. Only the item owner can call this function.

### `rent_item(env, renter)`
Rents the item if it is available. Verifies that the item is not currently rented.

### `return_item(env, renter)`
Allows the current renter to return the item, marking it as available again.

### `get_item_status(env)`
Fetches the current status of the item including who owns or is renting it.

---

✅ Built with [Soroban SDK](https://soroban.stellar.org/docs).
