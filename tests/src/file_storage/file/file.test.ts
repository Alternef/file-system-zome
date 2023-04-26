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
	createFile, CreateFileOutput, FileMetadata, getFilesMetadataByPath,
	sampleFileInput,
} from "./common";

function decodeOutputs(records: Record[]): unknown[] {
	return records.map(r => decode((r.entry as any).Present.entry));
}

const hAppPath = process.cwd() + '/../workdir/soushi-cloud.happ';
const appSource = {appBundleSource: {path: hAppPath}};

// test('create File and read FileMetadata', async () => {
// 	await runScenario(async scenario => {
// 		const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource])
// 		await scenario.shareAllAgents();
//
// 		let file = sampleFileInput();
// 		const records: CreateFileOutput = await createFile(alice.cells[0], file);
// 		assert.ok(records);
//
// 		await pause(1200);
//
// 		const readOutput: Record = await getFileMetadata(bob.cells[0], records.file_metadata.signed_action.hashed.hash);
// 		const decodedOutput = decodeOutputs([readOutput])[0] as FileMetadata;
// 		assert.equal(decodedOutput.name, file.name);
//
// 		await pause(1200);
//
// 		const readOutput2: Record = await getFileChunk(bob.cells[0], decodedOutput.chunks_hashes[0]);
// 		assert.ok(readOutput2);
//
// 		const decoder = new TextDecoder();
// 		const decodedOutput2 = decodeOutputs([readOutput2])[0] as Uint8Array;
// 		const decodedString = decoder.decode(decodedOutput2);
// 		const decodedFileString = decoder.decode(file.content);
// 		assert.equal(decodedString, decodedFileString);
// 	});
// });

test('ensure folder structure', async () => {
	await runScenario(async scenario => {
		const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource])
		await scenario.shareAllAgents();

		let rootFile = sampleFileInput();
		const records: CreateFileOutput = await createFile(alice.cells[0], rootFile);
		assert.ok(records);

		await pause(1200);

		let subFolderFile = sampleFileInput("/subfolder");
		const records2: CreateFileOutput = await createFile(alice.cells[0], subFolderFile);

		// let subFolderFile2 = sampleFileInput("/subfolder/subfolder2");
		// const records3: CreateFileOutput = await createFile(alice.cells[0], subFolderFile2);


		await pause(1200);

		const readOutput: Record[] = await getFilesMetadataByPath(bob.cells[0], "/subfolder");
		console.log("Number of files :", readOutput.length);
		// assert.equal(readOutput.length, 2);


		// const readOutput2: Record[] = await getFilesMetadataByPath(bob.cells[0], "/subfolder");
		// assert.equal(readOutput2.length, 1);
	});
});