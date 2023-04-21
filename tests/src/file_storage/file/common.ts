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
	author: string,
}

export async function createFileMetadata(cell: CallableCell, file: FileMetadata = undefined): Promise<Record> {
	return cell.callZome({
		zome_name: "file",
		fn_name: "create_file_metadata",
		payload: file || await sampleFile(cell),
	});
}