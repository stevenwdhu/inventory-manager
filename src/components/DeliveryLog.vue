<template>
  <el-container>
    <el-table :data="logList" border style="width: 100%; flex: 1">
      <el-table-column
        prop="time"
        label="时间"
        width="180"
        :formatter="timeFormatter"
      />
      <el-table-column prop="receiver" label="取用人" width="150" />
      <el-table-column prop="barcode" label="物品编码" width="150" />
      <el-table-column prop="product" label="物品名称" min-width="180" />
      <el-table-column prop="quantity" label="数量" width="80" />
    </el-table>
  </el-container>
</template>

<script setup lang="ts">
import { onMounted, onUpdated, ref, watch } from "vue";
import { dayjs, ElMessage } from "element-plus";
import { IDelivery, IStockItem } from "../types";
import { invoke } from "@tauri-apps/api/tauri";
import { DELIVERY_LOG } from "../const";

const logList = ref<IDelivery[]>([]);

const timeFormatter = (log: IDelivery) => {
  return dayjs.unix(log.time).format("YYYY-MM-DD HH:mm:ss");
};

onMounted(() => {
  invoke("get_data", { path: localStorage.getItem(DELIVERY_LOG) })
    .then((res) => (logList.value = JSON.parse(res as string)))
    .catch((e) => ElMessage.error(e));
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
