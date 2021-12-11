# Eincoin

my personal bitcoin clone

- every node opens a server and a client
- messages are bincode encoded message structs
- servers broadcast all messages from server to connected clients
- servers broadcast all messages from clients to connected server
- miners and nodes
- nodes publish transactions
- miners solve node
- libs:
  - TcpListener for server
  - openssl for keypairs & verification
  - sha256 for hashing
