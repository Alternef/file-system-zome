import {assert, test} from "vitest";

import {runScenario, pause, CallableCell} from '@holochain/tryorama';
import {
	NewEntryAction,
	ActionHash,
	Record,
	AppBundleSource,
	fakeDnaHash,
	fakeActionHash,
	fakeAgentPubKey,
	fakeEntryHash
} from '@holochain/client';
import {decode} from '@msgpack/msgpack';

import {
	createFile,
	FileMetadata, getFileChunk,
	getFileMetadata,
	sampleFileInput,
} from "./common";

function decodeOutput(record: Record) {
	return decode((record.entry as any).Present.entry);
}

const hAppPath = process.cwd() + '/../workdir/soushi-cloud.happ';
const appSource = {appBundleSource: {path: hAppPath}};

test('create File and read FileMetadata', async () => {
	await runScenario(async scenario => {
		const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource])
		await scenario.shareAllAgents();

		let file = sampleFileInput();
		const records: Record[] = await createFile(alice.cells[0], file);
		assert.ok(records);

		await pause(1200);

		const readOutput: Record = await getFileMetadata(bob.cells[0], records[0].signed_action.hashed.hash);
		const decodedOutput = decodeOutput(readOutput) as FileMetadata;
		assert.equal(decodedOutput.name, file.name);

		await pause(1200);

		const readOutput2: Record = await getFileChunk(bob.cells[0], decodedOutput.chunks_hashes[0]);
		assert.ok(readOutput2);

		const decoder = new TextDecoder();
		const decodedOutput2 = decodeOutput(readOutput2) as Uint8Array;
		const decodedString = decoder.decode(decodedOutput2);
		const decodedFileString = decoder.decode(file.content);
		assert.equal(decodedString, decodedFileString);
	});
});