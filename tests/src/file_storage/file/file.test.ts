import {assert, expect, test} from "vitest";

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
	createFile, deleteFile, FileMetadata, fiveMbFileInput, getFileMetadata,
	getFilesMetadataByPathRecursively, sampleFileInput, updateFile,
} from "./common";

function decodeOutputs(records: Record[]): unknown[] {
	return records.map(r => decode((r.entry as any).Present.entry));
}

const hAppPath = process.cwd() + '/../workdir/soushi-cloud.happ';
const appSource = {appBundleSource: {path: hAppPath}};

// test('create files and get files metadata by path', async () => {
// 	await runScenario(async scenario => {
// 		const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource])
// 		await scenario.shareAllAgents();
//
// 		// /index.txt
// 		await createFile(alice.cells[0], sampleFileInput());
// 		// /index2.txt
// 		await createFile(alice.cells[0], sampleFileInput("/", "index2.txt"));
// 		// /subfolder/index.txt
// 		await createFile(alice.cells[0], sampleFileInput("subfolder"));
// 		// /subfolder2/index2.txt
// 		await createFile(alice.cells[0], sampleFileInput("subfolder2", "index2.txt"));
// 		// /subfolder/subfolder3/index.txt
// 		await createFile(alice.cells[0], sampleFileInput("subfolder/subfolder3"));
//
// 		await pause(1200);
//
// 		let readOutput: Record[] = await getFilesMetadataByPathRecursively(bob.cells[0], "/");
// 		assert.equal(readOutput.length, 5);
//
// 		readOutput = await getFilesMetadataByPathRecursively(bob.cells[0], "/subfolder");
// 		assert.equal(readOutput.length, 2);
//
// 		readOutput = await getFilesMetadataByPathRecursively(bob.cells[0], "/subfolder2");
// 		assert.equal(readOutput.length, 1);
//
// 		readOutput = await getFilesMetadataByPathRecursively(bob.cells[0], "/subfolder/subfolder3");
// 		assert.equal(readOutput.length, 1);
//
// 		const decodedOutput = decodeOutputs(readOutput) as FileMetadata[];
// 		console.log(decodedOutput);
// 		assert.equal(decodedOutput[0].name, "test.txt");
// 		assert.equal(decodedOutput[0].path, "subfolder/subfolder3");
// 	});
// });
//
// test('create large file and delete it', async () => {
// 	await runScenario(async scenario => {
// 		const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource])
// 		await scenario.shareAllAgents();
//
// 		const records = await createFile(alice.cells[0], fiveMbFileInput("/", "large_file.txt"));
// 		assert.equal(records.file_chunks.length, 5);
//
// 		await pause(1200);
//
// 		await deleteFile(alice.cells[0], records.file_metadata.signed_action.hashed.hash);
//
// 		await pause(1200);
//
// 		const readOutput = getFileMetadata(bob.cells[0], records.file_metadata.signed_action.hashed.hash);
// 		expect(readOutput).rejects.toThrow();
// 	});
// });

test('create file and update it', async () => {
	await runScenario(async scenario => {
		const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource])
		await scenario.shareAllAgents();

		const records = await createFile(alice.cells[0], sampleFileInput());
		assert.equal(records.file_chunks.length, 1);

		await pause(1200);

		const newRecords = createFile(alice.cells[0], sampleFileInput("/", "test.txt", "new content"));
		expect(newRecords).rejects.toThrow();

		await pause(1200);

		const readOutput = await getFileMetadata(bob.cells[0], records.file_metadata.signed_action.hashed.hash);
		const decodedOutput = decodeOutputs([readOutput])[0] as FileMetadata;
		console.log(new TextDecoder().decode(decodedOutput.chunks_hashes[0]));

		await pause(1200);

		const updatedRecords = await updateFile(alice.cells[0], records.file_metadata.signed_action.hashed.hash, new TextEncoder().encode("new content"));
		assert.ok(updatedRecords);
		const decodedUpdatedOutput =  decodeOutputs([updatedRecords.file_metadata])[0] as FileMetadata;
		console.log(new TextDecoder().decode(decodedUpdatedOutput.chunks_hashes[0]));
	})
});