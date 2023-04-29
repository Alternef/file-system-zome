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

export type CreateFileOutput = {
	file_metadata: Record,
	file_chunks: Record[],
}

export async function createFile(cell: CallableCell, file: FileInput): Promise<CreateFileOutput> {
	return cell.callZome({
		zome_name: "file_storage",
		fn_name: "create_file",
		payload: file,
	});
}

export async function getFileMetadata(cell: CallableCell, file_hash: Uint8Array): Promise<Record> {
	return cell.callZome({
		zome_name: "file_storage",
		fn_name: "get_file_metadata",
		payload: file_hash,
	});
}

export async function getFilesMetadataByPathRecursively(cell: CallableCell, path: string): Promise<Record[]> {
	return cell.callZome({
		zome_name: "file_storage",
		fn_name: "get_files_metadata_by_path_recursively",
		payload: path,
	});
}

export async function deleteFile(cell: CallableCell, original_file_metadata_hash: Uint8Array): Promise<Record> {
	return cell.callZome({
		zome_name: "file_storage",
		fn_name: "delete_file_metadata_and_chunks",
		payload: original_file_metadata_hash,
	});
}

export function sampleFileInput(path: string = "/", name: string = "test.txt"): FileInput {
	return {
		name,
		path,
		file_type: "text/plain",
		content: new TextEncoder().encode("hello world !"),
	}
}


export function fiveMbFileInput(path: string = "/", name: string = "large_file.txt"): FileInput {
	return {
		name,
		path,
		file_type: "text/plain",
		content: new Uint8Array(5 * 1024 * 1024),
	}
}