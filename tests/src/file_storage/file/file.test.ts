// import {assert, test} from "vitest";
//
// import {runScenario, pause, CallableCell} from '@holochain/tryorama';
// import {
// 	NewEntryAction,
// 	ActionHash,
// 	Record,
// 	AppBundleSource,
// 	fakeDnaHash,
// 	fakeActionHash,
// 	fakeAgentPubKey,
// 	fakeEntryHash
// } from '@holochain/client';
// import {decode} from '@msgpack/msgpack';
//
// import {createFileMetadata} from "./common";
//
// test('create FileMetadata', async () => {
// 	await runScenario(async scenario => {
// 		// Construct proper paths for your app.
// 		// This assumes app bundle created by the `hc app pack` command.
// 		const testAppPath = process.cwd() + '/../workdir/soushi-coud.happ';
//
// 		// Set up the app to be installed
// 		const appSource = {appBundleSource: {path: testAppPath}};
//
// 		// Add 2 players with the test app to the Scenario. The returned players
// 		// can be destructured.
// 		const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);
//
// 		// Shortcut peer discovery through gossip and register all agents in every
// 		// conductor of the scenario.
// 		await scenario.shareAllAgents();
//
// 		// Alice creates a FileMetadata
// 		const record: Record = await createFileMetadata(alice.cells[0]);
// 		assert.ok(record);
// 	});
// });