<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { TodoItem } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textarea';

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();


let description: string = '';

let errorSnackbar: Snackbar;

$: description;
$: isTodoItemValid = true && description !== '';

onMount(() => {
});

async function createTodoItem() {  
  const todoItemEntry: TodoItem = { 
    description: description!,
  };
  
  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'todos',
      zome_name: 'todos',
      fn_name: 'create_todo_item',
      payload: todoItemEntry,
    });
    dispatch('todo-item-created', { todoItemHash: record.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error creating the todo item: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create TodoItem</span>
  

  <div style="margin-bottom: 16px">
    <mwc-textarea outlined label="Description" value={ description } on:input={e => { description = e.target.value;} } required></mwc-textarea>          
  </div>
            

  <mwc-button 
    raised
    label="Create TodoItem"
    disabled={!isTodoItemValid}
    on:click={() => createTodoItem()}
  ></mwc-button>
</div>
