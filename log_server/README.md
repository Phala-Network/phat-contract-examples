# Demo cluster log server

## How to play

### Prerequirements
Build and run a brand new local testnet with the following arguments

```bash
./phala-node --ws-port 19944 --dev --pruning=archive
```

```
./pruntime
```

```bash
./pherry --dev --no-wait --substrate-ws-endpoint ws://localhost:19944
```

### Build and deploy the demo server
Build & deploy:
```bash
make test
```

After the deploy succeeded, we can get the logs from the log server:
```bash
curl localhost:18080/log
```
The port `18080` is hardcoded in the contract ATM.
