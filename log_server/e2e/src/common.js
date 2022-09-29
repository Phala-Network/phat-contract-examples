const fs = require('fs');
const crypto = require('crypto');
const Phala = require('@phala/sdk');
const { checkUntil, checkUntilEq, hex } = require('./utils');

function loadContractFile(contractFile) {
    const metadata = JSON.parse(fs.readFileSync(contractFile));
    const constructor = metadata.V3.spec.constructors.find(c => c.label == 'default').selector;
    const name = metadata.contract.name;
    const wasm = metadata.source.wasm;
    return { wasm, metadata, constructor, name };
}

function loadContractDir(path) {
    const metadataPath = `${path}/target/ink/metadata.json`;
    const metadata = JSON.parse(fs.readFileSync(metadataPath));
    const name = metadata.contract.name;
    const wasmPath = `${path}/target/ink/${name}.wasm`;
    const sideprogPath = `${path}/sideprog.wasm`;
    const wasm = hex(fs.readFileSync(wasmPath, 'hex'));
    const constructor = metadata.V3.spec.constructors.find(c => c.label == 'default').selector;
    const sideprog = fs.existsSync(sideprogPath) ? hex(fs.readFileSync(sideprogPath, 'hex')) : null;
    return { wasm, metadata, constructor, sideprog, name };
}

async function deployContract(api, txqueue, pair, contract, clusterId) {
    console.log(`Contracts: uploading ${contract.name}`);
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
    console.log(`Contracts: ${contract.name} deployed`);
    return contract.address;
}

async function setLogHanlder(api, txqueue, pair, clusterId, system, contract) {
    await txqueue.submit(
        system.tx['system::setDriver']({}, "PinkLogger", contract),
        pair,
    );

    const certAlice = await Phala.signCertificate({ api, pair });
    await checkUntilEq(
        async () => {
            const { output } = await system.query['system::getDriver'](certAlice, {}, "PinkLogger");
            return output.toHex();
        },
        contract,
        4 * 6000
    );

    console.log('Cluster: Log hander set');
}

async function uploadSystemCode(api, txqueue, pair, wasm) {
    console.log(`Uploading system code`);
    await txqueue.submit(
        api.tx.sudo.sudo(api.tx.phalaFatContracts.setPinkSystemCode(hex(wasm))),
        pair
    );
    console.log(`Uploaded system code`);
}

module.exports = {
    loadContractFile,
    loadContractDir,
    deployContract,
    setLogHanlder,
    uploadSystemCode,
}
