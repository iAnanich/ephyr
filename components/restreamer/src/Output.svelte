<script lang="js">
  import { mutation, subscribe } from 'svelte-apollo';
  import { link, location } from 'svelte-spa-router';

  import {
    DisableOutput,
    EnableOutput,
    Info,
    RemoveOutput,
    TuneVolume,
  } from './api/graphql/client.graphql';

  import { showError, isOutputPage } from './util';

  import { outputModal } from './stores';

  import Confirm from './Confirm.svelte';
  import Toggle from './Toggle.svelte';
  import Mixin from './Mixin.svelte';
  import RecordsModal from './RecordsModal.svelte';
  import Url from './Url.svelte';

  const disableOutputMutation = mutation(DisableOutput);
  const enableOutputMutation = mutation(EnableOutput);
  const removeOutputMutation = mutation(RemoveOutput);
  const tuneVolumeMutation = mutation(TuneVolume);

  const info = subscribe(Info, { errorPolicy: 'all' });

  export let public_host;
  export let value;
  export let restream_id;
  export let hidden = false;

  let volume = 100;
  $: {
    // Trigger Svelte reactivity watching.
    value.volume = value.volume;
    // Move `volume` to a separate function to omit triggering this block when
    // it is changed, as we're only interested in `value` changes here.
    update_volume();
  }

  $: deleteConfirmation = $info.data
    ? $info.data.info.deleteConfirmation
    : true;

  // Last used non-zero volume.
  let last_volume = value.volume === 0 ? 100 : value.volume;

  function update_volume() {
    volume = value.volume;
  }

  async function toggle() {
    const variables = { restream_id, output_id: value.id };
    try {
      if (value.enabled) {
        await disableOutputMutation({ variables });
      } else {
        await enableOutputMutation({ variables });
      }
    } catch (e) {
      showError(e.message);
    }
  }

  async function remove() {
    const variables = { restream_id, output_id: value.id };
    try {
      await removeOutputMutation({ variables });
    } catch (e) {
      showError(e.message);
    }
  }

  async function tuneVolume() {
    if (volume !== 0) {
      last_volume = volume;
    }
    const variables = { restream_id, output_id: value.id, volume };
    try {
      await tuneVolumeMutation({ variables });
    } catch (e) {
      showError(e.message);
    }
  }

  async function toggleVolume() {
    volume = volume !== 0 ? 0 : last_volume;
    await tuneVolume();
  }

  function openEditOutputModal() {
    outputModal.openEdit(
      restream_id,
      value.id,
      value.label,
      value.previewUrl,
      value.dst,
      value.mixins.map((m) => m.src)
    );
  }
</script>

<template>
  <div
    class="uk-card uk-card-default uk-card-body uk-flex"
    class:hidden
    class:grouped={!isOutputPage($location)}
    class:uk-margin-left={!isOutputPage($location)}
  >
    <Confirm let:confirm>
      <button
        type="button"
        class="uk-close"
        uk-close
        on:click={deleteConfirmation ? () => confirm(remove) : remove}
      />
      <span slot="title">Removing output</span>
      <span slot="description"
        ><code class="overflow-wrap">{value.dst}</code>
        <br /><br />
        {#if value.dst.startsWith('file:///')}
          <b>Warning!</b> Any associated recorded files will be removed.
          <br /><br />
        {/if}
        You won't be able to undone this.</span
      >
      <span slot="confirm">Remove</span>
    </Confirm>

    {#if value.label}
      <span class="label">{value.label}</span>
    {/if}

    <div class="left-buttons-area" />
    <a
      class="edit-output"
      href="/"
      on:click|preventDefault={openEditOutputModal}
    >
      <i class="far fa-edit" title="Edit output" />
    </a>

    <div>
      <Toggle
        id="output-toggle-{value.id}"
        classes="small"
        checked={value.enabled}
        on:change={toggle}
      />
    </div>

    <div class="output-mixes">
      {#if value.status === 'ONLINE'}
        <span><i class="fas fa-circle uk-alert-success" /></span>
      {:else if value.status === 'INITIALIZING'}
        <span><i class="fas fa-dot-circle uk-alert-warning" /></span>
      {:else}
        <span><i class="far fa-dot-circle uk-alert-danger" /></span>
      {/if}
      {#if value.dst.startsWith('file:///') && value.status === 'OFFLINE'}
        <RecordsModal let:open id={value.id} {public_host}>
          <a
            class="dvr-link"
            href="/"
            on:click|preventDefault={open}
            title="Download records">{value.dst}</a
          >
        </RecordsModal>
      {:else}
        <Url url={value.dst} previewUrl={value.previewUrl} />
      {/if}

      {#if value.mixins.length > 0}
        {#if !isOutputPage($location)}
          <a
            class="single-view"
            href="/restream/{restream_id}/output/{value.id}"
            use:link
            title="Open in a separate window"
            ><i class="fas fa-external-link-alt" />
          </a>
        {/if}

        <div class="volume orig">
          <a href="/" on:click|preventDefault={toggleVolume}>
            {#if volume > 0}
              <span><i class="fas fa-volume-up" title="Volume" /></span>
            {:else}
              <span><i class="fas fa-volume-mute" title="Muted" /></span>
            {/if}
          </a>
          <input
            class="uk-range"
            type="range"
            min="0"
            max="200"
            step="1"
            bind:value={volume}
            on:change={tuneVolume}
          />
          <span>{volume}%</span>
        </div>

        {#each value.mixins as mixin}
          <Mixin {restream_id} output_id={value.id} value={mixin} />
        {/each}
      {/if}
    </div>
  </div>
</template>

<style lang="stylus">
  .uk-card
    position: relative
    padding: 6px
    margin-top: 15px !important
    min-width 250px
    font-size: 13px
    &.grouped
      width: calc((100% - (20px * 2)) / 2)
      @media screen and (max-width: 700px)
        width: 100%
    &.hidden
      display: none

    .uk-close
      position: absolute;
      right: 6px;
      top: 9px;

    .label
      position: absolute
      top: -12px
      left: 0
      padding: 0 6px
      font-size: 13px
      border-top-left-radius: 4px
      border-top-right-radius: 4px
      background-color: #fff

    a.single-view, a.edit-output
      position: absolute
      outline: none
      transition: opacity .3s ease
      &:hover
        text-decoration: none
    a.single-view
      top: 47px
      left: 16px
      color: #d9d9d9
      &:hover
        color: #c4c4c4
    a.edit-output
      left: -16px
      top: 6px
      color: var(--primary-text-color)
      &:hover
        color: #444
        opacity: 1
    &:not(:hover)
      a.single-view, a.edit-output
        opacity: 0

    .left-buttons-area
      position: absolute
      width: 18px
      right: 100%
      top: 0
      height: 100%

  .fa-circle, .fa-dot-circle
    font-size: 10px
    margin-top: -1px
  .fa-volume-up, .fa-volume-mute
    font-size: 10px

  .volume
    padding-left: 17px
    font-size: 10px

    a
      color: #d9d9d9
      outline: none
      &:hover
        text-decoration: none
        color: #c4c4c4

    .uk-range::-moz-range-thumb, .uk-range::-webkit-slider-thumb
      width: 7px
      height: 12px
    .uk-range
      display: inline-block
      width: 74%
      margin-top: -1px

  a.dvr-link
    color: var(--primary-text-color)

  .output-mixes
    width: calc(100% - 56px);
    margin-left: 4px
</style>
