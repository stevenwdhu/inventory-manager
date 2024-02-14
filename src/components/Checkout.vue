<template>
  <el-container>
    <el-main class="main" @click="focusInput">
      <el-form
        style="width: 100%"
        @submit.prevent="handleInputSubmit"
        label-position="left"
        label-width="80px"
      >
        <el-form-item label="姓名">
          <el-cascader
            placeholder="请先选择姓名"
            v-model="userName"
            :options="receiverOptions"
            :props="{ expandTrigger: 'hover', emitPath: false }"
            :show-all-levels="false"
          />
        </el-form-item>

        <el-form-item label="物品编码">
          <el-input
            v-focus
            ref="inputRef"
            v-model="codeInput"
            placeholder="扫码或直接输入"
            style="ime-mode: disabled"
            :formatter="(s:string) => s.toUpperCase()"
            @blur="focusInput"
            @keyup.enter="handleInputSubmit"
          />
        </el-form-item>
      </el-form>

      <el-table :data="itemList" border style="width: 100%; flex: 1">
        <el-table-column prop="barcode" label="物品编码" width="150" />
        <el-table-column prop="product" label="物品名称" min-width="180" />
        <el-table-column prop="quantity" label="数量" width="150">
          <template #default="scope">
            <el-input-number
              size="small"
              :min="1"
              v-model="scope.row.quantity"
            />
          </template>
        </el-table-column>
        <el-table-column fixed="right" label="操作" width="120">
          <template #default="row">
            <el-button
              link
              type="danger"
              size="small"
              @click.prevent="handleDelete(row.$index)"
              >删除
            </el-button>
          </template>
        </el-table-column>
      </el-table>

      <el-button type="primary" @click.prevent="handleSubmit">提交</el-button>
    </el-main>
  </el-container>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { dayjs, ElMessage } from "element-plus";
import { IDelivery, IReceiver, IStockItem } from "../types";
import { DELIVERY_LOG, INVENTORY, RECEIVER_LIST } from "../const";

const modalVisible = ref(false);
const tempUserName = ref("");
const inputRef = ref<any>(null);
const userName = ref("");
const codeInput = ref("");
const inventory = ref<IStockItem[]>([]);
const itemList = ref<IStockItem[]>([]);
const receiverList = ref<IReceiver[]>([]);

const receiverOptions = computed(() => {
  if (receiverList.value.length === 1) return [];
  const list: any[] = [];
  receiverList.value.forEach((person) => {
    let depIdx = list.findIndex((dep) => dep.label === person.department);
    if (depIdx < 0) {
      list.push({
        label: person.department,
        // value: person.department,
        children: [],
      });
      depIdx = list.length - 1;
    }
    list[depIdx].children.push({ label: person.name, value: person.name });
  });
  return list;
});

const handleSubmitName = () => {
  const { value } = tempUserName;
  if (value.length === 0) return;
  userName.value = value;
  modalVisible.value = false;
  focusInput();
};
const handleModalOpen = () => {
  tempUserName.value = userName.value;
};
const handleChangeName = () => {
  modalVisible.value = true;
};
const focusInput = () => {
  !modalVisible.value && inputRef.value?.focus();
};
const handleInputSubmit = () => {
  if (!userName.value) {
    ElMessage.error("请先选择姓名");
    codeInput.value = "";
    return;
  }
  const { value } = codeInput;

  const item = itemList.value.find((item) => item.barcode === value);
  if (item) {
    item.quantity++;
  } else {
    const itemInInventory = inventory.value.find((i) => i.barcode === value);
    if (itemInInventory) {
      itemList.value.push({
        barcode: value,
        quantity: 1,
        product: itemInInventory.product,
      });
    } else {
      ElMessage.error("该条码不存在");
    }
  }
  codeInput.value = "";
};

const handleDelete = (index: number) => {
  itemList.value.splice(index, 1);
};
const handleSubmit = async () => {
  let oldLogs: IStockItem[];
  try {
    oldLogs = JSON.parse(
      await invoke("get_data", {
        path: localStorage.getItem(DELIVERY_LOG),
      })
    );
  } catch (e) {
    ElMessage.error(e as string);
    return;
  }

  try {
    await invoke("update_data", {
      path: localStorage.getItem(DELIVERY_LOG),
      data: JSON.stringify(
        oldLogs.concat(
          itemList.value.map((item) => {
            const log: IDelivery = {
              ...item,
              time: dayjs().unix(),
              receiver: userName.value,
            };
            return log;
          })
        )
      ),
    });
  } catch (e) {
    ElMessage.error(e as string);
    return;
  }

  tempUserName.value = "";
  userName.value = "";
  itemList.value = [];
  ElMessage.success("录入成功");
};

onMounted(() => {
  invoke("get_data", { path: localStorage.getItem(INVENTORY) })
    .then((res) => (inventory.value = JSON.parse(res as string)))
    .catch((e) => ElMessage.error(e));
  invoke("get_data", { path: localStorage.getItem(RECEIVER_LIST) })
    .then((res) => (receiverList.value = JSON.parse(res as string)))
    .catch((e) => ElMessage.error(e));
});
</script>

<style scoped>
.el-container {
  width: 100%;
  height: 100%;
  justify-content: center;
}

.main {
  height: 100%;
  width: 100%;
  max-width: 800px;

  display: flex;
  flex-direction: column;
  gap: 20px;
}
</style>
