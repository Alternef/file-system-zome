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

import {createFileMetadata, sampleFile} from "./common";

const hAppPath = process.cwd() + '/../workdir/soushi-cloud.happ';
const appSource = {appBundleSource: {path: hAppPath}};

test('create and read FileMetadata', async () => {
	await runScenario(async scenario => {
		const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource])
		await scenario.shareAllAgents();

		let file = sampleFile(alice.agentPubKey);
		// Alice creates a FileMetadata
		const record: Record = await createFileMetadata(alice.cells[0], file);
		assert.ok(record);

		scenario.cleanUp();
	});
});