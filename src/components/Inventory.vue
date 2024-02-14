<template>
  <el-container>
    <div class="top-bar">
      <el-space>
        <el-button @click="handleImport">导入excel</el-button>
        <el-button @click="handleExport">导出csv</el-button>
      </el-space>
      <el-button type="success" @click="handleNewItem">新建条目</el-button>
    </div>
    <el-table :data="itemList" border style="width: 100%; flex: 1">
      <el-table-column type="index" :index="(i:number)=>i+1" />
      <el-table-column prop="barcode" label="物品编码" width="150">
        <template #default="scope">
          <el-input
            v-focus
            v-if="editing.index === scope.$index"
            size="small"
            v-model="editing.barcode"
            :formatter="(s:string)=>s.toUpperCase()"
            @keyup.enter="handleConfirmEdit"
          />
          <div v-else>{{ scope.row.barcode }}</div>
        </template>
      </el-table-column>
      <el-table-column prop="product" label="物品名称" min-width="180">
        <template #default="scope">
          <el-input
            v-if="editing.index === scope.$index"
            size="small"
            v-model="editing.product"
            @keyup.enter="handleConfirmEdit"
          />
          <div v-else>{{ scope.row.product }}</div>
        </template>
      </el-table-column>
      <el-table-column prop="quantity" label="数量" width="150">
        <template #default="scope">
          <el-input-number
            size="small"
            :min="1"
            v-model="itemList[scope.$index].quantity"
          />
        </template>
      </el-table-column>
      <el-table-column fixed="right" label="操作" width="120">
        <template #default="scope">
          <template v-if="editing.index === scope.$index">
            <el-button
              link
              type="primary"
              size="small"
              @click.prevent="handleConfirmEdit"
              >完成
            </el-button>
            <el-button
              link
              type="danger"
              size="small"
              @click.prevent="handleCancelEdit"
              >取消
            </el-button>
          </template>

          <template v-else>
            <el-button
              link
              type="primary"
              size="small"
              @click.prevent="handleEdit(scope.$index)"
              >编辑
            </el-button>
            <el-button
              link
              type="danger"
              size="small"
              @click.prevent="handleDelete(scope.$index)"
              >删除
            </el-button>
          </template>
        </template>
      </el-table-column>
    </el-table>
  </el-container>
</template>

<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import { dayjs, ElMessage } from "element-plus";
import { IStockItem } from "../types";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { INVENTORY } from "../const";

const editing = ref({
  index: -1,
  barcode: "",
  product: "",
});
const itemList = ref<IStockItem[]>([]);
const initEditing = () => {
  editing.value = {
    index: -1,
    barcode: "",
    product: "",
  };
};

const getInventory = () => {
  invoke("get_data", { path: localStorage.getItem(INVENTORY) })
    .then((res) => (itemList.value = JSON.parse(res as string)))
    .catch((e) => ElMessage.error(e));
};

const updateInventory = (value: IStockItem[]) => {
  invoke("update_data", {
    path: localStorage.getItem(INVENTORY),
    data: JSON.stringify(value),
  }).catch((e) => ElMessage.error(e));
};

const handleNewItem = () => {
  if (editing.value.index > -1 && !handleConfirmEdit()) return;
  itemList.value.push({
    barcode: "",
    quantity: 1,
    product: "",
  });
  handleEdit(itemList.value.length - 1);
};
const handleEdit = (index: number) => {
  if (editing.value.index > -1 && !handleConfirmEdit()) return;
  editing.value.index = index;
  editing.value.barcode = itemList.value[index].barcode;
  editing.value.product = itemList.value[index].product;
};
const handleConfirmEdit = (): boolean => {
  if (editing.value.index < 0) {
    initEditing();
    return true;
  }
  if (editing.value.barcode.length === 0) {
    ElMessage.error("条码不能为空");
    return false;
  }
  if (editing.value.product.length === 0) {
    ElMessage.error("物品名称不能为空");
    return false;
  }
  if (
    itemList.value
      .filter((_, index) => index !== editing.value.index)
      .some((item) => item.barcode === editing.value.barcode)
  ) {
    ElMessage.error("该条码已存在");
    return false;
  }
  itemList.value[editing.value.index].barcode = editing.value.barcode;
  itemList.value[editing.value.index].product = editing.value.product;

  initEditing();
  return true;
};
const handleCancelEdit = () => {
  initEditing();
};
const handleDelete = (index: number) => {
  itemList.value.splice(index, 1);
};

const handleImport = async () => {
  const filePath = await open({
    filters: [{ name: "Excel", extensions: ["xls", "xlsx"] }],
  });
  if (!filePath) return;
  invoke("import_excel_to_json", { path: filePath })
    .then((res) => {
      itemList.value = JSON.parse(res as string);
      ElMessage.success("导入成功");
    })
    .catch((e) => ElMessage.error(e as string));
};
const handleExport = async () => {
  const dirPath = await open({ directory: true });
  if (!dirPath) return;
  const path = dirPath + "/export" + dayjs().format("YYYYMMDD") + ".csv";
  invoke("export_json_to_csv", {
    path,
    data: JSON.stringify(itemList.value),
  })
    .then(() => ElMessage.success("导出成功"))
    .catch((e) => ElMessage.error(e));
};

onMounted(() => {
  getInventory();
});

watch(
  itemList,
  (value) => {
    updateInventory(value);
  },
  { deep: true }
);
</script>

<style scoped>
.el-container {
  flex-direction: column;
  width: 100%;
  height: 100%;
  max-width: 800px;
  gap: 20px;
}

.top-bar {
  display: flex;
  justify-content: space-between;
}
</style>
