<template>
  <el-container>
    <el-form label-width="150px" label-position="left">
      <el-form-item label="出库记录文件路径">
        <el-input v-model="deliveryLogPath" readonly>
          <template #append>
            <el-button @click="handleChoosePath(DELIVERY_LOG)"
              >选择路径</el-button
            >
          </template>
        </el-input>
      </el-form-item>
      <el-form-item label="库存数据文件路径">
        <el-input v-model="inventoryPath" readonly>
          <template #append>
            <el-button @click="handleChoosePath(INVENTORY)">选择路径</el-button>
          </template>
        </el-input>
      </el-form-item>
      <el-form-item label="人员列表文件路径">
        <el-input v-model="receiverListPath" readonly>
          <template #append>
            <el-button @click="handleChoosePath(RECEIVER_LIST)"
              >选择路径</el-button
            >
          </template>
        </el-input>
      </el-form-item>
    </el-form>
  </el-container>
</template>

<script setup lang="ts">
import { Ref, ref, watch } from "vue";
import { DELIVERY_LOG, INVENTORY, RECEIVER_LIST } from "../const";
import { open } from "@tauri-apps/api/dialog";

const deliveryLogPath = ref(localStorage.getItem(DELIVERY_LOG));
const inventoryPath = ref(localStorage.getItem(INVENTORY));
const receiverListPath = ref(localStorage.getItem(RECEIVER_LIST));

const handleChoosePath = async (setting: string) => {
  const path = await open({
    multiple: false,
    filters: [{ name: "Json", extensions: ["json"] }],
  });
  if (!path) return;
  switch (setting) {
    case DELIVERY_LOG:
      deliveryLogPath.value = path as string;
      return;
    case INVENTORY:
      inventoryPath.value = path as string;
      return;
    case RECEIVER_LIST:
      receiverListPath.value = path as string;
      return;
  }
};

watch(deliveryLogPath, (value) => {
  localStorage.setItem(DELIVERY_LOG, value as string);
});
watch(inventoryPath, (value) => {
  localStorage.setItem(INVENTORY, value as string);
});
watch(receiverListPath, (value) => {
  localStorage.setItem(RECEIVER_LIST, value as string);
});
</script>

<style scoped>
.el-container {
  flex-direction: column;
  width: 100%;
  height: 100%;
  max-width: 800px;
  gap: 20px;
}
</style>
