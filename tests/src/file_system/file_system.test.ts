import { assert, expect, test } from "vitest";
import { runScenario, pause, Scenario, Player, Dna } from "@holochain/tryorama";
import { AppSignal, AppSignalCb, Record } from "@holochain/client";
import { decode } from "@msgpack/msgpack";

import {
  createFile,
  deleteFile,
  FileMetadata,
  fiveMbFileInput,
  getFileChunks,
  getFileMetadata,
  getFilesMetadataByPathRecursively,
  sampleFileInput,
  updateFile,
} from "./common";

function decodeOutputs(records: Record[]): unknown[] {
  return records.map((r) => decode((r.entry as any).Present.entry));
}

const hAppPath = process.cwd() + "/../workdir/soushi-cloud.happ";
const appSource = { appBundleSource: { path: hAppPath } };

async function runScenarioWithTwoAgents(
  callback: (
    scenario: Scenario,
    alice: Player,
    bob: Player,
    signals: Promise<AppSignal>
  ) => Promise<void>
) {
  await runScenario(async (scenario) => {
    let signalHandler: AppSignalCb | undefined;
    const signalReceveived = new Promise<AppSignal>((resolve) => {
      signalHandler = (signal) => {
        resolve(signal);
      };
    });

    const [alice, bob] = await scenario.addPlayersWithApps([
      appSource,
      appSource,
    ]);
    await scenario.shareAllAgents();

    await callback(scenario, alice, bob, signalReceveived);
  });
}

async function signalHandler() {
  let signalHandler: AppSignalCb | undefined;
  return new Promise<AppSignal>((resolve) => {
    signalHandler = (signal) => {
      resolve(signal);
    };
  });
}

test.only("create files and get files metadata by path", async () => {
  const checkSignal = signalHandler();
  await runScenarioWithTwoAgents(async (scenario, alice, bob, signals) => {
    // Create various files in different folders
    await createFile(alice.cells[0], sampleFileInput());
    await createFile(alice.cells[0], sampleFileInput("/", "index2.txt"));
    await createFile(alice.cells[0], sampleFileInput("/subfolder"));
    await createFile(
      alice.cells[0],
      sampleFileInput("/subfolder2", "index2.txt")
    );
    await createFile(alice.cells[0], sampleFileInput("/subfolder/subfolder3"));

    await pause(1200);

    // const actuelSignals = await signals;
    // console.log(actuelSignals);

    // Check if the correct number of files is returned for each path
    let readOutput: Record[] = await getFilesMetadataByPathRecursively(
      bob.cells[0],
      "/"
    );
    assert.equal(readOutput.length, 5);

    readOutput = await getFilesMetadataByPathRecursively(
      bob.cells[0],
      "/subfolder"
    );
    assert.equal(readOutput.length, 2);

    readOutput = await getFilesMetadataByPathRecursively(
      bob.cells[0],
      "/subfolder2"
    );
    assert.equal(readOutput.length, 1);

    readOutput = await getFilesMetadataByPathRecursively(
      bob.cells[0],
      "/subfolder/subfolder3"
    );
    assert.equal(readOutput.length, 1);

    // Check if the file metadata is correct
    const decodedOutput = decodeOutputs(readOutput) as FileMetadata[];
    assert.equal(decodedOutput[0].name, "test.txt");
    assert.equal(decodedOutput[0].path, "/subfolder/subfolder3");
  });
});

test("create large file and delete it", async () => {
  await runScenarioWithTwoAgents(async (scenario, alice, bob) => {
    // Create a large file
    const records = await createFile(
      alice.cells[0],
      fiveMbFileInput("/", "large_file.txt")
    );
    assert.equal(records.file_chunks.length, 5);

    await pause(1200);

    // Delete the large file
    await deleteFile(
      alice.cells[0],
      records.file_metadata.signed_action.hashed.hash
    );

    await pause(1200);

    // Check if the file metadata is deleted
    const readOutput = getFileMetadata(
      bob.cells[0],
      records.file_metadata.signed_action.hashed.hash
    );
    expect(readOutput).rejects.toThrow();
  });
});

