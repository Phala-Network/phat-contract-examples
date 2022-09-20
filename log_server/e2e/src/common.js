const fs = require('fs');
const crypto = require('crypto');
const { checkUntil, checkUntilEq, hex } = require('./utils');
const { assert } = require('console');

function loadContract(name) {
    const wasmPath = `../target/ink/${name}.wasm`;
    const sideprogPath = `../sideprog.wasm`;
    const metadataPath = `../target/ink/metadata.json`;
    const wasm = hex(fs.readFileSync(wasmPath, 'hex'));
    const metadata = JSON.parse(fs.readFileSync(metadataPath));
    const constructor = metadata.V3.spec.constructors.find(c => c.label == 'default').selector;
    const sideprog = fs.existsSync(sideprogPath) ? hex(fs.readFileSync(sideprogPath, 'hex')) : null;
    return { wasm, metadata, constructor, sideprog };
}

async function deployContract(api, txqueue, pair, contract, clusterId) {
    console.log('Contracts: uploading');
    // upload the contract 
    const { events: deployEvents } = await txqueue.submit(
        api.tx.utility.batchAll(
            [
                api.tx.phalaFatContracts.clusterUploadResource(clusterId, 'SidevmCode', contract.sideprog),
                api.tx.phalaFatContracts.clusterUploadResource(clusterId, 'InkCode', contract.wasm),
                api.tx.phalaFatContracts.instantiateContract(
                    { WasmCode: contract.metadata.source.hash },
                    contract.constructor,
                    hex(crypto.randomBytes(4).toString('hex')), // salt
                    clusterId,
                )
            ]
        ),
        pair
    );
    const contractIds = deployEvents
        .filter(ev => ev.event.section == 'phalaFatContracts' && ev.event.method == 'Instantiating')
        .map(ev => ev.event.data[0].toString());
    const numContracts = 1;
    console.assert(contractIds.length == numContracts, 'Incorrect length:', `${contractIds.length} vs ${numContracts}`);
    contract.address = contractIds[0];
    await checkUntilEq(
        async () => (await api.query.phalaFatContracts.clusterContracts(clusterId))
            .filter(c => contractIds.includes(c.toString()))
            .length,
        numContracts,
        4 * 6000
    );
    console.log('Contracts: uploaded');
    await checkUntil(
        async () => (await api.query.phalaRegistry.contractKeys(contract.address)).isSome,
        4 * 6000
    );
    console.log('Contracts:', contract.address, 'key ready');
    console.log('Contracts: deployed');
}

async function setLogHanlder(api, txqueue, pair, clusterId, contract) {
    const { events } = await txqueue.submit(
        api.tx.phalaFatContracts.clusterSetLogHandler(clusterId, contract),
        pair
    );

    await checkUntilEq(async () =>
        events
            .filter(ev => ev.event.section == 'phalaFatContracts' && ev.event.method == 'ClusterSetLogReceiver')
            .length,
        1
    );
    console.log('Cluster: Log hander set');
}

module.exports = {
    loadContract,
    deployContract,
    setLogHanlder,
}
