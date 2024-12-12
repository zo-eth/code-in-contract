# code-in-contract
Solana's lowest gas on-chain project that allows everything to be written on-chain.


Introduction
Code-in is Solana On-chain storage system

Store values ​​using block state changes. 
"We don't allocate space to a single variable."


When users sign up, they are assigned two PDAs that can contain strings.

I will tell you what the two uses are.

PDA(data pda)
consists of 
Data: string (800 bytes)
Before_tx: string (100 bytes)

The first one is used to store data. 
The second is used to contain previous transactions.
The before_tx information of the first transaction left is called “Genesis”.

This is a wallet for storing data. 
It contains previous transactions Inspired by Linked List

DBPDA
consists of
tail_tx: String,
type_field: String,
offset: String,

tailtx 
For quick lookup, include the last tail_tx signature from the pda above

type_field
Write down what type you want to decode the data into. ex:image, video audio

offset
A space that can be freely used by developers as needed.
I wrote the data compression method name here.

DBPDA is a wallet for data inquiry.

--------------
After creating two types of wallets, as you might expect,
when storing data, the values of a general PDA are continuously updated.

During the update, it records the previous transaction information.
Finally, it completes the process by writing the last transaction information and its type to a location called dbPDA.

In other words, every transaction that occurs in the dbPDA points to the on-chain data stored by the user.
Think of this as using it like IPFS. Regardless of the data type, as long as it can be converted to a string, it can be stored.

By doing this, we can overcome the limitations of storage space on blockchain networks.
The website using Code In will be updated soon.

I will update this readme file as soon as I have time.

