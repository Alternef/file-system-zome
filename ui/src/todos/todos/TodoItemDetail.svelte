<script lang="ts">
import { createEventDispatcher, onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import { decode } from '@msgpack/msgpack';
import type { Record, ActionHash, AppAgentClient, EntryHash, AgentPubKey, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { TodoItem } from './types';
import '@material/mwc-circular-progress';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-snackbar';
import '@material/mwc-icon-button';
import EditTodoItem from './EditTodoItem.svelte'; 

const dispatch = createEventDispatcher();

export let todoItemHash: ActionHash;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let loading = true;
let error: any = undefined;

let record: Record | undefined;
let todoItem: TodoItem | undefined;

let editing = false;

let errorSnackbar: Snackbar;
  
$: editing,  error, loading, record, todoItem;

onMount(async () => {
  if (todoItemHash === undefined) {
    throw new Error(`The todoItemHash input is required for the TodoItemDetail element`);
  }
  await fetchTodoItem();
});

async function fetchTodoItem() {
  loading = true;
  error = undefined;
  record = undefined;
  todoItem = undefined;
  
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: 'todos',
      zome_name: 'todos',
      fn_name: 'get_todo_item',
      payload: todoItemHash,
    });
    if (record) {
      todoItem = decode((record.entry as any).Present.entry) as TodoItem;
    }
  } catch (e) {
    error = e;
  }

  loading = false;
}

async function deleteTodoItem() {
  try {
    await client.callZome({
      cap_secret: null,
      role_name: 'todos',
      zome_name: 'todos',
      fn_name: 'delete_todo_item',
      payload: todoItemHash,
    });
    dispatch('todo-item-deleted', { todoItemHash: todoItemHash });
  } catch (e: any) {
    errorSnackbar.labelText = `Error deleting the todo item: ${e.data.data}`;
    errorSnackbar.show();
  }
}
</script>

<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>

{#if loading}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the todo item: {error.data.data}</span>
{:else if editing}
<EditTodoItem
  originalTodoItemHash={ todoItemHash}
  currentRecord={record}
  on:todo-item-updated={async () => {
    editing = false;
    await fetchTodoItem()
  } }
  on:edit-canceled={() => { editing = false; } }
></EditTodoItem>
{:else}

<div style="display: flex; flex-direction: column">
  <div style="display: flex; flex-direction: row">
    <span style="flex: 1"></span>
    <mwc-icon-button style="margin-left: 8px" icon="edit" on:click={() => { editing = true; } }></mwc-icon-button>
    <mwc-icon-button style="margin-left: 8px" icon="delete" on:click={() => deleteTodoItem()}></mwc-icon-button>
  </div>

  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="margin-right: 4px"><strong>Description:</strong></span>
    <span style="white-space: pre-line">{ todoItem.description }</span>
  </div>

</div>
{/if}

