import {CallableCell} from '@holochain/tryorama';
import {ActionHash, AgentPubKey, Record} from '@holochain/client';

export type FileMetadata = {
	name: string,
	author: AgentPubKey,
	path: string,
	created: number,
	last_modified: number,
	size: number,
	file_type: string,
	chunks_hashes: Uint8Array[],
}

export type CreateFileInput = {
	name: string,
	path: string,
	file_type: string,
	content: Uint8Array,
}

export type FileOutput = {
	file_metadata: Record,
	file_chunks: Record[],
}

export async function createFile(cell: CallableCell, file: CreateFileInput): Promise<FileOutput> {
	return cell.callZome({
		zome_name: "file_storage",
		fn_name: "create_file",
		payload: file,
	});
}

export async function getFileChunks(cell: CallableCell, file_metadata_hash: ActionHash): Promise<Record[]> {
	return cell.callZome({
		zome_name: "file_storage",
		fn_name: "get_file_chunks",
		payload: file_metadata_hash,
	});
}

export async function getFileMetadata(cell: CallableCell, file_metadata_hash: ActionHash): Promise<Record | null> {
	return cell.callZome({
		zome_name: "file_storage",
		fn_name: "get_file_metadata",
		payload: file_metadata_hash,
	});
}

export async function getFilesMetadataByPathRecursively(cell: CallableCell, path: string): Promise<Record[]> {
	return cell.callZome({
		zome_name: "file_storage",
		fn_name: "get_files_metadata_by_path_recursively",
		payload: path,
	});
}

export async function updateFile(cell: CallableCell, original_file_metadata_hash: ActionHash, new_content: Uint8Array): Promise<FileOutput> {
	return cell.callZome({
		zome_name: "file_storage",
		fn_name: "update_file",
		payload: {
			original_file_metadata_hash,
			new_content,
		},
	});
}

export async function deleteFile(cell: CallableCell, original_file_metadata_hash: ActionHash): Promise<Record> {
	return cell.callZome({
		zome_name: "file_storage",
		fn_name: "delete_file",
		payload: original_file_metadata_hash,
	});
}

export function sampleFileInput(
	path: string = "/",
	name: string = "test.txt",
	content: string = "hello world !"
): CreateFileInput {
	return {
		name,
		path,
		file_type: "text/plain",
		content: new TextEncoder().encode(content),
	}
}

export function fiveMbFileInput(path: string = "/", name: string = "large_file.txt"): CreateFileInput {
	return {
		name,
		path,
		file_type: "text/plain",
		content: new Uint8Array(5 * 1024 * 1024),
	}
}