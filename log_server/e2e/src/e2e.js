const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { ContractPromise } = require('@polkadot/api-contract');
const Phala = require('@phala/sdk');

const { TxQueue, checkUntil, hex } = require('./utils');
const { loadContractDir, loadContractFile, deployContract, setLogHanlder, uploadSystemCode } = require('./common');

async function getWorkerPubkey(api) {
    const workers = await api.query.phalaRegistry.workers.entries();
    const worker = workers[0][0].args[0].toString();
    return worker;
}

async function setupGatekeeper(api, txpool, pair, worker) {
    if ((await api.query.phalaRegistry.gatekeeper()).length > 0) {
        return;
    }
    console.log('Gatekeeper: registering');
    await txpool.submit(
        api.tx.sudo.sudo(
            api.tx.phalaRegistry.registerGatekeeper(worker)
        ),
        pair,
    );
    await checkUntil(
        async () => (await api.query.phalaRegistry.gatekeeper()).length == 1,
        4 * 6000
    );
    console.log('Gatekeeper: added');
    await checkUntil(
        async () => (await api.query.phalaRegistry.gatekeeperMasterPubkey()).isSome,
        4 * 6000
    );
    console.log('Gatekeeper: master key ready');
}

async function deployCluster(api, txqueue, sudoer, owner, worker, defaultCluster = '0x0000000000000000000000000000000000000000000000000000000000000000') {
    const clusterInfo = await api.query.phalaFatContracts.clusters(defaultCluster);
    if (clusterInfo.isSome) {
        return { clusterId: defaultCluster, systemContract: clusterInfo.unwrap().systemContract.toHex() };
    }
    console.log('Cluster: creating');
    // crete contract cluster and wait for the setup
    const { events } = await txqueue.submit(
        api.tx.sudo.sudo(api.tx.phalaFatContracts.addCluster(
            owner,
            'Public', // can be {'OnlyOwner': accountId}
            [worker]
        )),
        sudoer
    );
    const ev = events[1].event;
    console.assert(ev.section == 'phalaFatContracts' && ev.method == 'ClusterCreated');
    const clusterId = ev.data[0].toString();
    const systemContract = ev.data[1].toString();
    console.log('Cluster: created', clusterId)
    await checkUntil(
        async () => (await api.query.phalaRegistry.clusterKeys(clusterId)).isSome,
        4 * 6000
    );
    await checkUntil(
        async () => (await api.query.phalaRegistry.contractKeys(systemContract)).isSome,
        4 * 6000
    );
    return { clusterId, systemContract };
}

async function main() {
    const contractSystem = loadContractFile('../../pink_system.contract');
    const contractSidevmop = loadContractDir('../../sidevmop');
    const contract = loadContractDir('..');

    // connect to the chain
    const wsProvider = new WsProvider('ws://localhost:19944');
    const api = await ApiPromise.create({
        provider: wsProvider,
        types: {
            ...Phala.types,
            'GistQuote': {
                username: 'String',
                accountId: 'AccountId',
            },
        }
    });
    const txqueue = new TxQueue(api);

    // prepare accounts
    const keyring = new Keyring({ type: 'sr25519' })
    const alice = keyring.addFromUri('//Alice')
    const bob = keyring.addFromUri('//Bob')

    // connect to pruntime
    const pruntimeURL = 'http://localhost:8000';
    const prpc = Phala.createPruntimeApi(pruntimeURL);
    const worker = await getWorkerPubkey(api);
    const connectedWorker = hex((await prpc.getInfo({})).publicKey);
    console.log('Worker:', worker);
    console.log('Connected worker:', connectedWorker);

    // basic phala network setup
    await setupGatekeeper(api, txqueue, alice, worker);

    // Upload the pink-system wasm to the chain. It is required to create a cluster.
    await uploadSystemCode(api, txqueue, alice, contractSystem.wasm);

    const { clusterId, systemContract } = await deployCluster(api, txqueue, alice, alice.address, worker);

    // deploy the driver contracts
    const driverAddress = await deployContract(api, txqueue, bob, contractSidevmop, clusterId);

    const newApi = await api.clone().isReady;
    const system = new ContractPromise(
        await Phala.create({ api: newApi, baseURL: pruntimeURL, contractId: systemContract }),
        contractSystem.metadata,
        systemContract
    );
    // setup driver permissions
    await txqueue.submit(
        system.tx["system::setDriver"]({}, "SidevmOperation", driverAddress),
        alice
    );
    await txqueue.submit(
        system.tx["system::grantAdmin"]({}, driverAddress),
        alice
    );
    // deploy our contract
    await deployContract(api, txqueue, bob, contract, clusterId);

    // set the contract as the log handler for the cluster
    await setLogHanlder(api, txqueue, alice, clusterId, system, contract.address);
}

main().then(process.exit).catch(err => console.error('Crashed', err)).finally(() => process.exit(-1));
