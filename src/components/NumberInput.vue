<template>
    <div class="number-input">
        <input type="number" required :value="model" @input="numberChanged" />
        <label for="input" class="label">输入神秘数字</label>
        <div class="underline"></div>
    </div>
</template>

<script setup lang="ts">
const model = defineModel();

const numberChanged = (e: Event) => {
    const target = e.target as HTMLInputElement;
    // 只能填入数字
    target.value = target.value.replace(/[^\d]/g, "");
    // 不能超过 20 位数字
    if (target.value.length > 20) {
        target.value = target.value.slice(0, 20);
    }
    model.value = target.value;
};
</script>

<style scoped lang="scss">
.number-input {
    position: relative;
    width: min(320px, 80%);

    input[type="number"] {
        font-size: 24px;
        width: 100%;
        border: none;
        color: var(--text-color);
        border-bottom: 1px solid var(--text-200);
        padding: 5px 0;
        background-color: transparent;
        outline: none;
        text-align: center;
    }

    input[type="number"]::-webkit-outer-spin-button,
    input[type="number"]::-webkit-inner-spin-button {
        -webkit-appearance: none !important;
    }
    input[type="number"] {
        appearance: textfield;
        -moz-appearance: textfield;
    }

    .label {
        font-size: 20px;
        position: absolute;
        top: 0;
        left: 50%;
        color: var(--text-200);
        transition: all 0.3s ease;
        pointer-events: none;
        transform: translateX(-50%);
    }

    input[type="number"]:focus ~ .label,
    input[type="number"]:valid ~ .label {
        top: -24px;
        font-size: 18px;
        color: var(--primary-200);
    }

    .underline {
        position: absolute;
        bottom: 0;
        left: 0;
        height: 1px;
        width: 100%;
        background-color: var(--primary-200);
        transform: scaleX(0);
        transition: all 0.3s ease;
    }

    input[type="number"]:focus ~ .underline,
    input[type="number"]:valid ~ .underline {
        transform: scaleX(1);
    }
}
</style>
