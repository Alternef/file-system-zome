<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, DnaHash, ActionHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';
import { clientContext } from '../../contexts';
import type { TodoItem } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textarea';

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let originalTodoItemHash!: ActionHash;

export let currentRecord!: Record;
let currentTodoItem: TodoItem = decode((currentRecord.entry as any).Present.entry) as TodoItem;

let description: string | undefined = currentTodoItem.description;

let errorSnackbar: Snackbar;

$: description;
$: isTodoItemValid = true && description !== '';

onMount(() => {
  if (currentRecord === undefined) {
    throw new Error(`The currentRecord input is required for the EditTodoItem element`);
  }
  if (originalTodoItemHash === undefined) {
    throw new Error(`The originalTodoItemHash input is required for the EditTodoItem element`);
  }
});

async function updateTodoItem() {

  const todoItem: TodoItem = { 
    description: description!,
  };

  try {
    const updateRecord: Record = await client.callZome({
      cap_secret: null,
      role_name: 'todos',
      zome_name: 'todos',
      fn_name: 'update_todo_item',
      payload: {
        original_todo_item_hash: originalTodoItemHash,
        previous_todo_item_hash: currentRecord.signed_action.hashed.hash,
        updated_todo_item: todoItem
      }
    });
  
    dispatch('todo-item-updated', { actionHash: updateRecord.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error updating the todo item: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Edit TodoItem</span>
  
  <div style="margin-bottom: 16px">
    <mwc-textarea outlined label="Description" value={ description } on:input={e => { description = e.target.value;} } required></mwc-textarea>    
  </div>


  <div style="display: flex; flex-direction: row">
    <mwc-button
      outlined
      label="Cancel"
      on:click={() => dispatch('edit-canceled')}
      style="flex: 1; margin-right: 16px"
    ></mwc-button>
    <mwc-button 
      raised
      label="Save"
      disabled={!isTodoItemValid}
      on:click={() => updateTodoItem()}
      style="flex: 1;"
    ></mwc-button>
  </div>
</div>
