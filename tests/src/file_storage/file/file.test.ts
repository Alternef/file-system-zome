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

import {createFileMetadata} from "./common";

const hAppPath = process.cwd() + '/../workdir/test-happ.happ';
const appSource = {appBundleSource: {path: hAppPath}};

test('create FileMetadata', async () => {
	await runScenario(async scenario => {
		const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource])
	// 	await scenario.shareAllAgents();
	//
	// 	// Alice creates a FileMetadata
	// 	const record: Record = await createFileMetadata(alice.cells[0]);
	// 	assert.ok(record);
	});
});