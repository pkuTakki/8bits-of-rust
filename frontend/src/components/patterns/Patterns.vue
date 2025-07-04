<!-- pattern管理界面 -->
<template>
  <div class="container" @contextmenu.prevent>
    <div class="input-container">
      <my-input v-model="patternName" />
      <my-button
        class="add-button"
        size="small"
        text="+"
        @click="addPattern()"
      />
    </div>
    <div class="patterns-container">
      <div
        v-for="(pattern, index) in patterns"
        class="pattern"
        draggable="true"
        @dragstart="dragStart(index)"
        @dragover="allowDrop"
        @drop="drop(index)"
      >
        <my-button
          :text="pattern.name"
          :active="activePattern === pattern.id"
          :color="pattern.color"
          @click.left="handleLeftClick(pattern.id, $event)"
          @click.right="deletePattern(pattern.id)"
          @dblclick="editPattern(pattern.id)"
        />
        <my-input
          class="rename"
          v-if="isEdit === pattern.id"
          v-model="newPatternName"
          placeholder="输入新名称..."
          @keyup.enter="renamePattern(pattern.id)"
          @keyup.esc="stopRenamePattern"
        />
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from "vue";
import { useStore } from "vuex";
import { useColorGenerator } from "@/components/common/ColorGenerator.js";
import { useDragDrop } from "./dragDrop";

const { getRandomColor } = useColorGenerator();
const store = useStore();
const { dragStart, allowDrop, drop } = useDragDrop(store.commit);
const patterns = computed(() => store.state.patterns);
const activePattern = computed(() => store.state.activePattern);
// 是否正在编辑
const isEdit = ref(-1);
const patternName = ref("");
const newPatternName = ref("");

// 更新选中的pattern和钢琴窗中的音符
const handleLeftClick = (id, e) => {
  store.commit("saveNotes");
  store.commit("setActivePattern", id);
  store.commit("loadNotes");
};

// 添加乐段
const addPattern = () => {
  const newPattern = {
    id: Date.now(),
    color: getRandomColor(),
    name:
      patternName.value !== ""
        ? patternName.value
        : "Pattern " + (patterns.value.length + 1),
    notes: [],

    scrollX: 0,
    scrollY: 0,
    scaleX: 1,
    scaleY: 1,
  };
  patternName.value = "";
  // console.log("New color:", newPattern.color);
  store.commit("addPattern", newPattern);
  // console.log("Added pattern:", newPattern);
};

// 删除乐段
const deletePattern = (id) => {
  if (isEdit.value === id) {
    isEdit.value = -1;
  } else if (confirm("确认删除乐段?")) {
    if (id === activePattern.value) {
      console.log("need reset active pattern");
      store.commit("setActivePattern", 0);
      store.commit("emptyNotes");
    }
    store.commit("deletePattern", id);
  }
};

// 重命名pattern
const editPattern = (id) => {
  // clearTimeout(timeout)
  isEdit.value = id;
};
const renamePattern = (id) => {
  console.log("enter");
  store.commit("renamePattern", {
    id: id,
    name: newPatternName.value,
  });
  newPatternName.value = "";
  isEdit.value = -1;
};
const stopRenamePattern = () => {
  console.log("esc");
  newPatternName.value = "";
  isEdit.value = -1;
};
</script>

<style scoped>
.container {
  display: flex;
  flex-direction: column !important;
  height: 100%;
  padding: 10px;
  background-color: var(--global-ghost);
}

.input-container {
  display: flex;
  flex-direction: row;
  position: sticky;
}

.patterns-container {
  display: flex;
  flex-direction: column;
  overflow-y: auto;
  gap: 6px;
}

.add-button {
  max-width: 20px;
}
.rename {
  width: 125px;
}
.pattern {
  z-index: 6;
}
</style>
