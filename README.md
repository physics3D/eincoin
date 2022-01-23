# Eincoin

My try at implementing a cryptocurrency

- P2P Network Topology: Tree (every node: 1 connection to server, multiple clients)
- every node opens a server and a client
- messages are bincode encoded Message structs
- addresses are the raw public keys
- servers broadcast all messages
  - from server to connected clients
  - from clients to connected server
- miners and nodes
- nodes publish transactions
- miners solve blocks and send blocks back through the network
- crates:
  - std TcpListener & TcpStream for networking
  - rsa for keypairs & verification
  - sha256 for hashing
  - serde & bincode for (de-)serialization
