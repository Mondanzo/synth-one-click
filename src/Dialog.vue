<script setup lang="ts">

import {useTemplateRef} from "vue";

const props = withDefaults(defineProps<{
  onlyClose: boolean
}>(), {
  onlyClose: true,
});

const dialogId = Math.random();

let resolveFunc = () => {};
let rejectFunc = () => {};

const dialogModal = useTemplateRef<HTMLDialogElement>("dialog_modal");

function open(): Promise<void> {

  dialogModal.value?.showModal();
  return new Promise((resolve, reject) => {
    resolveFunc = () => {
      resolve();
      resolveFunc = () => {};
    };
    rejectFunc = () => {
      reject();
      rejectFunc = () => {};
    }
  });
}

function finish() {
  if(resolveFunc) resolveFunc();
}

defineExpose({open});

</script>

<template>
<dialog class="modal" id="dialog-{{dialogId}}" ref="dialog_modal" @close="finish()">
  <div class="modal-box">
    <slot />
    <div class="modal-action">
      <form method="dialog">
        <button v-if="onlyClose" class="btn btn-neutral" type="submit">Continue</button>
        <slot v-else name="actions" />
      </form>
    </div>
  </div>
</dialog>
</template>

<style scoped>

</style>