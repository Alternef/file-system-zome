import {CallableCell} from '@holochain/tryorama';
import {Record} from '@holochain/client';

export type FileMetadata = {
	name: string,
	author: Uint8Array,
	path: string,
	created: number,
	last_modified: number,
	size: number,
	file_type: string,
	chunks_hashes: Uint8Array[],
}

export type FileInput = {
	name: string,
	path: string,
	file_type: string,
	content: Uint8Array,
}

export function sampleFileInput(): FileInput {
	return {
		name: "test.txt",
		path: "/test.txt",
		file_type: "text/plain",
		content: new TextEncoder().encode("hello world"),
	}
}

export async function createFile(cell: CallableCell, file: FileInput): Promise<Record[]> {
	return cell.callZome({
		zome_name: "file_storage",
		fn_name: "create_file",
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

export async function getFileChunk(cell: CallableCell, hash: Uint8Array): Promise<Record> {
	return cell.callZome({
		zome_name: "file_storage",
		fn_name: "get_file_chunk",
		payload: hash,
	});
}


