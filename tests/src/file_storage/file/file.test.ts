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

import {createFileMetadata, FileMetadata, getFileMetadata, sampleFile} from "./common";

function decodeOutput(record: Record) {
	return decode((record.entry as any).Present.entry);
}

const hAppPath = process.cwd() + '/../workdir/soushi-cloud.happ';
const appSource = {appBundleSource: {path: hAppPath}};

test('create and read FileMetadata', async () => {
	await runScenario(async scenario => {
		const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource])
		await scenario.shareAllAgents();

		let file = sampleFile(alice.agentPubKey);
		const record: Record = await createFileMetadata(alice.cells[0], file);
		assert.ok(record);

		await pause(1200);

		const readOutput: Record = await getFileMetadata(bob.cells[0], record.signed_action.hashed.hash);
		const decodedOutput = decodeOutput(readOutput) as FileMetadata;
		assert.equal(decodedOutput.name, file.name);

		scenario.cleanUp();
	});
});