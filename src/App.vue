<template>
    <div class="main">
        <div class="title">神秘数字等式构造</div>
        <div class="sub-title">By YYYCZ</div>
        <NumberInput v-model="mns"></NumberInput>
        <div class="result">
            {{ result }}<img src="/copy.svg" title="复制" @click="copyResult" />
        </div>
    </div>
</template>

<script setup lang="ts">
import debounce from "lodash/debounce";
import NumberInput from "./components/NumberInput.vue";
import { ref, watch } from "vue";
import { findMysteriousNumberEquation } from "./lib/mneq";

const mns = ref("");
const result = ref("无解");

// 输入框防抖
const debouncedFMNE = debounce((value: string) => {
    try {
        result.value = findMysteriousNumberEquation(value);
    } catch (errCode) {
        if (errCode === 0) {
            result.value = "无解";
        }
    }
}, 300);

// 输入框监听
watch(mns, (value) => {
    if (value.length >= 2) {
        debouncedFMNE(value);
    }
});

// 复制结果
function copyResult() {
    navigator.clipboard.writeText(result.value);
    alert("结果复制成功");
}
</script>

<style scoped lang="scss">
.main {
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100vh;
    width: 100vw;
    padding-left: 32px;
    padding-right: 32px;
    background-color: var(--background-color);

    .title {
        font-size: 48px;
        text-align: center;
        color: var(--primary-color);
        margin-bottom: 8px;
        user-select: none;
    }

    .sub-title {
        font-size: 24px;
        color: var(--text-200);
        margin-bottom: 72px;
        user-select: none;
    }

    .result {
        margin-top: 72px;
        font-size: 24px;
        color: var(--text-color);
        text-align: center;

        img {
            margin-left: 4px;
            cursor: pointer;
        }
    }
}
</style>
