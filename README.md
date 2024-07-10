# EMNetChain
- EMNetChain is a robust toolchain, drawing inspiration from WebRTC. It's a cross-platform server that can be utilized to establish nodes across the internet.

## Why?
- EMNetChain provides a generalized and straightforward approach for creating nodes across the internet. This makes it an accessible and efficient tool for network configuration and management.
- EMNetChain operates on simple JSON-structured texts.
- EMNetChain is designed as an educational application and is structured using multiple crates. This modular approach enhances the readability of the source code. (EMNet , EMDisi_lib , EMUtil , simple_json_parser)
- it can be used to create vpns , blockchains and etc

## install
```shell
cargo install EMNetChain
```

## Getting started
- you can start the app with cli flags -dis-addr and -chain-addr and both of them must be in ipv4:port format
### example:
```shell
EMNetChain -dis-addr 127.0.0.1:8080 -chain-addr 127.0.0.1:8081
```
- if it does not start with cli args , it will ask you to provide them for it
- it will start two servers , EMDisi and EMNetChain

### EMNetChain Server
- this server is used to create nodes and connect them to each other
- each client creates two udp sockets , one as tail and other one as head
- clients connect to it and send json messages to it in this format
```json
{
    "msg":"CONNECT",
    "head":"127.0.0.1:1222",
    "tail": "127.0.0.1:1224"
}
```
and then waits till server replies in this format
```json
{"msg":"CONNECT","part":"head","ip":"127.0.0.1:1224"}
```
**part specifies which udp socket must connect to that ipv4:port**

### EMDisi Server
- this server is used to repair the chain , if a client disconnects and etc
- In the event of a disconnection, the client initiates a protocol to maintain the integrity of the network. This involves the client sending a notification to its connected peers, indicating its impending offline status, accompanied by a unique identifier. Following this, the two clients that received the notification communicate with the server in a predefined format :
```json
{
    "msg":"RECONNECT",
    "id":"1234",
    "ip": "127.0.0.1:1224"
}
```
ip will be ipv4:port of udp socket which lost its friend
and server will reply this way if new friend is available
```json
{"msg":"CONNECT","ip":"127.0.0.1:1223"}
```
**you can write your own id gen server to make ids 100% unique**

### EMUDP Server
- to users get ipv4:port of their udp sockets , this server just echos the ipv4:port of connected clients https://github.com/SkillfulElectro/EMNet/tree/main/src/EMUDP

## TODO
- support dynamic ip addresses

## Conclusion
EMNetChain, inspired by WebRTC, is a robust toolchain for creating and managing internet nodes. Its user-friendly design, based on JSON-structured texts and multiple crates, makes it ideal for educational purposes and versatile applications like VPNs and blockchains. Its unique protocol ensures network integrity, even during disconnections, making it a reliable solution for network management. 