test("create file, update it, read it and delete in cascade", async () => {
  await runScenarioWithTwoAgents(async (scenario, alice, bob) => {
    // Create a file
    const records = await createFile(alice.cells[0], sampleFileInput());
    const original_action_hash =
      records.file_metadata.signed_action.hashed.hash;
    assert.equal(records.file_chunks.length, 1);

    await pause(1200);

    // Try to create a file with the same path (should fail)
    const newRecords = createFile(
      alice.cells[0],
      sampleFileInput("/", "test.txt", "new content")
    );
    expect(newRecords).rejects.toThrow();

    await pause(1200);

    // Update the file
    const updatedRecords = await updateFile(
      alice.cells[0],
      original_action_hash,
      new TextEncoder().encode("new content")
    );
    assert.ok(updatedRecords);

    await pause(1200);

    // Read the updated file
    const chunksRecords = await getFileChunks(
      bob.cells[0],
      original_action_hash
    );
    const decodedChunks = new TextDecoder().decode(
      decodeOutputs(chunksRecords)[0] as Uint8Array
    );
    assert.equal(decodedChunks, "new content");

    // Update the file again
    const secondUpdateRecords = await updateFile(
      alice.cells[0],
      original_action_hash,
      new TextEncoder().encode("new content 2")
    );
    assert.ok(secondUpdateRecords);

    await pause(1200);

    // Read the updated file again
    const secondChunksRecords = await getFileChunks(
      bob.cells[0],
      original_action_hash
    );
    const decodedSecondChunks = new TextDecoder().decode(
      decodeOutputs(secondChunksRecords)[0] as Uint8Array
    );
    assert.equal(decodedSecondChunks, "new content 2");

    // Delete the file and related updates
    await deleteFile(alice.cells[0], original_action_hash);

    await pause(1200);

    // Check if the file metadata is deleted
    const readOutput = await getFileMetadata(
      bob.cells[0],
      original_action_hash
    );
    assert.isNull(readOutput);

    // Check if the file chunks are deleted
    const readChunksOutput = await getFileChunks(
      bob.cells[0],
      original_action_hash
    );
    assert.equal(readChunksOutput.length, 0);

    // Check if the second update's metadata is deleted
    const readSecondOutput = await getFileMetadata(
      bob.cells[0],
      secondUpdateRecords.file_metadata.signed_action.hashed.hash
    );
    assert.isNull(readSecondOutput);

    // Check if the second update's chunks are deleted
    const readSecondChunksOutput = await getFileChunks(
      bob.cells[0],
      secondUpdateRecords.file_metadata.signed_action.hashed.hash
    );
    assert.equal(readSecondChunksOutput.length, 0);
  });
});

test("create file with empty name and check path standardization", async () => {
  await runScenarioWithTwoAgents(async (scenario, alice, bob) => {
    // Test case 1: Empty name
    await expect(
      createFile(alice.cells[0], sampleFileInput("/", ""))
    ).rejects.toThrow();

    // Test case 2: Path with forbidden characters
    await expect(
      createFile(alice.cells[0], sampleFileInput("/?.34", "test.txt"))
    ).rejects.toThrow();

    // Test case 3: Standardize path with '/'
    const record1 = await createFile(
      alice.cells[0],
      sampleFileInput("/", "test.txt")
    );
    assert.ok(record1);
    const decodedOutput1 = decodeOutputs([
      record1.file_metadata,
    ])[0] as FileMetadata;

    // Test case 4: Standardize path with '\'
    const record2 = await createFile(
      alice.cells[0],
      sampleFileInput("\\", "test2.txt")
    );
    assert.ok(record2);
    const decodedOutput2 = decodeOutputs([
      record2.file_metadata,
    ])[0] as FileMetadata;

    // Check if paths are standardized correctly
    assert.equal(decodedOutput1.path, decodedOutput2.path);

    // Test case 5: Standardize path with multiple '/'
    const record3 = await createFile(
      alice.cells[0],
      sampleFileInput("/subfolder///subfolder2", "test3.txt")
    );
    assert.ok(record3);
    const decodedOutput3 = decodeOutputs([
      record3.file_metadata,
    ])[0] as FileMetadata;
    assert.equal(decodedOutput3.path, "/subfolder/subfolder2");
  });
});

function extractErrorMessage(input: string): string | null {
  const regex = /Guest\("([^"]+)"\)/;
  const match = input.match(regex);

  if (match && match[1]) {
    return match[1];
  }

  return null;
}
