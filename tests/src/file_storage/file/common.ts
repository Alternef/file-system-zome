import {CallableCell} from '@holochain/tryorama';
import {
	NewEntryAction,
	ActionHash,
	Record,
	AppBundleSource,
	fakeActionHash,
	fakeAgentPubKey,
	fakeEntryHash,
	fakeDnaHash
} from '@holochain/client';

export type FileMetadata = {
	name: string,
	author: Uint8Array,
	path: string,
	created: number,
	last_modified: number,
	size: number,
	file_type: string,
	chunk_hashes: string[],
}

export async function createFileMetadata(cell: CallableCell, file: FileMetadata): Promise<Record> {
	return cell.callZome({
		zome_name: "file_storage",
		fn_name: "create_file_metadata",
		payload: file,
	});
}

export async function getFileMetadata(cell: CallableCell, hash: Uint8Array): Promise<Record> {
	return cell.callZome({
		zome_name: "file_storage",
		fn_name: "get_file_metadata",
		payload: hash,
	});
}

export function sampleFile(agentPubKey: Uint8Array): FileMetadata {
	return {
			name: "test.txt",
			author: agentPubKey,
			created: 1674053334548000,
			last_modified: 1674053334548000,
			size: 100,
			file_type: "text/plain",
			path: "/test.txt",
			chunk_hashes: [],
		};
}
