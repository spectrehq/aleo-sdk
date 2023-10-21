import * as fs from "fs";
import { fileURLToPath } from "url";
import wasm from "../dist/wasm.js";

const {
    initThreadPool: wasmInitThreadPool,
    Address,
    Execution,
    ExecutionResponse,
    Field,
    ProgramID,
    Metadata,
    OfflineQuery,
    Private,
    PrivateKey,
    PrivateKeyCiphertext,
    Program,
    ProvingKey,
    RecordCiphertext,
    RecordPlaintext,
    Plaintext,
    ProgramManager,
    Signature,
    Transaction,
    ViewKey,
    VerifyingKey,
    verifyFunctionExecution,
} = await wasm({
    importHook: () => {
        const url = new URL("assets/aleo_wasm.wasm", import.meta.url);
        if (typeof window !== 'undefined') {
            return url;
        } else {
            return fs.readFileSync(fileURLToPath(url));
        }
    },
});

async function initThreadPool(threads) {
    if (threads == null) {
        threads = navigator.hardwareConcurrency;
    }

    console.info(`Spawning ${threads} threads`);

    await wasmInitThreadPool(new URL("worker.js", import.meta.url), threads);
}

export {
    initThreadPool,
    Address,
    Execution,
    ExecutionResponse,
    Field,
    ProgramID,
    Metadata,
    OfflineQuery,
    PrivateKey,
    PrivateKeyCiphertext,
    Program,
    ProvingKey,
    RecordCiphertext,
    RecordPlaintext,
    Plaintext,
    ProgramManager,
    Signature,
    Transaction,
    ViewKey,
    VerifyingKey,
    verifyFunctionExecution,
};
