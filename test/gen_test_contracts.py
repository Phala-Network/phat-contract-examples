#!/usr/bin/env python
import json
import codecs

from hashlib import blake2b


def replace(tpl_wasm, key, n):
    fmt = f"{{:>0{len(key)}d}}"
    replacement = fmt.format(n)
    return tpl_wasm.replace(key, replacement.encode())


def render(contract_filename: str, key, rng):
    path_base, ext = contract_filename.rsplit('.', 1)
    assert(ext == 'contract')

    contract = json.load(open(contract_filename))
    tpl_wasm = codecs.decode(contract['source']['wasm'][2:], 'hex')

    for i in rng:
        wasm = replace(tpl_wasm, key, i)
        hash = blake2b(wasm, digest_size=32).hexdigest()
        contract['source']['wasm'] = '0x' + codecs.encode(wasm, 'hex').decode()
        contract['source']['hash'] = '0x' + hash
        json.dump(contract,  open(f'{path_base}.{i}.contract', 'w'), indent=4)
    

if __name__ == '__main__':
    render('target/ink/test.contract', b'__bin_id__', range(10000))
