<script lang="js">
  import { mutation } from 'svelte-apollo';

  import { SetSettings } from './api/graphql/client.graphql';

  import { showError } from './util';

  const setSettingsMutation = mutation(SetSettings);

  export let visible = false;
  export let info;

  function close() {
    visible = false;
  }

  async function submit_change() {
    try {
      await setSettingsMutation({ variables: info });
      close();
    } catch (e) {
      showError(e.message);
    }
  }
</script>

<template>
  <div class="uk-modal" class:uk-open={visible}>
    <div class="uk-modal-dialog uk-modal-body">
      <h2 class="uk-modal-title">Change settings</h2>
      <button
        class="uk-modal-close-outside"
        uk-close
        type="button"
        on:click={close}
      />
      <fieldset class="settings-form">
        <input class="uk-input" bind:value={info.title} placeholder="Title" />
        <div class="uk-alert">
          Title for the server. This title is visible in current tab of the
          browser
        </div>
        <label
          ><input
            class="uk-checkbox"
            bind:checked={info.deleteConfirmation}
            type="checkbox"
          /> Delete confirmation for inputs and outputs</label
        >
      </fieldset>

      <button class="uk-button uk-button-primary" on:click={submit_change}
        >Change</button
      >
    </div>
  </div>
</template>

<style lang="stylus">
  .uk-modal
    &.uk-open
      display: block

    .uk-modal-title
      font-size: 1.5rem

    .settings-form
      border: none

      & >.uk-alert
        margin-top: 5px !important;
</style>
