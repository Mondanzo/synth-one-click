<script setup lang="ts">

import {useTemplateRef} from "vue";

const props = withDefaults(defineProps<{
  onlyClose: boolean
}>(), {
  onlyClose: true,
});

let resolveFunc = (_: string | undefined) => {
};
const dialogModal = useTemplateRef<HTMLDialogElement>("dialog_modal");

function open(): Promise<string|undefined> {
  dialogModal.value?.showModal();
  return new Promise((resolve, _) => {
    resolveFunc = (data: string|undefined) => {
      resolve(data);
      resolveFunc = () => {
      };
    };
  });

}

function finish() {
  if (resolveFunc) resolveFunc(dialogModal.value!.returnValue);
}

function closeWithData(data: string): void {
  dialogModal.value!.returnValue = data;
  dialogModal.value!.close(data);
}

defineExpose({open, closeWithData});

</script>

<template>
  <dialog class="modal" id="dialog" ref="dialog_modal" @close="finish">
    <div class="modal-box">
      <slot/>
      <div class="modal-action">
        <form method="dialog" class="flex gap-3">
          <button v-if="props.onlyClose" class="btn btn-neutral" type="submit">Continue</button>
          <slot v-else name="actions"/>
        </form>
      </div>
    </div>
  </dialog>
</template>