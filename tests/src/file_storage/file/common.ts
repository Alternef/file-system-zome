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
	chunks: Record[],
}

export function sampleFileInput(path: string = "/", name: string = "test.txt"): FileInput {
	return {
		name,
		path,
		file_type: "text/plain",
		content: new TextEncoder().encode("hello world !"),
	}
}

export async function createFile(cell: CallableCell, file: FileInput): Promise<CreateFileOutput> {
	return cell.callZome({
		zome_name: "file_storage",
		fn_name: "create_file",
		payload: file,
	});
}

export async function getFilesMetadataByPathRecursively(cell: CallableCell, path: string): Promise<Record[]> {
	return cell.callZome({
		zome_name: "file_storage",
		fn_name: "get_files_metadata_by_path_recursively",
		payload: path,
	});
}