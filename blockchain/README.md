I have no previous experience with blockchain, but I do have some theoretical knowledge.

**Briefly explain Bitcoin's UTXO model of transaction validation (separate from POW)**  
UTXO - Unspent Transaction Output. This means that you keep the rest of the cryptocurrency that you receive from each transaction.  
e.g. You have 40 btc UTXO on your Bitcoin wallet. After making a transaction for 30 btc, your "40btc" UTXO is "deleted" and a new one for 10btc is created.
And the recipient of the transaction gets 30btc UTXO on his wallet.  
The application of the UTXO principle protects against coin "coupling" - payment for transactions with the same coins, or spending non-existent coins.
Also, this principle greatly simplifies the calculations on the blockchain. Instead of storing information about each individual transaction in blocks, we simply keep track of coins that have not been spent.

**What is the structure of a Block in bitcoin and how does it relate to the 'blockchain' (merkle tree vs merkle list of merkle trees)**  
Each bitcoin block has a header. Header contains values: a previous block hash and a merkle tree root. The merkle tree structure means changing the merkle tree root when any component of the tree changes.
Based on this fact, the bitcoin block uses a merkle tree to store the description of transactions.

**Explain some of the ways hashing functions enable blockchain technology**
1. The POW principle is implemented through the calculation of the hash function. This approach makes the operation quite complicated on the part of the user, but quite easy on the part of the verifier.
2. As I said above, the merkle tree is great for storing transactions and protecting against substitution information about previous transactions.